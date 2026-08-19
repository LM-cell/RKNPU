/*
 * RKNPU 多线程核心数量与 Submit 四阶段延迟测试，最后修改日期：2026-08-17。
 *
 * 本程序固定共享设备 fd，通过 --cores 1|2|3 选择 core_mask。它复用
 * core_scaling_benchmark 的七个场景组合，并按当前核心数把每个 Submit 的 Task
 * 均匀分配到逻辑 lane。新增 --submit-limit K，在业务线程数固定时限制同时进入
 * blocking ioctl 的 Submit 数，用于 A/B 验证过量 live Submit 是否扩大调度 refill。
 *
 * 每个 worker 先完成预热，再只同步一次正式测量起点；正式阶段连续执行 blocking
 * Submit，不在轮次之间等待其他线程，也不在相邻 ioctl 之间执行结果校验。预热
 * Submit 完整校验，正式流水结束后只校验每个线程最后一次 Submit 的完成状态和
 * 抽样矩阵结果，校验过程不计入吞吐时间。
 *
 * 程序还在测试开始前复位驱动 Submit trace，结束后一次性读取 t0～t4 原始记录，
 * 校验记录数量和时间顺序，再统计提交准备、排队等待、派发执行、完成返回四阶段。
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
#include "rknpu-ioctl.h"
#include "rknpu_performance_report.h"
#include "rknpu_scenario_workload.h"

#define DEFAULT_THREADS 3U
#define DEFAULT_ROUNDS 100U
#define DEFAULT_WARMUP_ROUNDS 2U
#define DEFAULT_NPU_CORES 3U
#define MAX_NPU_CORES 3U
#define MAX_TEST_THREADS 64U
#define USER_PAGE_SIZE 4096U

/* 编译期固定用户态和驱动共享结构尺寸，避免 ioctl 大小或字段偏移失配。 */
_Static_assert(sizeof(struct rknpu_task) == 40, "RKNPU task ABI must be 40 bytes");
_Static_assert(sizeof(struct rknpu_submit) == 104, "RKNPU submit ABI must be 104 bytes");
_Static_assert(sizeof(struct rknpu_submit_trace_record) == 48,
               "StarryOS submit trace record ABI must be 48 bytes");
_Static_assert(sizeof(struct rknpu_submit_trace_query) == 24,
               "StarryOS submit trace query ABI must be 24 bytes");
_Static_assert(sizeof(struct rknpu_schedule_trace_record) == 56,
               "StarryOS schedule trace record ABI must be 56 bytes");
_Static_assert(sizeof(struct rknpu_schedule_trace_query) == 40,
               "StarryOS schedule trace query ABI must be 40 bytes");

/* 每核最多保存一个正在执行的 Task，用于严格配对 Dispatch 和 Complete。 */
typedef struct {
    int busy;
    uint64_t queue_task;
    uint32_t task_index;
    uint64_t dispatch_ns;
    uint64_t last_complete_ns;
    uint64_t busy_ns;
    uint64_t idle_gap_ns;
    uint64_t max_refill_gap_ns;
    size_t dispatch_count;
    size_t complete_count;
    size_t refill_gap_count;
} core_usage_t;

/* 三核同时忙的区间通过事件扫描得到，不根据 core_mask 推断。 */
typedef struct {
    uint64_t window_start_ns;
    uint64_t window_end_ns;
    uint64_t all_cores_busy_ns;
} all_core_usage_t;

/* 预热结束后只使用一次的正式测量启动门，同时记录全局测量区间。 */
typedef struct {
    /* 保护以下全部状态。 */
    pthread_mutex_t mutex;
    pthread_cond_t cond;
    /* 必须完成预热并到达启动门的 worker 数。 */
    uint32_t expected_threads;
    /* 已完成预热的 worker 数。 */
    uint32_t ready_threads;
    /* 主线程置位后，全部 worker 开始连续正式 Submit。 */
    int started;
    /* 已完成全部正式 Submit、等待统一离开测量区间的 worker 数。 */
    uint32_t finished_threads;
    /* 创建线程失败时终止尚未开始正式测量的 worker。 */
    int stop;
} stream_gate_t;

/*
 * 用户态 Submit 并发闸门。每个 permit 覆盖一次完整 blocking ioctl，
 * 因而限制的是同时进入驱动并保持 live 的 Submit 数，而不是业务线程数。
 */
typedef struct {
    pthread_mutex_t mutex;
    pthread_cond_t cond;
    uint32_t limit;
    uint32_t active;
    uint32_t peak_active;
} submit_gate_t;

/* 每个 worker 的首个失败分类。 */
typedef enum {
    WORKER_OK = 0,
    WORKER_IOCTL_FAILED,
    WORKER_TASK_COUNTER_FAILED,
    WORKER_IRQ_STATUS_FAILED,
    WORKER_OUTPUT_FAILED,
} worker_error_t;

typedef struct {
    /* 线程编号，用于生成该线程独有的矩阵内容和 op_idx。 */
    uint32_t thread_id;
    /* 所有 worker 共享的 /dev/dri/card1 fd。 */
    int fd;
    /* 本次实验允许使用的连续核心掩码。 */
    uint32_t core_mask;
    /* 预热轮和正式测量轮。 */
    uint32_t warmup_rounds;
    uint32_t measure_rounds;
    /* 预热结束后共用的一次性正式测量启动门。 */
    stream_gate_t *gate;
    /* 限制同时 live blocking Submit 数的全局并发闸门。 */
    submit_gate_t *submit_gate;
    /* 当前场景和该线程独占的 GEM、矩阵及 Submit 模板。 */
    const rknpu_scenario_case_t *scenario;
    rknpu_scenario_workload_t *workload;
    /* 正式轮 blocking ioctl 服务延迟，单位为微秒。 */
    uint64_t *latency_samples_us;
    /* 等待 Submit permit 的时间，以及 request=gate_wait+blocking ioctl 的端到端时间。 */
    uint64_t *gate_wait_samples_us;
    uint64_t *request_latency_samples_us;
    size_t latency_sample_count;
    /* 由主线程 join 后汇总为全局连续测量区间，不在热路径中获取共享锁。 */
    uint64_t first_measured_start_us;
    uint64_t last_measured_end_us;
    size_t successful_measured_submits;
    /* 预热校验通过或正式阶段成功返回的 Submit 数。 */
    uint32_t successful_submits;
    /* 以下字段保存首个失败及其现场。 */
    worker_error_t error;
    uint32_t error_round;
    int error_errno;
    uint32_t error_task_counter;
    uint32_t error_task_index;
    uint32_t error_irq_status;
    uint32_t error_row;
    uint32_t error_col;
    float error_expected;
    float error_actual;
} worker_context_t;

typedef struct {
    /* 并发业务线程数。 */
    uint32_t threads;
    /* 同时允许进入 blocking ioctl 的 Submit 上限；默认等于 threads。 */
    uint32_t submit_limit;
    /* 正式测量和预热轮数。 */
    uint32_t rounds;
    uint32_t warmup_rounds;
    /* --cores 的解析结果。 */
    uint32_t npu_cores;
    /* 从 core0 起连续启用：1/2/3 核对应 0x1/0x3/0x7。 */
    uint32_t core_mask;
    /* 完整场景名称或 all，默认执行全部七个组合。 */
    const char *scenario_filter;
} test_options_t;

/* 独立 Schedule Trace 窗口的线程输入，不复用正式性能样本和 PASS 计数。 */
typedef struct {
    int fd;
    uint32_t rounds;
    uint32_t task_count;
    stream_gate_t *gate;
    submit_gate_t *submit_gate;
    rknpu_scenario_workload_t *workload;
    int error_errno;
    uint32_t completed;
} usage_worker_t;

