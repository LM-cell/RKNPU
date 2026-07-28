/*
 * 多进程 RKNPU 单次提交链式压测。
 *
 * 每个子进程只发 1 次 submit，但这次 submit 里面串着 20 个相关联的
 * FP16 matmul task。第 i 个 task 的输出会成为第 i+1 个 task 的输入。
 *
 * 因而这个 demo 验证的是：
 * - 驱动能否在“单个 task 完成 IRQ 边界”切走当前 owner；
 * - 切回来后能否继续推进同一次 submit 里剩余的 task；
 * - 最终所有链式结果是否仍与 CPU 参考一致。
 */

#include <errno.h>
#include <fcntl.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/ioctl.h>
#include <sys/mman.h>
#include <sys/types.h>
#include <sys/wait.h>
#include <unistd.h>

#include <libdrm/drm.h>

#include "npu_interface.h"
#include "npu_matmul.h"
#include "rknpu-ioctl.h"

#define STRESS_CHILDREN 4
#define STRESS_TASKS 20
#define STRESS_M 1
#define STRESS_K 1024
#define STRESS_N 1024
#define DMA_PAGE_SIZE 4096U
#define REGCMD_WORDS 112U
#define REGCMD_STRIDE_BYTES 1024U

static size_t align_up_size(size_t value, size_t align) {
    return (value + align - 1) & ~(align - 1);
}

static size_t task_array_bytes(void) {
    return align_up_size(STRESS_TASKS * sizeof(struct rknpu_task), DMA_PAGE_SIZE);
}

static size_t regcmd_array_bytes(void) {
    return align_up_size(STRESS_TASKS * REGCMD_STRIDE_BYTES, DMA_PAGE_SIZE);
}

static size_t state_slice_bytes(void) {
    return STRESS_M * STRESS_K * sizeof(_Float16);
}

static size_t state_array_bytes(void) {
    return align_up_size((STRESS_TASKS + 1) * state_slice_bytes(), DMA_PAGE_SIZE);
}

static size_t weights_bytes(void) {
    return align_up_size(STRESS_N * STRESS_K * sizeof(_Float16), DMA_PAGE_SIZE);
}

/* 给每个子进程一个不同的初始输入，避免 4 个进程跑出完全相同的轨迹。 */
static void fill_initial_state(int child_idx, _Float16 *logical_state) {
    for (int i = 0; i < STRESS_K; i++) {
        logical_state[i] = (_Float16)((i + child_idx * 7) & 0xf);
    }
}

/*
 * 构造一个“置换矩阵”形式的权重。
 *
 * 这样每一步 matmul 的 CPU 参考值都可以精确写成“向量重排”，
 * 便于在多进程交错执行后仍做严格逐元素校验。
 */
static void fill_permutation_weights(int child_idx, _Float16 *packed_weights, int *perm) {
    int shift = (child_idx * 29 + 5) & (STRESS_K - 1);
    if (shift == 0) {
        shift = 1;
    }

    memset(packed_weights, 0, STRESS_N * STRESS_K * sizeof(_Float16));
    for (int n = 0; n < STRESS_N; n++) {
        perm[n] = (n + shift) & (STRESS_K - 1);
        packed_weights[weight_fp16(STRESS_K, n + 1, perm[n] + 1)] = (_Float16)1.0;
    }
}

/* 把逻辑上一维状态向量按 NPU feature 布局打包进 DMA 缓冲区。 */
static void write_feature_slice(const _Float16 *logical_state, _Float16 *packed_state) {
    memset(packed_state, 0, state_slice_bytes());
    for (int k = 1; k <= STRESS_K; k++) {
        packed_state[feature_data(STRESS_K, STRESS_M, 1, 8, k, 1, 1)] = logical_state[k - 1];
    }
}

/* CPU 侧参考实现：根据置换表计算下一步理论输出。 */
static void apply_permutation_reference(
    const _Float16 *current_state,
    const int *perm,
    _Float16 *next_state
) {
    for (int n = 0; n < STRESS_N; n++) {
        next_state[n] = current_state[perm[n]];
    }
}

/*
 * 构造“单次 submit 内含 20 个 task”的链式工作负载。
 *
 * `state` DMA 缓冲区被切成 21 片：
 * - slice 0 是初始输入；
 * - task i 读取 slice i，写入 slice i + 1；
 * - 整条链跑完后即可逐片比对中间结果。
 */
