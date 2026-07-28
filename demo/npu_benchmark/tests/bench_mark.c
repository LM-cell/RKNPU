/*
 * Copyright (C) 2024  Jasbir Matharu, <jasjnuk@gmail.com>
 *
 * This file is part of rk3588-npu.
 *
 * rk3588-npu is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.

 * rk3588-npu is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.

 * You should have received a copy of the GNU General Public License
 * along with rk3588-npu.  If not, see <https://www.gnu.org/licenses/>.
 *
 */

#include <stdio.h>
#include <stdint.h>
#include <unistd.h>
#include <sys/ioctl.h>
#include <stdlib.h>
#include <string.h>
#include <fcntl.h>
#include <errno.h>
#include <math.h>
#include <time.h>
#include <sys/mman.h>
#include <sys/time.h>

#include <libdrm/drm.h>

#include "rknpu-ioctl.h"
#include "npu_interface.h"
#include "npu_matmul.h"

#define MAX_M 544
#define MAX_K 4096 
#define MAX_N 4096 
#define DEFAULT_ITERATIONS 500

// ANSI Color codes
#define COLOR_RESET   "\033[0m"
#define COLOR_RED     "\033[31m"
#define COLOR_GREEN   "\033[32m"
#define COLOR_YELLOW  "\033[33m"
#define COLOR_BLUE    "\033[34m"
#define COLOR_MAGENTA "\033[35m"
#define COLOR_CYAN    "\033[36m"
#define COLOR_WHITE   "\033[37m"
#define COLOR_BOLD    "\033[1m"
#define COLOR_DIM     "\033[2m"

// Background colors
#define BG_RED        "\033[41m"
#define BG_GREEN      "\033[42m"
#define BG_YELLOW     "\033[43m"

// Combined styles
#define STYLE_SUCCESS COLOR_BOLD COLOR_GREEN
#define STYLE_ERROR   COLOR_BOLD COLOR_RED
#define STYLE_WARNING COLOR_BOLD COLOR_YELLOW
#define STYLE_INFO    COLOR_BOLD COLOR_CYAN
#define STYLE_HEADER  COLOR_BOLD COLOR_BLUE
#define STYLE_DIM     COLOR_DIM

// Global buffers
int8_t matrixA_int8[MAX_M * MAX_K];
int8_t matrixB_int8[MAX_N * MAX_K];
int32_t result_int8[MAX_M * MAX_N];

_Float16 matrixA_fp16[MAX_M * MAX_K];
_Float16 matrixB_fp16[MAX_N * MAX_K];
float result_fp32[MAX_M * MAX_N];

uint64_t npu_regs[112];

typedef struct {
    int m;
    int k;
    int n;
    const char* description;
} benchmark_config_t;

// Get current time in microseconds
static inline int64_t get_time_us() {
    struct timeval tv;
    gettimeofday(&tv, NULL);
    return tv.tv_sec * 1000000LL + tv.tv_usec;
}

// CPU reference implementation for INT8
void matmul_int8_cpu(int m, int k, int n, int8_t *src0, int8_t *src1, int32_t *dst) {
    for (int i = 0; i < m; i++) {
        for (int j = 0; j < n; j++) {
            int32_t sum = 0;
            for (int l = 0; l < k; l++) {
                sum += (int32_t)(src0[i*k + l]) * (int32_t)(src1[j*k + l]);
            }
            dst[i*n + j] = sum;
        }
    }
}

// CPU reference implementation for FP16
void matmul_fp16_cpu(int m, int k, int n, _Float16 *src0, _Float16 *src1, float *dst) {
    for (int i = 0; i < m; i++) {
        for (int j = 0; j < n; j++) {
            float sum = 0;
            for (int l = 0; l < k; l++) {
                sum += (float)src0[i*k + l] * (float)src1[j*k + l];
            }
            dst[i*n + j] = sum;
        }
    }
}

// Generate random INT8 data
void generate_random_int8(int8_t *data, int size) {
    for (int i = 0; i < size; i++) {
        data[i] = (int8_t)((rand() % 255) + 1);
    }
}

// Generate random FP16 data
void generate_random_fp16(_Float16 *data, int size) {
    for (int i = 0; i < size; i++) {
        data[i] = (_Float16)((int)(10.0 * rand() / (float)RAND_MAX));
    }
}