/* 用户态 blocking ioctl 使用 CLOCK_MONOTONIC 计时，单位为微秒。 */
static uint64_t now_us(void) {
    struct timespec ts;

    clock_gettime(CLOCK_MONOTONIC, &ts);
    return (uint64_t)ts.tv_sec * 1000000ULL + (uint64_t)ts.tv_nsec / 1000ULL;
}

/* 初始化 Submit 并发闸门；limit 必须在 1..threads 范围内。 */
static int submit_gate_init(submit_gate_t *gate, uint32_t limit) {
    memset(gate, 0, sizeof(*gate));
    gate->limit = limit;

    if (pthread_mutex_init(&gate->mutex, NULL) != 0) {
        return -1;
    }
    if (pthread_cond_init(&gate->cond, NULL) != 0) {
        pthread_mutex_destroy(&gate->mutex);
        return -1;
    }
    return 0;
}

/*
 * 获取一个 live-Submit permit。等待发生在用户态，不占用驱动 Scheduler 的
 * ready/running 状态；返回后调用方必须一直持有到 blocking ioctl 返回。
 */
static void submit_gate_acquire(submit_gate_t *gate) {
    pthread_mutex_lock(&gate->mutex);
    while (gate->active >= gate->limit) {
        pthread_cond_wait(&gate->cond, &gate->mutex);
    }
    gate->active++;
    if (gate->active > gate->peak_active) {
        gate->peak_active = gate->active;
    }
    pthread_mutex_unlock(&gate->mutex);
}

/* 释放 permit 并唤醒一个等待者，使 active 始终不超过 limit。 */
static void submit_gate_release(submit_gate_t *gate) {
    pthread_mutex_lock(&gate->mutex);
    if (gate->active > 0U) {
        gate->active--;
    }
    pthread_cond_signal(&gate->cond);
    pthread_mutex_unlock(&gate->mutex);
}

static void submit_gate_destroy(submit_gate_t *gate) {
    pthread_cond_destroy(&gate->cond);
    pthread_mutex_destroy(&gate->mutex);
}

/*
 * 驱动批量 copy-out 前逐页写入接收数组，确保 StarryOS 已为整段用户地址建立物理页。
 * volatile 防止编译器删除这些只用于触发缺页处理的写操作。
 */
static void touch_trace_output_pages(void *buffer, size_t bytes) {
    volatile unsigned char *output = buffer;

    if (output == NULL || bytes == 0U) {
        return;
    }
    for (size_t offset = 0; offset < bytes; offset += USER_PAGE_SIZE) {
        output[offset] = 0U;
    }
    output[bytes - 1U] = 0U;
}

/* 清空驱动固定 trace 缓冲区，保证随后读取的记录只属于本次进程。 */
static int reset_submit_trace(int fd) {
    struct rknpu_submit_trace_query query;

    memset(&query, 0, sizeof(query));
    query.operation = RKNPU_SUBMIT_TRACE_RESET;
    return ioctl(fd, DRM_IOCTL_RKNPU_SUBMIT_TRACE, &query);
}

/*
 * 测试结束后一次性读取驱动保存的 t0～t4，避免逐 Submit 串口日志扰动时序。
 * records 由用户态分配，capacity 以记录条数为单位，驱动通过 query 返回实际数量。
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

/* 配置并清空调度事件；event_mask=0 用于关闭记录。 */
static int configure_schedule_trace(int fd, uint32_t event_mask) {
    struct rknpu_schedule_trace_query query;

    memset(&query, 0, sizeof(query));
    query.operation = RKNPU_SCHEDULE_TRACE_CONFIG_RESET;
    query.event_mask = event_mask;
    return ioctl(fd, DRM_IOCTL_RKNPU_SCHEDULE_TRACE, &query);
}

/* 一次性读取固定缓冲区中的调度事件；读取不会消费或清空记录。 */
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

static uint32_t busy_core_count(const core_usage_t *cores, uint32_t npu_cores) {
    uint32_t count = 0;

    for (uint32_t core = 0; core < npu_cores; core++) {
        if (cores[core].busy) {
            count++;
        }
    }
    return count;
}

/* 相同时间戳按原始 sequence 排序，保证同一时刻的事件顺序稳定。 */
static int compare_schedule_timestamp(const void *left_ptr, const void *right_ptr) {
    const struct rknpu_schedule_trace_record *left = left_ptr;
    const struct rknpu_schedule_trace_record *right = right_ptr;

    if (left->timestamp_ns != right->timestamp_ns) {
        return left->timestamp_ns < right->timestamp_ns ? -1 : 1;
    }
    return left->sequence < right->sequence ? -1 : left->sequence > right->sequence;
}

/*
 * 严格按事件顺序配对同一物理核心上的 Dispatch/Complete。
 * busy_ns 是 Task 在该核心从下发到完成的区间和；refill gap 是该核心完成一个
 * Task 到下一个 Task 下发的时间。任何缺失、重复或核心越界都会拒绝利用率结论。
 */
static int report_core_usage(
    struct rknpu_schedule_trace_record *records,
    size_t record_count,
    uint32_t npu_cores,
    size_t expected_tasks
) {
    core_usage_t cores[MAX_NPU_CORES] = {0};
    all_core_usage_t all = {0};
    uint64_t previous_timestamp_ns = 0;
    size_t dispatch_total = 0;
    size_t complete_total = 0;

    if (record_count == 0U || npu_cores == 0U || npu_cores > MAX_NPU_CORES) {
        return -1;
    }

    /* sequence 先证明记录完整，再按事件时间排序计算区间。 */
    for (size_t index = 0; index < record_count; index++) {
        if (records[index].sequence != index || records[index].queue_task == 0U ||
            records[index].core_slot >= npu_cores) {
            fprintf(stderr, "invalid core usage trace at index=%zu\n", index);
            return -1;
        }
    }
    qsort(records, record_count, sizeof(*records), compare_schedule_timestamp);

    for (size_t index = 0; index < record_count; index++) {
        const struct rknpu_schedule_trace_record *record = &records[index];
        core_usage_t *core;

        if (index == 0U) {
            all.window_start_ns = record->timestamp_ns;
        } else if (busy_core_count(cores, npu_cores) == npu_cores) {
            all.all_cores_busy_ns += record->timestamp_ns - previous_timestamp_ns;
        }
        previous_timestamp_ns = record->timestamp_ns;
        all.window_end_ns = record->timestamp_ns;
        core = &cores[record->core_slot];

        if (record->event_type == RKNPU_SCHEDULE_EVENT_DISPATCH) {
            uint64_t refill_gap_ns;

            if (core->busy) {
                fprintf(stderr, "core%u dispatched while already busy\n", record->core_slot);
                return -1;
            }
            if (core->complete_count > 0U) {
                if (record->timestamp_ns < core->last_complete_ns) {
                    return -1;
                }
                refill_gap_ns = record->timestamp_ns - core->last_complete_ns;
                core->idle_gap_ns += refill_gap_ns;
                if (refill_gap_ns > core->max_refill_gap_ns) {
                    core->max_refill_gap_ns = refill_gap_ns;
                }
                core->refill_gap_count++;
            }
            core->busy = 1;
            core->queue_task = record->queue_task;
            core->task_index = record->task_index;
            core->dispatch_ns = record->timestamp_ns;
            core->dispatch_count++;
            dispatch_total++;
        } else if (record->event_type == RKNPU_SCHEDULE_EVENT_COMPLETE) {
            if (!core->busy || core->queue_task != record->queue_task ||
                core->task_index != record->task_index ||
                record->timestamp_ns < core->dispatch_ns) {
                fprintf(stderr, "unmatched completion at index=%zu core=%u\n",
                        index, record->core_slot);
                return -1;
            }
            core->busy_ns += record->timestamp_ns - core->dispatch_ns;
            core->last_complete_ns = record->timestamp_ns;
            core->complete_count++;
            core->busy = 0;
            complete_total++;
        } else {
            fprintf(stderr, "unexpected core usage event at index=%zu type=0x%x\n",
                    index, record->event_type);
            return -1;
        }
    }

    if (dispatch_total != expected_tasks || complete_total != expected_tasks ||
        all.window_end_ns <= all.window_start_ns) {
        fprintf(stderr,
                "core usage event mismatch: dispatch=%zu complete=%zu expected=%zu\n",
                dispatch_total, complete_total, expected_tasks);
        return -1;
    }
    for (uint32_t core = 0; core < npu_cores; core++) {
        if (cores[core].busy || cores[core].dispatch_count != cores[core].complete_count ||
            cores[core].dispatch_count == 0U) {
            fprintf(stderr, "core%u has incomplete or empty event stream\n", core);
            return -1;
        }
    }

    {
        uint64_t trace_window_ns = all.window_end_ns - all.window_start_ns;
        size_t min_core_tasks = cores[0].dispatch_count;
        size_t max_core_tasks = cores[0].dispatch_count;

        printf("per-core Schedule Trace usage\n");
        printf("  trace tasks     : %zu Dispatch + %zu Complete\n",
               dispatch_total, complete_total);
        printf("  trace window    : %.3f ms\n", (double)trace_window_ns / 1000000.0);
        for (uint32_t core = 0; core < npu_cores; core++) {
            double utilization = (double)cores[core].busy_ns * 100.0 /
                (double)trace_window_ns;
            double mean_refill_us = cores[core].refill_gap_count > 0U
                ? (double)cores[core].idle_gap_ns /
                    (double)cores[core].refill_gap_count / 1000.0
                : 0.0;

            printf("  core%u           : tasks=%zu busy=%9.3f ms util=%6.2f%% "
                   "refill mean/max=%8.3f/%8.3f us\n",
                   core, cores[core].dispatch_count,
                   (double)cores[core].busy_ns / 1000000.0,
                   utilization, mean_refill_us,
                   (double)cores[core].max_refill_gap_ns / 1000.0);
            if (cores[core].dispatch_count < min_core_tasks) {
                min_core_tasks = cores[core].dispatch_count;
            }
            if (cores[core].dispatch_count > max_core_tasks) {
                max_core_tasks = cores[core].dispatch_count;
            }
        }
        printf("  dispatch balance: min=%zu max=%zu difference=%zu tasks\n",
               min_core_tasks, max_core_tasks, max_core_tasks - min_core_tasks);
        printf("  all-core busy   : %9.3f ms (%6.2f%% of trace window)\n",
               (double)all.all_cores_busy_ns / 1000000.0,
               (double)all.all_cores_busy_ns * 100.0 / (double)trace_window_ns);
    }
    return 0;
}

