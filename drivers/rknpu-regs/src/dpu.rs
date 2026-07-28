#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    s_status: SStatus,
    s_pointer: SPointer,
    operation_enable: OperationEnable,
    feature_mode_cfg: FeatureModeCfg,
    data_format: DataFormat,
    offset_pend: OffsetPend,
    _reserved6: [u8; 0x08],
    dst_base_addr: DstBaseAddr,
    dst_surf_stride: DstSurfStride,
    _reserved8: [u8; 0x08],
    data_cube_width: DataCubeWidth,
    data_cube_height: DataCubeHeight,
    data_cube_notch_addr: DataCubeNotchAddr,
    data_cube_channel: DataCubeChannel,
    bs_cfg: BsCfg,
    bs_alu_cfg: BsAluCfg,
    bs_mul_cfg: BsMulCfg,
    bs_relux_cmp_value: BsReluxCmpValue,
    bs_ow_cfg: BsOwCfg,
    bs_ow_op: BsOwOp,
    wdma_size_0: WdmaSize0,
    wdma_size_1: WdmaSize1,
    bn_cfg: BnCfg,
    bn_alu_cfg: BnAluCfg,
    bn_mul_cfg: BnMulCfg,
    bn_relux_cmp_value: BnReluxCmpValue,
    ew_cfg: EwCfg,
    ew_cvt_offset_value: EwCvtOffsetValue,
    ew_cvt_scale_value: EwCvtScaleValue,
    ew_relux_cmp_value: EwReluxCmpValue,
    out_cvt_offset: OutCvtOffset,
    out_cvt_scale: OutCvtScale,
    out_cvt_shift: OutCvtShift,
    _reserved31: [u8; 0x04],
    ew_op_value: [EwOpValue; 8],
    _reserved32: [u8; 0x10],
    surface_add: SurfaceAdd,
    _reserved33: [u8; 0x3c],
    lut_access_cfg: LutAccessCfg,
    lut_access_data: LutAccessData,
    lut_cfg: LutCfg,
    lut_info: LutInfo,
    lut_le_start: LutLeStart,
    lut_le_end: LutLeEnd,
    lut_lo_start: LutLoStart,
    lut_lo_end: LutLoEnd,
    lut_le_slope_scale: LutLeSlopeScale,
    lut_le_slope_shift: LutLeSlopeShift,
    lut_lo_slope_scale: LutLoSlopeScale,
    lut_lo_slope_shift: LutLoSlopeShift,
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
    #[doc = "0x0c - feature_mode_cfg"]
    #[inline(always)]
    pub const fn feature_mode_cfg(&self) -> &FeatureModeCfg {
        &self.feature_mode_cfg
    }
    #[doc = "0x10 - data_format"]
    #[inline(always)]
    pub const fn data_format(&self) -> &DataFormat {
        &self.data_format
    }
    #[doc = "0x14 - offset_pend"]
    #[inline(always)]
    pub const fn offset_pend(&self) -> &OffsetPend {
        &self.offset_pend
    }
    #[doc = "0x20 - dst_base_addr"]
    #[inline(always)]
    pub const fn dst_base_addr(&self) -> &DstBaseAddr {
        &self.dst_base_addr
    }
    #[doc = "0x24 - dst_surf_stride"]
    #[inline(always)]
    pub const fn dst_surf_stride(&self) -> &DstSurfStride {
        &self.dst_surf_stride
    }
    #[doc = "0x30 - data_cube_width"]
    #[inline(always)]
    pub const fn data_cube_width(&self) -> &DataCubeWidth {
        &self.data_cube_width
    }
    #[doc = "0x34 - data_cube_height"]
    #[inline(always)]
    pub const fn data_cube_height(&self) -> &DataCubeHeight {
        &self.data_cube_height
    }
    #[doc = "0x38 - data_cube_notch_addr"]
    #[inline(always)]
    pub const fn data_cube_notch_addr(&self) -> &DataCubeNotchAddr {
        &self.data_cube_notch_addr
    }
    #[doc = "0x3c - data_cube_channel"]
    #[inline(always)]
    pub const fn data_cube_channel(&self) -> &DataCubeChannel {
        &self.data_cube_channel
    }
    #[doc = "0x40 - bs_cfg"]
    #[inline(always)]
    pub const fn bs_cfg(&self) -> &BsCfg {
        &self.bs_cfg
    }
    #[doc = "0x44 - bs_alu_cfg"]
    #[inline(always)]
    pub const fn bs_alu_cfg(&self) -> &BsAluCfg {
        &self.bs_alu_cfg
    }
    #[doc = "0x48 - bs_mul_cfg"]
    #[inline(always)]
    pub const fn bs_mul_cfg(&self) -> &BsMulCfg {
        &self.bs_mul_cfg
    }
    #[doc = "0x4c - bs_relux_cmp_value"]
    #[inline(always)]
    pub const fn bs_relux_cmp_value(&self) -> &BsReluxCmpValue {
        &self.bs_relux_cmp_value
    }
    #[doc = "0x50 - bs_ow_cfg"]
    #[inline(always)]
    pub const fn bs_ow_cfg(&self) -> &BsOwCfg {
        &self.bs_ow_cfg
    }
    #[doc = "0x54 - bs_ow_op"]
    #[inline(always)]
    pub const fn bs_ow_op(&self) -> &BsOwOp {
        &self.bs_ow_op
    }
    #[doc = "0x58 - wdma_size_0"]
    #[inline(always)]
    pub const fn wdma_size_0(&self) -> &WdmaSize0 {
        &self.wdma_size_0
    }
    #[doc = "0x5c - wdma_size_1"]
    #[inline(always)]
    pub const fn wdma_size_1(&self) -> &WdmaSize1 {
        &self.wdma_size_1
    }
    #[doc = "0x60 - bn_cfg"]
    #[inline(always)]
    pub const fn bn_cfg(&self) -> &BnCfg {
        &self.bn_cfg
    }
    #[doc = "0x64 - bn_alu_cfg"]
    #[inline(always)]
    pub const fn bn_alu_cfg(&self) -> &BnAluCfg {
        &self.bn_alu_cfg
    }
    #[doc = "0x68 - bn_mul_cfg"]
    #[inline(always)]
    pub const fn bn_mul_cfg(&self) -> &BnMulCfg {
        &self.bn_mul_cfg
    }
    #[doc = "0x6c - bn_relux_cmp_value"]
    #[inline(always)]
    pub const fn bn_relux_cmp_value(&self) -> &BnReluxCmpValue {
        &self.bn_relux_cmp_value
    }
    #[doc = "0x70 - ew_cfg"]
    #[inline(always)]
    pub const fn ew_cfg(&self) -> &EwCfg {
        &self.ew_cfg
    }
    #[doc = "0x74 - ew_cvt_offset_value"]
    #[inline(always)]
    pub const fn ew_cvt_offset_value(&self) -> &EwCvtOffsetValue {
        &self.ew_cvt_offset_value
    }
    #[doc = "0x78 - ew_cvt_scale_value"]
    #[inline(always)]
    pub const fn ew_cvt_scale_value(&self) -> &EwCvtScaleValue {
        &self.ew_cvt_scale_value
    }
    #[doc = "0x7c - ew_relux_cmp_value"]
    #[inline(always)]
    pub const fn ew_relux_cmp_value(&self) -> &EwReluxCmpValue {
        &self.ew_relux_cmp_value
    }
    #[doc = "0x80 - out_cvt_offset"]
    #[inline(always)]
    pub const fn out_cvt_offset(&self) -> &OutCvtOffset {
        &self.out_cvt_offset
    }
    #[doc = "0x84 - out_cvt_scale"]
    #[inline(always)]
    pub const fn out_cvt_scale(&self) -> &OutCvtScale {
        &self.out_cvt_scale
    }
    #[doc = "0x88 - out_cvt_shift"]
    #[inline(always)]
    pub const fn out_cvt_shift(&self) -> &OutCvtShift {
        &self.out_cvt_shift
    }
    #[doc = "0x90..0xb0 - ew_op_value"]
    #[inline(always)]
    pub const fn ew_op_value(&self, n: usize) -> &EwOpValue {
        &self.ew_op_value[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x90..0xb0 - ew_op_value"]
    #[inline(always)]
    pub fn ew_op_value_iter(&self) -> impl Iterator<Item = &EwOpValue> {
        self.ew_op_value.iter()
    }
    #[doc = "0xc0 - surface_add"]
    #[inline(always)]
    pub const fn surface_add(&self) -> &SurfaceAdd {
        &self.surface_add
    }
    #[doc = "0x100 - lut_access_cfg"]
    #[inline(always)]
    pub const fn lut_access_cfg(&self) -> &LutAccessCfg {
        &self.lut_access_cfg
    }
    #[doc = "0x104 - lut_access_data"]
    #[inline(always)]
    pub const fn lut_access_data(&self) -> &LutAccessData {
        &self.lut_access_data
    }
    #[doc = "0x108 - lut_cfg"]
    #[inline(always)]
    pub const fn lut_cfg(&self) -> &LutCfg {
        &self.lut_cfg
    }
    #[doc = "0x10c - lut_info"]
    #[inline(always)]
    pub const fn lut_info(&self) -> &LutInfo {
        &self.lut_info
    }
    #[doc = "0x110 - lut_le_start"]
    #[inline(always)]
    pub const fn lut_le_start(&self) -> &LutLeStart {
        &self.lut_le_start
    }
    #[doc = "0x114 - lut_le_end"]
    #[inline(always)]
    pub const fn lut_le_end(&self) -> &LutLeEnd {
        &self.lut_le_end
    }
    #[doc = "0x118 - lut_lo_start"]
    #[inline(always)]
    pub const fn lut_lo_start(&self) -> &LutLoStart {
        &self.lut_lo_start
    }
    #[doc = "0x11c - lut_lo_end"]
    #[inline(always)]
    pub const fn lut_lo_end(&self) -> &LutLoEnd {
        &self.lut_lo_end
    }
    #[doc = "0x120 - lut_le_slope_scale"]
    #[inline(always)]
    pub const fn lut_le_slope_scale(&self) -> &LutLeSlopeScale {
        &self.lut_le_slope_scale
    }
    #[doc = "0x124 - lut_le_slope_shift"]
    #[inline(always)]
    pub const fn lut_le_slope_shift(&self) -> &LutLeSlopeShift {
        &self.lut_le_slope_shift
    }
    #[doc = "0x128 - lut_lo_slope_scale"]
    #[inline(always)]
    pub const fn lut_lo_slope_scale(&self) -> &LutLoSlopeScale {
        &self.lut_lo_slope_scale
    }
    #[doc = "0x12c - lut_lo_slope_shift"]
    #[inline(always)]
    pub const fn lut_lo_slope_shift(&self) -> &LutLoSlopeShift {
        &self.lut_lo_slope_shift
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
#[doc = "FEATURE_MODE_CFG (rw) register accessor: feature_mode_cfg\n\nYou can [`read`](crate::Reg::read) this register and get [`feature_mode_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`feature_mode_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@feature_mode_cfg`] module"]
#[doc(alias = "FEATURE_MODE_CFG")]
pub type FeatureModeCfg = crate::Reg<feature_mode_cfg::FeatureModeCfgSpec>;
#[doc = "feature_mode_cfg"]
pub mod feature_mode_cfg;
#[doc = "DATA_FORMAT (rw) register accessor: data_format\n\nYou can [`read`](crate::Reg::read) this register and get [`data_format::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data_format::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data_format`] module"]
#[doc(alias = "DATA_FORMAT")]
pub type DataFormat = crate::Reg<data_format::DataFormatSpec>;
#[doc = "data_format"]
pub mod data_format;
#[doc = "OFFSET_PEND (rw) register accessor: offset_pend\n\nYou can [`read`](crate::Reg::read) this register and get [`offset_pend::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`offset_pend::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@offset_pend`] module"]
#[doc(alias = "OFFSET_PEND")]
pub type OffsetPend = crate::Reg<offset_pend::OffsetPendSpec>;
#[doc = "offset_pend"]
pub mod offset_pend;
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
#[doc = "DATA_CUBE_NOTCH_ADDR (rw) register accessor: data_cube_notch_addr\n\nYou can [`read`](crate::Reg::read) this register and get [`data_cube_notch_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data_cube_notch_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data_cube_notch_addr`] module"]
#[doc(alias = "DATA_CUBE_NOTCH_ADDR")]
pub type DataCubeNotchAddr = crate::Reg<data_cube_notch_addr::DataCubeNotchAddrSpec>;
#[doc = "data_cube_notch_addr"]
pub mod data_cube_notch_addr;
#[doc = "DATA_CUBE_CHANNEL (rw) register accessor: data_cube_channel\n\nYou can [`read`](crate::Reg::read) this register and get [`data_cube_channel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data_cube_channel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data_cube_channel`] module"]
#[doc(alias = "DATA_CUBE_CHANNEL")]
pub type DataCubeChannel = crate::Reg<data_cube_channel::DataCubeChannelSpec>;
#[doc = "data_cube_channel"]
pub mod data_cube_channel;
#[doc = "BS_CFG (rw) register accessor: bs_cfg\n\nYou can [`read`](crate::Reg::read) this register and get [`bs_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bs_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bs_cfg`] module"]
#[doc(alias = "BS_CFG")]
pub type BsCfg = crate::Reg<bs_cfg::BsCfgSpec>;
#[doc = "bs_cfg"]
pub mod bs_cfg;
#[doc = "BS_ALU_CFG (rw) register accessor: bs_alu_cfg\n\nYou can [`read`](crate::Reg::read) this register and get [`bs_alu_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bs_alu_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bs_alu_cfg`] module"]
#[doc(alias = "BS_ALU_CFG")]
pub type BsAluCfg = crate::Reg<bs_alu_cfg::BsAluCfgSpec>;
#[doc = "bs_alu_cfg"]
pub mod bs_alu_cfg;
#[doc = "BS_MUL_CFG (rw) register accessor: bs_mul_cfg\n\nYou can [`read`](crate::Reg::read) this register and get [`bs_mul_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bs_mul_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bs_mul_cfg`] module"]
#[doc(alias = "BS_MUL_CFG")]
pub type BsMulCfg = crate::Reg<bs_mul_cfg::BsMulCfgSpec>;
#[doc = "bs_mul_cfg"]
pub mod bs_mul_cfg;
#[doc = "BS_RELUX_CMP_VALUE (rw) register accessor: bs_relux_cmp_value\n\nYou can [`read`](crate::Reg::read) this register and get [`bs_relux_cmp_value::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bs_relux_cmp_value::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bs_relux_cmp_value`] module"]
#[doc(alias = "BS_RELUX_CMP_VALUE")]
pub type BsReluxCmpValue = crate::Reg<bs_relux_cmp_value::BsReluxCmpValueSpec>;
#[doc = "bs_relux_cmp_value"]
pub mod bs_relux_cmp_value;
#[doc = "BS_OW_CFG (rw) register accessor: bs_ow_cfg\n\nYou can [`read`](crate::Reg::read) this register and get [`bs_ow_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bs_ow_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bs_ow_cfg`] module"]
#[doc(alias = "BS_OW_CFG")]
pub type BsOwCfg = crate::Reg<bs_ow_cfg::BsOwCfgSpec>;
#[doc = "bs_ow_cfg"]
pub mod bs_ow_cfg;
#[doc = "BS_OW_OP (rw) register accessor: bs_ow_op\n\nYou can [`read`](crate::Reg::read) this register and get [`bs_ow_op::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bs_ow_op::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bs_ow_op`] module"]
#[doc(alias = "BS_OW_OP")]
pub type BsOwOp = crate::Reg<bs_ow_op::BsOwOpSpec>;
#[doc = "bs_ow_op"]
pub mod bs_ow_op;
#[doc = "WDMA_SIZE_0 (rw) register accessor: wdma_size_0\n\nYou can [`read`](crate::Reg::read) this register and get [`wdma_size_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdma_size_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdma_size_0`] module"]
#[doc(alias = "WDMA_SIZE_0")]
pub type WdmaSize0 = crate::Reg<wdma_size_0::WdmaSize0Spec>;
#[doc = "wdma_size_0"]
pub mod wdma_size_0;
#[doc = "WDMA_SIZE_1 (rw) register accessor: wdma_size_1\n\nYou can [`read`](crate::Reg::read) this register and get [`wdma_size_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdma_size_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdma_size_1`] module"]
#[doc(alias = "WDMA_SIZE_1")]
pub type WdmaSize1 = crate::Reg<wdma_size_1::WdmaSize1Spec>;
#[doc = "wdma_size_1"]
pub mod wdma_size_1;
#[doc = "BN_CFG (rw) register accessor: bn_cfg\n\nYou can [`read`](crate::Reg::read) this register and get [`bn_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bn_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bn_cfg`] module"]
#[doc(alias = "BN_CFG")]
pub type BnCfg = crate::Reg<bn_cfg::BnCfgSpec>;
#[doc = "bn_cfg"]
pub mod bn_cfg;
#[doc = "BN_ALU_CFG (rw) register accessor: bn_alu_cfg\n\nYou can [`read`](crate::Reg::read) this register and get [`bn_alu_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bn_alu_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bn_alu_cfg`] module"]
#[doc(alias = "BN_ALU_CFG")]
pub type BnAluCfg = crate::Reg<bn_alu_cfg::BnAluCfgSpec>;
#[doc = "bn_alu_cfg"]
pub mod bn_alu_cfg;
#[doc = "BN_MUL_CFG (rw) register accessor: bn_mul_cfg\n\nYou can [`read`](crate::Reg::read) this register and get [`bn_mul_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bn_mul_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bn_mul_cfg`] module"]
#[doc(alias = "BN_MUL_CFG")]
pub type BnMulCfg = crate::Reg<bn_mul_cfg::BnMulCfgSpec>;
#[doc = "bn_mul_cfg"]
pub mod bn_mul_cfg;
#[doc = "BN_RELUX_CMP_VALUE (rw) register accessor: bn_relux_cmp_value\n\nYou can [`read`](crate::Reg::read) this register and get [`bn_relux_cmp_value::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bn_relux_cmp_value::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bn_relux_cmp_value`] module"]
#[doc(alias = "BN_RELUX_CMP_VALUE")]
pub type BnReluxCmpValue = crate::Reg<bn_relux_cmp_value::BnReluxCmpValueSpec>;
#[doc = "bn_relux_cmp_value"]
pub mod bn_relux_cmp_value;
#[doc = "EW_CFG (rw) register accessor: ew_cfg\n\nYou can [`read`](crate::Reg::read) this register and get [`ew_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ew_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ew_cfg`] module"]
#[doc(alias = "EW_CFG")]
pub type EwCfg = crate::Reg<ew_cfg::EwCfgSpec>;
#[doc = "ew_cfg"]
pub mod ew_cfg;
#[doc = "EW_CVT_OFFSET_VALUE (rw) register accessor: ew_cvt_offset_value\n\nYou can [`read`](crate::Reg::read) this register and get [`ew_cvt_offset_value::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ew_cvt_offset_value::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ew_cvt_offset_value`] module"]
#[doc(alias = "EW_CVT_OFFSET_VALUE")]
pub type EwCvtOffsetValue = crate::Reg<ew_cvt_offset_value::EwCvtOffsetValueSpec>;
#[doc = "ew_cvt_offset_value"]
pub mod ew_cvt_offset_value;
#[doc = "EW_CVT_SCALE_VALUE (rw) register accessor: ew_cvt_scale_value\n\nYou can [`read`](crate::Reg::read) this register and get [`ew_cvt_scale_value::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ew_cvt_scale_value::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ew_cvt_scale_value`] module"]
#[doc(alias = "EW_CVT_SCALE_VALUE")]
pub type EwCvtScaleValue = crate::Reg<ew_cvt_scale_value::EwCvtScaleValueSpec>;
#[doc = "ew_cvt_scale_value"]
pub mod ew_cvt_scale_value;
#[doc = "EW_RELUX_CMP_VALUE (rw) register accessor: ew_relux_cmp_value\n\nYou can [`read`](crate::Reg::read) this register and get [`ew_relux_cmp_value::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ew_relux_cmp_value::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ew_relux_cmp_value`] module"]
#[doc(alias = "EW_RELUX_CMP_VALUE")]
pub type EwReluxCmpValue = crate::Reg<ew_relux_cmp_value::EwReluxCmpValueSpec>;
#[doc = "ew_relux_cmp_value"]
pub mod ew_relux_cmp_value;
#[doc = "OUT_CVT_OFFSET (rw) register accessor: out_cvt_offset\n\nYou can [`read`](crate::Reg::read) this register and get [`out_cvt_offset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out_cvt_offset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_cvt_offset`] module"]
#[doc(alias = "OUT_CVT_OFFSET")]
pub type OutCvtOffset = crate::Reg<out_cvt_offset::OutCvtOffsetSpec>;
#[doc = "out_cvt_offset"]
pub mod out_cvt_offset;
#[doc = "OUT_CVT_SCALE (rw) register accessor: out_cvt_scale\n\nYou can [`read`](crate::Reg::read) this register and get [`out_cvt_scale::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out_cvt_scale::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_cvt_scale`] module"]
#[doc(alias = "OUT_CVT_SCALE")]
pub type OutCvtScale = crate::Reg<out_cvt_scale::OutCvtScaleSpec>;
#[doc = "out_cvt_scale"]
pub mod out_cvt_scale;
#[doc = "OUT_CVT_SHIFT (rw) register accessor: out_cvt_shift\n\nYou can [`read`](crate::Reg::read) this register and get [`out_cvt_shift::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out_cvt_shift::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_cvt_shift`] module"]
#[doc(alias = "OUT_CVT_SHIFT")]
pub type OutCvtShift = crate::Reg<out_cvt_shift::OutCvtShiftSpec>;
#[doc = "out_cvt_shift"]
pub mod out_cvt_shift;
#[doc = "EW_OP_VALUE (rw) register accessor: ew_op_value\n\nYou can [`read`](crate::Reg::read) this register and get [`ew_op_value::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ew_op_value::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ew_op_value`] module"]
#[doc(alias = "EW_OP_VALUE")]
pub type EwOpValue = crate::Reg<ew_op_value::EwOpValueSpec>;
#[doc = "ew_op_value"]
pub mod ew_op_value;
#[doc = "SURFACE_ADD (rw) register accessor: surface_add\n\nYou can [`read`](crate::Reg::read) this register and get [`surface_add::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`surface_add::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@surface_add`] module"]
#[doc(alias = "SURFACE_ADD")]
pub type SurfaceAdd = crate::Reg<surface_add::SurfaceAddSpec>;
#[doc = "surface_add"]
pub mod surface_add;
#[doc = "LUT_ACCESS_CFG (rw) register accessor: lut_access_cfg\n\nYou can [`read`](crate::Reg::read) this register and get [`lut_access_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lut_access_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lut_access_cfg`] module"]
#[doc(alias = "LUT_ACCESS_CFG")]
pub type LutAccessCfg = crate::Reg<lut_access_cfg::LutAccessCfgSpec>;
#[doc = "lut_access_cfg"]
pub mod lut_access_cfg;
#[doc = "LUT_ACCESS_DATA (rw) register accessor: lut_access_data\n\nYou can [`read`](crate::Reg::read) this register and get [`lut_access_data::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lut_access_data::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lut_access_data`] module"]
#[doc(alias = "LUT_ACCESS_DATA")]
pub type LutAccessData = crate::Reg<lut_access_data::LutAccessDataSpec>;
#[doc = "lut_access_data"]
pub mod lut_access_data;
#[doc = "LUT_CFG (rw) register accessor: lut_cfg\n\nYou can [`read`](crate::Reg::read) this register and get [`lut_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lut_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lut_cfg`] module"]
#[doc(alias = "LUT_CFG")]
pub type LutCfg = crate::Reg<lut_cfg::LutCfgSpec>;
#[doc = "lut_cfg"]
pub mod lut_cfg;
#[doc = "LUT_INFO (rw) register accessor: lut_info\n\nYou can [`read`](crate::Reg::read) this register and get [`lut_info::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lut_info::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lut_info`] module"]
#[doc(alias = "LUT_INFO")]
pub type LutInfo = crate::Reg<lut_info::LutInfoSpec>;
#[doc = "lut_info"]
pub mod lut_info;
#[doc = "LUT_LE_START (rw) register accessor: lut_le_start\n\nYou can [`read`](crate::Reg::read) this register and get [`lut_le_start::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lut_le_start::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lut_le_start`] module"]
#[doc(alias = "LUT_LE_START")]
pub type LutLeStart = crate::Reg<lut_le_start::LutLeStartSpec>;
#[doc = "lut_le_start"]
pub mod lut_le_start;
#[doc = "LUT_LE_END (rw) register accessor: lut_le_end\n\nYou can [`read`](crate::Reg::read) this register and get [`lut_le_end::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lut_le_end::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lut_le_end`] module"]
#[doc(alias = "LUT_LE_END")]
pub type LutLeEnd = crate::Reg<lut_le_end::LutLeEndSpec>;
#[doc = "lut_le_end"]
pub mod lut_le_end;
#[doc = "LUT_LO_START (rw) register accessor: lut_lo_start\n\nYou can [`read`](crate::Reg::read) this register and get [`lut_lo_start::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lut_lo_start::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lut_lo_start`] module"]
#[doc(alias = "LUT_LO_START")]
pub type LutLoStart = crate::Reg<lut_lo_start::LutLoStartSpec>;
#[doc = "lut_lo_start"]
pub mod lut_lo_start;
#[doc = "LUT_LO_END (rw) register accessor: lut_lo_end\n\nYou can [`read`](crate::Reg::read) this register and get [`lut_lo_end::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lut_lo_end::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lut_lo_end`] module"]
#[doc(alias = "LUT_LO_END")]
pub type LutLoEnd = crate::Reg<lut_lo_end::LutLoEndSpec>;
#[doc = "lut_lo_end"]
pub mod lut_lo_end;
#[doc = "LUT_LE_SLOPE_SCALE (rw) register accessor: lut_le_slope_scale\n\nYou can [`read`](crate::Reg::read) this register and get [`lut_le_slope_scale::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lut_le_slope_scale::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lut_le_slope_scale`] module"]
#[doc(alias = "LUT_LE_SLOPE_SCALE")]
pub type LutLeSlopeScale = crate::Reg<lut_le_slope_scale::LutLeSlopeScaleSpec>;
#[doc = "lut_le_slope_scale"]
pub mod lut_le_slope_scale;
#[doc = "LUT_LE_SLOPE_SHIFT (rw) register accessor: lut_le_slope_shift\n\nYou can [`read`](crate::Reg::read) this register and get [`lut_le_slope_shift::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lut_le_slope_shift::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lut_le_slope_shift`] module"]
#[doc(alias = "LUT_LE_SLOPE_SHIFT")]
pub type LutLeSlopeShift = crate::Reg<lut_le_slope_shift::LutLeSlopeShiftSpec>;
#[doc = "lut_le_slope_shift"]
pub mod lut_le_slope_shift;
#[doc = "LUT_LO_SLOPE_SCALE (rw) register accessor: lut_lo_slope_scale\n\nYou can [`read`](crate::Reg::read) this register and get [`lut_lo_slope_scale::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lut_lo_slope_scale::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lut_lo_slope_scale`] module"]
#[doc(alias = "LUT_LO_SLOPE_SCALE")]
pub type LutLoSlopeScale = crate::Reg<lut_lo_slope_scale::LutLoSlopeScaleSpec>;
#[doc = "lut_lo_slope_scale"]
pub mod lut_lo_slope_scale;
#[doc = "LUT_LO_SLOPE_SHIFT (rw) register accessor: lut_lo_slope_shift\n\nYou can [`read`](crate::Reg::read) this register and get [`lut_lo_slope_shift::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lut_lo_slope_shift::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lut_lo_slope_shift`] module"]
#[doc(alias = "LUT_LO_SLOPE_SHIFT")]
pub type LutLoSlopeShift = crate::Reg<lut_lo_slope_shift::LutLoSlopeShiftSpec>;
#[doc = "lut_lo_slope_shift"]
pub mod lut_lo_slope_shift;
