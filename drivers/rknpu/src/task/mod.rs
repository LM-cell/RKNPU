//! High-level task submission types.
//!
//! # Register-command buffers
//!
//! The NPU PC block works by DMA-reading a buffer of 64-bit register commands.
//! Each regcmd is packed with `npu_op(block, value, reg)` and tells the PC
//! block which register to write and which value to place there.
//!
//! ```text
//!  63        48 47        16 15         0
//!  ┌──────────┬─────────────┬───────────┐
//!  │  opcode  │    value    │  register │   = one `u64` regcmd
//!  │ (block)  │   (32-bit)  │  (offset) │
//!  └──────────┴─────────────┴───────────┘
//! ```
//!
//! One task is a contiguous range of regcmds that programs the full
//! CNA -> MAC -> DPU pipeline for one neural-network layer. Several tasks are
//! laid out back-to-back inside one DMA buffer.
//!
//! # Submission hierarchy
//!
//! ```text
//!  Submit                  (owns DMA buffer + operation list)
//!    └─ SubmitRef          (lightweight view passed to register code)
//!         └─ SubmitBase    (core index, interrupt mask, flags)
//! ```

use alloc::vec::Vec;
use dma_api::{DVec, Direction};

use crate::{JobMode, op::Operation};

pub mod cna;
pub(crate) mod def;
pub mod dpu;
pub mod op;
pub mod taskqueen;
pub use taskqueen::*;

/// Shared parameters for a batch of tasks, independent of the actual regcmd
/// buffer.
#[derive(Debug, Clone)]
pub struct SubmitBase {
    /// Execution-mode flags such as `PC`, `BLOCK`, or `PINGPONG`.
    pub flags: JobMode,
    /// DMA address of the `RknpuTask[]` descriptor array on the ioctl path.
    pub task_array_dma_address: u32,
    /// Target NPU core index.
    pub core_idx: usize,
    /// Interrupt mask to wait for after launch.
    pub int_mask: u32,
    /// Bits written to `INTERRUPT_CLEAR` before launch.
    pub int_clear: u32,
    /// Number of 64-bit regcmd words per task.
    pub regcfg_amount: u32,
}

/// Lightweight view of a submission batch that does not own DMA memory.
#[derive(Debug, Clone)]
pub struct SubmitRef {
    /// Shared submission parameters.
    pub base: SubmitBase,
    /// Number of tasks or layers in this batch.
    pub task_number: usize,
    /// DMA address of the first regcmd in the batch.
    pub regcmd_base_addr: u32,
}

/// Owned submission object with DMA-backed register commands and operations.
///
/// This type is built by higher-level Rust code, for example bare-metal demos,
/// rather than by the raw ioctl path that starts from `RknpuSubmit`.
///
/// Construction flow:
///
/// ```text
///  1. The caller builds `Vec<Operation>`.
///  2. `Submit::new()` allocates one large DMA buffer for all regcmds.
///  3. Each `Operation::fill_regcmd()` fills its slice.
///  4. The buffer is flushed so the NPU can DMA-read it.
///  5. `Rknpu::submit()` converts it into `SubmitRef` and calls `submit_pc()`.
/// ```
pub struct Submit {
    /// Shared submission parameters.
    pub base: SubmitBase,
    /// Contiguous DMA buffer containing all task regcmd streams.
    pub regcmd_all: DVec<u64>,
    /// Operation objects, one per task or layer.
    pub tasks: Vec<Operation>,
}

impl Submit {
    /// Build a submission from an operation list.
    pub fn new(tasks: Vec<Operation>) -> Self {
        let base = SubmitBase {
            flags: JobMode::PC | JobMode::BLOCK | JobMode::PINGPONG,
            task_array_dma_address: 0,
            core_idx: 0,
            int_mask: 0x300, // wait for DPU completion
            int_clear: 0x1ffff,
            regcfg_amount: tasks[0].reg_amount(),
        };

        // Allocate one large DMA buffer: regcfg_amount words times task count.
        let regcmd_all: DVec<u64> = DVec::zeros(
            u32::MAX as _,
            base.regcfg_amount as usize * tasks.len(),
            0x1000,
            Direction::Bidirectional,
        )
        .unwrap();

        assert!(
            regcmd_all.bus_addr() <= u32::MAX as u64,
            "regcmd base address exceeds u32"
        );

        // Fill each task slice with register commands.
        let amount = base.regcfg_amount as usize;
        for (i, task) in tasks.iter().enumerate() {
            let regcmd = unsafe {
                core::slice::from_raw_parts_mut(regcmd_all.as_ptr().add(i * amount), amount)
            };
            task.fill_regcmd(regcmd);
        }
        // Flush CPU cache so the NPU can DMA-read the commands.
        regcmd_all.confirm_write_all();

        Self {
            base,
            regcmd_all,
            tasks,
        }
    }

    /// Create a lightweight reference view for the register layer.
    pub fn as_ref(&self) -> SubmitRef {
        SubmitRef {
            base: self.base.clone(),
            task_number: self.tasks.len(),
            regcmd_base_addr: self.regcmd_all.bus_addr() as _,
        }
    }
}
