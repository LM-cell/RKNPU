#include <errno.h>
#include <math.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/ioctl.h>
#include <sys/mman.h>
#include <sys/time.h>
#include <unistd.h>

#include <libdrm/drm.h>

#include "benchmark_stats.h"
#include "npu_interface.h"
#include "npu_matmul.h"
#include "rknpu-ioctl.h"

#define REGCMD_WORDS 112U
#define REGCMD_BYTES (REGCMD_WORDS * sizeof(uint64_t))
#define INPUT_LAYOUT_C2 8
#define OUTPUT_LAYOUT_C2 4
#define DMA_SLICE_ALIGN 64U
#define DEFAULT_TIMEOUT_MS 6000U
#define DEFAULT_MEASURE_ROUNDS 100U
#define YIELD_GAP_HISTOGRAM_BUCKET_NS 100000ULL
#define YIELD_GAP_HISTOGRAM_BUCKET_COUNT 1000U
#define STARRYOS_USER_PAGE_BYTES 4096U

_Static_assert(sizeof(struct rknpu_submit_trace_record) == 48,
               "submit trace ABI size mismatch");
_Static_assert(sizeof(struct rknpu_worker_yield_trace_record) == 40,
               "worker yield trace record ABI size mismatch");
_Static_assert(sizeof(struct rknpu_worker_yield_trace_query) == 32,
               "worker yield trace query ABI size mismatch");

typedef enum {
    OPERANDS_SHARED = 0,
    OPERANDS_UNIQUE = 1,
} operand_mode_t;

typedef struct {
    const char *name;
    const char *description;
    uint32_t m;
    uint32_t k;
    uint32_t n;
    uint32_t shared_tasks;
    uint32_t unique_tasks;
    uint32_t warmup_rounds;
    uint32_t measure_rounds;
} benchmark_scenario_t;

typedef struct {
    const benchmark_scenario_t *scenario;
    operand_mode_t operand_mode;
    uint32_t core_count;
    uint32_t task_count;
    uint32_t warmup_rounds;
    uint32_t measure_rounds;

    size_t per_task_input_bytes;
    size_t per_task_weight_bytes;
    size_t per_task_output_bytes;
    size_t input_stride;
    size_t weight_stride;
    size_t output_stride;
    uint32_t input_slots;
    uint32_t weight_slots;

    size_t regcmd_bytes;
    size_t tasks_bytes;
    size_t input_bytes;
    size_t weight_bytes;
    size_t output_bytes;
    size_t total_dma_bytes;

    void *regcmd;
    uint64_t regcmd_dma;
    uint64_t regcmd_obj;
    uint32_t regcmd_handle;

    struct rknpu_task *tasks;
    uint64_t tasks_dma;
    uint64_t tasks_obj;
    uint32_t tasks_handle;

    _Float16 *input;
    uint64_t input_dma;
    uint64_t input_obj;
    uint32_t input_handle;

    _Float16 *weights;
    uint64_t weights_dma;
    uint64_t weights_obj;
    uint32_t weights_handle;

    float *output;
    uint64_t output_dma;
    uint64_t output_obj;
    uint32_t output_handle;
} batch_resources_t;

//保存一次NPU任务提交所需的全部资源。

typedef struct {
    int valid;
    const benchmark_scenario_t *scenario;
    operand_mode_t operand_mode;
    uint32_t core_count;
    uint32_t task_count;
    uint32_t warmup_rounds;
    uint32_t measure_rounds;

    size_t per_task_input_bytes;
    size_t per_task_weight_bytes;
    size_t per_task_output_bytes;
    size_t total_dma_bytes;

    uint64_t operand_prepare_us;
    uint64_t descriptor_build_us;
    uint64_t min_submit_us;
    uint64_t max_submit_us;
    uint64_t total_submit_us;
    uint64_t *submit_samples_us;
    benchmark_latency_stats_t latency_stats;

    /*
     * Experiment-one trace data. These buffers are populated only for the
     * explicitly requested three-core llama_decode_like measured window.
     * Worker timestamps stay in memory until every measured ioctl has ended.
     */
    struct rknpu_submit_trace_record *submit_trace_records;
    uint32_t submit_trace_count;
    struct rknpu_worker_yield_trace_record *yield_trace_records;
    uint32_t yield_trace_count;
    int yield_trace_collected;

    double avg_submit_us;
    double avg_submit_ms;
    double avg_task_us;
    double tasks_per_sec;
    double gmac_per_sec;
    double gflops_per_sec;
    double jitter_pct;
} benchmark_result_t;

typedef struct {
    const char *scenario_filter;
    uint32_t rounds_override;
    uint32_t warmup_override;
    int has_rounds_override;
    int has_warmup_override;
    uint32_t task_cap;
    int run_shared;
    int run_unique;
    int run_one_core;
    int run_three_core;
    int collect_yield_trace;
    uint32_t yield_trace_capacity;
    int has_yield_trace_capacity;
} cli_options_t;

typedef struct {
    uint32_t yield_count;
    uint32_t stalled_yield_count;
    uint64_t sum_gap_ns;
    uint64_t max_gap_ns;
    uint64_t last_gap_ns;
    uint64_t last_yield_end_ns;
} yield_round_summary_t;

//四种测试场景
static const benchmark_scenario_t k_scenarios[] = {
    {
        .name = "tiny_dispatch",
        .description =
            "Small matrices where submit/scheduler overhead dominates and core-scaling "
            "efficiency is usually lower than the ideal 3x.",
        .m = 4,
        .k = 32,
        .n = 16,
        .shared_tasks = 48,
        .unique_tasks = 96,
        .warmup_rounds = 2,
        .measure_rounds = DEFAULT_MEASURE_ROUNDS,
    },
    {
        .name = "mid_balanced",
        .description =
            "Balanced mid-size matrices where both driver overhead and arithmetic "
            "throughput contribute to the final timing.",
        .m = 64,
        .k = 512,
        .n = 512,
        .shared_tasks = 48,
        .unique_tasks = 12,
        .warmup_rounds = 2,
        .measure_rounds = DEFAULT_MEASURE_ROUNDS,
    },
    {
        .name = "throughput_heavy",
        .description =
            "Large matrices intended to shift the bottleneck toward math throughput. "
            "Unique mode uses fewer tasks to keep DMA footprint bounded.",
        .m = 128,
        .k = 1024,
        .n = 1024,
        .shared_tasks = 48,
        .unique_tasks = 4,
        .warmup_rounds = 2,
        .measure_rounds = DEFAULT_MEASURE_ROUNDS,
    },
    {
        .name = "llama_decode_like",
        .description =
            "Low-M, high-K, high-N projection shape similar to decode-time LLM linear "
            "layers. This highlights latency sensitivity more than raw throughput.",
        .m = 1,
        .k = 4096,
        .n = 4096,
        .shared_tasks = 48,
        .unique_tasks = 0,
        .warmup_rounds = 2,
        .measure_rounds = DEFAULT_MEASURE_ROUNDS,
    },
};

//
static uint64_t now_us(void) {
    struct timeval tv;
    gettimeofday(&tv, NULL);
    return (uint64_t)tv.tv_sec * 1000000ULL + (uint64_t)tv.tv_usec;
}

/*
 * Materialize every userspace page before the kernel writes a large trace.
 *
 * StarryOS user_copy() reports the remaining byte count when it encounters a
 * page fault; it does not resolve a demand-zero page on behalf of userspace.
 * A large calloc allocation can therefore have a valid virtual range whose
 * pages have not been faulted in yet. Volatile writes guarantee that the
 * compiler cannot remove these page touches as redundant zero stores.
 *
 * This runs before trace enablement and outside the measured submit window.
 */
static void prefault_writable_user_buffer(void *buffer, size_t size) {
    volatile unsigned char *bytes = (volatile unsigned char *)buffer;

    if (bytes == NULL || size == 0U) {
        return;
    }
    for (size_t offset = 0; offset < size; offset += STARRYOS_USER_PAGE_BYTES) {
        bytes[offset] = 0;
    }
    bytes[size - 1U] = 0;
}

/* Reset the existing per-submit t0..t4 trace immediately before measurement. */
static int reset_submit_trace(int fd) {
    struct rknpu_submit_trace_query query;

    memset(&query, 0, sizeof(query));
    query.operation = RKNPU_SUBMIT_TRACE_RESET;
    if (ioctl(fd, DRM_IOCTL_RKNPU_SUBMIT_TRACE, &query) < 0) {
        fprintf(stderr,
                "failed to reset submit trace: errno=%d (%s)\n",
                errno,
                strerror(errno));
        return -1;
    }
    return 0;
}

