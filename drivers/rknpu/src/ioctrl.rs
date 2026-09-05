// RKNPU ioctl 调度追踪与仿真测试，最后修改日期：2026-08-07。
use crate::{Rknpu, RknpuError, RknpuTask};
use core::mem::size_of;

/// Per-core task range passed in from userspace.
///
/// Each entry in `RknpuSubmit::subcore_task[5]` tells the driver which slice of
/// `tasks[]` should be dispatched to a given logical core slot.
#[repr(C)]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct RknpuSubcoreTask {
    /// Index of the first task assigned to this core slot.
    pub task_start: u32,
    /// Number of contiguous tasks assigned to this core slot.
    pub task_number: u32,
}

/// Parameters used to map a GEM buffer into userspace via `mmap`.
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct RknpuMemMap {
    /// GEM object handle returned by `MEM_CREATE`.
    pub handle: u32,
    /// Reserved padding for 64-bit alignment.
    pub reserved: u32,
    /// Driver-provided pseudo file offset suitable for `mmap()`.
    pub offset: u64,
}

/// Parameters for destroying one DMA buffer (GEM object).
///
/// Userspace passes back the opaque `handle` received from `MEM_CREATE`.
/// `obj_addr` is kept for ABI compatibility with the Linux-style ioctl
/// contract, but the driver-side lookup is keyed by `handle`.
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct RknpuMemDestroy {
    /// GEM object handle previously returned by `MEM_CREATE`.
    pub handle: u32,
    /// Reserved padding for 64-bit alignment.
    pub reserved: u32,
    /// CPU virtual address of the memory object (ABI compatibility only).
    pub obj_addr: u64,
}

/// Main task-submission ioctl structure.
///
/// Userspace fills this structure with task metadata and passes it to
/// `ioctl(SUBMIT)`. The driver uses it to locate the task descriptor array,
/// program the PC block on the target cores, and report terminal status back to
/// userspace.
///
/// Summary:
///
/// ```text
///  flags            -- JobMode bits (PC, NONBLOCK, PINGPONG, ...)
///  task_array_cpu_address    -- CPU address of the `RknpuTask[]` array
///  task_array_dma_address   -- DMA address of the same array
///  subcore_task[5]  -- task ranges per core slot
///  core_mask        -- core selection bitmask
/// ```
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct RknpuSubmit {
    /// Job-mode flags.
    pub flags: u32,
    /// Maximum wait time before timing out, in milliseconds.
    pub timeout: u32,
    /// Legacy global task-start index, superseded by `subcore_task[]`.
    pub task_start: u32,
    /// Total number of tasks across all cores.
    pub task_number: u32,
    /// Filled by the driver with the number of completed tasks.
    pub task_counter: u32,
    /// Scheduling priority hint. Lower values mean higher priority.
    pub priority: i32,
    /// CPU virtual address of the `RknpuTask[]` array.
    pub task_array_cpu_address: u64,
    /// IOMMU domain identifier used for address translation.
    pub iommu_domain_id: u32,
    /// Reserved field kept for ABI compatibility.
    pub reserved: u32,
    /// DMA or bus address of the `RknpuTask[]` array.
    pub task_array_dma_address: u64,
    /// Filled by the driver with a hardware execution-time estimate.
    pub hw_elapse_time: i64,
    /// Bitmask selecting which NPU cores may be used.
    pub core_mask: u32,
    /// Fence file descriptor for external synchronization.
    pub fence_fd: i32,
    /// Task range per logical core slot. Entries with `task_number == 0` are
    /// skipped.
    pub subcore_task: [RknpuSubcoreTask; 5],
}

/// Ioctl structure used to allocate a DMA-visible buffer.
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct RknpuMemCreate {
    /// Opaque handle returned by the driver.
    pub handle: u32,
    /// Memory type or caching flags.
    pub flags: u32,
    /// Requested allocation size in bytes.
    pub size: u64,
    /// CPU virtual address of the allocation.
    pub obj_addr: u64,
    /// DMA or bus address used by the NPU.
    pub dma_addr: u64,
    /// Actual allocated size, which may differ on special paths.
    pub sram_size: u64,
    /// IOMMU domain used for isolation.
    pub iommu_domain_id: i32,
    /// Core mask associated with the allocation.
    pub core_mask: u32,
}

