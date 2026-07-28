#[doc = "Register `TASK_DMA_BASE_ADDR` reader"]
pub type R = crate::R<TaskDmaBaseAddrSpec>;
#[doc = "Register `TASK_DMA_BASE_ADDR` writer"]
pub type W = crate::W<TaskDmaBaseAddrSpec>;
#[doc = "Field `DMA_BASE_ADDR` reader - 任务基址。各 DMA（feature DMA、weight DMA、DPU DMA、PPU DMA）的地址设为偏移地址，AXI 总线上的最终地址 = 基址 + 偏移地址"]
pub type DmaBaseAddrR = crate::FieldReader<u32>;
#[doc = "Field `DMA_BASE_ADDR` writer - 任务基址。各 DMA（feature DMA、weight DMA、DPU DMA、PPU DMA）的地址设为偏移地址，AXI 总线上的最终地址 = 基址 + 偏移地址"]
pub type DmaBaseAddrW<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    #[doc = "Bits 4:31 - 任务基址。各 DMA（feature DMA、weight DMA、DPU DMA、PPU DMA）的地址设为偏移地址，AXI 总线上的最终地址 = 基址 + 偏移地址"]
    #[inline(always)]
    pub fn dma_base_addr(&self) -> DmaBaseAddrR {
        DmaBaseAddrR::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bits 4:31 - 任务基址。各 DMA（feature DMA、weight DMA、DPU DMA、PPU DMA）的地址设为偏移地址，AXI 总线上的最终地址 = 基址 + 偏移地址"]
    #[inline(always)]
    pub fn dma_base_addr(&mut self) -> DmaBaseAddrW<'_, TaskDmaBaseAddrSpec> {
        DmaBaseAddrW::new(self, 4)
    }
}
#[doc = "task_dma_base_addr\n\nYou can [`read`](crate::Reg::read) this register and get [`task_dma_base_addr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`task_dma_base_addr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TaskDmaBaseAddrSpec;
impl crate::RegisterSpec for TaskDmaBaseAddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`task_dma_base_addr::R`](R) reader structure"]
impl crate::Readable for TaskDmaBaseAddrSpec {}
#[doc = "`write(|w| ..)` method takes [`task_dma_base_addr::W`](W) writer structure"]
impl crate::Writable for TaskDmaBaseAddrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TASK_DMA_BASE_ADDR to value 0"]
impl crate::Resettable for TaskDmaBaseAddrSpec {}
