/* 多线程核心数量参数化测试，最后修改日期：2026-08-05。 */

#include <errno.h>
#include <math.h>
#include <pthread.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/ioctl.h>
#include <sys/mman.h>
#include <time.h>
#include <unistd.h>

#include <libdrm/drm.h>

#include "benchmark_stats.h"
#include "npu_interface.h"
#include "npu_matmul.h"
#include "rknpu-ioctl.h"

#define TEST_M 64U
#define TEST_K 512U
#define TEST_N 512U
#define REGCMD_WORDS 112U
#define REGCMD_BYTES (REGCMD_WORDS * sizeof(uint64_t))
#define DEFAULT_THREADS 3U
#define DEFAULT_ROUNDS 100U
#define DEFAULT_WARMUP_ROUNDS 2U
#define DEFAULT_NPU_CORES 3U
#define MAX_NPU_CORES 3U
#define SUBMIT_TIMEOUT_MS 6000U
#define MAX_TEST_THREADS 64U
#define INPUT_LAYOUT_C2 8
#define OUTPUT_LAYOUT_C2 4

/* StarryOS extends the upstream 40-byte create request with two fields. */
struct rknpu_mem_create_starry {
    uint32_t handle;
    uint32_t flags;
    uint64_t size;
    uint64_t obj_addr;
    uint64_t dma_addr;
    uint64_t sram_size;
    int32_t iommu_domain_id;
    uint32_t core_mask;
};

#define DRM_IOCTL_RKNPU_MEM_CREATE_STARRY                                    \
    DRM_IOWR(DRM_COMMAND_BASE + RKNPU_MEM_CREATE, struct rknpu_mem_create_starry)

_Static_assert(sizeof(struct rknpu_mem_create_starry) == 48,
               "StarryOS MEM_CREATE ABI must be 48 bytes");
_Static_assert(sizeof(struct rknpu_task) == 40, "RKNPU task ABI must be 40 bytes");
_Static_assert(sizeof(struct rknpu_submit) == 104, "RKNPU submit ABI must be 104 bytes");

typedef struct {
    void *map;
    size_t size;
    uint64_t dma_addr;
    uint64_t obj_addr;
    uint32_t handle;
    int allocated;
} dma_buffer_t;

typedef struct {
    dma_buffer_t regcmd;
    dma_buffer_t task;
    dma_buffer_t input;
    dma_buffer_t weights;
    dma_buffer_t output;
    struct rknpu_submit submit_template;
} worker_resources_t;

typedef struct {
    pthread_mutex_t mutex;
    pthread_cond_t cond;
    uint32_t expected_threads;
    uint32_t ready_threads;
    uint32_t done_threads;
    uint32_t generation;
    uint64_t first_submit_start_us;
    uint64_t last_submit_end_us;
    uint32_t completed_submits;
    int stop;
} round_gate_t;

typedef enum {
    WORKER_OK = 0,
    WORKER_IOCTL_FAILED,
    WORKER_TASK_COUNTER_FAILED,
    WORKER_IRQ_STATUS_FAILED,
    WORKER_OUTPUT_FAILED,
} worker_error_t;

typedef struct {
    uint32_t thread_id;
    int fd;
    uint32_t core_mask;
    uint32_t warmup_rounds;
    uint32_t measure_rounds;
    round_gate_t *gate;
    worker_resources_t resources;
    uint64_t *latency_samples_us;
    size_t latency_sample_count;
    uint32_t successful_submits;
    worker_error_t error;
    uint32_t error_round;
    int error_errno;
    uint32_t error_task_counter;
    uint32_t error_irq_status;
    uint32_t error_row;
    uint32_t error_col;
    float error_expected;
    float error_actual;
} worker_context_t;

typedef struct {
    uint32_t threads;
    uint32_t rounds;
    uint32_t warmup_rounds;
    uint32_t npu_cores;
    uint32_t core_mask;
} test_options_t;

