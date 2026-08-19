/*
 * RKNPU 多线程并发 Submit 测试，最后修改日期：2026-08-12。
 *
 * 每个 pthread 持有一组独立 GEM，每轮通过屏障同时发起一个 blocking Submit。
 * 测试复用 core_scaling_benchmark 的七个场景组合，并把每个 Submit 的 Task 按
 * 三个可用核心均匀分配到逻辑 lane。
 *
 * --fd-mode shared     ：所有线程共享一个设备 fd；
 * --fd-mode per-thread ：每个线程独立打开设备，但仍使用同一硬件和同一调度器。
 *
 * 测试验证 ioctl 返回、Task 完成计数、IRQ 状态和抽样计算结果，并统计 blocking
 * ioctl 延迟及每轮并发 Submit 活跃窗口内的吞吐量。
 */

#include <errno.h>
#include <pthread.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/ioctl.h>
#include <time.h>

#include "npu_interface.h"
#include "rknpu-ioctl.h"
#include "rknpu_performance_report.h"
#include "rknpu_scenario_workload.h"

#define DEFAULT_THREADS 3U
#define DEFAULT_ROUNDS 100U
#define DEFAULT_WARMUP_ROUNDS 2U
#define NPU_CORES 3U
#define CORE_MASK 0x7U
#define MAX_TEST_THREADS 64U

/* 编译期锁定用户态与驱动 ABI 尺寸，禁止结构变化后静默使用错误 ioctl 大小。 */
_Static_assert(sizeof(struct rknpu_task) == 40, "RKNPU task ABI must be 40 bytes");
_Static_assert(sizeof(struct rknpu_submit) == 104, "RKNPU submit ABI must be 104 bytes");

/* 主线程与 worker 之间按轮同步，并记录本轮并发 ioctl 活跃窗口。 */
typedef struct {
    /* 保护本结构全部字段，并与 cond 组成条件变量谓词。 */
    pthread_mutex_t mutex;
    pthread_cond_t cond;
    /* 参与每轮屏障的固定 worker 数。 */
    uint32_t expected_threads;
    /* 已到达本轮开始屏障的 worker 数。 */
    uint32_t ready_threads;
    /* 已结束本轮协议的 worker 数，包括已发生错误的 worker。 */
    uint32_t done_threads;
    /* 主线程每放行一轮递增一次，防止条件变量虚假唤醒越过屏障。 */
    uint32_t generation;
    /* 本轮最早进入 blocking ioctl 的用户态时间。 */
    uint64_t first_submit_start_us;
    /* 本轮最晚从 blocking ioctl 返回的用户态时间。 */
    uint64_t last_submit_end_us;
    /* 本轮成功完成并参与吞吐量统计的 Submit 数。 */
    uint32_t completed_submits;
    /* 创建失败或主流程退出时终止全部 worker。 */
    int stop;
} round_gate_t;

/* 保存每个 worker 的首个失败类型，后续不再覆盖原始定位信息。 */
typedef enum {
    WORKER_OK = 0,
    WORKER_IOCTL_FAILED,
    WORKER_TASK_COUNTER_FAILED,
    WORKER_IRQ_STATUS_FAILED,
    WORKER_OUTPUT_FAILED,
} worker_error_t;

/*
 * fd 模式只改变 worker 持有设备文件描述符的方式，不改变线程数、Submit 数量、
 * Task 内容、priority、core_mask、轮次屏障或性能计时边界。
 */
typedef enum {
    /* 所有 worker 复用一次 npu_open() 返回的同一个 fd。 */
    FD_MODE_SHARED = 0,
    /* 每个 worker 分别调用一次 npu_open()，各自持有并关闭自己的 fd。 */
    FD_MODE_PER_THREAD,
} fd_mode_t;

