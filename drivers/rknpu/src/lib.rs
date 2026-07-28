

#![no_std]

extern crate alloc;
#[cfg(test)]
extern crate std;
#[macro_use]
extern crate log;
use core::{
    ops::{Deref, DerefMut},
    ptr::NonNull,
    sync::atomic::Ordering,
};

mod config;
mod data;
mod err;
mod gem;
mod job;
mod osal;
mod registers;
pub mod service;
pub mod status;
mod task;
use alloc::vec::Vec;
pub use config::*;
pub use err::*;
pub use gem::*;
pub use job::*;
pub use osal::*;
use rdif_base::DriverGeneric;
pub use status::*;
pub use task::*;
pub mod ioctrl;
use crate::data::RknpuData;
#[cfg(feature = "starryos")]
pub use crate::power::*;
use crate::registers::RknpuCore;

const VERSION_MAJOR: u32 = 0;
const VERSION_MINOR: u32 = 9;
const VERSION_PATCH: u32 = 8;

const fn version(major: u32, minor: u32, patch: u32) -> u32 {
    major * 10000 + minor * 100 + patch
}

/// Hardware actions that may be requested through [`Rknpu::action()`].
///
/// These mirror the Linux-style driver action codes. Most of them are
/// management or diagnostic operations rather than compute submissions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum RknpuAction {
    GetHwVersion = 0,
    GetDrvVersion = 1,
    GetFreq = 2,
    SetFreq = 3,
    GetVolt = 4,
    SetVolt = 5,
    ActReset = 6,
    GetBwPriority = 7,
    SetBwPriority = 8,
    GetBwExpect = 9,
    SetBwExpect = 10,
    GetBwTw = 11,
    SetBwTw = 12,
    ActClrTotalRwAmount = 13,
    GetDtWrAmount = 14,
    GetDtRdAmount = 15,
    GetWtRdAmount = 16,
    GetTotalRwAmount = 17,
    GetIommuEn = 18,
    SetProcNice = 19,
    PowerOn = 20,
    PowerOff = 21,
    GetTotalSramSize = 22,
    GetFreeSramSize = 23,
    GetIommuDomainId = 24,
    SetIommuDomainId = 25,
}

impl core::convert::TryFrom<u32> for RknpuAction {
    type Error = ();

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::GetHwVersion),
            1 => Ok(Self::GetDrvVersion),
            2 => Ok(Self::GetFreq),
            3 => Ok(Self::SetFreq),
            4 => Ok(Self::GetVolt),
            5 => Ok(Self::SetVolt),
            6 => Ok(Self::ActReset),
            7 => Ok(Self::GetBwPriority),
            8 => Ok(Self::SetBwPriority),
            9 => Ok(Self::GetBwExpect),
            10 => Ok(Self::SetBwExpect),
            11 => Ok(Self::GetBwTw),
            12 => Ok(Self::SetBwTw),
            13 => Ok(Self::ActClrTotalRwAmount),
            14 => Ok(Self::GetDtWrAmount),
            15 => Ok(Self::GetDtRdAmount),
            16 => Ok(Self::GetWtRdAmount),
            17 => Ok(Self::GetTotalRwAmount),
            18 => Ok(Self::GetIommuEn),
            19 => Ok(Self::SetProcNice),
            20 => Ok(Self::PowerOn),
            21 => Ok(Self::PowerOff),
            22 => Ok(Self::GetTotalSramSize),
            23 => Ok(Self::GetFreeSramSize),
            24 => Ok(Self::GetIommuDomainId),
            25 => Ok(Self::SetIommuDomainId),
            _ => Err(()),
        }
    }
}

/// Top-level driver handle for one RKNPU device.
///
/// It owns the MMIO windows for all visible cores, the chip-specific capability
/// data, and the GEM pool shared with userspace. Submission, action, and memory
/// management all flow through this type.
///
/// # Completion waiting
///
/// By default the driver remains backward-compatible and busy-polls
/// `INTERRUPT_STATUS` after a submit. Calling [`set_wait_fn`] switches the
/// driver to an interrupt-assisted wait path:
///
/// ```text
///  default (no wait_fn): submit -> loop { read MMIO } -> done
///  with wait_fn:         submit -> loop { check atomic; sleep } -> IRQ
///                        arrives -> handler stores status -> CPU wakes -> done
/// ```
pub struct Rknpu {
    /// Register-access wrapper per visible core.
    base: Vec<RknpuCore>,
    /// Static board or SoC configuration.
    #[allow(dead_code)]
    config: RknpuConfig,
    /// Variant-specific data such as DMA limits and register quirks.
    data: RknpuData,
    /// Whether an IOMMU sits between the NPU and system memory.
    iommu_enabled: bool,
    /// Current logical IOMMU domain id exposed through the action ioctl.
    ///
    /// StarryOS does not yet wire this to a real domain-switching backend, but
    /// keeping the state here preserves the userspace-visible contract and
    /// mirrors the Linux driver shape.
    iommu_domain_id: i32,
    /// DMA buffer pool shared with userspace (see [`gem`] module).
    pub(crate) gem: GemPool,
    /// Optional platform-provided wait function used for interrupt-driven mode.
    ///
    /// `None` keeps the legacy polling path. `Some(f)` lets the driver sleep
    /// between atomic checks and resume when an IRQ wakes the CPU.
    wait_fn: Option<fn()>,
}

