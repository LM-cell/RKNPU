#include "libnpu.h"

#include <errno.h>
#include <stdint.h>

#define SYS_DMA_MALLOC 400
#define SYS_DMA_FREE 401

#if !defined(__aarch64__)
#error "libnpu currently only supports aarch64 syscall ABI"
#endif

static inline long libnpu_syscall1(long sysno, long arg0) {
  register long x0 asm("x0") = arg0;
  register long x8 asm("x8") = sysno;

  /*
   * AArch64 Linux syscall ABI:
   * - x0: 第一个参数
   * - x8: syscall 号
   * - svc #0: 进入内核
   */
  asm volatile("svc #0" : "+r"(x0) : "r"(x8) : "memory");
  return x0;
}

static inline long libnpu_syscall2(long sysno, long arg0, long arg1) {
  register long x0 asm("x0") = arg0;
  register long x1 asm("x1") = arg1;
  register long x8 asm("x8") = sysno;

  /*
   * AArch64 Linux syscall ABI:
   * - x0/x1: 前两个参数
   * - x8: syscall 号
   * - svc #0: 进入内核
   */
  asm volatile("svc #0" : "+r"(x0) : "r"(x1), "r"(x8) : "memory");
  return x0;
}

void *dma_malloc(size_t size, uint64_t *dma_addr) {
  long ret;

  if (dma_addr == NULL) {
    errno = EINVAL;
    return NULL;
  }

  ret = libnpu_syscall2(SYS_DMA_MALLOC, (long)size, (long)(uintptr_t)dma_addr);
  if (ret < 0) {
    errno = (int)-ret;
    return NULL;
  }

  return (void *)(uintptr_t)ret;
}

int dma_free(void *ptr) {
  long ret;

  if (ptr == NULL) {
    errno = EINVAL;
    return -1;
  }

  ret = libnpu_syscall1(SYS_DMA_FREE, (long)(uintptr_t)ptr);
  if (ret < 0) {
    errno = (int)-ret;
    return -1;
  }

  return 0;
}