typedef struct {
    /* 线程编号，同时用于生成线程独有的矩阵数据和 task.op_idx。 */
    uint32_t thread_id;
    /* shared 模式为公共 fd，per-thread 模式为该线程独占 fd。 */
    int fd;
    /* 不进入性能结果的完整执行轮数。 */
    uint32_t warmup_rounds;
    /* 进入延迟与吞吐量统计的轮数。 */
    uint32_t measure_rounds;
    /* 指向所有线程共享的轮次屏障。 */
    round_gate_t *gate;
    /* 当前场景和该线程独占的 GEM、矩阵及 Submit 模板。 */
    const rknpu_scenario_case_t *scenario;
    rknpu_scenario_workload_t *workload;
    /* 每个正式轮的 blocking ioctl 延迟，单位为微秒。 */
    uint64_t *latency_samples_us;
    size_t latency_sample_count;
    /* 预热和正式轮中成功完成并通过校验的 Submit 总数。 */
    uint32_t successful_submits;
    /* 以下字段保存首个错误及其现场。 */
    worker_error_t error;
    uint32_t error_round;
    int error_errno;
    uint32_t error_task_counter;
    uint32_t error_task_index;
    uint32_t error_irq_status;
    uint32_t error_row;
    uint32_t error_col;
    float error_expected;
    float error_actual;
} worker_context_t;

typedef struct {
    /* 并发 Submit 线程数。 */
    uint32_t threads;
    /* 正式测量轮数。 */
    uint32_t rounds;
    /* 预热轮数。 */
    uint32_t warmup_rounds;
    /* shared 为原测试行为；per-thread 仅用于对照 fd/文件对象相关路径。 */
    fd_mode_t fd_mode;
    /* 完整场景名称或 all，默认执行全部七个组合。 */
    const char *scenario_filter;
} test_options_t;

/* 返回命令行和测试配置输出使用的稳定模式名称。 */
static const char *fd_mode_name(fd_mode_t mode) {
    return mode == FD_MODE_PER_THREAD ? "per-thread" : "shared";
}

/*
 * 在线程资源创建和性能计时开始前打开全部设备 fd。
 *
 * shared 模式只打开一次，再把同一个 fd 赋给所有 worker；per-thread 模式为每个
 * worker 分别打开一次。函数先把全部 fd 设为 -1，使部分打开失败后的清理不会
 * 把 calloc 得到的 0 误认为有效设备 fd。
 */
static int open_worker_fds(
    worker_context_t *workers,
    uint32_t thread_count,
    fd_mode_t mode
) {
    for (uint32_t index = 0; index < thread_count; index++) {
        workers[index].fd = -1;
    }

    if (mode == FD_MODE_SHARED) {
        int shared_fd = npu_open();

        if (shared_fd < 0) {
            return -1;
        }
        for (uint32_t index = 0; index < thread_count; index++) {
            workers[index].fd = shared_fd;
        }
        return 0;
    }

    for (uint32_t index = 0; index < thread_count; index++) {
        workers[index].fd = npu_open();
        if (workers[index].fd < 0) {
            return -1;
        }
    }
    return 0;
}

/*
 * 关闭 worker 持有的设备 fd；调用方必须先 join 全部线程并释放对应 DMA 资源。
 *
 * shared 模式只关闭 worker[0] 保存的公共 fd，避免重复 close；per-thread 模式逐个
 * 关闭独立 fd。清零为 -1 后，后续统一清理路径可以安全重复经过该函数。
 */
static void close_worker_fds(
    worker_context_t *workers,
    uint32_t thread_count,
    fd_mode_t mode
) {
    if (workers == NULL || thread_count == 0U) {
        return;
    }

    if (mode == FD_MODE_SHARED) {
        if (workers[0].fd >= 0) {
            npu_close(workers[0].fd);
        }
        for (uint32_t index = 0; index < thread_count; index++) {
            workers[index].fd = -1;
        }
        return;
    }

    for (uint32_t index = 0; index < thread_count; index++) {
        if (workers[index].fd >= 0) {
            npu_close(workers[index].fd);
            workers[index].fd = -1;
        }
    }
}

/* 读取用户态 CLOCK_MONOTONIC，作为 blocking ioctl 延迟和吞吐窗口的统一时钟。 */
static uint64_t now_us(void) {
    struct timespec ts;

    clock_gettime(CLOCK_MONOTONIC, &ts);
    return (uint64_t)ts.tv_sec * 1000000ULL + (uint64_t)ts.tv_nsec / 1000ULL;
}

/* 释放场景模块为该 worker 创建的全部 GEM。 */
static void release_worker_resources(worker_context_t *worker) {
    rknpu_scenario_workload_destroy(worker->workload);
    worker->workload = NULL;
}

/* 在创建 pthread 前准备当前场景的多 Task 资源和三核 lane。 */
static int prepare_worker_resources(worker_context_t *worker) {
    return rknpu_scenario_workload_create(
        worker->fd,
        worker->thread_id,
        worker->scenario,
        NPU_CORES,
        CORE_MASK,
        &worker->workload
    );
}