/// Ioctl structure used to synchronize a DMA buffer range.
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct RknpuMemSync {
    /// Direction flags such as `TO_DEVICE` or `FROM_DEVICE`.
    pub flags: u32,
    /// Reserved padding for 64-bit alignment.
    pub reserved: u32,
    /// CPU virtual address of the buffer to synchronize.
    pub obj_addr: u64,
    /// Byte offset into the buffer.
    pub offset: u64,
    /// Number of bytes to synchronize.
    pub size: u64,
}

/// Submit 四阶段延迟原始记录，最后修改日期：2026-08-07。
///
/// 每个成功返回的 blocking Submit 产生一条记录。用户态只对满足
/// `t0<=t1<=t2<=t3<=t4` 的完整记录计算阶段差值。
#[repr(C)]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct RknpuSubmitTraceRecord {
    /// 驱动内部 Submit 唯一编号，用于关联调度事件。
    pub queue_task: u64,
    /// Submit ioctl 进入驱动、尚未 copy-in 的时间。
    pub t0_ns: u64,
    /// Submit 已经进入调度队列的时间。
    pub t1_ns: u64,
    /// 第一个 Task 在持有设备锁后开始底层下发的时间。
    pub t2_ns: u64,
    /// 最后一个 Task 的完成状态被调度器收割的时间。
    pub t3_ns: u64,
    /// Task 数组和 Submit 头完成用户态 copy-out 的时间。
    pub t4_ns: u64,
}

/// 测试程序通过该结构复位或读取 Submit 延迟记录。
#[repr(C)]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct RknpuSubmitTraceQuery {
    /// RESET 或 READ 操作码。
    pub operation: u32,
    /// 用户态记录数组容量，单位为元素数量。
    pub capacity: u32,
    /// 驱动实际复制的记录数量。
    pub count: u32,
    /// 内核缓冲区满或用户数组不足时为 1。
    pub overflowed: u32,
    /// 用户态 `RknpuSubmitTraceRecord[]` 地址。
    pub records_address: u64,
}

pub const RKNPU_SUBMIT_TRACE_RESET: u32 = 0;
pub const RKNPU_SUBMIT_TRACE_READ: u32 = 1;
pub const RKNPU_SUBMIT_TRACE_CAPACITY: usize = 1024;

/// One scheduler-worker `yield_now()` interval captured entirely in memory.
///
/// `yield_start_ns` is sampled immediately before the worker calls the
/// platform yield hook and `yield_end_ns` immediately after that call returns.
/// Userspace computes `yield_gap_ns = end - start`; keeping both timestamps
/// also lets the experiment join the interval to the existing per-submit
/// `t0..t4` trace through `queue_task`.
#[repr(C)]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct RknpuWorkerYieldTraceRecord {
    /// Monotonic sequence assigned when the record is appended.
    pub sequence: u64,
    /// Submit owning every inflight core at the yield boundary.
    ///
    /// Zero means that no single owner could be identified. The single-submit
    /// `core_scaling_benchmark_fix` experiment always has one owner.
    pub queue_task: u64,
    /// Monotonic timestamp immediately before `yield_now()`.
    pub yield_start_ns: u64,
    /// Monotonic timestamp immediately after `yield_now()` returns.
    pub yield_end_ns: u64,
    /// Why the Worker yielded: inflight hardware or a stalled ready queue.
    pub reason: u32,
    /// Reserved for ABI stability; always zero.
    pub reserved: u32,
}

/// Configure/reset or read the worker-yield trace buffer.
#[repr(C)]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct RknpuWorkerYieldTraceQuery {
    /// CONFIG_RESET or READ operation code.
    pub operation: u32,
    /// CONFIG_RESET: 1 enables collection, 0 disables it. READ returns state.
    pub enabled: u32,
    /// CONFIG_RESET: requested kernel capacity; zero selects the default.
    /// READ: capacity of the userspace record array, in elements.
    pub capacity: u32,
    /// READ: number of records copied to userspace.
    pub count: u32,
    /// Set when the kernel buffer or userspace array was too small.
    pub overflowed: u32,
    /// Must be initialized to zero by userspace.
    pub reserved: u32,
    /// Userspace `RknpuWorkerYieldTraceRecord[]` address for READ.
    pub records_address: u64,
}

