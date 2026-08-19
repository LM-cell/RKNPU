/* 10 Task 矩阵乘法测试负载实现，最后修改日期：2026-08-07。 */

#include "rknpu_batch_workload.h"

#include <errno.h>
#include <math.h>
#include <stddef.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/ioctl.h>
#include <sys/mman.h>

#include <libdrm/drm.h>

#include "npu_interface.h"
#include "npu_matmul.h"

#define REGCMD_WORDS 112U
#define REGCMD_BYTES (REGCMD_WORDS * sizeof(uint64_t))
#define INPUT_LAYOUT_C2 8
#define OUTPUT_LAYOUT_C2 4
#define SUBMIT_TIMEOUT_MS 6000U

/*
 * StarryOS 的 MEM_CREATE 为 48 字节，比 demo 原上游结构多 iommu_domain_id
 * 和 core_mask。局部兼容结构保证 ioctl copy-out 不会越过用户对象末尾。
 */
struct rknpu_mem_create_starry {
    /* 驱动返回的 GEM handle。 */
    uint32_t handle;
    /* 缓存和映射属性。 */
    uint32_t flags;
    /* 用户请求的分配字节数。 */
    uint64_t size;
    /* 驱动返回的 CPU 对象地址。 */
    uint64_t obj_addr;
    /* 驱动返回的 NPU DMA 地址。 */
    uint64_t dma_addr;
    /* 驱动实际分配的字节数。 */
    uint64_t sram_size;
    /* 当前测试保持默认 0，不切换 IOMMU domain。 */
    int32_t iommu_domain_id;
    /* 告知驱动该缓冲区对应的实验核心掩码。 */
    uint32_t core_mask;
};

#define DRM_IOCTL_RKNPU_MEM_CREATE_STARRY                                    \
    DRM_IOWR(DRM_COMMAND_BASE + RKNPU_MEM_CREATE, struct rknpu_mem_create_starry)

_Static_assert(sizeof(struct rknpu_mem_create_starry) == 48,
               "StarryOS MEM_CREATE ABI must be 48 bytes");

typedef struct {
    /* 用户态 mmap 地址；MAP_FAILED 表示映射没有建立。 */
    void *map;
    /* 创建和解除映射时使用的完整字节数。 */
    size_t size;
    /* NPU 访问该 GEM 对象使用的 DMA 地址。 */
    uint64_t dma_addr;
    /* StarryOS MEM_CREATE 返回的 CPU 对象地址，销毁 ABI 需要原样带回。 */
    uint64_t obj_addr;
    /* 驱动 GEM 池中的对象句柄。 */
    uint32_t handle;
    /* MEM_CREATE 成功后置 1，保证 mmap 失败时仍会销毁 GEM。 */
    int allocated;
} dma_buffer_t;

/* 一个线程独占一组负载资源；线程之间只共享 DRM fd，不共享可写缓冲区。 */
struct rknpu_batch_workload {
    /* 用于生成互不相同的输入数据和 op_idx。 */
    uint32_t thread_id;
    /* 10 段寄存器命令，每个 Task 使用独立地址。 */
    dma_buffer_t regcmd;
    /* 10 个硬件 Task 描述符。 */
    dma_buffer_t tasks;
    /* 10 个 Task 只读共享的输入矩阵。 */
    dma_buffer_t input;
    /* 10 个 Task 只读共享的权重矩阵。 */
    dma_buffer_t weights;
    /* 10 个互不重叠的输出矩阵，避免并行 Task 相互覆盖。 */
    dma_buffer_t output;
    /* 指向上述 Task 数组的 blocking Submit 模板。 */
    struct rknpu_submit submit;
};

/*
 * 创建并映射一个 DMA 缓冲区。
 *
 * MEM_CREATE 成功后立即记录 handle 和 allocated，即使后续 MEM_MAP 或 mmap
 * 失败，外层统一清理仍能销毁驱动对象，不会留下半初始化 GEM。
 */
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

/* 按“解除用户映射，再销毁驱动对象”的顺序释放一个 DMA 缓冲区。 */
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

/* 生成可重复、数值范围较小的输入元素，避免 FP16 输入溢出。 */
static int input_value(uint32_t matrix_id, uint32_t row, uint32_t channel) {
    return ((int)(matrix_id * 13U + row * 7U + channel * 5U) % 11) - 5;
}

