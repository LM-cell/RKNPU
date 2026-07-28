#[doc = "Register `DT_RD_AMOUNT` reader"]
pub type R = crate::R<DtRdAmountSpec>;
#[doc = "Field `DT_RD_AMOUNT` reader - 数据读取量"]
pub type DtRdAmountR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - 数据读取量"]
    #[inline(always)]
    pub fn dt_rd_amount(&self) -> DtRdAmountR {
        DtRdAmountR::new(self.bits)
    }
}
#[doc = "dt_rd_amount\n\nYou can [`read`](crate::Reg::read) this register and get [`dt_rd_amount::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DtRdAmountSpec;
impl crate::RegisterSpec for DtRdAmountSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dt_rd_amount::R`](R) reader structure"]
impl crate::Readable for DtRdAmountSpec {}
#[doc = "`reset()` method sets DT_RD_AMOUNT to value 0"]
impl crate::Resettable for DtRdAmountSpec {}
