
/// Physical address in the CPU or system address space.
pub type PhysAddr = u64;

/// DMA or bus address as seen by the NPU hardware.
///
/// After a DMA buffer is allocated (see [`crate::gem::GemPool`]), the driver
/// programs this address into NPU registers so hardware knows where to fetch
/// inputs and where to place outputs.
pub type DmaAddr = u64;

/// Monotonic timestamp used to measure NPU job execution time.
pub type TimeStamp = u64;

/// Error type returned by low-level OS abstraction helpers.
///
/// These values represent failures such as memory allocation problems, invalid
/// parameters, device communication issues, or timeouts. Higher-level driver
/// errors live in [`crate::err`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OsalError {
    /// Memory allocation failed, for example while creating DMA buffers or page
    /// tables.
    OutOfMemory,
    /// The caller provided an invalid address, size, or alignment.
    InvalidParameter,
    /// A blocking operation such as waiting for an NPU interrupt timed out.
    TimeoutError,
    /// The NPU hardware reported a non-recoverable error.
    DeviceError,
    /// The requested feature is not supported by this NPU variant.
    NotSupported,
}