/// Replace the current trace with a fresh enabled or disabled buffer.
pub const RKNPU_WORKER_YIELD_TRACE_CONFIG_RESET: u32 = 0;
/// Copy one snapshot to userspace without clearing it.
pub const RKNPU_WORKER_YIELD_TRACE_READ: u32 = 1;
/// The Worker was waiting for at least one inflight hardware dispatch.
pub const RKNPU_WORKER_YIELD_REASON_INFLIGHT: u32 = 1;
/// Live scheduler work existed but no dispatch was currently possible.
pub const RKNPU_WORKER_YIELD_REASON_STALLED: u32 = 2;
/// Default capacity: 10 MiB of 40-byte records, allocated only when enabled.
pub const RKNPU_WORKER_YIELD_TRACE_DEFAULT_CAPACITY: usize = 262_144;
/// Safety ceiling for explicit experiments: 40 MiB of 40-byte records.
pub const RKNPU_WORKER_YIELD_TRACE_MAX_CAPACITY: usize = 1_048_576;

/// 一条调度事件的原始记录，最后修改日期：2026-08-19。
///
/// 驱动只保存原始事实，不在内核中计算平均值或百分位。用户态通过
/// `queue_task` 将 Enqueue、Dispatch、Complete 事件归并到同一个 Submit，
/// 再通过 `task_index`、`core_slot` 和 `lane_slot` 检查 Task 是否重复或丢失。
#[repr(C)]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct RknpuScheduleTraceRecord {
    /// 事件写入缓冲区时分配的连续序号，用于检测记录缺口或重复。
    pub sequence: u64,
    /// 事件发生时的单调时钟，单位为纳秒。
    pub timestamp_ns: u64,
    /// 驱动为一次 Submit 分配的内部唯一编号。
    pub queue_task: u64,
    /// 该 Submit 的调度优先级；数值越小，优先级越高。
    pub priority: i32,
    /// 事件类型，对应 Enqueue、Dispatch、Complete 或 Failed 位。
    pub event_type: u32,
    /// 用户态写入任务描述符的操作编号，用于匹配测试线程和 Task。
    pub op_idx: u32,
    /// 当前事件对应的 Task 在 Submit 任务数组中的下标。
    pub task_index: u32,
    /// 实际执行该 Task 的物理 NPU 核心编号；入队事件使用无效值。
    pub core_slot: u32,
    /// 当前 Task 所属的 Submit 逻辑 lane；入队事件使用无效值。
    pub lane_slot: u32,
    /// Dispatch 来自 Ready 提升还是 Running 续派；其他事件为 NONE。
    pub dispatch_source: u32,
    /// 作出 Dispatch 决策时，该核心可执行的最高优先级 Ready 值。
    pub ready_priority: i32,
    /// 对应核心进入 IRQ 处理函数的时间；仅 Complete 事件有效。
    pub irq_timestamp_ns: u64,
    /// Event 模式为 Worker 恢复时间，轮询模式为 Worker 首次观察完成的时间。
    pub worker_resume_ns: u64,
    /// Worker 调度循环编号，用于关联同一轮 Complete 与后续 Dispatch。
    pub worker_cycle: u64,
    /// 本轮 Worker 一次收割的 completion Core 数；仅 Complete 事件有效。
    pub harvested_cores: u32,
    /// 保留字段，用户态必须按 0 检查。
    pub timing_reserved: u32,
}

/// 调度事件测试 ioctl 的输入输出参数，最后修改日期：2026-08-07。
///
/// `operation` 决定其余字段的方向：CONFIG_RESET 使用 `event_mask`；READ
/// 使用用户提供的 `capacity` 和 `records_address`；STATE 使用
/// `state_address`。所有地址和容量都必须在驱动 copy-out 前完成检查。
#[repr(C)]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct RknpuScheduleTraceQuery {
    /// CONFIG_RESET、READ 或 STATE 操作码。
    pub operation: u32,
    /// 需要保存的事件位；READ 时返回当前已启用的事件位。
    pub event_mask: u32,
    /// 用户态记录数组可容纳的元素数量，不是字节数。
    pub capacity: u32,
    /// 驱动本次实际复制到用户态的记录数量。
    pub count: u32,
    /// 内核缓冲区已满或用户容量不足时返回 1。
    pub overflowed: u32,
    /// 保持结构布局稳定，用户态必须初始化为 0。
    pub reserved: u32,
    /// READ 操作使用的用户态 `RknpuScheduleTraceRecord[]` 地址。
    pub records_address: u64,
    /// STATE 操作使用的用户态 `RknpuSchedulerStateSnapshot` 地址。
    pub state_address: u64,
}

