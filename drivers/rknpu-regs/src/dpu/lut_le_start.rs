#[doc = "Register `LUT_LE_START` reader"]
pub type R = crate::R<LutLeStartSpec>;
#[doc = "Register `LUT_LE_START` writer"]
pub type W = crate::W<LutLeStartSpec>;
#[doc = "Field `LUT_LE_START` reader - LE LUT 起始点"]
pub type LutLeStartR = crate::FieldReader<u32>;
#[doc = "Field `LUT_LE_START` writer - LE LUT 起始点"]
pub type LutLeStartW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - LE LUT 起始点"]
    #[inline(always)]
    pub fn lut_le_start(&self) -> LutLeStartR {
        LutLeStartR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - LE LUT 起始点"]
    #[inline(always)]
    pub fn lut_le_start(&mut self) -> LutLeStartW<'_, LutLeStartSpec> {
        LutLeStartW::new(self, 0)
    }
}
#[doc = "lut_le_start\n\nYou can [`read`](crate::Reg::read) this register and get [`lut_le_start::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lut_le_start::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LutLeStartSpec;
impl crate::RegisterSpec for LutLeStartSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lut_le_start::R`](R) reader structure"]
impl crate::Readable for LutLeStartSpec {}
#[doc = "`write(|w| ..)` method takes [`lut_le_start::W`](W) writer structure"]
impl crate::Writable for LutLeStartSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LUT_LE_START to value 0"]
impl crate::Resettable for LutLeStartSpec {}
