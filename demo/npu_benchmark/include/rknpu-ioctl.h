/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) Rockchip Electronics Co.Ltd 
 * Author: Felix Zeng <felix.zeng@rock-chips.com> 
 */

#ifndef RKNPU_IOCTL_H
#define RKNPU_IOCTL_H

#include <linux/ioctl.h>
#include <linux/types.h>

#include <libdrm/drm.h>

#if !defined(__KERNEL__)
#define __user
#endif

#ifndef __packed
#define __packed __attribute__((packed))
#endif

#define RKNPU_OFFSET_VERSION 0x0
#define RKNPU_OFFSET_VERSION_NUM 0x4
#define RKNPU_OFFSET_PC_OP_EN 0x8
#define RKNPU_OFFSET_PC_DATA_ADDR 0x10
#define RKNPU_OFFSET_PC_DATA_AMOUNT 0x14
#define RKNPU_OFFSET_PC_TASK_CONTROL 0x30
#define RKNPU_OFFSET_PC_DMA_BASE_ADDR 0x34
#define RKNPU_OFFSET_PC_TASK_STATUS 0x3c

#define RKNPU_OFFSET_INT_MASK 0x20
#define RKNPU_OFFSET_INT_CLEAR 0x24
#define RKNPU_OFFSET_INT_STATUS 0x28
#define RKNPU_OFFSET_INT_RAW_STATUS 0x2c

#define RKNPU_OFFSET_CLR_ALL_RW_AMOUNT 0x8010
#define RKNPU_OFFSET_DT_WR_AMOUNT 0x8034
#define RKNPU_OFFSET_DT_RD_AMOUNT 0x8038
#define RKNPU_OFFSET_WT_RD_AMOUNT 0x803c

#define RKNPU_OFFSET_ENABLE_MASK 0xf008

#define RKNPU_INT_CLEAR 0x1ffff

#define RKNPU_PC_DATA_EXTRA_AMOUNT 4

#define RKNPU_STR_HELPER(x) #x

#define RKNPU_GET_DRV_VERSION_STRING(MAJOR, MINOR, PATCHLEVEL)                 \
        RKNPU_STR_HELPER(MAJOR)                                                \
        "." RKNPU_STR_HELPER(MINOR) "." RKNPU_STR_HELPER(PATCHLEVEL)
#define RKNPU_GET_DRV_VERSION_CODE(MAJOR, MINOR, PATCHLEVEL)                   \
        (MAJOR * 10000 + MINOR * 100 + PATCHLEVEL)
#define RKNPU_GET_DRV_VERSION_MAJOR(CODE) (CODE / 10000)
#define RKNPU_GET_DRV_VERSION_MINOR(CODE) ((CODE % 10000) / 100)
#define RKNPU_GET_DRV_VERSION_PATCHLEVEL(CODE) (CODE % 100)

/* memory type definitions. */
enum e_rknpu_mem_type {
        /* physically continuous memory and used as default. */
        RKNPU_MEM_CONTIGUOUS = 0 << 0,
        /* physically non-continuous memory. */
        RKNPU_MEM_NON_CONTIGUOUS = 1 << 0,
        /* non-cacheable mapping and used as default. */
        RKNPU_MEM_NON_CACHEABLE = 0 << 1,
        /* cacheable mapping. */
        RKNPU_MEM_CACHEABLE = 1 << 1,
        /* write-combine mapping. */
        RKNPU_MEM_WRITE_COMBINE = 1 << 2,
        /* dma attr kernel mapping */
        RKNPU_MEM_KERNEL_MAPPING = 1 << 3,
        /* iommu mapping */
        RKNPU_MEM_IOMMU = 1 << 4,
        /* zero mapping */
        RKNPU_MEM_ZEROING = 1 << 5,
        /* allocate secure buffer */
        RKNPU_MEM_SECURE = 1 << 6,
        /* allocate from non-dma32 zone */
        RKNPU_MEM_NON_DMA32 = 1 << 7,
        /* request SRAM */
        RKNPU_MEM_TRY_ALLOC_SRAM = 1 << 8,
        RKNPU_MEM_MASK = RKNPU_MEM_NON_CONTIGUOUS | RKNPU_MEM_CACHEABLE |
                         RKNPU_MEM_WRITE_COMBINE | RKNPU_MEM_KERNEL_MAPPING |
                         RKNPU_MEM_IOMMU | RKNPU_MEM_ZEROING |
                         RKNPU_MEM_SECURE | RKNPU_MEM_NON_DMA32 |
                         RKNPU_MEM_TRY_ALLOC_SRAM
};