/// 测试结束时读取的调度器与 GEM 资源状态，最后修改日期：2026-08-07。
///
/// 该结构用于比较测试前后的资源基线，并验证 blocking Submit 返回后没有
/// 遗留 Ready、Running、waiter 或核心绑定。它只暴露数量，不暴露内核地址。
#[repr(C)]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct RknpuSchedulerStateSnapshot {
    /// `tasks` 所有者表中仍存活的 Submit 数量。
    pub live_submits: u32,
    /// 所有优先级 Ready 桶中的 Submit 条目总数。
    pub ready_entries: u32,
    /// 所有优先级 Running 桶中的 Submit 条目总数。
    pub running_entries: u32,
    /// 已完成但尚未被 ioctl 路径取走的 Submit 数量。
    pub complete_entries: u32,
    /// 尚未删除的 blocking Submit waiter 数量。
    pub waiters: u32,
    /// 当前物理核心到 Task 的活动绑定数量。
    pub core_bindings: u32,
    /// 驱动 GEM 池仍持有的 DMA 缓冲区数量。
    pub gem_buffers: u32,
    /// 保持 64 位字段对齐。
    pub reserved: u32,
    /// 驱动 GEM 池仍持有的 DMA 总字节数。
    pub gem_bytes: u64,
}

/// 清空旧记录并按照 `event_mask` 开始或停止记录。
pub const RKNPU_SCHEDULE_TRACE_CONFIG_RESET: u32 = 0;
/// 一次性读取当前记录快照，不清空内核缓冲区。
pub const RKNPU_SCHEDULE_TRACE_READ: u32 = 1;
/// 读取调度器和 GEM 资源数量快照。
pub const RKNPU_SCHEDULE_TRACE_STATE: u32 = 2;
/// Submit 已经进入 Ready 队列。
pub const RKNPU_SCHEDULE_EVENT_ENQUEUE: u32 = 1 << 0;
/// 一个 Task 已经开始走底层 NPU 下发路径。
pub const RKNPU_SCHEDULE_EVENT_DISPATCH: u32 = 1 << 1;
/// 驱动已经从对应核心收割该 Task 的完成状态。
pub const RKNPU_SCHEDULE_EVENT_COMPLETE: u32 = 1 << 2;
/// DMA 同步或底层下发失败。
pub const RKNPU_SCHEDULE_EVENT_FAILED: u32 = 1 << 3;
pub const RKNPU_SCHEDULE_EVENT_ALL: u32 = RKNPU_SCHEDULE_EVENT_ENQUEUE
    | RKNPU_SCHEDULE_EVENT_DISPATCH
    | RKNPU_SCHEDULE_EVENT_COMPLETE
    | RKNPU_SCHEDULE_EVENT_FAILED;
/// 非 Dispatch 事件没有选择来源。
pub const RKNPU_DISPATCH_SOURCE_NONE: u32 = 0;
/// 当前 Task 来自 Ready Submit 的首次提升或重新提升。
pub const RKNPU_DISPATCH_SOURCE_READY: u32 = 1;
/// 当前 Task 来自已经处于 Running 状态的 Submit。
pub const RKNPU_DISPATCH_SOURCE_RUNNING: u32 = 2;
/// 单次诊断窗口最多保存 16384 条调度事件。
pub const RKNPU_SCHEDULE_TRACE_CAPACITY: usize = 16384;
/// 事件不存在核心、lane 或 Task 下标时使用的无效值。
pub const RKNPU_SCHEDULE_TRACE_NO_VALUE: u32 = u32::MAX;
/// 作出决策时没有可执行 Ready Submit。
pub const RKNPU_SCHEDULE_TRACE_NO_PRIORITY: i32 = i32::MAX;

