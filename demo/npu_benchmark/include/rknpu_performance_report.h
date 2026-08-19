/* RKNPU 并发性能报告接口，最后修改日期：2026-08-12。 */

#ifndef RKNPU_PERFORMANCE_REPORT_H
#define RKNPU_PERFORMANCE_REPORT_H

#include <stddef.h>
#include <stdint.h>

#include "rknpu_scenario_workload.h"

/* 两个并发测试共同使用的原始测量输入。 */
typedef struct {
    const rknpu_scenario_case_t *scenario;
    uint32_t npu_cores;
    uint32_t threads;
    uint32_t measured_rounds;
    const uint64_t *submit_samples_us;
    size_t submit_sample_count;
    size_t successful_submit_count;
    uint64_t measured_window_us;
    size_t total_dma_bytes;
    uint64_t setup_operands_us;
    uint64_t build_regcmds_us;
} rknpu_performance_report_t;

/* 复用 benchmark_stats 的延迟统计，输出与 core_scaling 相同口径的任务指标。 */
int rknpu_print_performance_report(const rknpu_performance_report_t *report);

#endif
