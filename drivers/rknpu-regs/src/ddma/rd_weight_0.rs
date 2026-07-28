#[doc = "Register `RD_WEIGHT_0` reader"]
pub type R = crate::R<RdWeight0Spec>;
#[doc = "Register `RD_WEIGHT_0` writer"]
pub type W = crate::W<RdWeight0Spec>;
#[doc = "Field `RD_WEIGHT_FEATURE` reader - 特征读 burst 权重"]
pub type RdWeightFeatureR = crate::FieldReader;
#[doc = "Field `RD_WEIGHT_FEATURE` writer - 特征读 burst 权重"]
pub type RdWeightFeatureW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RD_WEIGHT_KERNEL` reader - 权重读 burst 权重"]
pub type RdWeightKernelR = crate::FieldReader;
#[doc = "Field `RD_WEIGHT_KERNEL` writer - 权重读 burst 权重"]
pub type RdWeightKernelW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RD_WEIGHT_DPU` reader - DPU 读 burst 权重"]
pub type RdWeightDpuR = crate::FieldReader;
#[doc = "Field `RD_WEIGHT_DPU` writer - DPU 读 burst 权重"]
pub type RdWeightDpuW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RD_WEIGHT_PDP` reader - PPU 读 burst 权重"]
pub type RdWeightPdpR = crate::FieldReader;
#[doc = "Field `RD_WEIGHT_PDP` writer - PPU 读 burst 权重"]
pub type RdWeightPdpW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 特征读 burst 权重"]
    #[inline(always)]
    pub fn rd_weight_feature(&self) -> RdWeightFeatureR {
        RdWeightFeatureR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 权重读 burst 权重"]
    #[inline(always)]
    pub fn rd_weight_kernel(&self) -> RdWeightKernelR {
        RdWeightKernelR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - DPU 读 burst 权重"]
    #[inline(always)]
    pub fn rd_weight_dpu(&self) -> RdWeightDpuR {
        RdWeightDpuR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - PPU 读 burst 权重"]
    #[inline(always)]
    pub fn rd_weight_pdp(&self) -> RdWeightPdpR {
        RdWeightPdpR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 特征读 burst 权重"]
    #[inline(always)]
    pub fn rd_weight_feature(&mut self) -> RdWeightFeatureW<'_, RdWeight0Spec> {
        RdWeightFeatureW::new(self, 0)
    }
    #[doc = "Bits 8:15 - 权重读 burst 权重"]
    #[inline(always)]
    pub fn rd_weight_kernel(&mut self) -> RdWeightKernelW<'_, RdWeight0Spec> {
        RdWeightKernelW::new(self, 8)
    }
    #[doc = "Bits 16:23 - DPU 读 burst 权重"]
    #[inline(always)]
    pub fn rd_weight_dpu(&mut self) -> RdWeightDpuW<'_, RdWeight0Spec> {
        RdWeightDpuW::new(self, 16)
    }
    #[doc = "Bits 24:31 - PPU 读 burst 权重"]
    #[inline(always)]
    pub fn rd_weight_pdp(&mut self) -> RdWeightPdpW<'_, RdWeight0Spec> {
        RdWeightPdpW::new(self, 24)
    }
}
#[doc = "rd_weight_0\n\nYou can [`read`](crate::Reg::read) this register and get [`rd_weight_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rd_weight_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RdWeight0Spec;
impl crate::RegisterSpec for RdWeight0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rd_weight_0::R`](R) reader structure"]
impl crate::Readable for RdWeight0Spec {}
#[doc = "`write(|w| ..)` method takes [`rd_weight_0::W`](W) writer structure"]
impl crate::Writable for RdWeight0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RD_WEIGHT_0 to value 0"]
impl crate::Resettable for RdWeight0Spec {}