/* 对一个阶段的纳秒样本计算 Mean/P50/P95/P99，并在输出时换算为毫秒。 */
static int print_phase_stats(const char *name, uint64_t *samples_ns, size_t count) {
    benchmark_latency_stats_t stats;

    if (benchmark_compute_latency_stats(samples_ns, count, &stats) != 0) {
        return -1;
    }

    printf("  %-17s mean=%9.6f ms  P50=%9.6f ms  P95=%9.6f ms  P99=%9.6f ms\n",
           name,
           stats.mean_us / 1000000.0,
           (double)stats.p50_us / 1000000.0,
           (double)stats.p95_us / 1000000.0,
           (double)stats.p99_us / 1000000.0);
    return 0;
}

/* 对微秒级用户态样本计算 Mean/P50/P95/P99，并统一按毫秒输出。 */
static int print_user_latency_stats(const char *name, uint64_t *samples_us, size_t count) {
    benchmark_latency_stats_t stats;

    if (benchmark_compute_latency_stats(samples_us, count, &stats) != 0) {
        return -1;
    }
    printf("  %-17s mean=%9.6f ms  P50=%9.6f ms  P95=%9.6f ms  P99=%9.6f ms\n",
           name,
           stats.mean_us / 1000.0,
           (double)stats.p50_us / 1000.0,
           (double)stats.p95_us / 1000.0,
           (double)stats.p99_us / 1000.0);
    return 0;
}

/*
 * 先校验全部记录满足 queue_task!=0 和 t0<=t1<=t2<=t3<=t4，随后跳过
 * warmup_count 条记录，分别计算 t1-t0、t2-t1、t3-t2、t4-t3。
 * 记录不完整或任一时间倒序都会使本次实验 FAIL，禁止用残缺样本得出阶段结论。
 */
static int report_submit_trace(
    const struct rknpu_submit_trace_record *records,
    size_t record_count,
    size_t warmup_count
) {
    uint64_t *samples_ns;
    size_t measured_count;

    if (warmup_count >= record_count) {
        return -1;
    }
    for (size_t index = 0; index < record_count; index++) {
        const struct rknpu_submit_trace_record *record = &records[index];
        if (record->queue_task == 0U ||
            record->t0_ns > record->t1_ns ||
            record->t1_ns > record->t2_ns ||
            record->t2_ns > record->t3_ns ||
            record->t3_ns > record->t4_ns) {
            fprintf(stderr, "invalid submit trace record at index=%zu queue_task=%llu\n",
                    index, (unsigned long long)record->queue_task);
            return -1;
        }
    }

    measured_count = record_count - warmup_count;
    samples_ns = malloc(measured_count * sizeof(*samples_ns));
    if (samples_ns == NULL) {
        return -1;
    }

    for (size_t index = 0; index < measured_count; index++) {
        const struct rknpu_submit_trace_record *record = &records[warmup_count + index];
        samples_ns[index] = record->t1_ns - record->t0_ns;
    }
    if (print_phase_stats("submit_prepare", samples_ns, measured_count) != 0) {
        free(samples_ns);
        return -1;
    }

    for (size_t index = 0; index < measured_count; index++) {
        const struct rknpu_submit_trace_record *record = &records[warmup_count + index];
        samples_ns[index] = record->t2_ns - record->t1_ns;
    }
    if (print_phase_stats("queue_wait", samples_ns, measured_count) != 0) {
        free(samples_ns);
        return -1;
    }

    for (size_t index = 0; index < measured_count; index++) {
        const struct rknpu_submit_trace_record *record = &records[warmup_count + index];
        samples_ns[index] = record->t3_ns - record->t2_ns;
    }
    if (print_phase_stats("dispatch_execute", samples_ns, measured_count) != 0) {
        free(samples_ns);
        return -1;
    }

    for (size_t index = 0; index < measured_count; index++) {
        const struct rknpu_submit_trace_record *record = &records[warmup_count + index];
        samples_ns[index] = record->t4_ns - record->t3_ns;
    }
    if (print_phase_stats("complete_return", samples_ns, measured_count) != 0) {
        free(samples_ns);
        return -1;
    }

    free(samples_ns);
    return 0;
}

/* 释放场景模块为该 worker 创建的全部 GEM。 */
static void release_worker_resources(worker_context_t *worker) {
    rknpu_scenario_workload_destroy(worker->workload);
    worker->workload = NULL;
}

/* 在线程启动前按当前核心数创建多 Task 资源和逻辑 lane。 */
static int prepare_worker_resources(worker_context_t *worker, uint32_t npu_cores) {
    return rknpu_scenario_workload_create(
        worker->fd,
        worker->thread_id,
        worker->scenario,
        npu_cores,
        worker->core_mask,
        &worker->workload
    );
}

/* 初始化一次性正式测量启动门。 */
static int stream_gate_init(stream_gate_t *gate, uint32_t expected_threads) {
    memset(gate, 0, sizeof(*gate));
    gate->expected_threads = expected_threads;

    if (pthread_mutex_init(&gate->mutex, NULL) != 0) {
        return -1;
    }
    if (pthread_cond_init(&gate->cond, NULL) != 0) {
        pthread_mutex_destroy(&gate->mutex);
        return -1;
    }
    return 0;
}

