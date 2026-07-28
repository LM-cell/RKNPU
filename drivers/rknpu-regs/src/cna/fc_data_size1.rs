#[doc = "Register `FC_DATA_SIZE1` reader"]
pub type R = crate::R<FcDataSize1Spec>;
#[doc = "Register `FC_DATA_SIZE1` writer"]
pub type W = crate::W<FcDataSize1Spec>;
#[doc = "Field `DMA_CHANNEL` reader - AXI DMA 特征输入通道数"]
pub type DmaChannelR = crate::FieldReader<u16>;
#[doc = "Field `DMA_CHANNEL` writer - AXI DMA 特征输入通道数"]
pub type DmaChannelW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - AXI DMA 特征输入通道数"]
    #[inline(always)]
    pub fn dma_channel(&self) -> DmaChannelR {
        DmaChannelR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - AXI DMA 特征输入通道数"]
    #[inline(always)]
    pub fn dma_channel(&mut self) -> DmaChannelW<'_, FcDataSize1Spec> {
        DmaChannelW::new(self, 0)
    }
}
#[doc = "fc_data_size1\n\nYou can [`read`](crate::Reg::read) this register and get [`fc_data_size1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fc_data_size1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FcDataSize1Spec;
impl crate::RegisterSpec for FcDataSize1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fc_data_size1::R`](R) reader structure"]
impl crate::Readable for FcDataSize1Spec {}
#[doc = "`write(|w| ..)` method takes [`fc_data_size1::W`](W) writer structure"]
impl crate::Writable for FcDataSize1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FC_DATA_SIZE1 to value 0"]
impl crate::Resettable for FcDataSize1Spec {}