/* 初始化按轮屏障；expected_threads 在整个测试期间保持不变。 */
static int round_gate_init(round_gate_t *gate, uint32_t expected_threads) {
    memset(gate, 0, sizeof(*gate));
    gate->expected_threads = expected_threads;

    if (pthread_mutex_init(&gate->mutex, NULL) != 0) {
        return -1;
    }
    if (pthread_cond_init(&gate->cond, NULL) != 0) {
        pthread_mutex_destroy(&gate->mutex);
        return -1;
    }
    return 0;
}

/* 设置全局停止状态并广播，确保等待开始或结束条件的线程都能退出。 */
static void round_gate_stop(round_gate_t *gate) {
    pthread_mutex_lock(&gate->mutex);
    gate->stop = 1;
    pthread_cond_broadcast(&gate->cond);
    pthread_mutex_unlock(&gate->mutex);
}

/* 仅在所有 worker 已 join 后销毁同步对象。 */
static void round_gate_destroy(round_gate_t *gate) {
    pthread_cond_destroy(&gate->cond);
    pthread_mutex_destroy(&gate->mutex);
}

/*
 * worker 到达开始屏障后等待 generation 改变。返回 1 表示执行本轮，返回 0
 * 表示主线程已停止测试。谓词始终在 mutex 下检查，可处理条件变量虚假唤醒。
 */
static int worker_wait_for_round(round_gate_t *gate) {
    uint32_t generation;
    int should_run;

    pthread_mutex_lock(&gate->mutex);
    generation = gate->generation;
    gate->ready_threads++;
    pthread_cond_broadcast(&gate->cond);

    while (!gate->stop && generation == gate->generation) {
        pthread_cond_wait(&gate->cond, &gate->mutex);
    }
    should_run = !gate->stop;
    pthread_mutex_unlock(&gate->mutex);
    return should_run;
}

/* worker 无论成功还是已发生错误都必须报告本轮结束，避免其他线程永久等待。 */
static void worker_finish_round(round_gate_t *gate) {
    pthread_mutex_lock(&gate->mutex);
    gate->done_threads++;
    pthread_cond_broadcast(&gate->cond);
    pthread_mutex_unlock(&gate->mutex);
}

/*
 * 合并本轮成功 Submit 的用户态计时边界。
 * 最早 start 到最晚 end 构成本轮并发 ioctl 活跃窗口，结果校验不在窗口内。
 */
static void worker_record_submit_window(
    round_gate_t *gate,
    uint64_t start_us,
    uint64_t end_us
) {
    pthread_mutex_lock(&gate->mutex);
    if (gate->completed_submits == 0U || start_us < gate->first_submit_start_us) {
        gate->first_submit_start_us = start_us;
    }
    if (gate->completed_submits == 0U || end_us > gate->last_submit_end_us) {
        gate->last_submit_end_us = end_us;
    }
    gate->completed_submits++;
    pthread_mutex_unlock(&gate->mutex);
}

/*
 * 主线程等待所有 worker 就绪，递增 generation 同时放行，再等待全部 worker 结束。
 * 返回该轮成功 Submit 数和并发 ioctl 活跃窗口；函数不执行 NPU 计算。
 */
static int run_synchronized_round(
    round_gate_t *gate,
    uint64_t *submit_window_us,
    uint32_t *completed_submits
) {
    int stopped;

    pthread_mutex_lock(&gate->mutex);
    while (!gate->stop && gate->ready_threads < gate->expected_threads) {
        pthread_cond_wait(&gate->cond, &gate->mutex);
    }
    if (gate->stop) {
        pthread_mutex_unlock(&gate->mutex);
        return -1;
    }

    gate->ready_threads = 0;
    gate->done_threads = 0;
    gate->first_submit_start_us = 0;
    gate->last_submit_end_us = 0;
    gate->completed_submits = 0;
    gate->generation++;
    pthread_cond_broadcast(&gate->cond);

    while (!gate->stop && gate->done_threads < gate->expected_threads) {
        pthread_cond_wait(&gate->cond, &gate->mutex);
    }
    *completed_submits = gate->completed_submits;
    if (gate->completed_submits > 0U &&
        gate->last_submit_end_us >= gate->first_submit_start_us) {
        *submit_window_us = gate->last_submit_end_us - gate->first_submit_start_us;
    } else {
        *submit_window_us = 0;
    }
    stopped = gate->stop;
    pthread_mutex_unlock(&gate->mutex);
    return stopped ? -1 : 0;
}

