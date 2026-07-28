#[doc = "Register `RD_WEIGHT_1` reader"]
pub type R = crate::R<RdWeight1Spec>;
#[doc = "Register `RD_WEIGHT_1` writer"]
pub type W = crate::W<RdWeight1Spec>;
#[doc = "Field `RD_WEIGHT_PC` reader - PC 读 burst 权重"]
pub type RdWeightPcR = crate::FieldReader;
#[doc = "Field `RD_WEIGHT_PC` writer - PC 读 burst 权重"]
pub type RdWeightPcW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - PC 读 burst 权重"]
    #[inline(always)]
    pub fn rd_weight_pc(&self) -> RdWeightPcR {
        RdWeightPcR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - PC 读 burst 权重"]
    #[inline(always)]
    pub fn rd_weight_pc(&mut self) -> RdWeightPcW<'_, RdWeight1Spec> {
        RdWeightPcW::new(self, 0)
    }
}
#[doc = "rd_weight_1\n\nYou can [`read`](crate::Reg::read) this register and get [`rd_weight_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rd_weight_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RdWeight1Spec;
impl crate::RegisterSpec for RdWeight1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rd_weight_1::R`](R) reader structure"]
impl crate::Readable for RdWeight1Spec {}
#[doc = "`write(|w| ..)` method takes [`rd_weight_1::W`](W) writer structure"]
impl crate::Writable for RdWeight1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RD_WEIGHT_1 to value 0"]
impl crate::Resettable for RdWeight1Spec {}
