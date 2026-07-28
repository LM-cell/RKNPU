#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    s_status: SStatus,
    s_pointer: SPointer,
    operation_enable: OperationEnable,
    mac_gating: MacGating,
    misc_cfg: MiscCfg,
    dataout_size_0: DataoutSize0,
    dataout_size_1: DataoutSize1,
    clip_truncate: ClipTruncate,
}
impl RegisterBlock {
    #[doc = "0x00 - s_status"]
    #[inline(always)]
    pub const fn s_status(&self) -> &SStatus {
        &self.s_status
    }
    #[doc = "0x04 - s_pointer"]
    #[inline(always)]
    pub const fn s_pointer(&self) -> &SPointer {
        &self.s_pointer
    }
    #[doc = "0x08 - operation_enable"]
    #[inline(always)]
    pub const fn operation_enable(&self) -> &OperationEnable {
        &self.operation_enable
    }
    #[doc = "0x0c - mac_gating"]
    #[inline(always)]
    pub const fn mac_gating(&self) -> &MacGating {
        &self.mac_gating
    }
    #[doc = "0x10 - misc_cfg"]
    #[inline(always)]
    pub const fn misc_cfg(&self) -> &MiscCfg {
        &self.misc_cfg
    }
    #[doc = "0x14 - dataout_size_0"]
    #[inline(always)]
    pub const fn dataout_size_0(&self) -> &DataoutSize0 {
        &self.dataout_size_0
    }
    #[doc = "0x18 - dataout_size_1"]
    #[inline(always)]
    pub const fn dataout_size_1(&self) -> &DataoutSize1 {
        &self.dataout_size_1
    }
    #[doc = "0x1c - clip_truncate"]
    #[inline(always)]
    pub const fn clip_truncate(&self) -> &ClipTruncate {
        &self.clip_truncate
    }
}
#[doc = "S_STATUS (r) register accessor: s_status\n\nYou can [`read`](crate::Reg::read) this register and get [`s_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s_status`] module"]
#[doc(alias = "S_STATUS")]
pub type SStatus = crate::Reg<s_status::SStatusSpec>;
#[doc = "s_status"]
pub mod s_status;
#[doc = "S_POINTER (rw) register accessor: s_pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`s_pointer::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`s_pointer::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s_pointer`] module"]
#[doc(alias = "S_POINTER")]
pub type SPointer = crate::Reg<s_pointer::SPointerSpec>;
#[doc = "s_pointer"]
pub mod s_pointer;
#[doc = "OPERATION_ENABLE (rw) register accessor: operation_enable\n\nYou can [`read`](crate::Reg::read) this register and get [`operation_enable::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`operation_enable::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@operation_enable`] module"]
#[doc(alias = "OPERATION_ENABLE")]
pub type OperationEnable = crate::Reg<operation_enable::OperationEnableSpec>;
#[doc = "operation_enable"]
pub mod operation_enable;
#[doc = "MAC_GATING (rw) register accessor: mac_gating\n\nYou can [`read`](crate::Reg::read) this register and get [`mac_gating::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mac_gating::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mac_gating`] module"]
#[doc(alias = "MAC_GATING")]
pub type MacGating = crate::Reg<mac_gating::MacGatingSpec>;
#[doc = "mac_gating"]
pub mod mac_gating;
#[doc = "MISC_CFG (rw) register accessor: misc_cfg\n\nYou can [`read`](crate::Reg::read) this register and get [`misc_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`misc_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@misc_cfg`] module"]
#[doc(alias = "MISC_CFG")]
pub type MiscCfg = crate::Reg<misc_cfg::MiscCfgSpec>;
#[doc = "misc_cfg"]
pub mod misc_cfg;
#[doc = "DATAOUT_SIZE_0 (rw) register accessor: dataout_size_0\n\nYou can [`read`](crate::Reg::read) this register and get [`dataout_size_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dataout_size_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dataout_size_0`] module"]
#[doc(alias = "DATAOUT_SIZE_0")]
pub type DataoutSize0 = crate::Reg<dataout_size_0::DataoutSize0Spec>;
#[doc = "dataout_size_0"]
pub mod dataout_size_0;
#[doc = "DATAOUT_SIZE_1 (rw) register accessor: dataout_size_1\n\nYou can [`read`](crate::Reg::read) this register and get [`dataout_size_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dataout_size_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dataout_size_1`] module"]
#[doc(alias = "DATAOUT_SIZE_1")]
pub type DataoutSize1 = crate::Reg<dataout_size_1::DataoutSize1Spec>;
#[doc = "dataout_size_1"]
pub mod dataout_size_1;
#[doc = "CLIP_TRUNCATE (rw) register accessor: clip_truncate\n\nYou can [`read`](crate::Reg::read) this register and get [`clip_truncate::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clip_truncate::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clip_truncate`] module"]
#[doc(alias = "CLIP_TRUNCATE")]
pub type ClipTruncate = crate::Reg<clip_truncate::ClipTruncateSpec>;
#[doc = "clip_truncate"]
pub mod clip_truncate;