/*
 * 执行一个 blocking Submit：清理上一轮输出和 IRQ 影子，复制 Submit 模板，
 * 在 ioctl 前后计时，然后依次校验 task_counter、int_status 和矩阵结果。
 * 只有全部校验通过的 Submit 才计入成功数、吞吐窗口和正式轮延迟样本。
 */
static int run_worker_submit(worker_context_t *worker, uint32_t round) {
    struct rknpu_submit submit;
    rknpu_scenario_check_t check;
    uint64_t start_us;
    uint64_t end_us;

    rknpu_scenario_workload_begin(worker->workload, &submit);

    start_us = now_us();
    if (ioctl(worker->fd, DRM_IOCTL_RKNPU_SUBMIT, &submit) < 0) {
        worker->error = WORKER_IOCTL_FAILED;
        worker->error_errno = errno;
        worker->error_round = round;
        return -1;
    }
    end_us = now_us();

    if (rknpu_scenario_workload_check(worker->workload, &submit, &check) != 0) {
        worker->error_round = round;
        worker->error_task_index = check.task_index;
        worker->error_task_counter = check.task_counter;
        worker->error_irq_status = check.irq_status;
        worker->error_row = check.row;
        worker->error_col = check.column;
        worker->error_expected = check.expected;
        worker->error_actual = check.actual;
        if (check.error == RKNPU_SCENARIO_CHECK_TASK_COUNTER) {
            worker->error = WORKER_TASK_COUNTER_FAILED;
        } else if (check.error == RKNPU_SCENARIO_CHECK_IRQ_STATUS) {
            worker->error = WORKER_IRQ_STATUS_FAILED;
        } else {
            worker->error = WORKER_OUTPUT_FAILED;
        }
        return -1;
    }

    worker->successful_submits++;
    worker_record_submit_window(worker->gate, start_us, end_us);
    if (round >= worker->warmup_rounds) {
        worker->latency_samples_us[worker->latency_sample_count++] = end_us - start_us;
    }
    return 0;
}

/*
 * pthread 入口。每轮等待主线程统一放行；某轮失败后不再提交新任务，但继续参加
 * 后续屏障，直到主线程完成全部轮次，避免健康 worker 卡在 done/ready 计数上。
 */
static void *worker_main(void *arg) {
    worker_context_t *worker = arg;
    uint32_t total_rounds = worker->warmup_rounds + worker->measure_rounds;

    for (uint32_t round = 0; round < total_rounds; round++) {
        if (!worker_wait_for_round(worker->gate)) {
            break;
        }

        if (worker->error == WORKER_OK) {
            run_worker_submit(worker, round);
        }

        /* 失败线程继续完成轮次协议，防止其他线程死锁。 */
        worker_finish_round(worker->gate);
    }
    return NULL;
}

/* 把内部错误枚举转换为结果输出中的稳定短名称。 */
static const char *worker_error_name(worker_error_t error) {
    switch (error) {
        case WORKER_OK:
            return "none";
        case WORKER_IOCTL_FAILED:
            return "ioctl";
        case WORKER_TASK_COUNTER_FAILED:
            return "task_counter";
        case WORKER_IRQ_STATUS_FAILED:
            return "irq_status";
        case WORKER_OUTPUT_FAILED:
            return "output";
    }
    return "unknown";
}

/* 输出单线程完成数、正式样本均值和首个错误现场。 */
static void print_worker_result(const worker_context_t *worker, uint32_t total_rounds) {
    uint64_t total_latency_us = 0;
    double mean_us = 0.0;

    for (size_t index = 0; index < worker->latency_sample_count; index++) {
        total_latency_us += worker->latency_samples_us[index];
    }
    if (worker->latency_sample_count > 0U) {
        mean_us = (double)total_latency_us / (double)worker->latency_sample_count;
    }

    printf(
        "thread[%u]: completed=%u/%u measured=%zu mean=%.3f ms error=%s\n",
        worker->thread_id,
        worker->successful_submits,
        total_rounds,
        worker->latency_sample_count,
        mean_us / 1000.0,
        worker_error_name(worker->error)
    );

    if (worker->error == WORKER_IOCTL_FAILED) {
        printf(
            "  first failure: round=%u errno=%d (%s)\n",
            worker->error_round + 1U,
            worker->error_errno,
            strerror(worker->error_errno)
        );
    } else if (worker->error == WORKER_TASK_COUNTER_FAILED) {
        printf(
            "  first failure: round=%u task_counter=%u expected=%u\n",
            worker->error_round + 1U,
            worker->error_task_counter,
            worker->scenario->task_count
        );
    } else if (worker->error == WORKER_IRQ_STATUS_FAILED) {
        printf(
            "  first failure: round=%u task=%u int_status=0x%x expected=0x300\n",
            worker->error_round + 1U,
            worker->error_task_index,
            worker->error_irq_status
        );
    } else if (worker->error == WORKER_OUTPUT_FAILED) {
        printf(
            "  first failure: round=%u task=%u row=%u col=%u expected=%f actual=%f\n",
            worker->error_round + 1U,
            worker->error_task_index,
            worker->error_row,
            worker->error_col,
            worker->error_expected,
            worker->error_actual
        );
    }
}

