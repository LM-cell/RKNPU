#define _POSIX_C_SOURCE 200809L

/*
 * Concurrent mixed-workload launcher for RKNPU regression.
 *
 * This wrapper starts:
 *   1. matmul_multi_process
 *   2. llama_npu0 <stories15M.bin>
 *
 * It is intended to validate that the current task-boundary submit switching
 * path still behaves correctly when a long-running llama inference overlaps
 * with the multi-process matmul stress workload.
 */

#include <errno.h>
#include <signal.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/types.h>
#include <sys/wait.h>
#include <unistd.h>

#define DEFAULT_MATMUL_BIN "./matmul_multi_process"
#define DEFAULT_LLAMA_BIN "./llama_npu0"
#define DEFAULT_MODEL_PATH "./stories15M.bin"

typedef struct {
    const char *matmul_bin;
    const char *llama_bin;
    const char *model_path;
} launcher_config_t;

static void usage(const char *prog) {
    fprintf(stderr, "Usage: %s [options]\n", prog);
    fprintf(stderr, "Options:\n");
    fprintf(stderr, "  --matmul-bin <path>  matmul launcher, default: %s\n", DEFAULT_MATMUL_BIN);
    fprintf(stderr, "  --llama-bin <path>   llama executable, default: %s\n", DEFAULT_LLAMA_BIN);
    fprintf(stderr, "  --model <path>       llama model path, default: %s\n", DEFAULT_MODEL_PATH);
}

static int parse_args(int argc, char *argv[], launcher_config_t *cfg) {
    cfg->matmul_bin = DEFAULT_MATMUL_BIN;
    cfg->llama_bin = DEFAULT_LLAMA_BIN;
    cfg->model_path = DEFAULT_MODEL_PATH;

    for (int i = 1; i < argc; ) {
        if (strcmp(argv[i], "--matmul-bin") == 0) {
            if (i + 1 >= argc) {
                usage(argv[0]);
                return -1;
            }
            cfg->matmul_bin = argv[i + 1];
            i += 2;
        } else if (strcmp(argv[i], "--llama-bin") == 0) {
            if (i + 1 >= argc) {
                usage(argv[0]);
                return -1;
            }
            cfg->llama_bin = argv[i + 1];
            i += 2;
        } else if (strcmp(argv[i], "--model") == 0) {
            if (i + 1 >= argc) {
                usage(argv[0]);
                return -1;
            }
            cfg->model_path = argv[i + 1];
            i += 2;
        } else {
            usage(argv[0]);
            return -1;
        }
    }

    return 0;
}

static pid_t spawn_child(char *const child_argv[]) {
    pid_t pid = fork();
    if (pid < 0) {
        return -1;
    }
    if (pid == 0) {
        execvp(child_argv[0], child_argv);
        fprintf(
            stderr,
            "[mixed-launch] exec failed: %s errno=%d (%s)\n",
            child_argv[0],
            errno,
            strerror(errno)
        );
        _exit(127);
    }
    return pid;
}

static void terminate_child(pid_t pid, const char *name) {
    if (pid <= 0) {
        return;
    }
    if (kill(pid, SIGTERM) < 0 && errno != ESRCH) {
        fprintf(
            stderr,
            "[mixed-launch] failed to SIGTERM %s pid=%d: errno=%d (%s)\n",
            name,
            pid,
            errno,
            strerror(errno)
        );
    }
}

static int decode_status(const char *name, int status) {
    if (WIFEXITED(status)) {
        int code = WEXITSTATUS(status);
        if (code == 0) {
            fprintf(stderr, "[mixed-launch] %s exited cleanly\n", name);
            return 0;
        }
        fprintf(stderr, "[mixed-launch] %s exited with code=%d\n", name, code);
        return code;
    }
    if (WIFSIGNALED(status)) {
        fprintf(
            stderr,
            "[mixed-launch] %s terminated by signal=%d\n",
            name,
            WTERMSIG(status)
        );
        return 128 + WTERMSIG(status);
    }
    fprintf(stderr, "[mixed-launch] %s ended with unexpected status=0x%x\n", name, status);
    return 1;
}

int main(int argc, char *argv[]) {
    launcher_config_t cfg;
    pid_t matmul_pid = -1;
    pid_t llama_pid = -1;
    bool matmul_done = false;
    bool llama_done = false;
    int result = 0;

    if (parse_args(argc, argv, &cfg) != 0) {
        return 2;
    }

    if (access(cfg.model_path, R_OK) != 0) {
        fprintf(
            stderr,
            "[mixed-launch] model is not readable: %s errno=%d (%s)\n",
            cfg.model_path,
            errno,
            strerror(errno)
        );
        return 2;
    }

    char *matmul_argv[] = { (char *)cfg.matmul_bin, NULL };
    char *llama_argv[] = { (char *)cfg.llama_bin, (char *)cfg.model_path, NULL };

    fprintf(
        stderr,
        "[mixed-launch] starting matmul=%s llama=%s model=%s\n",
        cfg.matmul_bin,
        cfg.llama_bin,
        cfg.model_path
    );

    matmul_pid = spawn_child(matmul_argv);
    if (matmul_pid < 0) {
        fprintf(stderr, "[mixed-launch] failed to fork matmul child: errno=%d (%s)\n", errno, strerror(errno));
        return 2;
    }

    llama_pid = spawn_child(llama_argv);
    if (llama_pid < 0) {
        fprintf(stderr, "[mixed-launch] failed to fork llama child: errno=%d (%s)\n", errno, strerror(errno));
        terminate_child(matmul_pid, "matmul");
        waitpid(matmul_pid, NULL, 0);
        return 2;
    }

    fprintf(
        stderr,
        "[mixed-launch] spawned matmul pid=%d, llama pid=%d\n",
        matmul_pid,
        llama_pid
    );

    while (!matmul_done || !llama_done) {
        int status = 0;
        pid_t pid = waitpid(-1, &status, 0);
        if (pid < 0) {
            if (errno == EINTR) {
                continue;
            }
            fprintf(stderr, "[mixed-launch] waitpid failed: errno=%d (%s)\n", errno, strerror(errno));
            result = 1;
            break;
        }

        if (pid == matmul_pid) {
            matmul_done = true;
            int code = decode_status("matmul_multi_process", status);
            if (code != 0 && result == 0) {
                result = code;
                if (!llama_done) {
                    terminate_child(llama_pid, "llama");
                }
            }
        } else if (pid == llama_pid) {
            llama_done = true;
            int code = decode_status("llama_npu0", status);
            if (code != 0 && result == 0) {
                result = code;
                if (!matmul_done) {
                    terminate_child(matmul_pid, "matmul");
                }
            }
        }
    }

    if (result == 0) {
        fprintf(stderr, "[mixed-launch] pass\n");
    }
    return result;
}