/* sync mode definitions. */
enum e_rknpu_mem_sync_mode {
        RKNPU_MEM_SYNC_TO_DEVICE = 1 << 0,
        RKNPU_MEM_SYNC_FROM_DEVICE = 1 << 1,
        RKNPU_MEM_SYNC_MASK =
                RKNPU_MEM_SYNC_TO_DEVICE | RKNPU_MEM_SYNC_FROM_DEVICE
};

/* job mode definitions. */
enum e_rknpu_job_mode {
        RKNPU_JOB_SLAVE = 0 << 0,
        RKNPU_JOB_PC = 1 << 0,
        RKNPU_JOB_BLOCK = 0 << 1,
        RKNPU_JOB_NONBLOCK = 1 << 1,
        RKNPU_JOB_PINGPONG = 1 << 2,
        RKNPU_JOB_FENCE_IN = 1 << 3,
        RKNPU_JOB_FENCE_OUT = 1 << 4,
        RKNPU_JOB_MASK = RKNPU_JOB_PC | RKNPU_JOB_NONBLOCK |
                         RKNPU_JOB_PINGPONG | RKNPU_JOB_FENCE_IN |
                         RKNPU_JOB_FENCE_OUT,
        /*
         * Submit 内动态 Task 领取标志，最后修改日期：2026-08-18。
         * 该位只影响 StarryOS 调度器，不进入 NPU 硬件 JobMode。
         */
        RKNPU_JOB_DYNAMIC_TASKS = 1 << 5,
        RKNPU_SUBMIT_ALLOWED_MASK = RKNPU_JOB_MASK | RKNPU_JOB_DYNAMIC_TASKS
};

/* action definitions */
enum e_rknpu_action {
        RKNPU_GET_HW_VERSION = 0,
        RKNPU_GET_DRV_VERSION = 1,
        RKNPU_GET_FREQ = 2,
        RKNPU_SET_FREQ = 3,
        RKNPU_GET_VOLT = 4,
        RKNPU_SET_VOLT = 5,
        RKNPU_ACT_RESET = 6,
        RKNPU_GET_BW_PRIORITY = 7,
        RKNPU_SET_BW_PRIORITY = 8,
        RKNPU_GET_BW_EXPECT = 9,
        RKNPU_SET_BW_EXPECT = 10,
        RKNPU_GET_BW_TW = 11,
        RKNPU_SET_BW_TW = 12,
        RKNPU_ACT_CLR_TOTAL_RW_AMOUNT = 13,
        RKNPU_GET_DT_WR_AMOUNT = 14,
        RKNPU_GET_DT_RD_AMOUNT = 15,
        RKNPU_GET_WT_RD_AMOUNT = 16,
        RKNPU_GET_TOTAL_RW_AMOUNT = 17,
        RKNPU_GET_IOMMU_EN = 18,
        RKNPU_SET_PROC_NICE = 19,
        RKNPU_POWER_ON = 20,
        RKNPU_POWER_OFF = 21,
        RKNPU_GET_TOTAL_SRAM_SIZE = 22,
        RKNPU_GET_FREE_SRAM_SIZE = 23,
        RKNPU_GET_IOMMU_DOMAIN_ID = 24,
        RKNPU_SET_IOMMU_DOMAIN_ID = 25,
};

