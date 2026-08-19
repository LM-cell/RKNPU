/* RKNPU 并发优先级与四阶段延迟测试，最后修改日期：2026-08-07。 */

/*
 * 测试流程：
 * 1. 读取驱动资源基线，复位 Submit trace，并开启调度事件记录；
 * 2. 每个线程独占一组 10 Task 矩阵负载，但所有线程共享同一个 DRM fd；
 * 3. 使用轮次屏障同时发起 blocking Submit，执行 2 轮预热和 100 轮测量；
 * 4. 结束后一次性读取四阶段记录和调度事件，不在运行过程中打印逐次日志；
 * 5. 检查优先级、Running/Ready 来源、Task 下发/完成一一匹配和计算结果；
 * 6. 输出延迟与吞吐量，释放 GEM 后确认资源回到测试前基线。
 */

#include <errno.h>
#include <pthread.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/ioctl.h>
#include <time.h>

#include "benchmark_stats.h"
#include "npu_interface.h"
#include "rknpu_batch_workload.h"
#include "rknpu-ioctl.h"

#define DEFAULT_THREADS 3U
#define DEFAULT_CORES 3U
#define DEFAULT_WARMUP 2U
#define DEFAULT_ROUNDS 100U
#define MAX_THREADS 6U

/*
 * ioctl 命令编码包含结构尺寸；编译期断言可在上传板端前发现 C/Rust UAPI
 * 偏移不一致，避免驱动 copy-in/copy-out 读写错误长度。
 */
_Static_assert(sizeof(struct rknpu_task) == 40, "RKNPU task ABI must be 40 bytes");
_Static_assert(sizeof(struct rknpu_submit) == 104, "RKNPU submit ABI must be 104 bytes");
_Static_assert(sizeof(struct rknpu_submit_trace_record) == 48,
               "submit trace record ABI must be 48 bytes");
_Static_assert(sizeof(struct rknpu_schedule_trace_record) == 56,
               "schedule trace record ABI must be 56 bytes");
_Static_assert(sizeof(struct rknpu_schedule_trace_query) == 40,
               "schedule trace query ABI must be 40 bytes");
_Static_assert(sizeof(struct rknpu_scheduler_state_snapshot) == 40,
               "scheduler state ABI must be 40 bytes");

typedef enum {
    /* 所有线程 priority=0，用作相同并发和 trace 开销下的性能基线。 */
    PRIORITY_BASELINE = 0,
    /* 3 线程使用 -10/0/10，6 线程每个优先级各两个线程。 */
    PRIORITY_MIXED,
} priority_mode_t;

/* 一次进程运行只执行一个线程数、核心数和优先级组合。 */
typedef struct {
    /* 只允许 3 或 6，与实验矩阵保持一致。 */
    uint32_t threads;
    /* 允许使用的 NPU 核心数量，只允许 1、2、3。 */
    uint32_t cores;
    /* 根据 cores 生成的 0x1、0x3 或 0x7。 */
    uint32_t core_mask;
    /* 不进入正式统计但仍执行完整校验的预热轮数。 */
    uint32_t warmup;
    /* 进入延迟和吞吐量统计的正式轮数。 */
    uint32_t rounds;
    /* baseline 或 mixed。 */
    priority_mode_t priority_mode;
} test_options_t;

/*
 * 主线程与所有工作线程共享的轮次屏障。
 *
 * 工作线程先增加 ready 并等待 generation 改变；主线程确认所有线程就绪后
 * 同时放行。每轮只记录最早 ioctl 开始和最晚 ioctl 返回，形成并发 Submit
 * 活跃窗口，结果校验和下一轮屏障时间不会计入吞吐量分母。
 */
typedef struct {
    /* 保护该结构内全部计数器和时间边界。 */
    pthread_mutex_t mutex;
    /* 同时用于“全部就绪”和“全部完成”两个条件。 */
    pthread_cond_t cond;
    /* 本轮必须到达的工作线程数量。 */
    uint32_t expected;
    /* 已进入本轮等待点的线程数量。 */
    uint32_t ready;
    /* 本轮已经完成 Submit 和结果校验的线程数量。 */
    uint32_t done;
    /* 主线程每放行一轮增加一次，避免条件变量丢失唤醒。 */
    uint32_t generation;
    /* 本轮 ioctl 成功返回的 Submit 数量。 */
    uint32_t completed_submits;
    /* 本轮所有 blocking ioctl 中最早的开始时间。 */
    uint64_t first_submit_us;
    /* 本轮所有 blocking ioctl 中最晚的返回时间。 */
    uint64_t last_submit_us;
    /* 任一线程失败或主线程结束时置 1，唤醒全部等待者退出。 */
    int stop;
} round_gate_t;

/* 一个工作线程的固定配置、负载所有权和测量结果。 */
typedef struct {
    /* 线程编号，同时参与 op_idx 和输入矩阵生成。 */
    uint32_t thread_id;
    /* 所有 worker 共享的 DRM fd。 */
    int fd;
    /* 该线程每轮写入 Submit 的 priority。 */
    int32_t priority;
    /* 预热轮数。 */
    uint32_t warmup;
    /* 正式测量轮数。 */
    uint32_t rounds;
    /* 指向进程级轮次屏障。 */
    round_gate_t *gate;
    /* 该线程独占的 DMA 和 Submit 负载。 */
    rknpu_batch_workload_t *workload;
    /* 仅保存正式轮 blocking ioctl 延迟，单位为微秒。 */
    uint64_t *latency_us;
    /* 已写入 latency_us 的样本数。 */
    size_t latency_count;
    /* 包含预热在内成功完成并校验的 Submit 数量。 */
    uint32_t completed;
    /* 首次失败发生的零基轮次。 */
    uint32_t error_round;
    /* ioctl 失败时保存 errno。 */
    int error_errno;
    /* 结果校验失败时保存首个具体错误。 */
    rknpu_batch_check_t check;
} worker_t;