/* 设置停止标志并广播，解除所有等待开始或结束条件的线程。 */
static void stream_gate_stop(stream_gate_t *gate) {
    pthread_mutex_lock(&gate->mutex);
    gate->stop = 1;
    pthread_cond_broadcast(&gate->cond);
    pthread_mutex_unlock(&gate->mutex);
}

/* 所有 pthread join 后销毁条件变量和互斥锁。 */
static void stream_gate_destroy(stream_gate_t *gate) {
    pthread_cond_destroy(&gate->cond);
    pthread_mutex_destroy(&gate->mutex);
}

/*
 * worker 完成全部预热后报告 ready，只等待一次 started。返回 0 表示线程创建
 * 失败后主线程取消了实验，不进入正式 Submit。
 */
static int worker_wait_for_stream_start(stream_gate_t *gate) {
    int should_run;

    pthread_mutex_lock(&gate->mutex);
    gate->ready_threads++;
    pthread_cond_broadcast(&gate->cond);

    while (!gate->stop && !gate->started) {
        pthread_cond_wait(&gate->cond, &gate->mutex);
    }
    should_run = !gate->stop;
    pthread_mutex_unlock(&gate->mutex);
    return should_run;
}

/* 主线程等待全部 worker 完成预热，然后只广播一次正式测量开始。 */
static int stream_gate_start(stream_gate_t *gate) {
    pthread_mutex_lock(&gate->mutex);
    while (!gate->stop && gate->ready_threads < gate->expected_threads) {
        pthread_cond_wait(&gate->cond, &gate->mutex);
    }
    if (gate->stop) {
        pthread_mutex_unlock(&gate->mutex);
        return -1;
    }

    gate->started = 1;
    pthread_cond_broadcast(&gate->cond);
    pthread_mutex_unlock(&gate->mutex);
    return 0;
}

/*
 * 所有 worker 完成最后一个正式 Submit 后再统一开始抽样校验，避免先结束的线程
 * 在其他线程仍测量时占用 CPU。该门只使用一次，不会在 Submit 之间制造空档。
 */
static void worker_wait_for_stream_finish(stream_gate_t *gate) {
    pthread_mutex_lock(&gate->mutex);
    gate->finished_threads++;
    if (gate->finished_threads == gate->expected_threads) {
        pthread_cond_broadcast(&gate->cond);
    } else {
        while (gate->finished_threads < gate->expected_threads) {
            pthread_cond_wait(&gate->cond, &gate->mutex);
        }
    }
    pthread_mutex_unlock(&gate->mutex);
}

/*
 * 预热 Submit 保留完整清理和结果校验，确保正式测量开始前硬件与测试数据正确。
 */
static int run_checked_submit(worker_context_t *worker, uint32_t round) {
    struct rknpu_submit submit;
    rknpu_scenario_check_t check;

    rknpu_scenario_workload_begin(worker->workload, &submit);

    submit_gate_acquire(worker->submit_gate);
    if (ioctl(worker->fd, DRM_IOCTL_RKNPU_SUBMIT, &submit) < 0) {
        int submit_errno = errno;
        submit_gate_release(worker->submit_gate);
        worker->error = WORKER_IOCTL_FAILED;
        worker->error_errno = submit_errno;
        worker->error_round = round;
        return -1;
    }
    submit_gate_release(worker->submit_gate);
    if (rknpu_scenario_workload_check(worker->workload, &submit, &check) != 0) {
        worker->error_round = round;
        worker->error_task_index = check.task_index;
        worker->error_task_counter = check.task_counter;
        worker->error_irq_status = check.irq_status;
        worker->error_row = check.row;
        worker->error_col = check.column;
        worker->error_expected = check.expected;
        worker->error_actual = check.actual;
        if (check.error == RKNPU_SCENARIO_CHECK_TASK_COUNTER) {
            worker->error = WORKER_TASK_COUNTER_FAILED;
        } else if (check.error == RKNPU_SCENARIO_CHECK_IRQ_STATUS) {
            worker->error = WORKER_IRQ_STATUS_FAILED;
        } else {
            worker->error = WORKER_OUTPUT_FAILED;
        }
        return -1;
    }

    worker->successful_submits++;
    return 0;
}

/*
 * pthread 入口。各线程自行完成预热，然后在同一个起点开始连续 blocking Submit。
 * 正式阶段的 ioctl 返回后只保存时间和返回结构，立即发起下一次 Submit；所有正式
 * Submit 结束后才抽样校验最后一次完成状态，避免逐轮校验制造喂料空档。
 */
static void *worker_main(void *arg) {
    worker_context_t *worker = arg;
    struct rknpu_submit last_submit;
    int has_last_submit = 0;

    for (uint32_t round = 0; round < worker->warmup_rounds; round++) {
        if (run_checked_submit(worker, round) != 0) {
            break;
        }
    }

    if (!worker_wait_for_stream_start(worker->gate)) {
        return NULL;
    }

    if (worker->error == WORKER_OK) {
        for (uint32_t round = 0; round < worker->measure_rounds; round++) {
            struct rknpu_submit submit;
            uint64_t request_start_us;
            uint64_t ioctl_start_us;
            uint64_t end_us;
            int submit_errno = 0;

            rknpu_scenario_workload_next_submit(worker->workload, &submit);
            request_start_us = now_us();
            submit_gate_acquire(worker->submit_gate);
            ioctl_start_us = now_us();
            if (ioctl(worker->fd, DRM_IOCTL_RKNPU_SUBMIT, &submit) < 0) {
                submit_errno = errno;
            }
            end_us = now_us();
            submit_gate_release(worker->submit_gate);

            if (submit_errno != 0) {
                worker->error = WORKER_IOCTL_FAILED;
                worker->error_errno = submit_errno;
                worker->error_round = worker->warmup_rounds + round;
                break;
            }

            if (worker->successful_measured_submits == 0U) {
                /* 吞吐窗口包含 admission wait，避免只统计获得 permit 后的局部时间。 */
                worker->first_measured_start_us = request_start_us;
            }
            worker->last_measured_end_us = end_us;
            worker->latency_samples_us[worker->latency_sample_count] =
                end_us - ioctl_start_us;
            worker->gate_wait_samples_us[worker->latency_sample_count] =
                ioctl_start_us - request_start_us;
            worker->request_latency_samples_us[worker->latency_sample_count] =
                end_us - request_start_us;
            worker->latency_sample_count++;
            worker->successful_measured_submits++;
            worker->successful_submits++;
            /* 只保留最后一个正式 Submit，避免在每轮热路径复制返回结构。 */
            if (round + 1U == worker->measure_rounds) {
                last_submit = submit;
                has_last_submit = 1;
            }
        }
    }

    /* 全部 worker 先完成正式流水，随后才执行计时区间外的抽样校验。 */
    worker_wait_for_stream_finish(worker->gate);

    /* 只校验最后一次正式 Submit，不扫描每轮结果。 */
    if (worker->error == WORKER_OK && has_last_submit) {
        rknpu_scenario_check_t check;

        if (rknpu_scenario_workload_check(worker->workload, &last_submit, &check) != 0) {
            worker->error_round = worker->warmup_rounds + worker->measure_rounds - 1U;
            worker->error_task_index = check.task_index;
            worker->error_task_counter = check.task_counter;
            worker->error_irq_status = check.irq_status;
            worker->error_row = check.row;
            worker->error_col = check.column;
            worker->error_expected = check.expected;
            worker->error_actual = check.actual;
            if (check.error == RKNPU_SCENARIO_CHECK_TASK_COUNTER) {
                worker->error = WORKER_TASK_COUNTER_FAILED;
            } else if (check.error == RKNPU_SCENARIO_CHECK_IRQ_STATUS) {
                worker->error = WORKER_IRQ_STATUS_FAILED;
            } else {
                worker->error = WORKER_OUTPUT_FAILED;
            }
        }
    }
    return NULL;
}