/*
 * Replace the Worker trace with a fresh enabled or disabled buffer. Kernel
 * allocation therefore occurs outside the measured submit loop.
 */
static int configure_worker_yield_trace(int fd, int enabled, uint32_t capacity) {
    struct rknpu_worker_yield_trace_query query;

    memset(&query, 0, sizeof(query));
    query.operation = RKNPU_WORKER_YIELD_TRACE_CONFIG_RESET;
    query.enabled = enabled ? 1U : 0U;
    query.capacity = enabled ? capacity : 0U;
    if (ioctl(fd, DRM_IOCTL_RKNPU_WORKER_YIELD_TRACE, &query) < 0) {
        fprintf(stderr,
                "failed to %s Worker yield trace: errno=%d (%s)\n",
                enabled ? "enable" : "disable",
                errno,
                strerror(errno));
        return -1;
    }
    if (enabled && query.capacity != capacity) {
        fprintf(stderr,
                "Worker yield trace capacity mismatch: requested=%u configured=%u\n",
                capacity,
                query.capacity);
        return -1;
    }
    return 0;
}

static int read_submit_trace(
    int fd,
    struct rknpu_submit_trace_record *records,
    uint32_t capacity,
    struct rknpu_submit_trace_query *query_out
) {
    struct rknpu_submit_trace_query query;

    memset(&query, 0, sizeof(query));
    query.operation = RKNPU_SUBMIT_TRACE_READ;
    query.capacity = capacity;
    query.records_address = (uint64_t)(uintptr_t)records;
    if (ioctl(fd, DRM_IOCTL_RKNPU_SUBMIT_TRACE, &query) < 0) {
        fprintf(stderr,
                "failed to read submit trace: errno=%d (%s)\n",
                errno,
                strerror(errno));
        return -1;
    }
    *query_out = query;
    return 0;
}

static int read_worker_yield_trace(
    int fd,
    struct rknpu_worker_yield_trace_record *records,
    uint32_t capacity,
    struct rknpu_worker_yield_trace_query *query_out
) {
    struct rknpu_worker_yield_trace_query query;

    memset(&query, 0, sizeof(query));
    query.operation = RKNPU_WORKER_YIELD_TRACE_READ;
    query.capacity = capacity;
    query.records_address = (uint64_t)(uintptr_t)records;
    if (ioctl(fd, DRM_IOCTL_RKNPU_WORKER_YIELD_TRACE, &query) < 0) {
        fprintf(stderr,
                "failed to read Worker yield trace: errno=%d (%s)\n",
                errno,
                strerror(errno));
        return -1;
    }
    *query_out = query;
    return 0;
}
//
static size_t align_up_size(size_t value, size_t align) {
    return (value + align - 1) / align * align;
}

static const char *operand_mode_name(operand_mode_t mode) {
    return mode == OPERANDS_SHARED ? "shared-operands" : "unique-operands";
}

static const char *operand_mode_summary(operand_mode_t mode) {
    return mode == OPERANDS_SHARED
               ? "All tasks reuse one input/weight pair; output slices stay private."
               : "Each task owns a private input/weight/output slice.";
}

static int deterministic_input_value(int matrix_id, uint32_t row, uint32_t channel) {
    return ((matrix_id * 13 + (int)row * 7 + (int)channel * 5) % 11) - 5;
}

static int deterministic_weight_value(int matrix_id, uint32_t kernel, uint32_t channel) {
    return ((matrix_id * 17 + (int)kernel * 3 + (int)channel * 9) % 11) - 5;
}

static float reference_output_value(
    const benchmark_scenario_t *scenario,
    int matrix_id,
    uint32_t row,
    uint32_t column
) {
    float sum = 0.0f;
    for (uint32_t channel = 0; channel < scenario->k; channel++) {
        float a = (float)deterministic_input_value(matrix_id, row, channel);
        float b = (float)deterministic_weight_value(matrix_id, column, channel);
        sum += a * b;
    }
    return sum;
}

static void pack_input_slice(
    _Float16 *dst,
    const benchmark_scenario_t *scenario,
    int matrix_id
) {
    memset(dst, 0, scenario->m * scenario->k * sizeof(_Float16));
    for (uint32_t row = 0; row < scenario->m; row++) {
        for (uint32_t channel = 0; channel < scenario->k; channel++) {
            int index = feature_data(
                (int)scenario->k,
                (int)scenario->m,
                1,
                INPUT_LAYOUT_C2,
                (int)channel + 1,
                (int)row + 1,
                1
            );
            dst[index] = (_Float16)deterministic_input_value(matrix_id, row, channel);
        }
    }
}

static void pack_weight_slice(
    _Float16 *dst,
    const benchmark_scenario_t *scenario,
    int matrix_id
) {
    memset(dst, 0, scenario->n * scenario->k * sizeof(_Float16));
    for (uint32_t kernel = 0; kernel < scenario->n; kernel++) {
        for (uint32_t channel = 0; channel < scenario->k; channel++) {
            int index = weight_fp16(
                (int)scenario->k,
                (int)kernel + 1,
                (int)channel + 1
            );
            dst[index] = (_Float16)deterministic_weight_value(matrix_id, kernel, channel);
        }
    }
}

static uint32_t tasks_for_mode(const benchmark_scenario_t *scenario, operand_mode_t mode) {
    return mode == OPERANDS_SHARED ? scenario->shared_tasks : scenario->unique_tasks;
}

static int matrix_id_for_task(operand_mode_t mode, uint32_t task_index) {
    return mode == OPERANDS_SHARED ? 0 : (int)task_index;
}

static _Float16 *input_slice_ptr(const batch_resources_t *resources, uint32_t slot_index) {
    return (_Float16 *)((uint8_t *)resources->input + (size_t)slot_index * resources->input_stride);
}

static _Float16 *weight_slice_ptr(const batch_resources_t *resources, uint32_t slot_index) {
    return (_Float16 *)((uint8_t *)resources->weights + (size_t)slot_index * resources->weight_stride);
}

static float *output_slice_ptr(const batch_resources_t *resources, uint32_t task_index) {
    return (float *)((uint8_t *)resources->output + (size_t)task_index * resources->output_stride);
}

static uint64_t input_slice_dma(const batch_resources_t *resources, uint32_t slot_index) {
    return resources->input_dma + (uint64_t)slot_index * resources->input_stride;
}

static uint64_t weight_slice_dma(const batch_resources_t *resources, uint32_t slot_index) {
    return resources->weights_dma + (uint64_t)slot_index * resources->weight_stride;
}

static uint64_t output_slice_dma(const batch_resources_t *resources, uint32_t task_index) {
    return resources->output_dma + (uint64_t)task_index * resources->output_stride;
}

static void free_batch_resources(int fd, batch_resources_t *resources) {
    if (resources->regcmd) {
        munmap(resources->regcmd, resources->regcmd_bytes);
        mem_destroy(fd, resources->regcmd_handle, resources->regcmd_obj);
    }
    if (resources->tasks) {
        munmap(resources->tasks, resources->tasks_bytes);
        mem_destroy(fd, resources->tasks_handle, resources->tasks_obj);
    }
    if (resources->input) {
        munmap(resources->input, resources->input_bytes);
        mem_destroy(fd, resources->input_handle, resources->input_obj);
    }
    if (resources->weights) {
        munmap(resources->weights, resources->weight_bytes);
        mem_destroy(fd, resources->weights_handle, resources->weights_obj);
    }
    if (resources->output) {
        munmap(resources->output, resources->output_bytes);
        mem_destroy(fd, resources->output_handle, resources->output_obj);
    }

    memset(resources, 0, sizeof(*resources));
}

