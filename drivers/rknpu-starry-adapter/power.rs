use rockchip_pm::PD;
use rockchip_pm::RockchipPM;





#[cfg(target_arch = "aarch64")]
/// Yield while waiting for an NPU IRQ on AArch64.
///
/// The historical implementation used `wfi` directly here. The current version
/// yields to the scheduler instead so the system can continue making progress
/// while the submit path waits for interrupt completion.
pub(crate) fn irq_yield() {
    //unsafe { core::arch::asm!("wfi") };
    //axklib::time::busy_wait(core::time::Duration::from_micros(10));
    axtask::yield_now();
}

/// Power on the RK3588 NPU-related power domains required by the driver.
///
/// The Rockchip PM driver exposes per-domain control through [`RockchipPM`].
/// This helper turns on the top-level NPU domain and the individual subdomains
/// before MMIO access or IRQ setup begins.
pub fn enable_pm() {
    // RK3588 NPU-related power-domain identifiers.

    /// Main NPU power domain.
    pub const NPU: PD = PD(8);
    /// Top-level NPU power domain.
    pub const NPUTOP: PD = PD(9);
    /// NPU1 power domain.
    pub const NPU1: PD = PD(10);
    /// NPU2 power domain.
    pub const NPU2: PD = PD(11);

    let mut pm = rdrive::get_one::<RockchipPM>().unwrap().lock().unwrap();

    // Power domains are brought up explicitly so later register accesses and
    // submissions do not touch a gated NPU block.
    pm.power_domain_on(NPUTOP).unwrap();
    pm.power_domain_on(NPU).unwrap();
    pm.power_domain_on(NPU1).unwrap();
    pm.power_domain_on(NPU2).unwrap();
}




