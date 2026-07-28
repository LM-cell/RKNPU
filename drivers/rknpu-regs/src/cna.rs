#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    s_status: SStatus,
    s_pointer: SPointer,
    operation_enable: OperationEnable,
    conv_con1: ConvCon1,
    conv_con2: ConvCon2,
    conv_con3: ConvCon3,
    _reserved6: [u8; 0x08],
    data_size0: DataSize0,
    data_size1: DataSize1,
    data_size2: DataSize2,
    data_size3: DataSize3,
    weight_size0: WeightSize0,
    weight_size1: WeightSize1,
    weight_size2: WeightSize2,
    _reserved13: [u8; 0x04],
    cbuf_con0: CbufCon0,
    cbuf_con1: CbufCon1,
    _reserved15: [u8; 0x04],
    cvt_con0: CvtCon0,
    cvt_con1: CvtCon1,
    cvt_con2: CvtCon2,
    cvt_con3: CvtCon3,
    cvt_con4: CvtCon4,
    fc_con0: FcCon0,
    fc_con1: FcCon1,
    pad_con0: PadCon0,
    _reserved23: [u8; 0x04],
    feature_data_addr: FeatureDataAddr,
    fc_con2: FcCon2,
    dma_con0: DmaCon0,
    dma_con1: DmaCon1,
    dma_con2: DmaCon2,
    fc_data_size0: FcDataSize0,
    fc_data_size1: FcDataSize1,
    _reserved30: [u8; 0x04],
    clk_gate: ClkGate,
    _reserved31: [u8; 0x6c],
    dcomp_ctrl: DcompCtrl,
    dcomp_regnum: DcompRegnum,
    _reserved33: [u8; 0x08],
    dcomp_addr0: DcompAddr0,
    _reserved34: [u8; 0x2c],
    dcomp_amount: [DcompAmount; 16],
    cvt_con5: CvtCon5,
    pad_con1: PadCon1,
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
    #[doc = "0x0c - conv_con1"]
    #[inline(always)]
    pub const fn conv_con1(&self) -> &ConvCon1 {
        &self.conv_con1
    }
    #[doc = "0x10 - conv_con2"]
    #[inline(always)]
    pub const fn conv_con2(&self) -> &ConvCon2 {
        &self.conv_con2
    }
    #[doc = "0x14 - conv_con3"]
    #[inline(always)]
    pub const fn conv_con3(&self) -> &ConvCon3 {
        &self.conv_con3
    }
    #[doc = "0x20 - data_size0"]
    #[inline(always)]
    pub const fn data_size0(&self) -> &DataSize0 {
        &self.data_size0
    }
    #[doc = "0x24 - data_size1"]
    #[inline(always)]
    pub const fn data_size1(&self) -> &DataSize1 {
        &self.data_size1
    }
    #[doc = "0x28 - data_size2"]
    #[inline(always)]
    pub const fn data_size2(&self) -> &DataSize2 {
        &self.data_size2
    }
    #[doc = "0x2c - data_size3"]
    #[inline(always)]
    pub const fn data_size3(&self) -> &DataSize3 {
        &self.data_size3
    }
    #[doc = "0x30 - weight_size0"]
    #[inline(always)]
    pub const fn weight_size0(&self) -> &WeightSize0 {
        &self.weight_size0
    }
    #[doc = "0x34 - weight_size1"]
    #[inline(always)]
    pub const fn weight_size1(&self) -> &WeightSize1 {
        &self.weight_size1
    }
    #[doc = "0x38 - weight_size2"]
    #[inline(always)]
    pub const fn weight_size2(&self) -> &WeightSize2 {
        &self.weight_size2
    }
    #[doc = "0x40 - cbuf_con0"]
    #[inline(always)]
    pub const fn cbuf_con0(&self) -> &CbufCon0 {
        &self.cbuf_con0
    }
    #[doc = "0x44 - cbuf_con1"]
    #[inline(always)]
    pub const fn cbuf_con1(&self) -> &CbufCon1 {
        &self.cbuf_con1
    }
    #[doc = "0x4c - cvt_con0"]
    #[inline(always)]
    pub const fn cvt_con0(&self) -> &CvtCon0 {
        &self.cvt_con0
    }
    #[doc = "0x50 - cvt_con1"]
    #[inline(always)]
    pub const fn cvt_con1(&self) -> &CvtCon1 {
        &self.cvt_con1
    }
    #[doc = "0x54 - cvt_con2"]
    #[inline(always)]
    pub const fn cvt_con2(&self) -> &CvtCon2 {
        &self.cvt_con2
    }
    #[doc = "0x58 - cvt_con3"]
    #[inline(always)]
    pub const fn cvt_con3(&self) -> &CvtCon3 {
        &self.cvt_con3
    }
    #[doc = "0x5c - cvt_con4"]
    #[inline(always)]
    pub const fn cvt_con4(&self) -> &CvtCon4 {
        &self.cvt_con4
    }
    #[doc = "0x60 - fc_con0"]
    #[inline(always)]
    pub const fn fc_con0(&self) -> &FcCon0 {
        &self.fc_con0
    }
    #[doc = "0x64 - fc_con1"]
    #[inline(always)]
    pub const fn fc_con1(&self) -> &FcCon1 {
        &self.fc_con1
    }
    #[doc = "0x68 - pad_con0"]
    #[inline(always)]
    pub const fn pad_con0(&self) -> &PadCon0 {
        &self.pad_con0
    }
    #[doc = "0x70 - feature_data_addr"]
    #[inline(always)]
    pub const fn feature_data_addr(&self) -> &FeatureDataAddr {
        &self.feature_data_addr
    }
    #[doc = "0x74 - fc_con2"]
    #[inline(always)]
    pub const fn fc_con2(&self) -> &FcCon2 {
        &self.fc_con2
    }
    #[doc = "0x78 - dma_con0"]
    #[inline(always)]
    pub const fn dma_con0(&self) -> &DmaCon0 {
        &self.dma_con0
    }
    #[doc = "0x7c - dma_con1"]
    #[inline(always)]
    pub const fn dma_con1(&self) -> &DmaCon1 {
        &self.dma_con1
    }
    #[doc = "0x80 - dma_con2"]
    #[inline(always)]
    pub const fn dma_con2(&self) -> &DmaCon2 {
        &self.dma_con2
    }
    #[doc = "0x84 - fc_data_size0"]
    #[inline(always)]
    pub const fn fc_data_size0(&self) -> &FcDataSize0 {
        &self.fc_data_size0
    }
    #[doc = "0x88 - fc_data_size1"]
    #[inline(always)]
    pub const fn fc_data_size1(&self) -> &FcDataSize1 {
        &self.fc_data_size1
    }
    #[doc = "0x90 - clk_gate"]
    #[inline(always)]
    pub const fn clk_gate(&self) -> &ClkGate {
        &self.clk_gate
    }
    #[doc = "0x100 - dcomp_ctrl"]
    #[inline(always)]
    pub const fn dcomp_ctrl(&self) -> &DcompCtrl {
        &self.dcomp_ctrl
    }
    #[doc = "0x104 - dcomp_regnum"]
    #[inline(always)]
    pub const fn dcomp_regnum(&self) -> &DcompRegnum {
        &self.dcomp_regnum
    }
    #[doc = "0x110 - dcomp_addr0"]
    #[inline(always)]
    pub const fn dcomp_addr0(&self) -> &DcompAddr0 {
        &self.dcomp_addr0
    }
    #[doc = "0x140..0x180 - dcomp_amount"]
    #[inline(always)]
    pub const fn dcomp_amount(&self, n: usize) -> &DcompAmount {
        &self.dcomp_amount[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x140..0x180 - dcomp_amount"]
    #[inline(always)]
    pub fn dcomp_amount_iter(&self) -> impl Iterator<Item = &DcompAmount> {
        self.dcomp_amount.iter()
    }
    #[doc = "0x180 - cvt_con5"]
    #[inline(always)]
    pub const fn cvt_con5(&self) -> &CvtCon5 {
        &self.cvt_con5
    }
    #[doc = "0x184 - pad_con1"]
    #[inline(always)]
    pub const fn pad_con1(&self) -> &PadCon1 {
        &self.pad_con1
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
#[doc = "CONV_CON1 (rw) register accessor: conv_con1\n\nYou can [`read`](crate::Reg::read) this register and get [`conv_con1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`conv_con1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@conv_con1`] module"]
#[doc(alias = "CONV_CON1")]
pub type ConvCon1 = crate::Reg<conv_con1::ConvCon1Spec>;
#[doc = "conv_con1"]
pub mod conv_con1;
#[doc = "CONV_CON2 (rw) register accessor: conv_con2\n\nYou can [`read`](crate::Reg::read) this register and get [`conv_con2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`conv_con2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@conv_con2`] module"]
#[doc(alias = "CONV_CON2")]
pub type ConvCon2 = crate::Reg<conv_con2::ConvCon2Spec>;
#[doc = "conv_con2"]
pub mod conv_con2;
#[doc = "CONV_CON3 (rw) register accessor: conv_con3\n\nYou can [`read`](crate::Reg::read) this register and get [`conv_con3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`conv_con3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@conv_con3`] module"]
#[doc(alias = "CONV_CON3")]
pub type ConvCon3 = crate::Reg<conv_con3::ConvCon3Spec>;
#[doc = "conv_con3"]
pub mod conv_con3;
#[doc = "DATA_SIZE0 (rw) register accessor: data_size0\n\nYou can [`read`](crate::Reg::read) this register and get [`data_size0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data_size0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data_size0`] module"]
#[doc(alias = "DATA_SIZE0")]
pub type DataSize0 = crate::Reg<data_size0::DataSize0Spec>;
#[doc = "data_size0"]
pub mod data_size0;
#[doc = "DATA_SIZE1 (rw) register accessor: data_size1\n\nYou can [`read`](crate::Reg::read) this register and get [`data_size1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data_size1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data_size1`] module"]
#[doc(alias = "DATA_SIZE1")]
pub type DataSize1 = crate::Reg<data_size1::DataSize1Spec>;
#[doc = "data_size1"]
pub mod data_size1;
#[doc = "DATA_SIZE2 (rw) register accessor: data_size2\n\nYou can [`read`](crate::Reg::read) this register and get [`data_size2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data_size2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data_size2`] module"]
#[doc(alias = "DATA_SIZE2")]
pub type DataSize2 = crate::Reg<data_size2::DataSize2Spec>;
#[doc = "data_size2"]
pub mod data_size2;
#[doc = "DATA_SIZE3 (rw) register accessor: data_size3\n\nYou can [`read`](crate::Reg::read) this register and get [`data_size3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data_size3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data_size3`] module"]
#[doc(alias = "DATA_SIZE3")]
pub type DataSize3 = crate::Reg<data_size3::DataSize3Spec>;
#[doc = "data_size3"]
pub mod data_size3;
#[doc = "WEIGHT_SIZE0 (rw) register accessor: weight_size0\n\nYou can [`read`](crate::Reg::read) this register and get [`weight_size0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`weight_size0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@weight_size0`] module"]
#[doc(alias = "WEIGHT_SIZE0")]
pub type WeightSize0 = crate::Reg<weight_size0::WeightSize0Spec>;
#[doc = "weight_size0"]
pub mod weight_size0;
#[doc = "WEIGHT_SIZE1 (rw) register accessor: weight_size1\n\nYou can [`read`](crate::Reg::read) this register and get [`weight_size1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`weight_size1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@weight_size1`] module"]
#[doc(alias = "WEIGHT_SIZE1")]
pub type WeightSize1 = crate::Reg<weight_size1::WeightSize1Spec>;
#[doc = "weight_size1"]
pub mod weight_size1;
#[doc = "WEIGHT_SIZE2 (rw) register accessor: weight_size2\n\nYou can [`read`](crate::Reg::read) this register and get [`weight_size2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`weight_size2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@weight_size2`] module"]
#[doc(alias = "WEIGHT_SIZE2")]
pub type WeightSize2 = crate::Reg<weight_size2::WeightSize2Spec>;
#[doc = "weight_size2"]
pub mod weight_size2;
#[doc = "CBUF_CON0 (rw) register accessor: cbuf_con0\n\nYou can [`read`](crate::Reg::read) this register and get [`cbuf_con0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cbuf_con0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cbuf_con0`] module"]
#[doc(alias = "CBUF_CON0")]
pub type CbufCon0 = crate::Reg<cbuf_con0::CbufCon0Spec>;
#[doc = "cbuf_con0"]
pub mod cbuf_con0;
#[doc = "CBUF_CON1 (rw) register accessor: cbuf_con1\n\nYou can [`read`](crate::Reg::read) this register and get [`cbuf_con1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cbuf_con1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cbuf_con1`] module"]
#[doc(alias = "CBUF_CON1")]
pub type CbufCon1 = crate::Reg<cbuf_con1::CbufCon1Spec>;
#[doc = "cbuf_con1"]
pub mod cbuf_con1;
#[doc = "CVT_CON0 (rw) register accessor: cvt_con0\n\nYou can [`read`](crate::Reg::read) this register and get [`cvt_con0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cvt_con0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cvt_con0`] module"]
#[doc(alias = "CVT_CON0")]
pub type CvtCon0 = crate::Reg<cvt_con0::CvtCon0Spec>;
#[doc = "cvt_con0"]
pub mod cvt_con0;
#[doc = "CVT_CON1 (rw) register accessor: cvt_con1\n\nYou can [`read`](crate::Reg::read) this register and get [`cvt_con1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cvt_con1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cvt_con1`] module"]
#[doc(alias = "CVT_CON1")]
pub type CvtCon1 = crate::Reg<cvt_con1::CvtCon1Spec>;
#[doc = "cvt_con1"]
pub mod cvt_con1;
#[doc = "CVT_CON2 (rw) register accessor: cvt_con2\n\nYou can [`read`](crate::Reg::read) this register and get [`cvt_con2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cvt_con2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cvt_con2`] module"]
#[doc(alias = "CVT_CON2")]
pub type CvtCon2 = crate::Reg<cvt_con2::CvtCon2Spec>;
#[doc = "cvt_con2"]
pub mod cvt_con2;
#[doc = "CVT_CON3 (rw) register accessor: cvt_con3\n\nYou can [`read`](crate::Reg::read) this register and get [`cvt_con3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cvt_con3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cvt_con3`] module"]
#[doc(alias = "CVT_CON3")]
pub type CvtCon3 = crate::Reg<cvt_con3::CvtCon3Spec>;
#[doc = "cvt_con3"]
pub mod cvt_con3;
#[doc = "CVT_CON4 (rw) register accessor: cvt_con4\n\nYou can [`read`](crate::Reg::read) this register and get [`cvt_con4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cvt_con4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cvt_con4`] module"]
#[doc(alias = "CVT_CON4")]
pub type CvtCon4 = crate::Reg<cvt_con4::CvtCon4Spec>;
#[doc = "cvt_con4"]
pub mod cvt_con4;
#[doc = "FC_CON0 (rw) register accessor: fc_con0\n\nYou can [`read`](crate::Reg::read) this register and get [`fc_con0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fc_con0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_con0`] module"]
#[doc(alias = "FC_CON0")]
pub type FcCon0 = crate::Reg<fc_con0::FcCon0Spec>;
#[doc = "fc_con0"]
pub mod fc_con0;
#[doc = "FC_CON1 (rw) register accessor: fc_con1\n\nYou can [`read`](crate::Reg::read) this register and get [`fc_con1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fc_con1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_con1`] module"]
#[doc(alias = "FC_CON1")]
pub type FcCon1 = crate::Reg<fc_con1::FcCon1Spec>;
#[doc = "fc_con1"]
pub mod fc_con1;
#[doc = "PAD_CON0 (rw) register accessor: pad_con0\n\nYou can [`read`](crate::Reg::read) this register and get [`pad_con0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pad_con0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pad_con0`] module"]
#[doc(alias = "PAD_CON0")]
pub type PadCon0 = crate::Reg<pad_con0::PadCon0Spec>;
#[doc = "pad_con0"]
pub mod pad_con0;
#[doc = "FEATURE_DATA_ADDR (rw) register accessor: feature_data_addr\n\nYou can [`read`](crate::Reg::read) this register and get [`feature_data_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`feature_data_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@feature_data_addr`] module"]
#[doc(alias = "FEATURE_DATA_ADDR")]
pub type FeatureDataAddr = crate::Reg<feature_data_addr::FeatureDataAddrSpec>;
#[doc = "feature_data_addr"]
pub mod feature_data_addr;
#[doc = "FC_CON2 (rw) register accessor: fc_con2\n\nYou can [`read`](crate::Reg::read) this register and get [`fc_con2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fc_con2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_con2`] module"]
#[doc(alias = "FC_CON2")]
pub type FcCon2 = crate::Reg<fc_con2::FcCon2Spec>;
#[doc = "fc_con2"]
pub mod fc_con2;
#[doc = "DMA_CON0 (rw) register accessor: dma_con0\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_con0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_con0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_con0`] module"]
#[doc(alias = "DMA_CON0")]
pub type DmaCon0 = crate::Reg<dma_con0::DmaCon0Spec>;
#[doc = "dma_con0"]
pub mod dma_con0;
#[doc = "DMA_CON1 (rw) register accessor: dma_con1\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_con1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_con1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_con1`] module"]
#[doc(alias = "DMA_CON1")]
pub type DmaCon1 = crate::Reg<dma_con1::DmaCon1Spec>;
#[doc = "dma_con1"]
pub mod dma_con1;
#[doc = "DMA_CON2 (rw) register accessor: dma_con2\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_con2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_con2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_con2`] module"]
#[doc(alias = "DMA_CON2")]
pub type DmaCon2 = crate::Reg<dma_con2::DmaCon2Spec>;
#[doc = "dma_con2"]
pub mod dma_con2;
#[doc = "FC_DATA_SIZE0 (rw) register accessor: fc_data_size0\n\nYou can [`read`](crate::Reg::read) this register and get [`fc_data_size0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fc_data_size0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_data_size0`] module"]
#[doc(alias = "FC_DATA_SIZE0")]
pub type FcDataSize0 = crate::Reg<fc_data_size0::FcDataSize0Spec>;
#[doc = "fc_data_size0"]
pub mod fc_data_size0;
#[doc = "FC_DATA_SIZE1 (rw) register accessor: fc_data_size1\n\nYou can [`read`](crate::Reg::read) this register and get [`fc_data_size1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fc_data_size1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_data_size1`] module"]
#[doc(alias = "FC_DATA_SIZE1")]
pub type FcDataSize1 = crate::Reg<fc_data_size1::FcDataSize1Spec>;
#[doc = "fc_data_size1"]
pub mod fc_data_size1;
#[doc = "CLK_GATE (rw) register accessor: clk_gate\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_gate::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_gate::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_gate`] module"]
#[doc(alias = "CLK_GATE")]
pub type ClkGate = crate::Reg<clk_gate::ClkGateSpec>;
#[doc = "clk_gate"]
pub mod clk_gate;
#[doc = "DCOMP_CTRL (rw) register accessor: dcomp_ctrl\n\nYou can [`read`](crate::Reg::read) this register and get [`dcomp_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcomp_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcomp_ctrl`] module"]
#[doc(alias = "DCOMP_CTRL")]
pub type DcompCtrl = crate::Reg<dcomp_ctrl::DcompCtrlSpec>;
#[doc = "dcomp_ctrl"]
pub mod dcomp_ctrl;
#[doc = "DCOMP_REGNUM (rw) register accessor: dcomp_regnum\n\nYou can [`read`](crate::Reg::read) this register and get [`dcomp_regnum::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcomp_regnum::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcomp_regnum`] module"]
#[doc(alias = "DCOMP_REGNUM")]
pub type DcompRegnum = crate::Reg<dcomp_regnum::DcompRegnumSpec>;
#[doc = "dcomp_regnum"]
pub mod dcomp_regnum;
#[doc = "DCOMP_ADDR0 (rw) register accessor: dcomp_addr0\n\nYou can [`read`](crate::Reg::read) this register and get [`dcomp_addr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcomp_addr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcomp_addr0`] module"]
#[doc(alias = "DCOMP_ADDR0")]
pub type DcompAddr0 = crate::Reg<dcomp_addr0::DcompAddr0Spec>;
#[doc = "dcomp_addr0"]
pub mod dcomp_addr0;
#[doc = "DCOMP_AMOUNT (rw) register accessor: dcomp_amount\n\nYou can [`read`](crate::Reg::read) this register and get [`dcomp_amount::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcomp_amount::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcomp_amount`] module"]
#[doc(alias = "DCOMP_AMOUNT")]
pub type DcompAmount = crate::Reg<dcomp_amount::DcompAmountSpec>;
#[doc = "dcomp_amount"]
pub mod dcomp_amount;
#[doc = "CVT_CON5 (rw) register accessor: cvt_con5\n\nYou can [`read`](crate::Reg::read) this register and get [`cvt_con5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cvt_con5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cvt_con5`] module"]
#[doc(alias = "CVT_CON5")]
pub type CvtCon5 = crate::Reg<cvt_con5::CvtCon5Spec>;
#[doc = "cvt_con5"]
pub mod cvt_con5;
#[doc = "PAD_CON1 (rw) register accessor: pad_con1\n\nYou can [`read`](crate::Reg::read) this register and get [`pad_con1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pad_con1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pad_con1`] module"]
#[doc(alias = "PAD_CON1")]
pub type PadCon1 = crate::Reg<pad_con1::PadCon1Spec>;
#[doc = "pad_con1"]
pub mod pad_con1;