static int allocate_batch_resources(
    int fd,
    const benchmark_scenario_t *scenario,
    operand_mode_t mode,
    uint32_t task_count,
    batch_resources_t *resources
) {
    memset(resources, 0, sizeof(*resources));

    resources->scenario = scenario;
    resources->operand_mode = mode;
    resources->task_count = task_count;

    resources->per_task_input_bytes = (size_t)scenario->m * scenario->k * sizeof(_Float16);
    resources->per_task_weight_bytes = (size_t)scenario->n * scenario->k * sizeof(_Float16);
    resources->per_task_output_bytes = (size_t)scenario->m * scenario->n * sizeof(float);

    resources->input_stride = align_up_size(resources->per_task_input_bytes, DMA_SLICE_ALIGN);
    resources->weight_stride = align_up_size(resources->per_task_weight_bytes, DMA_SLICE_ALIGN);
    resources->output_stride = align_up_size(resources->per_task_output_bytes, DMA_SLICE_ALIGN);

    resources->input_slots = mode == OPERANDS_SHARED ? 1U : task_count;
    resources->weight_slots = mode == OPERANDS_SHARED ? 1U : task_count;

    resources->regcmd_bytes = task_count * REGCMD_BYTES;
    resources->tasks_bytes = task_count * sizeof(struct rknpu_task);
    resources->input_bytes = (size_t)resources->input_slots * resources->input_stride;
    resources->weight_bytes = (size_t)resources->weight_slots * resources->weight_stride;
    resources->output_bytes = (size_t)task_count * resources->output_stride;
    resources->total_dma_bytes = resources->regcmd_bytes
        + resources->tasks_bytes
        + resources->input_bytes
        + resources->weight_bytes
        + resources->output_bytes;

    resources->regcmd = mem_allocate(
        fd,
        resources->regcmd_bytes,
        &resources->regcmd_dma,
        &resources->regcmd_obj,
        0,
        &resources->regcmd_handle
    );
    resources->tasks = mem_allocate(
        fd,
        resources->tasks_bytes,
        &resources->tasks_dma,
        &resources->tasks_obj,
        RKNPU_MEM_KERNEL_MAPPING,
        &resources->tasks_handle
    );
    resources->input = mem_allocate(
        fd,
        resources->input_bytes,
        &resources->input_dma,
        &resources->input_obj,
        0,
        &resources->input_handle
    );
    resources->weights = mem_allocate(
        fd,
        resources->weight_bytes,
        &resources->weights_dma,
        &resources->weights_obj,
        0,
        &resources->weights_handle
    );
    resources->output = mem_allocate(
        fd,
        resources->output_bytes,
        &resources->output_dma,
        &resources->output_obj,
        0,
        &resources->output_handle
    );

    if (!resources->regcmd || !resources->tasks || !resources->input ||
        !resources->weights || !resources->output) {
        fprintf(stderr,
                "allocation failed for scenario=%s mode=%s task_count=%u\n",
                scenario->name,
                operand_mode_name(mode),
                task_count);
        free_batch_resources(fd, resources);
        return -1;
    }

    return 0;
}

static uint64_t prepare_operand_buffers(batch_resources_t *resources) {
    uint64_t start_us = now_us();

    memset(resources->input, 0, resources->input_bytes);
    memset(resources->weights, 0, resources->weight_bytes);
    memset(resources->output, 0, resources->output_bytes);

    for (uint32_t slot = 0; slot < resources->input_slots; slot++) {
        int matrix_id = matrix_id_for_task(resources->operand_mode, slot);
        pack_input_slice(input_slice_ptr(resources, slot), resources->scenario, matrix_id);
    }

    for (uint32_t slot = 0; slot < resources->weight_slots; slot++) {
        int matrix_id = matrix_id_for_task(resources->operand_mode, slot);
        pack_weight_slice(weight_slice_ptr(resources, slot), resources->scenario, matrix_id);
    }

    return now_us() - start_us;
}

static int build_task_descriptors(batch_resources_t *resources) {
    uint64_t regcmd_words[REGCMD_WORDS];
    memset(resources->tasks, 0, resources->tasks_bytes);
    memset(resources->regcmd, 0, resources->regcmd_bytes);

    for (uint32_t task_index = 0; task_index < resources->task_count; task_index++) {
        uint32_t slot_index = resources->operand_mode == OPERANDS_SHARED ? 0U : task_index;
        matmul_params_t params;

        memset(regcmd_words, 0, sizeof(regcmd_words));
        params.m = (uint16_t)resources->scenario->m;
        params.k = (uint16_t)resources->scenario->k;
        params.n = (uint16_t)resources->scenario->n;
        params.input_dma = (uint32_t)input_slice_dma(resources, slot_index);
        params.weights_dma = (uint32_t)weight_slice_dma(resources, slot_index);
        params.output_dma = (uint32_t)output_slice_dma(resources, task_index);
        params.tasks = regcmd_words;
        params.fp32tofp16 = 0;

        if (gen_matmul_fp16(&params) != 0) {
            fprintf(stderr,
                    "gen_matmul_fp16 failed for scenario=%s task=%u\n",
                    resources->scenario->name,
                    task_index);
            return -1;
        }

        memcpy(
            (uint8_t *)resources->regcmd + (size_t)task_index * REGCMD_BYTES,
            regcmd_words,
            REGCMD_BYTES
        );

        resources->tasks[task_index].flags = 0;
        resources->tasks[task_index].op_idx = task_index;
        resources->tasks[task_index].enable_mask = 0xd;
        resources->tasks[task_index].int_mask = 0x300;
        resources->tasks[task_index].int_clear = 0x1ffff;
        resources->tasks[task_index].int_status = 0;
        resources->tasks[task_index].regcfg_amount =
            REGCMD_WORDS - (RKNPU_PC_DATA_EXTRA_AMOUNT + 4);
        resources->tasks[task_index].regcfg_offset = 0;
        resources->tasks[task_index].regcmd_addr =
            resources->regcmd_dma + (uint64_t)task_index * REGCMD_BYTES;
    }

    return 0;
}

static uint64_t build_task_descriptors_timed(batch_resources_t *resources, int *status_out) {
    uint64_t start_us = now_us();
    *status_out = build_task_descriptors(resources);
    return now_us() - start_us;
}

static void distribute_tasks_to_cores(
    struct rknpu_submit *submit,
    uint32_t task_count,
    uint32_t requested_core_count
) {
    uint32_t used_cores = requested_core_count < task_count ? requested_core_count : task_count;
    uint32_t next_start = 0;

    memset(submit->subcore_task, 0, sizeof(submit->subcore_task));
    submit->core_mask = 0;

    for (uint32_t core = 0; core < used_cores; core++) {
        uint32_t remaining_tasks = task_count - next_start;
        uint32_t remaining_cores = used_cores - core;
        uint32_t chunk = (remaining_tasks + remaining_cores - 1) / remaining_cores;

        submit->subcore_task[core].task_start = next_start;
        submit->subcore_task[core].task_number = chunk;
        submit->core_mask |= (1U << core);
        next_start += chunk;
    }
}

static void init_submit_template(
    const batch_resources_t *resources,
    uint32_t core_count,
    struct rknpu_submit *submit
) {
    memset(submit, 0, sizeof(*submit));
    submit->flags = RKNPU_JOB_PC | RKNPU_JOB_BLOCK | RKNPU_JOB_PINGPONG;
    submit->timeout = DEFAULT_TIMEOUT_MS;
    submit->task_start = 0;
    submit->task_number = resources->task_count;
    submit->task_counter = 0;
    submit->priority = 0;
    submit->task_obj_addr = resources->tasks_obj;
    submit->regcfg_obj_addr = 0;
    submit->task_base_addr = 0;
    submit->user_data = 0;
    submit->fence_fd = -1;

    /*
     * Keep the legacy zero task_base_addr contract. The current queue-driven
     * driver path fetches one shadow task at a time and still expects this
     * userspace field to remain zero for compatibility with the existing demos.
     */
    distribute_tasks_to_cores(submit, resources->task_count, core_count);
}

static void clear_outputs_and_task_status(batch_resources_t *resources) {
    memset(resources->output, 0, resources->output_bytes);
    for (uint32_t task_index = 0; task_index < resources->task_count; task_index++) {
        resources->tasks[task_index].int_status = 0;
    }
}

static int verify_completion_status(
    const batch_resources_t *resources,
    const struct rknpu_submit *submit
) {
    if (submit->task_counter != resources->task_count) {
        fprintf(stderr,
                "task_counter mismatch: expected=%u actual=%u\n",
                resources->task_count,
                submit->task_counter);
        return -1;
    }

    for (uint32_t task_index = 0; task_index < resources->task_count; task_index++) {
        if (resources->tasks[task_index].int_status != 0x300) {
            fprintf(stderr,
                    "task[%u] int_status mismatch: expected=0x300 actual=0x%x\n",
                    task_index,
                    resources->tasks[task_index].int_status);
            return -1;
        }
    }

    return 0;
}