/**
 * User-desired buffer creation information structure.
 *
 * @handle: The handle of the created GEM object.
 * @flags: user request for setting memory type or cache attributes.
 * @size: user-desired memory allocation size.
 *      - this size value would be page-aligned internally.
 * @obj_addr: address of RKNPU memory object.
 * @dma_addr: dma address that access by rknpu.
 * @sram_size: user-desired sram memory allocation size.
 *  - this size value would be page-aligned internally.
 * @iommu_domain_id: iommu domain id.
 * @core_mask: core mask associated with the allocation.
 */
struct rknpu_mem_create {
        __u32 handle;
        __u32 flags;
        __u64 size;
        __u64 obj_addr;
        __u64 dma_addr;
        __u64 sram_size;
        __s32 iommu_domain_id;
        __u32 core_mask;
};

/**
 * A structure for getting a fake-offset that can be used with mmap.
 *
 * @handle: handle of gem object.
 * @reserved: just padding to be 64-bit aligned.
 * @offset: a fake-offset of gem object.
 */
struct rknpu_mem_map {
        __u32 handle;
        __u32 reserved;
        __u64 offset;
};

/**
 * For destroying DMA buffer
 *
 * @handle:     handle of the buffer.
 * @reserved: reserved for padding.
 * @obj_addr: rknpu_mem_object addr.
 */
struct rknpu_mem_destroy {
        __u32 handle;
        __u32 reserved;
        __u64 obj_addr;
};

/**
 * For synchronizing DMA buffer
 *
 * @flags: user request for setting memory type or cache attributes.
 * @reserved: reserved for padding.
 * @obj_addr: address of RKNPU memory object.
 * @offset: offset in bytes from start address of buffer.
 * @size: size of memory region.
 *
 */
struct rknpu_mem_sync {
        __u32 flags;
        __u32 reserved;
        __u64 obj_addr;
        __u64 offset;
        __u64 size;
};

/**
 * struct rknpu_task structure for task information
 *
 * @flags: flags for task
 * @op_idx: operator index
 * @enable_mask: enable mask
 * @int_mask: interrupt mask
 * @int_clear: interrupt clear
 * @int_status: interrupt status
 * @regcfg_amount: register config number
 * @regcfg_offset: offset for register config
 * @regcmd_addr: address for register command
 *
 */
struct rknpu_task {
        __u32 flags;
        __u32 op_idx;
        __u32 enable_mask;
        __u32 int_mask;
        __u32 int_clear;
        __u32 int_status;
        __u32 regcfg_amount;
        __u32 regcfg_offset;
        __u64 regcmd_addr;
} __packed;

/**
 * struct rknpu_subcore_task structure for subcore task index
 *
 * @task_start: task start index
 * @task_number: task number
 *
 */
struct rknpu_subcore_task {
        __u32 task_start;
        __u32 task_number;
};

/**
 * struct rknpu_submit structure for job submit
 *
 * @flags: flags for job submit
 * @timeout: submit timeout
 * @task_start: task start index
 * @task_number: task number
 * @task_counter: task counter
 * @priority: submit priority
 * @task_obj_addr: address of task object
 * @regcfg_obj_addr: address of register config object
 * @task_base_addr: task base address
 * @user_data: (optional) user data
 * @core_mask: core mask of rknpu
 * @fence_fd: dma fence fd
 * @subcore_task: subcore task
 *
 */
struct rknpu_submit {
        __u32 flags;
        __u32 timeout;
        __u32 task_start;
        __u32 task_number;
        __u32 task_counter;
        __s32 priority;
        __u64 task_obj_addr;
        __u64 regcfg_obj_addr;
        __u64 task_base_addr;
        __u64 user_data;
        __u32 core_mask;
        __s32 fence_fd;
        struct rknpu_subcore_task subcore_task[5];
};

/**
 * struct rknpu_task structure for action (GET, SET or ACT)
 *
 * @flags: flags for action
 * @value: GET or SET value
 *
 */
struct rknpu_action {
        __u32 flags;
        __u32 value;
};

