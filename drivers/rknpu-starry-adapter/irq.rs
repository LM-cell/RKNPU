// NPU IRQ 状态发布与 worker 唤醒，最后修改日期：2026-08-17。
use crate::{RknpuIrqHandler, card1::notify_rknpu_worker};
use core::cell::UnsafeCell;

/// Mutable storage slot used to hold one installed IRQ handler.
pub struct IrqSlot(pub UnsafeCell<Option<RknpuIrqHandler>>);

/// The slot may be shared across interrupt and probe contexts.
unsafe impl Sync for IrqSlot {}

/// Global per-core IRQ handler table populated during probe.
pub static NPU_IRQ_HANDLERS: [IrqSlot; 3] = [
    IrqSlot(UnsafeCell::new(None)),
    IrqSlot(UnsafeCell::new(None)),
    IrqSlot(UnsafeCell::new(None)),
];

/// Static trampoline table registered with the platform IRQ framework.
pub const NPU_IRQ_FNS: [fn(usize); 3] = [
    handle_npu_irq_core0,
    handle_npu_irq_core1,
    handle_npu_irq_core2,
];

/// 处理一个 NPU 核心的 completion IRQ，并在存在有效状态时唤醒调度 worker。
///
/// IRQ 中只调用底层 `handle()` 读取、清除并发布状态，再发送 Event 通知；
/// Task 回收、Ready 队列操作和后续派发全部由被唤醒的 worker 完成。
fn handle_npu_irq(core_slot: usize) {
    let status = unsafe {
        // 安全性：probe 在注册 IRQ 前写入对应槽位，注册后不再修改该槽位。
        // IRQ 处理器只通过共享引用调用内部原子状态发布逻辑。
        (&*NPU_IRQ_HANDLERS[core_slot].0.get())
            .as_ref()
            .map_or(0, RknpuIrqHandler::handle)
    };
    if status != 0 {
        notify_rknpu_worker();
    }
}

/// NPU core0 的静态 IRQ 入口。
fn handle_npu_irq_core0(_irq: usize) {
    handle_npu_irq(0);
}

/// NPU core1 的静态 IRQ 入口。
fn handle_npu_irq_core1(_irq: usize) {
    handle_npu_irq(1);
}

/// NPU core2 的静态 IRQ 入口。
fn handle_npu_irq_core2(_irq: usize) {
    handle_npu_irq(2);
}