/* 根据调度事件为每个 queue_task 汇总的完整性状态。 */
typedef struct {
    /* 驱动分配的 Submit 唯一编号。 */
    uint64_t queue_task;
    /* Enqueue 事件携带的 Submit 优先级。 */
    int32_t priority;
    /* Submit 进入调度队列的 t1。 */
    uint64_t enqueue_ns;
    /* 该 Submit 第一个 Task 的 Dispatch 时间。 */
    uint64_t first_dispatch_ns;
    /* 从首个 op_idx 还原出的工作线程编号。 */
    uint32_t thread_id;
    /* 该线程的第几个 Submit，用于排除预热轮。 */
    uint32_t round_index;
    /* 已观察到的 Dispatch 事件数量。 */
    uint32_t dispatch_count;
    /* 已观察到的 Complete 事件数量。 */
    uint32_t complete_count;
    /* task_index 位图，用于发现重复或漏下发。 */
    uint32_t dispatch_bitmap;
    /* task_index 位图，用于发现重复或漏完成。 */
    uint32_t complete_bitmap;
} submit_observation_t;

/* 读取用户态 CLOCK_MONOTONIC，作为 blocking ioctl 外部计时边界。 */
static uint64_t now_us(void) {
    struct timespec ts;

    clock_gettime(CLOCK_MONOTONIC, &ts);
    return (uint64_t)ts.tv_sec * 1000000ULL + (uint64_t)ts.tv_nsec / 1000ULL;
}

/* 清空驱动中上一场实验的 t0～t4 Submit 记录。 */
static int reset_submit_trace(int fd) {
    struct rknpu_submit_trace_query query;

    memset(&query, 0, sizeof(query));
    query.operation = RKNPU_SUBMIT_TRACE_RESET;
    return ioctl(fd, DRM_IOCTL_RKNPU_SUBMIT_TRACE, &query);
}

/* 清空调度事件并开启 Enqueue、Dispatch、Complete、Failed 全部事件。 */
static int configure_schedule_trace(int fd) {
    struct rknpu_schedule_trace_query query;

    memset(&query, 0, sizeof(query));
    query.operation = RKNPU_SCHEDULE_TRACE_CONFIG_RESET;
    query.event_mask = RKNPU_SCHEDULE_EVENT_ALL;
    return ioctl(fd, DRM_IOCTL_RKNPU_SCHEDULE_TRACE, &query);
}

/*
 * 一次性读取 t0～t4 记录。records 由调用者分配，query 返回实际数量和
 * overflowed；该函数不清空驱动记录，便于失败后重复读取定位。
 */
static int read_submit_trace(
    int fd,
    struct rknpu_submit_trace_record *records,
    uint32_t capacity,
    struct rknpu_submit_trace_query *query
) {
    memset(query, 0, sizeof(*query));
    query->operation = RKNPU_SUBMIT_TRACE_READ;
    query->capacity = capacity;
    query->records_address = (uint64_t)(uintptr_t)records;
    return ioctl(fd, DRM_IOCTL_RKNPU_SUBMIT_TRACE, query);
}

/* 一次性读取调度事件原始数组，容量单位为记录条数。 */
static int read_schedule_trace(
    int fd,
    struct rknpu_schedule_trace_record *records,
    uint32_t capacity,
    struct rknpu_schedule_trace_query *query
) {
    memset(query, 0, sizeof(*query));
    query->operation = RKNPU_SCHEDULE_TRACE_READ;
    query->capacity = capacity;
    query->records_address = (uint64_t)(uintptr_t)records;
    return ioctl(fd, DRM_IOCTL_RKNPU_SCHEDULE_TRACE, query);
}

/*
 * 获取调度器和 GEM 的瞬时资源计数。
 * 主程序分别在分配前、Submit 全部结束后和 GEM 释放后读取，用于检查泄漏。
 */
static int read_driver_state(
    int fd,
    struct rknpu_scheduler_state_snapshot *state
) {
    struct rknpu_schedule_trace_query query;

    memset(state, 0, sizeof(*state));
    memset(&query, 0, sizeof(query));
    query.operation = RKNPU_SCHEDULE_TRACE_STATE;
    query.state_address = (uint64_t)(uintptr_t)state;
    return ioctl(fd, DRM_IOCTL_RKNPU_SCHEDULE_TRACE, &query);
}

/* 初始化轮次屏障；条件变量初始化失败时立即回收已创建的互斥锁。 */
static int round_gate_init(round_gate_t *gate, uint32_t expected) {
    memset(gate, 0, sizeof(*gate));
    gate->expected = expected;
    if (pthread_mutex_init(&gate->mutex, NULL) != 0) {
        return -1;
    }
    if (pthread_cond_init(&gate->cond, NULL) != 0) {
        pthread_mutex_destroy(&gate->mutex);
        return -1;
    }
    return 0;
}

