#[doc = "Register `WT_RD_AMOUNT` reader"]
pub type R = crate::R<WtRdAmountSpec>;
#[doc = "Field `WT_RD_AMOUNT` reader - 权重读取量"]
pub type WtRdAmountR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - 权重读取量"]
    #[inline(always)]
    pub fn wt_rd_amount(&self) -> WtRdAmountR {
        WtRdAmountR::new(self.bits)
    }
}
#[doc = "wt_rd_amount\n\nYou can [`read`](crate::Reg::read) this register and get [`wt_rd_amount::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WtRdAmountSpec;
impl crate::RegisterSpec for WtRdAmountSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wt_rd_amount::R`](R) reader structure"]
impl crate::Readable for WtRdAmountSpec {}
#[doc = "`reset()` method sets WT_RD_AMOUNT to value 0"]
impl crate::Resettable for WtRdAmountSpec {}
