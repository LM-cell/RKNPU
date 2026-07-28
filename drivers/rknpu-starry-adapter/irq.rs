use crate::RknpuIrqHandler;
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

/// IRQ entry point for NPU core 0.
fn handle_npu_irq_core0(_irq: usize) {
    unsafe {
        if let Some(h) = &*NPU_IRQ_HANDLERS[0].0.get() {
            h.handle();
        }
    }
}

/// IRQ entry point for NPU core 1.
fn handle_npu_irq_core1(_irq: usize) {
    unsafe {
        if let Some(h) = &*NPU_IRQ_HANDLERS[1].0.get() {
            h.handle();
        }
    }
}

/// IRQ entry point for NPU core 2.
fn handle_npu_irq_core2(_irq: usize) {
    unsafe {
        if let Some(h) = &*NPU_IRQ_HANDLERS[2].0.get() {
            h.handle();
        }
    }
}
