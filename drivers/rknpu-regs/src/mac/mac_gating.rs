#[doc = "Register `MAC_GATING` reader"]
pub type R = crate::R<MacGatingSpec>;
#[doc = "Register `MAC_GATING` writer"]
pub type W = crate::W<MacGatingSpec>;
#[doc = "Field `SLCG_OP_EN` reader - 软时钟门控信号"]
pub type SlcgOpEnR = crate::FieldReader<u32>;
#[doc = "Field `SLCG_OP_EN` writer - 软时钟门控信号"]
pub type SlcgOpEnW<'a, REG> = crate::FieldWriter<'a, REG, 27, u32>;
impl R {
    #[doc = "Bits 0:26 - 软时钟门控信号"]
    #[inline(always)]
    pub fn slcg_op_en(&self) -> SlcgOpEnR {
        SlcgOpEnR::new(self.bits & 0x07ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:26 - 软时钟门控信号"]
    #[inline(always)]
    pub fn slcg_op_en(&mut self) -> SlcgOpEnW<'_, MacGatingSpec> {
        SlcgOpEnW::new(self, 0)
    }
}
#[doc = "mac_gating\n\nYou can [`read`](crate::Reg::read) this register and get [`mac_gating::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mac_gating::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MacGatingSpec;
impl crate::RegisterSpec for MacGatingSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mac_gating::R`](R) reader structure"]
impl crate::Readable for MacGatingSpec {}
#[doc = "`write(|w| ..)` method takes [`mac_gating::W`](W) writer structure"]
impl crate::Writable for MacGatingSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MAC_GATING to value 0x0780_0800"]
impl crate::Resettable for MacGatingSpec {
    const RESET_VALUE: u32 = 0x0780_0800;
}
