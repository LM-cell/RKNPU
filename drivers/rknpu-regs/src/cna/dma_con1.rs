#[doc = "Register `DMA_CON1` reader"]
pub type R = crate::R<DmaCon1Spec>;
#[doc = "Register `DMA_CON1` writer"]
pub type W = crate::W<DmaCon1Spec>;
#[doc = "Field `LINE_STRIDE` reader - 行步长。含虚拟框（Virtual box）的特征宽度"]
pub type LineStrideR = crate::FieldReader<u32>;
#[doc = "Field `LINE_STRIDE` writer - 行步长。含虚拟框（Virtual box）的特征宽度"]
pub type LineStrideW<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    #[doc = "Bits 0:27 - 行步长。含虚拟框（Virtual box）的特征宽度"]
    #[inline(always)]
    pub fn line_stride(&self) -> LineStrideR {
        LineStrideR::new(self.bits & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:27 - 行步长。含虚拟框（Virtual box）的特征宽度"]
    #[inline(always)]
    pub fn line_stride(&mut self) -> LineStrideW<'_, DmaCon1Spec> {
        LineStrideW::new(self, 0)
    }
}
#[doc = "dma_con1\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_con1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_con1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaCon1Spec;
impl crate::RegisterSpec for DmaCon1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_con1::R`](R) reader structure"]
impl crate::Readable for DmaCon1Spec {}
#[doc = "`write(|w| ..)` method takes [`dma_con1::W`](W) writer structure"]
impl crate::Writable for DmaCon1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMA_CON1 to value 0"]
impl crate::Resettable for DmaCon1Spec {}
