#[doc = "Register `DST_BASE_ADDR` reader"]
pub type R = crate::R<DstBaseAddrSpec>;
#[doc = "Register `DST_BASE_ADDR` writer"]
pub type W = crate::W<DstBaseAddrSpec>;
#[doc = "Field `DST_BASE_ADDR` reader - 输出 cube 目标基址"]
pub type DstBaseAddrR = crate::FieldReader<u32>;
#[doc = "Field `DST_BASE_ADDR` writer - 输出 cube 目标基址"]
pub type DstBaseAddrW<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    #[doc = "Bits 4:31 - 输出 cube 目标基址"]
    #[inline(always)]
    pub fn dst_base_addr(&self) -> DstBaseAddrR {
        DstBaseAddrR::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bits 4:31 - 输出 cube 目标基址"]
    #[inline(always)]
    pub fn dst_base_addr(&mut self) -> DstBaseAddrW<'_, DstBaseAddrSpec> {
        DstBaseAddrW::new(self, 4)
    }
}
#[doc = "dst_base_addr\n\nYou can [`read`](crate::Reg::read) this register and get [`dst_base_addr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dst_base_addr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DstBaseAddrSpec;
impl crate::RegisterSpec for DstBaseAddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dst_base_addr::R`](R) reader structure"]
impl crate::Readable for DstBaseAddrSpec {}
#[doc = "`write(|w| ..)` method takes [`dst_base_addr::W`](W) writer structure"]
impl crate::Writable for DstBaseAddrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DST_BASE_ADDR to value 0"]
impl crate::Resettable for DstBaseAddrSpec {}