/* Submit 四阶段延迟测试接口，最后修改日期：2026-08-07。 */
struct rknpu_submit_trace_record {
        /* 驱动内部 Submit 唯一编号，用于关联调度事件。 */
        __u64 queue_task;
        /* ioctl 进入驱动、尚未 copy-in 的时间。 */
        __u64 t0_ns;
        /* Submit 已进入调度队列的时间。 */
        __u64 t1_ns;
        /* 第一个 Task 开始底层 NPU 下发的时间。 */
        __u64 t2_ns;
        /* 最后一个 Task 完成状态被收割的时间。 */
        __u64 t3_ns;
        /* Task 和 Submit 完成用户态 copy-out 的时间。 */
        __u64 t4_ns;
};

struct rknpu_submit_trace_query {
        /* RESET 或 READ。 */
        __u32 operation;
        /* 用户记录数组容量，单位为条数。 */
        __u32 capacity;
        /* 驱动实际复制的记录数量。 */
        __u32 count;
        /* 内核缓冲区满或用户数组不足时为 1。 */
        __u32 overflowed;
        /* 用户态 struct rknpu_submit_trace_record[] 地址。 */
        __u64 records_address;
};

/*
 * One in-memory timing sample around the scheduler Worker's yield_now().
 * Userspace computes yield_gap_ns = yield_end_ns - yield_start_ns.
 */
struct rknpu_worker_yield_trace_record {
        /* Append order inside the driver trace buffer. */
        __u64 sequence;
        /* Owning Submit id; zero means no single owner could be identified. */
        __u64 queue_task;
        /* Monotonic timestamp immediately before yield_now(). */
        __u64 yield_start_ns;
        /* Monotonic timestamp immediately after yield_now() returned. */
        __u64 yield_end_ns;
        /* INFLIGHT or STALLED reason code. */
        __u32 reason;
        /* ABI padding; always emitted as zero by the driver. */
        __u32 reserved;
};

/* Configure/reset or read the Worker-yield experiment buffer. */
struct rknpu_worker_yield_trace_query {
        /* CONFIG_RESET or READ. */
        __u32 operation;
        /* CONFIG_RESET input; READ returns the current enabled state. */
        __u32 enabled;
        /* CONFIG_RESET requested kernel capacity; READ output-array capacity. */
        __u32 capacity;
        /* Number of records copied by READ. */
        __u32 count;
        /* Kernel-buffer or userspace-capacity overflow indicator. */
        __u32 overflowed;
        /* ABI padding; must be zero. */
        __u32 reserved;
        /* Userspace struct rknpu_worker_yield_trace_record[] address. */
        __u64 records_address;
};

/* 调度事件与资源状态测试接口，最后修改日期：2026-08-19。 */
struct rknpu_schedule_trace_record {
        /* 驱动按实际写入顺序分配的连续事件序号。 */
        __u64 sequence;
        /* 事件发生时的驱动单调时钟，单位为纳秒。 */
        __u64 timestamp_ns;
        /* 一次 blocking Submit 在驱动调度器中的唯一编号。 */
        __u64 queue_task;
        /* Submit 优先级，数值越小优先级越高。 */
        __s32 priority;
        /* ENQUEUE、DISPATCH、COMPLETE 或 FAILED 事件位。 */
        __u32 event_type;
        /* 用户态 Task 的操作编号，用于匹配测试线程和任务。 */
        __u32 op_idx;
        /* Task 在当前 Submit 任务数组中的下标。 */
        __u32 task_index;
        /* 实际下发或完成该 Task 的物理 NPU 核心。 */
        __u32 core_slot;
        /* Task 所属的 Submit 逻辑 lane。 */
        __u32 lane_slot;
        /* NONE、READY 或 RUNNING，表示本次 Dispatch 的选择来源。 */
        __u32 dispatch_source;
        /* 作出 Dispatch 决策时，该核心可执行的最高 Ready 优先级。 */
        __s32 ready_priority;
        /* 对应核心进入 IRQ 处理函数的时间；仅 COMPLETE 有效。 */
        __u64 irq_timestamp_ns;
        /* Event 模式为 Worker 恢复时间；轮询模式为首次观察完成的时间。 */
        __u64 worker_resume_ns;
        /* Worker 循环编号，用于关联同轮 COMPLETE 与后续 DISPATCH。 */
        __u64 worker_cycle;
        /* 本轮 Worker 一次收割的 completion Core 数。 */
        __u32 harvested_cores;
        /* ABI 保留字段，驱动固定写 0。 */
        __u32 timing_reserved;
};

