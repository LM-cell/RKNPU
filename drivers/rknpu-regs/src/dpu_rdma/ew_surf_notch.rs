#[doc = "Register `EW_SURF_NOTCH` reader"]
pub type R = crate::R<EwSurfNotchSpec>;
#[doc = "Register `EW_SURF_NOTCH` writer"]
pub type W = crate::W<EwSurfNotchSpec>;
#[doc = "Field `EW_SURF_NOTCH` reader - EW surface notch"]
pub type EwSurfNotchR = crate::FieldReader<u32>;
#[doc = "Field `EW_SURF_NOTCH` writer - EW surface notch"]
pub type EwSurfNotchW<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    #[doc = "Bits 4:31 - EW surface notch"]
    #[inline(always)]
    pub fn ew_surf_notch(&self) -> EwSurfNotchR {
        EwSurfNotchR::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bits 4:31 - EW surface notch"]
    #[inline(always)]
    pub fn ew_surf_notch(&mut self) -> EwSurfNotchW<'_, EwSurfNotchSpec> {
        EwSurfNotchW::new(self, 4)
    }
}
#[doc = "ew_surf_notch\n\nYou can [`read`](crate::Reg::read) this register and get [`ew_surf_notch::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ew_surf_notch::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EwSurfNotchSpec;
impl crate::RegisterSpec for EwSurfNotchSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ew_surf_notch::R`](R) reader structure"]
impl crate::Readable for EwSurfNotchSpec {}
#[doc = "`write(|w| ..)` method takes [`ew_surf_notch::W`](W) writer structure"]
impl crate::Writable for EwSurfNotchSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EW_SURF_NOTCH to value 0"]
impl crate::Resettable for EwSurfNotchSpec {}
