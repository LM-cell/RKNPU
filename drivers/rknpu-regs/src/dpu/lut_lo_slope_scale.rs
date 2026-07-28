#[doc = "Register `LUT_LO_SLOPE_SCALE` reader"]
pub type R = crate::R<LutLoSlopeScaleSpec>;
#[doc = "Register `LUT_LO_SLOPE_SCALE` writer"]
pub type W = crate::W<LutLoSlopeScaleSpec>;
#[doc = "Field `LUT_LO_SLOPE_UFLOW_SCALE` reader - LO LUT 下溢斜率缩放"]
pub type LutLoSlopeUflowScaleR = crate::FieldReader<u16>;
#[doc = "Field `LUT_LO_SLOPE_UFLOW_SCALE` writer - LO LUT 下溢斜率缩放"]
pub type LutLoSlopeUflowScaleW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `LUT_LO_SLOPE_OFLOW_SCALE` reader - LO LUT 上溢斜率缩放"]
pub type LutLoSlopeOflowScaleR = crate::FieldReader<u16>;
#[doc = "Field `LUT_LO_SLOPE_OFLOW_SCALE` writer - LO LUT 上溢斜率缩放"]
pub type LutLoSlopeOflowScaleW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - LO LUT 下溢斜率缩放"]
    #[inline(always)]
    pub fn lut_lo_slope_uflow_scale(&self) -> LutLoSlopeUflowScaleR {
        LutLoSlopeUflowScaleR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - LO LUT 上溢斜率缩放"]
    #[inline(always)]
    pub fn lut_lo_slope_oflow_scale(&self) -> LutLoSlopeOflowScaleR {
        LutLoSlopeOflowScaleR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - LO LUT 下溢斜率缩放"]
    #[inline(always)]
    pub fn lut_lo_slope_uflow_scale(&mut self) -> LutLoSlopeUflowScaleW<'_, LutLoSlopeScaleSpec> {
        LutLoSlopeUflowScaleW::new(self, 0)
    }
    #[doc = "Bits 16:31 - LO LUT 上溢斜率缩放"]
    #[inline(always)]
    pub fn lut_lo_slope_oflow_scale(&mut self) -> LutLoSlopeOflowScaleW<'_, LutLoSlopeScaleSpec> {
        LutLoSlopeOflowScaleW::new(self, 16)
    }
}
#[doc = "lut_lo_slope_scale\n\nYou can [`read`](crate::Reg::read) this register and get [`lut_lo_slope_scale::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lut_lo_slope_scale::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LutLoSlopeScaleSpec;
impl crate::RegisterSpec for LutLoSlopeScaleSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lut_lo_slope_scale::R`](R) reader structure"]
impl crate::Readable for LutLoSlopeScaleSpec {}
#[doc = "`write(|w| ..)` method takes [`lut_lo_slope_scale::W`](W) writer structure"]
impl crate::Writable for LutLoSlopeScaleSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LUT_LO_SLOPE_SCALE to value 0"]
impl crate::Resettable for LutLoSlopeScaleSpec {}
