
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
    /// Reserved or advisory core-mask field.
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
    use super::RknpuSubmit;
    use crate::{Rknpu, RknpuConfig, RknpuError, RknpuTask, RknpuType};
    use alloc::vec::Vec;
    use core::ptr::NonNull;

    const FAKE_MMIO_LEN: usize = 0x10000;

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