/*
 * 利用率窗口仍是连续 Submit，只在启动处同步一次。该窗口不计用户态性能，目的仅是
 * 生成完整的 Dispatch/Complete 事件流；最后一次 Submit 结束后检查返回计数。
 */
static void *usage_worker_main(void *arg) {
    usage_worker_t *worker = arg;
    struct rknpu_submit last_submit;
    int has_last_submit = 0;

    if (!worker_wait_for_stream_start(worker->gate)) {
        return NULL;
    }
    for (uint32_t round = 0; round < worker->rounds; round++) {
        struct rknpu_submit submit;

        rknpu_scenario_workload_next_submit(worker->workload, &submit);
        submit_gate_acquire(worker->submit_gate);
        if (ioctl(worker->fd, DRM_IOCTL_RKNPU_SUBMIT, &submit) < 0) {
            int submit_errno = errno;
            submit_gate_release(worker->submit_gate);
            worker->error_errno = submit_errno;
            break;
        }
        submit_gate_release(worker->submit_gate);
        last_submit = submit;
        has_last_submit = 1;
        worker->completed++;
    }
    if (worker->error_errno == 0 &&
        (!has_last_submit || last_submit.task_counter != worker->task_count)) {
        worker->error_errno = EPROTO;
    }
    return NULL;
}

/*
 * 在正式性能和四阶段统计完成后运行一个独立短窗口。轮数由固定 trace 容量推导，
 * 只启用 Dispatch/Complete，确保所有事件完整保存后才计算每核利用率。
 */
static int run_core_usage_window(
    int fd,
    worker_context_t *workers,
    uint32_t thread_count,
    uint32_t npu_cores,
    uint32_t task_count,
    submit_gate_t *submit_gate
) {
    usage_worker_t *usage_workers = NULL;
    pthread_t *threads = NULL;
    struct rknpu_schedule_trace_record *records = NULL;
    struct rknpu_schedule_trace_query query;
    stream_gate_t gate;
    size_t events_per_round;
    size_t expected_events;
    size_t expected_tasks;
    uint32_t rounds;
    uint32_t created_threads = 0;
    int gate_initialized = 0;
    int result = -1;

    events_per_round = (size_t)thread_count * task_count * 2U;
    if (events_per_round == 0U || events_per_round > RKNPU_SCHEDULE_TRACE_CAPACITY) {
        fprintf(stderr, "one core-usage round exceeds Schedule Trace capacity\n");
        return -1;
    }
    rounds = (uint32_t)(RKNPU_SCHEDULE_TRACE_CAPACITY / events_per_round);
    if (rounds > 10U) {
        rounds = 10U;
    }
    expected_tasks = (size_t)thread_count * rounds * task_count;
    expected_events = expected_tasks * 2U;

    usage_workers = calloc(thread_count, sizeof(*usage_workers));
    threads = calloc(thread_count, sizeof(*threads));
    records = calloc(expected_events, sizeof(*records));
    if (usage_workers == NULL || threads == NULL || records == NULL) {
        goto cleanup;
    }
    /* Trace 尚未开启，此处预建接收页不会进入逐核利用率统计窗口。 */
    touch_trace_output_pages(records, expected_events * sizeof(*records));
    if (stream_gate_init(&gate, thread_count) != 0) {
        goto cleanup;
    }
    gate_initialized = 1;

    if (configure_schedule_trace(
            fd,
            RKNPU_SCHEDULE_EVENT_DISPATCH | RKNPU_SCHEDULE_EVENT_COMPLETE
        ) < 0) {
        fprintf(stderr, "Schedule Trace configuration failed: errno=%d (%s)\n",
                errno, strerror(errno));
        goto cleanup;
    }

    printf("core usage trace window\n");
    printf("  rounds          : %u continuous rounds per thread\n", rounds);
    printf("  expected events : %zu/%u capacity\n",
           expected_events, RKNPU_SCHEDULE_TRACE_CAPACITY);

    for (uint32_t index = 0; index < thread_count; index++) {
        usage_workers[index].fd = fd;
        usage_workers[index].rounds = rounds;
        usage_workers[index].task_count = task_count;
        usage_workers[index].gate = &gate;
        usage_workers[index].submit_gate = submit_gate;
        usage_workers[index].workload = workers[index].workload;
        {
            int error = pthread_create(
                &threads[index], NULL, usage_worker_main, &usage_workers[index]);
            if (error != 0) {
                fprintf(stderr, "usage pthread_create failed: %s\n", strerror(error));
                stream_gate_stop(&gate);
                goto join_threads;
            }
        }
        created_threads++;
    }

    if (stream_gate_start(&gate) != 0) {
        goto join_threads;
    }

join_threads:
    for (uint32_t index = 0; index < created_threads; index++) {
        pthread_join(threads[index], NULL);
    }
    if (created_threads != thread_count) {
        goto cleanup;
    }
    for (uint32_t index = 0; index < thread_count; index++) {
        if (usage_workers[index].error_errno != 0 ||
            usage_workers[index].completed != rounds) {
            fprintf(stderr,
                    "usage worker[%u] incomplete: completed=%u/%u errno=%d\n",
                    index, usage_workers[index].completed, rounds,
                    usage_workers[index].error_errno);
            goto cleanup;
        }
    }

    if (read_schedule_trace(fd, records, (uint32_t)expected_events, &query) < 0 ||
        query.count != expected_events || query.overflowed != 0U) {
        fprintf(stderr,
                "Schedule Trace incomplete: count=%u/%zu overflow=%u errno=%d\n",
                query.count, expected_events, query.overflowed, errno);
        goto cleanup;
    }
    result = report_core_usage(records, query.count, npu_cores, expected_tasks);

cleanup:
    /* 后续场景的正式性能窗口必须运行在 Schedule Trace 关闭状态。 */
    if (fd >= 0 && configure_schedule_trace(fd, 0U) < 0) {
        fprintf(stderr, "Schedule Trace disable failed: errno=%d (%s)\n",
                errno, strerror(errno));
        result = -1;
    }
    if (gate_initialized) {
        stream_gate_destroy(&gate);
    }
    free(records);
    free(threads);
    free(usage_workers);
    return result;
}

/* 将错误枚举转换为单线程结果中的短名称。 */
static const char *worker_error_name(worker_error_t error) {
    switch (error) {
        case WORKER_OK:
            return "none";
        case WORKER_IOCTL_FAILED:
            return "ioctl";
        case WORKER_TASK_COUNTER_FAILED:
            return "task_counter";
        case WORKER_IRQ_STATUS_FAILED:
            return "irq_status";
        case WORKER_OUTPUT_FAILED:
            return "output";
    }
    return "unknown";
}

/* 输出单线程完成数、正式轮均值和首个错误现场。 */
static void print_worker_result(const worker_context_t *worker, uint32_t total_rounds) {
    uint64_t total_latency_us = 0;
    double mean_us = 0.0;

    for (size_t index = 0; index < worker->latency_sample_count; index++) {
        total_latency_us += worker->latency_samples_us[index];
    }
    if (worker->latency_sample_count > 0U) {
        mean_us = (double)total_latency_us / (double)worker->latency_sample_count;
    }

    printf(
        "thread[%u]: completed=%u/%u measured=%zu mean=%.3f ms error=%s\n",
        worker->thread_id,
        worker->successful_submits,
        total_rounds,
        worker->latency_sample_count,
        mean_us / 1000.0,
        worker_error_name(worker->error)
    );

    if (worker->error == WORKER_IOCTL_FAILED) {
        printf(
            "  first failure: round=%u errno=%d (%s)\n",
            worker->error_round + 1U,
            worker->error_errno,
            strerror(worker->error_errno)
        );
    } else if (worker->error == WORKER_TASK_COUNTER_FAILED) {
        printf(
            "  first failure: round=%u task_counter=%u expected=%u\n",
            worker->error_round + 1U,
            worker->error_task_counter,
            worker->scenario->task_count
        );
    } else if (worker->error == WORKER_IRQ_STATUS_FAILED) {
        printf(
            "  first failure: round=%u task=%u int_status=0x%x expected=0x300\n",
            worker->error_round + 1U,
            worker->error_task_index,
            worker->error_irq_status
        );
    } else if (worker->error == WORKER_OUTPUT_FAILED) {
        printf(
            "  first failure: round=%u task=%u row=%u col=%u expected=%f actual=%f\n",
            worker->error_round + 1U,
            worker->error_task_index,
            worker->error_row,
            worker->error_col,
            worker->error_expected,
            worker->error_actual
        );
    }
}