/* 设置停止标志并唤醒所有等待者，供错误路径和正常结束共同使用。 */
static void round_gate_stop(round_gate_t *gate) {
    pthread_mutex_lock(&gate->mutex);
    gate->stop = 1;
    pthread_cond_broadcast(&gate->cond);
    pthread_mutex_unlock(&gate->mutex);
}

/*
 * 工作线程声明本轮已就绪，并等待主线程增加 generation。
 * 条件检查放在 while 中，能够正确处理虚假唤醒和停止请求。
 */
static int worker_wait_round(round_gate_t *gate) {
    uint32_t generation;
    int run;

    pthread_mutex_lock(&gate->mutex);
    generation = gate->generation;
    gate->ready++;
    pthread_cond_broadcast(&gate->cond);
    while (!gate->stop && generation == gate->generation) {
        pthread_cond_wait(&gate->cond, &gate->mutex);
    }
    run = !gate->stop;
    pthread_mutex_unlock(&gate->mutex);
    return run;
}

/*
 * 工作线程提交本轮结果。
 * 只有 ioctl 成功返回才扩展 Submit 活跃窗口；无论成功失败都增加 done，
 * 确保主线程不会因某个 worker 已结束而永久等待。
 */
static void worker_finish_round(
    round_gate_t *gate,
    uint64_t start_us,
    uint64_t end_us,
    int submit_completed
) {
    pthread_mutex_lock(&gate->mutex);
    if (submit_completed) {
        if (gate->completed_submits == 0U || start_us < gate->first_submit_us) {
            gate->first_submit_us = start_us;
        }
        if (gate->completed_submits == 0U || end_us > gate->last_submit_us) {
            gate->last_submit_us = end_us;
        }
        gate->completed_submits++;
    }
    gate->done++;
    pthread_cond_broadcast(&gate->cond);
    pthread_mutex_unlock(&gate->mutex);
}

/*
 * 主线程等待所有 worker 就绪后同时放行，再等待所有 worker 完成本轮。
 * 返回的 active_us 是本轮最早 ioctl 开始到最晚 ioctl 返回，不包含线程
 * 创建、屏障等待和结果校验后的空闲时间。
 */
static int run_round(
    round_gate_t *gate,
    uint64_t *active_us,
    uint32_t *completed_submits
) {
    pthread_mutex_lock(&gate->mutex);
    while (!gate->stop && gate->ready < gate->expected) {
        pthread_cond_wait(&gate->cond, &gate->mutex);
    }
    if (gate->stop) {
        pthread_mutex_unlock(&gate->mutex);
        return -1;
    }

    gate->ready = 0;
    gate->done = 0;
    gate->completed_submits = 0;
    gate->first_submit_us = 0;
    gate->last_submit_us = 0;
    gate->generation++;
    pthread_cond_broadcast(&gate->cond);
    while (!gate->stop && gate->done < gate->expected) {
        pthread_cond_wait(&gate->cond, &gate->mutex);
    }

    *completed_submits = gate->completed_submits;
    *active_us = gate->completed_submits > 0U
        ? gate->last_submit_us - gate->first_submit_us
        : 0;
    if (gate->stop) {
        pthread_mutex_unlock(&gate->mutex);
        return -1;
    }
    pthread_mutex_unlock(&gate->mutex);
    return 0;
}

/*
 * 执行工作线程的一次 blocking Submit。
 *
 * 计时只包围 ioctl；返回后立即校验 task_counter、全部 IRQ 和矩阵抽样值。
 * 任一步失败都会保存错误上下文、完成本轮计数并停止全部线程。
 */
static int run_worker_submit(worker_t *worker, uint32_t round) {
    struct rknpu_submit *submit =
        rknpu_batch_begin(worker->workload, worker->priority);
    uint64_t start_us = now_us();
    uint64_t end_us;

    if (ioctl(worker->fd, DRM_IOCTL_RKNPU_SUBMIT, submit) < 0) {
        worker->error_errno = errno;
        worker->error_round = round;
        worker_finish_round(worker->gate, start_us, start_us, 0);
        round_gate_stop(worker->gate);
        return -1;
    }
    end_us = now_us();
    if (rknpu_batch_check(worker->workload, submit, &worker->check) != 0) {
        worker->error_round = round;
        worker_finish_round(worker->gate, start_us, end_us, 1);
        round_gate_stop(worker->gate);
        return -1;
    }

    worker->completed++;
    if (round >= worker->warmup) {
        worker->latency_us[worker->latency_count++] = end_us - start_us;
    }
    worker_finish_round(worker->gate, start_us, end_us, 1);
    return 0;
}

/* 按“预热+正式轮数”重复等待屏障和执行 Submit。 */
static void *worker_main(void *arg) {
    worker_t *worker = arg;
    uint32_t total_rounds = worker->warmup + worker->rounds;

    for (uint32_t round = 0; round < total_rounds; round++) {
        if (!worker_wait_round(worker->gate)) {
            break;
        }
        if (run_worker_submit(worker, round) != 0) {
            break;
        }
    }
    return NULL;
}

/* 根据实验模式为线程分配固定优先级，保证 3/6 线程结果可重复比较。 */
static int32_t worker_priority(priority_mode_t mode, uint32_t threads, uint32_t id) {
    if (mode == PRIORITY_BASELINE) {
        return 0;
    }
    if (threads == 3U) {
        static const int32_t priorities[3] = {-10, 0, 10};
        return priorities[id];
    }
    {
        static const int32_t priorities[6] = {-10, -10, 0, 0, 10, 10};
        return priorities[id];
    }
}