static int build_task_chain(
    int child_idx,
    uint64_t regcmd_dma,
    uint64_t *regcmd,
    struct rknpu_task *tasks,
    uint64_t state_dma,
    _Float16 *state,
    uint64_t weights_dma,
    _Float16 *weights,
    _Float16 *expected_states,
    int *perm
) {
    fill_initial_state(child_idx, expected_states);
    fill_permutation_weights(child_idx, weights, perm);
    write_feature_slice(expected_states, state);

    memset(tasks, 0, task_array_bytes());
    memset(regcmd, 0, regcmd_array_bytes());

    for (int task_idx = 0; task_idx < STRESS_TASKS; task_idx++) {
        uint64_t npu_regs[REGCMD_WORDS];
        uint8_t *regcmd_bytes = (uint8_t *)regcmd + (task_idx * REGCMD_STRIDE_BYTES);
        matmul_params_t params;

        /* 先把这一轮 task 的 CPU 参考结果推进出来，便于最终逐步校验。 */
        apply_permutation_reference(
            &expected_states[task_idx * STRESS_K],
            perm,
            &expected_states[(task_idx + 1) * STRESS_K]
        );

        params.m = STRESS_M;
        params.k = STRESS_K;
        params.n = STRESS_N;
        params.input_dma = (uint32_t)(state_dma + (task_idx * state_slice_bytes()));
        params.weights_dma = (uint32_t)weights_dma;
        params.output_dma = (uint32_t)(state_dma + ((task_idx + 1) * state_slice_bytes()));
        params.tasks = npu_regs;
        params.fp32tofp16 = 1;

        if (gen_matmul_fp16(&params) != 0) {
            fprintf(stderr, "[matmul-multi] child=%d gen_matmul_fp16 failed at task=%d\n", child_idx, task_idx);
            return -1;
        }

        memcpy(regcmd_bytes, npu_regs, sizeof(npu_regs));

        /* 每个 task 各自指向独立 regcmd 区，但都属于同一次 submit。 */
        tasks[task_idx].flags = 0;
        tasks[task_idx].op_idx = 0;
        tasks[task_idx].enable_mask = 0xd;
        tasks[task_idx].int_mask = 0x300;
        tasks[task_idx].int_clear = 0x1ffff;
        tasks[task_idx].int_status = 0;
        tasks[task_idx].regcfg_amount =
            (sizeof(npu_regs) / sizeof(uint64_t)) - (RKNPU_PC_DATA_EXTRA_AMOUNT + 4);
        tasks[task_idx].regcfg_offset = 0;
        tasks[task_idx].regcmd_addr = regcmd_dma + ((uint64_t)task_idx * REGCMD_STRIDE_BYTES);
    }

    return 0;
}

/* 检查 20 个 task 每一步产出的状态切片都与 CPU 参考一致。 */
static int validate_task_chain(
    int child_idx,
    const _Float16 *state,
    const _Float16 *expected_states
) {
    for (int task_idx = 0; task_idx < STRESS_TASKS; task_idx++) {
        const _Float16 *state_slice = state + ((task_idx + 1) * STRESS_K);
        const _Float16 *expected_slice = expected_states + ((task_idx + 1) * STRESS_K);

        for (int k = 1; k <= STRESS_K; k++) {
            _Float16 actual = state_slice[feature_data(STRESS_K, STRESS_M, 1, 8, k, 1, 1)];
            _Float16 expected = expected_slice[k - 1];

            if (actual != expected) {
                fprintf(
                    stderr,
                    "[matmul-multi] child=%d task=%d elem=%d mismatch actual=%f expected=%f\n",
                    child_idx,
                    task_idx + 1,
                    k,
                    (float)actual,
                    (float)expected
                );
                return -1;
            }
        }
    }

    return 0;
}

