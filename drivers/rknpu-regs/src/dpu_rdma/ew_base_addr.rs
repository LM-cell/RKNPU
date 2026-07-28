#[doc = "Register `EW_BASE_ADDR` reader"]
pub type R = crate::R<EwBaseAddrSpec>;
#[doc = "Register `EW_BASE_ADDR` writer"]
pub type W = crate::W<EwBaseAddrSpec>;
#[doc = "Field `EW_BASE_ADDR` reader - 读取 EW 操作数的基址"]
pub type EwBaseAddrR = crate::FieldReader<u32>;
#[doc = "Field `EW_BASE_ADDR` writer - 读取 EW 操作数的基址"]
pub type EwBaseAddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 读取 EW 操作数的基址"]
    #[inline(always)]
    pub fn ew_base_addr(&self) -> EwBaseAddrR {
        EwBaseAddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 读取 EW 操作数的基址"]
    #[inline(always)]
    pub fn ew_base_addr(&mut self) -> EwBaseAddrW<'_, EwBaseAddrSpec> {
        EwBaseAddrW::new(self, 0)
    }
}
#[doc = "ew_base_addr\n\nYou can [`read`](crate::Reg::read) this register and get [`ew_base_addr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ew_base_addr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EwBaseAddrSpec;
impl crate::RegisterSpec for EwBaseAddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ew_base_addr::R`](R) reader structure"]
impl crate::Readable for EwBaseAddrSpec {}
#[doc = "`write(|w| ..)` method takes [`ew_base_addr::W`](W) writer structure"]
impl crate::Writable for EwBaseAddrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EW_BASE_ADDR to value 0"]
impl crate::Resettable for EwBaseAddrSpec {}
