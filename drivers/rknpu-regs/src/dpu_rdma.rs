#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    s_status: SStatus,
    s_pointer: SPointer,
    operation_enable: OperationEnable,
    data_cube_width: DataCubeWidth,
    data_cube_height: DataCubeHeight,
    data_cube_channel: DataCubeChannel,
    src_base_addr: SrcBaseAddr,
    brdma_cfg: BrdmaCfg,
    bs_base_addr: BsBaseAddr,
    _reserved9: [u8; 0x04],
    nrdma_cfg: NrdmaCfg,
    bn_base_addr: BnBaseAddr,
    _reserved11: [u8; 0x04],
    erdma_cfg: ErdmaCfg,
    ew_base_addr: EwBaseAddr,
    _reserved13: [u8; 0x04],
    ew_surf_stride: EwSurfStride,
    feature_mode_cfg: FeatureModeCfg,
    src_dma_cfg: SrcDmaCfg,
    surf_notch: SurfNotch,
    _reserved17: [u8; 0x14],
    pad_cfg: PadCfg,
    weight: Weight,
    ew_surf_notch: EwSurfNotch,
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
    #[doc = "0x0c - data_cube_width"]
    #[inline(always)]
    pub const fn data_cube_width(&self) -> &DataCubeWidth {
        &self.data_cube_width
    }
    #[doc = "0x10 - data_cube_height"]
    #[inline(always)]
    pub const fn data_cube_height(&self) -> &DataCubeHeight {
        &self.data_cube_height
    }
    #[doc = "0x14 - data_cube_channel"]
    #[inline(always)]
    pub const fn data_cube_channel(&self) -> &DataCubeChannel {
        &self.data_cube_channel
    }
    #[doc = "0x18 - src_base_addr"]
    #[inline(always)]
    pub const fn src_base_addr(&self) -> &SrcBaseAddr {
        &self.src_base_addr
    }
    #[doc = "0x1c - brdma_cfg"]
    #[inline(always)]
    pub const fn brdma_cfg(&self) -> &BrdmaCfg {
        &self.brdma_cfg
    }
    #[doc = "0x20 - bs_base_addr"]
    #[inline(always)]
    pub const fn bs_base_addr(&self) -> &BsBaseAddr {
        &self.bs_base_addr
    }
    #[doc = "0x28 - nrdma_cfg"]
    #[inline(always)]
    pub const fn nrdma_cfg(&self) -> &NrdmaCfg {
        &self.nrdma_cfg
    }
    #[doc = "0x2c - bn_base_addr"]
    #[inline(always)]
    pub const fn bn_base_addr(&self) -> &BnBaseAddr {
        &self.bn_base_addr
    }
    #[doc = "0x34 - erdma_cfg"]
    #[inline(always)]
    pub const fn erdma_cfg(&self) -> &ErdmaCfg {
        &self.erdma_cfg
    }
    #[doc = "0x38 - ew_base_addr"]
    #[inline(always)]
    pub const fn ew_base_addr(&self) -> &EwBaseAddr {
        &self.ew_base_addr
    }
    #[doc = "0x40 - ew_surf_stride"]
    #[inline(always)]
    pub const fn ew_surf_stride(&self) -> &EwSurfStride {
        &self.ew_surf_stride
    }
    #[doc = "0x44 - feature_mode_cfg"]
    #[inline(always)]
    pub const fn feature_mode_cfg(&self) -> &FeatureModeCfg {
        &self.feature_mode_cfg
    }
    #[doc = "0x48 - src_dma_cfg"]
    #[inline(always)]
    pub const fn src_dma_cfg(&self) -> &SrcDmaCfg {
        &self.src_dma_cfg
    }
    #[doc = "0x4c - surf_notch"]
    #[inline(always)]
    pub const fn surf_notch(&self) -> &SurfNotch {
        &self.surf_notch
    }
    #[doc = "0x64 - pad_cfg"]
    #[inline(always)]
    pub const fn pad_cfg(&self) -> &PadCfg {
        &self.pad_cfg
    }
    #[doc = "0x68 - weight"]
    #[inline(always)]
    pub const fn weight(&self) -> &Weight {
        &self.weight
    }
    #[doc = "0x6c - ew_surf_notch"]
    #[inline(always)]
    pub const fn ew_surf_notch(&self) -> &EwSurfNotch {
        &self.ew_surf_notch
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
#[doc = "DATA_CUBE_WIDTH (rw) register accessor: data_cube_width\n\nYou can [`read`](crate::Reg::read) this register and get [`data_cube_width::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data_cube_width::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data_cube_width`] module"]
#[doc(alias = "DATA_CUBE_WIDTH")]
pub type DataCubeWidth = crate::Reg<data_cube_width::DataCubeWidthSpec>;
#[doc = "data_cube_width"]
pub mod data_cube_width;
#[doc = "DATA_CUBE_HEIGHT (rw) register accessor: data_cube_height\n\nYou can [`read`](crate::Reg::read) this register and get [`data_cube_height::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data_cube_height::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data_cube_height`] module"]
#[doc(alias = "DATA_CUBE_HEIGHT")]
pub type DataCubeHeight = crate::Reg<data_cube_height::DataCubeHeightSpec>;
#[doc = "data_cube_height"]
pub mod data_cube_height;
#[doc = "DATA_CUBE_CHANNEL (rw) register accessor: data_cube_channel\n\nYou can [`read`](crate::Reg::read) this register and get [`data_cube_channel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data_cube_channel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data_cube_channel`] module"]
#[doc(alias = "DATA_CUBE_CHANNEL")]
pub type DataCubeChannel = crate::Reg<data_cube_channel::DataCubeChannelSpec>;
#[doc = "data_cube_channel"]
pub mod data_cube_channel;
#[doc = "SRC_BASE_ADDR (rw) register accessor: src_base_addr\n\nYou can [`read`](crate::Reg::read) this register and get [`src_base_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`src_base_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@src_base_addr`] module"]
#[doc(alias = "SRC_BASE_ADDR")]
pub type SrcBaseAddr = crate::Reg<src_base_addr::SrcBaseAddrSpec>;
#[doc = "src_base_addr"]
pub mod src_base_addr;
#[doc = "BRDMA_CFG (rw) register accessor: brdma_cfg\n\nYou can [`read`](crate::Reg::read) this register and get [`brdma_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`brdma_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@brdma_cfg`] module"]
#[doc(alias = "BRDMA_CFG")]
pub type BrdmaCfg = crate::Reg<brdma_cfg::BrdmaCfgSpec>;
#[doc = "brdma_cfg"]
pub mod brdma_cfg;
#[doc = "BS_BASE_ADDR (rw) register accessor: bs_base_addr\n\nYou can [`read`](crate::Reg::read) this register and get [`bs_base_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bs_base_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bs_base_addr`] module"]
#[doc(alias = "BS_BASE_ADDR")]
pub type BsBaseAddr = crate::Reg<bs_base_addr::BsBaseAddrSpec>;
#[doc = "bs_base_addr"]
pub mod bs_base_addr;
#[doc = "NRDMA_CFG (rw) register accessor: nrdma_cfg\n\nYou can [`read`](crate::Reg::read) this register and get [`nrdma_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nrdma_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nrdma_cfg`] module"]
#[doc(alias = "NRDMA_CFG")]
pub type NrdmaCfg = crate::Reg<nrdma_cfg::NrdmaCfgSpec>;
#[doc = "nrdma_cfg"]
pub mod nrdma_cfg;
#[doc = "BN_BASE_ADDR (rw) register accessor: bn_base_addr\n\nYou can [`read`](crate::Reg::read) this register and get [`bn_base_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bn_base_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bn_base_addr`] module"]
#[doc(alias = "BN_BASE_ADDR")]
pub type BnBaseAddr = crate::Reg<bn_base_addr::BnBaseAddrSpec>;
#[doc = "bn_base_addr"]
pub mod bn_base_addr;
#[doc = "ERDMA_CFG (rw) register accessor: erdma_cfg\n\nYou can [`read`](crate::Reg::read) this register and get [`erdma_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`erdma_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@erdma_cfg`] module"]
#[doc(alias = "ERDMA_CFG")]
pub type ErdmaCfg = crate::Reg<erdma_cfg::ErdmaCfgSpec>;
#[doc = "erdma_cfg"]
pub mod erdma_cfg;
#[doc = "EW_BASE_ADDR (rw) register accessor: ew_base_addr\n\nYou can [`read`](crate::Reg::read) this register and get [`ew_base_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ew_base_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ew_base_addr`] module"]
#[doc(alias = "EW_BASE_ADDR")]
pub type EwBaseAddr = crate::Reg<ew_base_addr::EwBaseAddrSpec>;
#[doc = "ew_base_addr"]
pub mod ew_base_addr;
#[doc = "EW_SURF_STRIDE (rw) register accessor: ew_surf_stride\n\nYou can [`read`](crate::Reg::read) this register and get [`ew_surf_stride::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ew_surf_stride::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ew_surf_stride`] module"]
#[doc(alias = "EW_SURF_STRIDE")]
pub type EwSurfStride = crate::Reg<ew_surf_stride::EwSurfStrideSpec>;
#[doc = "ew_surf_stride"]
pub mod ew_surf_stride;
#[doc = "FEATURE_MODE_CFG (rw) register accessor: feature_mode_cfg\n\nYou can [`read`](crate::Reg::read) this register and get [`feature_mode_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`feature_mode_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@feature_mode_cfg`] module"]
#[doc(alias = "FEATURE_MODE_CFG")]
pub type FeatureModeCfg = crate::Reg<feature_mode_cfg::FeatureModeCfgSpec>;
#[doc = "feature_mode_cfg"]
pub mod feature_mode_cfg;
#[doc = "SRC_DMA_CFG (rw) register accessor: src_dma_cfg\n\nYou can [`read`](crate::Reg::read) this register and get [`src_dma_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`src_dma_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@src_dma_cfg`] module"]
#[doc(alias = "SRC_DMA_CFG")]
pub type SrcDmaCfg = crate::Reg<src_dma_cfg::SrcDmaCfgSpec>;
#[doc = "src_dma_cfg"]
pub mod src_dma_cfg;
#[doc = "SURF_NOTCH (rw) register accessor: surf_notch\n\nYou can [`read`](crate::Reg::read) this register and get [`surf_notch::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`surf_notch::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@surf_notch`] module"]
#[doc(alias = "SURF_NOTCH")]
pub type SurfNotch = crate::Reg<surf_notch::SurfNotchSpec>;
#[doc = "surf_notch"]
pub mod surf_notch;
#[doc = "PAD_CFG (rw) register accessor: pad_cfg\n\nYou can [`read`](crate::Reg::read) this register and get [`pad_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pad_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pad_cfg`] module"]
#[doc(alias = "PAD_CFG")]
pub type PadCfg = crate::Reg<pad_cfg::PadCfgSpec>;
#[doc = "pad_cfg"]
pub mod pad_cfg;
#[doc = "WEIGHT (rw) register accessor: weight\n\nYou can [`read`](crate::Reg::read) this register and get [`weight::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`weight::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@weight`] module"]
#[doc(alias = "WEIGHT")]
pub type Weight = crate::Reg<weight::WeightSpec>;
#[doc = "weight"]
pub mod weight;
#[doc = "EW_SURF_NOTCH (rw) register accessor: ew_surf_notch\n\nYou can [`read`](crate::Reg::read) this register and get [`ew_surf_notch::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ew_surf_notch::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ew_surf_notch`] module"]
#[doc(alias = "EW_SURF_NOTCH")]
pub type EwSurfNotch = crate::Reg<ew_surf_notch::EwSurfNotchSpec>;
#[doc = "ew_surf_notch"]
pub mod ew_surf_notch;
