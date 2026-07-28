#ifndef LIBNPU_H
#define LIBNPU_H

#include <stddef.h>
#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef struct libnpu_subcore_task {
  uint32_t task_start;
  uint32_t task_number;
} libnpu_subcore_task;

typedef struct libnpu_submit {
  uint32_t flags;
  uint32_t timeout;
  uint32_t task_start;
  uint32_t task_number;
  uint32_t task_counter;
  int32_t priority;
  uint64_t task_obj_addr;
  uint32_t iommu_domain_id;
  uint32_t reserved;
  uint64_t task_base_addr;
  int64_t hw_elapse_time;
  uint32_t core_mask;
  int32_t fence_fd;
  libnpu_subcore_task subcore_task[5];
} libnpu_submit;

void *dma_malloc(size_t size, uint64_t *dma_addr);
int dma_free(void *ptr);

#ifdef __cplusplus
}
#endif

#endif
