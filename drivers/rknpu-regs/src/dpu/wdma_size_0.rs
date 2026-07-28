#[doc = "Register `WDMA_SIZE_0` reader"]
pub type R = crate::R<WdmaSize0Spec>;
#[doc = "Register `WDMA_SIZE_0` writer"]
pub type W = crate::W<WdmaSize0Spec>;
#[doc = "Field `CHANNEL_WDMA` reader - WDMA 通道数"]
pub type ChannelWdmaR = crate::FieldReader<u16>;
#[doc = "Field `CHANNEL_WDMA` writer - WDMA 通道数"]
pub type ChannelWdmaW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
#[doc = "Field `SIZE_C_WDMA` reader - WDMA 的 size_c"]
pub type SizeCWdmaR = crate::FieldReader<u16>;
#[doc = "Field `SIZE_C_WDMA` writer - WDMA 的 size_c"]
pub type SizeCWdmaW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `TP_PRECISION` reader - 转置精度。0：8bit；1：16bit"]
pub type TpPrecisionR = crate::BitReader;
#[doc = "Field `TP_PRECISION` writer - 转置精度。0：8bit；1：16bit"]
pub type TpPrecisionW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:12 - WDMA 通道数"]
    #[inline(always)]
    pub fn channel_wdma(&self) -> ChannelWdmaR {
        ChannelWdmaR::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bits 16:26 - WDMA 的 size_c"]
    #[inline(always)]
    pub fn size_c_wdma(&self) -> SizeCWdmaR {
        SizeCWdmaR::new(((self.bits >> 16) & 0x07ff) as u16)
    }
    #[doc = "Bit 27 - 转置精度。0：8bit；1：16bit"]
    #[inline(always)]
    pub fn tp_precision(&self) -> TpPrecisionR {
        TpPrecisionR::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:12 - WDMA 通道数"]
    #[inline(always)]
    pub fn channel_wdma(&mut self) -> ChannelWdmaW<'_, WdmaSize0Spec> {
        ChannelWdmaW::new(self, 0)
    }
    #[doc = "Bits 16:26 - WDMA 的 size_c"]
    #[inline(always)]
    pub fn size_c_wdma(&mut self) -> SizeCWdmaW<'_, WdmaSize0Spec> {
        SizeCWdmaW::new(self, 16)
    }
    #[doc = "Bit 27 - 转置精度。0：8bit；1：16bit"]
    #[inline(always)]
    pub fn tp_precision(&mut self) -> TpPrecisionW<'_, WdmaSize0Spec> {
        TpPrecisionW::new(self, 27)
    }
}
#[doc = "wdma_size_0\n\nYou can [`read`](crate::Reg::read) this register and get [`wdma_size_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdma_size_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WdmaSize0Spec;
impl crate::RegisterSpec for WdmaSize0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wdma_size_0::R`](R) reader structure"]
impl crate::Readable for WdmaSize0Spec {}
#[doc = "`write(|w| ..)` method takes [`wdma_size_0::W`](W) writer structure"]
impl crate::Writable for WdmaSize0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WDMA_SIZE_0 to value 0"]
impl crate::Resettable for WdmaSize0Spec {}