/* 每个子进程独占自己的 DMA 资源，避免用户态缓冲区互相踩踏。 */
static int prepare_child_workload(
    int fd,
    uint64_t *regcmd_dma,
    uint64_t *regcmd_obj,
    uint32_t *regcmd_handle,
    uint64_t **regcmd,
    uint64_t *tasks_dma,
    uint64_t *tasks_obj,
    uint32_t *tasks_handle,
    struct rknpu_task **tasks,
    uint64_t *state_dma,
    uint64_t *state_obj,
    uint32_t *state_handle,
    _Float16 **state,
    uint64_t *weights_dma,
    uint64_t *weights_obj,
    uint32_t *weights_handle,
    _Float16 **weights
) {
    *regcmd = mem_allocate(
        fd,
        regcmd_array_bytes(),
        regcmd_dma,
        regcmd_obj,
        0,
        regcmd_handle
    );
    *tasks = mem_allocate(
        fd,
        task_array_bytes(),
        tasks_dma,
        tasks_obj,
        RKNPU_MEM_KERNEL_MAPPING,
        tasks_handle
    );
    *state = mem_allocate(
        fd,
        state_array_bytes(),
        state_dma,
        state_obj,
        0,
        state_handle
    );
    *weights = mem_allocate(
        fd,
        weights_bytes(),
        weights_dma,
        weights_obj,
        0,
        weights_handle
    );

    if (*regcmd == NULL || *tasks == NULL || *state == NULL || *weights == NULL) {
        fprintf(stderr, "[matmul-multi] allocation failed\n");
        return -1;
    }

    memset(*regcmd, 0, regcmd_array_bytes());
    memset(*tasks, 0, task_array_bytes());
    memset(*state, 0, state_array_bytes());
    memset(*weights, 0, weights_bytes());
    return 0;
}

/* 统一释放一个子进程的 DMA 缓冲区与映射。 */
static void cleanup_child_workload(
    int fd,
    uint64_t *regcmd,
    uint64_t regcmd_obj,
    uint32_t regcmd_handle,
    struct rknpu_task *tasks,
    uint64_t tasks_obj,
    uint32_t tasks_handle,
    _Float16 *state,
    uint64_t state_obj,
    uint32_t state_handle,
    _Float16 *weights,
    uint64_t weights_obj,
    uint32_t weights_handle
) {
    if (regcmd != NULL) {
        munmap(regcmd, regcmd_array_bytes());
        mem_destroy(fd, regcmd_handle, regcmd_obj);
    }
    if (tasks != NULL) {
        munmap(tasks, task_array_bytes());
        mem_destroy(fd, tasks_handle, tasks_obj);
    }
    if (state != NULL) {
        munmap(state, state_array_bytes());
        mem_destroy(fd, state_handle, state_obj);
    }
    if (weights != NULL) {
        munmap(weights, weights_bytes());
        mem_destroy(fd, weights_handle, weights_obj);
    }
}

/*
 * 单个子进程流程：
 * 1. 构造 20 个相关 task 的单次 submit
 * 2. 发 1 次 ioctl
 * 3. 等驱动在 task 边界跨进程交错推进
 * 4. 最后逐片检查链式结果
 */
