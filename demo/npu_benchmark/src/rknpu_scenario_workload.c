/* RKNPU 多场景并发负载实现，最后修改日期：2026-08-12。 */

#include "rknpu_scenario_workload.h"

#include <errno.h>
#include <math.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/ioctl.h>
#include <sys/mman.h>
#include <time.h>

#include <libdrm/drm.h>

#include "npu_interface.h"
#include "npu_matmul.h"

#define REGCMD_WORDS 112U
#define REGCMD_BYTES (REGCMD_WORDS * sizeof(uint64_t))
#define INPUT_LAYOUT_C2 8
#define OUTPUT_LAYOUT_C2 4
#define DMA_SLICE_ALIGN 64U
#define SUBMIT_TIMEOUT_MS 6000U

_Static_assert(sizeof(struct rknpu_mem_create) == 48,
               "StarryOS MEM_CREATE ABI must be 48 bytes");

typedef struct {
    void *map;
    size_t size;
    uint64_t dma_addr;
    uint64_t obj_addr;
    uint32_t handle;
    int allocated;
} dma_buffer_t;

struct rknpu_scenario_workload {
    int fd;
    uint32_t thread_id;
    const rknpu_scenario_case_t *scenario;
    uint32_t input_slots;
    uint32_t weight_slots;
    size_t input_stride;
    size_t weight_stride;
    size_t output_stride;
    dma_buffer_t regcmd;
    dma_buffer_t tasks;
    dma_buffer_t input;
    dma_buffer_t weights;
    dma_buffer_t output;
    struct rknpu_submit submit_template;
    rknpu_scenario_metrics_t metrics;
};

/*
 * 四种矩阵形状展开为七个可运行组合。llama_decode_like 的 unique_tasks 为 0，
 * 因此只保留 shared；其余 Task 数量与 core_scaling_benchmark_fix.c 一致。
 */
static const rknpu_scenario_case_t k_cases[] = {
    {"tiny_dispatch-shared", 4U, 32U, 16U, 96U,
     RKNPU_OPERANDS_SHARED},
    {"tiny_dispatch-unique", 4U, 32U, 16U, 96U,
     RKNPU_OPERANDS_UNIQUE},
    {"mid_balanced-shared", 64U, 512U, 512U, 48U,
     RKNPU_OPERANDS_SHARED},
    {"mid_balanced-unique", 64U, 512U, 512U, 12U,
     RKNPU_OPERANDS_UNIQUE},
    {"throughput_heavy-shared", 128U, 1024U, 1024U, 24U,
     RKNPU_OPERANDS_SHARED},
    {"throughput_heavy-unique", 128U, 1024U, 1024U, 4U,
     RKNPU_OPERANDS_UNIQUE},
    {"llama_decode_like-shared", 1U, 4096U, 4096U, 48U,
     RKNPU_OPERANDS_SHARED},
};

const rknpu_scenario_case_t *rknpu_scenario_cases(size_t *count) {
    if (count != NULL) {
        *count = sizeof(k_cases) / sizeof(k_cases[0]);
    }
    return k_cases;
}

const rknpu_scenario_case_t *rknpu_scenario_find(const char *name) {
    size_t count;

    rknpu_scenario_cases(&count);
    for (size_t index = 0; index < count; index++) {
        if (strcmp(name, k_cases[index].name) == 0) {
            return &k_cases[index];
        }
    }
    return NULL;
}

const char *rknpu_operand_mode_name(rknpu_operand_mode_t mode) {
    return mode == RKNPU_OPERANDS_UNIQUE ? "unique" : "shared";
}

static size_t align_up_size(size_t value, size_t alignment) {
    return (value + alignment - 1U) / alignment * alignment;
}

