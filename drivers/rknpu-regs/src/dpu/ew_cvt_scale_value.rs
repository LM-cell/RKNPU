#[doc = "Register `EW_CVT_SCALE_VALUE` reader"]
pub type R = crate::R<EwCvtScaleValueSpec>;
#[doc = "Register `EW_CVT_SCALE_VALUE` writer"]
pub type W = crate::W<EwCvtScaleValueSpec>;
#[doc = "Field `EW_OP_CVT_SCALE` reader - EW 转换缩放"]
pub type EwOpCvtScaleR = crate::FieldReader<u16>;
#[doc = "Field `EW_OP_CVT_SCALE` writer - EW 转换缩放"]
pub type EwOpCvtScaleW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `EW_OP_CVT_SHIFT` reader - EW 转换移位值"]
pub type EwOpCvtShiftR = crate::FieldReader;
#[doc = "Field `EW_OP_CVT_SHIFT` writer - EW 转换移位值"]
pub type EwOpCvtShiftW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `EW_TRUNCATE` reader - EW CORE 移位值"]
pub type EwTruncateR = crate::FieldReader<u16>;
#[doc = "Field `EW_TRUNCATE` writer - EW CORE 移位值"]
pub type EwTruncateW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:15 - EW 转换缩放"]
    #[inline(always)]
    pub fn ew_op_cvt_scale(&self) -> EwOpCvtScaleR {
        EwOpCvtScaleR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:21 - EW 转换移位值"]
    #[inline(always)]
    pub fn ew_op_cvt_shift(&self) -> EwOpCvtShiftR {
        EwOpCvtShiftR::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 22:31 - EW CORE 移位值"]
    #[inline(always)]
    pub fn ew_truncate(&self) -> EwTruncateR {
        EwTruncateR::new(((self.bits >> 22) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - EW 转换缩放"]
    #[inline(always)]
    pub fn ew_op_cvt_scale(&mut self) -> EwOpCvtScaleW<'_, EwCvtScaleValueSpec> {
        EwOpCvtScaleW::new(self, 0)
    }
    #[doc = "Bits 16:21 - EW 转换移位值"]
    #[inline(always)]
    pub fn ew_op_cvt_shift(&mut self) -> EwOpCvtShiftW<'_, EwCvtScaleValueSpec> {
        EwOpCvtShiftW::new(self, 16)
    }
    #[doc = "Bits 22:31 - EW CORE 移位值"]
    #[inline(always)]
    pub fn ew_truncate(&mut self) -> EwTruncateW<'_, EwCvtScaleValueSpec> {
        EwTruncateW::new(self, 22)
    }
}
#[doc = "ew_cvt_scale_value\n\nYou can [`read`](crate::Reg::read) this register and get [`ew_cvt_scale_value::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ew_cvt_scale_value::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EwCvtScaleValueSpec;
impl crate::RegisterSpec for EwCvtScaleValueSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ew_cvt_scale_value::R`](R) reader structure"]
impl crate::Readable for EwCvtScaleValueSpec {}
#[doc = "`write(|w| ..)` method takes [`ew_cvt_scale_value::W`](W) writer structure"]
impl crate::Writable for EwCvtScaleValueSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EW_CVT_SCALE_VALUE to value 0"]
impl crate::Resettable for EwCvtScaleValueSpec {}
