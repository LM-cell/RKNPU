//! Minimal completion records published by the driver.

/// RK3588 RKNPU currently supports up to three hardware cores.
pub const NPU_MAX_CORES: usize = 3;

/// One raw completion harvested from one hardware core.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CoreCompletion {
    pub core_slot: u8,
    pub observed_irq_status: u32,
}
