#ifndef BENCHMARK_STATS_H
#define BENCHMARK_STATS_H

#include <stddef.h>
#include <stdint.h>

typedef struct {
    size_t sample_count;
    uint64_t min_us;
    uint64_t max_us;
    uint64_t p50_us;
    uint64_t p95_us;
    uint64_t p99_us;
    double mean_us;
    double p95_over_p50;
    double p99_over_p50;
    size_t above_150pct_p50_count;
} benchmark_latency_stats_t;

/*
 * Uses the nearest-rank percentile definition:
 *   rank = ceil(percentile * sample_count)
 *
 * The input array is never modified. Returns zero on success.
 */
int benchmark_compute_latency_stats(
    const uint64_t *samples_us,
    size_t sample_count,
    benchmark_latency_stats_t *stats_out
);

#endif
