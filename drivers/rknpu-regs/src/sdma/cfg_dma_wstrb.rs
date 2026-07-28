#[doc = "Register `CFG_DMA_WSTRB` reader"]
pub type R = crate::R<CfgDmaWstrbSpec>;
#[doc = "Register `CFG_DMA_WSTRB` writer"]
pub type W = crate::W<CfgDmaWstrbSpec>;
#[doc = "Field `WR_WSTRB` reader - AXI 写选通信号"]
pub type WrWstrbR = crate::FieldReader<u32>;
#[doc = "Field `WR_WSTRB` writer - AXI 写选通信号"]
pub type WrWstrbW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - AXI 写选通信号"]
    #[inline(always)]
    pub fn wr_wstrb(&self) -> WrWstrbR {
        WrWstrbR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - AXI 写选通信号"]
    #[inline(always)]
    pub fn wr_wstrb(&mut self) -> WrWstrbW<'_, CfgDmaWstrbSpec> {
        WrWstrbW::new(self, 0)
    }
}
#[doc = "cfg_dma_wstrb\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_dma_wstrb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_dma_wstrb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgDmaWstrbSpec;
impl crate::RegisterSpec for CfgDmaWstrbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_dma_wstrb::R`](R) reader structure"]
impl crate::Readable for CfgDmaWstrbSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_dma_wstrb::W`](W) writer structure"]
impl crate::Writable for CfgDmaWstrbSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CFG_DMA_WSTRB to value 0"]
impl crate::Resettable for CfgDmaWstrbSpec {}
