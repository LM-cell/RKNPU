//! Error type used by RKNPU driver operations.
//!
//! Every fallible operation in the driver, from memory allocation to task
//! submission and interrupt handling, returns `Result<T, RknpuError>`. The enum
//! is `no_std`-friendly and stays close to the error surface of the original
//! Linux-style driver.

use core::fmt::Display;

/// Unified error type for all RKNPU driver operations.
///
/// Typical examples:
///
/// | Scenario | Likely variant |
/// |---|---|
/// | Userspace passed a bad ioctl payload | `InvalidParameter` |
/// | DMA allocation failed | `OutOfMemory` |
/// | The NPU did not finish in time | `Timeout` |
/// | IRQ status reported a fault bit | `TaskError` / `HardwareFault` |
/// | A GEM handle no longer exists | `InvalidHandle` |
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RknpuError {
    /// The caller supplied an invalid address, size, core mask, or flag value.
    InvalidParameter,
    /// The NPU did not finish before the configured deadline.
    Timeout,
    /// DMA-buffer or internal-state allocation failed.
    OutOfMemory,
    /// The requested operation is not supported on this NPU variant.
    NotSupported,
    /// Another task is already running on the requested core.
    DeviceBusy,
    /// The NPU has not been initialized or powered up yet.
    DeviceNotReady,
    /// Generic hardware error when no better classification is available.
    DeviceError,
    /// The interrupt status register reported a fatal hardware fault.
    HardwareFault,
    /// IOMMU translation fault while the NPU accessed an unmapped address.
    IommuError,
    /// DMA transfer failed, for example because of address or coherence issues.
    DmaError,
    /// Task completion status did not match the expected interrupt mask.
    TaskError,
    /// GEM or memory-pool operation failed.
    MemoryError,
    /// The requested GEM object handle was not found.
    InvalidHandle,
    /// The resource is temporarily unavailable and the caller should retry.
    TryAgain,
    /// A blocking wait was interrupted.
    Interrupted,
    /// The caller lacks permission for this operation.
    PermissionDenied,
    /// Internal driver logic error. This should not normally happen.
    InternalError,
}

impl Display for RknpuError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl core::error::Error for RknpuError {}