static uint64_t now_us(void) {
    struct timespec ts;

    clock_gettime(CLOCK_MONOTONIC, &ts);
    return (uint64_t)ts.tv_sec * 1000000ULL + (uint64_t)ts.tv_nsec / 1000ULL;
}

static int allocate_dma_buffer(
    int fd,
    size_t size,
    uint32_t flags,
    uint32_t core_mask,
    dma_buffer_t *buffer
) {
    struct rknpu_mem_create_starry create;
    struct rknpu_mem_map map;

    memset(buffer, 0, sizeof(*buffer));
    memset(&create, 0, sizeof(create));
    memset(&map, 0, sizeof(map));

    buffer->size = size;
    create.flags = flags | RKNPU_MEM_NON_CACHEABLE;
    create.size = size;
    create.core_mask = core_mask;

    if (ioctl(fd, DRM_IOCTL_RKNPU_MEM_CREATE_STARRY, &create) < 0) {
        fprintf(stderr, "RKNPU_MEM_CREATE failed: errno=%d (%s)\n",
                errno, strerror(errno));
        return -1;
    }

    buffer->allocated = 1;
    buffer->handle = create.handle;
    buffer->obj_addr = create.obj_addr;
    buffer->dma_addr = create.dma_addr;

    map.handle = create.handle;
    if (ioctl(fd, DRM_IOCTL_RKNPU_MEM_MAP, &map) < 0) {
        fprintf(stderr, "RKNPU_MEM_MAP failed: errno=%d (%s)\n",
                errno, strerror(errno));
        return -1;
    }

    buffer->map = mmap(NULL, size, PROT_READ | PROT_WRITE, MAP_SHARED, fd, map.offset);
    if (buffer->map == MAP_FAILED) {
        fprintf(stderr, "mmap failed: errno=%d (%s)\n", errno, strerror(errno));
        return -1;
    }

    return 0;
}

static void release_dma_buffer(int fd, dma_buffer_t *buffer) {
    if (!buffer->allocated) {
        return;
    }

    if (buffer->map != NULL && buffer->map != MAP_FAILED) {
        munmap(buffer->map, buffer->size);
    }
    mem_destroy(fd, buffer->handle, buffer->obj_addr);
    memset(buffer, 0, sizeof(*buffer));
}

static int input_value(uint32_t thread_id, uint32_t row, uint32_t channel) {
    return ((int)(thread_id * 13U + row * 7U + channel * 5U) % 11) - 5;
}

static int weight_value(uint32_t thread_id, uint32_t column, uint32_t channel) {
    return ((int)(thread_id * 17U + column * 3U + channel * 9U) % 11) - 5;
}

static void prepare_operands(worker_context_t *worker) {
    _Float16 *input = worker->resources.input.map;
    _Float16 *weights = worker->resources.weights.map;
    uint32_t matrix_id = worker->thread_id + 1U;

    memset(input, 0, worker->resources.input.size);
    memset(weights, 0, worker->resources.weights.size);

    for (uint32_t row = 0; row < TEST_M; row++) {
        for (uint32_t channel = 0; channel < TEST_K; channel++) {
            int offset = feature_data(
                TEST_K,
                TEST_M,
                1,
                INPUT_LAYOUT_C2,
                channel + 1U,
                row + 1U,
                1
            );
            input[offset] = (_Float16)input_value(matrix_id, row, channel);
        }
    }

    for (uint32_t column = 0; column < TEST_N; column++) {
        for (uint32_t channel = 0; channel < TEST_K; channel++) {
            int offset = weight_fp16(TEST_K, column + 1U, channel + 1U);
            weights[offset] = (_Float16)weight_value(matrix_id, column, channel);
        }
    }
}

