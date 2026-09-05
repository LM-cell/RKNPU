/* RKNPU 48 Task 混合负载实现，最后修改日期：2026-08-18。 */

#include "rknpu_mixed_workload.h"

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
_Static_assert(sizeof(struct rknpu_task) == 40,
               "RKNPU task ABI must be 40 bytes");

typedef struct {
    void *map;
    size_t size;
    uint64_t dma_addr;
    uint64_t obj_addr;
    uint32_t handle;
    int allocated;
} dma_buffer_t;

typedef struct {
    const char *name;
    uint32_t m;
    uint32_t k;
    uint32_t n;
} mixed_shape_t;

/* 每种形状共享一份只读输入和权重，并拥有 12 个独立输出切片。 */
typedef struct {
    dma_buffer_t input;
    dma_buffer_t weights;
    dma_buffer_t output;
    size_t output_stride;
} mixed_type_buffers_t;

struct rknpu_mixed_workload {
    int fd;
    uint32_t thread_id;
    rknpu_mixed_layout_t layout;
    dma_buffer_t regcmd;
    dma_buffer_t tasks;
    mixed_type_buffers_t type_buffers[RKNPU_MIXED_TYPE_COUNT];
    struct rknpu_submit submit_template;
    rknpu_mixed_metrics_t metrics;
};

static const mixed_shape_t k_shapes[RKNPU_MIXED_TYPE_COUNT] = {
    {"tiny_dispatch-shared", 4U, 32U, 16U},
    {"mid_balanced-shared", 64U, 512U, 512U},
    {"throughput_heavy-shared", 128U, 1024U, 1024U},
    {"llama_decode_like-shared", 1U, 4096U, 4096U},
};

const char *rknpu_mixed_layout_name(rknpu_mixed_layout_t layout) {
    return layout == RKNPU_MIXED_INTERLEAVED ? "interleaved" : "clustered";
}

const char *rknpu_mixed_type_name(rknpu_mixed_task_type_t type) {
    if ((uint32_t)type >= RKNPU_MIXED_TYPE_COUNT) {
        return "invalid";
    }
    return k_shapes[type].name;
}

rknpu_mixed_task_type_t rknpu_mixed_task_type(
    rknpu_mixed_layout_t layout,
    uint32_t task_index
) {
    if (layout == RKNPU_MIXED_INTERLEAVED) {
        return (rknpu_mixed_task_type_t)(task_index % RKNPU_MIXED_TYPE_COUNT);
    }
    return (rknpu_mixed_task_type_t)(task_index / RKNPU_MIXED_TASKS_PER_TYPE);
}

/* 返回同类型 Task 在其输出缓冲区中的 0～11 序号。 */
static uint32_t task_type_ordinal(rknpu_mixed_layout_t layout, uint32_t task_index) {
    if (layout == RKNPU_MIXED_INTERLEAVED) {
        return task_index / RKNPU_MIXED_TYPE_COUNT;
    }
    return task_index % RKNPU_MIXED_TASKS_PER_TYPE;
}

static size_t align_up_size(size_t value, size_t alignment) {
    return (value + alignment - 1U) / alignment * alignment;
}

static uint64_t monotonic_time_us(void) {
    struct timespec ts;

    clock_gettime(CLOCK_MONOTONIC, &ts);
    return (uint64_t)ts.tv_sec * 1000000ULL + (uint64_t)ts.tv_nsec / 1000ULL;
}

/* 分配失败后仍保留 handle，保证统一清理能销毁已创建的 GEM。 */
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
    return buffer->size > 0U && buffer->dma_addr <= UINT32_MAX &&
        buffer->size - 1U <= UINT32_MAX - buffer->dma_addr;
}

static int input_value(int matrix_id, uint32_t row, uint32_t channel) {
    return ((matrix_id * 13 + (int)row * 7 + (int)channel * 5) % 11) - 5;
}

static int weight_value(int matrix_id, uint32_t column, uint32_t channel) {
    return ((matrix_id * 17 + (int)column * 3 + (int)channel * 9) % 11) - 5;
}

/* 每个线程、每种形状使用不同矩阵，线程之间不共享可写 GEM。 */
static int matrix_id(const rknpu_mixed_workload_t *workload, uint32_t type) {
    return (int)(workload->thread_id * RKNPU_MIXED_TYPE_COUNT + type + 1U);
}