static int verify_sampled_outputs(const batch_resources_t *resources) {
    uint32_t sample_tasks[3];
    uint32_t sample_rows[4];
    uint32_t sample_cols[4];
    uint32_t task_sample_count = 0;
    uint32_t row_sample_count = 0;
    uint32_t col_sample_count = 0;

    sample_tasks[task_sample_count++] = 0;
    if (resources->task_count > 2) {
        sample_tasks[task_sample_count++] = resources->task_count / 2;
    }
    if (resources->task_count > 1) {
        sample_tasks[task_sample_count++] = resources->task_count - 1;
    }

    sample_rows[row_sample_count++] = 0;
    if (resources->scenario->m > 1) {
        sample_rows[row_sample_count++] = resources->scenario->m > 3 ? 3 : resources->scenario->m - 1;
    }

    sample_cols[col_sample_count++] = 0;
    if (resources->scenario->n > 1) {
        sample_cols[col_sample_count++] = resources->scenario->n > 3 ? 3 : resources->scenario->n - 1;
    }

    for (uint32_t sample_task = 0; sample_task < task_sample_count; sample_task++) {
        uint32_t task_index = sample_tasks[sample_task];
        int matrix_id = matrix_id_for_task(resources->operand_mode, task_index);
        float *output = output_slice_ptr(resources, task_index);

        for (uint32_t row_index = 0; row_index < row_sample_count; row_index++) {
            for (uint32_t col_index = 0; col_index < col_sample_count; col_index++) {
                uint32_t row = sample_rows[row_index];
                uint32_t col = sample_cols[col_index];
                int output_offset = feature_data(
                    (int)resources->scenario->n,
                    (int)resources->scenario->m,
                    1,
                    OUTPUT_LAYOUT_C2,
                    (int)col + 1,
                    (int)row + 1,
                    1
                );
                float actual = output[output_offset];
                float expected = reference_output_value(resources->scenario, matrix_id, row, col);

                if (fabsf(actual - expected) > 0.001f) {
                    fprintf(stderr,
                            "sample mismatch: task=%u row=%u col=%u expected=%f actual=%f\n",
                            task_index,
                            row,
                            col,
                            expected,
                            actual);
                    return -1;
                }
            }
        }
    }

    return 0;
}

static int run_submit_round(
    int fd,
    batch_resources_t *resources,
    const struct rknpu_submit *submit_template,
    uint64_t *elapsed_us_out,
    int verify_outputs
) {
    struct rknpu_submit submit = *submit_template;
    int ret;
    uint64_t start_us;
    uint64_t end_us;

    clear_outputs_and_task_status(resources);

    start_us = now_us();
    ret = ioctl(fd, DRM_IOCTL_RKNPU_SUBMIT, &submit);
    end_us = now_us();

    if (ret < 0) {
        fprintf(stderr,
                "RKNPU_SUBMIT failed: errno=%d (%s)\n",
                errno,
                strerror(errno));
        return -1;
    }

    if (verify_completion_status(resources, &submit) != 0) {
        return -1;
    }

    if (verify_outputs && verify_sampled_outputs(resources) != 0) {
        return -1;
    }

    *elapsed_us_out = end_us - start_us;
    return 0;
}

static int run_benchmark_case(
    int fd,
    const benchmark_scenario_t *scenario,
    operand_mode_t mode,
    uint32_t core_count,
    uint32_t task_count,
    uint32_t warmup_rounds,
    uint32_t measure_rounds,
    int collect_yield_trace,
    uint32_t yield_trace_capacity,
    benchmark_result_t *result
) {
    batch_resources_t resources;
    struct rknpu_submit submit_template;
    uint64_t submit_elapsed_us;
    uint64_t total_submit_us = 0;
    uint64_t min_submit_us = UINT64_MAX;
    uint64_t max_submit_us = 0;
    int descriptor_status = 0;
    int yield_trace_active = 0;

    memset(result, 0, sizeof(*result));

    if (measure_rounds == 0U) {
        fprintf(stderr, "invalid measured round count: %u\n", measure_rounds);
        return -1;
    }

    result->submit_samples_us =
        calloc((size_t)measure_rounds, sizeof(*result->submit_samples_us));
    if (result->submit_samples_us == NULL) {
        fprintf(stderr,
                "failed to allocate %u submit latency samples\n",
                measure_rounds);
        return -1;
    }

    if (core_count > 1 && task_count < core_count) {
        fprintf(stderr,
                "skip scenario=%s mode=%s because task_count=%u is smaller than core_count=%u\n",
                scenario->name,
                operand_mode_name(mode),
                task_count,
                core_count);
        free(result->submit_samples_us);
        result->submit_samples_us = NULL;
        return -1;
    }

    if (allocate_batch_resources(fd, scenario, mode, task_count, &resources) != 0) {
        free(result->submit_samples_us);
        result->submit_samples_us = NULL;
        return -1;
    }

    result->scenario = scenario;
    result->operand_mode = mode;
    result->core_count = core_count;
    result->task_count = task_count;
    result->warmup_rounds = warmup_rounds;
    result->measure_rounds = measure_rounds;
    result->per_task_input_bytes = resources.per_task_input_bytes;
    result->per_task_weight_bytes = resources.per_task_weight_bytes;
    result->per_task_output_bytes = resources.per_task_output_bytes;
    result->total_dma_bytes = resources.total_dma_bytes;

    result->operand_prepare_us = prepare_operand_buffers(&resources);
    result->descriptor_build_us = build_task_descriptors_timed(&resources, &descriptor_status);
    if (descriptor_status != 0) {
        free_batch_resources(fd, &resources);
        free(result->submit_samples_us);
        result->submit_samples_us = NULL;
        return -1;
    }

    init_submit_template(&resources, core_count, &submit_template);

    if (npu_reset(fd) < 0) {
        fprintf(stderr, "warning: npu_reset failed before benchmark case\n");
    }

    printf("  core=%u running %u warmup rounds\n", core_count, warmup_rounds);
    fflush(stdout);
    for (uint32_t round = 0; round < warmup_rounds; round++) {
        if (run_submit_round(fd, &resources, &submit_template, &submit_elapsed_us, 0) != 0) {
            free_batch_resources(fd, &resources);
            free(result->submit_samples_us);
            result->submit_samples_us = NULL;
            return -1;
        }
    }

    if (collect_yield_trace) {
        result->submit_trace_records = calloc(
            (size_t)measure_rounds,
            sizeof(*result->submit_trace_records));
        result->yield_trace_records = calloc(
            (size_t)yield_trace_capacity,
            sizeof(*result->yield_trace_records));
        if (result->submit_trace_records == NULL || result->yield_trace_records == NULL) {
            fprintf(stderr, "failed to allocate userspace yield-trace buffers\n");
            free(result->submit_trace_records);
            free(result->yield_trace_records);
            result->submit_trace_records = NULL;
            result->yield_trace_records = NULL;
            free_batch_resources(fd, &resources);
            free(result->submit_samples_us);
            result->submit_samples_us = NULL;
            return -1;
        }

        /*
         * Do this after allocation but before enabling either trace. The
         * kernel can then copy the complete snapshot without taking a user
         * demand-page fault, and none of the prefault cost enters timings.
         */
        prefault_writable_user_buffer(
            result->submit_trace_records,
            (size_t)measure_rounds * sizeof(*result->submit_trace_records));
        prefault_writable_user_buffer(
            result->yield_trace_records,
            (size_t)yield_trace_capacity *
                sizeof(*result->yield_trace_records));

        /*
         * Warmup is intentionally excluded. Both trace buffers are reset only
         * after warmup and before the first measured blocking ioctl.
         */
        if (reset_submit_trace(fd) != 0 ||
            configure_worker_yield_trace(fd, 1, yield_trace_capacity) != 0) {
            free(result->submit_trace_records);
            free(result->yield_trace_records);
            result->submit_trace_records = NULL;
            result->yield_trace_records = NULL;
            free_batch_resources(fd, &resources);
            free(result->submit_samples_us);
            result->submit_samples_us = NULL;
            return -1;
        }
        yield_trace_active = 1;
    }

    /*
     * Do not print inside the measured loop: serial-console output between
     * rounds changes cooling and scheduler timing. Raw samples are printed
     * after all measurements complete.
     */
    printf("  core=%u collecting %u measured rounds\n", core_count, measure_rounds);
    fflush(stdout);
    for (uint32_t round = 0; round < measure_rounds; round++) {
        if (run_submit_round(
                fd,
                &resources,
                &submit_template,
                &submit_elapsed_us,
                round == 0
            ) != 0) {
            if (yield_trace_active) {
                (void)configure_worker_yield_trace(fd, 0, 0);
            }
            free_batch_resources(fd, &resources);
            free(result->submit_trace_records);
            free(result->yield_trace_records);
            result->submit_trace_records = NULL;
            result->yield_trace_records = NULL;
            free(result->submit_samples_us);
            result->submit_samples_us = NULL;
            return -1;
        }

        result->submit_samples_us[round] = submit_elapsed_us;
        total_submit_us += submit_elapsed_us;
        if (submit_elapsed_us < min_submit_us) {
            min_submit_us = submit_elapsed_us;
        }
        if (submit_elapsed_us > max_submit_us) {
            max_submit_us = submit_elapsed_us;
        }
    }

    if (collect_yield_trace) {
        struct rknpu_submit_trace_query submit_query;
        struct rknpu_worker_yield_trace_query yield_query;
        int trace_status = 0;

        memset(&submit_query, 0, sizeof(submit_query));
        memset(&yield_query, 0, sizeof(yield_query));

        if (read_worker_yield_trace(
                fd,
                result->yield_trace_records,
                yield_trace_capacity,
                &yield_query
            ) != 0) {
            trace_status = -1;
        }
        if (read_submit_trace(
                fd,
                result->submit_trace_records,
                measure_rounds,
                &submit_query
            ) != 0) {
            trace_status = -1;
        }

        /* Reading is complete; disabling also releases the kernel trace buffer. */
        if (configure_worker_yield_trace(fd, 0, 0) != 0) {
            trace_status = -1;
        }
        yield_trace_active = 0;

        if (trace_status != 0 ||
            yield_query.enabled != 1U ||
            yield_query.overflowed != 0 ||
            submit_query.overflowed != 0 ||
            submit_query.count != measure_rounds) {
            fprintf(stderr,
                    "invalid yield experiment trace: yields=%u yield_overflow=%u "
                    "capacity=%u submits=%u expected=%u submit_overflow=%u\n",
                    yield_query.count,
                    yield_query.overflowed,
                    yield_trace_capacity,
                    submit_query.count,
                    measure_rounds,
                    submit_query.overflowed);
            free_batch_resources(fd, &resources);
            free(result->submit_trace_records);
            free(result->yield_trace_records);
            result->submit_trace_records = NULL;
            result->yield_trace_records = NULL;
            free(result->submit_samples_us);
            result->submit_samples_us = NULL;
            return -1;
        }

        result->submit_trace_count = submit_query.count;
        result->yield_trace_count = yield_query.count;
        result->yield_trace_collected = 1;
    }

    if (benchmark_compute_latency_stats(
            result->submit_samples_us,
            measure_rounds,
            &result->latency_stats
        ) != 0) {
        fprintf(stderr, "failed to calculate submit latency statistics\n");
        free_batch_resources(fd, &resources);
        free(result->submit_trace_records);
        free(result->yield_trace_records);
        result->submit_trace_records = NULL;
        result->yield_trace_records = NULL;
        free(result->submit_samples_us);
        result->submit_samples_us = NULL;
        return -1;
    }

    result->min_submit_us = result->latency_stats.min_us;
    result->max_submit_us = result->latency_stats.max_us;
    result->total_submit_us = total_submit_us;
    result->avg_submit_us = result->latency_stats.mean_us;
    result->avg_submit_ms = result->avg_submit_us / 1000.0;
    result->avg_task_us = result->avg_submit_us / (double)task_count;
    result->tasks_per_sec = (double)task_count * 1000000.0 / result->avg_submit_us;

    {
        double macs_per_round =
            (double)scenario->m * (double)scenario->k * (double)scenario->n * (double)task_count;
        double seconds_per_round = result->avg_submit_us / 1000000.0;
        result->gmac_per_sec = macs_per_round / seconds_per_round / 1000000000.0;
        result->gflops_per_sec = result->gmac_per_sec * 2.0;
    }

    result->jitter_pct = result->avg_submit_us > 0.0
        ? ((double)(max_submit_us - min_submit_us) / result->avg_submit_us) * 100.0
        : 0.0;
    result->valid = 1;

    free_batch_resources(fd, &resources);
    return 0;
}