impl Rknpu {
    /// Create a new driver instance from raw MMIO base addresses.
    ///
    /// # Safety
    ///
    /// The caller must ensure that every pointer in `base_addrs` points to a
    /// valid mapped RKNPU register file and remains valid for the lifetime of
    /// the returned object.
    pub fn new(base_addrs: &[NonNull<u8>], config: RknpuConfig) -> Self {
        let data = RknpuData::new(config.rknpu_type);

        Self {
            base: base_addrs
                .iter()
                .enumerate()
                .map(|(core_slot, &addr)| unsafe { RknpuCore::new(addr, core_slot as u8) })
                .collect(),
            data,
            config,
            iommu_enabled: false,
            iommu_domain_id: 0,
            gem: GemPool::new(),
            wait_fn: None,
        }
    }

    pub fn open(&mut self) -> Result<(), RknpuError> {
        Ok(())
    }

    /// Enable interrupt-driven waiting by installing a platform sleep function.
    ///
    /// After this call the submit path stops busy-polling MMIO directly and
    /// instead checks the shared `irq_status` atomic between calls to `f()`.
    ///
    /// Typical AArch64 usage:
    ///
    /// ```ignore
    /// // After registering the NPU IRQ handler:
    /// npu.set_wait_fn(axcpu::wait_for_irqs);
    /// ```
    ///
    /// Safety notes:
    ///
    /// The provided function must return when any interrupt arrives, including
    /// the NPU IRQ. The platform must also register the NPU IRQ before relying
    /// on this mode.
    pub fn set_wait_fn(&mut self, f: fn()) {
        warn!("[NPU] Interrupt-driven mode enabled (WFI)");
        self.wait_fn = Some(f);
    }

    #[allow(dead_code)]
    fn dma_bit_mask(&self) -> u64 {
        self.data.dma_mask
    }

    pub fn get_hw_version(&self) -> u32 {
        self.base[0].version()
    }

    fn read_core_u32(&self, core_slot: usize, offset: u16) -> u32 {
        unsafe {
            self.base[core_slot]
                .offset_ptr::<u32>(offset as usize)
                .as_ptr()
                .read_volatile()
        }
    }

    pub fn clear_rw_amount(&mut self) -> Result<(), RknpuError> {
        let Some(amount_top) = self.data.amount_top else {
            warn!("RKNPU does not support read/write amount statistics");
            return Ok(());
        };

        if self.data.pc_dma_ctrl > 0 {
            let pc_data_addr = self.base[0].pc().base_address().read().bits();
            unsafe {
                self.base[0]
                    .offset_ptr::<u32>(pc_data_addr as usize)
                    .write_volatile(1);
                self.base[0]
                    .offset_ptr::<u32>(amount_top.offset_clr_all as usize)
                    .write_volatile(0x80000101);
                self.base[0]
                    .offset_ptr::<u32>(amount_top.offset_clr_all as usize)
                    .write_volatile(0x00000101);
                if let Some(amount_core) = self.data.amount_core {
                    self.base[0]
                        .offset_ptr::<u32>(amount_core.offset_clr_all as usize)
                        .write_volatile(0x80000101);
                    self.base[0]
                        .offset_ptr::<u32>(amount_core.offset_clr_all as usize)
                        .write_volatile(0x00000101);
                }
            };
        } else {
            unsafe {
                self.base[0]
                    .offset_ptr::<u32>(amount_top.offset_clr_all as usize)
                    .write_volatile(0x80000101);
                self.base[0]
                    .offset_ptr::<u32>(amount_top.offset_clr_all as usize)
                    .write_volatile(0x00000101);
                if let Some(amount_core) = self.data.amount_core {
                    self.base[0]
                        .offset_ptr::<u32>(amount_core.offset_clr_all as usize)
                        .write_volatile(0x80000101);
                    self.base[0]
                        .offset_ptr::<u32>(amount_core.offset_clr_all as usize)
                        .write_volatile(0x00000101);
                }
            }
        }

        Ok(())
    }

