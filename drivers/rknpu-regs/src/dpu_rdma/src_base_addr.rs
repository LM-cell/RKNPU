#[doc = "Register `SRC_BASE_ADDR` reader"]
pub type R = crate::R<SrcBaseAddrSpec>;
#[doc = "Register `SRC_BASE_ADDR` writer"]
pub type W = crate::W<SrcBaseAddrSpec>;
#[doc = "Field `SRC_BASE_ADDR` reader - Flying 模式源地址"]
pub type SrcBaseAddrR = crate::FieldReader<u32>;
#[doc = "Field `SRC_BASE_ADDR` writer - Flying 模式源地址"]
pub type SrcBaseAddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Flying 模式源地址"]
    #[inline(always)]
    pub fn src_base_addr(&self) -> SrcBaseAddrR {
        SrcBaseAddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Flying 模式源地址"]
    #[inline(always)]
    pub fn src_base_addr(&mut self) -> SrcBaseAddrW<'_, SrcBaseAddrSpec> {
        SrcBaseAddrW::new(self, 0)
    }
}
#[doc = "src_base_addr\n\nYou can [`read`](crate::Reg::read) this register and get [`src_base_addr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`src_base_addr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrcBaseAddrSpec;
impl crate::RegisterSpec for SrcBaseAddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`src_base_addr::R`](R) reader structure"]
impl crate::Readable for SrcBaseAddrSpec {}
#[doc = "`write(|w| ..)` method takes [`src_base_addr::W`](W) writer structure"]
impl crate::Writable for SrcBaseAddrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SRC_BASE_ADDR to value 0"]
impl crate::Resettable for SrcBaseAddrSpec {}