/* 严格解析十进制 u32，拒绝空串、尾随字符和范围溢出。 */
static int parse_u32(const char *text, uint32_t *value) {
    char *end = NULL;
    unsigned long parsed;

    errno = 0;
    parsed = strtoul(text, &end, 10);
    if (errno != 0 || end == text || *end != '\0' || parsed > UINT32_MAX) {
        return -1;
    }
    *value = (uint32_t)parsed;
    return 0;
}

/* 打印线程、轮数和核心数量参数。 */
static void print_usage(const char *program) {
    size_t scenario_count;
    const rknpu_scenario_case_t *scenarios = rknpu_scenario_cases(&scenario_count);

    printf("Usage: %s [options]\n", program);
    printf("Options:\n");
    printf("  --scenario <name|all>  scenario case, default: all\n");
    printf("  --threads <count>      submit threads, default: %u\n", DEFAULT_THREADS);
    printf("  --submit-limit <count> max simultaneous live Submit, default: threads\n");
    printf("  --rounds <count>       measured rounds, default: %u\n", DEFAULT_ROUNDS);
    printf("  --warmup <count>   warmup rounds, default: %u\n", DEFAULT_WARMUP_ROUNDS);
    printf("  --cores <1|2|3>    NPU core count, default: %u\n", DEFAULT_NPU_CORES);
    printf("  --help             show this message\n");
    printf("Scenarios:\n");
    for (size_t index = 0; index < scenario_count; index++) {
        printf("  %-28s M=%u K=%u N=%u tasks=%u\n",
               scenarios[index].name,
               scenarios[index].m,
               scenarios[index].k,
               scenarios[index].n,
               scenarios[index].task_count);
    }
}

/*
 * 解析参数并把核心数量转换为连续 core_mask：1->0x1、2->0x3、3->0x7。
 * 同时检查线程范围以及 warmup+rounds 的 u32 加法溢出。
 */
static int parse_options(int argc, char **argv, test_options_t *options) {
    options->threads = DEFAULT_THREADS;
    options->submit_limit = 0U;
    options->rounds = DEFAULT_ROUNDS;
    options->warmup_rounds = DEFAULT_WARMUP_ROUNDS;
    options->npu_cores = DEFAULT_NPU_CORES;
    options->core_mask = (1U << DEFAULT_NPU_CORES) - 1U;
    options->scenario_filter = "all";

    for (int index = 1; index < argc; index++) {
        const char *arg = argv[index];

        if (strcmp(arg, "--help") == 0) {
            print_usage(argv[0]);
            return 1;
        }
        if (strcmp(arg, "--scenario") != 0 &&
            strcmp(arg, "--threads") != 0 &&
            strcmp(arg, "--submit-limit") != 0 &&
            strcmp(arg, "--rounds") != 0 &&
            strcmp(arg, "--warmup") != 0 &&
            strcmp(arg, "--cores") != 0) {
            fprintf(stderr, "unknown option: %s\n", arg);
            print_usage(argv[0]);
            return -1;
        }
        if (++index >= argc) {
            fprintf(stderr, "missing value for %s\n", arg);
            return -1;
        }

        if (strcmp(arg, "--scenario") == 0) {
            if (strcmp(argv[index], "all") != 0 &&
                rknpu_scenario_find(argv[index]) == NULL) {
                fprintf(stderr, "invalid --scenario value: %s\n", argv[index]);
                return -1;
            }
            options->scenario_filter = argv[index];
        } else if (strcmp(arg, "--threads") == 0) {
            if (parse_u32(argv[index], &options->threads) != 0 ||
                options->threads == 0U || options->threads > MAX_TEST_THREADS) {
                fprintf(stderr, "invalid --threads value: %s (valid: 1-%u)\n",
                        argv[index], MAX_TEST_THREADS);
                return -1;
            }
        } else if (strcmp(arg, "--submit-limit") == 0) {
            if (parse_u32(argv[index], &options->submit_limit) != 0 ||
                options->submit_limit == 0U || options->submit_limit > MAX_TEST_THREADS) {
                fprintf(stderr, "invalid --submit-limit value: %s (valid: 1-%u)\n",
                        argv[index], MAX_TEST_THREADS);
                return -1;
            }
        } else if (strcmp(arg, "--rounds") == 0) {
            if (parse_u32(argv[index], &options->rounds) != 0 || options->rounds == 0U) {
                fprintf(stderr, "invalid --rounds value: %s\n", argv[index]);
                return -1;
            }
        } else if (strcmp(arg, "--warmup") == 0) {
            if (parse_u32(argv[index], &options->warmup_rounds) != 0) {
                fprintf(stderr, "invalid --warmup value: %s\n", argv[index]);
                return -1;
            }
        } else if (parse_u32(argv[index], &options->npu_cores) != 0 ||
                   options->npu_cores == 0U ||
                   options->npu_cores > MAX_NPU_CORES) {
            fprintf(stderr, "invalid --cores value: %s (valid: 1-%u)\n",
                    argv[index], MAX_NPU_CORES);
            return -1;
        }
    }

    /* 未显式限制时保持原程序行为：允许所有业务线程同时进入 blocking Submit。 */
    if (options->submit_limit == 0U) {
        options->submit_limit = options->threads;
    }
    if (options->submit_limit > options->threads) {
        fprintf(stderr,
                "--submit-limit (%u) cannot exceed --threads (%u)\n",
                options->submit_limit, options->threads);
        return -1;
    }

    /* 核心数量映射为从 core0 开始的连续物理核心掩码。 */
    options->core_mask = (1U << options->npu_cores) - 1U;

    if (UINT32_MAX - options->warmup_rounds < options->rounds) {
        fprintf(stderr, "warmup and measured round count overflow\n");
        return -1;
    }
    return 0;
}

/* all 选择七个组合，指定完整名称时只执行一个组合。 */
static int scenario_selected(const test_options_t *options, const char *name) {
    return strcmp(options->scenario_filter, "all") == 0 ||
        strcmp(options->scenario_filter, name) == 0;
}

/*
 * 程序级流程：解析 core_mask -> 打开共享 fd 并复位 -> 清空 trace -> 准备每线程 DMA
 * -> 各线程预热 -> 单次同步后连续 Submit -> join -> 读取 trace -> 汇总指标 -> 释放 DMA 和 fd。
 * trace 记录数量和顺序也是 PASS 条件，不接受缺失或溢出的四阶段样本。
 */