static void print_dma_footprint(const benchmark_result_t *result) {
    printf("  per-task input  : %8.2f KiB\n", (double)result->per_task_input_bytes / 1024.0);
    printf("  per-task weight : %8.2f KiB\n", (double)result->per_task_weight_bytes / 1024.0);
    printf("  per-task output : %8.2f KiB\n", (double)result->per_task_output_bytes / 1024.0);
    printf("  total DMA bytes : %8.2f MiB\n", (double)result->total_dma_bytes / (1024.0 * 1024.0));
}

static double ratio_u64(uint64_t numerator, uint64_t denominator) {
    return denominator == 0U ? 0.0 : (double)numerator / (double)denominator;
}

static const char *latency_stability_hint(const benchmark_latency_stats_t *stats) {
    double max_over_p50 = ratio_u64(stats->max_us, stats->p50_us);

    if (stats->p95_over_p50 >= 1.20) {
        return "recurring/persistent tail: P95 is at least 20% above P50";
    }
    if (stats->p99_over_p50 >= 1.20 || max_over_p50 >= 1.50) {
        return "sporadic tail: P50/P95 are stable but rare rounds are slower";
    }
    return "stable by the 20% percentile-tail heuristic";
}

static void print_latency_samples(const benchmark_result_t *result) {
    printf("  raw submit samples (us), core=%u:\n", result->core_count);
    for (uint32_t round = 0; round < result->measure_rounds; ++round) {
        if (round % 10U == 0U) {
            printf("    %03u-%03u:",
                   round + 1U,
                   round + 10U < result->measure_rounds
                       ? round + 10U
                       : result->measure_rounds);
        }
        printf(" %llu",
               (unsigned long long)result->submit_samples_us[round]);
        if (round % 10U == 9U || round + 1U == result->measure_rounds) {
            printf("\n");
        }
    }
}

static void print_case_metrics(const benchmark_result_t *result) {
    const benchmark_latency_stats_t *stats = &result->latency_stats;
    double slow_sample_pct =
        (double)stats->above_150pct_p50_count * 100.0 /
        (double)stats->sample_count;

    printf("  setup operands  : %8.3f ms\n", (double)result->operand_prepare_us / 1000.0);
    printf("  build regcmds   : %8.3f ms\n", (double)result->descriptor_build_us / 1000.0);
    printf("  samples         : %8zu measured rounds\n", stats->sample_count);
    printf("  mean submit     : %8.3f ms\n", result->avg_submit_ms);
    printf("  P50 / P95 / P99: %8.3f / %8.3f / %8.3f ms\n",
           (double)stats->p50_us / 1000.0,
           (double)stats->p95_us / 1000.0,
           (double)stats->p99_us / 1000.0);
    printf("  min / max       : %8.3f / %8.3f ms\n",
           (double)result->min_submit_us / 1000.0,
           (double)result->max_submit_us / 1000.0);
    printf("  P95/P50 P99/P50: %8.3f / %8.3f x\n",
           stats->p95_over_p50,
           stats->p99_over_p50);
    printf("  >150%% of P50    : %8zu / %zu rounds (%5.1f%%)\n",
           stats->above_150pct_p50_count,
           stats->sample_count,
           slow_sample_pct);
    printf("  stability hint  : %s\n", latency_stability_hint(stats));
    printf("  avg task        : %8.3f us\n", result->avg_task_us);
    printf("  tasks / sec     : %8.2f\n", result->tasks_per_sec);
    printf("  GMAC / sec      : %8.3f\n", result->gmac_per_sec);
    printf("  GFLOP / sec     : %8.3f\n", result->gflops_per_sec);
    printf("  jitter span     : %8.2f %%\n", result->jitter_pct);
    print_latency_samples(result);
}

static int find_submit_trace_index(
    const benchmark_result_t *result,
    uint64_t queue_task
) {
    for (uint32_t index = 0; index < result->submit_trace_count; index++) {
        if (result->submit_trace_records[index].queue_task == queue_task) {
            return (int)index;
        }
    }
    return -1;
}

static const char *yield_reason_name(uint32_t reason) {
    if (reason == RKNPU_WORKER_YIELD_REASON_INFLIGHT) {
        return "inflight";
    }
    if (reason == RKNPU_WORKER_YIELD_REASON_STALLED) {
        return "stalled";
    }
    return "invalid";
}

/*
 * Analyze only after all measured submits have returned. This function is the
 * sole printing point for the experiment timestamps; the new Worker trace hook
 * performs no logging, and the measured loop performs no sample logging.
 */
