#[doc = "Register `CFG_DMA_RD_QOS` reader"]
pub type R = crate::R<CfgDmaRdQosSpec>;
#[doc = "Register `CFG_DMA_RD_QOS` writer"]
pub type W = crate::W<CfgDmaRdQosSpec>;
#[doc = "Field `RD_FEATURE_QOS` reader - Feature 读 QoS"]
pub type RdFeatureQosR = crate::FieldReader;
#[doc = "Field `RD_FEATURE_QOS` writer - Feature 读 QoS"]
pub type RdFeatureQosW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RD_KERNEL_QOS` reader - Kernel 读 QoS"]
pub type RdKernelQosR = crate::FieldReader;
#[doc = "Field `RD_KERNEL_QOS` writer - Kernel 读 QoS"]
pub type RdKernelQosW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RD_DPU_QOS` reader - DPU 读 QoS"]
pub type RdDpuQosR = crate::FieldReader;
#[doc = "Field `RD_DPU_QOS` writer - DPU 读 QoS"]
pub type RdDpuQosW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RD_PPU_QOS` reader - PPU 读 QoS"]
pub type RdPpuQosR = crate::FieldReader;
#[doc = "Field `RD_PPU_QOS` writer - PPU 读 QoS"]
pub type RdPpuQosW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RD_PC_QOS` reader - PC 读 QoS"]
pub type RdPcQosR = crate::FieldReader;
#[doc = "Field `RD_PC_QOS` writer - PC 读 QoS"]
pub type RdPcQosW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - Feature 读 QoS"]
    #[inline(always)]
    pub fn rd_feature_qos(&self) -> RdFeatureQosR {
        RdFeatureQosR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Kernel 读 QoS"]
    #[inline(always)]
    pub fn rd_kernel_qos(&self) -> RdKernelQosR {
        RdKernelQosR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - DPU 读 QoS"]
    #[inline(always)]
    pub fn rd_dpu_qos(&self) -> RdDpuQosR {
        RdDpuQosR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - PPU 读 QoS"]
    #[inline(always)]
    pub fn rd_ppu_qos(&self) -> RdPpuQosR {
        RdPpuQosR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - PC 读 QoS"]
    #[inline(always)]
    pub fn rd_pc_qos(&self) -> RdPcQosR {
        RdPcQosR::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Feature 读 QoS"]
    #[inline(always)]
    pub fn rd_feature_qos(&mut self) -> RdFeatureQosW<'_, CfgDmaRdQosSpec> {
        RdFeatureQosW::new(self, 0)
    }
    #[doc = "Bits 2:3 - Kernel 读 QoS"]
    #[inline(always)]
    pub fn rd_kernel_qos(&mut self) -> RdKernelQosW<'_, CfgDmaRdQosSpec> {
        RdKernelQosW::new(self, 2)
    }
    #[doc = "Bits 4:5 - DPU 读 QoS"]
    #[inline(always)]
    pub fn rd_dpu_qos(&mut self) -> RdDpuQosW<'_, CfgDmaRdQosSpec> {
        RdDpuQosW::new(self, 4)
    }
    #[doc = "Bits 6:7 - PPU 读 QoS"]
    #[inline(always)]
    pub fn rd_ppu_qos(&mut self) -> RdPpuQosW<'_, CfgDmaRdQosSpec> {
        RdPpuQosW::new(self, 6)
    }
    #[doc = "Bits 8:9 - PC 读 QoS"]
    #[inline(always)]
    pub fn rd_pc_qos(&mut self) -> RdPcQosW<'_, CfgDmaRdQosSpec> {
        RdPcQosW::new(self, 8)
    }
}
#[doc = "cfg_dma_rd_qos\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_dma_rd_qos::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_dma_rd_qos::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgDmaRdQosSpec;
impl crate::RegisterSpec for CfgDmaRdQosSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_dma_rd_qos::R`](R) reader structure"]
impl crate::Readable for CfgDmaRdQosSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_dma_rd_qos::W`](W) writer structure"]
impl crate::Writable for CfgDmaRdQosSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CFG_DMA_RD_QOS to value 0"]
impl crate::Resettable for CfgDmaRdQosSpec {}
