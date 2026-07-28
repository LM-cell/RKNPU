#[doc = "Register `LUT_LO_SLOPE_SHIFT` reader"]
pub type R = crate::R<LutLoSlopeShiftSpec>;
#[doc = "Register `LUT_LO_SLOPE_SHIFT` writer"]
pub type W = crate::W<LutLoSlopeShiftSpec>;
#[doc = "Field `LUT_LO_SLOPE_UFLOW_SHIFT` reader - LO LUT 下溢斜率移位"]
pub type LutLoSlopeUflowShiftR = crate::FieldReader;
#[doc = "Field `LUT_LO_SLOPE_UFLOW_SHIFT` writer - LO LUT 下溢斜率移位"]
pub type LutLoSlopeUflowShiftW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `LUT_LO_SLOPE_OFLOW_SHIFT` reader - LO LUT 上溢斜率移位"]
pub type LutLoSlopeOflowShiftR = crate::FieldReader;
#[doc = "Field `LUT_LO_SLOPE_OFLOW_SHIFT` writer - LO LUT 上溢斜率移位"]
pub type LutLoSlopeOflowShiftW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - LO LUT 下溢斜率移位"]
    #[inline(always)]
    pub fn lut_lo_slope_uflow_shift(&self) -> LutLoSlopeUflowShiftR {
        LutLoSlopeUflowShiftR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - LO LUT 上溢斜率移位"]
    #[inline(always)]
    pub fn lut_lo_slope_oflow_shift(&self) -> LutLoSlopeOflowShiftR {
        LutLoSlopeOflowShiftR::new(((self.bits >> 5) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - LO LUT 下溢斜率移位"]
    #[inline(always)]
    pub fn lut_lo_slope_uflow_shift(&mut self) -> LutLoSlopeUflowShiftW<'_, LutLoSlopeShiftSpec> {
        LutLoSlopeUflowShiftW::new(self, 0)
    }
    #[doc = "Bits 5:9 - LO LUT 上溢斜率移位"]
    #[inline(always)]
    pub fn lut_lo_slope_oflow_shift(&mut self) -> LutLoSlopeOflowShiftW<'_, LutLoSlopeShiftSpec> {
        LutLoSlopeOflowShiftW::new(self, 5)
    }
}
#[doc = "lut_lo_slope_shift\n\nYou can [`read`](crate::Reg::read) this register and get [`lut_lo_slope_shift::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lut_lo_slope_shift::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LutLoSlopeShiftSpec;
impl crate::RegisterSpec for LutLoSlopeShiftSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lut_lo_slope_shift::R`](R) reader structure"]
impl crate::Readable for LutLoSlopeShiftSpec {}
#[doc = "`write(|w| ..)` method takes [`lut_lo_slope_shift::W`](W) writer structure"]
impl crate::Writable for LutLoSlopeShiftSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LUT_LO_SLOPE_SHIFT to value 0"]
impl crate::Resettable for LutLoSlopeShiftSpec {}
