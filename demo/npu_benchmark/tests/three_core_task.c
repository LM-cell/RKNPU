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

#define MAX_M 384 
#define MAX_K 4096 
#define MAX_N 4096 
#define TASK_NUMBER 30 //matmul task number 

// Test currently runs against kernel 5.10 haven't tested 6.1 kernel.

// matrix A max size
_Float16 matrixA[(MAX_M*MAX_K)];

// matrix B max size
_Float16 matrixB[(MAX_N*MAX_K)];

// matrix C max size
float expected_result[MAX_M*MAX_N];

uint64_t npu_regs[112];

static void print_submit_summary(const struct rknpu_submit *submit) {
    printf("submit summary\n");
    printf("  flags         = 0x%x\n", submit->flags);
    printf("  timeout       = %u\n", submit->timeout);
    printf("  task_number   = %u\n", submit->task_number);
    printf("  task_counter  = %u\n", submit->task_counter);
    printf("  priority      = %d\n", submit->priority);
    printf("  task_obj_addr = 0x%016llx\n", (unsigned long long)submit->task_obj_addr);
    printf("  task_base_addr= 0x%016llx\n", (unsigned long long)submit->task_base_addr);
    printf("  core_mask     = 0x%x\n", submit->core_mask);
    for (int i = 0; i < 3; i++) {
        printf("  subcore[%d]    = start=%u number=%u\n",
               i,
               submit->subcore_task[i].task_start,
               submit->subcore_task[i].task_number);
    }
}

static void print_task_descriptor_summary(
    const struct rknpu_task *tasks,
    int task_number,
    uint64_t regcmd_dma,
    size_t single_output_bytes,
    uint64_t output_dma_base
) {
    printf("task descriptor summary\n");
    for (int i = 0; i < task_number; i++) {
        uint64_t output_dma = output_dma_base + (uint64_t)i * single_output_bytes;
        printf("  task[%02d] op=%u enable=0x%x int_mask=0x%x int_status=0x%x regcfg=%u regcmd=0x%016llx output_dma=0x%016llx\n",
               i,
               tasks[i].op_idx,
               tasks[i].enable_mask,
               tasks[i].int_mask,
               tasks[i].int_status,
               tasks[i].regcfg_amount,
               (unsigned long long)tasks[i].regcmd_addr,
               (unsigned long long)output_dma);
        if (tasks[i].regcmd_addr != regcmd_dma + (uint64_t)i * sizeof(npu_regs)) {
            printf("    warning: unexpected regcmd addr for task[%d]\n", i);
        }
    }
}

static void print_task_completion_summary(const struct rknpu_task *tasks, int task_number) {
    printf("task completion summary\n");
    for (int i = 0; i < task_number; i++) {
        printf("  task[%02d] int_status=0x%x\n", i, tasks[i].int_status);
    }
}

static void print_output_samples(
    const void *output,
    int task_number,
    size_t single_output_bytes,
    unsigned int M,
    unsigned int N
) {
    printf("output samples\n");
    for (int task_idx = 0; task_idx < task_number; task_idx++) {
        const float *output_data = (const float *)((const uint8_t *)output + task_idx * single_output_bytes);
        int printed = 0;
        int nonzero = 0;
        printf("  task[%02d]:", task_idx);
        for (unsigned int m = 1; m <= M && printed < 4; m++) {
            for (unsigned int n = 1; n <= N && printed < 4; n++) {
                float value = output_data[feature_data(N, M, 1, 4, n, m, 1)];
                printf(" %6.1f", value);
                printed++;
            }
        }

        for (unsigned int m = 1; m <= M; m++) {
            for (unsigned int n = 1; n <= N; n++) {
                float value = output_data[feature_data(N, M, 1, 4, n, m, 1)];
                if (value != 0.0f) {
                    nonzero++;
                }
            }
        }
        printf(" | nonzero_count=%d\n", nonzero);
    }
}

void matmul_fp32(int m, int k, int n, _Float16 *src0 , _Float16 *src1, float* dst) {
    for (int i = 0; i < m; i++) {
        for (int j = 0; j < n; j++) {
        float sum = 0;
        for (int l = 0; l < k; l++) {
            sum += src0[i*k + l] * src1[j*k + l];
        }
        dst[i*n + j] = sum;
        }
    }
}