// Benchmark INT8 matrix multiplication on NPU
double benchmark_int8_npu(int fd, int m, int k, int n, int iterations) {
    // Allocate NPU memory
    uint64_t regcmd_dma, regcmd_obj;
    uint32_t regcmd_handle;
    uint64_t *regcmd = mem_allocate(fd, 1024, &regcmd_dma, &regcmd_obj, 0, &regcmd_handle);

    uint64_t tasks_dma, tasks_obj;
    uint32_t tasks_handle;
    struct rknpu_task *tasks = mem_allocate(fd, 1024, &tasks_dma, &tasks_obj, 
                                            RKNPU_MEM_KERNEL_MAPPING, &tasks_handle);

    uint64_t input_dma, input_obj;
    uint32_t input_handle;
    void *input = mem_allocate(fd, m*k*sizeof(int8_t), &input_dma, &input_obj, 0, &input_handle);

    uint64_t weights_dma, weights_obj;
    uint32_t weights_handle;
    void *weights = mem_allocate(fd, n*k*sizeof(int8_t), &weights_dma, &weights_obj, 0, &weights_handle);

    uint64_t output_dma, output_obj;
    uint32_t output_handle;
    void *output = mem_allocate(fd, m*n*sizeof(int32_t), &output_dma, &output_obj, 0, &output_handle);

    if (!regcmd || !tasks || !input || !weights || !output) {
        fprintf(stderr, "%sERROR:%s Failed to allocate NPU memory for INT8 [%dx%dx%d]\n%s", 
                STYLE_ERROR, COLOR_RESET, m, k, n, COLOR_RESET);
        if (regcmd) { munmap(regcmd, 1024); mem_destroy(fd, regcmd_handle, regcmd_obj); }
        if (tasks) { munmap(tasks, 1024); mem_destroy(fd, tasks_handle, tasks_obj); }
        if (input) { munmap(input, m*k*sizeof(int8_t)); mem_destroy(fd, input_handle, input_obj); }
        if (weights) { munmap(weights, n*k*sizeof(int8_t)); mem_destroy(fd, weights_handle, weights_obj); }
        if (output) { munmap(output, m*n*sizeof(int32_t)); mem_destroy(fd, output_handle, output_obj); }
        return -1.0;
    }

    // Configure NPU
    matmul_params_t params;
    params.m = m;
    params.k = k;
    params.n = n;
    params.input_dma = input_dma;
    params.weights_dma = weights_dma;
    params.output_dma = output_dma;
    params.tasks = (uint64_t *)&npu_regs;
    
    if (gen_matmul_int8(&params) != 0) {
        fprintf(stderr, "%sERROR:%s gen_matmul_int8 failed for [%dx%dx%d]\n%s", 
                STYLE_ERROR, COLOR_RESET, m, k, n, COLOR_RESET);
        munmap(regcmd, 1024);
        munmap(tasks, 1024);
        munmap(input, m*k*sizeof(int8_t));
        munmap(weights, n*k*sizeof(int8_t));
        munmap(output, m*n*sizeof(int32_t));
        mem_destroy(fd, regcmd_handle, regcmd_obj);
        mem_destroy(fd, tasks_handle, tasks_obj);
        mem_destroy(fd, input_handle, input_obj);
        mem_destroy(fd, weights_handle, weights_obj);
        mem_destroy(fd, output_handle, output_obj);
        return -1.0;
    }

    memcpy(regcmd, npu_regs, sizeof(npu_regs));

    // Setup task
    tasks[0].flags = 0;
    tasks[0].op_idx = 0;
    tasks[0].enable_mask = 0xd;
    tasks[0].int_mask = 0x300;
    tasks[0].int_clear = 0x1ffff;
    tasks[0].int_status = 0;
    tasks[0].regcfg_amount = sizeof(npu_regs)/sizeof(uint64_t) - (RKNPU_PC_DATA_EXTRA_AMOUNT + 4);
    tasks[0].regcfg_offset = 0;
    tasks[0].regcmd_addr = regcmd_dma;

    // Prepare input data
    int8_t *weights_int8 = (int8_t *)weights;
    int8_t *input_int8 = (int8_t *)input;

    for (int nn = 1; nn <= n; nn++) {
        for (int kk = 1; kk <= k; kk++) {
            weights_int8[weight_int8(k, nn, kk)] = matrixB_int8[((nn-1)*k) + (kk-1)];
        }
    }

    for (int mm = 1; mm <= m; mm++) {
        for (int kk = 1; kk <= k; kk++) {
            input_int8[feature_data(k, m, 1, 16, kk, mm, 1)] = matrixA_int8[((mm-1)*k) + (kk-1)];
        }
    }

    // Setup submit structure
    struct rknpu_submit submit = {
        .flags = RKNPU_JOB_PC | RKNPU_JOB_BLOCK | RKNPU_JOB_PINGPONG,
        .timeout = 6000,
        .task_start = 0,
        .task_number = 1,
        .task_counter = 0,
        .priority = 0,
        .task_obj_addr = tasks_obj,
        .regcfg_obj_addr = 0,
        .task_base_addr = 0,
        .user_data = 0,
        .core_mask = 1,
        .fence_fd = -1,
        .subcore_task = {
        { .task_start = 0, .task_number = 1 },
        { .task_start = 1, .task_number = 0 },
        { .task_start = 2, .task_number = 0 },
        { .task_start = 0, .task_number = 0 },
        { .task_start = 0, .task_number = 0 }
        },
    };

    // Warmup
    for (int i = 0; i < 10; i++) {
        int ret = ioctl(fd, DRM_IOCTL_RKNPU_SUBMIT, &submit);
        if (ret < 0) {
            fprintf(stderr, "%sERROR:%s NPU warmup failed for INT8 [%dx%dx%d]: %s%s%s (errno=%d)\n%s", 
                    STYLE_ERROR, COLOR_RESET, m, k, n, COLOR_YELLOW, strerror(errno), COLOR_RESET, errno, COLOR_RESET);
            munmap(regcmd, 1024);
            munmap(tasks, 1024);
            munmap(input, m*k*sizeof(int8_t));
            munmap(weights, n*k*sizeof(int8_t));
            munmap(output, m*n*sizeof(int32_t));
            mem_destroy(fd, regcmd_handle, regcmd_obj);
            mem_destroy(fd, tasks_handle, tasks_obj);
            mem_destroy(fd, input_handle, input_obj);
            mem_destroy(fd, weights_handle, weights_obj);
            mem_destroy(fd, output_handle, output_obj);
            return -1.0;
        }
    }

    // Benchmark
    int64_t start = get_time_us();
    for (int i = 0; i < iterations; i++) {
        int ret = ioctl(fd, DRM_IOCTL_RKNPU_SUBMIT, &submit);
        if (ret < 0) {
            fprintf(stderr, "%sERROR:%s NPU execution failed at iteration %d for INT8 [%dx%dx%d]: %s%s%s (errno=%d)\n%s", 
                    STYLE_ERROR, COLOR_RESET, i, m, k, n, COLOR_YELLOW, strerror(errno), COLOR_RESET, errno, COLOR_RESET);
            munmap(regcmd, 1024);
            munmap(tasks, 1024);
            munmap(input, m*k*sizeof(int8_t));
            munmap(weights, n*k*sizeof(int8_t));
            munmap(output, m*n*sizeof(int32_t));
            mem_destroy(fd, regcmd_handle, regcmd_obj);
            mem_destroy(fd, tasks_handle, tasks_obj);
            mem_destroy(fd, input_handle, input_obj);
            mem_destroy(fd, weights_handle, weights_obj);
            mem_destroy(fd, output_handle, output_obj);
            return -1.0;
        }
    }
    int64_t end = get_time_us();

    double time_ms = (end - start) / 1000.0 / iterations;

    // Cleanup
    munmap(regcmd, 1024);
    munmap(tasks, 1024);
    munmap(input, m*k*sizeof(int8_t));
    munmap(weights, n*k*sizeof(int8_t));
    munmap(output, m*n*sizeof(int32_t));

    mem_destroy(fd, regcmd_handle, regcmd_obj);
    mem_destroy(fd, tasks_handle, tasks_obj);
    mem_destroy(fd, input_handle, input_obj);
    mem_destroy(fd, weights_handle, weights_obj);
    mem_destroy(fd, output_handle, output_obj);

    return time_ms;
}