struct rknpu_schedule_trace_query {
        /* CONFIG_RESET、READ 或 STATE。 */
        __u32 operation;
        /* 配置的事件位；READ 时返回驱动当前事件位。 */
        __u32 event_mask;
        /* records_address 指向的数组容量，单位为记录条数。 */
        __u32 capacity;
        /* 驱动实际复制到用户数组的记录数量。 */
        __u32 count;
        /* 内核缓冲区已满或用户数组不足时为 1。 */
        __u32 overflowed;
        /* ABI 保留字段，调用者初始化为 0。 */
        __u32 reserved;
        /* 用户态 struct rknpu_schedule_trace_record[] 地址。 */
        __u64 records_address;
        /* 用户态 struct rknpu_scheduler_state_snapshot 地址。 */
        __u64 state_address;
};

struct rknpu_scheduler_state_snapshot {
        /* 调度器仍持有的 Submit 数量。 */
        __u32 live_submits;
        /* 所有 Ready 优先级桶中的条目总数。 */
        __u32 ready_entries;
        /* 所有 Running 优先级桶中的条目总数。 */
        __u32 running_entries;
        /* 已完成但尚未由 ioctl 取走的 Submit 数量。 */
        __u32 complete_entries;
        /* 尚未删除的 blocking waiter 数量。 */
        __u32 waiters;
        /* 当前核心到 Task 的活动绑定数量。 */
        __u32 core_bindings;
        /* GEM 池仍持有的 DMA 缓冲区数量。 */
        __u32 gem_buffers;
        /* 64 位字段对齐保留值。 */
        __u32 reserved;
        /* GEM 池仍持有的 DMA 总字节数。 */
        __u64 gem_bytes;
};

#define RKNPU_ACTION 0x00
#define RKNPU_SUBMIT 0x01
#define RKNPU_MEM_CREATE 0x02
#define RKNPU_MEM_MAP 0x03
#define RKNPU_MEM_DESTROY 0x04
#define RKNPU_MEM_SYNC 0x05
#define RKNPU_SUBMIT_TRACE 0x06
#define RKNPU_SCHEDULE_TRACE 0x07
#define RKNPU_WORKER_YIELD_TRACE 0x08

#define RKNPU_SUBMIT_TRACE_RESET 0U
#define RKNPU_SUBMIT_TRACE_READ 1U
#define RKNPU_SUBMIT_TRACE_CAPACITY 1024U

#define RKNPU_WORKER_YIELD_TRACE_CONFIG_RESET 0U
#define RKNPU_WORKER_YIELD_TRACE_READ 1U
#define RKNPU_WORKER_YIELD_REASON_INFLIGHT 1U
#define RKNPU_WORKER_YIELD_REASON_STALLED 2U
#define RKNPU_WORKER_YIELD_TRACE_DEFAULT_CAPACITY 262144U
#define RKNPU_WORKER_YIELD_TRACE_MAX_CAPACITY 1048576U

