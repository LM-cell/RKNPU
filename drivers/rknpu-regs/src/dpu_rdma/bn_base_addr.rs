#[doc = "Register `BN_BASE_ADDR` reader"]
pub type R = crate::R<BnBaseAddrSpec>;
#[doc = "Register `BN_BASE_ADDR` writer"]
pub type W = crate::W<BnBaseAddrSpec>;
#[doc = "Field `BN_BASE_ADDR` reader - 读取 BN ALU、BN MUL 操作数的基址"]
pub type BnBaseAddrR = crate::FieldReader<u32>;
#[doc = "Field `BN_BASE_ADDR` writer - 读取 BN ALU、BN MUL 操作数的基址"]
pub type BnBaseAddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 读取 BN ALU、BN MUL 操作数的基址"]
    #[inline(always)]
    pub fn bn_base_addr(&self) -> BnBaseAddrR {
        BnBaseAddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 读取 BN ALU、BN MUL 操作数的基址"]
    #[inline(always)]
    pub fn bn_base_addr(&mut self) -> BnBaseAddrW<'_, BnBaseAddrSpec> {
        BnBaseAddrW::new(self, 0)
    }
}
#[doc = "bn_base_addr\n\nYou can [`read`](crate::Reg::read) this register and get [`bn_base_addr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bn_base_addr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BnBaseAddrSpec;
impl crate::RegisterSpec for BnBaseAddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bn_base_addr::R`](R) reader structure"]
impl crate::Readable for BnBaseAddrSpec {}
#[doc = "`write(|w| ..)` method takes [`bn_base_addr::W`](W) writer structure"]
impl crate::Writable for BnBaseAddrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BN_BASE_ADDR to value 0"]
impl crate::Resettable for BnBaseAddrSpec {}