static int report_worker_yield_trace(const benchmark_result_t *result) {
    benchmark_latency_stats_t gap_stats;
    yield_round_summary_t *rounds = NULL;
    uint64_t *gap_samples_ns = NULL;
    uint32_t gap_histogram[YIELD_GAP_HISTOGRAM_BUCKET_COUNT + 1U] = {0};
    uint32_t inflight_count = 0;
    uint32_t stalled_count = 0;
    int status = -1;

    if (!result->yield_trace_collected) {
        return 0;
    }
    if (result->submit_trace_count != result->measure_rounds ||
        result->yield_trace_count == 0) {
        fprintf(stderr,
                "yield trace cannot be analyzed: submits=%u rounds=%u yields=%u\n",
                result->submit_trace_count,
                result->measure_rounds,
                result->yield_trace_count);
        return -1;
    }

    rounds = calloc((size_t)result->measure_rounds, sizeof(*rounds));
    gap_samples_ns = calloc((size_t)result->yield_trace_count, sizeof(*gap_samples_ns));
    if (rounds == NULL || gap_samples_ns == NULL) {
        fprintf(stderr, "failed to allocate yield trace analysis buffers\n");
        goto out;
    }

    for (uint32_t index = 0; index < result->submit_trace_count; index++) {
        const struct rknpu_submit_trace_record *submit =
            &result->submit_trace_records[index];
        if (submit->queue_task == 0 ||
            submit->t0_ns > submit->t1_ns ||
            submit->t1_ns > submit->t2_ns ||
            submit->t2_ns > submit->t3_ns ||
            submit->t3_ns > submit->t4_ns) {
            fprintf(stderr, "invalid submit trace record at index=%u\n", index);
            goto out;
        }
    }

    for (uint32_t index = 0; index < result->yield_trace_count; index++) {
        const struct rknpu_worker_yield_trace_record *record =
            &result->yield_trace_records[index];
        uint64_t gap_ns;
        int submit_index;
        yield_round_summary_t *round;
        const struct rknpu_submit_trace_record *submit;

        if (record->sequence != index ||
            record->reserved != 0 ||
            record->queue_task == 0 ||
            record->yield_end_ns < record->yield_start_ns ||
            (record->reason != RKNPU_WORKER_YIELD_REASON_INFLIGHT &&
             record->reason != RKNPU_WORKER_YIELD_REASON_STALLED)) {
            fprintf(stderr, "invalid Worker yield trace record at index=%u\n", index);
            goto out;
        }

        submit_index = find_submit_trace_index(result, record->queue_task);
        if (submit_index < 0) {
            fprintf(stderr,
                    "yield trace queue_task=%llu has no measured submit record\n",
                    (unsigned long long)record->queue_task);
            goto out;
        }
        submit = &result->submit_trace_records[submit_index];
        uint64_t earliest_yield_ns =
            record->reason == RKNPU_WORKER_YIELD_REASON_INFLIGHT
                ? submit->t2_ns
                : submit->t1_ns;
        if (record->yield_start_ns < earliest_yield_ns ||
            record->yield_end_ns > submit->t3_ns) {
            fprintf(stderr,
                    "yield trace sequence=%llu lies outside its submit lifecycle\n",
                    (unsigned long long)record->sequence);
            goto out;
        }

        gap_ns = record->yield_end_ns - record->yield_start_ns;
        gap_samples_ns[index] = gap_ns;
        {
            uint64_t bucket = gap_ns / YIELD_GAP_HISTOGRAM_BUCKET_NS;
            uint32_t bucket_index = bucket < YIELD_GAP_HISTOGRAM_BUCKET_COUNT
                ? (uint32_t)bucket
                : YIELD_GAP_HISTOGRAM_BUCKET_COUNT;
            gap_histogram[bucket_index] += 1U;
        }
        round = &rounds[submit_index];
        round->yield_count += 1;
        round->sum_gap_ns += gap_ns;
        if (gap_ns > round->max_gap_ns) {
            round->max_gap_ns = gap_ns;
        }
        round->last_gap_ns = gap_ns;
        round->last_yield_end_ns = record->yield_end_ns;

        if (record->reason == RKNPU_WORKER_YIELD_REASON_INFLIGHT) {
            inflight_count += 1;
        } else {
            stalled_count += 1;
            round->stalled_yield_count += 1;
        }
    }

    if (benchmark_compute_latency_stats(
            gap_samples_ns,
            result->yield_trace_count,
            &gap_stats
        ) != 0) {
        fprintf(stderr, "failed to calculate yield_gap statistics\n");
        goto out;
    }

    printf("  Worker yield experiment (measured window only)\n");
    printf("  yield records    : %8u total (%u inflight, %u stalled)\n",
           result->yield_trace_count,
           inflight_count,
           stalled_count);
    printf("  yield_gap mean   : %8.3f us\n", gap_stats.mean_us / 1000.0);
    printf("  yield_gap P50/95/99: %7.3f / %7.3f / %7.3f us\n",
           (double)gap_stats.p50_us / 1000.0,
           (double)gap_stats.p95_us / 1000.0,
           (double)gap_stats.p99_us / 1000.0);
    printf("  yield_gap min/max: %8.3f / %8.3f us\n",
           (double)gap_stats.min_us / 1000.0,
           (double)gap_stats.max_us / 1000.0);

    /*
     * A 0.1 ms bucket is narrow enough to expose levels separated by about
     * 1.9 ms without treating nanosecond-level noise as a scheduler class.
     * The last bucket collects gaps of 100 ms or more.
     */
    printf("  yield_gap histogram (0.1 ms buckets, non-empty only):\n");
    for (uint32_t index = 0; index < YIELD_GAP_HISTOGRAM_BUCKET_COUNT; index++) {
        if (gap_histogram[index] == 0U) {
            continue;
        }
        printf("    [%6.1f, %6.1f) ms : %6u (%6.2f%%)\n",
               (double)index / 10.0,
               (double)(index + 1U) / 10.0,
               gap_histogram[index],
               (double)gap_histogram[index] * 100.0 /
                   (double)result->yield_trace_count);
    }
    if (gap_histogram[YIELD_GAP_HISTOGRAM_BUCKET_COUNT] != 0U) {
        printf("    [%6.1f,    inf) ms : %6u (%6.2f%%)\n",
               (double)YIELD_GAP_HISTOGRAM_BUCKET_COUNT / 10.0,
               gap_histogram[YIELD_GAP_HISTOGRAM_BUCKET_COUNT],
               (double)gap_histogram[YIELD_GAP_HISTOGRAM_BUCKET_COUNT] * 100.0 /
                   (double)result->yield_trace_count);
    }

    printf("  per-round yield correlation:\n");
    printf("    round queue_task submit_us driver_t3-t2_us yields stalled "
           "sum_gap_us max_gap_us last_gap_us t3-after-last-us\n");
    for (uint32_t index = 0; index < result->measure_rounds; index++) {
        const struct rknpu_submit_trace_record *submit =
            &result->submit_trace_records[index];
        const yield_round_summary_t *round = &rounds[index];
        uint64_t after_last_ns = round->last_yield_end_ns == 0
            ? 0
            : submit->t3_ns - round->last_yield_end_ns;

        printf("    %03u %10llu %9llu %15.3f %6u %7u %10.3f %10.3f %11.3f %16.3f\n",
               index + 1U,
               (unsigned long long)submit->queue_task,
               (unsigned long long)result->submit_samples_us[index],
               (double)(submit->t3_ns - submit->t2_ns) / 1000.0,
               round->yield_count,
               round->stalled_yield_count,
               (double)round->sum_gap_ns / 1000.0,
               (double)round->max_gap_ns / 1000.0,
               (double)round->last_gap_ns / 1000.0,
               (double)after_last_ns / 1000.0);
    }

    printf("  raw Worker yield timestamps (printed after measurement):\n");
    printf("    sequence queue_task start_ns end_ns yield_gap_ns reason\n");
    for (uint32_t index = 0; index < result->yield_trace_count; index++) {
        const struct rknpu_worker_yield_trace_record *record =
            &result->yield_trace_records[index];
        printf("    %8llu %10llu %16llu %16llu %12llu %s\n",
               (unsigned long long)record->sequence,
               (unsigned long long)record->queue_task,
               (unsigned long long)record->yield_start_ns,
               (unsigned long long)record->yield_end_ns,
               (unsigned long long)(record->yield_end_ns - record->yield_start_ns),
               yield_reason_name(record->reason));
    }

    status = 0;

out:
    free(gap_samples_ns);
    free(rounds);
    return status;
}

