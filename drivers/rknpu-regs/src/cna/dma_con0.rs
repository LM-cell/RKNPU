#[doc = "Register `DMA_CON0` reader"]
pub type R = crate::R<DmaCon0Spec>;
#[doc = "Register `DMA_CON0` writer"]
pub type W = crate::W<DmaCon0Spec>;
#[doc = "Field `DATA_BURST_LEN` reader - 特征 DMA AXI burst 长度。编码同上"]
pub type DataBurstLenR = crate::FieldReader;
#[doc = "Field `DATA_BURST_LEN` writer - 特征 DMA AXI burst 长度。编码同上"]
pub type DataBurstLenW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `WEIGHT_BURST_LEN` reader - 权重 DMA AXI burst 长度。3：burst=4；7：burst=8；15：burst=16"]
pub type WeightBurstLenR = crate::FieldReader;
#[doc = "Field `WEIGHT_BURST_LEN` writer - 权重 DMA AXI burst 长度。3：burst=4；7：burst=8；15：burst=16"]
pub type WeightBurstLenW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `OV4K_BYPASS` reader - 超 4K burst 拆分。0：使能（将超 4K 的 burst 拆为 2 个）；1：旁路"]
pub type Ov4kBypassR = crate::BitReader;
#[doc = "Field `OV4K_BYPASS` writer - 超 4K burst 拆分。0：使能（将超 4K 的 burst 拆为 2 个）；1：旁路"]
pub type Ov4kBypassW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - 特征 DMA AXI burst 长度。编码同上"]
    #[inline(always)]
    pub fn data_burst_len(&self) -> DataBurstLenR {
        DataBurstLenR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - 权重 DMA AXI burst 长度。3：burst=4；7：burst=8；15：burst=16"]
    #[inline(always)]
    pub fn weight_burst_len(&self) -> WeightBurstLenR {
        WeightBurstLenR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 31 - 超 4K burst 拆分。0：使能（将超 4K 的 burst 拆为 2 个）；1：旁路"]
    #[inline(always)]
    pub fn ov4k_bypass(&self) -> Ov4kBypassR {
        Ov4kBypassR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - 特征 DMA AXI burst 长度。编码同上"]
    #[inline(always)]
    pub fn data_burst_len(&mut self) -> DataBurstLenW<'_, DmaCon0Spec> {
        DataBurstLenW::new(self, 0)
    }
    #[doc = "Bits 16:19 - 权重 DMA AXI burst 长度。3：burst=4；7：burst=8；15：burst=16"]
    #[inline(always)]
    pub fn weight_burst_len(&mut self) -> WeightBurstLenW<'_, DmaCon0Spec> {
        WeightBurstLenW::new(self, 16)
    }
    #[doc = "Bit 31 - 超 4K burst 拆分。0：使能（将超 4K 的 burst 拆为 2 个）；1：旁路"]
    #[inline(always)]
    pub fn ov4k_bypass(&mut self) -> Ov4kBypassW<'_, DmaCon0Spec> {
        Ov4kBypassW::new(self, 31)
    }
}
#[doc = "dma_con0\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_con0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_con0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaCon0Spec;
impl crate::RegisterSpec for DmaCon0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_con0::R`](R) reader structure"]
impl crate::Readable for DmaCon0Spec {}
#[doc = "`write(|w| ..)` method takes [`dma_con0::W`](W) writer structure"]
impl crate::Writable for DmaCon0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMA_CON0 to value 0"]
impl crate::Resettable for DmaCon0Spec {}
