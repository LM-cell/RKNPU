#ifndef LIBNPU_H
#define LIBNPU_H

#include <stddef.h>
#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

/*
 * 申请一段可被 NPU 直接访问的连续 DMA 内存。
 *
 * 返回值:
 * - 成功: 用户态虚拟地址
 * - 失败: NULL，并设置 errno
 *
 * 输出参数:
 * - dma_addr: 设备侧访问这段内存时使用的 DMA/bus 地址
 */
void *dma_malloc(size_t size, uint64_t *dma_addr);

/*
 * 释放一段由 dma_malloc 返回的 DMA 内存。
 *
 * 注意:
 * - ptr 必须是 dma_malloc 原样返回的首地址
 * - 不能传偏移地址
 * - 不能跨进程释放
 *
 * 返回值:
 * - 成功: 0
 * - 失败: -1，并设置 errno
 */
int dma_free(void *ptr);

#ifdef __cplusplus
}
#endif

#endif /* LIBNPU_H */
