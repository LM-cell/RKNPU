#[doc = "Register `LUT_LE_END` reader"]
pub type R = crate::R<LutLeEndSpec>;
#[doc = "Register `LUT_LE_END` writer"]
pub type W = crate::W<LutLeEndSpec>;
#[doc = "Field `LUT_LE_END` reader - LE LUT 终止点"]
pub type LutLeEndR = crate::FieldReader<u32>;
#[doc = "Field `LUT_LE_END` writer - LE LUT 终止点"]
pub type LutLeEndW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - LE LUT 终止点"]
    #[inline(always)]
    pub fn lut_le_end(&self) -> LutLeEndR {
        LutLeEndR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - LE LUT 终止点"]
    #[inline(always)]
    pub fn lut_le_end(&mut self) -> LutLeEndW<'_, LutLeEndSpec> {
        LutLeEndW::new(self, 0)
    }
}
#[doc = "lut_le_end\n\nYou can [`read`](crate::Reg::read) this register and get [`lut_le_end::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lut_le_end::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LutLeEndSpec;
impl crate::RegisterSpec for LutLeEndSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lut_le_end::R`](R) reader structure"]
impl crate::Readable for LutLeEndSpec {}
#[doc = "`write(|w| ..)` method takes [`lut_le_end::W`](W) writer structure"]
impl crate::Writable for LutLeEndSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LUT_LE_END to value 0"]
impl crate::Resettable for LutLeEndSpec {}
