#[doc = "Register `BS_BASE_ADDR` reader"]
pub type R = crate::R<BsBaseAddrSpec>;
#[doc = "Register `BS_BASE_ADDR` writer"]
pub type W = crate::W<BsBaseAddrSpec>;
#[doc = "Field `BS_BASE_ADDR` reader - 读取 BS ALU、BS CPEND、BS MUL 操作数的基址"]
pub type BsBaseAddrR = crate::FieldReader<u32>;
#[doc = "Field `BS_BASE_ADDR` writer - 读取 BS ALU、BS CPEND、BS MUL 操作数的基址"]
pub type BsBaseAddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 读取 BS ALU、BS CPEND、BS MUL 操作数的基址"]
    #[inline(always)]
    pub fn bs_base_addr(&self) -> BsBaseAddrR {
        BsBaseAddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 读取 BS ALU、BS CPEND、BS MUL 操作数的基址"]
    #[inline(always)]
    pub fn bs_base_addr(&mut self) -> BsBaseAddrW<'_, BsBaseAddrSpec> {
        BsBaseAddrW::new(self, 0)
    }
}
#[doc = "bs_base_addr\n\nYou can [`read`](crate::Reg::read) this register and get [`bs_base_addr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bs_base_addr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BsBaseAddrSpec;
impl crate::RegisterSpec for BsBaseAddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bs_base_addr::R`](R) reader structure"]
impl crate::Readable for BsBaseAddrSpec {}
#[doc = "`write(|w| ..)` method takes [`bs_base_addr::W`](W) writer structure"]
impl crate::Writable for BsBaseAddrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BS_BASE_ADDR to value 0"]
impl crate::Resettable for BsBaseAddrSpec {}