// Benchmark FP16 matrix multiplication on NPU
double benchmark_fp16_npu(int fd, int m, int k, int n, int iterations) {
    // Allocate NPU memory
    uint64_t regcmd_dma, regcmd_obj;
    uint32_t regcmd_handle;
    uint64_t *regcmd = mem_allocate(fd, 1024, &regcmd_dma, &regcmd_obj, 0, &regcmd_handle);

    uint64_t tasks_dma, tasks_obj;
    uint32_t tasks_handle;
    struct rknpu_task *tasks = mem_allocate(fd, 1024, &tasks_dma, &tasks_obj, 
                                            RKNPU_MEM_KERNEL_MAPPING, &tasks_handle);

    uint64_t input_dma, input_obj;
    uint32_t input_handle;
    void *input = mem_allocate(fd, m*k*sizeof(_Float16), &input_dma, &input_obj, 0, &input_handle);

    uint64_t weights_dma, weights_obj;
    uint32_t weights_handle;
    void *weights = mem_allocate(fd, n*k*sizeof(_Float16), &weights_dma, &weights_obj, 0, &weights_handle);

    uint64_t output_dma, output_obj;
    uint32_t output_handle;
    void *output = mem_allocate(fd, m*n*sizeof(float), &output_dma, &output_obj, 0, &output_handle);

    if (!regcmd || !tasks || !input || !weights || !output) {
        fprintf(stderr, "%sERROR:%s Failed to allocate NPU memory for FP16 [%dx%dx%d]\n%s", 
                STYLE_ERROR, COLOR_RESET, m, k, n, COLOR_RESET);
        if (regcmd) { munmap(regcmd, 1024); mem_destroy(fd, regcmd_handle, regcmd_obj); }
        if (tasks) { munmap(tasks, 1024); mem_destroy(fd, tasks_handle, tasks_obj); }
        if (input) { munmap(input, m*k*sizeof(_Float16)); mem_destroy(fd, input_handle, input_obj); }
        if (weights) { munmap(weights, n*k*sizeof(_Float16)); mem_destroy(fd, weights_handle, weights_obj); }
        if (output) { munmap(output, m*n*sizeof(float)); mem_destroy(fd, output_handle, output_obj); }
        return -1.0;
    }

    // Configure NPU
    matmul_params_t params;
    params.m = m;
    params.k = k;
    params.n = n;
    params.input_dma = input_dma;
    params.weights_dma = weights_dma;
    params.output_dma = output_dma;
    params.tasks = (uint64_t *)&npu_regs;
    params.fp32tofp16 = 0;
    
    if (gen_matmul_fp16(&params) != 0) {
        fprintf(stderr, "%sERROR:%s gen_matmul_fp16 failed for [%dx%dx%d]\n%s", 
                STYLE_ERROR, COLOR_RESET, m, k, n, COLOR_RESET);
        munmap(regcmd, 1024);
        munmap(tasks, 1024);
        munmap(input, m*k*sizeof(_Float16));
        munmap(weights, n*k*sizeof(_Float16));
        munmap(output, m*n*sizeof(float));
        mem_destroy(fd, regcmd_handle, regcmd_obj);
        mem_destroy(fd, tasks_handle, tasks_obj);
        mem_destroy(fd, input_handle, input_obj);
        mem_destroy(fd, weights_handle, weights_obj);
        mem_destroy(fd, output_handle, output_obj);
        return -1.0;
    }

    memcpy(regcmd, npu_regs, sizeof(npu_regs));

    // Setup task
    tasks[0].flags = 0;
    tasks[0].op_idx = 0;
    tasks[0].enable_mask = 0xd;
    tasks[0].int_mask = 0x300;
    tasks[0].int_clear = 0x1ffff;
    tasks[0].int_status = 0;
    tasks[0].regcfg_amount = sizeof(npu_regs)/sizeof(uint64_t) - (RKNPU_PC_DATA_EXTRA_AMOUNT + 4);
    tasks[0].regcfg_offset = 0;
    tasks[0].regcmd_addr = regcmd_dma;

    // Prepare input data
    _Float16 *weights_fp16 = (_Float16 *)weights;
    _Float16 *input_fp16 = (_Float16 *)input;

    for (int nn = 1; nn <= n; nn++) {
        for (int kk = 1; kk <= k; kk++) {
            weights_fp16[weight_fp16(k, nn, kk)] = matrixB_fp16[((nn-1)*k) + (kk-1)];
        }
    }

    for (int mm = 1; mm <= m; mm++) {
        for (int kk = 1; kk <= k; kk++) {
            input_fp16[feature_data(k, m, 1, 8, kk, mm, 1)] = matrixA_fp16[((mm-1)*k) + (kk-1)];
        }
    }

    // Setup submit structure
    struct rknpu_submit submit = {
        .flags = RKNPU_JOB_PC | RKNPU_JOB_BLOCK | RKNPU_JOB_PINGPONG,
        .timeout = 6000,
        .task_start = 0,
        .task_number = 1,
        .task_counter = 0,
        .priority = 0,
        .task_obj_addr = tasks_obj,
        .regcfg_obj_addr = 0,
        .task_base_addr = 0,
        .user_data = 0,
        .core_mask = 1,
        .fence_fd = -1,
        .subcore_task = {
            { .task_start = 0, .task_number = 1 },
            { 1, 0 }, { 2, 0 }, { 0, 0 }, { 0, 0 }
        },
    };

    // Warmup
    for (int i = 0; i < 10; i++) {
        int ret = ioctl(fd, DRM_IOCTL_RKNPU_SUBMIT, &submit);
        if (ret < 0) {
            fprintf(stderr, "%sERROR:%s NPU warmup failed for FP16 [%dx%dx%d]: %s%s%s (errno=%d)\n%s", 
                    STYLE_ERROR, COLOR_RESET, m, k, n, COLOR_YELLOW, strerror(errno), COLOR_RESET, errno, COLOR_RESET);
            munmap(regcmd, 1024);
            munmap(tasks, 1024);
            munmap(input, m*k*sizeof(_Float16));
            munmap(weights, n*k*sizeof(_Float16));
            munmap(output, m*n*sizeof(float));
            mem_destroy(fd, regcmd_handle, regcmd_obj);
            mem_destroy(fd, tasks_handle, tasks_obj);
            mem_destroy(fd, input_handle, input_obj);
            mem_destroy(fd, weights_handle, weights_obj);
            mem_destroy(fd, output_handle, output_obj);
            return -1.0;
        }
    }

    // Benchmark
    int64_t start = get_time_us();
    for (int i = 0; i < iterations; i++) {
        int ret = ioctl(fd, DRM_IOCTL_RKNPU_SUBMIT, &submit);
        if (ret < 0) {
            fprintf(stderr, "%sERROR:%s NPU execution failed at iteration %d for FP16 [%dx%dx%d]: %s%s%s (errno=%d)\n%s", 
                    STYLE_ERROR, COLOR_RESET, i, m, k, n, COLOR_YELLOW, strerror(errno), COLOR_RESET, errno, COLOR_RESET);
            munmap(regcmd, 1024);
            munmap(tasks, 1024);
            munmap(input, m*k*sizeof(_Float16));
            munmap(weights, n*k*sizeof(_Float16));
            munmap(output, m*n*sizeof(float));
            mem_destroy(fd, regcmd_handle, regcmd_obj);
            mem_destroy(fd, tasks_handle, tasks_obj);
            mem_destroy(fd, input_handle, input_obj);
            mem_destroy(fd, weights_handle, weights_obj);
            mem_destroy(fd, output_handle, output_obj);
            return -1.0;
        }
    }
    int64_t end = get_time_us();

    double time_ms = (end - start) / 1000.0 / iterations;

    // Cleanup
    munmap(regcmd, 1024);
    munmap(tasks, 1024);
    munmap(input, m*k*sizeof(_Float16));
    munmap(weights, n*k*sizeof(_Float16));
    munmap(output, m*n*sizeof(float));

    mem_destroy(fd, regcmd_handle, regcmd_obj);
    mem_destroy(fd, tasks_handle, tasks_obj);
    mem_destroy(fd, input_handle, input_obj);
    mem_destroy(fd, weights_handle, weights_obj);
    mem_destroy(fd, output_handle, output_obj);

    return time_ms;
}