static void print_comparison(
    const benchmark_result_t *one_core,
    const benchmark_result_t *three_core
) {
    double mean_speedup = three_core->avg_submit_us > 0.0
        ? one_core->avg_submit_us / three_core->avg_submit_us
        : 0.0;
    double efficiency = three_core->core_count > 0U
        ? mean_speedup / (double)three_core->core_count * 100.0
        : 0.0;
    double p50_speedup = ratio_u64(
        one_core->latency_stats.p50_us,
        three_core->latency_stats.p50_us);
    double p95_speedup = ratio_u64(
        one_core->latency_stats.p95_us,
        three_core->latency_stats.p95_us);
    double p99_speedup = ratio_u64(
        one_core->latency_stats.p99_us,
        three_core->latency_stats.p99_us);

    printf("comparison summary\n");
    printf("                         1-core       %u-core\n", three_core->core_count);
    printf("  mean submit       : %10.3f / %10.3f ms\n",
           one_core->avg_submit_ms,
           three_core->avg_submit_ms);
    printf("  P50 submit        : %10.3f / %10.3f ms\n",
           (double)one_core->latency_stats.p50_us / 1000.0,
           (double)three_core->latency_stats.p50_us / 1000.0);
    printf("  P95 submit        : %10.3f / %10.3f ms\n",
           (double)one_core->latency_stats.p95_us / 1000.0,
           (double)three_core->latency_stats.p95_us / 1000.0);
    printf("  P99 submit        : %10.3f / %10.3f ms\n",
           (double)one_core->latency_stats.p99_us / 1000.0,
           (double)three_core->latency_stats.p99_us / 1000.0);
    printf("  mean speedup      : %10.3f x (ratio of mean latencies)\n",
           mean_speedup);
    printf("  P50/P95/P99 gain : %10.3f / %10.3f / %10.3f x\n",
           p50_speedup,
           p95_speedup,
           p99_speedup);
    printf("  parallel eff.     : %10.2f %% of ideal %ux\n",
           efficiency,
           three_core->core_count);
    printf("  throughput gain   : %8.3f x tasks/sec\n",
           three_core->tasks_per_sec / one_core->tasks_per_sec);
    printf("  GMAC gain         : %8.3f x\n",
           three_core->gmac_per_sec / one_core->gmac_per_sec);
}

static void free_benchmark_result(benchmark_result_t *result) {
    free(result->submit_samples_us);
    free(result->submit_trace_records);
    free(result->yield_trace_records);
    result->submit_samples_us = NULL;
    result->submit_trace_records = NULL;
    result->yield_trace_records = NULL;
    result->valid = 0;
}

static void print_usage(const char *argv0) {
    printf("Usage: %s [options]\n", argv0);
    printf("Options:\n");
    printf("  --scenario <name|all>   Run only one scenario (default: all)\n");
    printf("  --rounds <count>        Override measured rounds (default: %u)\n",
           DEFAULT_MEASURE_ROUNDS);
    printf("  --warmup <count>        Override warmup rounds for every scenario\n");
    printf("  --task-cap <count>      Cap task count for both shared/unique modes\n");
    printf("  --cores <1|3|both>      Select core cases (default: both)\n");
    printf("  --yield-trace           Collect Worker yield gaps; requires llama_decode_like and --cores 3\n");
    printf("  --yield-capacity <n>    Trace records to preallocate (default: %u, max: %u)\n",
           RKNPU_WORKER_YIELD_TRACE_DEFAULT_CAPACITY,
           RKNPU_WORKER_YIELD_TRACE_MAX_CAPACITY);
    printf("  --no-shared             Skip shared-operands mode\n");
    printf("  --no-unique             Skip unique-operands mode\n");
    printf("  --help                  Show this message\n");
    printf("\nAvailable scenarios:\n");
    for (size_t index = 0; index < sizeof(k_scenarios) / sizeof(k_scenarios[0]); index++) {
        printf("  %-18s  M=%u K=%u N=%u\n",
               k_scenarios[index].name,
               k_scenarios[index].m,
               k_scenarios[index].k,
               k_scenarios[index].n);
    }
}

static int parse_u32(const char *text, uint32_t *value_out) {
    char *end = NULL;
    unsigned long parsed;

    errno = 0;
    parsed = strtoul(text, &end, 10);
    if (!text[0] ||
        end == text ||
        (end != NULL && *end != '\0') ||
        errno == ERANGE ||
        parsed > UINT32_MAX) {
        return -1;
    }
    *value_out = (uint32_t)parsed;
    return 0;
}

static int parse_cli(int argc, char **argv, cli_options_t *options) {
    memset(options, 0, sizeof(*options));
    options->scenario_filter = "all";
    options->run_shared = 1;
    options->run_unique = 1;
    options->run_one_core = 1;
    options->run_three_core = 1;
    options->yield_trace_capacity = RKNPU_WORKER_YIELD_TRACE_DEFAULT_CAPACITY;

    for (int index = 1; index < argc; index++) {
        const char *arg = argv[index];

        if (strcmp(arg, "--help") == 0) {
            print_usage(argv[0]);
            return 1;
        }
        if (strcmp(arg, "--no-shared") == 0) {
            options->run_shared = 0;
            continue;
        }
        if (strcmp(arg, "--no-unique") == 0) {
            options->run_unique = 0;
            continue;
        }
        if (strcmp(arg, "--yield-trace") == 0) {
            options->collect_yield_trace = 1;
            continue;
        }
        if ((strcmp(arg, "--scenario") == 0) ||
            (strcmp(arg, "--rounds") == 0) ||
            (strcmp(arg, "--warmup") == 0) ||
            (strcmp(arg, "--task-cap") == 0) ||
            (strcmp(arg, "--yield-capacity") == 0) ||
            (strcmp(arg, "--cores") == 0)) {
            if (index + 1 >= argc) {
                fprintf(stderr, "missing value for %s\n", arg);
                return -1;
            }
            index += 1;

            if (strcmp(arg, "--scenario") == 0) {
                options->scenario_filter = argv[index];
            } else if (strcmp(arg, "--rounds") == 0) {
                if (parse_u32(argv[index], &options->rounds_override) != 0 ||
                    options->rounds_override == 0U) {
                    fprintf(stderr, "invalid --rounds value: %s\n", argv[index]);
                    return -1;
                }
                options->has_rounds_override = 1;
            } else if (strcmp(arg, "--warmup") == 0) {
                if (parse_u32(argv[index], &options->warmup_override) != 0) {
                    fprintf(stderr, "invalid --warmup value: %s\n", argv[index]);
                    return -1;
                }
                options->has_warmup_override = 1;
            } else if (strcmp(arg, "--task-cap") == 0) {
                if (parse_u32(argv[index], &options->task_cap) != 0) {
                    fprintf(stderr, "invalid --task-cap value: %s\n", argv[index]);
                    return -1;
                }
            } else if (strcmp(arg, "--yield-capacity") == 0) {
                if (parse_u32(argv[index], &options->yield_trace_capacity) != 0 ||
                    options->yield_trace_capacity == 0U ||
                    options->yield_trace_capacity >
                        RKNPU_WORKER_YIELD_TRACE_MAX_CAPACITY) {
                    fprintf(stderr, "invalid --yield-capacity value: %s\n", argv[index]);
                    return -1;
                }
                options->has_yield_trace_capacity = 1;
            } else if (strcmp(arg, "--cores") == 0) {
                if (strcmp(argv[index], "1") == 0) {
                    options->run_one_core = 1;
                    options->run_three_core = 0;
                } else if (strcmp(argv[index], "3") == 0) {
                    options->run_one_core = 0;
                    options->run_three_core = 1;
                } else if (strcmp(argv[index], "both") == 0) {
                    options->run_one_core = 1;
                    options->run_three_core = 1;
                } else {
                    fprintf(stderr, "invalid --cores value: %s\n", argv[index]);
                    return -1;
                }
            }
            continue;
        }

        fprintf(stderr, "unknown option: %s\n", arg);
        return -1;
    }

    if (!options->run_shared && !options->run_unique) {
        fprintf(stderr, "both shared and unique modes were disabled\n");
        return -1;
    }

    if (options->has_yield_trace_capacity && !options->collect_yield_trace) {
        fprintf(stderr, "--yield-capacity requires --yield-trace\n");
        return -1;
    }

    if (options->collect_yield_trace &&
        (strcmp(options->scenario_filter, "llama_decode_like") != 0 ||
         options->run_one_core ||
         !options->run_three_core)) {
        fprintf(stderr,
                "--yield-trace requires --scenario llama_decode_like --cores 3\n");
        return -1;
    }

    return 0;
}

static int scenario_matches_filter(const benchmark_scenario_t *scenario, const char *filter) {
    return strcmp(filter, "all") == 0 || strcmp(filter, scenario->name) == 0;
}

static uint32_t maybe_cap_tasks(uint32_t task_count, uint32_t cap) {
    if (cap == 0 || task_count <= cap) {
        return task_count;
    }
    return cap;
}

static uint32_t effective_warmup_rounds(
    const benchmark_scenario_t *scenario,
    const cli_options_t *options
) {
    return options->has_warmup_override ? options->warmup_override : scenario->warmup_rounds;
}

