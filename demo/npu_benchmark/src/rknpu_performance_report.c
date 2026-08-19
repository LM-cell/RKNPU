/* RKNPU 并发性能报告实现，最后修改日期：2026-08-12。 */

#include "rknpu_performance_report.h"

#include <stdio.h>

#include "benchmark_stats.h"

static double ratio_u64(uint64_t numerator, uint64_t denominator) {
    return denominator == 0U ? 0.0 : (double)numerator / (double)denominator;
}

/* 保持 core_scaling_benchmark_fix 使用的尾延迟判定文字和阈值。 */
static const char *latency_stability_hint(const benchmark_latency_stats_t *stats) {
    double max_over_p50 = ratio_u64(stats->max_us, stats->p50_us);

    if (stats->p95_over_p50 >= 1.20) {
        return "recurring/persistent tail: P95 is at least 20% above P50";
    }
    if (stats->p99_over_p50 >= 1.20 || max_over_p50 >= 1.50) {
        return "sporadic tail: P50/P95 are stable but rare submits are slower";
    }
    return "stable by the 20% percentile-tail heuristic";
}

/* 样本按线程编号、再按该线程轮次输出；排序只在统计模块的副本中进行。 */
static void print_latency_samples(const rknpu_performance_report_t *report) {
    printf("  raw submit samples (us), thread-major:\n");
    for (size_t index = 0; index < report->submit_sample_count; index++) {
        if (index % 10U == 0U) {
            size_t end = index + 10U < report->submit_sample_count
                ? index + 10U
                : report->submit_sample_count;
            printf("    %03zu-%03zu:", index + 1U, end);
        }
        printf(" %llu", (unsigned long long)report->submit_samples_us[index]);
        if (index % 10U == 9U || index + 1U == report->submit_sample_count) {
            printf("\n");
        }
    }
}

int rknpu_print_performance_report(const rknpu_performance_report_t *report) {
    benchmark_latency_stats_t stats;
    double avg_task_us;
    double tasks_per_sec;
    double macs_per_task;
    double gmac_per_sec;
    double slow_sample_pct;
    double jitter_pct;
    size_t successful_task_count;

    if (report == NULL || report->scenario == NULL ||
        report->submit_samples_us == NULL || report->submit_sample_count == 0U ||
        report->measured_window_us == 0U) {
        return -1;
    }
    if (benchmark_compute_latency_stats(
            report->submit_samples_us,
            report->submit_sample_count,
            &stats
        ) != 0) {
        return -1;
    }

    successful_task_count =
        report->successful_submit_count * (size_t)report->scenario->task_count;
    avg_task_us = stats.mean_us / (double)report->scenario->task_count;
    tasks_per_sec = (double)successful_task_count * 1000000.0 /
        (double)report->measured_window_us;
    macs_per_task = (double)report->scenario->m *
        (double)report->scenario->k * (double)report->scenario->n;
    gmac_per_sec = tasks_per_sec * macs_per_task / 1000000000.0;
    slow_sample_pct = (double)stats.above_150pct_p50_count * 100.0 /
        (double)stats.sample_count;
    jitter_pct = stats.mean_us > 0.0
        ? (double)(stats.max_us - stats.min_us) / stats.mean_us * 100.0
        : 0.0;

    printf("%u-core concurrent metrics\n", report->npu_cores);
    printf("  per-task input  : %8.2f KiB\n",
           (double)report->scenario->m * report->scenario->k * sizeof(_Float16) / 1024.0);
    printf("  per-task weight : %8.2f KiB\n",
           (double)report->scenario->n * report->scenario->k * sizeof(_Float16) / 1024.0);
    printf("  per-task output : %8.2f KiB\n",
           (double)report->scenario->m * report->scenario->n * sizeof(float) / 1024.0);
    printf("  total DMA bytes : %8.2f MiB\n",
           (double)report->total_dma_bytes / (1024.0 * 1024.0));
    printf("  setup operands  : %8.3f ms\n",
           (double)report->setup_operands_us / 1000.0);
    printf("  build regcmds   : %8.3f ms\n",
           (double)report->build_regcmds_us / 1000.0);
    printf("  samples         : %8zu measured submits (%u threads x %u rounds)\n",
           stats.sample_count, report->threads, report->measured_rounds);
    printf("  mean submit     : %8.3f ms\n", stats.mean_us / 1000.0);
    printf("  P50 / P95 / P99: %8.3f / %8.3f / %8.3f ms\n",
           (double)stats.p50_us / 1000.0,
           (double)stats.p95_us / 1000.0,
           (double)stats.p99_us / 1000.0);
    printf("  min / max       : %8.3f / %8.3f ms\n",
           (double)stats.min_us / 1000.0,
           (double)stats.max_us / 1000.0);
    printf("  P95/P50 P99/P50: %8.3f / %8.3f x\n",
           stats.p95_over_p50, stats.p99_over_p50);
    printf("  >150%% of P50    : %8zu / %zu submits (%5.1f%%)\n",
           stats.above_150pct_p50_count, stats.sample_count, slow_sample_pct);
    printf("  stability hint  : %s\n", latency_stability_hint(&stats));
    printf("  avg task        : %8.3f us\n", avg_task_us);
    printf("  tasks / sec     : %8.2f\n", tasks_per_sec);
    printf("  GMAC / sec      : %8.3f\n", gmac_per_sec);
    printf("  GFLOP / sec     : %8.3f\n", gmac_per_sec * 2.0);
    printf("  jitter span     : %8.2f %%\n", jitter_pct);
    print_latency_samples(report);
    return 0;
}
