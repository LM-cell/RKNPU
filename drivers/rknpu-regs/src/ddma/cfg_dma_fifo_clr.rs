#[doc = "Register `CFG_DMA_FIFO_CLR` reader"]
pub type R = crate::R<CfgDmaFifoClrSpec>;
#[doc = "Register `CFG_DMA_FIFO_CLR` writer"]
pub type W = crate::W<CfgDmaFifoClrSpec>;
#[doc = "Field `DMA_FIFO_CLR` reader - 清除 DMA FIFO"]
pub type DmaFifoClrR = crate::BitReader;
#[doc = "Field `DMA_FIFO_CLR` writer - 清除 DMA FIFO"]
pub type DmaFifoClrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 清除 DMA FIFO"]
    #[inline(always)]
    pub fn dma_fifo_clr(&self) -> DmaFifoClrR {
        DmaFifoClrR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 清除 DMA FIFO"]
    #[inline(always)]
    pub fn dma_fifo_clr(&mut self) -> DmaFifoClrW<'_, CfgDmaFifoClrSpec> {
        DmaFifoClrW::new(self, 0)
    }
}
#[doc = "cfg_dma_fifo_clr\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_dma_fifo_clr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_dma_fifo_clr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgDmaFifoClrSpec;
impl crate::RegisterSpec for CfgDmaFifoClrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_dma_fifo_clr::R`](R) reader structure"]
impl crate::Readable for CfgDmaFifoClrSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_dma_fifo_clr::W`](W) writer structure"]
impl crate::Writable for CfgDmaFifoClrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CFG_DMA_FIFO_CLR to value 0"]
impl crate::Resettable for CfgDmaFifoClrSpec {}