/* 在已建立的 Submit 观察数组中按 queue_task 查找对应项。 */
static submit_observation_t *find_observation(
    submit_observation_t *observations,
    size_t count,
    uint64_t queue_task
) {
    for (size_t index = 0; index < count; index++) {
        if (observations[index].queue_task == queue_task) {
            return &observations[index];
        }
    }
    return NULL;
}

/*
 * 校验完整调度事件流并汇总每个 Submit。
 *
 * 第一遍只建立 queue_task 映射，因为多线程下 Enqueue 记录写入 trace 的顺序
 * 可能与 worker 的 Dispatch 记录交错；第二遍再检查每个 Task 的下发与完成。
 *
 * 核心判定：
 * - sequence 必须连续，证明固定缓冲区没有覆盖或缺失；
 * - 每个 Submit 必须恰有 10 个不同 task_index 的 Dispatch 和 Complete；
 * - Complete 必须对应已经 Dispatch 的 Task；
 * - READY 来源的 priority 必须等于当时记录的最高 Ready priority；
 * - mixed 场景必须实际观察到 Running 压住更高优先级 Ready，否则本次运行
 *   没有覆盖目标竞争关系，不能作为该策略的板端证据；
 * - 任何 FAILED 事件、重复事件或未知来源都使测试失败。
 */
static int analyze_schedule_trace(
    const struct rknpu_schedule_trace_record *records,
    size_t record_count,
    const test_options_t *options,
    submit_observation_t *observations,
    size_t expected_submits
) {
    uint32_t thread_rounds[MAX_THREADS] = {0};
    size_t observation_count = 0;
    size_t ready_decisions = 0;
    size_t running_decisions = 0;
    size_t running_over_higher_ready = 0;
    size_t priority_violations = 0;
    size_t failed_events = 0;

    /* 先建立 queue_task 映射，允许并发记录的入队与下发事件交错。 */
    for (size_t index = 0; index < record_count; index++) {
        const struct rknpu_schedule_trace_record *record = &records[index];
        submit_observation_t *observation;

        if (record->sequence != index || record->queue_task == 0U) {
            fprintf(stderr, "invalid schedule sequence at index=%zu\n", index);
            return -1;
        }
        if (record->event_type != RKNPU_SCHEDULE_EVENT_ENQUEUE) {
            continue;
        }
        {
            uint32_t thread_id = record->op_idx / RKNPU_BATCH_TASKS;
            if (observation_count >= expected_submits ||
                thread_id >= options->threads ||
                find_observation(observations, observation_count,
                                 record->queue_task) != NULL) {
                fprintf(stderr, "invalid enqueue event at index=%zu\n", index);
                return -1;
            }
            observation = &observations[observation_count++];
            observation->queue_task = record->queue_task;
            observation->priority = record->priority;
            observation->enqueue_ns = record->timestamp_ns;
            observation->thread_id = thread_id;
            observation->round_index = thread_rounds[thread_id]++;
        }
    }

    for (size_t index = 0; index < record_count; index++) {
        const struct rknpu_schedule_trace_record *record = &records[index];
        submit_observation_t *observation;

        if (record->event_type == RKNPU_SCHEDULE_EVENT_ENQUEUE) {
            continue;
        }

        observation = find_observation(observations, observation_count,
                                       record->queue_task);
        if (observation == NULL || observation->priority != record->priority ||
            record->task_index >= RKNPU_BATCH_TASKS ||
            record->op_idx % RKNPU_BATCH_TASKS != record->task_index ||
            record->core_slot >= options->cores) {
            fprintf(stderr, "unmatched schedule event at index=%zu\n", index);
            return -1;
        }

        if (record->event_type == RKNPU_SCHEDULE_EVENT_DISPATCH) {
            uint32_t bit = 1U << record->task_index;
            if (observation->dispatch_bitmap & bit) {
                fprintf(stderr, "duplicate dispatch queue_task=%llu task=%u\n",
                        (unsigned long long)record->queue_task, record->task_index);
                return -1;
            }
            observation->dispatch_bitmap |= bit;
            observation->dispatch_count++;
            if (observation->first_dispatch_ns == 0U) {
                observation->first_dispatch_ns = record->timestamp_ns;
            }
            if (record->dispatch_source == RKNPU_DISPATCH_SOURCE_READY) {
                /* Ready 选择时，实际 Submit 必须就是该核心的最高优先级候选。 */
                ready_decisions++;
                if (record->ready_priority != record->priority) {
                    priority_violations++;
                }
            } else if (record->dispatch_source == RKNPU_DISPATCH_SOURCE_RUNNING) {
                /* 该计数直接证明 Running-first 是否让更高优先级 Ready 等待。 */
                running_decisions++;
                if (record->ready_priority < record->priority) {
                    running_over_higher_ready++;
                }
            } else {
                fprintf(stderr, "invalid dispatch source at index=%zu\n", index);
                return -1;
            }
        } else if (record->event_type == RKNPU_SCHEDULE_EVENT_COMPLETE) {
            uint32_t bit = 1U << record->task_index;
            if ((observation->dispatch_bitmap & bit) == 0U ||
                (observation->complete_bitmap & bit) != 0U) {
                fprintf(stderr, "invalid completion queue_task=%llu task=%u\n",
                        (unsigned long long)record->queue_task, record->task_index);
                return -1;
            }
            observation->complete_bitmap |= bit;
            observation->complete_count++;
        } else if (record->event_type == RKNPU_SCHEDULE_EVENT_FAILED) {
            failed_events++;
        } else {
            fprintf(stderr, "unknown schedule event at index=%zu\n", index);
            return -1;
        }
    }

    /* mixed 必须真正覆盖 Running 与更高优先级 Ready 的同时竞争。 */
    if (observation_count != expected_submits || failed_events != 0U ||
        priority_violations != 0U || ready_decisions < expected_submits ||
        (options->priority_mode == PRIORITY_MIXED &&
         running_over_higher_ready == 0U)) {
        fprintf(stderr,
                "schedule summary mismatch: submits=%zu/%zu failures=%zu "
                "ready=%zu running_over_high=%zu violations=%zu\n",
                observation_count, expected_submits, failed_events, ready_decisions,
                running_over_higher_ready, priority_violations);
        return -1;
    }
    /* 0x3ff 表示下标 0～9 的十个 Task 每个都恰好出现一次。 */
    for (size_t index = 0; index < observation_count; index++) {
        const submit_observation_t *observation = &observations[index];
        if (observation->dispatch_count != RKNPU_BATCH_TASKS ||
            observation->complete_count != RKNPU_BATCH_TASKS ||
            observation->dispatch_bitmap != 0x3ffU ||
            observation->complete_bitmap != 0x3ffU ||
            observation->first_dispatch_ns < observation->enqueue_ns) {
            fprintf(stderr, "incomplete queue_task=%llu dispatch=%u complete=%u\n",
                    (unsigned long long)observation->queue_task,
                    observation->dispatch_count, observation->complete_count);
            return -1;
        }
    }

    printf("schedule decisions\n");
    printf("  Ready dispatches                 : %zu\n", ready_decisions);
    printf("  Running dispatches               : %zu\n", running_decisions);
    printf("  Running over higher Ready        : %zu\n",
           running_over_higher_ready);
    printf("  Ready priority violations        : %zu\n", priority_violations);
    printf("  matched task completions         : %zu/%zu\n",
           observation_count * RKNPU_BATCH_TASKS,
           expected_submits * RKNPU_BATCH_TASKS);
    return 0;
}

