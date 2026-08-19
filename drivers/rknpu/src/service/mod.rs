//! High-level RKNPU service layer.
//!
//! This module sits above the low-level MMIO/GEM/task code and below OS
//! device-node adapters. It owns:
//!
//! - blocking-submit scheduling
//! - per-submit waiter management
//! - RKNPU-specific ioctl payload handling
//! - small trait boundaries for OS services such as userspace copy, sleeping,
//!   and worker spawning

use alloc::sync::Arc;

mod error;
mod ioctl;
mod platform;
mod scheduler;

pub use error::RknpuServiceError;
pub use ioctl::{RknpuCmd, RknpuUserAction};
pub use platform::{
    RknpuDeviceAccess, RknpuPlatform, RknpuSchedulerRuntime, RknpuSubmitWaiter, RknpuUserMemory,
    RknpuWorkerListener, RknpuWorkerSignal,
};
pub use scheduler::CompletedSubmit;

use scheduler::RknpuScheduler;

/// 调度 Worker 等待 NPU completion 的方式。
///
/// 最后修改日期：2026-08-19。该配置只改变 inflight 等待方式，不改变
/// Submit、调度队列、IRQ 状态读取和完成回收流程。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RknpuWorkerWaitMode {
    /// 通过 completion IRQ 唤醒阻塞中的 Worker。
    IrqEvent,
    /// 使用原 Worker `yield_now()` 轮询方式建立性能对照基线。
    YieldPolling,
}

/// Shared high-level RKNPU service instance.
///
/// The service is intentionally not a crate-global singleton. Each embedding
/// OS or test harness can construct and own an instance with its own platform
/// adapter.
pub struct RknpuService<P: RknpuPlatform> {
    inner: Arc<RknpuServiceInner<P>>,
}

struct RknpuServiceInner<P: RknpuPlatform> {
    platform: P,
    scheduler: RknpuScheduler<P>,
    /// 服务创建后固定，禁止运行过程中切换等待方式破坏实验口径。
    worker_wait_mode: RknpuWorkerWaitMode,
}

impl<P: RknpuPlatform> Clone for RknpuService<P> {
    /// Clone the shared service handle by cloning the inner `Arc`.
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl<P: RknpuPlatform> RknpuService<P> {
    /// Build a new service around one platform adapter.
    pub fn new(platform: P) -> Self {
        Self::new_with_worker_wait_mode(platform, RknpuWorkerWaitMode::IrqEvent)
    }

    /// 使用指定的 completion 等待方式创建服务。
    ///
    /// 最后修改日期：2026-08-19。该入口用于在同一份调度代码上对比
    /// Event/Waker 与 `yield_now()`；配置只在初始化时设置一次。
    pub fn new_with_worker_wait_mode(
        platform: P,
        worker_wait_mode: RknpuWorkerWaitMode,
    ) -> Self {
        let scheduler = RknpuScheduler::new(&platform);
        Self {
            inner: Arc::new(RknpuServiceInner {
                platform,
                scheduler,
                worker_wait_mode,
            }),
        }
    }
}

#[cfg(test)]
mod tests;