impl Rknpu {
    /// Dispatches one queued task to one hardware core.
    ///
    /// This is the driver-side execution primitive used by the queue
    /// scheduler. One call does exactly one thing:
    ///
    /// - bind one queued task to one physical core
    /// - program that core with one task descriptor
    /// - return immediately without waiting for completion
    ///
    /// The blocking behavior now lives outside this function in the OS-side
    /// queue scheduler. That layer may sleep the submitter thread, harvest IRQ
    /// completions, and dispatch follow-up tasks later on.
    ///
    /// Legacy userspace may leave `task_array_dma_address` as zero. The previous submit
    /// path forwarded that zero unchanged, so the queue scheduler preserves the
    /// same behavior instead of rejecting the submit.
    pub fn submit_ioctrl_step(
        &mut self,
        core_slot: usize,
        submit_flags: u32,
        task_total: u32,
        task_dma_base: u64,
        subcore_slot: u8,
        task_index: u32,
        task: &mut RknpuTask,
    ) -> Result<(), RknpuError> {
        if core_slot >= self.base.len() {
            debug!(
                "[NPU] submit_ioctrl_step rejected invalid core_slot={} base_len={} task_index={}",
                core_slot,
                self.base.len(),
                task_index
            );
            return Err(RknpuError::InvalidParameter);
        }

        if task_total == 0 {
            debug!(
                "[NPU] submit_ioctrl_step rejected empty submit core={} task_array_dma_address={:#x}",
                core_slot, task_dma_base
            );
            return Err(RknpuError::InvalidParameter);
        }

        if task_index >= task_total {
            debug!(
                "[NPU] submit_ioctrl_step rejected task_index={} >= task_number={} core={}",
                task_index, task_total, core_slot
            );
            return Err(RknpuError::InvalidParameter);
        }

        // `task_array_dma_address` belongs to the legacy DMA task-array contract. The
        // new queue path still preserves the visible field, but it may remain
        // zero while the real task descriptor is supplied through `task`.
        let task_dma_addr = if task_dma_base == 0 {
            0
        } else {
            task_dma_base + u64::from(task_index).saturating_mul(size_of::<RknpuTask>() as u64)
        };

        // Reset completion state before the task enters hardware. The final
        // `int_status` will be written back later by the scheduler harvest path.
        task.int_status = 0;
        let regcmd_addr = task.regcmd_addr;
        let regcfg_amount = task.regcfg_amount;
        let int_mask = task.int_mask;
        debug!(
            "[NPU] submit_ioctrl_step dispatch core={} subcore={} task_index={} task_number={} flags={:#x} task_array_dma_address={:#x} task_dma_addr={:#x} regcmd_addr={:#x} regcfg_amount={} int_mask={:#x}",
            core_slot,
            subcore_slot,
            task_index,
            task_total,
            submit_flags,
            task_dma_base,
            task_dma_addr,
            regcmd_addr,
            regcfg_amount,
            int_mask
        );

        // Drain stale completion state before rebinding this core. Otherwise an
        // old IRQ could be misattributed to the new dispatch.
        self.base[core_slot].drain_pending_interrupts();

        if let Err(err) = self.base[core_slot].start_execute_one(
            core_slot,
            &self.data,
            task,
            submit_flags,
            task_dma_addr,
        ) {
            debug!(
                "[NPU] submit_ioctrl_step start_execute_one failed core={} task_index={}: {:?}",
                core_slot, task_index, err
            );
            return Err(err);
        }

        debug!(
            "[NPU] submit_ioctrl_step programmed hardware core={} subcore={} task_index={}",
            core_slot, subcore_slot, task_index
        );

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::{
        RknpuScheduleTraceQuery, RknpuScheduleTraceRecord, RknpuSchedulerStateSnapshot,
        RknpuSubmit, RknpuSubmitTraceQuery, RknpuSubmitTraceRecord,
        RknpuWorkerYieldTraceQuery, RknpuWorkerYieldTraceRecord,
    };
    use crate::{Rknpu, RknpuConfig, RknpuError, RknpuTask, RknpuType};
    use alloc::{vec, vec::Vec};
    use core::ptr::NonNull;

    const FAKE_MMIO_LEN: usize = 0x10000;

    #[test]
    fn mem_create_abi_size_is_stable() {
        assert_eq!(core::mem::size_of::<super::RknpuMemCreate>(), 48);
    }

    #[test]
    fn submit_trace_abi_size_is_stable() {
        // DRM ioctl 编码包含结构尺寸。这里同时固定 Rust 侧四阶段记录、调度
        // 事件和状态快照的大小，必须与 rknpu-ioctl.h 的 C 静态断言一致。
        assert_eq!(core::mem::size_of::<RknpuSubmitTraceRecord>(), 48);
        assert_eq!(core::mem::size_of::<RknpuSubmitTraceQuery>(), 24);
        assert_eq!(core::mem::size_of::<RknpuWorkerYieldTraceRecord>(), 40);
        assert_eq!(core::mem::size_of::<RknpuWorkerYieldTraceQuery>(), 32);
        assert_eq!(core::mem::size_of::<RknpuScheduleTraceRecord>(), 88);
        assert_eq!(core::mem::size_of::<RknpuScheduleTraceQuery>(), 40);
        assert_eq!(core::mem::size_of::<RknpuSchedulerStateSnapshot>(), 40);
    }

    fn build_fake_rknpu() -> (Rknpu, Vec<Vec<u8>>) {
        let mut mmios = vec![vec![0_u8; FAKE_MMIO_LEN]; 3];
        let base_addrs = mmios
            .iter_mut()
            .map(|mmio| NonNull::new(mmio.as_mut_ptr()).unwrap())
            .collect::<Vec<_>>();
        let config = RknpuConfig {
            rknpu_type: RknpuType::Rk3588,
        };

        (Rknpu::new(&base_addrs, config), mmios)
    }

    fn fake_submit(tasks: &mut [RknpuTask], task_number: u32) -> RknpuSubmit {
        let mut submit = RknpuSubmit::default();
        submit.task_array_cpu_address = tasks.as_mut_ptr() as u64;
        submit.task_array_dma_address = 0x2000;
        submit.task_number = task_number;
        submit.core_mask = 0x1;
        submit.subcore_task[0].task_start = 0;
        submit.subcore_task[0].task_number = task_number;
        submit
    }

    #[test]
    fn submit_step_rejects_invalid_core_slot() {
        let (mut npu, _mmios) = build_fake_rknpu();
        let mut tasks = [RknpuTask::default()];
        let submit = fake_submit(&mut tasks, 1);

        let err = npu
            .submit_ioctrl_step(
                3,
                submit.flags,
                submit.task_number,
                submit.task_array_dma_address,
                0,
                0,
                &mut tasks[0],
            )
            .unwrap_err();

        assert_eq!(err, RknpuError::InvalidParameter);
    }

    #[test]
    fn submit_step_dispatches_one_task() {
        let (mut npu, _mmios) = build_fake_rknpu();
        let mut tasks = [RknpuTask {
            int_mask: 0x300,
            ..RknpuTask::default()
        }];
        let submit = fake_submit(&mut tasks, 1);

        npu.submit_ioctrl_step(
            0,
            submit.flags,
            submit.task_number,
            submit.task_array_dma_address,
            0,
            0,
            &mut tasks[0],
        )
        .unwrap();

        assert_eq!(tasks[0].int_status, 0);
        assert_eq!(
            npu.base[0]
                .irq_status
                .load(core::sync::atomic::Ordering::Acquire),
            0
        );
    }

    #[test]
    fn submit_step_rejects_out_of_range_task_index() {
        let (mut npu, _mmios) = build_fake_rknpu();
        let mut tasks = [RknpuTask {
            int_mask: 0x300,
            ..RknpuTask::default()
        }];
        let submit = fake_submit(&mut tasks, 1);
        let err = npu
            .submit_ioctrl_step(
                0,
                submit.flags,
                submit.task_number,
                submit.task_array_dma_address,
                0,
                1,
                &mut tasks[0],
            )
            .unwrap_err();

        assert_eq!(err, RknpuError::InvalidParameter);
    }

    #[test]
    fn submit_step_accepts_legacy_zero_task_array_dma_address() {
        let (mut npu, mmios) = build_fake_rknpu();
        let mut tasks = [RknpuTask {
            int_mask: 0x300,
            ..RknpuTask::default()
        }];
        let mut submit = fake_submit(&mut tasks, 1);
        submit.task_array_dma_address = 0;

        npu.submit_ioctrl_step(
            0,
            submit.flags,
            submit.task_number,
            submit.task_array_dma_address,
            0,
            0,
            &mut tasks[0],
        )
        .unwrap();

        let task_dma_reg = u32::from_le_bytes(mmios[0][0x34..0x38].try_into().unwrap());
        assert_eq!(task_dma_reg, 0);
    }
}
