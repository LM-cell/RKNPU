#[doc = "Register `DT_WR_AMOUNT` reader"]
pub type R = crate::R<DtWrAmountSpec>;
#[doc = "Field `DT_WR_AMOUNT` reader - 数据写入量"]
pub type DtWrAmountR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - 数据写入量"]
    #[inline(always)]
    pub fn dt_wr_amount(&self) -> DtWrAmountR {
        DtWrAmountR::new(self.bits)
    }
}
#[doc = "dt_wr_amount\n\nYou can [`read`](crate::Reg::read) this register and get [`dt_wr_amount::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DtWrAmountSpec;
impl crate::RegisterSpec for DtWrAmountSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dt_wr_amount::R`](R) reader structure"]
impl crate::Readable for DtWrAmountSpec {}
#[doc = "`reset()` method sets DT_WR_AMOUNT to value 0"]
impl crate::Resettable for DtWrAmountSpec {}
