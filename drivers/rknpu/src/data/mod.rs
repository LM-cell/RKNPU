

use core::fmt::Debug;

use crate::{Rknpu, RknpuError, RknpuType};

/// Return a mask with the lowest `n` bits set.
///
/// For example, `dma_bit_mask(40)` yields `0xFF_FFFF_FFFF`, which matches a
/// 40-bit DMA address space.
pub(crate) const fn dma_bit_mask(n: u32) -> u64 {
    if n >= 64 {
        u64::MAX
    } else {
        (1u64 << n) - 1u64
    }
}

/// Register offsets for hardware read and write byte counters.
///
/// Some NPU variants expose counters for traffic accounting. These offsets are
/// used when the driver wants to read bandwidth statistics from hardware.
#[derive(Copy, Clone, Debug, Default)]
pub struct RknpuAmountData {
    /// Register offset that clears all counters.
    pub offset_clr_all: u16,
    /// Register offset of the "data write" counter.
    pub offset_dt_wr: u16,
    /// Register offset of the "data read" counter.
    pub offset_dt_rd: u16,
    /// Register offset of the "weight read" counter.
    pub offset_wt_rd: u16,
}

/// Chip-variant data that differs across RKNPU generations.
///
/// The register layer and submit path query these values when programming the
/// PC block, validating task counts, or reporting platform capabilities.
#[derive(Debug, Clone)]
pub(crate) struct RknpuData {
    /// MMIO base address of the bandwidth-priority register block.
    pub bw_priority_addr: u32,
    /// Length in bytes of the bandwidth-priority register block.
    pub bw_priority_length: u32,
    /// Maximum DMA address visible to the NPU, for example a 40-bit space.
    pub dma_mask: u64,
    /// Scale factor for the PC `REGISTER_AMOUNTS` field.
    /// RK3588 uses `2`, meaning one amount unit describes two regcmd words.
    pub pc_data_amount_scale: u32,
    /// Width in bits of the task-count field inside `TASK_CON`.
    /// RK3588 uses 12 bits, so one submit can carry at most 4095 tasks.
    pub pc_task_number_bits: u32,
    /// Bitmask used to extract the task-count field.
    pub pc_task_number_mask: u32,
    /// Register offset used by the PC block to report task completion state.
    pub pc_task_status_offset: u32,
    /// Non-zero when the PC block exposes legacy DMA control registers.
    pub pc_dma_ctrl: u32,
    /// Physical address of local on-chip SRAM, or zero if absent.
    pub nbuf_phyaddr: u64,
    /// Size in bytes of the local SRAM buffer.
    pub nbuf_size: u64,
    /// Maximum number of tasks the PC block can accept in one batch.
    pub max_submit_number: u64,
    /// Bitmask of available hardware cores.
    pub core_mask: u32,
    /// Static IRQ descriptor table, one entry per visible core.
    pub irqs: &'static [NpuIrq],
    /// Top-level traffic counter offsets, if supported.
    pub amount_top: Option<RknpuAmountData>,
    /// Per-core traffic counter offsets, if supported.
    pub amount_core: Option<RknpuAmountData>,
    /// Optional platform-specific status initialization hook.
    pub state_init: Option<fn(&mut dyn core::any::Any) -> Result<(), RknpuError>>,
    /// Optional cache scatter-gather initialization hook.
    pub cache_sgt_init: Option<fn(&mut dyn core::any::Any) -> Result<(), RknpuError>>,
}

impl RknpuData {
    /// Construct variant data for the selected NPU type.
    pub fn new(ty: RknpuType) -> Self {
        match ty {
            RknpuType::Rk3588 => Self::new_3588(),
        }
    }

    /// Build the hard-coded RK3588 variant description.
    fn new_3588() -> Self {
        Self {
            bw_priority_addr: 0x0,
            bw_priority_length: 0x0,
            dma_mask: dma_bit_mask(40), // 40-bit DMA address space
            pc_data_amount_scale: 2,    // one unit = two regcmd u64 words
            pc_task_number_bits: 12,    // TASK_CON[11:0] carries the task count
            pc_task_number_mask: 0xfff,
            pc_task_status_offset: 0x3c, // TASK_STATUS register offset
            pc_dma_ctrl: 0,              // no legacy DMA control block
            irqs: RK3588_IRQS,           // one IRQ line per visible core
            nbuf_phyaddr: 0,
            nbuf_size: 0,
            max_submit_number: (1u64 << 12) - 1, // up to 4095 tasks
            core_mask: 0x7,                      // cores 0, 1, and 2 exist
            amount_top: None,                    // traffic counters not wired yet
            amount_core: None,
            state_init: None,
            cache_sgt_init: None,
        }
    }
}

/// Static RK3588 IRQ table, with one line per NPU core.
const RK3588_IRQS: &[NpuIrq] = &[
    NpuIrq {
        name: "npu0_irq",
        irq_hdl: |_, _| None,
    },
    NpuIrq {
        name: "npu1_irq",
        irq_hdl: |_, _| None,
    },
    NpuIrq {
        name: "npu2_irq",
        irq_hdl: |_, _| None,
    },
];

/// Interrupt descriptor for one NPU core.
pub struct NpuIrq {
    /// Human-readable IRQ name, usually matching the device-tree entry.
    pub name: &'static str,
    /// Callback invoked when this core's interrupt fires.
    pub irq_hdl: fn(&mut Rknpu, irq: usize) -> Option<()>,
}

impl Debug for NpuIrq {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("NpuIrq").field("name", &self.name).finish()
    }
}
