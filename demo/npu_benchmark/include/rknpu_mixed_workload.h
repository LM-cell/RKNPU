/* RKNPU 48 Task 混合负载接口，最后修改日期：2026-08-18。 */

#ifndef RKNPU_MIXED_WORKLOAD_H
#define RKNPU_MIXED_WORKLOAD_H

#include <stddef.h>
#include <stdint.h>

#include "rknpu-ioctl.h"

#define RKNPU_MIXED_TASK_COUNT 48U
#define RKNPU_MIXED_TYPE_COUNT 4U
#define RKNPU_MIXED_TASKS_PER_TYPE 12U

/* 四种 Task 形状均来自现有 shared 场景。 */
typedef enum {
    RKNPU_MIXED_TINY = 0,
    RKNPU_MIXED_MID,
    RKNPU_MIXED_HEAVY,
    RKNPU_MIXED_LLAMA,
} rknpu_mixed_task_type_t;

/* clustered 主动制造 Lane 不均衡；interleaved 作为均衡对照。 */
typedef enum {
    RKNPU_MIXED_CLUSTERED = 0,
    RKNPU_MIXED_INTERLEAVED,
} rknpu_mixed_layout_t;

typedef struct rknpu_mixed_workload rknpu_mixed_workload_t;

typedef struct {
    size_t total_dma_bytes;
    uint64_t setup_operands_us;
    uint64_t build_regcmds_us;
} rknpu_mixed_metrics_t;

typedef enum {
    RKNPU_MIXED_CHECK_OK = 0,
    RKNPU_MIXED_CHECK_TASK_COUNTER,
    RKNPU_MIXED_CHECK_IRQ_STATUS,
    RKNPU_MIXED_CHECK_OUTPUT,
} rknpu_mixed_check_error_t;

/* 保存首个错误，避免板端只得到笼统的 FAIL。 */
typedef struct {
    rknpu_mixed_check_error_t error;
    uint32_t task_index;
    uint32_t task_counter;
    uint32_t irq_status;
    uint32_t row;
    uint32_t column;
    float expected;
    float actual;
} rknpu_mixed_check_t;

/* 返回命令行和报告使用的稳定名称。 */
const char *rknpu_mixed_layout_name(rknpu_mixed_layout_t layout);
const char *rknpu_mixed_type_name(rknpu_mixed_task_type_t type);

/* 按布局返回某个 Task 的类型，用于构建命令和分析调度 trace。 */
rknpu_mixed_task_type_t rknpu_mixed_task_type(
    rknpu_mixed_layout_t layout,
    uint32_t task_index
);

/*
 * 创建一个线程独占的 48 Task 混合负载。
 * 四类输入和权重在同一线程内只读共享；每个 Task 使用独立输出切片和 regcmd。
 */
int rknpu_mixed_workload_create(
    int fd,
    uint32_t thread_id,
    uint32_t npu_cores,
    uint32_t core_mask,
    rknpu_mixed_layout_t layout,
    int dynamic_tasks,
    rknpu_mixed_workload_t **workload_out
);

/* 按“解除映射，再销毁 GEM”的顺序释放全部资源。 */
void rknpu_mixed_workload_destroy(rknpu_mixed_workload_t *workload);

/* 清空输出和 IRQ 状态，并返回第一轮使用的 Submit。 */
void rknpu_mixed_workload_begin(
    rknpu_mixed_workload_t *workload,
    struct rknpu_submit *submit
);

/* 流水阶段只恢复 Submit 头，不在 blocking ioctl 之间清空大块输出。 */
void rknpu_mixed_workload_next_submit(
    const rknpu_mixed_workload_t *workload,
    struct rknpu_submit *submit
);

/* 返回真实分配字节数和负载准备时间。 */
void rknpu_mixed_workload_get_metrics(
    const rknpu_mixed_workload_t *workload,
    rknpu_mixed_metrics_t *metrics
);

/* 校验 48 个完成状态，并对每个 Task 的独立输出切片抽样。 */
int rknpu_mixed_workload_check(
    const rknpu_mixed_workload_t *workload,
    const struct rknpu_submit *submit,
    rknpu_mixed_check_t *check
);

#endif