/* 生成与输入模式不同的确定性权重元素。 */
static int weight_value(uint32_t matrix_id, uint32_t column, uint32_t channel) {
    return ((int)(matrix_id * 17U + column * 3U + channel * 9U) % 11) - 5;
}

/*
 * 根据 npu_matmul 使用的硬件布局填充输入和权重。
 * thread_id 参与 matrix_id，能够发现共享 fd 下不同线程错误复用缓冲区。
 */
static void prepare_operands(rknpu_batch_workload_t *workload) {
    _Float16 *input = workload->input.map;
    _Float16 *weights = workload->weights.map;
    uint32_t matrix_id = workload->thread_id + 1U;

    memset(input, 0, workload->input.size);
    memset(weights, 0, workload->weights.size);

    for (uint32_t row = 0; row < RKNPU_BATCH_M; row++) {
        for (uint32_t channel = 0; channel < RKNPU_BATCH_K; channel++) {
            int offset = feature_data(
                RKNPU_BATCH_K,
                RKNPU_BATCH_M,
                1,
                INPUT_LAYOUT_C2,
                channel + 1U,
                row + 1U,
                1
            );
            input[offset] = (_Float16)input_value(matrix_id, row, channel);
        }
    }

    for (uint32_t column = 0; column < RKNPU_BATCH_N; column++) {
        for (uint32_t channel = 0; channel < RKNPU_BATCH_K; channel++) {
            int offset = weight_fp16(RKNPU_BATCH_K, column + 1U, channel + 1U);
            weights[offset] = (_Float16)weight_value(matrix_id, column, channel);
        }
    }
}

/*
 * 把固定 10 Task 分配到逻辑 lane：1 核为 10，2 核为 5+5，3 核为 4+3+3。
 * lane 只描述 Submit 内连续 Task 范围；实际物理核心仍由驱动结合 core_mask
 * 选择，测试通过调度 trace 检查最终 core_slot。
 */
static void configure_lanes(struct rknpu_submit *submit, uint32_t npu_cores) {
    static const uint32_t lane_counts[3][3] = {
        {10U, 0U, 0U},
        {5U, 5U, 0U},
        {4U, 3U, 3U},
    };
    uint32_t task_start = 0;

    for (uint32_t lane = 0; lane < npu_cores; lane++) {
        submit->subcore_task[lane].task_start = task_start;
        submit->subcore_task[lane].task_number = lane_counts[npu_cores - 1U][lane];
        task_start += lane_counts[npu_cores - 1U][lane];
    }
}

/*
 * 为十个 Task 分别生成寄存器命令和描述符，并构造共享 Submit 模板。
 * 输入和权重只读共享，regcmd 与输出按 task_index 分段，支持多核同时执行。
 */
