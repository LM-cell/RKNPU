#[doc = "Register `WR_WEIGHT_0` reader"]
pub type R = crate::R<WrWeight0Spec>;
#[doc = "Register `WR_WEIGHT_0` writer"]
pub type W = crate::W<WrWeight0Spec>;
#[doc = "Field `WR_WEIGHT_DPU` reader - DPU 写权重"]
pub type WrWeightDpuR = crate::FieldReader;
#[doc = "Field `WR_WEIGHT_DPU` writer - DPU 写权重"]
pub type WrWeightDpuW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `WR_WEIGHT_PDP` reader - PPU 写权重"]
pub type WrWeightPdpR = crate::FieldReader;
#[doc = "Field `WR_WEIGHT_PDP` writer - PPU 写权重"]
pub type WrWeightPdpW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - DPU 写权重"]
    #[inline(always)]
    pub fn wr_weight_dpu(&self) -> WrWeightDpuR {
        WrWeightDpuR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - PPU 写权重"]
    #[inline(always)]
    pub fn wr_weight_pdp(&self) -> WrWeightPdpR {
        WrWeightPdpR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - DPU 写权重"]
    #[inline(always)]
    pub fn wr_weight_dpu(&mut self) -> WrWeightDpuW<'_, WrWeight0Spec> {
        WrWeightDpuW::new(self, 0)
    }
    #[doc = "Bits 8:15 - PPU 写权重"]
    #[inline(always)]
    pub fn wr_weight_pdp(&mut self) -> WrWeightPdpW<'_, WrWeight0Spec> {
        WrWeightPdpW::new(self, 8)
    }
}
#[doc = "wr_weight_0\n\nYou can [`read`](crate::Reg::read) this register and get [`wr_weight_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wr_weight_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WrWeight0Spec;
impl crate::RegisterSpec for WrWeight0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wr_weight_0::R`](R) reader structure"]
impl crate::Readable for WrWeight0Spec {}
#[doc = "`write(|w| ..)` method takes [`wr_weight_0::W`](W) writer structure"]
impl crate::Writable for WrWeight0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WR_WEIGHT_0 to value 0"]
impl crate::Resettable for WrWeight0Spec {}
