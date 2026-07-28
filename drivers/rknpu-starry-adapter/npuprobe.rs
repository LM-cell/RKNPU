use rdrive::module_driver;
use rdrive::register::FdtInfo;
use rdrive::PlatformDevice;
use rdrive::probe::OnProbeError;
use crate::RknpuConfig;
use crate::RknpuType;
use alloc::vec::Vec;
use crate::Rknpu;
use crate::tool::iomap;
use crate::enable_pm;
use crate::irq::NPU_IRQ_HANDLERS;
use crate::irq::NPU_IRQ_FNS;
#[cfg(target_arch = "aarch64")]
use crate::power::irq_yield;

/// Convert an ARM GIC FDT interrupt tuple `[type, number, flags]` into a GIC
/// IRQ number.
///
/// The current driver only needs the interrupt type and index:
///
/// - type `0` (SPI): IRQ = `number + 32`
/// - type `1` (PPI): IRQ = `number + 16`
fn fdt_irq_to_gic_num(cells: &[u32]) -> usize {
    let irq_type = cells[0];
    let irq_num = cells[1] as usize;
    match irq_type {
        0 => irq_num + 32, // SPI (Shared Peripheral Interrupt)
        1 => irq_num + 16, // PPI (Private Peripheral Interrupt)
        _ => panic!("Unknown GIC interrupt type: {}", irq_type),
    }
}

/// Probe and register one Rockchip RKNPU instance from device-tree metadata.
///
/// This routine:
///
/// - selects the chip-specific configuration from the `compatible` string
/// - maps every MMIO register range described by the FDT node
/// - powers on the NPU-related domains
/// - creates the top-level [`Rknpu`] driver object
/// - wires per-core IRQ handlers into the platform interrupt framework
/// - enables interrupt-driven waiting on AArch64
/// - publishes the device through the platform driver registry
pub fn rknpu_probe(info: FdtInfo<'_>, plat_dev: PlatformDevice) -> Result<(), OnProbeError> {
    let mut config = None;
    for c in info.node.compatibles() {
        if c == "rockchip,rk3588-rknpu" {
            config = Some(RknpuConfig {
                rknpu_type: RknpuType::Rk3588,
            });
            break;
        }
    }

    let config = config.expect("Unsupported RKNPU compatible");
    let regs = info.node.reg().unwrap();

    let mut base_regs = Vec::new();
    let page_size = 0x1000;
    for reg in regs {
        let start_raw = reg.address as usize;
        let end = start_raw + reg.size.unwrap_or(0x1000);

        let start = start_raw & !(page_size - 1);
        let offset = start_raw - start;
        let end = (end + page_size - 1) & !(page_size - 1);
        let size = end - start;

        base_regs.push(unsafe { iomap(start as _, size)?.add(offset) });
    }

    enable_pm();

    info!("NPU power enabled");

    #[allow(unused_mut)] // mut needed for set_wait_fn on aarch64
    let mut npu = Rknpu::new(&base_regs, config);

    // Register one IRQ callback per visible NPU core.
    //
    // Flow:
    //  1. Parse each core IRQ from the FDT node.
    //  2. Build a lightweight per-core handler from `Rknpu`.
    //  3. Store it in a global slot because the platform only accepts `fn()`.
    //  4. Register the trampoline with the IRQ framework.
    //  5. Enable interrupt-driven waiting afterwards.
    //
    // Completion path after a task finishes:
    //  NPU raises IRQ -> GIC routes it -> handle_npu_irq_coreN() ->
    //  RknpuIrqHandler::handle() -> read/clear hardware state ->
    //  publish irq_status -> waiting CPU wakes up -> submit path observes
    //  completion and continues.
    let interrupts = info.interrupts();
    for (i, irq_cells) in interrupts.iter().enumerate() {
        if i >= 3 {
            break;
        }
        let gic_irq = fdt_irq_to_gic_num(irq_cells);

        // Extract the handler and place it in the global slot used by the
        // per-core `fn()` trampoline.
        let handler = npu.new_irq_handler(i);
        unsafe { *NPU_IRQ_HANDLERS[i].0.get() = Some(handler) };

        // Register the IRQ line with the platform framework. Registration also
        // enables the line on the platform side.
        axklib::irq::register(gic_irq, NPU_IRQ_FNS[i]);
        warn!("[NPU] Core {} IRQ registered: GIC #{}", i, gic_irq);
    }

    // Switch the legacy busy-wait submit path to interrupt-assisted waiting on
    // supported architectures.
    #[cfg(target_arch = "aarch64")]
    npu.set_wait_fn(irq_yield);

    plat_dev.register(npu);
    warn!("NPU registered successfully");
    Ok(())
}


// Register the Rockchip RKNPU platform probe hook.
module_driver!(
    name: "Rockchip NPU",
    level: ProbeLevel::PostKernel,
    priority: ProbePriority::DEFAULT,
    probe_kinds: &[
        ProbeKind::Fdt {
            compatibles: &["rockchip,rk3588-rknpu"],
            on_probe: rknpu_probe
        }
    ],
);