static void prepare_operands(rknpu_mixed_workload_t *workload) {
    for (uint32_t type = 0; type < RKNPU_MIXED_TYPE_COUNT; type++) {
        const mixed_shape_t *shape = &k_shapes[type];
        mixed_type_buffers_t *buffers = &workload->type_buffers[type];
        _Float16 *input = buffers->input.map;
        _Float16 *weights = buffers->weights.map;
        int id = matrix_id(workload, type);

        memset(input, 0, buffers->input.size);
        memset(weights, 0, buffers->weights.size);
        memset(buffers->output.map, 0, buffers->output.size);

        for (uint32_t row = 0; row < shape->m; row++) {
            for (uint32_t channel = 0; channel < shape->k; channel++) {
                int offset = feature_data(
                    (int)shape->k, (int)shape->m, 1, INPUT_LAYOUT_C2,
                    (int)channel + 1, (int)row + 1, 1
                );
                input[offset] = (_Float16)input_value(id, row, channel);
            }
        }
        for (uint32_t column = 0; column < shape->n; column++) {
            for (uint32_t channel = 0; channel < shape->k; channel++) {
                int offset = weight_fp16(
                    (int)shape->k, (int)column + 1, (int)channel + 1
                );
                weights[offset] = (_Float16)weight_value(id, column, channel);
            }
        }
    }
}

/* 固定 48 Task 按当前核心数均分为连续 Lane，便于静态/动态使用同一输入。 */
static void configure_lanes(struct rknpu_submit *submit, uint32_t npu_cores) {
    uint32_t task_start = 0;

    memset(submit->subcore_task, 0, sizeof(submit->subcore_task));
    for (uint32_t lane = 0; lane < npu_cores; lane++) {
        uint32_t remaining_tasks = RKNPU_MIXED_TASK_COUNT - task_start;
        uint32_t remaining_lanes = npu_cores - lane;
        uint32_t lane_tasks =
            (remaining_tasks + remaining_lanes - 1U) / remaining_lanes;

        submit->subcore_task[lane].task_start = task_start;
        submit->subcore_task[lane].task_number = lane_tasks;
        task_start += lane_tasks;
    }
}

/* 为每个 Task 生成独立 regcmd，并把输出定向到其类型内的独立切片。 */
static int prepare_tasks(
    rknpu_mixed_workload_t *workload,
    uint32_t npu_cores,
    uint32_t core_mask,
    int dynamic_tasks
) {
    struct rknpu_task *tasks = workload->tasks.map;

    memset(workload->regcmd.map, 0, workload->regcmd.size);
    memset(tasks, 0, workload->tasks.size);
    memset(&workload->submit_template, 0, sizeof(workload->submit_template));

    for (uint32_t task_index = 0; task_index < RKNPU_MIXED_TASK_COUNT; task_index++) {
        rknpu_mixed_task_type_t type =
            rknpu_mixed_task_type(workload->layout, task_index);
        uint32_t ordinal = task_type_ordinal(workload->layout, task_index);
        const mixed_shape_t *shape = &k_shapes[type];
        mixed_type_buffers_t *buffers = &workload->type_buffers[type];
        uint64_t regcmd_words[REGCMD_WORDS];
        matmul_params_t params;

        memset(regcmd_words, 0, sizeof(regcmd_words));
        memset(&params, 0, sizeof(params));
        params.m = (uint16_t)shape->m;
        params.k = (uint16_t)shape->k;
        params.n = (uint16_t)shape->n;
        params.input_dma = (uint32_t)buffers->input.dma_addr;
        params.weights_dma = (uint32_t)buffers->weights.dma_addr;
        params.output_dma = (uint32_t)(
            buffers->output.dma_addr + (uint64_t)ordinal * buffers->output_stride
        );
        params.tasks = regcmd_words;
        if (gen_matmul_fp16(&params) != 0) {
            fprintf(stderr, "gen_matmul_fp16 failed: type=%s task=%u\n",
                    shape->name, task_index);
            return -1;
        }

        memcpy((uint8_t *)workload->regcmd.map + (size_t)task_index * REGCMD_BYTES,
               regcmd_words, sizeof(regcmd_words));
        tasks[task_index].op_idx =
            workload->thread_id * RKNPU_MIXED_TASK_COUNT + task_index;
        tasks[task_index].enable_mask = 0xd;
        tasks[task_index].int_mask = 0x300;
        tasks[task_index].int_clear = 0x1ffff;
        tasks[task_index].regcfg_amount =
            REGCMD_WORDS - (RKNPU_PC_DATA_EXTRA_AMOUNT + 4U);
        tasks[task_index].regcmd_addr =
            workload->regcmd.dma_addr + (uint64_t)task_index * REGCMD_BYTES;
    }

    workload->submit_template.flags =
        RKNPU_JOB_PC | RKNPU_JOB_BLOCK | RKNPU_JOB_PINGPONG;
    if (dynamic_tasks) {
        workload->submit_template.flags |= RKNPU_JOB_DYNAMIC_TASKS;
    }
    workload->submit_template.timeout = SUBMIT_TIMEOUT_MS;
    workload->submit_template.task_number = RKNPU_MIXED_TASK_COUNT;
    workload->submit_template.task_obj_addr = workload->tasks.obj_addr;
    /*
     * 当前 StarryOS 调度器直接把内核 Task 副本交给单 Task 硬件派发路径。
     * 最后修改日期：2026-08-18。这里沿用现有 demo 的零地址约定；传入
     * Task GEM 的 DMA 地址会让 Submit 正常完成，但 NPU 不写回计算结果。
     */
    workload->submit_template.task_base_addr = 0;
    workload->submit_template.core_mask = core_mask;
    workload->submit_template.fence_fd = -1;
    configure_lanes(&workload->submit_template, npu_cores);
    return 0;
}

