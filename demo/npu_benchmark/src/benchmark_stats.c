#include "benchmark_stats.h"

#include <stdlib.h>
#include <string.h>

static int compare_u64(const void *left_ptr, const void *right_ptr) {
    const uint64_t left = *(const uint64_t *)left_ptr;
    const uint64_t right = *(const uint64_t *)right_ptr;
    return (left > right) - (left < right);
}

static size_t nearest_rank_index(size_t sample_count, size_t percentile) {
    const size_t quotient = sample_count / 100U;
    const size_t remainder = sample_count % 100U;
    const size_t one_based_rank =
        quotient * percentile + (remainder * percentile + 99U) / 100U;
    return one_based_rank - 1U;
}

int benchmark_compute_latency_stats(
    const uint64_t *samples_us,
    size_t sample_count,
    benchmark_latency_stats_t *stats_out
) {
    uint64_t *sorted_samples;
    long double total_us = 0.0L;

    if (samples_us == NULL || stats_out == NULL || sample_count == 0U) {
        return -1;
    }
    if (sample_count > SIZE_MAX / sizeof(*sorted_samples)) {
        return -1;
    }

    sorted_samples = malloc(sample_count * sizeof(*sorted_samples));
    if (sorted_samples == NULL) {
        return -1;
    }
    memcpy(sorted_samples, samples_us, sample_count * sizeof(*sorted_samples));
    qsort(sorted_samples, sample_count, sizeof(*sorted_samples), compare_u64);

    memset(stats_out, 0, sizeof(*stats_out));
    stats_out->sample_count = sample_count;
    stats_out->min_us = sorted_samples[0];
    stats_out->max_us = sorted_samples[sample_count - 1U];
    stats_out->p50_us = sorted_samples[nearest_rank_index(sample_count, 50U)];
    stats_out->p95_us = sorted_samples[nearest_rank_index(sample_count, 95U)];
    stats_out->p99_us = sorted_samples[nearest_rank_index(sample_count, 99U)];

    for (size_t index = 0; index < sample_count; ++index) {
        total_us += (long double)samples_us[index];
        if ((long double)samples_us[index] >
            (long double)stats_out->p50_us * 1.5L) {
            stats_out->above_150pct_p50_count += 1U;
        }
    }
    stats_out->mean_us = (double)(total_us / (long double)sample_count);

    if (stats_out->p50_us != 0U) {
        stats_out->p95_over_p50 =
            (double)stats_out->p95_us / (double)stats_out->p50_us;
        stats_out->p99_over_p50 =
            (double)stats_out->p99_us / (double)stats_out->p50_us;
    }

    free(sorted_samples);
    return 0;
}