static int prepare_task(worker_context_t *worker) {
    uint64_t regcmd_words[REGCMD_WORDS];
    struct rknpu_task *task = worker->resources.task.map;
    matmul_params_t params;

    memset(regcmd_words, 0, sizeof(regcmd_words));
    memset(task, 0, sizeof(*task));

    params.m = TEST_M;
    params.k = TEST_K;
    params.n = TEST_N;
    params.input_dma = (uint32_t)worker->resources.input.dma_addr;
    params.weights_dma = (uint32_t)worker->resources.weights.dma_addr;
    params.output_dma = (uint32_t)worker->resources.output.dma_addr;
    params.tasks = regcmd_words;
    params.fp32tofp16 = 0;

    if (gen_matmul_fp16(&params) != 0) {
        return -1;
    }

    memcpy(worker->resources.regcmd.map, regcmd_words, sizeof(regcmd_words));

    task->flags = 0;
    task->op_idx = worker->thread_id;
    task->enable_mask = 0xd;
    task->int_mask = 0x300;
    task->int_clear = 0x1ffff;
    task->int_status = 0;
    task->regcfg_amount = REGCMD_WORDS - (RKNPU_PC_DATA_EXTRA_AMOUNT + 4U);
    task->regcfg_offset = 0;
    task->regcmd_addr = worker->resources.regcmd.dma_addr;

    memset(&worker->resources.submit_template, 0, sizeof(worker->resources.submit_template));
    worker->resources.submit_template.flags =
        RKNPU_JOB_PC | RKNPU_JOB_BLOCK | RKNPU_JOB_PINGPONG;
    worker->resources.submit_template.timeout = SUBMIT_TIMEOUT_MS;
    worker->resources.submit_template.task_number = 1;
    worker->resources.submit_template.task_obj_addr = worker->resources.task.obj_addr;
    worker->resources.submit_template.core_mask = worker->core_mask;
    worker->resources.submit_template.fence_fd = -1;

    /* 单逻辑通道允许多个独立 Submit 在所选物理核心之间竞争。 */
    worker->resources.submit_template.subcore_task[0].task_start = 0;
    worker->resources.submit_template.subcore_task[0].task_number = 1;
    return 0;
}

static void release_worker_resources(worker_context_t *worker) {
    release_dma_buffer(worker->fd, &worker->resources.output);
    release_dma_buffer(worker->fd, &worker->resources.weights);
    release_dma_buffer(worker->fd, &worker->resources.input);
    release_dma_buffer(worker->fd, &worker->resources.task);
    release_dma_buffer(worker->fd, &worker->resources.regcmd);
}

static int prepare_worker_resources(worker_context_t *worker) {
    if (allocate_dma_buffer(
            worker->fd,
            REGCMD_BYTES,
            0,
            worker->core_mask,
            &worker->resources.regcmd
        ) != 0 ||
        allocate_dma_buffer(
            worker->fd,
            sizeof(struct rknpu_task),
            RKNPU_MEM_KERNEL_MAPPING,
            worker->core_mask,
            &worker->resources.task
        ) != 0 ||
        allocate_dma_buffer(
            worker->fd,
            TEST_M * TEST_K * sizeof(_Float16),
            0,
            worker->core_mask,
            &worker->resources.input
        ) != 0 ||
        allocate_dma_buffer(
            worker->fd,
            TEST_N * TEST_K * sizeof(_Float16),
            0,
            worker->core_mask,
            &worker->resources.weights
        ) != 0 ||
        allocate_dma_buffer(
            worker->fd,
            TEST_M * TEST_N * sizeof(float),
            0,
            worker->core_mask,
            &worker->resources.output
        ) != 0) {
        release_worker_resources(worker);
        return -1;
    }

    if (worker->resources.input.dma_addr > UINT32_MAX ||
        worker->resources.weights.dma_addr > UINT32_MAX ||
        worker->resources.output.dma_addr > UINT32_MAX) {
        fprintf(stderr, "matmul data DMA address exceeds the 32-bit generator interface\n");
        release_worker_resources(worker);
        return -1;
    }

    prepare_operands(worker);
    memset(worker->resources.output.map, 0, worker->resources.output.size);

    if (prepare_task(worker) != 0) {
        release_worker_resources(worker);
        return -1;
    }
    return 0;
}

