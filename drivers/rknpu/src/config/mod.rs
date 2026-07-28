//! RKNPU configuration bindings translated from the original C
//! `struct rknpu_config`.
//!
//! This module keeps a small `#[repr(C)]`-compatible Rust view of the board or
//! SoC configuration used by the rest of the driver.

/// Supported RKNPU hardware variants.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum RknpuType {
    /// Rockchip RK3588 NPU.
    Rk3588,
}

/// Top-level static configuration for one RKNPU instance.
#[derive(Debug, Clone)]
pub struct RknpuConfig {
    /// Chip variant used to select hardware-specific register and capability
    /// data.
    pub rknpu_type: RknpuType,
}