static uint32_t effective_measure_rounds(
    const benchmark_scenario_t *scenario,
    const cli_options_t *options
) {
    return options->has_rounds_override ? options->rounds_override : scenario->measure_rounds;
}

static uint32_t count_selected_mode_pairs(const cli_options_t *options) {
    uint32_t count = 0;

    for (size_t scenario_index = 0; scenario_index < sizeof(k_scenarios) / sizeof(k_scenarios[0]); scenario_index++) {
        const benchmark_scenario_t *scenario = &k_scenarios[scenario_index];

        if (!scenario_matches_filter(scenario, options->scenario_filter)) {
            continue;
        }

        if (options->run_shared &&
            maybe_cap_tasks(tasks_for_mode(scenario, OPERANDS_SHARED), options->task_cap) > 0) {
            count += 1;
        }

        if (options->run_unique &&
            maybe_cap_tasks(tasks_for_mode(scenario, OPERANDS_UNIQUE), options->task_cap) > 0) {
            count += 1;
        }
    }

    return count;
}

static uint32_t count_planned_submit_rounds(const cli_options_t *options) {
    uint32_t total_rounds = 0;
    uint32_t core_case_count =
        (options->run_one_core ? 1U : 0U) +
        (options->run_three_core ? 1U : 0U);

    for (size_t scenario_index = 0; scenario_index < sizeof(k_scenarios) / sizeof(k_scenarios[0]); scenario_index++) {
        const benchmark_scenario_t *scenario = &k_scenarios[scenario_index];
        uint32_t rounds_per_case;

        if (!scenario_matches_filter(scenario, options->scenario_filter)) {
            continue;
        }

        rounds_per_case = effective_warmup_rounds(scenario, options) +
            effective_measure_rounds(scenario, options);

        if (options->run_shared &&
            maybe_cap_tasks(tasks_for_mode(scenario, OPERANDS_SHARED), options->task_cap) > 0) {
            total_rounds += rounds_per_case * core_case_count;
        }

        if (options->run_unique &&
            maybe_cap_tasks(tasks_for_mode(scenario, OPERANDS_UNIQUE), options->task_cap) > 0) {
            total_rounds += rounds_per_case * core_case_count;
        }
    }

    return total_rounds;
}

static int run_mode_pair(
    int fd,
    const benchmark_scenario_t *scenario,
    operand_mode_t mode,
    const cli_options_t *options,
    uint32_t mode_pair_index,
    uint32_t total_mode_pairs
) {
    benchmark_result_t one_core;
    benchmark_result_t three_core;
    uint32_t task_count = maybe_cap_tasks(tasks_for_mode(scenario, mode), options->task_cap);
    uint32_t warmup_rounds = effective_warmup_rounds(scenario, options);
    uint32_t measure_rounds = effective_measure_rounds(scenario, options);

    memset(&one_core, 0, sizeof(one_core));
    memset(&three_core, 0, sizeof(three_core));

    if (task_count == 0) {
        printf("skip mode=%s because this scenario does not define a valid task batch\n",
               operand_mode_name(mode));
        return 0;
    }

    printf("\n[case %u/%u] [%s] task_count=%u warmup=%u measured=%u\n",
           mode_pair_index,
           total_mode_pairs,
           operand_mode_name(mode),
           task_count,
           warmup_rounds,
           measure_rounds);
    printf("  %s\n", operand_mode_summary(mode));

    if (options->run_one_core) {
        if (run_benchmark_case(
                fd,
                scenario,
                mode,
                1,
                task_count,
                warmup_rounds,
                measure_rounds,
                0,
                options->yield_trace_capacity,
                &one_core
            ) != 0) {
            fprintf(stderr, "1-core benchmark failed for scenario=%s mode=%s\n",
                    scenario->name,
                    operand_mode_name(mode));
            return -1;
        }

        printf("  1-core metrics\n");
        print_dma_footprint(&one_core);
        print_case_metrics(&one_core);
    }

    if (options->run_three_core) {
        if (run_benchmark_case(
                fd,
                scenario,
                mode,
                3,
                task_count,
                warmup_rounds,
                measure_rounds,
                options->collect_yield_trace,
                options->yield_trace_capacity,
                &three_core
            ) != 0) {
            fprintf(stderr, "3-core benchmark failed for scenario=%s mode=%s\n",
                    scenario->name,
                    operand_mode_name(mode));
            if (one_core.valid) {
                free_benchmark_result(&one_core);
            }
            return -1;
        }

        printf("  3-core metrics\n");
        print_dma_footprint(&three_core);
        print_case_metrics(&three_core);
        if (report_worker_yield_trace(&three_core) != 0) {
            free_benchmark_result(&three_core);
            if (one_core.valid) {
                free_benchmark_result(&one_core);
            }
            return -1;
        }
    }

    if (one_core.valid && three_core.valid) {
        print_comparison(&one_core, &three_core);
    }
    printf("  mode complete   : %s\n", operand_mode_name(mode));
    if (three_core.valid) {
        free_benchmark_result(&three_core);
    }
    if (one_core.valid) {
        free_benchmark_result(&one_core);
    }
    fflush(stdout);
    return 0;
}

int main(int argc, char **argv) {
    cli_options_t options;
    int fd;
    int overall_status = 0;
    int matched_any = 0;
    uint32_t total_mode_pairs;
    uint32_t planned_submit_rounds;
    uint32_t mode_pair_index = 0;

    {
        int parse_status = parse_cli(argc, argv, &options);
        if (parse_status > 0) {
            return 0;
        }
        if (parse_status < 0) {
            print_usage(argv[0]);
            return 1;
        }
    }

    fd = npu_open();
    if (fd < 0) {
        fprintf(stderr, "failed to open /dev/dri/card1\n");
        return 1;
    }

    printf("core scaling benchmark\n");
    printf("  measured window : blocking DRM_IOCTL_RKNPU_SUBMIT only\n");
    printf("  setup timing    : operand packing and regcmd generation are reported separately\n");
    if (options.run_one_core && options.run_three_core) {
        printf("  comparison      : 1 core vs 3 cores on identical task batches\n");
    } else {
        printf("  selected cores  : %s only\n", options.run_three_core ? "3" : "1");
    }
    printf("  Worker trace    : %s\n",
           options.collect_yield_trace
               ? "enabled for the 3-core measured window"
               : "disabled");
    if (options.collect_yield_trace) {
        printf("  trace capacity  : %u records (%.2f MiB per buffer)\n",
               options.yield_trace_capacity,
               (double)options.yield_trace_capacity *
                   (double)sizeof(struct rknpu_worker_yield_trace_record) /
                   (1024.0 * 1024.0));
    }
    total_mode_pairs = count_selected_mode_pairs(&options);
    planned_submit_rounds = count_planned_submit_rounds(&options);
    printf("  selected pairs  : %u\n", total_mode_pairs);
    printf("  planned submits : %u blocking ioctl rounds\n", planned_submit_rounds);

    for (size_t scenario_index = 0; scenario_index < sizeof(k_scenarios) / sizeof(k_scenarios[0]); scenario_index++) {
        const benchmark_scenario_t *scenario = &k_scenarios[scenario_index];

        if (!scenario_matches_filter(scenario, options.scenario_filter)) {
            continue;
        }

        matched_any = 1;

        printf("\n================================================================================\n");
        printf("scenario: %s\n", scenario->name);
        printf("  description : %s\n", scenario->description);
        printf("  dimensions  : M=%u K=%u N=%u\n", scenario->m, scenario->k, scenario->n);
        printf("================================================================================\n");

        if (options.run_shared) {
            if (maybe_cap_tasks(tasks_for_mode(scenario, OPERANDS_SHARED), options.task_cap) > 0) {
                mode_pair_index += 1;
                if (run_mode_pair(
                        fd,
                        scenario,
                        OPERANDS_SHARED,
                        &options,
                        mode_pair_index,
                        total_mode_pairs
                    ) != 0) {
                    overall_status = 1;
                }
            }
        }

        if (options.run_unique) {
            if (maybe_cap_tasks(tasks_for_mode(scenario, OPERANDS_UNIQUE), options.task_cap) > 0) {
                mode_pair_index += 1;
                if (run_mode_pair(
                        fd,
                        scenario,
                        OPERANDS_UNIQUE,
                        &options,
                        mode_pair_index,
                        total_mode_pairs
                    ) != 0) {
                    overall_status = 1;
                }
            }
        }
    }

    if (!matched_any) {
        fprintf(stderr, "no scenario matched filter: %s\n", options.scenario_filter);
        overall_status = 1;
    }

    printf("\nbenchmark complete status=%d\n", overall_status);
    fflush(stdout);
    npu_close(fd);
    return overall_status;
}