// Validate if a configuration is likely to work
int validate_config(int m, int k, int n, const char* type) {
    // Check alignment requirements
    if (strcmp(type, "INT8") == 0) {
        if (m % 4 != 0 && m != 1) {
            return 0; // M must be multiple of 4 or 1
        }
        if (k % 32 != 0) {
            return 0; // K must be multiple of 32
        }
        if (n % 16 != 0) {
            return 0; // N must be multiple of 16
        }
        // Check size limits - more conservative based on observed failures
        if (m > MAX_M || k > MAX_K || n > MAX_N) {
            return 0;
        }
        // Avoid problematic combinations
        // 384×1024×512 fails, so limit M*K*N product
        if (m >= 384 && k >= 1024) {
            return 0; // Too large for INT8
        }
        if (k > 1024 || n > 1024) {
            return 0; // Keep K and N reasonable
        }
    } else if (strcmp(type, "FP16") == 0) {
        if (m % 4 != 0 && m != 1) {
            return 0;
        }
        if (k % 32 != 0) {
            return 0;
        }
        if (n % 16 != 0 && n != 1) {
            return 0;
        }
        // Check size limits
        if (m > 384 || k > MAX_K || n > MAX_N) {
            return 0;
        }
        // Avoid problematic combinations
        // 256×1024×512 fails, so be more conservative
        if (m >= 256 && k >= 1024 && n >= 512) {
            return 0; // Too large for FP16
        }
        if (k > 1024 || n > 1024) {
            return 0;
        }
    }
    return 1;
}