static int run_scenario(
    const test_options_t *options,
    const rknpu_scenario_case_t *scenario
) {
    worker_context_t *workers = NULL;
    pthread_t *threads = NULL;
    stream_gate_t gate;
    submit_gate_t submit_gate;
    uint32_t prepared_workers = 0;
    uint32_t created_threads = 0;
    uint32_t total_rounds;
    uint64_t first_measured_start_us = 0;
    uint64_t last_measured_end_us = 0;
    uint64_t measured_window_us = 0;
    uint64_t setup_operands_us = 0;
    uint64_t build_regcmds_us = 0;
    size_t total_dma_bytes = 0;
    uint64_t *all_samples = NULL;
    uint64_t *all_gate_wait_samples = NULL;
    uint64_t *all_request_samples = NULL;
    /* 驱动一次性返回的四阶段原始记录。 */
    struct rknpu_submit_trace_record *trace_records = NULL;
    struct rknpu_submit_trace_query trace_query;
    size_t all_sample_count = 0;
    size_t sample_offset = 0;
    size_t measured_submit_count = 0;
    size_t expected_measured_count;
    size_t expected_trace_count;
    size_t warmup_trace_count;
    int fd = -1;
    int gate_initialized = 0;
    int submit_gate_initialized = 0;
    int result = 1;
    int join_failed = 0;
    int trace_ok = 0;
    int metrics_ok = 0;
    int core_usage_ok = 0;
    total_rounds = options->warmup_rounds + options->rounds;
    expected_measured_count = (size_t)options->threads * options->rounds;
    if (total_rounds > RKNPU_SUBMIT_TRACE_CAPACITY / options->threads) {
        fprintf(stderr,
                "trace capacity exceeded: threads=%u total_rounds=%u capacity=%u\n",
                options->threads, total_rounds, RKNPU_SUBMIT_TRACE_CAPACITY);
        return 2;
    }
    expected_trace_count = (size_t)options->threads * total_rounds;
    warmup_trace_count = (size_t)options->threads * options->warmup_rounds;

    /* 2. 整次实验共享一次 npu_open，并且只执行一次硬件 reset。 */
    fd = npu_open();
    if (fd < 0) {
        return 1;
    }
    if (npu_reset(fd) < 0) {
        fprintf(stderr, "RKNPU reset failed: errno=%d (%s)\n", errno, strerror(errno));
        goto cleanup;
    }
    /* 新实验开始前清空驱动记录；旧驱动不支持该 ioctl 时直接停止测试。 */
    if (reset_submit_trace(fd) < 0 || configure_schedule_trace(fd, 0U) < 0) {
        fprintf(stderr, "RKNPU trace reset failed: errno=%d (%s)\n",
                errno, strerror(errno));
        goto cleanup;
    }

    /* 3. 按预期 Submit 总数预分配用户态上下文；trace 容量已在打开设备前检查。 */
    workers = calloc(options->threads, sizeof(*workers));
    threads = calloc(options->threads, sizeof(*threads));
    if (workers == NULL || threads == NULL) {
        fprintf(stderr, "thread context allocation failed\n");
        goto cleanup;
    }

    /* 4. 每个线程独占五块 DMA，但所有 GEM 都通过同一个 fd 创建。 */
    for (uint32_t index = 0; index < options->threads; index++) {
        workers[index].thread_id = index;
        workers[index].fd = fd;
        workers[index].scenario = scenario;
        workers[index].core_mask = options->core_mask;
        workers[index].warmup_rounds = options->warmup_rounds;
        workers[index].measure_rounds = options->rounds;

        workers[index].latency_samples_us = calloc(
            options->rounds,
            sizeof(*workers[index].latency_samples_us)
        );
        workers[index].gate_wait_samples_us = calloc(
            options->rounds,
            sizeof(*workers[index].gate_wait_samples_us)
        );
        workers[index].request_latency_samples_us = calloc(
            options->rounds,
            sizeof(*workers[index].request_latency_samples_us)
        );
        if (workers[index].latency_samples_us != NULL &&
            workers[index].gate_wait_samples_us != NULL &&
            workers[index].request_latency_samples_us != NULL) {
            /* 启动线程前写入全部样本页，避免正式首轮触发延迟数组缺页。 */
            for (uint32_t round = 0; round < options->rounds; round++) {
                workers[index].latency_samples_us[round] = UINT64_MAX;
                workers[index].gate_wait_samples_us[round] = UINT64_MAX;
                workers[index].request_latency_samples_us[round] = UINT64_MAX;
            }
        }
        /* 分配 DMA、填充矩阵并生成该线程的 Task 和寄存器命令。 */
        if (workers[index].latency_samples_us == NULL ||
            workers[index].gate_wait_samples_us == NULL ||
            workers[index].request_latency_samples_us == NULL ||
            prepare_worker_resources(&workers[index], options->npu_cores) != 0) {
            fprintf(stderr, "worker[%u] resource preparation failed\n", index);
            goto cleanup;
        }
        {
            rknpu_scenario_metrics_t metrics;
            rknpu_scenario_workload_get_metrics(workers[index].workload, &metrics);
            total_dma_bytes += metrics.total_dma_bytes;
            setup_operands_us += metrics.setup_operands_us;
            build_regcmds_us += metrics.build_regcmds_us;
        }
        prepared_workers++;
    }

    /* 5. 先初始化 Submit admission gate，再初始化一次性正式测量启动门。 */
    if (submit_gate_init(&submit_gate, options->submit_limit) != 0) {
        fprintf(stderr, "submit admission gate initialization failed\n");
        goto cleanup;
    }
    submit_gate_initialized = 1;

    if (stream_gate_init(&gate, options->threads) != 0) {
        fprintf(stderr, "stream synchronization initialization failed\n");
        goto cleanup;
    }
    gate_initialized = 1;

    /* 6. 输出线程、负载、核心掩码和轮数，作为结果的实验条件。 */
    printf("rknpu_multithread_submit_core\n");
    printf("  scenario        : %s\n", scenario->name);
    printf("  operand mode    : %s\n", rknpu_operand_mode_name(scenario->operand_mode));
    printf("  shared fd       : yes\n");
    printf("  threads         : %u\n", options->threads);
    printf("  submit limit    : %u simultaneous live blocking Submit(s)\n",
           options->submit_limit);
    printf("  workload        : M=%u K=%u N=%u, %u tasks per submit\n",
           scenario->m, scenario->k, scenario->n, scenario->task_count);
    printf("  lanes           : %u, tasks distributed evenly\n", options->npu_cores);
    printf("  NPU cores       : %u\n", options->npu_cores);
    printf("  core mask       : 0x%x\n", options->core_mask);
    printf("  rounds          : %u warmup + %u measured\n",
           options->warmup_rounds, options->rounds);
    printf("  submit mode     : continuous blocking stream\n");
    printf("  validation      : warmup + last measured Submit per thread\n");

    /* 7. 全部 DMA 就绪后创建线程，资源准备时间不计入性能。 */
    for (uint32_t index = 0; index < options->threads; index++) {
        workers[index].gate = &gate;
        workers[index].submit_gate = &submit_gate;
        int thread_error = pthread_create(&threads[index], NULL, worker_main, &workers[index]);
        if (thread_error != 0) {
            fprintf(stderr, "pthread_create failed for worker[%u]: %s\n",
                    index, strerror(thread_error));
            /* 唤醒已经完成预热并等待正式起点的线程。 */
            stream_gate_stop(&gate);
            goto join_threads;
        }
        created_threads++;
    }

    /* 8. 等待各线程独立完成预热，只广播一次正式连续流水的起点。 */
    if (stream_gate_start(&gate) != 0) {
        fprintf(stderr, "continuous submit stream failed to start\n");
        goto join_threads;
    }

/* 9. 正常完成和创建失败都在此 join 已创建线程。 */
join_threads:
    for (uint32_t index = 0; index < created_threads; index++) {
        int thread_error = pthread_join(threads[index], NULL);
        if (thread_error != 0) {
            fprintf(stderr, "pthread_join failed for worker[%u]: %s\n",
                    index, strerror(thread_error));
            join_failed = 1;
        }
    }

    /* join 失败时不能证明 worker 已停止，因此不能安全释放其上下文。 */
    if (join_failed) {
        return 1;
    }
    if (created_threads != options->threads) {
        goto cleanup;
    }

    /*
     * join 后按线程局部时间取全局最早进入和最晚返回，热路径无需共享锁。
     * 这个连续区间保留线程间调度空档，可用于证明持续满载能力。
     */
    for (uint32_t index = 0; index < options->threads; index++) {
        if (workers[index].successful_measured_submits == 0U) {
            continue;
        }
        if (measured_submit_count == 0U ||
            workers[index].first_measured_start_us < first_measured_start_us) {
            first_measured_start_us = workers[index].first_measured_start_us;
        }
        if (measured_submit_count == 0U ||
            workers[index].last_measured_end_us > last_measured_end_us) {
            last_measured_end_us = workers[index].last_measured_end_us;
        }
        measured_submit_count += workers[index].successful_measured_submits;
    }
    if (measured_submit_count > 0U && last_measured_end_us >= first_measured_start_us) {
        measured_window_us = last_measured_end_us - first_measured_start_us;
    }

    /* 10. worker 全部停止后读取稳定 trace，禁止边运行边打印驱动阶段日志。 */
    trace_records = calloc(expected_trace_count, sizeof(*trace_records));
    if (trace_records == NULL) {
        fprintf(stderr, "submit trace allocation failed\n");
        goto cleanup;
    }
    /* Submit 已全部结束，预建接收页不会影响上面的性能时间。 */
    touch_trace_output_pages(
        trace_records,
        expected_trace_count * sizeof(*trace_records)
    );
    if (read_submit_trace(fd, trace_records, (uint32_t)expected_trace_count,
                          &trace_query) < 0) {
        fprintf(stderr, "RKNPU submit trace read failed: errno=%d (%s)\n",
                errno, strerror(errno));
        goto cleanup;
    }
    printf("trace records: %u/%zu overflow=%u\n",
           trace_query.count, expected_trace_count, trace_query.overflowed);
    if (trace_query.count == expected_trace_count && trace_query.overflowed == 0U) {
        printf("four-stage latency (measured submits: %zu)\n",
               expected_trace_count - warmup_trace_count);
        trace_ok = report_submit_trace(trace_records, trace_query.count,
                                       warmup_trace_count) == 0;
    } else {
        fprintf(stderr, "submit trace is incomplete\n");
    }

    /* 汇总每个线程的正式轮 blocking ioctl 延迟样本。 */
    for (uint32_t index = 0; index < options->threads; index++) {
        print_worker_result(&workers[index], total_rounds);
        all_sample_count += workers[index].latency_sample_count;
    }

    if (all_sample_count != expected_measured_count ||
        measured_submit_count != expected_measured_count) {
        fprintf(stderr,
                "incomplete measured submits: samples=%zu completed=%zu expected=%zu\n",
                all_sample_count, measured_submit_count, expected_measured_count);
    } else {
        all_samples = malloc(all_sample_count * sizeof(*all_samples));
        all_gate_wait_samples = malloc(all_sample_count * sizeof(*all_gate_wait_samples));
        all_request_samples = malloc(all_sample_count * sizeof(*all_request_samples));
        if (all_samples == NULL || all_gate_wait_samples == NULL || all_request_samples == NULL) {
            fprintf(stderr, "latency sample allocation failed\n");
            goto cleanup;
        }

        /* 三类样本保持完全相同的 Submit 顺序，便于比较 gate/ioctl/end-to-end。 */
        for (uint32_t index = 0; index < options->threads; index++) {
            size_t bytes = workers[index].latency_sample_count * sizeof(*all_samples);
            memcpy(&all_samples[sample_offset],
                   workers[index].latency_samples_us, bytes);
            memcpy(&all_gate_wait_samples[sample_offset],
                   workers[index].gate_wait_samples_us, bytes);
            memcpy(&all_request_samples[sample_offset],
                   workers[index].request_latency_samples_us, bytes);
            sample_offset += workers[index].latency_sample_count;
        }

        {
            rknpu_performance_report_t report = {
                .scenario = scenario,
                .npu_cores = options->npu_cores,
                .threads = options->threads,
                .measured_rounds = options->rounds,
                .submit_samples_us = all_samples,
                .submit_sample_count = all_sample_count,
                .successful_submit_count = measured_submit_count,
                .measured_window_us = measured_window_us,
                .total_dma_bytes = total_dma_bytes,
                .setup_operands_us = setup_operands_us,
                .build_regcmds_us = build_regcmds_us,
            };

            /* 性能报告复用 core_scaling 的统计接口和任务吞吐量公式。 */
            if (rknpu_print_performance_report(&report) != 0) {
                fprintf(stderr, "performance statistics failed\n");
                goto cleanup;
            }
            printf("submit admission latency (includes only measured submits)\n");
            if (print_user_latency_stats(
                    "gate_wait", all_gate_wait_samples, all_sample_count) != 0 ||
                print_user_latency_stats(
                    "request_e2e", all_request_samples, all_sample_count) != 0) {
                fprintf(stderr, "submit admission latency statistics failed\n");
                goto cleanup;
            }
            printf("  peak gate active : %u/%u\n",
                   submit_gate.peak_active, submit_gate.limit);
            metrics_ok = 1;
        }
    }

    /*
     * 正式性能与四阶段样本结束后另开短连续窗口。Schedule Trace 的额外记录锁
     * 不进入上面的性能数字，窗口只用于证明物理核心实际派发和忙碌时间。
     */
    core_usage_ok = run_core_usage_window(
        fd, workers, options->threads, options->npu_cores, scenario->task_count,
        &submit_gate
    ) == 0;

    /* 11. 两类 trace、性能样本及所有 worker 均完整时才 PASS。 */
    result = trace_ok && metrics_ok && core_usage_ok ? 0 : 1;
    for (uint32_t index = 0; index < options->threads; index++) {
        if (workers[index].error != WORKER_OK ||
            workers[index].successful_submits != total_rounds ||
            workers[index].latency_sample_count != options->rounds) {
            result = 1;
        }
    }
    printf("rknpu_multithread_submit_core: %s\n", result == 0 ? "PASS" : "FAIL");

    /* 12. join 后销毁启动门和 DMA，最后关闭共享 fd。 */
cleanup:
    if (gate_initialized) {
        stream_gate_destroy(&gate);
    }
    if (submit_gate_initialized) {
        submit_gate_destroy(&submit_gate);
    }

    free(all_request_samples);
    free(all_gate_wait_samples);
    free(all_samples);
    free(trace_records);

    for (uint32_t index = 0; index < prepared_workers; index++) {
        release_worker_resources(&workers[index]);
        free(workers[index].request_latency_samples_us);
        free(workers[index].gate_wait_samples_us);
        free(workers[index].latency_samples_us);
    }

    /* 失败 worker 可能只创建了样本数组，尚未计入 prepared_workers。 */
    if (workers != NULL && prepared_workers < options->threads) {
        free(workers[prepared_workers].request_latency_samples_us);
        free(workers[prepared_workers].gate_wait_samples_us);
        free(workers[prepared_workers].latency_samples_us);
    }

    free(threads);
    free(workers);

    if (fd >= 0) {
        npu_close(fd);
    }
    return result;
}

/* 依次运行所选场景；每个场景单独清空和读取四阶段 trace。 */
int main(int argc, char **argv) {
    test_options_t options;
    const rknpu_scenario_case_t *scenarios;
    size_t scenario_count;
    int parse_result;
    int result = 0;

    parse_result = parse_options(argc, argv, &options);
    if (parse_result != 0) {
        return parse_result > 0 ? 0 : 2;
    }

    scenarios = rknpu_scenario_cases(&scenario_count);
    for (size_t index = 0; index < scenario_count; index++) {
        if (!scenario_selected(&options, scenarios[index].name)) {
            continue;
        }
        if (run_scenario(&options, &scenarios[index]) != 0) {
            result = 1;
        }
    }
    printf("rknpu_multithread_submit_core all selected scenarios: %s\n",
           result == 0 ? "PASS" : "FAIL");
    return result;
}
