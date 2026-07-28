#[doc = "Register `SURFACE_ADD` reader"]
pub type R = crate::R<SurfaceAddSpec>;
#[doc = "Register `SURFACE_ADD` writer"]
pub type W = crate::W<SurfaceAddSpec>;
#[doc = "Field `SURF_ADD` reader - 一行中有多少个 surface"]
pub type SurfAddR = crate::FieldReader<u32>;
#[doc = "Field `SURF_ADD` writer - 一行中有多少个 surface"]
pub type SurfAddW<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    #[doc = "Bits 4:31 - 一行中有多少个 surface"]
    #[inline(always)]
    pub fn surf_add(&self) -> SurfAddR {
        SurfAddR::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bits 4:31 - 一行中有多少个 surface"]
    #[inline(always)]
    pub fn surf_add(&mut self) -> SurfAddW<'_, SurfaceAddSpec> {
        SurfAddW::new(self, 4)
    }
}
#[doc = "surface_add\n\nYou can [`read`](crate::Reg::read) this register and get [`surface_add::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`surface_add::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SurfaceAddSpec;
impl crate::RegisterSpec for SurfaceAddSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`surface_add::R`](R) reader structure"]
impl crate::Readable for SurfaceAddSpec {}
#[doc = "`write(|w| ..)` method takes [`surface_add::W`](W) writer structure"]
impl crate::Writable for SurfaceAddSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SURFACE_ADD to value 0"]
impl crate::Resettable for SurfaceAddSpec {}
