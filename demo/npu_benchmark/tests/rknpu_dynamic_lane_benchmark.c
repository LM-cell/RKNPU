/*
 * RKNPU Dynamic Lane 与 Submit Admission 对照测试，最后修改日期：2026-08-18。
 *
 * 每个 blocking Submit 固定包含 48 个相互独立的矩阵乘法 Task，四种形状各
 * 12 个。clustered 布局故意把不同耗时的 Task 集中到三条静态 Lane，
 * interleaved 布局作为相对均衡对照。--mode dynamic 只增加调度标志，负载、
 * GEM 和 Task 数量保持不变。
 *
 * 正式性能阶段记录 blocking ioctl 延迟和标准完成吞吐；调度 trace 在正式阶段
 * 结束后使用短窗口单独采集，避免逐事件记录改变正式性能结论。
 */

#include <errno.h>
#include <pthread.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/ioctl.h>
#include <time.h>

#include "npu_interface.h"
#include "rknpu-ioctl.h"
#include "rknpu_mixed_workload.h"

#define DEFAULT_THREADS 6U
#define DEFAULT_ROUNDS 100U
#define DEFAULT_WARMUP_ROUNDS 2U
#define DEFAULT_TRACE_ROUNDS 5U
#define NPU_CORES 3U
#define NPU_CORE_MASK 0x7U
#define MAX_THREADS 64U
#define USER_PAGE_SIZE 4096U

_Static_assert(sizeof(struct rknpu_submit) == 104,
               "RKNPU submit ABI must be 104 bytes");
_Static_assert(sizeof(struct rknpu_submit_trace_record) == 48,
               "RKNPU submit trace record ABI must be 48 bytes");
_Static_assert(sizeof(struct rknpu_schedule_trace_record) == 56,
               "RKNPU schedule trace record ABI must be 56 bytes");

typedef struct {
    uint64_t min;
    uint64_t max;
    uint64_t p50;
    uint64_t p95;
    uint64_t p99;
    double mean;
} latency_stats_t;

/* 所有 worker 只在正式阶段开始前同步一次。 */
typedef struct {
    pthread_mutex_t mutex;
    pthread_cond_t cond;
    uint32_t expected;
    uint32_t ready;
    int started;
    int stop;
} start_gate_t;

/* 只包围 blocking ioctl，用于限制 driver-visible 的 live Submit 数量。 */
typedef struct {
    pthread_mutex_t mutex;
    pthread_cond_t cond;
    uint32_t capacity;
    uint32_t active;
    uint32_t peak_active;
} submit_gate_t;

typedef struct {
    uint32_t thread_id;
    int fd;
    uint32_t warmup_rounds;
    uint32_t measured_rounds;
    start_gate_t *start_gate;
    submit_gate_t *submit_gate;
    rknpu_mixed_workload_t *workload;
    uint64_t *samples_ns;
    uint64_t first_submit_ns;
    uint64_t last_submit_ns;
    uint32_t completed;
    int error;
    int error_errno;
    rknpu_mixed_check_t check;
} worker_t;

typedef struct {
    uint32_t thread_id;
    int fd;
    uint32_t rounds;
    start_gate_t *start_gate;
    submit_gate_t *submit_gate;
    rknpu_mixed_workload_t *workload;
    uint32_t completed;
    int error_errno;
    rknpu_mixed_check_t check;
} trace_worker_t;

typedef struct {
    uint32_t threads;
    uint32_t rounds;
    uint32_t warmup_rounds;
    uint32_t trace_rounds;
    uint32_t submit_limit;
    int dynamic_tasks;
    rknpu_mixed_layout_t layout;
} options_t;

/* 每个 Submit 的 trace 校验状态；48 个 Task 使用一个 64 位位图。 */
typedef struct {
    uint64_t queue_task;
    uint64_t dispatched;
    uint64_t completed;
    uint8_t dispatch_core[RKNPU_MIXED_TASK_COUNT];
    uint8_t active_per_lane[5];
    uint8_t max_same_lane_parallel;
} traced_submit_t;

typedef struct {
    int busy;
    uint64_t dispatch_ns;
    uint64_t last_complete_ns;
    uint64_t busy_ns;
    uint64_t *refill_samples_ns;
    size_t refill_count;
    size_t dispatch_count;
    size_t complete_count;
} traced_core_t;

static uint64_t now_ns(void) {
    struct timespec ts;

    clock_gettime(CLOCK_MONOTONIC, &ts);
    return (uint64_t)ts.tv_sec * 1000000000ULL + (uint64_t)ts.tv_nsec;
}

static int compare_u64(const void *left_ptr, const void *right_ptr) {
    uint64_t left = *(const uint64_t *)left_ptr;
    uint64_t right = *(const uint64_t *)right_ptr;

    return (left > right) - (left < right);
}

/* 样本后续不再按原顺序使用，因此直接原地排序，不做额外防御性拷贝。 */
static int calculate_stats(uint64_t *samples, size_t count, latency_stats_t *stats) {
    uint64_t total = 0;

    if (samples == NULL || stats == NULL || count == 0U) {
        return -1;
    }
    qsort(samples, count, sizeof(*samples), compare_u64);
    for (size_t index = 0; index < count; index++) {
        total += samples[index];
    }
    stats->min = samples[0];
    stats->max = samples[count - 1U];
    stats->p50 = samples[(count * 50U + 99U) / 100U - 1U];
    stats->p95 = samples[(count * 95U + 99U) / 100U - 1U];
    stats->p99 = samples[(count * 99U + 99U) / 100U - 1U];
    stats->mean = (double)total / (double)count;
    return 0;
}

