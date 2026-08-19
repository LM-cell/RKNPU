// Submit延迟平台接口，最后修改日期：2026-08-04。

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

    /// 唤醒一个阻塞中的调度 worker。
    ///
    /// 最后修改日期：2026-08-17。该方法会从 NPU IRQ 上下文调用，平台实现
    /// 不得睡眠、分配内存或获取调度器业务锁。
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

    /// Read the monotonic clock used by submit tracing.
    fn monotonic_time_ns(&self) -> u64;

    /// Spawn the singleton worker thread/task.
    fn spawn_worker<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static;

    /// 让出当前 CPU，供 stalled 重试和可选的 completion 轮询方式使用。
    ///
    /// 最后修改日期：2026-08-19。Event/Waker 模式只在 stalled 状态调用；
    /// YieldPolling 对照模式还会在等待 NPU completion 时调用。
    fn yield_now(&self);
}

/// Convenience bound used by [`crate::service::RknpuService`].
pub trait RknpuPlatform: RknpuDeviceAccess + RknpuUserMemory + RknpuSchedulerRuntime {}

impl<T> RknpuPlatform for T where T: RknpuDeviceAccess + RknpuUserMemory + RknpuSchedulerRuntime {}