    pub fn get_rw_amount(
        &self,
        dt_wr: Option<&mut u32>,
        dt_rd: Option<&mut u32>,
        wt_rd: Option<&mut u32>,
    ) -> Result<(), RknpuError> {
        let Some(amount_top) = self.data.amount_top else {
            warn!("Get rw_amount is not supported on this device!");
            return Ok(());
        };

        let amount_scale = self.data.pc_data_amount_scale;
        let amount_core = self.data.amount_core;

        if let Some(dt_wr) = dt_wr {
            *dt_wr = self
                .read_core_u32(0, amount_top.offset_dt_wr)
                .saturating_mul(amount_scale);
            if let Some(amount_core) = amount_core {
                *dt_wr = dt_wr.saturating_add(
                    self.read_core_u32(0, amount_core.offset_dt_wr)
                        .saturating_mul(amount_scale),
                );
            }
        }

        if let Some(dt_rd) = dt_rd {
            *dt_rd = self
                .read_core_u32(0, amount_top.offset_dt_rd)
                .saturating_mul(amount_scale);
            if let Some(amount_core) = amount_core {
                *dt_rd = dt_rd.saturating_add(
                    self.read_core_u32(0, amount_core.offset_dt_rd)
                        .saturating_mul(amount_scale),
                );
            }
        }

        if let Some(wt_rd) = wt_rd {
            *wt_rd = self
                .read_core_u32(0, amount_top.offset_wt_rd)
                .saturating_mul(amount_scale);
            if let Some(amount_core) = amount_core {
                *wt_rd = wt_rd.saturating_add(
                    self.read_core_u32(0, amount_core.offset_wt_rd)
                        .saturating_mul(amount_scale),
                );
            }
        }

        Ok(())
    }

    pub fn get_total_rw_amount(&self) -> Result<u32, RknpuError> {
        let Some(_amount_top) = self.data.amount_top else {
            warn!("Get total_rw_amount is not supported on this device!");
            return Ok(0);
        };

        let mut dt_wr = 0;
        let mut dt_rd = 0;
        let mut wt_rd = 0;
        self.get_rw_amount(Some(&mut dt_wr), Some(&mut dt_rd), Some(&mut wt_rd))?;
        Ok(dt_wr.saturating_add(dt_rd).saturating_add(wt_rd))
    }

    pub fn soft_reset(&mut self) -> Result<(), RknpuError> {
        // Linux performs a full reset-controller dance here. StarryOS does not
        // yet have that platform plumbing in this crate, so the best-effort
        // reset path clears every core's pending IRQ state and published
        // completion shadow.
        for core in &self.base {
            core.clean_interrupts();
            core.drain_pending_interrupts();
            core.irq_status.store(0, Ordering::Release);
        }
        Ok(())
    }

