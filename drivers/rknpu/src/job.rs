

#![allow(dead_code)]

// Hardware limits.

/// Maximum number of hardware cores supported by the RK3588 NPU IP.
///
/// Core 0 is always present. Cores 1 and 2 may be fused off on some SKUs.
pub const RKNPU_MAX_CORES: usize = 3;

/// Maximum number of per-core task ranges accepted by one submit ioctl.
pub const RKNPU_MAX_SUBCORE_TASKS: usize = 5;

// Core-selection bitmasks.

/// Let the driver pick a suitable core automatically.
pub const RKNPU_CORE_AUTO_MASK: u32 = 0x00;
/// Select core 0, which always exists.
pub const RKNPU_CORE0_MASK: u32 = 0x01;
/// Select core 1 when available.
pub const RKNPU_CORE1_MASK: u32 = 0x02;
/// Select core 2 when available.
pub const RKNPU_CORE2_MASK: u32 = 0x04;

// Job flags passed in from userspace.

/// Run in PC mode, where the hardware command parser fetches register commands.
pub const RKNPU_JOB_PC: u32 = 1 << 0;
/// Return immediately without waiting for the NPU to finish.
pub const RKNPU_JOB_NONBLOCK: u32 = 1 << 1;
/// Enable ping-pong buffering to overlap DMA and compute.
pub const RKNPU_JOB_PINGPONG: u32 = 1 << 2;
/// Wait on an external fence before execution starts.
pub const RKNPU_JOB_FENCE_IN: u32 = 1 << 3;
/// Signal an external fence when execution completes.
pub const RKNPU_JOB_FENCE_OUT: u32 = 1 << 4;

/// Hardware task descriptor consumed by the PC block.
///
/// The PC engine DMA-reads an array of these descriptors from
/// `task_dma_base_addr`. For each entry it loads `regcfg_amount` 64-bit
/// register commands from `regcmd_addr`, programs the pipeline, and waits for
/// the interrupt bits described by `int_mask`.
///
/// The layout must match hardware exactly, hence `#[repr(C, packed)]`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[repr(C, packed)]
pub struct RknpuTask {
    /// Per-task job flags, usually copied from `RKNPU_JOB_*`.
    pub flags: u32,
    /// Operation index inside the compiled model graph.
    pub op_idx: u32,
    /// Bitmask of hardware blocks enabled for this task, for example
    /// `CNA | MAC | DPU` on a convolution layer.
    pub enable_mask: u32,
    /// Interrupt bits the driver should wait for after launching this task.
    pub int_mask: u32,
    /// Bits written to `INTERRUPT_CLEAR` before the task starts.
    pub int_clear: u32,
    /// Actual interrupt status observed on completion. Filled in by the driver.
    pub int_status: u32,
    /// Number of 64-bit register commands to DMA-read for this task.
    pub regcfg_amount: u32,
    /// Byte offset of this task's command stream when several tasks share one
    /// command buffer.
    pub regcfg_offset: u32,
    /// DMA address of this task's register-command buffer.
    pub regcmd_addr: u64,
}

/// Range of tasks assigned to one logical subcore slot.
///
/// In multi-core mode, different ranges may be sent to different cores so they
/// can run in parallel.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[repr(C)]
pub struct RknpuSubcoreTask {
    /// Index of the first task in the task array.
    pub task_start: u32,
    /// Number of contiguous tasks starting at `task_start`.
    pub task_number: u32,
}

bitflags::bitflags! {
    /// Internal job-mode flags used on the Rust side of the driver.
    ///
    /// - `PC`: use the hardware command parser
    /// - `SLAVE`: legacy software-driven mode
    /// - `BLOCK`: wait synchronously for completion
    /// - `NONBLOCK`: return early and rely on a fence later
    /// - `PINGPONG`: enable double-buffered execution
    #[repr(C)]
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct  JobMode: u32 {
        const SLAVE =  0;
        const PC = 1 << 0;
        const BLOCK = 0 << 1;
        const NONBLOCK = 1 << 1;
        const PINGPONG = 1 << 2;
        const FENCE_IN = 1 << 3;
        const FENCE_OUT = 1 << 4;
    }
}

/// Convert a core index into its corresponding selection bit.
pub const fn core_mask_from_index(index: usize) -> u32 {
    match index {
        0 => RKNPU_CORE0_MASK,
        1 => RKNPU_CORE1_MASK,
        2 => RKNPU_CORE2_MASK,
        _ => 0,
    }
}

/// Count how many cores are selected in a core-mask bitfield.
pub const fn core_count_from_mask(mask: u32) -> u32 {
    mask.count_ones()
}
