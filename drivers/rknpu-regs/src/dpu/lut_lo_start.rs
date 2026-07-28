#[doc = "Register `LUT_LO_START` reader"]
pub type R = crate::R<LutLoStartSpec>;
#[doc = "Register `LUT_LO_START` writer"]
pub type W = crate::W<LutLoStartSpec>;
#[doc = "Field `LUT_LO_START` reader - LO LUT 起始点"]
pub type LutLoStartR = crate::FieldReader<u32>;
#[doc = "Field `LUT_LO_START` writer - LO LUT 起始点"]
pub type LutLoStartW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - LO LUT 起始点"]
    #[inline(always)]
    pub fn lut_lo_start(&self) -> LutLoStartR {
        LutLoStartR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - LO LUT 起始点"]
    #[inline(always)]
    pub fn lut_lo_start(&mut self) -> LutLoStartW<'_, LutLoStartSpec> {
        LutLoStartW::new(self, 0)
    }
}
#[doc = "lut_lo_start\n\nYou can [`read`](crate::Reg::read) this register and get [`lut_lo_start::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lut_lo_start::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LutLoStartSpec;
impl crate::RegisterSpec for LutLoStartSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lut_lo_start::R`](R) reader structure"]
impl crate::Readable for LutLoStartSpec {}
#[doc = "`write(|w| ..)` method takes [`lut_lo_start::W`](W) writer structure"]
impl crate::Writable for LutLoStartSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LUT_LO_START to value 0"]
impl crate::Resettable for LutLoStartSpec {}
