#[doc = "Register `OFFSET_PEND` reader"]
pub type R = crate::R<OffsetPendSpec>;
#[doc = "Register `OFFSET_PEND` writer"]
pub type W = crate::W<OffsetPendSpec>;
#[doc = "Field `OFFSET_PEND` reader - 额外通道设置值"]
pub type OffsetPendR = crate::FieldReader<u16>;
#[doc = "Field `OFFSET_PEND` writer - 额外通道设置值"]
pub type OffsetPendW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 额外通道设置值"]
    #[inline(always)]
    pub fn offset_pend(&self) -> OffsetPendR {
        OffsetPendR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 额外通道设置值"]
    #[inline(always)]
    pub fn offset_pend(&mut self) -> OffsetPendW<'_, OffsetPendSpec> {
        OffsetPendW::new(self, 0)
    }
}
#[doc = "offset_pend\n\nYou can [`read`](crate::Reg::read) this register and get [`offset_pend::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`offset_pend::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OffsetPendSpec;
impl crate::RegisterSpec for OffsetPendSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`offset_pend::R`](R) reader structure"]
impl crate::Readable for OffsetPendSpec {}
#[doc = "`write(|w| ..)` method takes [`offset_pend::W`](W) writer structure"]
impl crate::Writable for OffsetPendSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OFFSET_PEND to value 0"]
impl crate::Resettable for OffsetPendSpec {}
