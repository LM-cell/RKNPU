#[doc = "Register `LUT_LO_END` reader"]
pub type R = crate::R<LutLoEndSpec>;
#[doc = "Register `LUT_LO_END` writer"]
pub type W = crate::W<LutLoEndSpec>;
#[doc = "Field `LUT_LO_END` reader - LO LUT 终止点"]
pub type LutLoEndR = crate::FieldReader<u32>;
#[doc = "Field `LUT_LO_END` writer - LO LUT 终止点"]
pub type LutLoEndW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - LO LUT 终止点"]
    #[inline(always)]
    pub fn lut_lo_end(&self) -> LutLoEndR {
        LutLoEndR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - LO LUT 终止点"]
    #[inline(always)]
    pub fn lut_lo_end(&mut self) -> LutLoEndW<'_, LutLoEndSpec> {
        LutLoEndW::new(self, 0)
    }
}
#[doc = "lut_lo_end\n\nYou can [`read`](crate::Reg::read) this register and get [`lut_lo_end::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lut_lo_end::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LutLoEndSpec;
impl crate::RegisterSpec for LutLoEndSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lut_lo_end::R`](R) reader structure"]
impl crate::Readable for LutLoEndSpec {}
#[doc = "`write(|w| ..)` method takes [`lut_lo_end::W`](W) writer structure"]
impl crate::Writable for LutLoEndSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LUT_LO_END to value 0"]
impl crate::Resettable for LutLoEndSpec {}
