#[doc = "Register `WDMA_SIZE_1` reader"]
pub type R = crate::R<WdmaSize1Spec>;
#[doc = "Register `WDMA_SIZE_1` writer"]
pub type W = crate::W<WdmaSize1Spec>;
#[doc = "Field `WIDTH_WDMA` reader - WDMA 宽度"]
pub type WidthWdmaR = crate::FieldReader<u16>;
#[doc = "Field `WIDTH_WDMA` writer - WDMA 宽度"]
pub type WidthWdmaW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
#[doc = "Field `HEIGHT_WDMA` reader - WDMA 高度"]
pub type HeightWdmaR = crate::FieldReader<u16>;
#[doc = "Field `HEIGHT_WDMA` writer - WDMA 高度"]
pub type HeightWdmaW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    #[doc = "Bits 0:12 - WDMA 宽度"]
    #[inline(always)]
    pub fn width_wdma(&self) -> WidthWdmaR {
        WidthWdmaR::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bits 16:28 - WDMA 高度"]
    #[inline(always)]
    pub fn height_wdma(&self) -> HeightWdmaR {
        HeightWdmaR::new(((self.bits >> 16) & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:12 - WDMA 宽度"]
    #[inline(always)]
    pub fn width_wdma(&mut self) -> WidthWdmaW<'_, WdmaSize1Spec> {
        WidthWdmaW::new(self, 0)
    }
    #[doc = "Bits 16:28 - WDMA 高度"]
    #[inline(always)]
    pub fn height_wdma(&mut self) -> HeightWdmaW<'_, WdmaSize1Spec> {
        HeightWdmaW::new(self, 16)
    }
}
#[doc = "wdma_size_1\n\nYou can [`read`](crate::Reg::read) this register and get [`wdma_size_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdma_size_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WdmaSize1Spec;
impl crate::RegisterSpec for WdmaSize1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wdma_size_1::R`](R) reader structure"]
impl crate::Readable for WdmaSize1Spec {}
#[doc = "`write(|w| ..)` method takes [`wdma_size_1::W`](W) writer structure"]
impl crate::Writable for WdmaSize1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WDMA_SIZE_1 to value 0"]
impl crate::Resettable for WdmaSize1Spec {}