static int round_gate_init(round_gate_t *gate, uint32_t expected_threads) {
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

static void round_gate_stop(round_gate_t *gate) {
    pthread_mutex_lock(&gate->mutex);
    gate->stop = 1;
    pthread_cond_broadcast(&gate->cond);
    pthread_mutex_unlock(&gate->mutex);
}

static void round_gate_destroy(round_gate_t *gate) {
    pthread_cond_destroy(&gate->cond);
    pthread_mutex_destroy(&gate->mutex);
}

static int worker_wait_for_round(round_gate_t *gate) {
    uint32_t generation;
    int should_run;

    pthread_mutex_lock(&gate->mutex);
    generation = gate->generation;
    gate->ready_threads++;
    pthread_cond_broadcast(&gate->cond);

    while (!gate->stop && generation == gate->generation) {
        pthread_cond_wait(&gate->cond, &gate->mutex);
    }
    should_run = !gate->stop;
    pthread_mutex_unlock(&gate->mutex);
    return should_run;
}

static void worker_finish_round(round_gate_t *gate) {
    pthread_mutex_lock(&gate->mutex);
    gate->done_threads++;
    pthread_cond_broadcast(&gate->cond);
    pthread_mutex_unlock(&gate->mutex);
}

/* 最后修改：2026-08-04
 * 吞吐量只统计每轮 blocking submit 的实际活跃时间。
 */
static void worker_record_submit_window(
    round_gate_t *gate,
    uint64_t start_us,
    uint64_t end_us
) {
    pthread_mutex_lock(&gate->mutex);
    if (gate->completed_submits == 0U || start_us < gate->first_submit_start_us) {
        gate->first_submit_start_us = start_us;
    }
    if (gate->completed_submits == 0U || end_us > gate->last_submit_end_us) {
        gate->last_submit_end_us = end_us;
    }
    gate->completed_submits++;
    pthread_mutex_unlock(&gate->mutex);
}

static int run_synchronized_round(
    round_gate_t *gate,
    uint64_t *submit_window_us,
    uint32_t *completed_submits
) {
    int stopped;

    pthread_mutex_lock(&gate->mutex);
    while (!gate->stop && gate->ready_threads < gate->expected_threads) {
        pthread_cond_wait(&gate->cond, &gate->mutex);
    }
    if (gate->stop) {
        pthread_mutex_unlock(&gate->mutex);
        return -1;
    }

    gate->ready_threads = 0;
    gate->done_threads = 0;
    gate->first_submit_start_us = 0;
    gate->last_submit_end_us = 0;
    gate->completed_submits = 0;
    gate->generation++;
    pthread_cond_broadcast(&gate->cond);

    while (!gate->stop && gate->done_threads < gate->expected_threads) {
        pthread_cond_wait(&gate->cond, &gate->mutex);
    }
    *completed_submits = gate->completed_submits;
    if (gate->completed_submits > 0U &&
        gate->last_submit_end_us >= gate->first_submit_start_us) {
        *submit_window_us = gate->last_submit_end_us - gate->first_submit_start_us;
    } else {
        *submit_window_us = 0;
    }
    stopped = gate->stop;
    pthread_mutex_unlock(&gate->mutex);
    return stopped ? -1 : 0;
}

static float expected_output(
    uint32_t matrix_id,
    uint32_t row,
    uint32_t column
) {
    float sum = 0.0f;

    for (uint32_t channel = 0; channel < TEST_K; channel++) {
        sum += (float)input_value(matrix_id, row, channel) *
            (float)weight_value(matrix_id, column, channel);
    }
    return sum;
}

static int verify_output(worker_context_t *worker) {
    static const uint32_t rows[] = {0U, 3U};
    static const uint32_t columns[] = {0U, 3U};
    const float *output = worker->resources.output.map;
    uint32_t matrix_id = worker->thread_id + 1U;

    for (size_t row_index = 0; row_index < sizeof(rows) / sizeof(rows[0]); row_index++) {
        for (size_t column_index = 0;
             column_index < sizeof(columns) / sizeof(columns[0]);
             column_index++) {
            uint32_t row = rows[row_index];
            uint32_t column = columns[column_index];
            int offset = feature_data(
                TEST_N,
                TEST_M,
                1,
                OUTPUT_LAYOUT_C2,
                column + 1U,
                row + 1U,
                1
            );
            float expected = expected_output(matrix_id, row, column);
            float actual = output[offset];

            if (fabsf(actual - expected) > 0.001f) {
                worker->error_row = row;
                worker->error_col = column;
                worker->error_expected = expected;
                worker->error_actual = actual;
                return -1;
            }
        }
    }
    return 0;
}

static int run_worker_submit(worker_context_t *worker, uint32_t round) {
    struct rknpu_task *task = worker->resources.task.map;
    struct rknpu_submit submit = worker->resources.submit_template;
    uint64_t start_us;
    uint64_t end_us;

    memset(worker->resources.output.map, 0, worker->resources.output.size);
    task->int_status = 0;

    start_us = now_us();
    if (ioctl(worker->fd, DRM_IOCTL_RKNPU_SUBMIT, &submit) < 0) {
        worker->error = WORKER_IOCTL_FAILED;
        worker->error_errno = errno;
        worker->error_round = round;
        return -1;
    }
    end_us = now_us();

    if (submit.task_counter != 1U) {
        worker->error = WORKER_TASK_COUNTER_FAILED;
        worker->error_task_counter = submit.task_counter;
        worker->error_round = round;
        return -1;
    }
    if (task->int_status != 0x300U) {
        worker->error = WORKER_IRQ_STATUS_FAILED;
        worker->error_irq_status = task->int_status;
        worker->error_round = round;
        return -1;
    }
    if (verify_output(worker) != 0) {
        worker->error = WORKER_OUTPUT_FAILED;
        worker->error_round = round;
        return -1;
    }

    worker->successful_submits++;
    worker_record_submit_window(worker->gate, start_us, end_us);
    if (round >= worker->warmup_rounds) {
        worker->latency_samples_us[worker->latency_sample_count++] = end_us - start_us;
    }
    return 0;
}

static void *worker_main(void *arg) {
    worker_context_t *worker = arg;
    uint32_t total_rounds = worker->warmup_rounds + worker->measure_rounds;

    for (uint32_t round = 0; round < total_rounds; round++) {
        if (!worker_wait_for_round(worker->gate)) {
            break;
        }

        if (worker->error == WORKER_OK) {
            run_worker_submit(worker, round);
        }

        /* A failed worker stays in the round protocol so peers cannot deadlock. */
        worker_finish_round(worker->gate);
    }
    return NULL;
}

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
            "  first failure: round=%u task_counter=%u expected=1\n",
            worker->error_round + 1U,
            worker->error_task_counter
        );
    } else if (worker->error == WORKER_IRQ_STATUS_FAILED) {
        printf(
            "  first failure: round=%u int_status=0x%x expected=0x300\n",
            worker->error_round + 1U,
            worker->error_irq_status
        );
    } else if (worker->error == WORKER_OUTPUT_FAILED) {
        printf(
            "  first failure: round=%u row=%u col=%u expected=%f actual=%f\n",
            worker->error_round + 1U,
            worker->error_row,
            worker->error_col,
            worker->error_expected,
            worker->error_actual
        );
    }
}

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

