#include "libnpu.h"

#include <errno.h>
#include <stdint.h>

#define SYS_DMA_MALLOC 400
#define SYS_DMA_FREE 401

#if defined(__aarch64__)
static inline long libnpu_syscall1(long sysno, long arg0) {
  register long x0 __asm__("x0") = arg0;
  register long x8 __asm__("x8") = sysno;

  __asm__ volatile("svc #0" : "+r"(x0) : "r"(x8) : "memory");
  return x0;
}

static inline long libnpu_syscall2(long sysno, long arg0, long arg1) {
  register long x0 __asm__("x0") = arg0;
  register long x1 __asm__("x1") = arg1;
  register long x8 __asm__("x8") = sysno;

  __asm__ volatile("svc #0" : "+r"(x0) : "r"(x1), "r"(x8) : "memory");
  return x0;
}
#elif defined(__riscv) && __riscv_xlen == 64
static inline long libnpu_syscall1(long sysno, long arg0) {
  register long a0 __asm__("a0") = arg0;
  register long a7 __asm__("a7") = sysno;

  __asm__ volatile("ecall" : "+r"(a0) : "r"(a7) : "memory");
  return a0;
}

static inline long libnpu_syscall2(long sysno, long arg0, long arg1) {
  register long a0 __asm__("a0") = arg0;
  register long a1 __asm__("a1") = arg1;
  register long a7 __asm__("a7") = sysno;

  __asm__ volatile("ecall" : "+r"(a0) : "r"(a1), "r"(a7) : "memory");
  return a0;
}
#else
#error "libnpu only supports aarch64 and riscv64 syscall ABIs"
#endif

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
