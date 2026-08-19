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
use crate::card1::init_rknpu_service;
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

    // 最后修改日期：2026-08-17。必须在启用 IRQ 前初始化 Event 唤醒链路，
    // 防止第一次硬件中断在 IRQ 上下文触发调度服务的懒初始化和内存分配。
    init_rknpu_service();

    // 每个可见 NPU 核心注册一个 IRQ 回调。完成路径为：NPU 产生中断，
    // RknpuIrqHandler 读取、清除并发布 irq_status，Event 唤醒调度 worker，
    // worker 在普通任务上下文回收完成状态并继续派发 Ready Task。
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
