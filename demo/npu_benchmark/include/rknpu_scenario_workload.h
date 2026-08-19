/* RKNPU 多场景并发负载接口，最后修改日期：2026-08-12。 */

#ifndef RKNPU_SCENARIO_WORKLOAD_H
#define RKNPU_SCENARIO_WORKLOAD_H

#include <stddef.h>
#include <stdint.h>

#include "rknpu-ioctl.h"

/* Task 复用一组输入和权重，或者分别使用独立输入和权重。 */
typedef enum {
    RKNPU_OPERANDS_SHARED = 0,
    RKNPU_OPERANDS_UNIQUE,
} rknpu_operand_mode_t;

/* 一个可直接运行的场景组合；七项数据与 core_scaling_benchmark 保持一致。 */
typedef struct {
    /* 命令行使用的完整名称，例如 mid_balanced-shared。 */
    const char *name;
    /* 单个 Task 的矩阵尺寸。 */
    uint32_t m;
    uint32_t k;
    uint32_t n;
    /* 每个 Submit 包含的 Task 数量。 */
    uint32_t task_count;
    /* 输入和权重的共享方式。 */
    rknpu_operand_mode_t operand_mode;
} rknpu_scenario_case_t;

/* 隐藏 GEM、DMA 切片和 Submit 模板，调用方只管理其生命周期。 */
typedef struct rknpu_scenario_workload rknpu_scenario_workload_t;

/*
 * 单个 worker 的场景资源指标。字节数来自实际分配尺寸，两个准备时间只覆盖
 * 矩阵填充和寄存器命令生成，不包含 GEM 创建、mmap 或线程启动。
 */
typedef struct {
    size_t total_dma_bytes;
    uint64_t setup_operands_us;
    uint64_t build_regcmds_us;
} rknpu_scenario_metrics_t;

typedef enum {
    RKNPU_SCENARIO_CHECK_OK = 0,
    RKNPU_SCENARIO_CHECK_TASK_COUNTER,
    RKNPU_SCENARIO_CHECK_IRQ_STATUS,
    RKNPU_SCENARIO_CHECK_OUTPUT,
} rknpu_scenario_check_error_t;

/* 保存一次完成校验的首个错误现场。 */
typedef struct {
    rknpu_scenario_check_error_t error;
    uint32_t task_index;
    uint32_t task_counter;
    uint32_t irq_status;
    uint32_t row;
    uint32_t column;
    float expected;
    float actual;
} rknpu_scenario_check_t;

/* 返回七个固定场景组合；数组在程序整个生命周期内有效。 */
const rknpu_scenario_case_t *rknpu_scenario_cases(size_t *count);

/* 按完整场景名称查找；找不到时返回 NULL。 */
const rknpu_scenario_case_t *rknpu_scenario_find(const char *name);

/* 返回 shared 或 unique，用于实验条件输出。 */
const char *rknpu_operand_mode_name(rknpu_operand_mode_t mode);

/* 按当前核心数均分 Task；只修改 subcore_task[]，不改变调用方的 core_mask。 */
void rknpu_distribute_tasks_to_lanes(
    struct rknpu_submit *submit,
    uint32_t task_count,
    uint32_t npu_cores
);

/*
 * 为一个线程创建指定场景的全部 GEM，并按 npu_cores 均匀设置逻辑 lane。
 * core_mask 只限制可用物理核心，lane 不固定绑定具体物理核心。
 */
int rknpu_scenario_workload_create(
    int fd,
    uint32_t thread_id,
    const rknpu_scenario_case_t *scenario,
    uint32_t npu_cores,
    uint32_t core_mask,
    rknpu_scenario_workload_t **workload_out
);

/* 释放全部映射和 GEM；允许 workload 为 NULL。 */
void rknpu_scenario_workload_destroy(rknpu_scenario_workload_t *workload);

/* 清理上一轮输出和 IRQ 状态，再复制一份可供本轮 ioctl 修改的 Submit。 */
void rknpu_scenario_workload_begin(
    rknpu_scenario_workload_t *workload,
    struct rknpu_submit *submit
);

/*
 * 连续流水测试只复制干净的 Submit 模板，不清空输出和 IRQ 状态，避免在相邻
 * blocking ioctl 之间加入与 NPU 提交无关的大块内存操作。
 */
void rknpu_scenario_workload_next_submit(
    const rknpu_scenario_workload_t *workload,
    struct rknpu_submit *submit
);

/* 读取创建阶段保存的资源大小和准备耗时。 */
void rknpu_scenario_workload_get_metrics(
    const rknpu_scenario_workload_t *workload,
    rknpu_scenario_metrics_t *metrics
);

/* 校验 Task 完成数量、每个 Task 的 IRQ 状态和抽样矩阵结果。 */
int rknpu_scenario_workload_check(
    const rknpu_scenario_workload_t *workload,
    const struct rknpu_submit *submit,
    rknpu_scenario_check_t *check
);

#endif