/* 严格解析十进制 u32；拒绝空串、尾随字符、errno 错误和范围溢出。 */
static int parse_u32(const char *text, uint32_t *value) {
    char *end = NULL;
    unsigned long parsed;

    errno = 0;
    parsed = strtoul(text, &end, 10);
    if (errno != 0 || end == text || *end != '\0' || parsed > UINT32_MAX) {
        return -1;
    }
    *value = (uint32_t)parsed;
    return 0;
}

/* 打印用户态参数，不触发设备访问。 */
static void print_usage(const char *program) {
    size_t scenario_count;
    const rknpu_scenario_case_t *scenarios = rknpu_scenario_cases(&scenario_count);

    printf("Usage: %s [options]\n", program);
    printf("Options:\n");
    printf("  --scenario <name|all>  scenario case, default: all\n");
    printf("  --fd-mode <mode>   shared or per-thread, default: shared\n");
    printf("  --threads <count>  submit threads, default: %u\n", DEFAULT_THREADS);
    printf("  --rounds <count>   measured rounds, default: %u\n", DEFAULT_ROUNDS);
    printf("  --warmup <count>   warmup rounds, default: %u\n", DEFAULT_WARMUP_ROUNDS);
    printf("  --help             show this message\n");
    printf("Scenarios:\n");
    for (size_t index = 0; index < scenario_count; index++) {
        printf("  %-28s M=%u K=%u N=%u tasks=%u\n",
               scenarios[index].name,
               scenarios[index].m,
               scenarios[index].k,
               scenarios[index].n,
               scenarios[index].task_count);
    }
}

/*
 * 解析测试参数。`--fd-mode` 只接受 shared 和 per-thread，防止拼写错误时静默
 * 回退到另一种模式，导致两组性能结果使用了错误的 fd 所有权模型。
 */
static int parse_options(int argc, char **argv, test_options_t *options) {
    options->threads = DEFAULT_THREADS;
    options->rounds = DEFAULT_ROUNDS;
    options->warmup_rounds = DEFAULT_WARMUP_ROUNDS;
    options->fd_mode = FD_MODE_SHARED;
    options->scenario_filter = "all";

    for (int index = 1; index < argc; index++) {
        const char *arg = argv[index];

        if (strcmp(arg, "--help") == 0) {
            print_usage(argv[0]);
            return 1;
        }
        if (strcmp(arg, "--scenario") != 0 &&
            strcmp(arg, "--fd-mode") != 0 &&
            strcmp(arg, "--threads") != 0 &&
            strcmp(arg, "--rounds") != 0 &&
            strcmp(arg, "--warmup") != 0) {
            fprintf(stderr, "unknown option: %s\n", arg);
            print_usage(argv[0]);
            return -1;
        }
        if (++index >= argc) {
            fprintf(stderr, "missing value for %s\n", arg);
            return -1;
        }

        if (strcmp(arg, "--scenario") == 0) {
            if (strcmp(argv[index], "all") != 0 &&
                rknpu_scenario_find(argv[index]) == NULL) {
                fprintf(stderr, "invalid --scenario value: %s\n", argv[index]);
                return -1;
            }
            options->scenario_filter = argv[index];
        } else if (strcmp(arg, "--fd-mode") == 0) {
            if (strcmp(argv[index], "shared") == 0) {
                options->fd_mode = FD_MODE_SHARED;
            } else if (strcmp(argv[index], "per-thread") == 0) {
                options->fd_mode = FD_MODE_PER_THREAD;
            } else {
                fprintf(stderr,
                        "invalid --fd-mode value: %s (valid: shared, per-thread)\n",
                        argv[index]);
                return -1;
            }
        } else if (strcmp(arg, "--threads") == 0) {
            if (parse_u32(argv[index], &options->threads) != 0 ||
                options->threads == 0U || options->threads > MAX_TEST_THREADS) {
                fprintf(stderr, "invalid --threads value: %s (valid: 1-%u)\n",
                        argv[index], MAX_TEST_THREADS);
                return -1;
            }
        } else if (strcmp(arg, "--rounds") == 0) {
            if (parse_u32(argv[index], &options->rounds) != 0 || options->rounds == 0U) {
                fprintf(stderr, "invalid --rounds value: %s\n", argv[index]);
                return -1;
            }
        } else if (parse_u32(argv[index], &options->warmup_rounds) != 0) {
            fprintf(stderr, "invalid --warmup value: %s\n", argv[index]);
            return -1;
        }
    }

    if (UINT32_MAX - options->warmup_rounds < options->rounds) {
        fprintf(stderr, "warmup and measured round count overflow\n");
        return -1;
    }
    return 0;
}

