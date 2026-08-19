/* 10 Task 矩阵乘法测试负载，最后修改日期：2026-08-07。 */

#ifndef RKNPU_BATCH_WORKLOAD_H
#define RKNPU_BATCH_WORKLOAD_H

#include <stdint.h>

#include "rknpu-ioctl.h"

/* 每个 Submit 固定包含 10 个相同尺寸、输出缓冲区互相独立的 Task。 */
#define RKNPU_BATCH_TASKS 10U
#define RKNPU_BATCH_M 64U
#define RKNPU_BATCH_K 512U
#define RKNPU_BATCH_N 512U

/*
 * 负载内部拥有 GEM handle、mmap 地址、任务描述符和 Submit 模板。
 * benchmark 只通过以下接口创建、提交、校验和销毁，不直接操作 DMA 所有权。
 */
typedef struct rknpu_batch_workload rknpu_batch_workload_t;

/* 结果校验失败的具体阶段。 */
typedef enum {
    /* task_counter、IRQ 和矩阵输出均正确。 */
    RKNPU_BATCH_OK = 0,
    /* 驱动返回的完成 Task 数不是 10。 */
    RKNPU_BATCH_TASK_COUNTER_ERROR,
    /* 某个 Task 的 int_status 不是预期的 0x300。 */
    RKNPU_BATCH_IRQ_STATUS_ERROR,
    /* 某个抽样输出元素与 CPU 参考值不一致。 */
    RKNPU_BATCH_OUTPUT_ERROR,
} rknpu_batch_error_t;

/* 保存首个校验错误，供主测试打印可定位的信息。 */
typedef struct {
    /* 错误类别。 */
    rknpu_batch_error_t error;
    /* 出错 Task 在 10 Task 数组中的下标。 */
    uint32_t task_index;
    /* 输出错误对应的矩阵行。 */
    uint32_t row;
    /* 输出错误对应的矩阵列。 */
    uint32_t column;
    /* task_counter 或 int_status 的实际整数值。 */
    uint32_t actual_u32;
    /* CPU 计算的矩阵参考结果。 */
    float expected;
    /* NPU 输出缓冲区中的实际结果。 */
    float actual;
} rknpu_batch_check_t;

/*
 * 为一个测试线程创建固定 10 Task 负载。
 *
 * 函数一次性分配 regcmd、Task、输入、权重和输出五个 GEM 对象，生成矩阵
 * 命令，并按 npu_cores 设置 10、5+5 或 4+3+3 的 lane。成功后所有权通过
 * workload_out 交给调用者；失败时函数内部释放已经创建的部分资源。
 */
int rknpu_batch_create(
    int fd,
    uint32_t thread_id,
    uint32_t npu_cores,
    uint32_t core_mask,
    rknpu_batch_workload_t **workload_out
);

/* 解除 mmap、销毁五个 GEM handle 并释放 workload；允许 workload 为 NULL。 */
void rknpu_batch_destroy(int fd, rknpu_batch_workload_t *workload);

/*
 * 开始一轮 Submit 前清空十份输出和 int_status，复位 task_counter，并写入
 * 本轮优先级。返回的 Submit 指针在 rknpu_batch_destroy 前保持有效。
 */
struct rknpu_submit *rknpu_batch_begin(
    rknpu_batch_workload_t *workload,
    int32_t priority
);

/*
 * 校验 task_counter、全部 Task 的 IRQ 状态，以及每个 Task 的四个矩阵元素。
 * 返回 0 表示通过；返回 -1 时 check 给出首个错误位置和实际值。
 */
int rknpu_batch_check(
    const rknpu_batch_workload_t *workload,
    const struct rknpu_submit *submit,
    rknpu_batch_check_t *check
);

#endif
