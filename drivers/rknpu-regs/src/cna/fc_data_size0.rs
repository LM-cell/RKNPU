#[doc = "Register `FC_DATA_SIZE0` reader"]
pub type R = crate::R<FcDataSize0Spec>;
#[doc = "Register `FC_DATA_SIZE0` writer"]
pub type W = crate::W<FcDataSize0Spec>;
#[doc = "Field `DMA_HEIGHT` reader - AXI DMA 特征输入高度"]
pub type DmaHeightR = crate::FieldReader<u16>;
#[doc = "Field `DMA_HEIGHT` writer - AXI DMA 特征输入高度"]
pub type DmaHeightW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `DMA_WIDTH` reader - AXI DMA 特征输入宽度"]
pub type DmaWidthR = crate::FieldReader<u16>;
#[doc = "Field `DMA_WIDTH` writer - AXI DMA 特征输入宽度"]
pub type DmaWidthW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    #[doc = "Bits 0:10 - AXI DMA 特征输入高度"]
    #[inline(always)]
    pub fn dma_height(&self) -> DmaHeightR {
        DmaHeightR::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 16:29 - AXI DMA 特征输入宽度"]
    #[inline(always)]
    pub fn dma_width(&self) -> DmaWidthR {
        DmaWidthR::new(((self.bits >> 16) & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - AXI DMA 特征输入高度"]
    #[inline(always)]
    pub fn dma_height(&mut self) -> DmaHeightW<'_, FcDataSize0Spec> {
        DmaHeightW::new(self, 0)
    }
    #[doc = "Bits 16:29 - AXI DMA 特征输入宽度"]
    #[inline(always)]
    pub fn dma_width(&mut self) -> DmaWidthW<'_, FcDataSize0Spec> {
        DmaWidthW::new(self, 16)
    }
}
#[doc = "fc_data_size0\n\nYou can [`read`](crate::Reg::read) this register and get [`fc_data_size0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fc_data_size0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FcDataSize0Spec;
impl crate::RegisterSpec for FcDataSize0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fc_data_size0::R`](R) reader structure"]
impl crate::Readable for FcDataSize0Spec {}
#[doc = "`write(|w| ..)` method takes [`fc_data_size0::W`](W) writer structure"]
impl crate::Writable for FcDataSize0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FC_DATA_SIZE0 to value 0"]
impl crate::Resettable for FcDataSize0Spec {}
