#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    s_status: SStatus,
    s_pointer: SPointer,
    operation_enable: OperationEnable,
    data_cube_in_width: DataCubeInWidth,
    data_cube_in_height: DataCubeInHeight,
    data_cube_in_channel: DataCubeInChannel,
    data_cube_out_width: DataCubeOutWidth,
    data_cube_out_height: DataCubeOutHeight,
    data_cube_out_channel: DataCubeOutChannel,
    operation_mode_cfg: OperationModeCfg,
    _reserved10: [u8; 0x0c],
    pooling_kernel_cfg: PoolingKernelCfg,
    recip_kernel_width: RecipKernelWidth,
    recip_kernel_height: RecipKernelHeight,
    pooling_padding_cfg: PoolingPaddingCfg,
    padding_value_1_cfg: PaddingValue1Cfg,
    padding_value_2_cfg: PaddingValue2Cfg,
    _reserved16: [u8; 0x24],
    dst_base_addr: DstBaseAddr,
    _reserved17: [u8; 0x08],
    dst_surf_stride: DstSurfStride,
    _reserved18: [u8; 0x04],
    data_format: DataFormat,
    _reserved19: [u8; 0x54],
    misc_ctrl: MiscCtrl,
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
    #[doc = "0x0c - data_cube_in_width"]
    #[inline(always)]
    pub const fn data_cube_in_width(&self) -> &DataCubeInWidth {
        &self.data_cube_in_width
    }
    #[doc = "0x10 - data_cube_in_height"]
    #[inline(always)]
    pub const fn data_cube_in_height(&self) -> &DataCubeInHeight {
        &self.data_cube_in_height
    }
    #[doc = "0x14 - data_cube_in_channel"]
    #[inline(always)]
    pub const fn data_cube_in_channel(&self) -> &DataCubeInChannel {
        &self.data_cube_in_channel
    }
    #[doc = "0x18 - data_cube_out_width"]
    #[inline(always)]
    pub const fn data_cube_out_width(&self) -> &DataCubeOutWidth {
        &self.data_cube_out_width
    }
    #[doc = "0x1c - data_cube_out_height"]
    #[inline(always)]
    pub const fn data_cube_out_height(&self) -> &DataCubeOutHeight {
        &self.data_cube_out_height
    }
    #[doc = "0x20 - data_cube_out_channel"]
    #[inline(always)]
    pub const fn data_cube_out_channel(&self) -> &DataCubeOutChannel {
        &self.data_cube_out_channel
    }
    #[doc = "0x24 - operation_mode_cfg"]
    #[inline(always)]
    pub const fn operation_mode_cfg(&self) -> &OperationModeCfg {
        &self.operation_mode_cfg
    }
    #[doc = "0x34 - pooling_kernel_cfg"]
    #[inline(always)]
    pub const fn pooling_kernel_cfg(&self) -> &PoolingKernelCfg {
        &self.pooling_kernel_cfg
    }
    #[doc = "0x38 - recip_kernel_width"]
    #[inline(always)]
    pub const fn recip_kernel_width(&self) -> &RecipKernelWidth {
        &self.recip_kernel_width
    }
    #[doc = "0x3c - recip_kernel_height"]
    #[inline(always)]
    pub const fn recip_kernel_height(&self) -> &RecipKernelHeight {
        &self.recip_kernel_height
    }
    #[doc = "0x40 - pooling_padding_cfg"]
    #[inline(always)]
    pub const fn pooling_padding_cfg(&self) -> &PoolingPaddingCfg {
        &self.pooling_padding_cfg
    }
    #[doc = "0x44 - padding_value_1_cfg"]
    #[inline(always)]
    pub const fn padding_value_1_cfg(&self) -> &PaddingValue1Cfg {
        &self.padding_value_1_cfg
    }
    #[doc = "0x48 - padding_value_2_cfg"]
    #[inline(always)]
    pub const fn padding_value_2_cfg(&self) -> &PaddingValue2Cfg {
        &self.padding_value_2_cfg
    }
    #[doc = "0x70 - dst_base_addr"]
    #[inline(always)]
    pub const fn dst_base_addr(&self) -> &DstBaseAddr {
        &self.dst_base_addr
    }
    #[doc = "0x7c - dst_surf_stride"]
    #[inline(always)]
    pub const fn dst_surf_stride(&self) -> &DstSurfStride {
        &self.dst_surf_stride
    }
    #[doc = "0x84 - data_format"]
    #[inline(always)]
    pub const fn data_format(&self) -> &DataFormat {
        &self.data_format
    }
    #[doc = "0xdc - misc_ctrl"]
    #[inline(always)]
    pub const fn misc_ctrl(&self) -> &MiscCtrl {
        &self.misc_ctrl
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
#[doc = "DATA_CUBE_IN_WIDTH (rw) register accessor: data_cube_in_width\n\nYou can [`read`](crate::Reg::read) this register and get [`data_cube_in_width::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data_cube_in_width::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data_cube_in_width`] module"]
#[doc(alias = "DATA_CUBE_IN_WIDTH")]
pub type DataCubeInWidth = crate::Reg<data_cube_in_width::DataCubeInWidthSpec>;
#[doc = "data_cube_in_width"]
pub mod data_cube_in_width;
#[doc = "DATA_CUBE_IN_HEIGHT (rw) register accessor: data_cube_in_height\n\nYou can [`read`](crate::Reg::read) this register and get [`data_cube_in_height::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data_cube_in_height::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data_cube_in_height`] module"]
#[doc(alias = "DATA_CUBE_IN_HEIGHT")]
pub type DataCubeInHeight = crate::Reg<data_cube_in_height::DataCubeInHeightSpec>;
#[doc = "data_cube_in_height"]
pub mod data_cube_in_height;
#[doc = "DATA_CUBE_IN_CHANNEL (rw) register accessor: data_cube_in_channel\n\nYou can [`read`](crate::Reg::read) this register and get [`data_cube_in_channel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data_cube_in_channel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data_cube_in_channel`] module"]
#[doc(alias = "DATA_CUBE_IN_CHANNEL")]
pub type DataCubeInChannel = crate::Reg<data_cube_in_channel::DataCubeInChannelSpec>;
#[doc = "data_cube_in_channel"]
pub mod data_cube_in_channel;
#[doc = "DATA_CUBE_OUT_WIDTH (rw) register accessor: data_cube_out_width\n\nYou can [`read`](crate::Reg::read) this register and get [`data_cube_out_width::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data_cube_out_width::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data_cube_out_width`] module"]
#[doc(alias = "DATA_CUBE_OUT_WIDTH")]
pub type DataCubeOutWidth = crate::Reg<data_cube_out_width::DataCubeOutWidthSpec>;
#[doc = "data_cube_out_width"]
pub mod data_cube_out_width;
#[doc = "DATA_CUBE_OUT_HEIGHT (rw) register accessor: data_cube_out_height\n\nYou can [`read`](crate::Reg::read) this register and get [`data_cube_out_height::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data_cube_out_height::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data_cube_out_height`] module"]
#[doc(alias = "DATA_CUBE_OUT_HEIGHT")]
pub type DataCubeOutHeight = crate::Reg<data_cube_out_height::DataCubeOutHeightSpec>;
#[doc = "data_cube_out_height"]
pub mod data_cube_out_height;
#[doc = "DATA_CUBE_OUT_CHANNEL (rw) register accessor: data_cube_out_channel\n\nYou can [`read`](crate::Reg::read) this register and get [`data_cube_out_channel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data_cube_out_channel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data_cube_out_channel`] module"]
#[doc(alias = "DATA_CUBE_OUT_CHANNEL")]
pub type DataCubeOutChannel = crate::Reg<data_cube_out_channel::DataCubeOutChannelSpec>;
#[doc = "data_cube_out_channel"]
pub mod data_cube_out_channel;
#[doc = "OPERATION_MODE_CFG (rw) register accessor: operation_mode_cfg\n\nYou can [`read`](crate::Reg::read) this register and get [`operation_mode_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`operation_mode_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@operation_mode_cfg`] module"]
#[doc(alias = "OPERATION_MODE_CFG")]
pub type OperationModeCfg = crate::Reg<operation_mode_cfg::OperationModeCfgSpec>;
#[doc = "operation_mode_cfg"]
pub mod operation_mode_cfg;
#[doc = "POOLING_KERNEL_CFG (rw) register accessor: pooling_kernel_cfg\n\nYou can [`read`](crate::Reg::read) this register and get [`pooling_kernel_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pooling_kernel_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pooling_kernel_cfg`] module"]
#[doc(alias = "POOLING_KERNEL_CFG")]
pub type PoolingKernelCfg = crate::Reg<pooling_kernel_cfg::PoolingKernelCfgSpec>;
#[doc = "pooling_kernel_cfg"]
pub mod pooling_kernel_cfg;
#[doc = "RECIP_KERNEL_WIDTH (rw) register accessor: recip_kernel_width\n\nYou can [`read`](crate::Reg::read) this register and get [`recip_kernel_width::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`recip_kernel_width::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@recip_kernel_width`] module"]
#[doc(alias = "RECIP_KERNEL_WIDTH")]
pub type RecipKernelWidth = crate::Reg<recip_kernel_width::RecipKernelWidthSpec>;
#[doc = "recip_kernel_width"]
pub mod recip_kernel_width;
#[doc = "RECIP_KERNEL_HEIGHT (rw) register accessor: recip_kernel_height\n\nYou can [`read`](crate::Reg::read) this register and get [`recip_kernel_height::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`recip_kernel_height::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@recip_kernel_height`] module"]
#[doc(alias = "RECIP_KERNEL_HEIGHT")]
pub type RecipKernelHeight = crate::Reg<recip_kernel_height::RecipKernelHeightSpec>;
#[doc = "recip_kernel_height"]
pub mod recip_kernel_height;
#[doc = "POOLING_PADDING_CFG (rw) register accessor: pooling_padding_cfg\n\nYou can [`read`](crate::Reg::read) this register and get [`pooling_padding_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pooling_padding_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pooling_padding_cfg`] module"]
#[doc(alias = "POOLING_PADDING_CFG")]
pub type PoolingPaddingCfg = crate::Reg<pooling_padding_cfg::PoolingPaddingCfgSpec>;
#[doc = "pooling_padding_cfg"]
pub mod pooling_padding_cfg;
#[doc = "PADDING_VALUE_1_CFG (rw) register accessor: padding_value_1_cfg\n\nYou can [`read`](crate::Reg::read) this register and get [`padding_value_1_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`padding_value_1_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@padding_value_1_cfg`] module"]
#[doc(alias = "PADDING_VALUE_1_CFG")]
pub type PaddingValue1Cfg = crate::Reg<padding_value_1_cfg::PaddingValue1CfgSpec>;
#[doc = "padding_value_1_cfg"]
pub mod padding_value_1_cfg;
#[doc = "PADDING_VALUE_2_CFG (rw) register accessor: padding_value_2_cfg\n\nYou can [`read`](crate::Reg::read) this register and get [`padding_value_2_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`padding_value_2_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@padding_value_2_cfg`] module"]
#[doc(alias = "PADDING_VALUE_2_CFG")]
pub type PaddingValue2Cfg = crate::Reg<padding_value_2_cfg::PaddingValue2CfgSpec>;
#[doc = "padding_value_2_cfg"]
pub mod padding_value_2_cfg;
#[doc = "DST_BASE_ADDR (rw) register accessor: dst_base_addr\n\nYou can [`read`](crate::Reg::read) this register and get [`dst_base_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dst_base_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dst_base_addr`] module"]
#[doc(alias = "DST_BASE_ADDR")]
pub type DstBaseAddr = crate::Reg<dst_base_addr::DstBaseAddrSpec>;
#[doc = "dst_base_addr"]
pub mod dst_base_addr;
#[doc = "DST_SURF_STRIDE (rw) register accessor: dst_surf_stride\n\nYou can [`read`](crate::Reg::read) this register and get [`dst_surf_stride::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dst_surf_stride::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dst_surf_stride`] module"]
#[doc(alias = "DST_SURF_STRIDE")]
pub type DstSurfStride = crate::Reg<dst_surf_stride::DstSurfStrideSpec>;
#[doc = "dst_surf_stride"]
pub mod dst_surf_stride;
#[doc = "DATA_FORMAT (rw) register accessor: data_format\n\nYou can [`read`](crate::Reg::read) this register and get [`data_format::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data_format::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data_format`] module"]
#[doc(alias = "DATA_FORMAT")]
pub type DataFormat = crate::Reg<data_format::DataFormatSpec>;
#[doc = "data_format"]
pub mod data_format;
#[doc = "MISC_CTRL (rw) register accessor: misc_ctrl\n\nYou can [`read`](crate::Reg::read) this register and get [`misc_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`misc_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@misc_ctrl`] module"]
#[doc(alias = "MISC_CTRL")]
pub type MiscCtrl = crate::Reg<misc_ctrl::MiscCtrlSpec>;
#[doc = "misc_ctrl"]
pub mod misc_ctrl;