static int child_main(int child_idx) {
    int fd = -1;
    uint64_t regcmd_dma = 0, regcmd_obj = 0;
    uint32_t regcmd_handle = 0;
    uint64_t *regcmd = NULL;
    uint64_t tasks_dma = 0, tasks_obj = 0;
    uint32_t tasks_handle = 0;
    struct rknpu_task *tasks = NULL;
    uint64_t state_dma = 0, state_obj = 0;
    uint32_t state_handle = 0;
    _Float16 *state = NULL;
    uint64_t weights_dma = 0, weights_obj = 0;
    uint32_t weights_handle = 0;
    _Float16 *weights = NULL;
    _Float16 *expected_states = NULL;
    int *perm = NULL;
    int ret = -1;

    fd = npu_open();
    if (fd < 0) {
        fprintf(stderr, "[matmul-multi] child=%d failed to open NPU: errno=%d\n", child_idx, errno);
        return -1;
    }

    if (prepare_child_workload(
            fd,
            &regcmd_dma,
            &regcmd_obj,
            &regcmd_handle,
            &regcmd,
            &tasks_dma,
            &tasks_obj,
            &tasks_handle,
            &tasks,
            &state_dma,
            &state_obj,
            &state_handle,
            &state,
            &weights_dma,
            &weights_obj,
            &weights_handle,
            &weights) != 0) {
        goto cleanup;
    }

    expected_states = malloc((STRESS_TASKS + 1) * STRESS_K * sizeof(_Float16));
    perm = malloc(STRESS_N * sizeof(int));
    if (expected_states == NULL || perm == NULL) {
        fprintf(stderr, "[matmul-multi] child=%d host allocation failed\n", child_idx);
        goto cleanup;
    }

    if (build_task_chain(
            child_idx,
            regcmd_dma,
            regcmd,
            tasks,
            state_dma,
            state,
            weights_dma,
            weights,
            expected_states,
            perm) != 0) {
        goto cleanup;
    }

    /* 整个压测的关键点：每个子进程只发这 1 次 submit，但里面有 20 个 task。 */
    struct rknpu_submit submit = {
        .flags = RKNPU_JOB_PC | RKNPU_JOB_BLOCK | RKNPU_JOB_PINGPONG,
        .timeout = 6000,
        .task_start = 0,
        .task_number = STRESS_TASKS,
        .task_counter = 0,
        .priority = 0,
        .task_obj_addr = tasks_obj,
        .regcfg_obj_addr = 0,
        .task_base_addr = 0,
        .user_data = 0,
        .core_mask = 0x1,
        .fence_fd = -1,
        .subcore_task = {
            { .task_start = 0, .task_number = STRESS_TASKS },
            { 0, 0 }, { 0, 0 }, { 0, 0 }, { 0, 0 }
        },
    };

    if (ioctl(fd, DRM_IOCTL_RKNPU_SUBMIT, &submit) < 0) {
        fprintf(
            stderr,
            "[matmul-multi] child=%d submit failed: errno=%d (%s)\n",
            child_idx,
            errno,
            strerror(errno)
        );
        goto cleanup;
    }

    if (validate_task_chain(child_idx, state, expected_states) != 0) {
        goto cleanup;
    }

    ret = 0;

cleanup:
    free(expected_states);
    free(perm);
    cleanup_child_workload(
        fd,
        regcmd,
        regcmd_obj,
        regcmd_handle,
        tasks,
        tasks_obj,
        tasks_handle,
        state,
        state_obj,
        state_handle,
        weights,
        weights_obj,
        weights_handle
    );
    if (fd >= 0) {
        npu_close(fd);
    }
    return ret;
}

int main(void) {
    pid_t children[STRESS_CHILDREN];
    int parent_fd = npu_open();
    int ret = 0;

    for (int child_idx = 0; child_idx < STRESS_CHILDREN; child_idx++) {
        children[child_idx] = -1;
    }

    if (parent_fd >= 0) {
        /*
         * 并发压测前只由父进程统一 reset 一次。
         * 子进程绝不能各自 reset，否则会直接清掉别的进程尚未完成的 task 链，
         * 整个“边界切换是否正确”的测试就失去意义。
         */
        npu_reset(parent_fd);
        npu_close(parent_fd);
    }

    printf(
        "matmul_multi_process: children=%d tasks=%d shape=(%dx%d)x(%dx%d)\n",
        STRESS_CHILDREN,
        STRESS_TASKS,
        STRESS_M,
        STRESS_K,
        STRESS_N,
        STRESS_K
    );

    for (int child_idx = 0; child_idx < STRESS_CHILDREN; child_idx++) {
        pid_t pid = fork();
        if (pid < 0) {
            fprintf(stderr, "[matmul-multi] fork failed at child=%d: errno=%d\n", child_idx, errno);
            ret = 1;
            break;
        }
        if (pid == 0) {
            _exit(child_main(child_idx) == 0 ? 0 : 1);
        }
        children[child_idx] = pid;
    }

    /* 父进程只做收尾判定：任一子进程失败，整次压测即失败。 */
    for (int child_idx = 0; child_idx < STRESS_CHILDREN; child_idx++) {
        int status = 0;
        if (children[child_idx] <= 0) {
            ret = 1;
            continue;
        }
        if (waitpid(children[child_idx], &status, 0) < 0) {
            fprintf(stderr, "[matmul-multi] waitpid failed for child=%d: errno=%d\n", child_idx, errno);
            ret = 1;
            continue;
        }
        if (!WIFEXITED(status) || WEXITSTATUS(status) != 0) {
            fprintf(stderr, "[matmul-multi] child=%d failed status=0x%x\n", child_idx, status);
            ret = 1;
        }
    }

    if (ret == 0) {
        printf("matmul_multi_process: pass\n");
    }
    return ret;
}