#define RKNPU_SCHEDULE_TRACE_CONFIG_RESET 0U
#define RKNPU_SCHEDULE_TRACE_READ 1U
#define RKNPU_SCHEDULE_TRACE_STATE 2U
#define RKNPU_SCHEDULE_EVENT_ENQUEUE (1U << 0)
#define RKNPU_SCHEDULE_EVENT_DISPATCH (1U << 1)
#define RKNPU_SCHEDULE_EVENT_COMPLETE (1U << 2)
#define RKNPU_SCHEDULE_EVENT_FAILED (1U << 3)
#define RKNPU_SCHEDULE_EVENT_ALL ((1U << 4) - 1U)
#define RKNPU_DISPATCH_SOURCE_NONE 0U
#define RKNPU_DISPATCH_SOURCE_READY 1U
#define RKNPU_DISPATCH_SOURCE_RUNNING 2U
#define RKNPU_SCHEDULE_TRACE_CAPACITY 16384U
#define RKNPU_SCHEDULE_TRACE_NO_VALUE 0xffffffffU
#define RKNPU_SCHEDULE_TRACE_NO_PRIORITY 0x7fffffff

#define RKNPU_IOC_MAGIC 'r'
#define RKNPU_IOW(nr, type) _IOW(RKNPU_IOC_MAGIC, nr, type)
#define RKNPU_IOR(nr, type) _IOR(RKNPU_IOC_MAGIC, nr, type)
#define RKNPU_IOWR(nr, type) _IOWR(RKNPU_IOC_MAGIC, nr, type)

//#include <libdrm/drm.h>

#define DRM_IOCTL_RKNPU_ACTION                                                 \
        DRM_IOWR(DRM_COMMAND_BASE + RKNPU_ACTION, struct rknpu_action)
#define DRM_IOCTL_RKNPU_SUBMIT                                                 \
        DRM_IOWR(DRM_COMMAND_BASE + RKNPU_SUBMIT, struct rknpu_submit)
#define DRM_IOCTL_RKNPU_MEM_CREATE                                             \
        DRM_IOWR(DRM_COMMAND_BASE + RKNPU_MEM_CREATE, struct rknpu_mem_create)
#define DRM_IOCTL_RKNPU_MEM_MAP                                                \
        DRM_IOWR(DRM_COMMAND_BASE + RKNPU_MEM_MAP, struct rknpu_mem_map)
#define DRM_IOCTL_RKNPU_MEM_DESTROY                                            \
        DRM_IOWR(DRM_COMMAND_BASE + RKNPU_MEM_DESTROY, struct rknpu_mem_destroy)
#define DRM_IOCTL_RKNPU_MEM_SYNC                                               \
        DRM_IOWR(DRM_COMMAND_BASE + RKNPU_MEM_SYNC, struct rknpu_mem_sync)
#define DRM_IOCTL_RKNPU_SUBMIT_TRACE                                           \
        DRM_IOWR(DRM_COMMAND_BASE + RKNPU_SUBMIT_TRACE,                        \
                 struct rknpu_submit_trace_query)
#define DRM_IOCTL_RKNPU_SCHEDULE_TRACE                                         \
        DRM_IOWR(DRM_COMMAND_BASE + RKNPU_SCHEDULE_TRACE,                      \
                 struct rknpu_schedule_trace_query)
#define DRM_IOCTL_RKNPU_WORKER_YIELD_TRACE                                     \
        DRM_IOWR(DRM_COMMAND_BASE + RKNPU_WORKER_YIELD_TRACE,                  \
                 struct rknpu_worker_yield_trace_query)

#define IOCTL_RKNPU_ACTION RKNPU_IOWR(RKNPU_ACTION, struct rknpu_action)
#define IOCTL_RKNPU_SUBMIT RKNPU_IOWR(RKNPU_SUBMIT, struct rknpu_submit)
#define IOCTL_RKNPU_MEM_CREATE                                                 \
        RKNPU_IOWR(RKNPU_MEM_CREATE, struct rknpu_mem_create)
#define IOCTL_RKNPU_MEM_MAP RKNPU_IOWR(RKNPU_MEM_MAP, struct rknpu_mem_map)
#define IOCTL_RKNPU_MEM_DESTROY                                                \
        RKNPU_IOWR(RKNPU_MEM_DESTROY, struct rknpu_mem_destroy)
#define IOCTL_RKNPU_MEM_SYNC RKNPU_IOWR(RKNPU_MEM_SYNC, struct rknpu_mem_sync)

#endif // RKNPU_IOCTL_H
