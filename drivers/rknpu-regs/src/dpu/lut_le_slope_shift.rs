#[doc = "Register `LUT_LE_SLOPE_SHIFT` reader"]
pub type R = crate::R<LutLeSlopeShiftSpec>;
#[doc = "Register `LUT_LE_SLOPE_SHIFT` writer"]
pub type W = crate::W<LutLeSlopeShiftSpec>;
#[doc = "Field `LUT_LE_SLOPE_UFLOW_SHIFT` reader - LE LUT 下溢斜率移位"]
pub type LutLeSlopeUflowShiftR = crate::FieldReader;
#[doc = "Field `LUT_LE_SLOPE_UFLOW_SHIFT` writer - LE LUT 下溢斜率移位"]
pub type LutLeSlopeUflowShiftW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `LUT_LE_SLOPE_OFLOW_SHIFT` reader - LE LUT 上溢斜率移位"]
pub type LutLeSlopeOflowShiftR = crate::FieldReader;
#[doc = "Field `LUT_LE_SLOPE_OFLOW_SHIFT` writer - LE LUT 上溢斜率移位"]
pub type LutLeSlopeOflowShiftW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - LE LUT 下溢斜率移位"]
    #[inline(always)]
    pub fn lut_le_slope_uflow_shift(&self) -> LutLeSlopeUflowShiftR {
        LutLeSlopeUflowShiftR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - LE LUT 上溢斜率移位"]
    #[inline(always)]
    pub fn lut_le_slope_oflow_shift(&self) -> LutLeSlopeOflowShiftR {
        LutLeSlopeOflowShiftR::new(((self.bits >> 5) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - LE LUT 下溢斜率移位"]
    #[inline(always)]
    pub fn lut_le_slope_uflow_shift(&mut self) -> LutLeSlopeUflowShiftW<'_, LutLeSlopeShiftSpec> {
        LutLeSlopeUflowShiftW::new(self, 0)
    }
    #[doc = "Bits 5:9 - LE LUT 上溢斜率移位"]
    #[inline(always)]
    pub fn lut_le_slope_oflow_shift(&mut self) -> LutLeSlopeOflowShiftW<'_, LutLeSlopeShiftSpec> {
        LutLeSlopeOflowShiftW::new(self, 5)
    }
}
#[doc = "lut_le_slope_shift\n\nYou can [`read`](crate::Reg::read) this register and get [`lut_le_slope_shift::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lut_le_slope_shift::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LutLeSlopeShiftSpec;
impl crate::RegisterSpec for LutLeSlopeShiftSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lut_le_slope_shift::R`](R) reader structure"]
impl crate::Readable for LutLeSlopeShiftSpec {}
#[doc = "`write(|w| ..)` method takes [`lut_le_slope_shift::W`](W) writer structure"]
impl crate::Writable for LutLeSlopeShiftSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LUT_LE_SLOPE_SHIFT to value 0"]
impl crate::Resettable for LutLeSlopeShiftSpec {}
