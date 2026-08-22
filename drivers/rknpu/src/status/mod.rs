//! NPU 核心完成状态，最后修改日期：2026-08-19。

/// RK3588 RKNPU 最多包含三个硬件核心。
pub const NPU_MAX_CORES: usize = 3;

/// Worker 从一个硬件核心收割的一条原始完成状态。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CoreCompletion {
    pub core_slot: u8,
    pub observed_irq_status: u32,
    /// 与 observed_irq_status 同一次发布对应的 IRQ 入口时间。
    pub irq_timestamp_ns: u64,
}