/* 场景准备时间使用单调时钟，避免系统时间调整造成负间隔。 */
static uint64_t monotonic_time_us(void) {
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
    struct rknpu_mem_create create;
    struct rknpu_mem_map map;

    memset(buffer, 0, sizeof(*buffer));
    memset(&create, 0, sizeof(create));
    memset(&map, 0, sizeof(map));
    buffer->size = size;
    create.flags = flags | RKNPU_MEM_NON_CACHEABLE;
    create.size = size;
    create.core_mask = core_mask;

    if (ioctl(fd, DRM_IOCTL_RKNPU_MEM_CREATE, &create) < 0) {
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

static int dma_range_fits_u32(const dma_buffer_t *buffer) {
    return buffer->size > 0U &&
        buffer->dma_addr <= UINT32_MAX &&
        buffer->size - 1U <= UINT32_MAX - buffer->dma_addr;
}

static int input_value(int matrix_id, uint32_t row, uint32_t channel) {
    return ((matrix_id * 13 + (int)row * 7 + (int)channel * 5) % 11) - 5;
}

static int weight_value(int matrix_id, uint32_t column, uint32_t channel) {
    return ((matrix_id * 17 + (int)column * 3 + (int)channel * 9) % 11) - 5;
}

/* shared 在一个线程内复用同一矩阵；unique 为每个 Task 生成不同矩阵。 */
static int matrix_id_for_task(
    const rknpu_scenario_workload_t *workload,
    uint32_t task_index
) {
    if (workload->scenario->operand_mode == RKNPU_OPERANDS_SHARED) {
        return (int)workload->thread_id + 1;
    }
    return (int)(workload->thread_id * workload->scenario->task_count + task_index + 1U);
}

static void *slice_ptr(const dma_buffer_t *buffer, size_t stride, uint32_t index) {
    return (uint8_t *)buffer->map + stride * index;
}

static uint64_t slice_dma(const dma_buffer_t *buffer, size_t stride, uint32_t index) {
    return buffer->dma_addr + (uint64_t)stride * index;
}

static void prepare_operands(rknpu_scenario_workload_t *workload) {
    const rknpu_scenario_case_t *scenario = workload->scenario;

    memset(workload->input.map, 0, workload->input.size);
    memset(workload->weights.map, 0, workload->weights.size);
    memset(workload->output.map, 0, workload->output.size);

    for (uint32_t slot = 0; slot < workload->input_slots; slot++) {
        _Float16 *input = slice_ptr(&workload->input, workload->input_stride, slot);
        int matrix_id = matrix_id_for_task(workload, slot);

        for (uint32_t row = 0; row < scenario->m; row++) {
            for (uint32_t channel = 0; channel < scenario->k; channel++) {
                int offset = feature_data(
                    (int)scenario->k, (int)scenario->m, 1, INPUT_LAYOUT_C2,
                    (int)channel + 1, (int)row + 1, 1
                );
                input[offset] = (_Float16)input_value(matrix_id, row, channel);
            }
        }
    }

    for (uint32_t slot = 0; slot < workload->weight_slots; slot++) {
        _Float16 *weights = slice_ptr(&workload->weights, workload->weight_stride, slot);
        int matrix_id = matrix_id_for_task(workload, slot);

        for (uint32_t column = 0; column < scenario->n; column++) {
            for (uint32_t channel = 0; channel < scenario->k; channel++) {
                int offset = weight_fp16(
                    (int)scenario->k, (int)column + 1, (int)channel + 1
                );
                weights[offset] = (_Float16)weight_value(matrix_id, column, channel);
            }
        }
    }
}

static int prepare_tasks(rknpu_scenario_workload_t *workload) {
    const rknpu_scenario_case_t *scenario = workload->scenario;
    struct rknpu_task *tasks = workload->tasks.map;

    memset(workload->regcmd.map, 0, workload->regcmd.size);
    memset(tasks, 0, workload->tasks.size);

    for (uint32_t task_index = 0; task_index < scenario->task_count; task_index++) {
        uint32_t slot = scenario->operand_mode == RKNPU_OPERANDS_SHARED ? 0U : task_index;
        uint64_t regcmd_words[REGCMD_WORDS];
        matmul_params_t params;

        memset(regcmd_words, 0, sizeof(regcmd_words));
        memset(&params, 0, sizeof(params));
        params.m = (uint16_t)scenario->m;
        params.k = (uint16_t)scenario->k;
        params.n = (uint16_t)scenario->n;
        params.input_dma = (uint32_t)slice_dma(&workload->input, workload->input_stride, slot);
        params.weights_dma = (uint32_t)slice_dma(&workload->weights, workload->weight_stride, slot);
        params.output_dma =
            (uint32_t)slice_dma(&workload->output, workload->output_stride, task_index);
        params.tasks = regcmd_words;

        if (gen_matmul_fp16(&params) != 0) {
            fprintf(stderr, "gen_matmul_fp16 failed: scenario=%s task=%u\n",
                    scenario->name, task_index);
            return -1;
        }

        memcpy((uint8_t *)workload->regcmd.map + (size_t)task_index * REGCMD_BYTES,
               regcmd_words, sizeof(regcmd_words));
        tasks[task_index].op_idx =
            workload->thread_id * scenario->task_count + task_index;
        tasks[task_index].enable_mask = 0xd;
        tasks[task_index].int_mask = 0x300;
        tasks[task_index].int_clear = 0x1ffff;
        tasks[task_index].regcfg_amount =
            REGCMD_WORDS - (RKNPU_PC_DATA_EXTRA_AMOUNT + 4U);
        tasks[task_index].regcmd_addr =
            workload->regcmd.dma_addr + (uint64_t)task_index * REGCMD_BYTES;
    }
    return 0;
}

/* 按 Task 数量和当前核心数创建有效 lane，余数优先分给前面的 lane。 */
void rknpu_distribute_tasks_to_lanes(
    struct rknpu_submit *submit,
    uint32_t task_count,
    uint32_t npu_cores
) {
    uint32_t used_lanes = npu_cores < task_count ? npu_cores : task_count;
    uint32_t task_start = 0;

    memset(submit->subcore_task, 0, sizeof(submit->subcore_task));
    for (uint32_t lane = 0; lane < used_lanes; lane++) {
        uint32_t remaining_tasks = task_count - task_start;
        uint32_t remaining_lanes = used_lanes - lane;
        uint32_t lane_tasks =
            (remaining_tasks + remaining_lanes - 1U) / remaining_lanes;

        submit->subcore_task[lane].task_start = task_start;
        submit->subcore_task[lane].task_number = lane_tasks;
        task_start += lane_tasks;
    }
}

int rknpu_scenario_workload_create(
    int fd,
    uint32_t thread_id,
    const rknpu_scenario_case_t *scenario,
    uint32_t npu_cores,
    uint32_t core_mask,
    rknpu_scenario_workload_t **workload_out
) {
    rknpu_scenario_workload_t *workload;
    size_t input_bytes;
    size_t weight_bytes;
    size_t output_bytes;

    if (scenario == NULL || workload_out == NULL || npu_cores == 0U || npu_cores > 3U) {
        return -1;
    }
    *workload_out = NULL;
    workload = calloc(1, sizeof(*workload));
    if (workload == NULL) {
        return -1;
    }

    workload->fd = fd;
    workload->thread_id = thread_id;
    workload->scenario = scenario;
    workload->input_slots =
        scenario->operand_mode == RKNPU_OPERANDS_SHARED ? 1U : scenario->task_count;
    workload->weight_slots = workload->input_slots;
    workload->input_stride = align_up_size(
        (size_t)scenario->m * scenario->k * sizeof(_Float16), DMA_SLICE_ALIGN);
    workload->weight_stride = align_up_size(
        (size_t)scenario->n * scenario->k * sizeof(_Float16), DMA_SLICE_ALIGN);
    workload->output_stride = align_up_size(
        (size_t)scenario->m * scenario->n * sizeof(float), DMA_SLICE_ALIGN);
    input_bytes = workload->input_stride * workload->input_slots;
    weight_bytes = workload->weight_stride * workload->weight_slots;
    output_bytes = workload->output_stride * scenario->task_count;
    workload->metrics.total_dma_bytes =
        (size_t)scenario->task_count * REGCMD_BYTES +
        (size_t)scenario->task_count * sizeof(struct rknpu_task) +
        input_bytes + weight_bytes + output_bytes;

    if (allocate_dma_buffer(fd, (size_t)scenario->task_count * REGCMD_BYTES,
                            0, core_mask, &workload->regcmd) != 0 ||
        allocate_dma_buffer(fd,
                            (size_t)scenario->task_count * sizeof(struct rknpu_task),
                            RKNPU_MEM_KERNEL_MAPPING, core_mask, &workload->tasks) != 0 ||
        allocate_dma_buffer(fd, input_bytes, 0, core_mask, &workload->input) != 0 ||
        allocate_dma_buffer(fd, weight_bytes, 0, core_mask, &workload->weights) != 0 ||
        allocate_dma_buffer(fd, output_bytes, 0, core_mask, &workload->output) != 0) {
        rknpu_scenario_workload_destroy(workload);
        return -1;
    }

    if (!dma_range_fits_u32(&workload->input) ||
        !dma_range_fits_u32(&workload->weights) ||
        !dma_range_fits_u32(&workload->output)) {
        fprintf(stderr, "scenario=%s data DMA range exceeds 32-bit generator interface\n",
                scenario->name);
        rknpu_scenario_workload_destroy(workload);
        return -1;
    }

    {
        uint64_t start_us = monotonic_time_us();
        prepare_operands(workload);
        workload->metrics.setup_operands_us = monotonic_time_us() - start_us;
    }
    {
        uint64_t start_us = monotonic_time_us();
        int prepare_result = prepare_tasks(workload);
        workload->metrics.build_regcmds_us = monotonic_time_us() - start_us;
        if (prepare_result != 0) {
            rknpu_scenario_workload_destroy(workload);
            return -1;
        }
    }

    memset(&workload->submit_template, 0, sizeof(workload->submit_template));
    workload->submit_template.flags =
        RKNPU_JOB_PC | RKNPU_JOB_BLOCK | RKNPU_JOB_PINGPONG;
    workload->submit_template.timeout = SUBMIT_TIMEOUT_MS;
    workload->submit_template.task_number = scenario->task_count;
    workload->submit_template.task_obj_addr = workload->tasks.obj_addr;
    workload->submit_template.core_mask = core_mask;
    workload->submit_template.fence_fd = -1;
    rknpu_distribute_tasks_to_lanes(
        &workload->submit_template,
        scenario->task_count,
        npu_cores
    );

    *workload_out = workload;
    return 0;
}

void rknpu_scenario_workload_destroy(rknpu_scenario_workload_t *workload) {
    if (workload == NULL) {
        return;
    }
    release_dma_buffer(workload->fd, &workload->output);
    release_dma_buffer(workload->fd, &workload->weights);
    release_dma_buffer(workload->fd, &workload->input);
    release_dma_buffer(workload->fd, &workload->tasks);
    release_dma_buffer(workload->fd, &workload->regcmd);
    free(workload);
}

void rknpu_scenario_workload_begin(
    rknpu_scenario_workload_t *workload,
    struct rknpu_submit *submit
) {
    struct rknpu_task *tasks = workload->tasks.map;

    memset(workload->output.map, 0, workload->output.size);
    for (uint32_t index = 0; index < workload->scenario->task_count; index++) {
        tasks[index].int_status = 0;
    }
    *submit = workload->submit_template;
}

void rknpu_scenario_workload_next_submit(
    const rknpu_scenario_workload_t *workload,
    struct rknpu_submit *submit
) {
    *submit = workload->submit_template;
}

void rknpu_scenario_workload_get_metrics(
    const rknpu_scenario_workload_t *workload,
    rknpu_scenario_metrics_t *metrics
) {
    *metrics = workload->metrics;
}

static float expected_output(
    const rknpu_scenario_workload_t *workload,
    uint32_t task_index,
    uint32_t row,
    uint32_t column
) {
    float sum = 0.0f;
    int matrix_id = matrix_id_for_task(workload, task_index);

    for (uint32_t channel = 0; channel < workload->scenario->k; channel++) {
        sum += (float)input_value(matrix_id, row, channel) *
            (float)weight_value(matrix_id, column, channel);
    }
    return sum;
}

int rknpu_scenario_workload_check(
    const rknpu_scenario_workload_t *workload,
    const struct rknpu_submit *submit,
    rknpu_scenario_check_t *check
) {
    uint32_t sample_tasks[3];
    uint32_t task_samples = 0;
    uint32_t rows[2] = {0U, 0U};
    uint32_t columns[2] = {0U, 0U};
    uint32_t row_samples = 1U;
    uint32_t column_samples = 1U;
    const struct rknpu_task *tasks = workload->tasks.map;

    memset(check, 0, sizeof(*check));
    if (submit->task_counter != workload->scenario->task_count) {
        check->error = RKNPU_SCENARIO_CHECK_TASK_COUNTER;
        check->task_counter = submit->task_counter;
        return -1;
    }

    for (uint32_t index = 0; index < workload->scenario->task_count; index++) {
        if (tasks[index].int_status != 0x300U) {
            check->error = RKNPU_SCENARIO_CHECK_IRQ_STATUS;
            check->task_index = index;
            check->irq_status = tasks[index].int_status;
            return -1;
        }
    }

    sample_tasks[task_samples++] = 0U;
    if (workload->scenario->task_count > 2U) {
        sample_tasks[task_samples++] = workload->scenario->task_count / 2U;
    }
    if (workload->scenario->task_count > 1U) {
        sample_tasks[task_samples++] = workload->scenario->task_count - 1U;
    }
    if (workload->scenario->m > 1U) {
        rows[row_samples++] = workload->scenario->m > 3U ? 3U : workload->scenario->m - 1U;
    }
    if (workload->scenario->n > 1U) {
        columns[column_samples++] =
            workload->scenario->n > 3U ? 3U : workload->scenario->n - 1U;
    }

    for (uint32_t task_sample = 0; task_sample < task_samples; task_sample++) {
        uint32_t task_index = sample_tasks[task_sample];
        const float *output =
            slice_ptr(&workload->output, workload->output_stride, task_index);

        for (uint32_t row_index = 0; row_index < row_samples; row_index++) {
            for (uint32_t column_index = 0; column_index < column_samples; column_index++) {
                uint32_t row = rows[row_index];
                uint32_t column = columns[column_index];
                int offset = feature_data(
                    (int)workload->scenario->n, (int)workload->scenario->m, 1,
                    OUTPUT_LAYOUT_C2, (int)column + 1, (int)row + 1, 1
                );
                float expected = expected_output(workload, task_index, row, column);
                float actual = output[offset];

                if (fabsf(actual - expected) > 0.001f) {
                    check->error = RKNPU_SCENARIO_CHECK_OUTPUT;
                    check->task_index = task_index;
                    check->row = row;
                    check->column = column;
                    check->expected = expected;
                    check->actual = actual;
                    return -1;
                }
            }
        }
    }
    return 0;
}
