#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x08],
    operation_enable: OperationEnable,
}
impl RegisterBlock {
    #[doc = "0x08 - operation_enable"]
    #[inline(always)]
    pub const fn operation_enable(&self) -> &OperationEnable {
        &self.operation_enable
    }
}
#[doc = "OPERATION_ENABLE (rw) register accessor: operation_enable\n\nYou can [`read`](crate::Reg::read) this register and get [`operation_enable::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`operation_enable::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@operation_enable`] module"]
#[doc(alias = "OPERATION_ENABLE")]
pub type OperationEnable = crate::Reg<operation_enable::OperationEnableSpec>;
#[doc = "operation_enable"]
pub mod operation_enable;
