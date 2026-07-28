/*
 * Copyright (C) 2024  Jasbir Matharu, <jasjnuk@gmail.com>
 *
 * This file is part of rk3588-npu.
 *
 * rk3588-npu is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * rk3588-npu is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with rk3588-npu.  If not, see <https://www.gnu.org/licenses/>.
 *
 * ---------------------------------------------------------------------------
 * Benchmark intent
 * ---------------------------------------------------------------------------
 *
 * This program compares one-core and three-core execution on the same batch of
 * independent matmul tasks. The goal is not to measure only one number, but to
 * break the workload into several views that help explain why a result is fast
 * or slow:
 *
 * 1. Shared-operands mode
 *    Every task reuses the same input and weight buffers but writes to a
 *    private output slice. This isolates scheduler/core-parallel scaling by
 *    minimizing DMA working-set growth.
 *
 * 2. Unique-operands mode
 *    Every task owns a private input, weight, and output slice. This stresses
 *    the real "many independent matrices" case where DMA traffic, GEM
 *    footprint, and descriptor fan-out all grow with task count.
 *
 * 3. One-time setup vs steady-state submit
 *    Buffer packing and regcmd generation are timed separately from the
 *    blocking submit ioctl. This avoids mixing CPU-side preparation cost with
 *    the actual NPU execution window.
 *
 * 4. Correctness sampling
 *    For each measured case we validate task completion status and check a few
 *    sampled output coordinates against a deterministic CPU reference. The
 *    reference uses integer-valued operands so expected results stay stable and
 *    easy to inspect.
 *
 * In short:
 *   - one core vs three cores tells us parallel scaling
 *   - shared vs unique operands tells us scheduler-vs-memory sensitivity
 *   - setup vs submit tells us software overhead vs hardware throughput
 */

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

#include "npu_interface.h"
#include "npu_matmul.h"
#include "rknpu-ioctl.h"

#define REGCMD_WORDS 112U
#define REGCMD_BYTES (REGCMD_WORDS * sizeof(uint64_t))
#define INPUT_LAYOUT_C2 8
#define OUTPUT_LAYOUT_C2 4
#define DMA_SLICE_ALIGN 64U
#define DEFAULT_TIMEOUT_MS 6000U

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
} cli_options_t;

static const benchmark_scenario_t k_scenarios[] = {
    {
        .name = "tiny_dispatch",
        .description =
            "Small matrices where submit/scheduler overhead dominates and core-scaling "
            "efficiency is usually lower than the ideal 3x.",
        .m = 4,
        .k = 32,
        .n = 16,
        .shared_tasks = 96,
        .unique_tasks = 96,
        .warmup_rounds = 2,
        .measure_rounds = 12,
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
        .measure_rounds = 8,
    },
    {
        .name = "throughput_heavy",
        .description =
            "Large matrices intended to shift the bottleneck toward math throughput. "
            "Unique mode uses fewer tasks to keep DMA footprint bounded.",
        .m = 128,
        .k = 1024,
        .n = 1024,
        .shared_tasks = 24,
        .unique_tasks = 4,
        .warmup_rounds = 2,
        .measure_rounds = 5,
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
        .measure_rounds = 8,
    },
};