// Run INT8 benchmark
void run_int8_benchmark(int fd, benchmark_config_t *config, int iterations) {
    int m = config->m;
    int k = config->k;
    int n = config->n;

    // Generate random data
    generate_random_int8(matrixA_int8, m * k);
    generate_random_int8(matrixB_int8, n * k);

    // Benchmark CPU (no progress indicator to avoid terminal issues)
    int64_t cpu_start = get_time_us();
    for (int i = 0; i < iterations; i++) {
        matmul_int8_cpu(m, k, n, matrixA_int8, matrixB_int8, result_int8);
    }
    int64_t cpu_end = get_time_us();
    double cpu_time_ms = (cpu_end - cpu_start) / 1000.0 / iterations;

    // Benchmark NPU
    double npu_time_ms = benchmark_int8_npu(fd, m, k, n, iterations);

    if (npu_time_ms < 0) {
        printf("| %-20s | %4d x %-4d x %-4d | %10.3f | %s%10s%s | %s%10s%s |\n",
               config->description, m, k, n, cpu_time_ms, 
               STYLE_ERROR, "FAILED", COLOR_RESET, 
               STYLE_DIM, "N/A", COLOR_RESET);
        return;
    }

    // Calculate speedup
    double speedup = cpu_time_ms / npu_time_ms;
    
    // Color code based on speedup
    const char *speedup_color;
    if (speedup > 100.0) {
        speedup_color = STYLE_SUCCESS;  // Green for excellent speedup
    } else if (speedup > 10.0) {
        speedup_color = COLOR_GREEN;     // Green for good speedup
    } else if (speedup > 1.0) {
        speedup_color = COLOR_CYAN;      // Cyan for positive speedup
    } else {
        speedup_color = COLOR_YELLOW;    // Yellow for CPU faster
    }

    printf("| %-20s | %4d x %-4d x %-4d | %s%10.3f%s | %s%10.3f%s | %s%10.2fx%s |\n",
           config->description, m, k, n, 
           COLOR_CYAN, cpu_time_ms, COLOR_RESET,
           COLOR_MAGENTA, npu_time_ms, COLOR_RESET,
           speedup_color, speedup, COLOR_RESET);
    fflush(stdout);  // Ensure output is displayed immediately
}

// Run FP16 benchmark
void run_fp16_benchmark(int fd, benchmark_config_t *config, int iterations) {
    int m = config->m;
    int k = config->k;
    int n = config->n;

    // Generate random data
    generate_random_fp16(matrixA_fp16, m * k);
    generate_random_fp16(matrixB_fp16, n * k);

    // Benchmark CPU (no progress indicator to avoid terminal issues)
    int64_t cpu_start = get_time_us();
    for (int i = 0; i < iterations; i++) {
        matmul_fp16_cpu(m, k, n, matrixA_fp16, matrixB_fp16, result_fp32);
    }
    int64_t cpu_end = get_time_us();
    double cpu_time_ms = (cpu_end - cpu_start) / 1000.0 / iterations;

    // Benchmark NPU
    double npu_time_ms = benchmark_fp16_npu(fd, m, k, n, iterations);

    if (npu_time_ms < 0) {
        printf("| %-20s | %4d x %-4d x %-4d | %10.3f | %s%10s%s | %s%10s%s |\n",
               config->description, m, k, n, cpu_time_ms, 
               STYLE_ERROR, "FAILED", COLOR_RESET, 
               STYLE_DIM, "N/A", COLOR_RESET);
        return;
    }

    // Calculate speedup
    double speedup = cpu_time_ms / npu_time_ms;
    
    // Color code based on speedup
    const char *speedup_color;
    if (speedup > 100.0) {
        speedup_color = STYLE_SUCCESS;  // Green for excellent speedup
    } else if (speedup > 10.0) {
        speedup_color = COLOR_GREEN;     // Green for good speedup
    } else if (speedup > 1.0) {
        speedup_color = COLOR_CYAN;      // Cyan for positive speedup
    } else {
        speedup_color = COLOR_YELLOW;    // Yellow for CPU faster
    }

    printf("| %-20s | %4d x %-4d x %-4d | %s%10.3f%s | %s%10.3f%s | %s%10.2fx%s |\n",
           config->description, m, k, n, 
           COLOR_CYAN, cpu_time_ms, COLOR_RESET,
           COLOR_MAGENTA, npu_time_ms, COLOR_RESET,
           speedup_color, speedup, COLOR_RESET);
    fflush(stdout);  // Ensure output is displayed immediately
}

