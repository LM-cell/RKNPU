#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cfg_outstanding: CfgOutstanding,
    rd_weight_0: RdWeight0,
    wr_weight_0: WrWeight0,
    cfg_id_error: CfgIdError,
    rd_weight_1: RdWeight1,
    cfg_dma_fifo_clr: CfgDmaFifoClr,
    cfg_dma_arb: CfgDmaArb,
    _reserved7: [u8; 0x04],
    cfg_dma_rd_qos: CfgDmaRdQos,
    cfg_dma_rd_cfg: CfgDmaRdCfg,
    cfg_dma_wr_cfg: CfgDmaWrCfg,
    cfg_dma_wstrb: CfgDmaWstrb,
    cfg_status: CfgStatus,
    dt_wr_amount: DtWrAmount,
    dt_rd_amount: DtRdAmount,
    wt_rd_amount: WtRdAmount,
}
impl RegisterBlock {
    #[doc = "0x00 - cfg_outstanding"]
    #[inline(always)]
    pub const fn cfg_outstanding(&self) -> &CfgOutstanding {
        &self.cfg_outstanding
    }
    #[doc = "0x04 - rd_weight_0"]
    #[inline(always)]
    pub const fn rd_weight_0(&self) -> &RdWeight0 {
        &self.rd_weight_0
    }
    #[doc = "0x08 - wr_weight_0"]
    #[inline(always)]
    pub const fn wr_weight_0(&self) -> &WrWeight0 {
        &self.wr_weight_0
    }
    #[doc = "0x0c - cfg_id_error"]
    #[inline(always)]
    pub const fn cfg_id_error(&self) -> &CfgIdError {
        &self.cfg_id_error
    }
    #[doc = "0x10 - rd_weight_1"]
    #[inline(always)]
    pub const fn rd_weight_1(&self) -> &RdWeight1 {
        &self.rd_weight_1
    }
    #[doc = "0x14 - cfg_dma_fifo_clr"]
    #[inline(always)]
    pub const fn cfg_dma_fifo_clr(&self) -> &CfgDmaFifoClr {
        &self.cfg_dma_fifo_clr
    }
    #[doc = "0x18 - cfg_dma_arb"]
    #[inline(always)]
    pub const fn cfg_dma_arb(&self) -> &CfgDmaArb {
        &self.cfg_dma_arb
    }
    #[doc = "0x20 - cfg_dma_rd_qos"]
    #[inline(always)]
    pub const fn cfg_dma_rd_qos(&self) -> &CfgDmaRdQos {
        &self.cfg_dma_rd_qos
    }
    #[doc = "0x24 - cfg_dma_rd_cfg"]
    #[inline(always)]
    pub const fn cfg_dma_rd_cfg(&self) -> &CfgDmaRdCfg {
        &self.cfg_dma_rd_cfg
    }
    #[doc = "0x28 - cfg_dma_wr_cfg"]
    #[inline(always)]
    pub const fn cfg_dma_wr_cfg(&self) -> &CfgDmaWrCfg {
        &self.cfg_dma_wr_cfg
    }
    #[doc = "0x2c - cfg_dma_wstrb"]
    #[inline(always)]
    pub const fn cfg_dma_wstrb(&self) -> &CfgDmaWstrb {
        &self.cfg_dma_wstrb
    }
    #[doc = "0x30 - cfg_status"]
    #[inline(always)]
    pub const fn cfg_status(&self) -> &CfgStatus {
        &self.cfg_status
    }
    #[doc = "0x34 - dt_wr_amount"]
    #[inline(always)]
    pub const fn dt_wr_amount(&self) -> &DtWrAmount {
        &self.dt_wr_amount
    }
    #[doc = "0x38 - dt_rd_amount"]
    #[inline(always)]
    pub const fn dt_rd_amount(&self) -> &DtRdAmount {
        &self.dt_rd_amount
    }
    #[doc = "0x3c - wt_rd_amount"]
    #[inline(always)]
    pub const fn wt_rd_amount(&self) -> &WtRdAmount {
        &self.wt_rd_amount
    }
}
#[doc = "CFG_OUTSTANDING (rw) register accessor: cfg_outstanding\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_outstanding::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_outstanding::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_outstanding`] module"]
#[doc(alias = "CFG_OUTSTANDING")]
pub type CfgOutstanding = crate::Reg<cfg_outstanding::CfgOutstandingSpec>;
#[doc = "cfg_outstanding"]
pub mod cfg_outstanding;
#[doc = "RD_WEIGHT_0 (rw) register accessor: rd_weight_0\n\nYou can [`read`](crate::Reg::read) this register and get [`rd_weight_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rd_weight_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rd_weight_0`] module"]
#[doc(alias = "RD_WEIGHT_0")]
pub type RdWeight0 = crate::Reg<rd_weight_0::RdWeight0Spec>;
#[doc = "rd_weight_0"]
pub mod rd_weight_0;
#[doc = "WR_WEIGHT_0 (rw) register accessor: wr_weight_0\n\nYou can [`read`](crate::Reg::read) this register and get [`wr_weight_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wr_weight_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wr_weight_0`] module"]
#[doc(alias = "WR_WEIGHT_0")]
pub type WrWeight0 = crate::Reg<wr_weight_0::WrWeight0Spec>;
#[doc = "wr_weight_0"]
pub mod wr_weight_0;
#[doc = "CFG_ID_ERROR (rw) register accessor: cfg_id_error\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_id_error::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_id_error::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_id_error`] module"]
#[doc(alias = "CFG_ID_ERROR")]
pub type CfgIdError = crate::Reg<cfg_id_error::CfgIdErrorSpec>;
#[doc = "cfg_id_error"]
pub mod cfg_id_error;
#[doc = "RD_WEIGHT_1 (rw) register accessor: rd_weight_1\n\nYou can [`read`](crate::Reg::read) this register and get [`rd_weight_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rd_weight_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rd_weight_1`] module"]
#[doc(alias = "RD_WEIGHT_1")]
pub type RdWeight1 = crate::Reg<rd_weight_1::RdWeight1Spec>;
#[doc = "rd_weight_1"]
pub mod rd_weight_1;
#[doc = "CFG_DMA_FIFO_CLR (rw) register accessor: cfg_dma_fifo_clr\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_dma_fifo_clr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_dma_fifo_clr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_dma_fifo_clr`] module"]
#[doc(alias = "CFG_DMA_FIFO_CLR")]
pub type CfgDmaFifoClr = crate::Reg<cfg_dma_fifo_clr::CfgDmaFifoClrSpec>;
#[doc = "cfg_dma_fifo_clr"]
pub mod cfg_dma_fifo_clr;
#[doc = "CFG_DMA_ARB (rw) register accessor: cfg_dma_arb\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_dma_arb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_dma_arb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_dma_arb`] module"]
#[doc(alias = "CFG_DMA_ARB")]
pub type CfgDmaArb = crate::Reg<cfg_dma_arb::CfgDmaArbSpec>;
#[doc = "cfg_dma_arb"]
pub mod cfg_dma_arb;
#[doc = "CFG_DMA_RD_QOS (rw) register accessor: cfg_dma_rd_qos\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_dma_rd_qos::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_dma_rd_qos::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_dma_rd_qos`] module"]
#[doc(alias = "CFG_DMA_RD_QOS")]
pub type CfgDmaRdQos = crate::Reg<cfg_dma_rd_qos::CfgDmaRdQosSpec>;
#[doc = "cfg_dma_rd_qos"]
pub mod cfg_dma_rd_qos;
#[doc = "CFG_DMA_RD_CFG (rw) register accessor: cfg_dma_rd_cfg\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_dma_rd_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_dma_rd_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_dma_rd_cfg`] module"]
#[doc(alias = "CFG_DMA_RD_CFG")]
pub type CfgDmaRdCfg = crate::Reg<cfg_dma_rd_cfg::CfgDmaRdCfgSpec>;
#[doc = "cfg_dma_rd_cfg"]
pub mod cfg_dma_rd_cfg;
#[doc = "CFG_DMA_WR_CFG (rw) register accessor: cfg_dma_wr_cfg\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_dma_wr_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_dma_wr_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_dma_wr_cfg`] module"]
#[doc(alias = "CFG_DMA_WR_CFG")]
pub type CfgDmaWrCfg = crate::Reg<cfg_dma_wr_cfg::CfgDmaWrCfgSpec>;
#[doc = "cfg_dma_wr_cfg"]
pub mod cfg_dma_wr_cfg;
#[doc = "CFG_DMA_WSTRB (rw) register accessor: cfg_dma_wstrb\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_dma_wstrb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_dma_wstrb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_dma_wstrb`] module"]
#[doc(alias = "CFG_DMA_WSTRB")]
pub type CfgDmaWstrb = crate::Reg<cfg_dma_wstrb::CfgDmaWstrbSpec>;
#[doc = "cfg_dma_wstrb"]
pub mod cfg_dma_wstrb;
#[doc = "CFG_STATUS (rw) register accessor: cfg_status\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_status`] module"]
#[doc(alias = "CFG_STATUS")]
pub type CfgStatus = crate::Reg<cfg_status::CfgStatusSpec>;
#[doc = "cfg_status"]
pub mod cfg_status;
#[doc = "DT_WR_AMOUNT (r) register accessor: dt_wr_amount\n\nYou can [`read`](crate::Reg::read) this register and get [`dt_wr_amount::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dt_wr_amount`] module"]
#[doc(alias = "DT_WR_AMOUNT")]
pub type DtWrAmount = crate::Reg<dt_wr_amount::DtWrAmountSpec>;
#[doc = "dt_wr_amount"]
pub mod dt_wr_amount;
#[doc = "DT_RD_AMOUNT (r) register accessor: dt_rd_amount\n\nYou can [`read`](crate::Reg::read) this register and get [`dt_rd_amount::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dt_rd_amount`] module"]
#[doc(alias = "DT_RD_AMOUNT")]
pub type DtRdAmount = crate::Reg<dt_rd_amount::DtRdAmountSpec>;
#[doc = "dt_rd_amount"]
pub mod dt_rd_amount;
#[doc = "WT_RD_AMOUNT (r) register accessor: wt_rd_amount\n\nYou can [`read`](crate::Reg::read) this register and get [`wt_rd_amount::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wt_rd_amount`] module"]
#[doc(alias = "WT_RD_AMOUNT")]
pub type WtRdAmount = crate::Reg<wt_rd_amount::WtRdAmountSpec>;
#[doc = "wt_rd_amount"]
pub mod wt_rd_amount;
