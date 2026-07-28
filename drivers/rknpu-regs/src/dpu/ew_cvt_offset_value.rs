#[doc = "Register `EW_CVT_OFFSET_VALUE` reader"]
pub type R = crate::R<EwCvtOffsetValueSpec>;
#[doc = "Register `EW_CVT_OFFSET_VALUE` writer"]
pub type W = crate::W<EwCvtOffsetValueSpec>;
#[doc = "Field `EW_OP_CVT_OFFSET` reader - EW 转换偏移"]
pub type EwOpCvtOffsetR = crate::FieldReader<u32>;
#[doc = "Field `EW_OP_CVT_OFFSET` writer - EW 转换偏移"]
pub type EwOpCvtOffsetW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - EW 转换偏移"]
    #[inline(always)]
    pub fn ew_op_cvt_offset(&self) -> EwOpCvtOffsetR {
        EwOpCvtOffsetR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - EW 转换偏移"]
    #[inline(always)]
    pub fn ew_op_cvt_offset(&mut self) -> EwOpCvtOffsetW<'_, EwCvtOffsetValueSpec> {
        EwOpCvtOffsetW::new(self, 0)
    }
}
#[doc = "ew_cvt_offset_value\n\nYou can [`read`](crate::Reg::read) this register and get [`ew_cvt_offset_value::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ew_cvt_offset_value::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EwCvtOffsetValueSpec;
impl crate::RegisterSpec for EwCvtOffsetValueSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ew_cvt_offset_value::R`](R) reader structure"]
impl crate::Readable for EwCvtOffsetValueSpec {}
#[doc = "`write(|w| ..)` method takes [`ew_cvt_offset_value::W`](W) writer structure"]
impl crate::Writable for EwCvtOffsetValueSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EW_CVT_OFFSET_VALUE to value 0"]
impl crate::Resettable for EwCvtOffsetValueSpec {}