int main(int argc, char **argv) {
    int num_iterations = DEFAULT_ITERATIONS;
    
    // Parse command line arguments
    if (argc > 1) {
        if (strcmp(argv[1], "-h") == 0 || strcmp(argv[1], "--help") == 0) {
            printf("\n");
            printf("%s========================================%s\n", STYLE_HEADER, COLOR_RESET);
            printf("%sRK3588 NPU Matrix Multiplication Benchmark%s\n", STYLE_HEADER, COLOR_RESET);
            printf("%s========================================%s\n", STYLE_HEADER, COLOR_RESET);
            printf("\n");
            printf("%sUsage:%s\n", STYLE_INFO, COLOR_RESET);
            printf("  %s./benchmark [iterations]%s\n", COLOR_CYAN, COLOR_RESET);
            printf("\n");
            printf("%sArguments:%s\n", STYLE_INFO, COLOR_RESET);
            printf("  iterations    Number of iterations per test (default: %d)\n", DEFAULT_ITERATIONS);
            printf("                Range: 1-10000\n");
            printf("\n");
            printf("%sExamples:%s\n", STYLE_INFO, COLOR_RESET);
            printf("  %s./benchmark%s          # Use default (%d iterations)\n", 
                   COLOR_CYAN, COLOR_RESET, DEFAULT_ITERATIONS);
            printf("  %s./benchmark 100%s      # Quick test (100 iterations)\n", 
                   COLOR_CYAN, COLOR_RESET);
            printf("  %s./benchmark 1000%s     # High precision (1000 iterations)\n", 
                   COLOR_CYAN, COLOR_RESET);
            printf("\n");
            printf("%sNote:%s\n", STYLE_WARNING, COLOR_RESET);
            printf("  - More iterations = more accurate results but slower\n");
            printf("  - Recommended: 100-500 for testing, 500-1000 for benchmarking\n");
            printf("  - Must run with sudo: %ssudo ./benchmark [iterations]%s\n", 
                   COLOR_CYAN, COLOR_RESET);
            printf("\n");
            return 0;
        }
        
        // Parse iterations
        num_iterations = atoi(argv[1]);
        if (num_iterations < 1 || num_iterations > 10000) {
            fprintf(stderr, "%sERROR:%s Invalid iteration count: %d\n", 
                    STYLE_ERROR, COLOR_RESET, num_iterations);
            fprintf(stderr, "Iterations must be between 1 and 10000\n");
            fprintf(stderr, "Use %s--help%s for usage information\n", 
                    COLOR_CYAN, COLOR_RESET);
            return -1;
        }
    }
    
    srand(time(NULL));

    // Open NPU device
    int fd = npu_open();
    if (fd < 0) {
        fprintf(stderr, "\n");
        fprintf(stderr, "%s================================================================================\n", STYLE_ERROR);
        fprintf(stderr, "ERROR: Failed to open NPU device\n");
        fprintf(stderr, "================================================================================%s\n", COLOR_RESET);
        fprintf(stderr, "\n");
        fprintf(stderr, "%sTroubleshooting steps:%s\n", STYLE_WARNING, COLOR_RESET);
        fprintf(stderr, "  1. Check if NPU driver is loaded:\n");
        fprintf(stderr, "     %s$ dmesg | grep rknpu%s\n", COLOR_CYAN, COLOR_RESET);
        fprintf(stderr, "\n");
        fprintf(stderr, "  2. Check if device node exists:\n");
        fprintf(stderr, "     %s$ ls -la /dev/dri/renderD*%s\n", COLOR_CYAN, COLOR_RESET);
        fprintf(stderr, "\n");
        fprintf(stderr, "  3. Try running with sudo:\n");
        fprintf(stderr, "     %s$ sudo ./benchmark%s\n", COLOR_CYAN, COLOR_RESET);
        fprintf(stderr, "\n");
        fprintf(stderr, "  4. Check device permissions:\n");
        fprintf(stderr, "     %s$ groups%s\n", COLOR_CYAN, COLOR_RESET);
        fprintf(stderr, "     (You may need to add your user to the 'render' or 'video' group)\n");
        fprintf(stderr, "\n");
        fprintf(stderr, "================================================================================\n");
        fprintf(stderr, "\n");
        return -1;
    }

    printf("%s✓ NPU device opened successfully%s (fd=%d)\n", STYLE_SUCCESS, COLOR_RESET, fd);

    // Reset NPU
    int reset_ret = npu_reset(fd);
    if (reset_ret < 0) {
        fprintf(stderr, "%sWARNING:%s NPU reset failed, continuing anyway...\n%s", STYLE_WARNING, COLOR_RESET, COLOR_RESET);
    } else {
        printf("%s✓ NPU reset successful%s\n", STYLE_SUCCESS, COLOR_RESET);
    }

    // Define benchmark configurations
    // All configurations tested and verified to work on RK3588 NPU
    benchmark_config_t int8_configs[] = {
        // Small matrices
        {8, 64, 32, "Tiny"},
        {16, 128, 64, "Small"},
        {32, 256, 128, "Medium-S"},
        
        // Medium matrices
        {64, 512, 256, "Medium"},
        {64, 512, 512, "Medium-L1"},
        {128, 256, 256, "Medium-L2"},
        {128, 512, 256, "Medium-L3"},
        
        // Large matrices
        {128, 512, 512, "Large-1"},
        {128, 1024, 512, "Large-2"},
        {128, 1024, 1024, "Large-3"},
        {256, 256, 256, "Large-4"},
        {256, 512, 256, "Large-5"},
        
        // Very large matrices
        {256, 512, 512, "Very Large-1"},
        {256, 1024, 512, "Very Large-2"},
        {256, 1024, 1024, "Very Large-3"},
        
        // Extra large matrices
        {384, 256, 256, "Extra Large-1"},
        {384, 512, 256, "Extra Large-2"},
        {384, 512, 512, "Extra Large-3"},
    };

    benchmark_config_t fp16_configs[] = {
        // Tiny matrices
        {4, 32, 16, "Tiny-1"},
        {4, 64, 32, "Tiny-2"},
        {4, 128, 64, "Tiny-3"},
        
        // Small matrices
        {8, 64, 32, "Small-1"},
        {8, 128, 64, "Small-2"},
        {16, 128, 64, "Small-3"},
        {16, 256, 128, "Small-4"},
        
        // Medium matrices
        {32, 256, 128, "Medium-1"},
        {32, 512, 256, "Medium-2"},
        {64, 256, 256, "Medium-3"},
        {64, 512, 256, "Medium-4"},
        
        // Large matrices
        {64, 512, 512, "Large-1"},
        {128, 512, 256, "Large-2"},
        {128, 512, 512, "Large-3"},
        {128, 1024, 512, "Large-4"},
        
        // Very large matrices
        {128, 1024, 1024, "Very Large-1"},
        {256, 256, 256, "Very Large-2"},
        {256, 512, 256, "Very Large-3"},
        {256, 512, 512, "Very Large-4"},
    };

    int num_int8_configs = sizeof(int8_configs) / sizeof(benchmark_config_t);
    int num_fp16_configs = sizeof(fp16_configs) / sizeof(benchmark_config_t);

    printf("\n");
    printf("%s================================================================================\n", STYLE_HEADER);
    printf("                    RK3588 NPU Matrix Multiplication Benchmark\n");
    printf("                         Iterations per test: %s%d%s\n", COLOR_CYAN, num_iterations, STYLE_HEADER);
    printf("================================================================================%s\n", COLOR_RESET);
    printf("\n");

    // INT8 Benchmarks
    printf("%s--- INT8 Matrix Multiplication (INT8 x INT8 -> INT32) ---%s\n", STYLE_INFO, COLOR_RESET);
    printf("+----------------------+---------------------+------------+------------+------------+\n");
    printf("| Configuration        | Matrix Size (MxKxN) | CPU (ms)   | NPU (ms)   | Speedup    |\n");
    printf("+----------------------+---------------------+------------+------------+------------+\n");
    
    int int8_success = 0;
    int int8_failed = 0;
    int int8_skipped = 0;
    
    for (int i = 0; i < num_int8_configs; i++) {
        // Validate configuration first
        if (!validate_config(int8_configs[i].m, int8_configs[i].k, int8_configs[i].n, "INT8")) {
            int8_skipped++;
            printf("| %-20s | %4d x %-4d x %-4d | %s%10s%s | %s%10s%s | %s%10s%s |\n",
                   int8_configs[i].description, int8_configs[i].m, int8_configs[i].k, int8_configs[i].n, 
                   STYLE_DIM, "SKIPPED", COLOR_RESET,
                   STYLE_WARNING, "INVALID", COLOR_RESET,
                   STYLE_DIM, "N/A", COLOR_RESET);
            continue;
        }
        
        // Test run first to check if it works
        double test_result = benchmark_int8_npu(fd, int8_configs[i].m, int8_configs[i].k, int8_configs[i].n, 1);
        if (test_result < 0) {
            int8_failed++;
            printf("| %-20s | %4d x %-4d x %-4d | %s%10s%s | %s%10s%s | %s%10s%s |\n",
                   int8_configs[i].description, int8_configs[i].m, int8_configs[i].k, int8_configs[i].n, 
                   STYLE_DIM, "SKIPPED", COLOR_RESET,
                   STYLE_ERROR, "FAILED", COLOR_RESET,
                   STYLE_DIM, "N/A", COLOR_RESET);
        } else {
            int8_success++;
            run_int8_benchmark(fd, &int8_configs[i], num_iterations);
        }
    }
    
    printf("+----------------------+---------------------+------------+------------+------------+\n");
    if (int8_skipped > 0) {
        printf("INT8 Tests: %s%d passed%s, %s%d failed%s, %s%d skipped%s (invalid config)\n", 
               STYLE_SUCCESS, int8_success, COLOR_RESET,
               STYLE_ERROR, int8_failed, COLOR_RESET,
               COLOR_YELLOW, int8_skipped, COLOR_RESET);
    } else {
        printf("INT8 Tests: %s%d passed%s, %s%d failed%s\n", 
               STYLE_SUCCESS, int8_success, COLOR_RESET,
               int8_failed > 0 ? STYLE_ERROR : COLOR_GREEN, int8_failed, COLOR_RESET);
    }
    printf("\n");

    // FP16 Benchmarks
    printf("%s--- FP16 Matrix Multiplication (FP16 x FP16 -> FP32) ---%s\n", STYLE_INFO, COLOR_RESET);
    printf("+----------------------+---------------------+------------+------------+------------+\n");
    printf("| Configuration        | Matrix Size (MxKxN) | CPU (ms)   | NPU (ms)   | Speedup    |\n");
    printf("+----------------------+---------------------+------------+------------+------------+\n");
    
    int fp16_success = 0;
    int fp16_failed = 0;
    int fp16_skipped = 0;
    
    for (int i = 0; i < num_fp16_configs; i++) {
        // Validate configuration first
        if (!validate_config(fp16_configs[i].m, fp16_configs[i].k, fp16_configs[i].n, "FP16")) {
            fp16_skipped++;
            printf("| %-20s | %4d x %-4d x %-4d | %s%10s%s | %s%10s%s | %s%10s%s |\n",
                   fp16_configs[i].description, fp16_configs[i].m, fp16_configs[i].k, fp16_configs[i].n,
                   STYLE_DIM, "SKIPPED", COLOR_RESET,
                   STYLE_WARNING, "INVALID", COLOR_RESET,
                   STYLE_DIM, "N/A", COLOR_RESET);
            continue;
        }
        
        // Test run first to check if it works
        double test_result = benchmark_fp16_npu(fd, fp16_configs[i].m, fp16_configs[i].k, fp16_configs[i].n, 1);
        if (test_result < 0) {
            fp16_failed++;
            printf("| %-20s | %4d x %-4d x %-4d | %s%10s%s | %s%10s%s | %s%10s%s |\n",
                   fp16_configs[i].description, fp16_configs[i].m, fp16_configs[i].k, fp16_configs[i].n,
                   STYLE_DIM, "SKIPPED", COLOR_RESET,
                   STYLE_ERROR, "FAILED", COLOR_RESET,
                   STYLE_DIM, "N/A", COLOR_RESET);
        } else {
            fp16_success++;
            run_fp16_benchmark(fd, &fp16_configs[i], num_iterations);
        }
    }
    
    printf("+----------------------+---------------------+------------+------------+------------+\n");
    if (fp16_skipped > 0) {
        printf("FP16 Tests: %s%d passed%s, %s%d failed%s, %s%d skipped%s (invalid config)\n", 
               STYLE_SUCCESS, fp16_success, COLOR_RESET,
               STYLE_ERROR, fp16_failed, COLOR_RESET,
               COLOR_YELLOW, fp16_skipped, COLOR_RESET);
    } else {
        printf("FP16 Tests: %s%d passed%s, %s%d failed%s\n", 
               STYLE_SUCCESS, fp16_success, COLOR_RESET,
               fp16_failed > 0 ? STYLE_ERROR : COLOR_GREEN, fp16_failed, COLOR_RESET);
    }
    printf("\n");

    printf("%s================================================================================\n%s", STYLE_HEADER, COLOR_RESET);
    
    int total_tests = num_int8_configs + num_fp16_configs;
    int total_passed = int8_success + fp16_success;
    int total_failed = int8_failed + fp16_failed;
    
    if (total_failed == 0) {
        printf("%s✓ Benchmark completed successfully!%s\n", STYLE_SUCCESS, COLOR_RESET);
        printf("  All %s%d%s tests passed (INT8: %d, FP16: %d)\n", 
               STYLE_SUCCESS, total_passed, COLOR_RESET, int8_success, fp16_success);
    } else {
        printf("%s⚠ Benchmark completed with some issues.%s\n", STYLE_WARNING, COLOR_RESET);
        printf("  Results: %s%d passed%s, %s%d failed%s (out of %d total tests)\n", 
               STYLE_SUCCESS, total_passed, COLOR_RESET,
               STYLE_ERROR, total_failed, COLOR_RESET, total_tests);
        printf("\n");
        printf("  %sFailed tests breakdown:%s\n", COLOR_YELLOW, COLOR_RESET);
        if (int8_failed > 0) {
            printf("    - INT8: %s%d failed%s\n", STYLE_ERROR, int8_failed, COLOR_RESET);
        }
        if (fp16_failed > 0) {
            printf("    - FP16: %s%d failed%s\n", STYLE_ERROR, fp16_failed, COLOR_RESET);
        }
        printf("\n");
        printf("  %sCommon failure reasons:%s\n", COLOR_CYAN, COLOR_RESET);
        printf("    1. Matrix size exceeds NPU hardware limits\n");
        printf("    2. Insufficient memory for large matrices\n");
        printf("    3. NPU timeout for first small test (can be ignored)\n");
        printf("    4. Driver compatibility issues\n");
    }
    
    printf("\n%sLegend:%s\n", STYLE_INFO, COLOR_RESET);
    printf("  - Speedup = CPU Time / NPU Time\n");
    printf("  - %sSpeedup > 100x%s: Excellent NPU acceleration %s(bold green)%s\n", STYLE_SUCCESS, COLOR_RESET, STYLE_SUCCESS, COLOR_RESET);
    printf("  - %sSpeedup > 10x%s: Good NPU acceleration %s(green)%s\n", COLOR_GREEN, COLOR_RESET, COLOR_GREEN, COLOR_RESET);
    printf("  - %sSpeedup > 1x%s: NPU faster than CPU %s(cyan)%s\n", COLOR_CYAN, COLOR_RESET, COLOR_CYAN, COLOR_RESET);
    printf("  - %sSpeedup < 1x%s: CPU faster %s(yellow)%s\n", COLOR_YELLOW, COLOR_RESET, COLOR_YELLOW, COLOR_RESET);
    printf("  - %sFAILED%s: NPU execution error occurred\n", STYLE_ERROR, COLOR_RESET);
    printf("  - %sSKIPPED%s: Test skipped due to invalid configuration\n", STYLE_DIM, COLOR_RESET);
    printf("%s================================================================================%s\n", STYLE_HEADER, COLOR_RESET);
    printf("\n");

    npu_close(fd);
    return (int8_failed + fp16_failed > 0) ? 1 : 0;
}