static int prepare_tasks(
    rknpu_batch_workload_t *workload,
    uint32_t npu_cores,
    uint32_t core_mask
) {
    struct rknpu_task *tasks = workload->tasks.map;
    size_t output_stride = RKNPU_BATCH_M * RKNPU_BATCH_N * sizeof(float);

    memset(workload->regcmd.map, 0, workload->regcmd.size);
    memset(tasks, 0, workload->tasks.size);
    memset(&workload->submit, 0, sizeof(workload->submit));

    for (uint32_t task_index = 0; task_index < RKNPU_BATCH_TASKS; task_index++) {
        uint64_t regcmd_words[REGCMD_WORDS];
        matmul_params_t params;

        memset(regcmd_words, 0, sizeof(regcmd_words));
        memset(&params, 0, sizeof(params));
        params.m = RKNPU_BATCH_M;
        params.k = RKNPU_BATCH_K;
        params.n = RKNPU_BATCH_N;
        params.input_dma = (uint32_t)workload->input.dma_addr;
        params.weights_dma = (uint32_t)workload->weights.dma_addr;
        params.output_dma = (uint32_t)(workload->output.dma_addr +
                                       task_index * output_stride);
        params.tasks = regcmd_words;
        params.fp32tofp16 = 0;
        if (gen_matmul_fp16(&params) != 0) {
            return -1;
        }

        memcpy((uint8_t *)workload->regcmd.map + task_index * REGCMD_BYTES,
               regcmd_words, sizeof(regcmd_words));
        /* op_idx 的商表示线程，余数表示 Task 下标，便于 trace 交叉校验。 */
        tasks[task_index].op_idx = workload->thread_id * RKNPU_BATCH_TASKS + task_index;
        tasks[task_index].enable_mask = 0xd;
        tasks[task_index].int_mask = 0x300;
        tasks[task_index].int_clear = 0x1ffff;
        tasks[task_index].regcfg_amount =
            REGCMD_WORDS - (RKNPU_PC_DATA_EXTRA_AMOUNT + 4U);
        tasks[task_index].regcmd_addr =
            workload->regcmd.dma_addr + task_index * REGCMD_BYTES;
    }

    workload->submit.flags =
        RKNPU_JOB_PC | RKNPU_JOB_BLOCK | RKNPU_JOB_PINGPONG;
    
    workload->submit.timeout = SUBMIT_TIMEOUT_MS;
    
    workload->submit.task_start = 0;
    workload->submit.task_number = RKNPU_BATCH_TASKS;
    workload->submit.task_counter = 0;
    
    /*
     * 当前 StarryOS queue-driven RKNPU 路径通过 task_obj_addr
     * 获取用户态 Task 对象并构造 shadow task。
     *
     * 与现有可工作的多 Task benchmark 保持一致，
     * task_base_addr 必须保持为 0。
     */
    workload->submit.task_obj_addr = workload->tasks.obj_addr;
    workload->submit.regcfg_obj_addr = 0;
    workload->submit.task_base_addr = 0;
    
    workload->submit.user_data = 0;
    workload->submit.core_mask = core_mask;
    workload->submit.fence_fd = -1;
    
    configure_lanes(&workload->submit, npu_cores);
    return 0;
}

/*
 * 创建一个线程的完整负载资源。
 *
 * 五个分配按依赖顺序完成；任一步失败都调用 rknpu_batch_destroy 回收已成功
 * 部分。矩阵生成器只接受 32 位数据地址，因此还要验证整个输入、权重和
 * 输出范围都落在 UINT32_MAX 内，而不是只检查起始地址。
 */
int rknpu_batch_create(
    int fd,
    uint32_t thread_id,
    uint32_t npu_cores,
    uint32_t core_mask,
    rknpu_batch_workload_t **workload_out
) {
    rknpu_batch_workload_t *workload;
    size_t output_bytes =
        RKNPU_BATCH_TASKS * RKNPU_BATCH_M * RKNPU_BATCH_N * sizeof(float);

    if (workload_out == NULL || npu_cores == 0U || npu_cores > 3U) {
        return -1;
    }
    *workload_out = NULL;
    workload = calloc(1, sizeof(*workload));
    if (workload == NULL) {
        return -1;
    }
    workload->thread_id = thread_id;

    if (allocate_dma_buffer(fd, RKNPU_BATCH_TASKS * REGCMD_BYTES, 0, core_mask,
                            &workload->regcmd) != 0 ||
        allocate_dma_buffer(fd, RKNPU_BATCH_TASKS * sizeof(struct rknpu_task),
                            RKNPU_MEM_KERNEL_MAPPING, core_mask,
                            &workload->tasks) != 0 ||
        allocate_dma_buffer(fd, RKNPU_BATCH_M * RKNPU_BATCH_K * sizeof(_Float16),
                            0, core_mask, &workload->input) != 0 ||
        allocate_dma_buffer(fd, RKNPU_BATCH_N * RKNPU_BATCH_K * sizeof(_Float16),
                            0, core_mask, &workload->weights) != 0 ||
        allocate_dma_buffer(fd, output_bytes, 0, core_mask, &workload->output) != 0) {
        rknpu_batch_destroy(fd, workload);
        return -1;
    }

    if (workload->input.dma_addr > UINT32_MAX - (workload->input.size - 1U) ||
        workload->weights.dma_addr > UINT32_MAX - (workload->weights.size - 1U) ||
        workload->output.dma_addr > UINT32_MAX - (output_bytes - 1U)) {
        fprintf(stderr, "matmul data DMA address exceeds 32-bit generator range\n");
        rknpu_batch_destroy(fd, workload);
        return -1;
    }

    prepare_operands(workload);
    if (prepare_tasks(workload, npu_cores, core_mask) != 0) {
        rknpu_batch_destroy(fd, workload);
        return -1;
    }
    *workload_out = workload;
    return 0;
}