/* 只读版本的 queue_task 查找，用于统计阶段关联 Submit 属性。 */
static const submit_observation_t *find_observation_const(
    const submit_observation_t *observations,
    size_t count,
    uint64_t queue_task
) {
    for (size_t index = 0; index < count; index++) {
        if (observations[index].queue_task == queue_task) {
            return &observations[index];
        }
    }
    return NULL;
}

/*
 * 从一条合法 t0～t4 记录中取指定阶段的纳秒差值：
 * 0=提交准备，1=排队等待，2=派发与执行，3=完成返回。
 */
static uint64_t phase_ns(
    const struct rknpu_submit_trace_record *record,
    uint32_t phase
) {
    if (phase == 0U) {
        return record->t1_ns - record->t0_ns;
    }
    if (phase == 1U) {
        return record->t2_ns - record->t1_ns;
    }
    if (phase == 2U) {
        return record->t3_ns - record->t2_ns;
    }
    return record->t4_ns - record->t3_ns;
}

/*
 * 统计某一阶段、某一优先级的正式轮样本。
 *
 * 先通过 queue_task 找到线程轮次并排除预热，再验证时间戳单调，最后复用
 * 项目现有 benchmark_stats 计算 Mean/P50/P95/P99。输入保持纳秒，打印时
 * 统一换算为毫秒，避免在保存样本时提前截断精度。
 */
static int print_phase_group(
    const char *phase_name,
    uint32_t phase,
    int32_t priority,
    const struct rknpu_submit_trace_record *records,
    size_t record_count,
    const submit_observation_t *observations,
    size_t observation_count,
    uint32_t warmup
) {
    uint64_t *samples = malloc(record_count * sizeof(*samples));
    benchmark_latency_stats_t stats;
    size_t count = 0;

    if (samples == NULL) {
        return -1;
    }
    for (size_t index = 0; index < record_count; index++) {
        const struct rknpu_submit_trace_record *record = &records[index];
        const submit_observation_t *observation = find_observation_const(
            observations, observation_count, record->queue_task);
        if (observation == NULL ||
            observation->round_index < warmup ||
            observation->priority != priority) {
            continue;
        }
        if (record->t0_ns > record->t1_ns || record->t1_ns > record->t2_ns ||
            record->t2_ns > record->t3_ns || record->t3_ns > record->t4_ns) {
            free(samples);
            return -1;
        }
        samples[count++] = phase_ns(record, phase);
    }
    if (count == 0U || benchmark_compute_latency_stats(samples, count, &stats) != 0) {
        free(samples);
        return -1;
    }
    printf("  %-17s priority=%3d  mean=%9.6f ms  P50=%9.6f  P95=%9.6f  P99=%9.6f\n",
           phase_name, priority, stats.mean_us / 1000000.0,
           (double)stats.p50_us / 1000000.0,
           (double)stats.p95_us / 1000000.0,
           (double)stats.p99_us / 1000000.0);
    free(samples);
    return 0;
}

/*
 * 按优先级输出四阶段延迟。baseline 只输出 priority=0；mixed 分别输出
 * -10、0、10，直接比较高优先级 Ready 是否在 queue_wait 中持续等待。
 */