    /// Execute an `RknpuAction` request.
    ///
    /// This mirrors the Linux driver's `rknpu_action` path and provides a Rust
    /// interface for hardware management operations.
    pub fn action(&mut self, action: RknpuAction, value: u32) -> Result<u32, RknpuError> {
        match action {
            RknpuAction::GetHwVersion => Ok(self.get_hw_version()),
            RknpuAction::GetDrvVersion => Ok(version(VERSION_MAJOR, VERSION_MINOR, VERSION_PATCH)),
            RknpuAction::GetFreq => {
                warn!("GetFreq requires platform clock integration");
                Err(RknpuError::NotSupported)
            }
            RknpuAction::SetFreq => {
                warn!("SetFreq requires platform clock integration");
                let _ = value;
                Err(RknpuError::NotSupported)
            }
            RknpuAction::GetVolt => {
                warn!("GetVolt requires platform regulator integration");
                Err(RknpuError::NotSupported)
            }
            RknpuAction::SetVolt => {
                warn!("SetVolt requires platform regulator integration");
                let _ = value;
                Err(RknpuError::NotSupported)
            }
            RknpuAction::ActReset => {
                self.soft_reset()?;
                Ok(0)
            }
            RknpuAction::GetBwPriority => {
                warn!("GetBwPriority requires a mapped bw_priority register window");
                Err(RknpuError::NotSupported)
            }
            RknpuAction::SetBwPriority => {
                warn!("SetBwPriority requires a mapped bw_priority register window");
                let _ = value;
                Err(RknpuError::NotSupported)
            }
            RknpuAction::GetBwExpect => {
                warn!("GetBwExpect requires a mapped bw_priority register window");
                Err(RknpuError::NotSupported)
            }
            RknpuAction::SetBwExpect => {
                warn!("SetBwExpect requires a mapped bw_priority register window");
                let _ = value;
                Err(RknpuError::NotSupported)
            }
            RknpuAction::GetBwTw => {
                warn!("GetBwTw requires a mapped bw_priority register window");
                Err(RknpuError::NotSupported)
            }
            RknpuAction::SetBwTw => {
                warn!("SetBwTw requires a mapped bw_priority register window");
                let _ = value;
                Err(RknpuError::NotSupported)
            }
            RknpuAction::ActClrTotalRwAmount => {
                self.clear_rw_amount()?;
                Ok(0)
            }
            RknpuAction::GetDtWrAmount => {
                let mut out = 0;
                self.get_rw_amount(Some(&mut out), None, None)?;
                Ok(out)
            }
            RknpuAction::GetDtRdAmount => {
                let mut out = 0;
                self.get_rw_amount(None, Some(&mut out), None)?;
                Ok(out)
            }
            RknpuAction::GetWtRdAmount => {
                let mut out = 0;
                self.get_rw_amount(None, None, Some(&mut out))?;
                Ok(out)
            }
            RknpuAction::GetTotalRwAmount => self.get_total_rw_amount(),
            RknpuAction::GetIommuEn => Ok(if self.iommu_enabled { 1 } else { 0 }),
            RknpuAction::SetProcNice => {
                let nice = i32::from_ne_bytes(value.to_ne_bytes());
                warn!(
                    "SetProcNice({}) is not applicable in the bare-metal driver context",
                    nice
                );
                Ok(0)
            }
            RknpuAction::PowerOn => {
                warn!("PowerOn requires platform PM integration");
                Err(RknpuError::NotSupported)
            }
            RknpuAction::PowerOff => {
                warn!("PowerOff requires platform PM integration");
                Err(RknpuError::NotSupported)
            }
            RknpuAction::GetTotalSramSize => Ok(self.data.nbuf_size as u32),
            RknpuAction::GetFreeSramSize => Ok(self.data.nbuf_size as u32),
            RknpuAction::GetIommuDomainId => Ok(self.iommu_domain_id as u32),
            RknpuAction::SetIommuDomainId => {
                let domain_id = i32::from_ne_bytes(value.to_ne_bytes());
                if !(0..16).contains(&domain_id) {
                    return Err(RknpuError::InvalidParameter);
                }
                self.iommu_domain_id = domain_id;
                Ok(0)
            }
        }
    }

    /// Convenience method to check IOMMU status using action interface
    pub fn is_iommu_enabled(&self) -> bool {
        self.iommu_enabled
    }

    /// Enable or disable IOMMU
    pub fn set_iommu_enabled(&mut self, enabled: bool) {
        self.iommu_enabled = enabled;
    }

    /// Submit one prebuilt task batch to core 0 through the high-level API.
    ///
    /// Unlike the ioctl path, which steps one task at a time, this accepts a
    /// [`Submit`] object built by the `task/` module.
    pub fn submit(&mut self, job: &mut Submit) -> Result<(), RknpuError> {
        self.base[0].submit(&self.data, self.wait_fn, job)
    }

    /// Harvests every per-core completion currently published by IRQ handlers.
    ///
    /// The scheduler calls this from normal task context. Each returned record
    /// clears the corresponding active per-core dispatch slot.
    pub fn harvest_completed_dispatches(&mut self) -> Vec<CoreCompletion> {
        let mut completed = Vec::new();

        for core_slot in 0..self.base.len().min(NPU_MAX_CORES) {
            let irq_status = self.base[core_slot]
                .irq_status
                .swap(0, Ordering::AcqRel);
            if irq_status == 0 {
                continue;
            }
            debug!(
                "[NPU] harvest_completed_dispatches core{} irq_status={:#x} observed={:#x}",
                core_slot, irq_status, irq_status
            );

            completed.push(CoreCompletion {
                core_slot: core_slot as u8,
                observed_irq_status: irq_status,
            });
        }

        completed
    }

    /// Read and clear pending interrupt state on core 0.
    ///
    /// Returns the fuzzed status mask. Any non-zero value means at least one
    /// task completed or faulted since the previous read.
    pub fn handle_interrupt0(&mut self) -> u32 {
        self.base[0].handle_interrupt()
    }

    /// Create a lightweight `Send + Sync` IRQ handler for one core.
    ///
    /// This is useful when the platform IRQ framework needs a callable object
    /// that can run in interrupt context without borrowing `&mut Rknpu`.
    pub fn new_irq_handler(&self, core_idx: usize) -> RknpuIrqHandler {
        RknpuIrqHandler(self.base[core_idx].clone())
    }
}