int rknpu_mixed_workload_create(
    int fd,
    uint32_t thread_id,
    uint32_t npu_cores,
    uint32_t core_mask,
    rknpu_mixed_layout_t layout,
    int dynamic_tasks,
    rknpu_mixed_workload_t **workload_out
) {
    rknpu_mixed_workload_t *workload;

    if (workload_out == NULL || npu_cores == 0U || npu_cores > 3U ||
        (layout != RKNPU_MIXED_CLUSTERED && layout != RKNPU_MIXED_INTERLEAVED)) {
        return -1;
    }
    *workload_out = NULL;
    workload = calloc(1, sizeof(*workload));
    if (workload == NULL) {
        return -1;
    }
    workload->fd = fd;
    workload->thread_id = thread_id;
    workload->layout = layout;

    if (allocate_dma_buffer(fd, RKNPU_MIXED_TASK_COUNT * REGCMD_BYTES,
                            0, core_mask, &workload->regcmd) != 0 ||
        allocate_dma_buffer(fd,
                            RKNPU_MIXED_TASK_COUNT * sizeof(struct rknpu_task),
                            RKNPU_MEM_KERNEL_MAPPING, core_mask,
                            &workload->tasks) != 0) {
        rknpu_mixed_workload_destroy(workload);
        return -1;
    }
    workload->metrics.total_dma_bytes = workload->regcmd.size + workload->tasks.size;

    for (uint32_t type = 0; type < RKNPU_MIXED_TYPE_COUNT; type++) {
        const mixed_shape_t *shape = &k_shapes[type];
        mixed_type_buffers_t *buffers = &workload->type_buffers[type];
        size_t input_bytes = (size_t)shape->m * shape->k * sizeof(_Float16);
        size_t weight_bytes = (size_t)shape->n * shape->k * sizeof(_Float16);

        buffers->output_stride = align_up_size(
            (size_t)shape->m * shape->n * sizeof(float), DMA_SLICE_ALIGN
        );
        if (allocate_dma_buffer(fd, input_bytes, 0, core_mask, &buffers->input) != 0 ||
            allocate_dma_buffer(fd, weight_bytes, 0, core_mask, &buffers->weights) != 0 ||
            allocate_dma_buffer(fd,
                                buffers->output_stride * RKNPU_MIXED_TASKS_PER_TYPE,
                                0, core_mask, &buffers->output) != 0) {
            rknpu_mixed_workload_destroy(workload);
            return -1;
        }
        if (!dma_range_fits_u32(&buffers->input) ||
            !dma_range_fits_u32(&buffers->weights) ||
            !dma_range_fits_u32(&buffers->output)) {
            fprintf(stderr, "mixed type=%s DMA range exceeds 32-bit generator interface\n",
                    shape->name);
            rknpu_mixed_workload_destroy(workload);
            return -1;
        }
        workload->metrics.total_dma_bytes +=
            buffers->input.size + buffers->weights.size + buffers->output.size;
    }

    {
        uint64_t start_us = monotonic_time_us();
        prepare_operands(workload);
        workload->metrics.setup_operands_us = monotonic_time_us() - start_us;
    }
    {
        uint64_t start_us = monotonic_time_us();
        int result = prepare_tasks(
            workload, npu_cores, core_mask, dynamic_tasks
        );
        workload->metrics.build_regcmds_us = monotonic_time_us() - start_us;
        if (result != 0) {
            rknpu_mixed_workload_destroy(workload);
            return -1;
        }
    }

    *workload_out = workload;
    return 0;
}

