use alloc::vec::Vec;

use crate::{Rknpu, RknpuError};

use super::error::RknpuServiceError;

/// OS hook that grants temporary mutable access to the low-level RKNPU device.
///
/// The service layer never owns a global driver singleton itself. Instead, the
/// embedding OS implements this trait and decides how the low-level
/// [`Rknpu`] instance is stored and locked.
pub trait RknpuDeviceAccess: Send + Sync + 'static {
    /// Borrow the low-level device for one operation.
    fn with_device<T, F>(&self, f: F) -> Result<T, RknpuServiceError>
    where
        F: FnOnce(&mut Rknpu) -> Result<T, RknpuError>;
}

/// OS hook used to copy ioctl payloads between userspace and kernel memory.
pub trait RknpuUserMemory: Send + Sync + 'static {
    /// Copy one userspace buffer into kernel-owned memory.
    fn copy_from_user(
        &self,
        dst: *mut u8,
        src: *const u8,
        size: usize,
    ) -> Result<(), RknpuServiceError>;

    /// Copy one kernel-owned buffer back into userspace memory.
    fn copy_to_user(
        &self,
        dst: *mut u8,
        src: *const u8,
        size: usize,
    ) -> Result<(), RknpuServiceError>;
}

/// Per-submit blocking primitive used by the blocking submit ioctl path.
pub trait RknpuSubmitWaiter: Send + Sync + 'static {
    /// Block until the associated submit becomes terminal.
    fn wait(&self) -> Result<(), RknpuServiceError>;

    /// Wake the waiter after terminal completion.
    fn complete(&self);
}

/// One prepared worker-sleep listener.
///
/// The service uses a two-phase "listen, re-check, then wait" sequence to
/// avoid lost wake-ups when the singleton worker goes idle.
pub trait RknpuWorkerListener {
    /// Sleep until the corresponding signal fires.
    fn wait(self);
}

/// Global wake-up object for the singleton scheduler worker.
pub trait RknpuWorkerSignal: Send + Sync + 'static {
    /// Prepared listener type created before the worker re-checks work.
    type Listener: RknpuWorkerListener;

    /// Register a listener for the next worker wake-up.
    fn listen(&self) -> Self::Listener;

    /// Wake one sleeping worker.
    fn notify_one(&self);
}

/// OS runtime hooks needed by the scheduler.
pub trait RknpuSchedulerRuntime: Send + Sync + 'static {
    /// Concrete waiter type created for each blocking submit.
    type Waiter: RknpuSubmitWaiter;
    /// Concrete worker signal type used by the singleton worker.
    type WorkerSignal: RknpuWorkerSignal;

    /// Create a fresh waiter for one submit.
    fn new_waiter(&self) -> Self::Waiter;

    /// Create the global worker wake-up primitive.
    fn new_worker_signal(&self) -> Self::WorkerSignal;

    /// Spawn the singleton worker thread/task.
    fn spawn_worker<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static;

    /// Yield execution while hardware is inflight or scheduling is stalled.
    fn yield_now(&self);
}

/// Convenience bound used by [`crate::service::RknpuService`].
pub trait RknpuPlatform: RknpuDeviceAccess + RknpuUserMemory + RknpuSchedulerRuntime {}

impl<T> RknpuPlatform for T where T: RknpuDeviceAccess + RknpuUserMemory + RknpuSchedulerRuntime {}