/* all 选择七个组合，指定完整名称时只执行一个组合。 */
static int scenario_selected(const test_options_t *options, const char *name) {
    return strcmp(options->scenario_filter, "all") == 0 ||
        strcmp(options->scenario_filter, name) == 0;
}

/*
 * 程序级流程：解析参数 -> 打开全部 fd 并复位一次 -> 为每线程准备 DMA ->
 * 创建线程 -> 执行同步轮次 -> join -> 汇总指标 -> 释放 DMA -> 关闭 fd。
 * 所有错误路径汇入 cleanup，保持 GEM 必须先于对应 fd 释放的所有权顺序。
 */
static int run_scenario(
    const test_options_t *options,
    const rknpu_scenario_case_t *scenario
) {
    worker_context_t *workers = NULL;
    pthread_t *threads = NULL;
    round_gate_t gate;
    uint32_t prepared_workers = 0;
    uint32_t created_threads = 0;
    uint32_t total_rounds;
    uint64_t measured_submit_us = 0;
    uint64_t setup_operands_us = 0;
    uint64_t build_regcmds_us = 0;
    size_t total_dma_bytes = 0;
    uint64_t *all_samples = NULL;
    size_t all_sample_count = 0;
    size_t sample_offset = 0;
    size_t measured_submit_count = 0;
    size_t expected_measured_count;
    int gate_initialized = 0;
    int result = 1;
    int join_failed = 0;
    int metrics_ok = 0;
    total_rounds = options->warmup_rounds + options->rounds;
    expected_measured_count = (size_t)options->threads * options->rounds;

    /* 2. 分配线程上下文；fd 必须显式初始化为 -1，便于部分失败时安全清理。 */
    workers = calloc(options->threads, sizeof(*workers));
    /*
     * calloc 会把 fd 初始化为 0；必须在任何 cleanup 分支之前改成 -1，避免后续
     * 数组分配失败时把标准输入误当成已经打开的 NPU fd。
     */
    if (workers != NULL) {
        for (uint32_t index = 0; index < options->threads; index++) {
            workers[index].fd = -1;
        }
    }
    threads = calloc(options->threads, sizeof(*threads));
    if (workers == NULL || threads == NULL) {
        fprintf(stderr, "thread context allocation failed\n");
        goto cleanup;
    }

    /* 3. 在线程和计时开始前打开全部 fd，并且整次实验只复位一次 NPU。 */
    /*
     * 两种模式都在 DMA 创建、线程启动和性能计时之前打开全部 fd，open() 时间不会
     * 进入延迟或吞吐量。独立 fd 模式也只使用第一个 fd 执行一次 reset，避免后续
     * reset 清除其他 worker 已准备或正在执行的任务状态。
     */
    if (open_worker_fds(workers, options->threads, options->fd_mode) != 0) {
        fprintf(stderr, "RKNPU fd initialization failed for mode=%s\n",
                fd_mode_name(options->fd_mode));
        goto cleanup;
    }
    if (npu_reset(workers[0].fd) < 0) {
        fprintf(stderr, "RKNPU reset failed: errno=%d (%s)\n", errno, strerror(errno));
        goto cleanup;
    }

    /* 4. 每个线程使用自己的 fd 创建独立 DMA；shared 模式下这些 fd 数值相同。 */
    for (uint32_t index = 0; index < options->threads; index++) {
        workers[index].thread_id = index;
        workers[index].scenario = scenario;
        workers[index].warmup_rounds = options->warmup_rounds;
        workers[index].measure_rounds = options->rounds;

        workers[index].latency_samples_us = calloc(
            options->rounds,
            sizeof(*workers[index].latency_samples_us)
        );

        /* 分配五块 DMA、填充矩阵，并生成该线程的 Task 和寄存器命令。 */
        if (workers[index].latency_samples_us == NULL ||
            prepare_worker_resources(&workers[index]) != 0) {
            fprintf(stderr, "worker[%u] resource preparation failed\n", index);
            goto cleanup;
        }
        {
            rknpu_scenario_metrics_t metrics;
            rknpu_scenario_workload_get_metrics(workers[index].workload, &metrics);
            total_dma_bytes += metrics.total_dma_bytes;
            setup_operands_us += metrics.setup_operands_us;
            build_regcmds_us += metrics.build_regcmds_us;
        }
        prepared_workers++;
    }

    /* 5. 初始化轮次屏障，使每轮多个 Submit 尽量同时进入驱动。 */
    if (round_gate_init(&gate, options->threads) != 0) {
        fprintf(stderr, "round synchronization initialization failed\n");
        goto cleanup;
    }
    gate_initialized = 1;

    /* 6. 输出能够唯一描述本次实验的 fd 模式、线程数、负载和轮数。 */
    printf("rknpu_multithread_submit\n");
    printf("  scenario        : %s\n", scenario->name);
    printf("  operand mode    : %s\n", rknpu_operand_mode_name(scenario->operand_mode));
    printf("  fd mode         : %s\n", fd_mode_name(options->fd_mode));
    printf("  shared fd       : %s\n",
           options->fd_mode == FD_MODE_SHARED ? "yes" : "no");
    printf("  threads         : %u\n", options->threads);
    printf("  workload        : M=%u K=%u N=%u, %u tasks per submit\n",
           scenario->m, scenario->k, scenario->n, scenario->task_count);
    printf("  lanes           : %u, tasks distributed evenly\n", NPU_CORES);
    printf("  core mask       : 0x%x\n", CORE_MASK);
    printf("  rounds          : %u warmup + %u measured\n",
           options->warmup_rounds, options->rounds);

    /* 7. 所有 DMA 就绪后再创建线程，资源准备时间不进入测量。 */
    for (uint32_t index = 0; index < options->threads; index++) {
        workers[index].gate = &gate;
        int thread_error = pthread_create(&threads[index], NULL, worker_main, &workers[index]);
        if (thread_error != 0) {
            fprintf(stderr, "pthread_create failed for worker[%u]: %s\n",
                    index, strerror(thread_error));
            /* 已创建线程可能正在等首轮，必须广播停止后才能 join。 */
            round_gate_stop(&gate);
            goto join_threads;
        }
        created_threads++;
    }

    /* 8. 主线程逐轮放行 worker；预热轮执行完整校验但不累计性能指标。 */
    for (uint32_t round = 0; round < total_rounds; round++) {
        uint64_t round_submit_us = 0;
        uint32_t round_submit_count = 0;

        /* 等待全部就绪、同时放行、等待全部结束，并取得本轮 ioctl 活跃窗口。 */
        if (run_synchronized_round(
                &gate,
                &round_submit_us,
                &round_submit_count
            ) != 0) {
            fprintf(stderr, "round synchronization stopped at round=%u\n", round + 1U);
            goto join_threads;
        }
        if (round >= options->warmup_rounds) {
            measured_submit_us += round_submit_us;
            measured_submit_count += round_submit_count;
        }
    }

/* 9. 正常完成和创建失败都从这里 join 已创建线程。 */
join_threads:
    for (uint32_t index = 0; index < created_threads; index++) {
        int thread_error = pthread_join(threads[index], NULL);
        if (thread_error != 0) {
            fprintf(stderr, "pthread_join failed for worker[%u]: %s\n",
                    index, strerror(thread_error));
            join_failed = 1;
        }
    }

    /* join 失败时无法证明 worker 已停止，不能继续释放其上下文和 DMA。 */
    if (join_failed) {
        return 1;
    }
    if (created_threads != options->threads) {
        goto cleanup;
    }

    /* 10. 汇总每线程正式轮 blocking ioctl 样本。 */
    for (uint32_t index = 0; index < options->threads; index++) {
        print_worker_result(&workers[index], total_rounds);
        all_sample_count += workers[index].latency_sample_count;
    }

    if (all_sample_count != expected_measured_count ||
        measured_submit_count != expected_measured_count) {
        fprintf(stderr,
                "incomplete measured submits: samples=%zu completed=%zu expected=%zu\n",
                all_sample_count, measured_submit_count, expected_measured_count);
    } else {
        all_samples = malloc(all_sample_count * sizeof(*all_samples));
        if (all_samples == NULL) {
            fprintf(stderr, "latency sample allocation failed\n");
            goto cleanup;
        }

        /* 统计接口接收连续数组，因此按线程顺序合并样本。 */
        for (uint32_t index = 0; index < options->threads; index++) {
            memcpy(
                &all_samples[sample_offset],
                workers[index].latency_samples_us,
                workers[index].latency_sample_count * sizeof(*all_samples)
            );
            sample_offset += workers[index].latency_sample_count;
        }

        {
            rknpu_performance_report_t report = {
                .scenario = scenario,
                .npu_cores = NPU_CORES,
                .threads = options->threads,
                .measured_rounds = options->rounds,
                .submit_samples_us = all_samples,
                .submit_sample_count = all_sample_count,
                .successful_submit_count = measured_submit_count,
                .measured_window_us = measured_submit_us,
                .total_dma_bytes = total_dma_bytes,
                .setup_operands_us = setup_operands_us,
                .build_regcmds_us = build_regcmds_us,
            };

            /* 指标公式集中在公共报告模块，与 core_scaling 保持同一统计口径。 */
            if (rknpu_print_performance_report(&report) != 0) {
                fprintf(stderr, "performance statistics failed\n");
                goto cleanup;
            }
            metrics_ok = 1;
        }
    }

    /* 11. 任一线程少完成、少采样或出现首个错误，整次实验均为 FAIL。 */
    result = metrics_ok ? 0 : 1;
    for (uint32_t index = 0; index < options->threads; index++) {
        if (workers[index].error != WORKER_OK ||
            workers[index].successful_submits != total_rounds ||
            workers[index].latency_sample_count != options->rounds) {
            result = 1;
        }
    }
    printf("rknpu_multithread_submit: %s\n", result == 0 ? "PASS" : "FAIL");

    /* 12. 已 join 后销毁屏障；DMA 必须使用创建时的 fd 释放，再关闭 fd。 */
cleanup:
    if (gate_initialized) {
        round_gate_destroy(&gate);
    }

    free(all_samples);

    /*
     * 每个 worker 必须用创建这些 GEM 的同一个 fd 完成 unmap 和 MEM_DESTROY。
     * 该循环位于全部 pthread_join() 之后，并且必须先于 close_worker_fds()。
     */
    for (uint32_t index = 0; index < prepared_workers; index++) {
        release_worker_resources(&workers[index]);
        free(workers[index].latency_samples_us);
    }

    /* 失败 worker 可能只创建了延迟数组，尚未计入 prepared_workers。 */
    if (workers != NULL && prepared_workers < options->threads) {
        free(workers[prepared_workers].latency_samples_us);
    }

    /* DMA 已全部释放后，shared 关闭一次，per-thread 分别关闭各自 fd。 */
    close_worker_fds(workers, options->threads, options->fd_mode);

    free(threads);
    free(workers);
    return result;
}

/* 依次运行所选场景，每个场景重新创建资源并执行一次 NPU reset。 */
int main(int argc, char **argv) {
    test_options_t options;
    const rknpu_scenario_case_t *scenarios;
    size_t scenario_count;
    int parse_result;
    int result = 0;

    parse_result = parse_options(argc, argv, &options);
    if (parse_result != 0) {
        return parse_result > 0 ? 0 : 2;
    }

    scenarios = rknpu_scenario_cases(&scenario_count);
    for (size_t index = 0; index < scenario_count; index++) {
        if (!scenario_selected(&options, scenarios[index].name)) {
            continue;
        }
        if (run_scenario(&options, &scenarios[index]) != 0) {
            result = 1;
        }
    }
    printf("rknpu_multithread_submit all selected scenarios: %s\n",
           result == 0 ? "PASS" : "FAIL");
    return result;
}