void rknpu_mixed_workload_destroy(rknpu_mixed_workload_t *workload) {
    if (workload == NULL) {
        return;
    }
    for (uint32_t type = RKNPU_MIXED_TYPE_COUNT; type > 0; type--) {
        mixed_type_buffers_t *buffers = &workload->type_buffers[type - 1U];
        release_dma_buffer(workload->fd, &buffers->output);
        release_dma_buffer(workload->fd, &buffers->weights);
        release_dma_buffer(workload->fd, &buffers->input);
    }
    release_dma_buffer(workload->fd, &workload->tasks);
    release_dma_buffer(workload->fd, &workload->regcmd);
    free(workload);
}

void rknpu_mixed_workload_begin(
    rknpu_mixed_workload_t *workload,
    struct rknpu_submit *submit
) {
    struct rknpu_task *tasks = workload->tasks.map;

    for (uint32_t type = 0; type < RKNPU_MIXED_TYPE_COUNT; type++) {
        memset(workload->type_buffers[type].output.map, 0,
               workload->type_buffers[type].output.size);
    }
    for (uint32_t task = 0; task < RKNPU_MIXED_TASK_COUNT; task++) {
        tasks[task].int_status = 0;
    }
    *submit = workload->submit_template;
}

void rknpu_mixed_workload_next_submit(
    const rknpu_mixed_workload_t *workload,
    struct rknpu_submit *submit
) {
    *submit = workload->submit_template;
}

void rknpu_mixed_workload_get_metrics(
    const rknpu_mixed_workload_t *workload,
    rknpu_mixed_metrics_t *metrics
) {
    *metrics = workload->metrics;
}

static float expected_output(
    const rknpu_mixed_workload_t *workload,
    uint32_t type,
    uint32_t row,
    uint32_t column
) {
    const mixed_shape_t *shape = &k_shapes[type];
    int id = matrix_id(workload, type);
    float sum = 0.0f;

    for (uint32_t channel = 0; channel < shape->k; channel++) {
        sum += (float)input_value(id, row, channel) *
            (float)weight_value(id, column, channel);
    }
    return sum;
}

int rknpu_mixed_workload_check(
    const rknpu_mixed_workload_t *workload,
    const struct rknpu_submit *submit,
    rknpu_mixed_check_t *check
) {
    const struct rknpu_task *tasks = workload->tasks.map;

    memset(check, 0, sizeof(*check));
    if (submit->task_counter != RKNPU_MIXED_TASK_COUNT) {
        check->error = RKNPU_MIXED_CHECK_TASK_COUNTER;
        check->task_counter = submit->task_counter;
        return -1;
    }

    for (uint32_t task_index = 0; task_index < RKNPU_MIXED_TASK_COUNT; task_index++) {
        rknpu_mixed_task_type_t type =
            rknpu_mixed_task_type(workload->layout, task_index);
        uint32_t ordinal = task_type_ordinal(workload->layout, task_index);
        const mixed_shape_t *shape = &k_shapes[type];
        const mixed_type_buffers_t *buffers = &workload->type_buffers[type];
        const float *output = (const float *)(
            (const uint8_t *)buffers->output.map +
            (size_t)ordinal * buffers->output_stride
        );
        int offset;

        if (tasks[task_index].int_status != 0x300U) {
            check->error = RKNPU_MIXED_CHECK_IRQ_STATUS;
            check->task_index = task_index;
            check->irq_status = tasks[task_index].int_status;
            return -1;
        }

        /* 每个 Task 检查左上角元素，确保 48 个输出切片均被实际写入。 */
        offset = feature_data(
            (int)shape->n, (int)shape->m, 1, OUTPUT_LAYOUT_C2, 1, 1, 1
        );
        check->expected = expected_output(workload, type, 0, 0);
        check->actual = output[offset];
        if (fabsf(check->actual - check->expected) > 0.001f) {
            check->error = RKNPU_MIXED_CHECK_OUTPUT;
            check->task_index = task_index;
            check->row = 0;
            check->column = 0;
            return -1;
        }
    }

    check->error = RKNPU_MIXED_CHECK_OK;
    return 0;
}
