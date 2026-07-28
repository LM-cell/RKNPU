#[doc = "Register `DCOMP_ADDR0` reader"]
pub type R = crate::R<DcompAddr0Spec>;
#[doc = "Register `DCOMP_ADDR0` writer"]
pub type W = crate::W<DcompAddr0Spec>;
#[doc = "Field `DECOMPRESS_ADDR0` reader - 权重基址"]
pub type DecompressAddr0R = crate::FieldReader<u32>;
#[doc = "Field `DECOMPRESS_ADDR0` writer - 权重基址"]
pub type DecompressAddr0W<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    #[doc = "Bits 4:31 - 权重基址"]
    #[inline(always)]
    pub fn decompress_addr0(&self) -> DecompressAddr0R {
        DecompressAddr0R::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bits 4:31 - 权重基址"]
    #[inline(always)]
    pub fn decompress_addr0(&mut self) -> DecompressAddr0W<'_, DcompAddr0Spec> {
        DecompressAddr0W::new(self, 4)
    }
}
#[doc = "dcomp_addr0\n\nYou can [`read`](crate::Reg::read) this register and get [`dcomp_addr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcomp_addr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DcompAddr0Spec;
impl crate::RegisterSpec for DcompAddr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcomp_addr0::R`](R) reader structure"]
impl crate::Readable for DcompAddr0Spec {}
#[doc = "`write(|w| ..)` method takes [`dcomp_addr0::W`](W) writer structure"]
impl crate::Writable for DcompAddr0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DCOMP_ADDR0 to value 0"]
impl crate::Resettable for DcompAddr0Spec {}
