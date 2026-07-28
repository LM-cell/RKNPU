#[doc = "Register `LUT_LE_SLOPE_SCALE` reader"]
pub type R = crate::R<LutLeSlopeScaleSpec>;
#[doc = "Register `LUT_LE_SLOPE_SCALE` writer"]
pub type W = crate::W<LutLeSlopeScaleSpec>;
#[doc = "Field `LUT_LE_SLOPE_UFLOW_SCALE` reader - LE LUT 下溢斜率缩放"]
pub type LutLeSlopeUflowScaleR = crate::FieldReader<u16>;
#[doc = "Field `LUT_LE_SLOPE_UFLOW_SCALE` writer - LE LUT 下溢斜率缩放"]
pub type LutLeSlopeUflowScaleW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `LUT_LE_SLOPE_OFLOW_SCALE` reader - LE LUT 上溢斜率缩放"]
pub type LutLeSlopeOflowScaleR = crate::FieldReader<u16>;
#[doc = "Field `LUT_LE_SLOPE_OFLOW_SCALE` writer - LE LUT 上溢斜率缩放"]
pub type LutLeSlopeOflowScaleW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - LE LUT 下溢斜率缩放"]
    #[inline(always)]
    pub fn lut_le_slope_uflow_scale(&self) -> LutLeSlopeUflowScaleR {
        LutLeSlopeUflowScaleR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - LE LUT 上溢斜率缩放"]
    #[inline(always)]
    pub fn lut_le_slope_oflow_scale(&self) -> LutLeSlopeOflowScaleR {
        LutLeSlopeOflowScaleR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - LE LUT 下溢斜率缩放"]
    #[inline(always)]
    pub fn lut_le_slope_uflow_scale(&mut self) -> LutLeSlopeUflowScaleW<'_, LutLeSlopeScaleSpec> {
        LutLeSlopeUflowScaleW::new(self, 0)
    }
    #[doc = "Bits 16:31 - LE LUT 上溢斜率缩放"]
    #[inline(always)]
    pub fn lut_le_slope_oflow_scale(&mut self) -> LutLeSlopeOflowScaleW<'_, LutLeSlopeScaleSpec> {
        LutLeSlopeOflowScaleW::new(self, 16)
    }
}
#[doc = "lut_le_slope_scale\n\nYou can [`read`](crate::Reg::read) this register and get [`lut_le_slope_scale::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lut_le_slope_scale::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LutLeSlopeScaleSpec;
impl crate::RegisterSpec for LutLeSlopeScaleSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lut_le_slope_scale::R`](R) reader structure"]
impl crate::Readable for LutLeSlopeScaleSpec {}
#[doc = "`write(|w| ..)` method takes [`lut_le_slope_scale::W`](W) writer structure"]
impl crate::Writable for LutLeSlopeScaleSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LUT_LE_SLOPE_SCALE to value 0"]
impl crate::Resettable for LutLeSlopeScaleSpec {}