static void print_usage(const char *program) {
    printf("Usage: %s [options]\n", program);
    printf("Options:\n");
    printf("  --threads <count>  submit threads, default: %u\n", DEFAULT_THREADS);
    printf("  --rounds <count>   measured rounds, default: %u\n", DEFAULT_ROUNDS);
    printf("  --warmup <count>   warmup rounds, default: %u\n", DEFAULT_WARMUP_ROUNDS);
    printf("  --cores <1|2|3>    NPU core count, default: %u\n", DEFAULT_NPU_CORES);
    printf("  --help             show this message\n");
}

static int parse_options(int argc, char **argv, test_options_t *options) {
    options->threads = DEFAULT_THREADS;
    options->rounds = DEFAULT_ROUNDS;
    options->warmup_rounds = DEFAULT_WARMUP_ROUNDS;
    options->npu_cores = DEFAULT_NPU_CORES;
    options->core_mask = (1U << DEFAULT_NPU_CORES) - 1U;

    for (int index = 1; index < argc; index++) {
        const char *arg = argv[index];

        if (strcmp(arg, "--help") == 0) {
            print_usage(argv[0]);
            return 1;
        }
        if (strcmp(arg, "--threads") != 0 &&
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

        if (strcmp(arg, "--threads") == 0) {
            if (parse_u32(argv[index], &options->threads) != 0 ||
                options->threads == 0U || options->threads > MAX_TEST_THREADS) {
                fprintf(stderr, "invalid --threads value: %s (valid: 1-%u)\n",
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

    /* 核心数量映射为从 core0 开始的连续物理核心掩码。 */
    options->core_mask = (1U << options->npu_cores) - 1U;

    if (UINT32_MAX - options->warmup_rounds < options->rounds) {
        fprintf(stderr, "warmup and measured round count overflow\n");
        return -1;
    }
    return 0;
}

int main(int argc, char **argv) {
    /* ------------------------ 变量声明与初始化 ------------------------ */
    test_options_t options;              // 存储解析后的命令行参数
    worker_context_t *workers = NULL;    // 工作线程上下文数组指针，存储每个线程的私有数据
    pthread_t *threads = NULL;           // POSIX 线程 ID 数组指针
    round_gate_t gate;                   // 同步屏障结构体，用于协调所有线程同时开始和结束
    uint32_t prepared_workers = 0;       // 计数器：记录成功完成资源准备的线程数
    uint32_t created_threads = 0;        // 计数器：记录成功创建的线程数
    uint32_t total_rounds;               // 总轮数 = 预热轮数 + 测量轮数
    uint64_t measured_submit_us = 0;     // measured 轮次中实际 submit 活跃窗口的累计时间
    uint64_t *all_samples = NULL;        // 指针：用于汇总所有线程的延迟数据
    size_t all_sample_count = 0;         // 汇总的样本总数
    size_t sample_offset = 0;            // 数据拷贝时的偏移量
    size_t measured_submit_count = 0;    // 吞吐量窗口内成功完成的 measured submit 数量
    int fd = -1;                         // NPU 设备文件描述符，初始化为 -1
    int gate_initialized = 0;            // 标志位：同步屏障是否初始化成功
    int result = 1;                      // 程序退出码，默认为 1 (失败)，只有全部通过才置 0
    int join_failed = 0;                 // 标志位：线程 join 是否失败
    int parse_result;

    /* ------------------------ 1. 参数解析 ------------------------ */
    parse_result = parse_options(argc, argv, &options);
    if (parse_result != 0) {
        // parse_result > 0 表示是 --help 正常退出，< 0 表示参数错误
        return parse_result > 0 ? 0 : 2;
    }
    total_rounds = options.warmup_rounds + options.rounds;

    /* ------------------------ 2. 设备初始化 ------------------------ */
    // 打开 NPU 设备节点 (如 /dev/npu)
    fd = npu_open();
    if (fd < 0) {
        return 1;
    }
    // 复位 NPU 硬件状态，确保测试环境干净
    if (npu_reset(fd) < 0) {
        fprintf(stderr, "RKNPU reset failed: errno=%d (%s)\n", errno, strerror(errno));
        goto cleanup; // 失败则跳转到清理流程
    }

    /* ------------------------ 3. 内存分配 ------------------------ */
    // 分配工作线程上下文数组
    workers = calloc(options.threads, sizeof(*workers));
    // 分配线程 ID 数组
    threads = calloc(options.threads, sizeof(*threads));
    if (workers == NULL || threads == NULL) {
        fprintf(stderr, "thread context allocation failed\n");
        goto cleanup;
    }

    /* ------------------------ 4. 为每个线程准备资源 ------------------------ */
    // 关键点：这里分配的是 NPU 专用的 DMA 缓冲区，而非普通堆内存
    for (uint32_t index = 0; index < options.threads; index++) {
        workers[index].thread_id = index;
        workers[index].fd = fd; // 所有线程共享同一个设备 FD
        workers[index].core_mask = options.core_mask;
        workers[index].warmup_rounds = options.warmup_rounds;
        workers[index].measure_rounds = options.rounds;

        // 为每个线程分配用于存储延迟数据的数组
        workers[index].latency_samples_us = calloc(
            options.rounds,
            sizeof(*workers[index].latency_samples_us)
        );
        
        // 准备 NPU 任务资源：
        // 1. 分配输入、权重、输出、指令 DMA 缓冲区
        // 2. 填充测试数据
        // 3. 构建 NPU 任务指令
        if (workers[index].latency_samples_us == NULL ||
            prepare_worker_resources(&workers[index]) != 0) {
            fprintf(stderr, "worker[%u] resource preparation failed\n", index);
            goto cleanup;
        }
        prepared_workers++; // 成功准备的线程数加 1，用于后续精准清理
    }

    /* ------------------------ 5. 初始化同步屏障 ------------------------ */
    // 初始化多线程同步机制，确保所有线程在同一时刻提交任务，产生最大并发压力
    if (round_gate_init(&gate, options.threads) != 0) {
        fprintf(stderr, "round synchronization initialization failed\n");
        goto cleanup;
    }
    gate_initialized = 1;

    /* ------------------------ 6. 打印测试配置 ------------------------ */
    printf("rknpu_multithread_submit_core\n");
    printf("  shared fd       : yes\n");
    printf("  threads         : %u\n", options.threads);
    printf("  workload        : M=%u K=%u N=%u, one task per submit\n",
           TEST_M, TEST_K, TEST_N);
    printf("  NPU cores       : %u\n", options.npu_cores);
    printf("  core mask       : 0x%x\n", options.core_mask);
    printf("  rounds          : %u warmup + %u measured\n",
           options.warmup_rounds, options.rounds);

    /* ------------------------ 7. 创建工作线程 ------------------------ */
    for (uint32_t index = 0; index < options.threads; index++) {
        workers[index].gate = &gate; // 将同步屏障指针传递给线程
        int thread_error = pthread_create(&threads[index], NULL, worker_main, &workers[index]);
        if (thread_error != 0) {
            fprintf(stderr, "pthread_create failed for worker[%u]: %s\n",
                    index, strerror(thread_error));
            round_gate_stop(&gate); // 创建失败时，必须通知同步屏障停止，防止已创建线程死锁
            goto join_threads;
        }
        created_threads++;
    }

    /* ------------------------ 8. 执行基准测试主循环 ------------------------ */
    // 主线程充当“指挥官”，控制每一轮测试的开始和结束
    for (uint32_t round = 0; round < total_rounds; round++) {
        uint64_t round_submit_us = 0;
        uint32_t round_submit_count = 0;

        // 运行一轮同步测试：
        // 1. 等待所有工作线程就绪
        // 2. 广播唤醒所有线程并等待本轮结束
        // 3. 汇总最早 ioctl 进入到最晚 ioctl 返回的 submit 活跃窗口
        if (run_synchronized_round(
                &gate,
                &round_submit_us,
                &round_submit_count
            ) != 0) {
            fprintf(stderr, "round synchronization stopped at round=%u\n", round + 1U);
            goto join_threads;
        }
        // 只有预热轮之后的轮次才计入统计数据
        if (round >= options.warmup_rounds) {
            measured_submit_us += round_submit_us;
            measured_submit_count += round_submit_count;
        }
    }

/* ------------------------ 9. 线程汇合 ------------------------ */
join_threads:
    // 等待所有工作线程退出
    for (uint32_t index = 0; index < created_threads; index++) {
        int thread_error = pthread_join(threads[index], NULL);
        if (thread_error != 0) {
            fprintf(stderr, "pthread_join failed for worker[%u]: %s\n",
                    index, strerror(thread_error));
            join_failed = 1;
        }
    }
    
    // 如果 join 失败，说明线程资源未完全回收，直接返回避免非法内存访问
    if (join_failed) {
        return 1;
    }
    // 如果创建的线程数少于预期，说明中途出错，跳转到清理流程
    if (created_threads != options.threads) {
        goto cleanup;
    }

    /* ------------------------ 10. 收集与打印结果 ------------------------ */
    // 收集各线程的统计数据
    for (uint32_t index = 0; index < options.threads; index++) {
        print_worker_result(&workers[index], total_rounds);
        all_sample_count += workers[index].latency_sample_count;
    }

    // 如果有样本数据，进行整体统计计算
    if (all_sample_count > 0U) {
        benchmark_latency_stats_t stats;

        // 分配全局数组以存放所有样本
        all_samples = malloc(all_sample_count * sizeof(*all_samples));
        if (all_samples == NULL) {
            fprintf(stderr, "latency sample allocation failed\n");
            goto cleanup;
        }
        
        // 将各线程的样本数据拷贝到连续的全局数组中
        for (uint32_t index = 0; index < options.threads; index++) {
            memcpy(
                &all_samples[sample_offset],
                workers[index].latency_samples_us,
                workers[index].latency_sample_count * sizeof(*all_samples)
            );
            sample_offset += workers[index].latency_sample_count;
        }

        // 计算统计数据 (均值、分位数等)
        if (benchmark_compute_latency_stats(all_samples, all_sample_count, &stats) != 0) {
            fprintf(stderr, "latency statistics failed\n");
            goto cleanup;
        }

        printf("aggregate measured submits: %zu\n", all_sample_count);
        printf("  mean : %.3f ms\n", stats.mean_us / 1000.0);
        printf("  P50  : %.3f ms\n", (double)stats.p50_us / 1000.0);
        printf("  P95  : %.3f ms\n", (double)stats.p95_us / 1000.0);
        printf("  P99  : %.3f ms\n", (double)stats.p99_us / 1000.0);
    }

    // 使用成功 submit 数量除以并发 ioctl 的实际活跃时间。
    if (measured_submit_us > 0U) {
        double throughput = (double)measured_submit_count * 1000000.0 /
            (double)measured_submit_us;
        printf("  throughput: %.2f submits/s\n", throughput);
    }

    /* ------------------------ 11. 结果判定 ------------------------ */
    result = 0;
    // 检查每个线程是否全部成功：无错误、提交数正确、采样数正确
    for (uint32_t index = 0; index < options.threads; index++) {
        if (workers[index].error != WORKER_OK ||
            workers[index].successful_submits != total_rounds ||
            workers[index].latency_sample_count != options.rounds) {
            result = 1;
        }
    }
    printf("rknpu_multithread_submit_core: %s\n", result == 0 ? "PASS" : "FAIL");

    /* ------------------------ 12. 资源清理 ------------------------ */
cleanup:
    // 销毁同步屏障
    if (gate_initialized) {
        round_gate_destroy(&gate);
    }
    
    free(all_samples);
    
    // 释放每个工作线程申请的 DMA 缓冲区和内存
    for (uint32_t index = 0; index < prepared_workers; index++) {
        release_worker_resources(&workers[index]);
        free(workers[index].latency_samples_us);
    }
    
    // 处理资源准备阶段中断的情况：第 prepared_workers 个线程可能已分配了 latency 数组但未计入 prepared_workers
    if (workers != NULL && prepared_workers < options.threads) {
        free(workers[prepared_workers].latency_samples_us);
    }

    free(threads);
    free(workers);
    
    // 关闭 NPU 设备
    if (fd >= 0) {
        npu_close(fd);
    }
    return result;
}