static int report_four_phases(
    const struct rknpu_submit_trace_record *records,
    size_t record_count,
    const submit_observation_t *observations,
    size_t observation_count,
    const test_options_t *options
) {
    static const char *phase_names[4] = {
        "submit_prepare", "queue_wait", "dispatch_execute", "complete_return"
    };
    static const int32_t mixed_priorities[3] = {-10, 0, 10};
    const int32_t baseline_priority = 0;
    const int32_t *priorities = options->priority_mode == PRIORITY_MIXED
        ? mixed_priorities
        : &baseline_priority;
    size_t priority_count = options->priority_mode == PRIORITY_MIXED ? 3U : 1U;

    printf("four-stage latency\n");
    for (uint32_t phase = 0; phase < 4U; phase++) {
        for (size_t priority_index = 0; priority_index < priority_count;
             priority_index++) {
            if (print_phase_group(
                    phase_names[phase], phase, priorities[priority_index], records,
                    record_count, observations, observation_count, options->warmup
                ) != 0) {
                return -1;
            }
        }
    }
    return 0;
}

/* 严格解析无符号十进制参数，拒绝空字符串、尾随字符、溢出和 errno 错误。 */
static int parse_u32(const char *text, uint32_t *value) {
    char *end;
    unsigned long parsed;

    errno = 0;
    parsed = strtoul(text, &end, 10);
    if (errno != 0 || end == text || *end != '\0' || parsed > UINT32_MAX) {
        return -1;
    }
    *value = (uint32_t)parsed;
    return 0;
}

/* 打印单场景运行参数；完整实验矩阵由 run_priority_matrix.sh 组合。 */
static void print_usage(const char *program) {
    printf("Usage: %s [options]\n", program);
    printf("  --threads <3|6>\n");
    printf("  --cores <1|2|3>\n");
    printf("  --priority <baseline|mixed>\n");
    printf("  --warmup <count>   default: %u\n", DEFAULT_WARMUP);
    printf("  --rounds <count>   default: %u\n", DEFAULT_ROUNDS);
}

/*
 * 解析并约束实验参数。
 *
 * 线程数只允许 3/6，核心数只允许 1/2/3，防止运行结果偏离预先定义的
 * 对照矩阵。最后检查 warmup+rounds 加法不会回绕，并生成连续核心掩码。
 */
static int parse_options(int argc, char **argv, test_options_t *options) {
    options->threads = DEFAULT_THREADS;
    options->cores = DEFAULT_CORES;
    options->warmup = DEFAULT_WARMUP;
    options->rounds = DEFAULT_ROUNDS;
    options->priority_mode = PRIORITY_MIXED;

    for (int index = 1; index < argc; index++) {
        const char *name = argv[index];
        if (strcmp(name, "--help") == 0) {
            print_usage(argv[0]);
            return 1;
        }
        if (++index >= argc) {
            return -1;
        }
        if (strcmp(name, "--threads") == 0) {
            if (parse_u32(argv[index], &options->threads) != 0 ||
                (options->threads != 3U && options->threads != 6U)) {
                return -1;
            }
        } else if (strcmp(name, "--cores") == 0) {
            if (parse_u32(argv[index], &options->cores) != 0 ||
                options->cores == 0U || options->cores > 3U) {
                return -1;
            }
        } else if (strcmp(name, "--warmup") == 0) {
            if (parse_u32(argv[index], &options->warmup) != 0) {
                return -1;
            }
        } else if (strcmp(name, "--rounds") == 0) {
            if (parse_u32(argv[index], &options->rounds) != 0 ||
                options->rounds == 0U) {
                return -1;
            }
        } else if (strcmp(name, "--priority") == 0) {
            if (strcmp(argv[index], "baseline") == 0) {
                options->priority_mode = PRIORITY_BASELINE;
            } else if (strcmp(argv[index], "mixed") == 0) {
                options->priority_mode = PRIORITY_MIXED;
            } else {
                return -1;
            }
        } else {
            return -1;
        }
    }
    options->core_mask = (1U << options->cores) - 1U;
    if (UINT32_MAX - options->warmup < options->rounds) {
        return -1;
    }
    return 0;
}

/* 按 ioctl 错误或负载校验错误打印首个失败点，不在成功路径逐轮输出。 */
static void print_worker_error(const worker_t *worker) {
    if (worker->error_errno != 0) {
        fprintf(stderr, "thread[%u] ioctl failed round=%u errno=%d (%s)\n",
                worker->thread_id, worker->error_round + 1U,
                worker->error_errno, strerror(worker->error_errno));
        return;
    }
    if (worker->check.error != RKNPU_BATCH_OK) {
        fprintf(stderr,
                "thread[%u] validation failed round=%u type=%d task=%u value=0x%x expected=%f actual=%f\n",
                worker->thread_id, worker->error_round + 1U, worker->check.error,
                worker->check.task_index, worker->check.actual_u32,
                worker->check.expected, worker->check.actual);
    }
}

/*
 * 单场景入口。
 *
 * 资源所有权顺序为 fd -> gate -> worker workload -> trace 数组。任何错误都
 * 汇合到 cleanup；已经创建的线程先 stop/join，再销毁它们可能仍访问的负载。
 * 只有调度、计算、统计和两次资源基线检查全部成功才把 result 改为 0。
 */