/* 销毁顺序与创建顺序相反；各子缓冲区允许处于未创建的零状态。 */
void rknpu_batch_destroy(int fd, rknpu_batch_workload_t *workload) {
    if (workload == NULL) {
        return;
    }
    release_dma_buffer(fd, &workload->output);
    release_dma_buffer(fd, &workload->weights);
    release_dma_buffer(fd, &workload->input);
    release_dma_buffer(fd, &workload->tasks);
    release_dma_buffer(fd, &workload->regcmd);
    free(workload);
}

/*
 * 为下一轮复用已生成的任务，只清理可变完成状态和输出。
 * 输入、权重、regcmd 和 Task 地址保持不变，避免把分配时间计入 Submit 延迟。
 */
struct rknpu_submit *rknpu_batch_begin(
    rknpu_batch_workload_t *workload,
    int32_t priority
) {
    struct rknpu_task *tasks = workload->tasks.map;

    memset(workload->output.map, 0, workload->output.size);
    for (uint32_t task_index = 0; task_index < RKNPU_BATCH_TASKS; task_index++) {
        tasks[task_index].int_status = 0;
    }
    workload->submit.task_counter = 0;
    workload->submit.priority = priority;
    return &workload->submit;
}

/* 使用与填充阶段相同的确定性公式，在 CPU 上计算一个参考输出元素。 */
static float expected_output(
    uint32_t matrix_id,
    uint32_t row,
    uint32_t column
) {
    float sum = 0.0f;

    for (uint32_t channel = 0; channel < RKNPU_BATCH_K; channel++) {
        sum += (float)input_value(matrix_id, row, channel) *
               (float)weight_value(matrix_id, column, channel);
    }
    return sum;
}

/*
 * 校验一次 Submit 的完整性和计算正确性。
 *
 * 先检查 Submit 级 task_counter，再检查十个 Task 的 IRQ，最后每个 Task
 * 抽查四个输出位置。按这个顺序可以优先报告调度/完成错误，避免把未完成
 * Task 的零输出误报成矩阵计算错误。
 */
int rknpu_batch_check(
    const rknpu_batch_workload_t *workload,
    const struct rknpu_submit *submit,
    rknpu_batch_check_t *check
) {
    static const uint32_t rows[] = {0U, 3U};
    static const uint32_t columns[] = {0U, 3U};
    const struct rknpu_task *tasks = workload->tasks.map;
    const float *output = workload->output.map;
    size_t output_elements = RKNPU_BATCH_M * RKNPU_BATCH_N;
    uint32_t matrix_id = workload->thread_id + 1U;

    memset(check, 0, sizeof(*check));
    if (submit->task_counter != RKNPU_BATCH_TASKS) {
        check->error = RKNPU_BATCH_TASK_COUNTER_ERROR;
        check->actual_u32 = submit->task_counter;
        return -1;
    }

    for (uint32_t task_index = 0; task_index < RKNPU_BATCH_TASKS; task_index++) {
        if (tasks[task_index].int_status != 0x300U) {
            check->error = RKNPU_BATCH_IRQ_STATUS_ERROR;
            check->task_index = task_index;
            check->actual_u32 = tasks[task_index].int_status;
            return -1;
        }
        for (size_t row_index = 0; row_index < sizeof(rows) / sizeof(rows[0]); row_index++) {
            for (size_t column_index = 0;
                 column_index < sizeof(columns) / sizeof(columns[0]);
                 column_index++) {
                uint32_t row = rows[row_index];
                uint32_t column = columns[column_index];
                int offset = feature_data(
                    RKNPU_BATCH_N,
                    RKNPU_BATCH_M,
                    1,
                    OUTPUT_LAYOUT_C2,
                    column + 1U,
                    row + 1U,
                    1
                );
                float expected = expected_output(matrix_id, row, column);
                float actual = output[task_index * output_elements + (size_t)offset];
                if (fabsf(actual - expected) > 0.001f) {
                    check->error = RKNPU_BATCH_OUTPUT_ERROR;
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
    check->error = RKNPU_BATCH_OK;
    return 0;
}