/// Lightweight interrupt handler for one NPU core.
///
/// Cloned from [`Rknpu`] and safe to call from IRQ context. It only reads and
/// clears interrupt state; it does not allocate memory or block.
pub struct RknpuIrqHandler(RknpuCore);

unsafe impl Send for RknpuIrqHandler {}
unsafe impl Sync for RknpuIrqHandler {}

impl RknpuIrqHandler {
    /// Read and clear pending interrupts, returning the fuzzed status.
    pub fn handle(&self) -> u32 {
        self.0.handle_interrupt()
    }
}

impl DriverGeneric for Rknpu {
    fn open(&mut self) -> Result<(), rdif_base::KError> {
        Self::open(self).map_err(|_| rdif_base::KError::Unknown("open fail"))
    }

    fn close(&mut self) -> Result<(), rdif_base::KError> {
        Ok(())
    }
}

impl Deref for Rknpu {
    type Target = GemPool;

    fn deref(&self) -> &Self::Target {
        &self.gem
    }
}

impl DerefMut for Rknpu {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.gem
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build_test_npu(core_count: usize) -> Rknpu {
        let mut mmios = (0..core_count)
            .map(|_| vec![0_u8; 0x10000])
            .collect::<Vec<_>>();
        let base_addrs = mmios
            .iter_mut()
            .map(|mmio| NonNull::new(mmio.as_mut_ptr()).unwrap())
            .collect::<Vec<_>>();
        let config = RknpuConfig {
            rknpu_type: RknpuType::Rk3588,
        };
        Rknpu::new(&base_addrs, config)
    }

    #[test]
    fn harvest_completed_dispatches_returns_raw_core_status() {
        let mut npu = build_test_npu(1);
        npu.base[0].irq_status.store(0x300, Ordering::Release);

        let completed = npu.harvest_completed_dispatches();

        assert_eq!(completed.len(), 1);
        assert_eq!(completed[0].core_slot, 0);
        assert_eq!(completed[0].observed_irq_status, 0x300);
        assert_eq!(npu.base[0].irq_status.load(Ordering::Acquire), 0);
    }

    #[test]
    fn harvest_completed_dispatches_collects_multiple_cores() {
        let mut npu = build_test_npu(2);

        npu.base[0].irq_status.store(0x100, Ordering::Release);
        npu.base[1].irq_status.store(0x300, Ordering::Release);

        let completed = npu.harvest_completed_dispatches();

        assert_eq!(completed.len(), 2);
        assert_eq!(completed[0].core_slot, 0);
        assert_eq!(completed[0].observed_irq_status, 0x100);
        assert_eq!(completed[1].core_slot, 1);
        assert_eq!(completed[1].observed_irq_status, 0x300);
        assert_eq!(npu.base[0].irq_status.load(Ordering::Acquire), 0);
        assert_eq!(npu.base[1].irq_status.load(Ordering::Acquire), 0);
    }

    #[test]
    fn action_iommu_domain_round_trip_uses_input_value() {
        let mut npu = build_test_npu(1);

        assert_eq!(npu.action(RknpuAction::GetIommuDomainId, 0).unwrap(), 0);
        assert_eq!(npu.action(RknpuAction::SetIommuDomainId, 7).unwrap(), 0);
        assert_eq!(npu.action(RknpuAction::GetIommuDomainId, 0).unwrap(), 7);
    }

    #[test]
    fn action_rejects_out_of_range_iommu_domain() {
        let mut npu = build_test_npu(1);

        let err = npu.action(RknpuAction::SetIommuDomainId, 16).unwrap_err();

        assert_eq!(err, RknpuError::InvalidParameter);
    }

    #[test]
    fn action_reports_total_sram_from_variant_data() {
        let mut npu = build_test_npu(1);
        npu.data.nbuf_size = 0x4000;

        assert_eq!(
            npu.action(RknpuAction::GetTotalSramSize, 0).unwrap(),
            0x4000
        );
        assert_eq!(npu.action(RknpuAction::GetFreeSramSize, 0).unwrap(), 0x4000);
    }

    #[test]
    fn soft_reset_clears_published_irq_state() {
        let mut npu = build_test_npu(2);
        npu.base[0].irq_status.store(0x100, Ordering::Release);
        npu.base[1].irq_status.store(0x300, Ordering::Release);

        npu.soft_reset().unwrap();

        assert_eq!(npu.base[0].irq_status.load(Ordering::Acquire), 0);
        assert_eq!(npu.base[1].irq_status.load(Ordering::Acquire), 0);
    }
}