int main(int argc, char **argv) {
    test_options_t options;
    worker_t *workers = NULL;
    pthread_t *threads = NULL;
    round_gate_t gate;
    struct rknpu_submit_trace_record *submit_records = NULL;
    struct rknpu_schedule_trace_record *schedule_records = NULL;
    submit_observation_t *observations = NULL;
    struct rknpu_submit_trace_query submit_query;
    struct rknpu_schedule_trace_query schedule_query;
    struct rknpu_scheduler_state_snapshot initial_state;
    struct rknpu_scheduler_state_snapshot final_state;
    uint64_t measured_active_us = 0;
    uint64_t measured_tasks = 0;
    uint32_t total_rounds;
    size_t expected_submits;
    size_t expected_events;
    uint32_t created_threads = 0;
    int fd = -1;
    int gate_ready = 0;
    int result = 1;
    int parsed = parse_options(argc, argv, &options);

    if (parsed != 0) {
        if (parsed < 0) {
            print_usage(argv[0]);
        }
        return parsed > 0 ? 0 : 2;
    }
    /*
     * 每个 Submit 产生 1 条 Enqueue、10 条 Dispatch 和 10 条 Complete。
     * 在启动线程前验证两类 trace 都能容纳完整实验，禁止带溢出的样本运行。
     */
    total_rounds = options.warmup + options.rounds;
    expected_submits = (size_t)options.threads * total_rounds;
    expected_events = expected_submits * (1U + 2U * RKNPU_BATCH_TASKS);
    if (expected_submits > RKNPU_SUBMIT_TRACE_CAPACITY ||
        expected_events > RKNPU_SCHEDULE_TRACE_CAPACITY) {
        fprintf(stderr, "trace capacity exceeded: submits=%zu events=%zu\n",
                expected_submits, expected_events);
        return 2;
    }

    /* 在任何测试 GEM 分配前保存资源基线，并清空两类驱动记录。 */
    fd = npu_open();
    if (fd < 0 || npu_reset(fd) < 0 || read_driver_state(fd, &initial_state) < 0 ||
        reset_submit_trace(fd) < 0 || configure_schedule_trace(fd) < 0) {
        fprintf(stderr, "driver setup failed: errno=%d (%s)\n", errno, strerror(errno));
        goto cleanup;
    }

    /* 每个 worker 有独立负载和延迟数组，但 gate 与 fd 由进程共享。 */
    workers = calloc(options.threads, sizeof(*workers));
    threads = calloc(options.threads, sizeof(*threads));
    if (workers == NULL || threads == NULL ||
        round_gate_init(&gate, options.threads) != 0) {
        goto cleanup;
    }
    gate_ready = 1;

    /* 先完成所有 DMA 和命令准备，确保分配、建模时间不进入测量窗口。 */
    for (uint32_t index = 0; index < options.threads; index++) {
        worker_t *worker = &workers[index];
        worker->thread_id = index;
        worker->fd = fd;
        worker->priority = worker_priority(options.priority_mode, options.threads, index);
        worker->warmup = options.warmup;
        worker->rounds = options.rounds;
        worker->gate = &gate;
        worker->latency_us = calloc(options.rounds, sizeof(*worker->latency_us));
        if (worker->latency_us == NULL ||
            rknpu_batch_create(fd, index, options.cores, options.core_mask,
                               &worker->workload) != 0) {
            fprintf(stderr, "worker[%u] resource setup failed\n", index);
            goto stop_threads;
        }
    }

    printf("rknpu_priority_benchmark\n");
    printf("  shared fd       : yes\n");
    printf("  threads         : %u\n", options.threads);
    printf("  NPU cores       : %u (mask=0x%x)\n", options.cores, options.core_mask);
    printf("  priority mode   : %s\n",
           options.priority_mode == PRIORITY_MIXED ? "mixed" : "baseline");
    printf("  tasks/Submit    : %u, lane split=%s\n", RKNPU_BATCH_TASKS,
           options.cores == 1U ? "10" : (options.cores == 2U ? "5+5" : "4+3+3"));
    printf("  rounds          : %u warmup + %u measured\n",
           options.warmup, options.rounds);

    /* 所有资源就绪后再创建线程，避免部分线程提前开始第一轮。 */
    for (uint32_t index = 0; index < options.threads; index++) {
        int error = pthread_create(&threads[index], NULL, worker_main, &workers[index]);
        if (error != 0) {
            fprintf(stderr, "pthread_create failed: %s\n", strerror(error));
            goto stop_threads;
        }
        created_threads++;
    }

    /* 主线程逐轮统一放行；仅正式轮累加并发 ioctl 活跃时间和完成 Task 数。 */
    for (uint32_t round = 0; round < total_rounds; round++) {
        uint64_t active_us;
        uint32_t completed_submits;
        if (run_round(&gate, &active_us, &completed_submits) != 0) {
            goto stop_threads;
        }
        if (round >= options.warmup) {
            measured_active_us += active_us;
            measured_tasks += (uint64_t)completed_submits * RKNPU_BATCH_TASKS;
        }
    }

stop_threads:
    /* 正常结束和错误路径都先唤醒、join 线程，之后才能读取稳定 trace。 */
    if (gate_ready) {
        round_gate_stop(&gate);
    }
    for (uint32_t index = 0; index < created_threads; index++) {
        pthread_join(threads[index], NULL);
    }
    if (created_threads != options.threads) {
        goto cleanup;
    }
    for (uint32_t index = 0; index < options.threads; index++) {
        if (workers[index].completed != total_rounds ||
            workers[index].latency_count != options.rounds) {
            print_worker_error(&workers[index]);
            goto cleanup;
        }
    }

    /* 按预先计算的精确数量分配用户数组，并一次性读取两类原始记录。 */
    submit_records = calloc(expected_submits, sizeof(*submit_records));
    schedule_records = calloc(expected_events, sizeof(*schedule_records));
    observations = calloc(expected_submits, sizeof(*observations));
    if (submit_records == NULL || schedule_records == NULL || observations == NULL ||
        read_submit_trace(fd, submit_records, (uint32_t)expected_submits,
                          &submit_query) < 0 ||
        read_schedule_trace(fd, schedule_records, (uint32_t)expected_events,
                            &schedule_query) < 0) {
        fprintf(stderr, "trace read failed: errno=%d (%s)\n", errno, strerror(errno));
        goto cleanup;
    }
    if (submit_query.count != expected_submits || submit_query.overflowed != 0U ||
        schedule_query.count != expected_events || schedule_query.overflowed != 0U) {
        fprintf(stderr,
                "trace incomplete: submit=%u/%zu schedule=%u/%zu overflow=%u/%u\n",
                submit_query.count, expected_submits, schedule_query.count,
                expected_events, submit_query.overflowed, schedule_query.overflowed);
        goto cleanup;
    }
    /* 调度正确性先通过，四阶段样本才具有可解释性。 */
    if (analyze_schedule_trace(schedule_records, schedule_query.count, &options,
                               observations, expected_submits) != 0 ||
        report_four_phases(submit_records, submit_query.count, observations,
                           expected_submits, &options) != 0) {
        goto cleanup;
    }

    {
        /* 汇总所有线程正式轮的用户态 blocking ioctl 延迟。 */
        uint64_t *samples = malloc((size_t)options.threads * options.rounds *
                                   sizeof(*samples));
        benchmark_latency_stats_t stats;
        size_t count = 0;
        if (samples == NULL) {
            goto cleanup;
        }
        for (uint32_t index = 0; index < options.threads; index++) {
            memcpy(&samples[count], workers[index].latency_us,
                   workers[index].latency_count * sizeof(*samples));
            count += workers[index].latency_count;
        }
        if (benchmark_compute_latency_stats(samples, count, &stats) != 0) {
            free(samples);
            goto cleanup;
        }
        printf("blocking Submit latency\n");
        printf("  samples=%zu mean=%.3f ms P50=%.3f P95=%.3f P99=%.3f\n",
               count, stats.mean_us / 1000.0, (double)stats.p50_us / 1000.0,
               (double)stats.p95_us / 1000.0, (double)stats.p99_us / 1000.0);
        free(samples);
    }
    if (measured_active_us == 0U) {
        goto cleanup;
    }
    /* 吞吐量分母是各正式轮 Submit 活跃窗口之和，不包含屏障和结果校验。 */
    printf("throughput\n");
    printf("  tasks/s   : %.2f\n",
           (double)measured_tasks * 1000000.0 / (double)measured_active_us);
    printf("  Submits/s : %.2f\n",
           ((double)measured_tasks / RKNPU_BATCH_TASKS) * 1000000.0 /
               (double)measured_active_us);

    /* blocking ioctl 全部返回后，所有调度容器、waiter 和核心绑定必须为空。 */
    if (read_driver_state(fd, &final_state) < 0 ||
        final_state.live_submits != 0U || final_state.ready_entries != 0U ||
        final_state.running_entries != 0U || final_state.complete_entries != 0U ||
        final_state.waiters != 0U || final_state.core_bindings != 0U) {
        fprintf(stderr, "scheduler state was not released after test\n");
        goto cleanup;
    }

    /* 主动释放测试 GEM，再与进程启动时的驱动资源基线比较。 */
    for (uint32_t index = 0; index < options.threads; index++) {
        rknpu_batch_destroy(fd, workers[index].workload);
        workers[index].workload = NULL;
    }
    if (read_driver_state(fd, &final_state) < 0 ||
        final_state.gem_buffers != initial_state.gem_buffers ||
        final_state.gem_bytes != initial_state.gem_bytes) {
        fprintf(stderr,
                "GEM state mismatch: buffers=%u/%u bytes=%llu/%llu\n",
                final_state.gem_buffers, initial_state.gem_buffers,
                (unsigned long long)final_state.gem_bytes,
                (unsigned long long)initial_state.gem_bytes);
        goto cleanup;
    }
    printf("resource state: scheduler empty, GEM returned to baseline\n");
    result = 0;

cleanup:
    /* workload 销毁允许 NULL，成功路径已释放的指针也已清零，避免重复销毁。 */
    if (workers != NULL) {
        for (uint32_t index = 0; index < options.threads; index++) {
            rknpu_batch_destroy(fd, workers[index].workload);
            free(workers[index].latency_us);
        }
    }
    if (gate_ready) {
        pthread_cond_destroy(&gate.cond);
        pthread_mutex_destroy(&gate.mutex);
    }
    free(observations);
    free(schedule_records);
    free(submit_records);
    free(threads);
    free(workers);
    if (fd >= 0) {
        npu_close(fd);
    }
    printf("rknpu_priority_benchmark: %s\n", result == 0 ? "PASS" : "FAIL");
    return result;
}