static int start_gate_init(start_gate_t *gate, uint32_t expected) {
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

static void start_gate_destroy(start_gate_t *gate) {
    pthread_cond_destroy(&gate->cond);
    pthread_mutex_destroy(&gate->mutex);
}

/* worker 报到后等待主线程完成 trace 复位并统一放行。 */
static int start_gate_worker_wait(start_gate_t *gate) {
    int stop;

    pthread_mutex_lock(&gate->mutex);
    gate->ready++;
    pthread_cond_broadcast(&gate->cond);
    while (!gate->started && !gate->stop) {
        pthread_cond_wait(&gate->cond, &gate->mutex);
    }
    stop = gate->stop;
    pthread_mutex_unlock(&gate->mutex);
    return stop ? -1 : 0;
}

static void start_gate_wait_ready(start_gate_t *gate) {
    pthread_mutex_lock(&gate->mutex);
    while (gate->ready < gate->expected) {
        pthread_cond_wait(&gate->cond, &gate->mutex);
    }
    pthread_mutex_unlock(&gate->mutex);
}

static void start_gate_release(start_gate_t *gate, int stop) {
    pthread_mutex_lock(&gate->mutex);
    gate->stop = stop;
    gate->started = !stop;
    pthread_cond_broadcast(&gate->cond);
    pthread_mutex_unlock(&gate->mutex);
}

static int submit_gate_init(submit_gate_t *gate, uint32_t capacity) {
    memset(gate, 0, sizeof(*gate));
    gate->capacity = capacity;
    if (capacity == 0U || pthread_mutex_init(&gate->mutex, NULL) != 0) {
        return -1;
    }
    if (pthread_cond_init(&gate->cond, NULL) != 0) {
        pthread_mutex_destroy(&gate->mutex);
        return -1;
    }
    return 0;
}

static void submit_gate_destroy(submit_gate_t *gate) {
    pthread_cond_destroy(&gate->cond);
    pthread_mutex_destroy(&gate->mutex);
}

static void submit_gate_acquire(submit_gate_t *gate) {
    pthread_mutex_lock(&gate->mutex);
    while (gate->active >= gate->capacity) {
        pthread_cond_wait(&gate->cond, &gate->mutex);
    }
    gate->active++;
    if (gate->active > gate->peak_active) {
        gate->peak_active = gate->active;
    }
    pthread_mutex_unlock(&gate->mutex);
}

static void submit_gate_release(submit_gate_t *gate) {
    pthread_mutex_lock(&gate->mutex);
    gate->active--;
    pthread_cond_signal(&gate->cond);
    pthread_mutex_unlock(&gate->mutex);
}

/* Gate 等待不计入单次 ioctl 延迟，但自然包含在全局完成吞吐窗口中。 */
static int run_gated_submit(
    submit_gate_t *gate,
    int fd,
    struct rknpu_submit *submit,
    uint64_t *start_ns,
    uint64_t *end_ns
) {
    int result;
    int saved_errno;

    submit_gate_acquire(gate);
    if (start_ns != NULL) {
        *start_ns = now_ns();
    }
    result = ioctl(fd, DRM_IOCTL_RKNPU_SUBMIT, submit);
    saved_errno = errno;
    if (end_ns != NULL) {
        *end_ns = now_ns();
    }
    submit_gate_release(gate);
    errno = saved_errno;
    return result;
}

static void touch_output_pages(void *buffer, size_t bytes) {
    volatile unsigned char *output = buffer;

    if (buffer == NULL || bytes == 0U) {
        return;
    }
    for (size_t offset = 0; offset < bytes; offset += USER_PAGE_SIZE) {
        output[offset] = 0;
    }
    output[bytes - 1U] = 0;
}

static int reset_submit_trace(int fd) {
    struct rknpu_submit_trace_query query;

    memset(&query, 0, sizeof(query));
    query.operation = RKNPU_SUBMIT_TRACE_RESET;
    return ioctl(fd, DRM_IOCTL_RKNPU_SUBMIT_TRACE, &query);
}

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

static int configure_schedule_trace(int fd, uint32_t event_mask) {
    struct rknpu_schedule_trace_query query;

    memset(&query, 0, sizeof(query));
    query.operation = RKNPU_SCHEDULE_TRACE_CONFIG_RESET;
    query.event_mask = event_mask;
    return ioctl(fd, DRM_IOCTL_RKNPU_SCHEDULE_TRACE, &query);
}

/* 正式性能前关闭旧的 Worker yield 实验，避免上一次异常退出遗留采集状态。 */
static int disable_worker_yield_trace(int fd) {
    struct rknpu_worker_yield_trace_query query;

    memset(&query, 0, sizeof(query));
    query.operation = RKNPU_WORKER_YIELD_TRACE_CONFIG_RESET;
    query.enabled = 0;
    return ioctl(fd, DRM_IOCTL_RKNPU_WORKER_YIELD_TRACE, &query);
}

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

static int read_scheduler_state(int fd, struct rknpu_scheduler_state_snapshot *state) {
    struct rknpu_schedule_trace_query query;

    memset(&query, 0, sizeof(query));
    memset(state, 0, sizeof(*state));
    query.operation = RKNPU_SCHEDULE_TRACE_STATE;
    query.state_address = (uint64_t)(uintptr_t)state;
    return ioctl(fd, DRM_IOCTL_RKNPU_SCHEDULE_TRACE, &query);
}

static int check_workload(
    const rknpu_mixed_workload_t *workload,
    const struct rknpu_submit *submit,
    rknpu_mixed_check_t *check
) {
    return rknpu_mixed_workload_check(workload, submit, check);
}

static void *worker_main(void *opaque) {
    worker_t *worker = opaque;
    struct rknpu_submit submit;

    /* 预热每轮都校验，正式阶段只在连续流水结束后校验最后一轮。 */
    for (uint32_t round = 0; round < worker->warmup_rounds; round++) {
        rknpu_mixed_workload_begin(worker->workload, &submit);
        if (run_gated_submit(worker->submit_gate, worker->fd, &submit, NULL, NULL) < 0) {
            worker->error = 1;
            worker->error_errno = errno;
            break;
        }
        if (check_workload(worker->workload, &submit, &worker->check) != 0) {
            worker->error = 2;
            break;
        }
    }

    rknpu_mixed_workload_begin(worker->workload, &submit);
    if (start_gate_worker_wait(worker->start_gate) != 0 || worker->error != 0) {
        return NULL;
    }

    for (uint32_t round = 0; round < worker->measured_rounds; round++) {
        uint64_t start_ns;
        uint64_t end_ns;

        if (round != 0U) {
            rknpu_mixed_workload_next_submit(worker->workload, &submit);
        }
        if (run_gated_submit(
                worker->submit_gate, worker->fd, &submit, &start_ns, &end_ns
            ) < 0) {
            worker->error = 1;
            worker->error_errno = errno;
            break;
        }
        if (worker->completed == 0U) {
            worker->first_submit_ns = start_ns;
        }
        worker->last_submit_ns = end_ns;
        worker->samples_ns[worker->completed] = end_ns - start_ns;
        worker->completed++;
    }

    if (worker->error == 0 &&
        check_workload(worker->workload, &submit, &worker->check) != 0) {
        worker->error = 2;
    }
    return NULL;
}

static void *trace_worker_main(void *opaque) {
    trace_worker_t *worker = opaque;
    struct rknpu_submit submit;

    rknpu_mixed_workload_begin(worker->workload, &submit);
    if (start_gate_worker_wait(worker->start_gate) != 0) {
        return NULL;
    }
    for (uint32_t round = 0; round < worker->rounds; round++) {
        if (round != 0U) {
            rknpu_mixed_workload_next_submit(worker->workload, &submit);
        }
        if (run_gated_submit(worker->submit_gate, worker->fd, &submit, NULL, NULL) < 0) {
            worker->error_errno = errno;
            break;
        }
        worker->completed++;
    }
    if (worker->error_errno == 0 &&
        check_workload(worker->workload, &submit, &worker->check) != 0) {
        worker->error_errno = EIO;
    }
    return NULL;
}

static void print_check_error(uint32_t thread_id, const rknpu_mixed_check_t *check) {
    fprintf(stderr,
            "thread[%u] check failed: type=%d task=%u counter=%u irq=%#x "
            "position=(%u,%u) expected=%f actual=%f\n",
            thread_id, check->error, check->task_index, check->task_counter,
            check->irq_status, check->row, check->column,
            check->expected, check->actual);
}

static int report_submit_latency(uint64_t *samples, size_t count, uint64_t window_ns) {
    latency_stats_t stats;
    double tasks_per_second;

    if (calculate_stats(samples, count, &stats) != 0 || window_ns == 0U) {
        return -1;
    }
    tasks_per_second =
        (double)(count * RKNPU_MIXED_TASK_COUNT) * 1000000000.0 / (double)window_ns;

    printf("正式性能指标\n");
    printf("  measured Submit : %zu\n", count);
    printf("  Mean             : %.3f ms\n", stats.mean / 1000000.0);
    printf("  P50              : %.3f ms\n", (double)stats.p50 / 1000000.0);
    printf("  P95              : %.3f ms\n", (double)stats.p95 / 1000000.0);
    printf("  P99              : %.3f ms\n", (double)stats.p99 / 1000000.0);
    printf("  Min / Max        : %.3f / %.3f ms\n",
           (double)stats.min / 1000000.0, (double)stats.max / 1000000.0);
    printf("  Throughput       : %.2f tasks/s\n", tasks_per_second);
    return 0;
}

/* 四阶段使用驱动原始单调时钟，正式阶段结束后一次性统计。 */
static int report_four_stage_latency(
    const struct rknpu_submit_trace_record *records,
    size_t count
) {
    const char *names[4] = {
        "submit_prepare", "queue_wait", "dispatch_execute", "complete_return"
    };
    uint64_t *samples;

    samples = calloc(count * 4U, sizeof(*samples));
    if (samples == NULL) {
        return -1;
    }
    for (size_t index = 0; index < count; index++) {
        const struct rknpu_submit_trace_record *record = &records[index];

        if (record->queue_task == 0U || record->t0_ns > record->t1_ns ||
            record->t1_ns > record->t2_ns || record->t2_ns > record->t3_ns ||
            record->t3_ns > record->t4_ns) {
            fprintf(stderr, "invalid submit trace at index=%zu\n", index);
            free(samples);
            return -1;
        }
        samples[index] = record->t1_ns - record->t0_ns;
        samples[count + index] = record->t2_ns - record->t1_ns;
        samples[count * 2U + index] = record->t3_ns - record->t2_ns;
        samples[count * 3U + index] = record->t4_ns - record->t3_ns;
    }

    printf("四阶段延迟（正式 Submit，驱动单调时钟）\n");
    for (size_t stage = 0; stage < 4U; stage++) {
        latency_stats_t stats;
        uint64_t *stage_samples = samples + stage * count;

        calculate_stats(stage_samples, count, &stats);
        printf("  %-16s Mean/P50/P95/P99 = %.3f / %.3f / %.3f / %.3f ms\n",
               names[stage], stats.mean / 1000000.0,
               (double)stats.p50 / 1000000.0,
               (double)stats.p95 / 1000000.0,
               (double)stats.p99 / 1000000.0);
    }
    free(samples);
    return 0;
}

static int compare_schedule_record(const void *left_ptr, const void *right_ptr) {
    const struct rknpu_schedule_trace_record *left = left_ptr;
    const struct rknpu_schedule_trace_record *right = right_ptr;

    if (left->timestamp_ns != right->timestamp_ns) {
        return left->timestamp_ns < right->timestamp_ns ? -1 : 1;
    }
    return left->sequence < right->sequence ? -1 : left->sequence > right->sequence;
}

static traced_submit_t *find_traced_submit(
    traced_submit_t *submits,
    size_t submit_count,
    uint64_t queue_task
) {
    for (size_t index = 0; index < submit_count; index++) {
        if (submits[index].queue_task == queue_task) {
            return &submits[index];
        }
    }
    return NULL;
}

static uint32_t busy_core_count(const traced_core_t *cores) {
    uint32_t count = 0;

    for (uint32_t core = 0; core < NPU_CORES; core++) {
        count += cores[core].busy ? 1U : 0U;
    }
    return count;
}

/*
 * 调度短窗口同时验证：Task 不重不漏、完成核心匹配、同 Lane 并行峰值、
 * 每核 Task 类型分布、refill 延迟、每核利用率和三核同时忙比例。
 */
static int report_schedule_trace(
    struct rknpu_schedule_trace_record *records,
    size_t record_count,
    size_t expected_submits,
    rknpu_mixed_layout_t layout
) {
    traced_submit_t *submits;
    traced_core_t cores[NPU_CORES] = {0};
    uint32_t type_per_core[NPU_CORES][RKNPU_MIXED_TYPE_COUNT] = {{0}};
    size_t submit_count = 0;
    uint64_t previous_ns = 0;
    uint64_t window_start_ns = 0;
    uint64_t window_end_ns = 0;
    uint64_t all_core_busy_ns = 0;
    uint8_t max_same_lane_parallel = 0;
    int result = -1;

    submits = calloc(expected_submits, sizeof(*submits));
    if (submits == NULL) {
        return -1;
    }
    for (uint32_t core = 0; core < NPU_CORES; core++) {
        cores[core].refill_samples_ns =
            calloc(record_count, sizeof(*cores[core].refill_samples_ns));
        if (cores[core].refill_samples_ns == NULL) {
            goto out;
        }
    }

    /* sequence 在排序前检查，任何缓冲区溢出或记录缺口都拒绝性能结论。 */
    for (size_t index = 0; index < record_count; index++) {
        if (records[index].sequence != index || records[index].queue_task == 0U) {
            fprintf(stderr, "invalid schedule trace sequence at index=%zu\n", index);
            goto out;
        }
    }
    qsort(records, record_count, sizeof(*records), compare_schedule_record);

    for (size_t index = 0; index < record_count; index++) {
        struct rknpu_schedule_trace_record *record = &records[index];
        traced_submit_t *submit;

        if (window_start_ns != 0U && record->timestamp_ns >= previous_ns &&
            busy_core_count(cores) == NPU_CORES) {
            all_core_busy_ns += record->timestamp_ns - previous_ns;
        }
        if (window_start_ns != 0U) {
            previous_ns = record->timestamp_ns;
        }

        if (record->event_type == RKNPU_SCHEDULE_EVENT_ENQUEUE) {
            if (submit_count >= expected_submits ||
                find_traced_submit(submits, submit_count, record->queue_task) != NULL) {
                fprintf(stderr, "duplicate or excess enqueue queue_task=%llu\n",
                        (unsigned long long)record->queue_task);
                goto out;
            }
            submits[submit_count++].queue_task = record->queue_task;
            continue;
        }

        submit = find_traced_submit(submits, submit_count, record->queue_task);
        if (submit == NULL || record->task_index >= RKNPU_MIXED_TASK_COUNT ||
            record->core_slot >= NPU_CORES || record->lane_slot >= 5U) {
            fprintf(stderr, "unmatched schedule event at sorted index=%zu\n", index);
            goto out;
        }

        if (record->event_type == RKNPU_SCHEDULE_EVENT_DISPATCH) {
            uint64_t bit = 1ULL << record->task_index;
            traced_core_t *core = &cores[record->core_slot];
            rknpu_mixed_task_type_t type =
                rknpu_mixed_task_type(layout, record->task_index);

            if ((submit->dispatched & bit) != 0U || core->busy) {
                fprintf(stderr, "duplicate dispatch task=%u core=%u\n",
                        record->task_index, record->core_slot);
                goto out;
            }
            if (window_start_ns == 0U) {
                window_start_ns = record->timestamp_ns;
                previous_ns = record->timestamp_ns;
            }
            if (core->last_complete_ns != 0U) {
                core->refill_samples_ns[core->refill_count++] =
                    record->timestamp_ns - core->last_complete_ns;
            }
            core->busy = 1;
            core->dispatch_ns = record->timestamp_ns;
            core->dispatch_count++;
            submit->dispatched |= bit;
            submit->dispatch_core[record->task_index] = (uint8_t)record->core_slot;
            submit->active_per_lane[record->lane_slot]++;
            if (submit->active_per_lane[record->lane_slot] >
                submit->max_same_lane_parallel) {
                submit->max_same_lane_parallel =
                    submit->active_per_lane[record->lane_slot];
            }
            if (submit->max_same_lane_parallel > max_same_lane_parallel) {
                max_same_lane_parallel = submit->max_same_lane_parallel;
            }
            type_per_core[record->core_slot][type]++;
        } else if (record->event_type == RKNPU_SCHEDULE_EVENT_COMPLETE) {
            uint64_t bit = 1ULL << record->task_index;
            traced_core_t *core = &cores[record->core_slot];

            if ((submit->dispatched & bit) == 0U || (submit->completed & bit) != 0U ||
                submit->dispatch_core[record->task_index] != record->core_slot ||
                !core->busy || submit->active_per_lane[record->lane_slot] == 0U) {
                fprintf(stderr, "unmatched completion task=%u core=%u\n",
                        record->task_index, record->core_slot);
                goto out;
            }
            submit->completed |= bit;
            submit->active_per_lane[record->lane_slot]--;
            core->busy = 0;
            core->busy_ns += record->timestamp_ns - core->dispatch_ns;
            core->last_complete_ns = record->timestamp_ns;
            core->complete_count++;
            window_end_ns = record->timestamp_ns;
        } else {
            fprintf(stderr, "failed or unknown schedule event type=%u\n", record->event_type);
            goto out;
        }
    }

    if (submit_count != expected_submits || window_end_ns <= window_start_ns) {
        fprintf(stderr, "schedule trace submit/window mismatch\n");
        goto out;
    }
    for (size_t index = 0; index < submit_count; index++) {
        uint64_t expected_mask = (1ULL << RKNPU_MIXED_TASK_COUNT) - 1ULL;

        if (submits[index].dispatched != expected_mask ||
            submits[index].completed != expected_mask) {
            fprintf(stderr, "missing task queue_task=%llu\n",
                    (unsigned long long)submits[index].queue_task);
            goto out;
        }
    }

    printf("调度短窗口（不计入正式性能）\n");
    printf("  traced Submit            : %zu\n", submit_count);
    printf("  same-lane parallel peak  : %u\n", max_same_lane_parallel);
    printf("  all-core busy ratio      : %.2f %%\n",
           (double)all_core_busy_ns * 100.0 /
               (double)(window_end_ns - window_start_ns));
    for (uint32_t core_index = 0; core_index < NPU_CORES; core_index++) {
        traced_core_t *core = &cores[core_index];

        printf("  Core%u utilization        : %.2f %% (%zu tasks)\n",
               core_index,
               (double)core->busy_ns * 100.0 /
                   (double)(window_end_ns - window_start_ns),
               core->dispatch_count);
        if (core->refill_count != 0U) {
            latency_stats_t stats;
            calculate_stats(core->refill_samples_ns, core->refill_count, &stats);
            printf("    refill Mean/P50/P95/P99: %.3f / %.3f / %.3f / %.3f ms\n",
                   stats.mean / 1000000.0, (double)stats.p50 / 1000000.0,
                   (double)stats.p95 / 1000000.0,
                   (double)stats.p99 / 1000000.0);
        }
        printf("    type tasks tiny/mid/heavy/llama: %u/%u/%u/%u\n",
               type_per_core[core_index][RKNPU_MIXED_TINY],
               type_per_core[core_index][RKNPU_MIXED_MID],
               type_per_core[core_index][RKNPU_MIXED_HEAVY],
               type_per_core[core_index][RKNPU_MIXED_LLAMA]);
    }
    result = 0;

out:
    for (uint32_t core = 0; core < NPU_CORES; core++) {
        free(cores[core].refill_samples_ns);
    }
    free(submits);
    return result;
}

static int run_trace_window(
    const options_t *options,
    int fd,
    rknpu_mixed_workload_t **workloads
) {
    size_t expected_submits = (size_t)options->threads * options->trace_rounds;
    size_t expected_records = expected_submits * (1U + 2U * RKNPU_MIXED_TASK_COUNT);
    struct rknpu_schedule_trace_record *records = NULL;
    struct rknpu_schedule_trace_query query;
    trace_worker_t *workers = NULL;
    pthread_t *threads = NULL;
    start_gate_t start_gate;
    submit_gate_t submit_gate;
    uint32_t created = 0;
    int start_gate_ready = 0;
    int submit_gate_ready = 0;
    int result = -1;

    if (options->trace_rounds == 0U) {
        return 0;
    }
    if (expected_records > RKNPU_SCHEDULE_TRACE_CAPACITY) {
        fprintf(stderr, "trace window needs %zu records, capacity is %u\n",
                expected_records, RKNPU_SCHEDULE_TRACE_CAPACITY);
        return -1;
    }
    records = calloc(expected_records, sizeof(*records));
    workers = calloc(options->threads, sizeof(*workers));
    threads = calloc(options->threads, sizeof(*threads));
    if (records == NULL || workers == NULL || threads == NULL) {
        goto out_alloc;
    }
    if (start_gate_init(&start_gate, options->threads) != 0) {
        goto out_alloc;
    }
    start_gate_ready = 1;
    if (submit_gate_init(&submit_gate, options->submit_limit) != 0) {
        goto out_gates;
    }
    submit_gate_ready = 1;
    touch_output_pages(records, expected_records * sizeof(*records));
    if (configure_schedule_trace(fd, RKNPU_SCHEDULE_EVENT_ALL) < 0) {
        goto out_gates;
    }

    for (uint32_t index = 0; index < options->threads; index++) {
        workers[index].thread_id = index;
        workers[index].fd = fd;
        workers[index].rounds = options->trace_rounds;
        workers[index].start_gate = &start_gate;
        workers[index].submit_gate = &submit_gate;
        workers[index].workload = workloads[index];
        if (pthread_create(&threads[index], NULL, trace_worker_main, &workers[index]) != 0) {
            start_gate_release(&start_gate, 1);
            goto join;
        }
        created++;
    }
    start_gate_wait_ready(&start_gate);
    start_gate_release(&start_gate, 0);

join:
    for (uint32_t index = 0; index < created; index++) {
        pthread_join(threads[index], NULL);
    }
    if (created != options->threads) {
        goto disable;
    }
    for (uint32_t index = 0; index < options->threads; index++) {
        if (workers[index].error_errno != 0 ||
            workers[index].completed != options->trace_rounds) {
            fprintf(stderr, "trace worker[%u] failed errno=%d completed=%u\n",
                    index, workers[index].error_errno, workers[index].completed);
            if (workers[index].check.error != RKNPU_MIXED_CHECK_OK) {
                print_check_error(index, &workers[index].check);
            }
            goto disable;
        }
    }
    if (read_schedule_trace(fd, records, (uint32_t)expected_records, &query) < 0 ||
        query.overflowed != 0U || query.count != expected_records) {
        fprintf(stderr, "schedule trace read mismatch count=%u expected=%zu overflow=%u\n",
                query.count, expected_records, query.overflowed);
        goto disable;
    }
    result = report_schedule_trace(
        records, query.count, expected_submits, options->layout
    );

disable:
    if (configure_schedule_trace(fd, 0) < 0) {
        result = -1;
    }
out_gates:
    if (submit_gate_ready) {
        submit_gate_destroy(&submit_gate);
    }
    if (start_gate_ready) {
        start_gate_destroy(&start_gate);
    }
out_alloc:
    free(threads);
    free(workers);
    free(records);
    return result;
}

static int parse_u32(const char *text, uint32_t *value) {
    char *end;
    unsigned long parsed;

    errno = 0;
    parsed = strtoul(text, &end, 0);
    if (errno != 0 || *text == '\0' || *end != '\0' || parsed > UINT32_MAX) {
        return -1;
    }
    *value = (uint32_t)parsed;
    return 0;
}

static void print_usage(const char *program) {
    printf("Usage: %s [options]\n", program);
    printf("  --mode static|dynamic          default: static\n");
    printf("  --layout clustered|interleaved default: clustered\n");
    printf("  --threads N                    default: 6\n");
    printf("  --submit-limit N               default: threads\n");
    printf("  --rounds N                     default: 100\n");
    printf("  --trace-rounds N               default: 5, 0 disables\n");
}

static int parse_options(int argc, char **argv, options_t *options) {
    int submit_limit_set = 0;

    memset(options, 0, sizeof(*options));
    options->threads = DEFAULT_THREADS;
    options->rounds = DEFAULT_ROUNDS;
    options->warmup_rounds = DEFAULT_WARMUP_ROUNDS;
    options->trace_rounds = DEFAULT_TRACE_ROUNDS;
    options->layout = RKNPU_MIXED_CLUSTERED;

    for (int index = 1; index < argc; index++) {
        const char *arg = argv[index];

        if (strcmp(arg, "--help") == 0) {
            print_usage(argv[0]);
            return 1;
        }
        if (index + 1 >= argc) {
            return -1;
        }
        index++;
        if (strcmp(arg, "--mode") == 0) {
            if (strcmp(argv[index], "dynamic") == 0) {
                options->dynamic_tasks = 1;
            } else if (strcmp(argv[index], "static") != 0) {
                return -1;
            }
        } else if (strcmp(arg, "--layout") == 0) {
            if (strcmp(argv[index], "interleaved") == 0) {
                options->layout = RKNPU_MIXED_INTERLEAVED;
            } else if (strcmp(argv[index], "clustered") != 0) {
                return -1;
            }
        } else if (strcmp(arg, "--threads") == 0) {
            if (parse_u32(argv[index], &options->threads) != 0) {
                return -1;
            }
        } else if (strcmp(arg, "--submit-limit") == 0) {
            if (parse_u32(argv[index], &options->submit_limit) != 0) {
                return -1;
            }
            submit_limit_set = 1;
        } else if (strcmp(arg, "--rounds") == 0) {
            if (parse_u32(argv[index], &options->rounds) != 0) {
                return -1;
            }
        } else if (strcmp(arg, "--trace-rounds") == 0) {
            if (parse_u32(argv[index], &options->trace_rounds) != 0) {
                return -1;
            }
        } else {
            return -1;
        }
    }

    if (!submit_limit_set) {
        options->submit_limit = options->threads;
    }
    if (options->threads == 0U || options->threads > MAX_THREADS ||
        options->rounds == 0U || options->submit_limit == 0U ||
        options->submit_limit > options->threads ||
        (size_t)options->threads * options->rounds > RKNPU_SUBMIT_TRACE_CAPACITY) {
        return -1;
    }
    return 0;
}

static int same_state(
    const struct rknpu_scheduler_state_snapshot *left,
    const struct rknpu_scheduler_state_snapshot *right
) {
    return left->live_submits == right->live_submits &&
        left->ready_entries == right->ready_entries &&
        left->running_entries == right->running_entries &&
        left->complete_entries == right->complete_entries &&
        left->waiters == right->waiters &&
        left->core_bindings == right->core_bindings &&
        left->gem_buffers == right->gem_buffers &&
        left->gem_bytes == right->gem_bytes;
}

int main(int argc, char **argv) {
    options_t options;
    int parse_result;
    int fd = -1;
    rknpu_mixed_workload_t **workloads = NULL;
    worker_t *workers = NULL;
    pthread_t *threads = NULL;
    uint64_t *all_samples = NULL;
    struct rknpu_submit_trace_record *submit_records = NULL;
    struct rknpu_submit_trace_query submit_query;
    struct rknpu_scheduler_state_snapshot state_before;
    struct rknpu_scheduler_state_snapshot state_after;
    start_gate_t start_gate;
    submit_gate_t submit_gate;
    size_t expected_submits;
    uint32_t created = 0;
    uint64_t window_start_ns = UINT64_MAX;
    uint64_t window_end_ns = 0;
    size_t total_dma_bytes = 0;
    uint64_t setup_operands_us = 0;
    uint64_t build_regcmds_us = 0;
    int start_gate_ready = 0;
    int submit_gate_ready = 0;
    int state_before_valid = 0;
    int result = EXIT_FAILURE;

    parse_result = parse_options(argc, argv, &options);
    if (parse_result != 0) {
        if (parse_result < 0) {
            print_usage(argv[0]);
        }
        return parse_result > 0 ? EXIT_SUCCESS : EXIT_FAILURE;
    }
    expected_submits = (size_t)options.threads * options.rounds;

    fd = npu_open();
    if (fd < 0 || npu_reset(fd) != 0 || configure_schedule_trace(fd, 0) < 0 ||
        disable_worker_yield_trace(fd) < 0 ||
        read_scheduler_state(fd, &state_before) < 0) {
        goto out;
    }
    state_before_valid = 1;

    printf("rknpu_dynamic_lane_benchmark\n");
    printf("  threads / submit_limit : %u / %u\n",
           options.threads, options.submit_limit);
    printf("  mode / layout          : %s / %s\n",
           options.dynamic_tasks ? "dynamic" : "static",
           rknpu_mixed_layout_name(options.layout));
    printf("  workload               : 48 tasks = 12 tiny + 12 mid + 12 heavy + 12 llama\n");
    printf("  cores / core mask      : 3 / 0x7\n");
    printf("  rounds                 : 2 warmup + %u measured\n", options.rounds);

    workloads = calloc(options.threads, sizeof(*workloads));
    workers = calloc(options.threads, sizeof(*workers));
    threads = calloc(options.threads, sizeof(*threads));
    all_samples = calloc(expected_submits, sizeof(*all_samples));
    submit_records = calloc(expected_submits, sizeof(*submit_records));
    if (workloads == NULL || workers == NULL || threads == NULL ||
        all_samples == NULL || submit_records == NULL) {
        goto out;
    }
    touch_output_pages(submit_records, expected_submits * sizeof(*submit_records));

    for (uint32_t index = 0; index < options.threads; index++) {
        rknpu_mixed_metrics_t metrics;

        if (rknpu_mixed_workload_create(
                fd, index, NPU_CORES, NPU_CORE_MASK, options.layout,
                options.dynamic_tasks, &workloads[index]
            ) != 0) {
            fprintf(stderr, "failed to create workload for thread[%u]\n", index);
            goto out;
        }
        rknpu_mixed_workload_get_metrics(workloads[index], &metrics);
        total_dma_bytes += metrics.total_dma_bytes;
        setup_operands_us += metrics.setup_operands_us;
        build_regcmds_us += metrics.build_regcmds_us;
    }
    printf("  total DMA              : %.2f MiB\n",
           (double)total_dma_bytes / (1024.0 * 1024.0));
    printf("  setup operands / regcmd: %.3f / %.3f ms\n",
           (double)setup_operands_us / 1000.0,
           (double)build_regcmds_us / 1000.0);

    if (start_gate_init(&start_gate, options.threads) != 0) {
        goto out;
    }
    start_gate_ready = 1;
    if (submit_gate_init(&submit_gate, options.submit_limit) != 0) {
        goto out;
    }
    submit_gate_ready = 1;
    for (uint32_t index = 0; index < options.threads; index++) {
        workers[index].thread_id = index;
        workers[index].fd = fd;
        workers[index].warmup_rounds = options.warmup_rounds;
        workers[index].measured_rounds = options.rounds;
        workers[index].start_gate = &start_gate;
        workers[index].submit_gate = &submit_gate;
        workers[index].workload = workloads[index];
        workers[index].samples_ns = all_samples + (size_t)index * options.rounds;
        if (pthread_create(&threads[index], NULL, worker_main, &workers[index]) != 0) {
            start_gate_release(&start_gate, 1);
            goto join_workers;
        }
        created++;
    }

    start_gate_wait_ready(&start_gate);
    for (uint32_t index = 0; index < options.threads; index++) {
        if (workers[index].error != 0) {
            start_gate_release(&start_gate, 1);
            goto join_workers;
        }
    }
    if (reset_submit_trace(fd) < 0) {
        start_gate_release(&start_gate, 1);
        goto join_workers;
    }
    start_gate_release(&start_gate, 0);

join_workers:
    for (uint32_t index = 0; index < created; index++) {
        pthread_join(threads[index], NULL);
    }
    if (created != options.threads) {
        goto out;
    }
    for (uint32_t index = 0; index < options.threads; index++) {
        if (workers[index].error != 0 || workers[index].completed != options.rounds) {
            fprintf(stderr, "worker[%u] failed error=%d errno=%d completed=%u\n",
                    index, workers[index].error, workers[index].error_errno,
                    workers[index].completed);
            if (workers[index].check.error != RKNPU_MIXED_CHECK_OK) {
                print_check_error(index, &workers[index].check);
            }
            goto out;
        }
        if (workers[index].first_submit_ns < window_start_ns) {
            window_start_ns = workers[index].first_submit_ns;
        }
        if (workers[index].last_submit_ns > window_end_ns) {
            window_end_ns = workers[index].last_submit_ns;
        }
    }
    if (submit_gate.peak_active > options.submit_limit ||
        report_submit_latency(
            all_samples, expected_submits, window_end_ns - window_start_ns
        ) != 0) {
        goto out;
    }
    printf("  SubmitGate peak        : %u\n", submit_gate.peak_active);

    if (read_submit_trace(
            fd, submit_records, (uint32_t)expected_submits, &submit_query
        ) < 0 || submit_query.overflowed != 0U ||
        submit_query.count != expected_submits ||
        report_four_stage_latency(submit_records, submit_query.count) != 0) {
        fprintf(stderr, "submit trace mismatch count=%u expected=%zu overflow=%u\n",
                submit_query.count, expected_submits, submit_query.overflowed);
        goto out;
    }

    if (run_trace_window(&options, fd, workloads) != 0) {
        goto out;
    }
    result = EXIT_SUCCESS;

out:
    if (submit_gate_ready) {
        submit_gate_destroy(&submit_gate);
    }
    if (start_gate_ready) {
        start_gate_destroy(&start_gate);
    }
    if (workloads != NULL) {
        for (uint32_t index = 0; index < options.threads; index++) {
            rknpu_mixed_workload_destroy(workloads[index]);
        }
    }
    if (fd >= 0 && state_before_valid) {
        if (read_scheduler_state(fd, &state_after) < 0) {
            fprintf(stderr, "failed to read final scheduler state\n");
            result = EXIT_FAILURE;
        } else if (!same_state(&state_before, &state_after)) {
            fprintf(stderr,
                    "resource baseline mismatch: before gem=%u/%llu after gem=%u/%llu "
                    "live=%u ready=%u running=%u complete=%u waiters=%u bindings=%u\n",
                    state_before.gem_buffers,
                    (unsigned long long)state_before.gem_bytes,
                    state_after.gem_buffers,
                    (unsigned long long)state_after.gem_bytes,
                    state_after.live_submits, state_after.ready_entries,
                    state_after.running_entries, state_after.complete_entries,
                    state_after.waiters, state_after.core_bindings);
            result = EXIT_FAILURE;
        }
    }
    free(submit_records);
    free(all_samples);
    free(threads);
    free(workers);
    free(workloads);
    if (fd >= 0) {
        npu_close(fd);
    }

    printf("rknpu_dynamic_lane_benchmark: %s\n",
           result == EXIT_SUCCESS ? "PASS" : "FAIL");
    return result;
}