float rand_float() {
    return rand()/(float)RAND_MAX;
}

static inline int64_t get_time_us(void) {
    struct timeval tv;
    gettimeofday(&tv, NULL);
    return tv.tv_sec * 1000000LL + tv.tv_usec;
}

int main(int argc, char **argv) {
    unsigned int M=0;
    unsigned int K=0;
    unsigned int N=0;

    int gen_ret[TASK_NUMBER];
    int exit_code = 0;
    memset(gen_ret, 0, sizeof(gen_ret));

    if (argc !=4) {
        printf("Invalid number of args %d, needs to supply M K N ie matmul_fp16 <M> <K> <N>\n",argc);
        return -1; 
    }

    M = atoi(argv[1]);
    K = atoi(argv[2]);
    N = atoi(argv[3]);

    if ((M<=0) || (M>MAX_M) | (((M%4)!=0) && (M!=1))) {
        printf("M [%d] is out of range or not a mutliple of 4 \n",M);
        return -1;
    }

    if ((K<=0) || (K>MAX_K) || ((K%32) != 0)) {
        printf("K [%d] is out of range or not a mutliple of 32\n",K);
        return -1;
    }

    if ((N<=0) || (N>MAX_N) || ((N%16) != 0)) {
        printf("N [%d] is out of range or not a mutliple of 16\n",N);
        return -1;
    }

    // Open DRI called "rknpu"
    int fd = npu_open();

    size_t regcmd_bytes = sizeof(npu_regs) * TASK_NUMBER;
    size_t tasks_bytes = sizeof(struct rknpu_task) * TASK_NUMBER;
    size_t input_bytes = M*K*sizeof(_Float16);
    size_t weights_bytes = N*K*sizeof(_Float16);
    size_t single_output_bytes = M*N*sizeof(float);
    size_t total_output_bytes = single_output_bytes * TASK_NUMBER;

    uint64_t regcmd_dma, regcmd_obj;
    uint32_t regcmd_handle;
    uint64_t *regcmd = mem_allocate(fd, regcmd_bytes, &regcmd_dma, &regcmd_obj, 0, &regcmd_handle);

    uint64_t tasks_dma, tasks_obj;
    uint32_t tasks_handle;
    struct rknpu_task *tasks = mem_allocate(fd, tasks_bytes, &tasks_dma, &tasks_obj, RKNPU_MEM_KERNEL_MAPPING, &tasks_handle);

    uint64_t input_dma, input_obj;
    uint32_t input_handle;
    void *input = mem_allocate(fd, input_bytes, &input_dma, &input_obj, 0, &input_handle);

    uint64_t weights_dma, weights_obj;
    uint32_t weights_handle;
    void *weights = mem_allocate(fd, weights_bytes, &weights_dma, &weights_obj, 0, &weights_handle);

    uint64_t output_dma_base, output_obj;
    uint32_t output_handle;
    void *output = mem_allocate(fd, total_output_bytes, &output_dma_base, &output_obj, 0, &output_handle);

    printf("buffer allocation summary\n");
    printf("  regcmd  : handle=%u obj=0x%016llx dma=0x%016llx size=0x%zx map=%p\n",
           regcmd_handle, (unsigned long long)regcmd_obj, (unsigned long long)regcmd_dma, regcmd_bytes, (void *)regcmd);
    printf("  tasks   : handle=%u obj=0x%016llx dma=0x%016llx size=0x%zx map=%p\n",
           tasks_handle, (unsigned long long)tasks_obj, (unsigned long long)tasks_dma, tasks_bytes, (void *)tasks);
    printf("  input   : handle=%u obj=0x%016llx dma=0x%016llx size=0x%zx map=%p\n",
           input_handle, (unsigned long long)input_obj, (unsigned long long)input_dma, input_bytes, input);
    printf("  weights : handle=%u obj=0x%016llx dma=0x%016llx size=0x%zx map=%p\n",
           weights_handle, (unsigned long long)weights_obj, (unsigned long long)weights_dma, weights_bytes, weights);
    printf("  output  : handle=%u obj=0x%016llx dma=0x%016llx size=0x%zx map=%p\n",
           output_handle, (unsigned long long)output_obj, (unsigned long long)output_dma_base, total_output_bytes, output);
    if ((regcmd == NULL) || (tasks == NULL) || (input == NULL) || (weights == NULL) || (output == NULL)) {
        printf("Failed to allocate memory \n");
        exit(1);
    }

    // Reset the NPU
    npu_reset(fd);

    /*
     * Reuse the same input and weight buffers for every task. They are read-only
     * from the NPU's perspective. Each task gets its own output DMA slice so
     * the 30 matmul results do not overwrite each other.
     */
    for (int i = 0; i < TASK_NUMBER; i++) {
        uint64_t task_output_dma = output_dma_base + i * single_output_bytes;
        matmul_params_t params;
        params.m = M;
        params.k = K;
        params.n = N;
        params.input_dma = input_dma;
        params.weights_dma = weights_dma;
        params.output_dma = task_output_dma;
        params.tasks = (uint64_t *) &npu_regs;
        params.fp32tofp16 = 0;
        gen_ret[i] = gen_matmul_fp16(&params);
        if (gen_ret[i] !=0) {
            printf("gen_matmul_fp16 failed %d\n",gen_ret[i]);
            exit_code = gen_ret[i];
            goto cleanup;
        }
        memcpy((uint8_t* )regcmd + i* sizeof(npu_regs),npu_regs,sizeof(npu_regs));
        tasks[i].flags  = 0;
        tasks[i].op_idx = i;
        tasks[i].enable_mask = 0xd;
        tasks[i].int_mask = 0x300; // wait for DPU to finish
        tasks[i].int_clear = 0x1ffff;
        tasks[i].int_status =0;
        tasks[i].regcfg_amount = sizeof(npu_regs)/sizeof(uint64_t)-(RKNPU_PC_DATA_EXTRA_AMOUNT+4);
        tasks[i].regcfg_offset = 0;
        tasks[i].regcmd_addr = regcmd_dma + i * sizeof(npu_regs);
    }
    
    



    memset((void *)input,0,input_bytes);
    memset((void *)weights,0,weights_bytes);
    memset((void *)output,0,total_output_bytes);

    srand(time(NULL));

    // Need to use whole numbers for now as decimals return a slighty 
    // different result compared to ARM float calculations. Hence Rockchip
    // examples don't perform a exact comparison between expected and acutal
    // results from the matrix mutlipcation for fp16. Need to know why?
    for (int i = 0; i < M*K; i++) {
        matrixA[i] = (int)(10.0*rand_float()); 
    }
    
    for (int i = 0; i < N*K; i++) {
        matrixB[i] = (int)(10.0*rand_float());
    }

    _Float16 *weights_fp16 = weights;
    
    for(int n=1;n<=N;n++) {
        for(int k=1;k<=K;k++) {
        weights_fp16[weight_fp16(K,n,k)]= matrixB[((n-1)*K)+(k-1)];
        }
    }
    
    _Float16 *feature_data_fp16 = (_Float16*) input;

    for (int m=1;m<=M;m++) {
        for (int k=1;k<=K;k++) {
        feature_data_fp16[feature_data(K,M,1,8,k,m,1)]= matrixA[((m-1)*K)+(k-1)];
        }
    }

    matmul_fp32(M,K,N,(_Float16 *)&matrixA, (_Float16 *)&matrixB, (float *)&expected_result);

    struct rknpu_submit submit = {0};
    submit.flags = RKNPU_JOB_PC | RKNPU_JOB_BLOCK | RKNPU_JOB_PINGPONG;
    submit.timeout = 6000;
    submit.task_start = 0;
    submit.task_number = TASK_NUMBER;
    submit.task_counter = 0;
    submit.priority = 0;
    submit.task_obj_addr = tasks_obj;
    submit.regcfg_obj_addr = 0;
    /*
     * Keep the legacy zero task_base_addr contract used by the other
     * benchmark demos. The current driver queue path programs one task at a
     * time from the shadow descriptor and preserves zero here for
     * compatibility. Passing tasks_dma here can make the submission return
     * successfully while producing zero output on the current stack.
     */
    submit.task_base_addr = 0;
    submit.user_data = 0;
    submit.core_mask = 0x7;
    submit.fence_fd = -1;

    submit.subcore_task[0].task_start = 0;
    submit.subcore_task[0].task_number = TASK_NUMBER / 3;

    submit.subcore_task[1].task_start = submit.subcore_task[0].task_number;
    submit.subcore_task[1].task_number = TASK_NUMBER / 3;

    submit.subcore_task[2].task_start =
    submit.subcore_task[1].task_start + submit.subcore_task[1].task_number;
    submit.subcore_task[2].task_number = TASK_NUMBER - submit.subcore_task[2].task_start;

    printf("geometry summary\n");
    printf("  logical dims  : M=%u K=%u N=%u\n", M, K, N);
    printf("  task number   : %d\n", TASK_NUMBER);
    printf("  regcmd bytes  : 0x%zx\n", regcmd_bytes);
    printf("  task bytes    : 0x%zx\n", tasks_bytes);
    printf("  output/task   : 0x%zx\n", single_output_bytes);
    print_submit_summary(&submit);
    print_task_descriptor_summary(tasks, TASK_NUMBER, regcmd_dma, single_output_bytes, output_dma_base);

    int64_t submit_start_us = get_time_us();
    int npu_ret = ioctl(fd, DRM_IOCTL_RKNPU_SUBMIT, &submit);
    int64_t submit_end_us = get_time_us();
    int64_t submit_elapsed_us = submit_end_us - submit_start_us;

    printf("RKNPU_SUBMIT returned %d\n", npu_ret);
    printf("Submit blocking time: %lld us (%.3f ms)\n",
           (long long)submit_elapsed_us,
           (double)submit_elapsed_us / 1000.0);
    print_submit_summary(&submit);
    print_task_completion_summary(tasks, TASK_NUMBER);
    print_output_samples(output, TASK_NUMBER, single_output_bytes, M, N);
    if (npu_ret <0) {
        exit_code = npu_ret;
        goto cleanup;
    }

    printf("=========================================================================================================\n");
    float *baseline_output = (float *)output;
    for (int task_idx = 0; task_idx < TASK_NUMBER; task_idx++) {
        float *output_data = (float *)((uint8_t *)output + task_idx * single_output_bytes);
        for (int m=1;m<=M;m++) {
            for (int n=1;n<=N;n++) {
                float actual = output_data[feature_data(N, M, 1, 4, n, m, 1)];
                float expected = expected_result[((m-1)*N)+(n-1)];
                float baseline = baseline_output[feature_data(N, M, 1, 4, n, m, 1)];
                int32_t *e, *a;
                int32_t *b;
                e = (int32_t *)&expected;
                a = (int32_t *)&actual;
                b = (int32_t *)&baseline;
                if (actual != expected) {
                    printf("\ntask:%d mismatch m:%d n:%d  expected:%6.5f acutal:%6.5f %x %x\n",
                           task_idx, m, n, expected, actual, *e, *a);
                    npu_ret = -1;
                }
                if (task_idx > 0 && actual != baseline) {
                    printf("\ntask:%d differs from task:0 at m:%d n:%d baseline:%6.5f actual:%6.5f %x %x\n",
                           task_idx, m, n, baseline, actual, *b, *a);
                    npu_ret = -1;
                }
            }
        }
    }
    if (npu_ret == 0) {
        printf("Multiplication of [%d,%d] x [%d,%d] successful for %d tasks\n",M,K,N,K, TASK_NUMBER);
        printf("All %d task outputs match the CPU reference and are identical to task 0\n", TASK_NUMBER);
    } else {
        exit_code = npu_ret;
    }
    printf("=========================================================================================================\n");

    cleanup:
    munmap(regcmd,regcmd_bytes);
    munmap(tasks,tasks_bytes);
    munmap(input,input_bytes);
    munmap(weights,weights_bytes);
    munmap(output,total_output_bytes);

    mem_destroy(fd, regcmd_handle, regcmd_obj);
    mem_destroy(fd, tasks_handle, tasks_obj );
    mem_destroy(fd, input_handle, input_obj);
    mem_destroy(fd, weights_handle, weights_obj);
    mem_destroy(fd, output_handle, output_obj);

    npu_close(fd);
    return exit_code;
}