static uint64_t now_us(void) {
    struct timeval tv;
    gettimeofday(&tv, NULL);
    return (uint64_t)tv.tv_sec * 1000000ULL + (uint64_t)tv.tv_usec;
}

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
    benchmark_result_t *result
) {
    batch_resources_t resources;
    struct rknpu_submit submit_template;
    uint64_t submit_elapsed_us;
    uint64_t total_submit_us = 0;
    uint64_t min_submit_us = UINT64_MAX;
    uint64_t max_submit_us = 0;
    int descriptor_status = 0;

    memset(result, 0, sizeof(*result));

    if (core_count > 1 && task_count < core_count) {
        fprintf(stderr,
                "skip scenario=%s mode=%s because task_count=%u is smaller than core_count=%u\n",
                scenario->name,
                operand_mode_name(mode),
                task_count,
                core_count);
        return -1;
    }

    if (allocate_batch_resources(fd, scenario, mode, task_count, &resources) != 0) {
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
        return -1;
    }

    init_submit_template(&resources, core_count, &submit_template);

    if (npu_reset(fd) < 0) {
        fprintf(stderr, "warning: npu_reset failed before benchmark case\n");
    }

    for (uint32_t round = 0; round < warmup_rounds; round++) {
        printf("  core=%u warmup round %u/%u\n", core_count, round + 1, warmup_rounds);
        fflush(stdout);
        if (run_submit_round(fd, &resources, &submit_template, &submit_elapsed_us, 0) != 0) {
            free_batch_resources(fd, &resources);
            return -1;
        }
    }

    for (uint32_t round = 0; round < measure_rounds; round++) {
        printf("  core=%u measure round %u/%u\n", core_count, round + 1, measure_rounds);
        fflush(stdout);
        if (run_submit_round(
                fd,
                &resources,
                &submit_template,
                &submit_elapsed_us,
                round == 0
            ) != 0) {
            free_batch_resources(fd, &resources);
            return -1;
        }

        total_submit_us += submit_elapsed_us;
        if (submit_elapsed_us < min_submit_us) {
            min_submit_us = submit_elapsed_us;
        }
        if (submit_elapsed_us > max_submit_us) {
            max_submit_us = submit_elapsed_us;
        }
    }

    result->min_submit_us = min_submit_us;
    result->max_submit_us = max_submit_us;
    result->total_submit_us = total_submit_us;
    result->avg_submit_us = (double)total_submit_us / (double)measure_rounds;
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

static void print_case_metrics(const benchmark_result_t *result) {
    printf("  setup operands  : %8.3f ms\n", (double)result->operand_prepare_us / 1000.0);
    printf("  build regcmds   : %8.3f ms\n", (double)result->descriptor_build_us / 1000.0);
    printf("  avg submit      : %8.3f ms\n", result->avg_submit_ms);
    printf("  min / max       : %8.3f / %8.3f ms\n",
           (double)result->min_submit_us / 1000.0,
           (double)result->max_submit_us / 1000.0);
    printf("  avg task        : %8.3f us\n", result->avg_task_us);
    printf("  tasks / sec     : %8.2f\n", result->tasks_per_sec);
    printf("  GMAC / sec      : %8.3f\n", result->gmac_per_sec);
    printf("  GFLOP / sec     : %8.3f\n", result->gflops_per_sec);
    printf("  jitter span     : %8.2f %%\n", result->jitter_pct);
}

static void print_comparison(
    const benchmark_result_t *one_core,
    const benchmark_result_t *three_core
) {
    double speedup = one_core->avg_submit_us / three_core->avg_submit_us;
    double efficiency = speedup / 3.0 * 100.0;

    printf("comparison summary\n");
    printf("  1-core avg submit : %8.3f ms\n", one_core->avg_submit_ms);
    printf("  3-core avg submit : %8.3f ms\n", three_core->avg_submit_ms);
    printf("  speedup           : %8.3f x\n", speedup);
    printf("  parallel eff.     : %8.2f %% of ideal 3x\n", efficiency);
    printf("  throughput gain   : %8.3f x tasks/sec\n",
           three_core->tasks_per_sec / one_core->tasks_per_sec);
    printf("  GMAC gain         : %8.3f x\n",
           three_core->gmac_per_sec / one_core->gmac_per_sec);
}

static void print_usage(const char *argv0) {
    printf("Usage: %s [options]\n", argv0);
    printf("Options:\n");
    printf("  --scenario <name|all>   Run only one scenario (default: all)\n");
    printf("  --rounds <count>        Override measured rounds for every scenario\n");
    printf("  --warmup <count>        Override warmup rounds for every scenario\n");
    printf("  --task-cap <count>      Cap task count for both shared/unique modes\n");
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
    unsigned long parsed = strtoul(text, &end, 10);
    if (!text[0] || (end && *end != '\0')) {
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
        if ((strcmp(arg, "--scenario") == 0) ||
            (strcmp(arg, "--rounds") == 0) ||
            (strcmp(arg, "--warmup") == 0) ||
            (strcmp(arg, "--task-cap") == 0)) {
            if (index + 1 >= argc) {
                fprintf(stderr, "missing value for %s\n", arg);
                return -1;
            }
            index += 1;

            if (strcmp(arg, "--scenario") == 0) {
                options->scenario_filter = argv[index];
            } else if (strcmp(arg, "--rounds") == 0) {
                if (parse_u32(argv[index], &options->rounds_override) != 0) {
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
            total_rounds += rounds_per_case * 2;
        }

        if (options->run_unique &&
            maybe_cap_tasks(tasks_for_mode(scenario, OPERANDS_UNIQUE), options->task_cap) > 0) {
            total_rounds += rounds_per_case * 2;
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

    if (run_benchmark_case(
            fd,
            scenario,
            mode,
            1,
            task_count,
            warmup_rounds,
            measure_rounds,
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

    if (run_benchmark_case(
            fd,
            scenario,
            mode,
            3,
            task_count,
            warmup_rounds,
            measure_rounds,
            &three_core
        ) != 0) {
        fprintf(stderr, "3-core benchmark failed for scenario=%s mode=%s\n",
                scenario->name,
                operand_mode_name(mode));
        return -1;
    }

    printf("  3-core metrics\n");
    print_dma_footprint(&three_core);
    print_case_metrics(&three_core);
    print_comparison(&one_core, &three_core);
    printf("  mode complete   : %s\n", operand_mode_name(mode));
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
    printf("  comparison      : 1 core vs 3 cores on identical task batches\n");
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