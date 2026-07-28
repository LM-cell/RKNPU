#[doc = "Register `DMA_CON2` reader"]
pub type R = crate::R<DmaCon2Spec>;
#[doc = "Register `DMA_CON2` writer"]
pub type W = crate::W<DmaCon2Spec>;
#[doc = "Field `SURF_STRIDE` reader - Surface 步长。特征图实际 surface 面积"]
pub type SurfStrideR = crate::FieldReader<u32>;
#[doc = "Field `SURF_STRIDE` writer - Surface 步长。特征图实际 surface 面积"]
pub type SurfStrideW<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    #[doc = "Bits 0:27 - Surface 步长。特征图实际 surface 面积"]
    #[inline(always)]
    pub fn surf_stride(&self) -> SurfStrideR {
        SurfStrideR::new(self.bits & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:27 - Surface 步长。特征图实际 surface 面积"]
    #[inline(always)]
    pub fn surf_stride(&mut self) -> SurfStrideW<'_, DmaCon2Spec> {
        SurfStrideW::new(self, 0)
    }
}
#[doc = "dma_con2\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_con2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_con2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaCon2Spec;
impl crate::RegisterSpec for DmaCon2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_con2::R`](R) reader structure"]
impl crate::Readable for DmaCon2Spec {}
#[doc = "`write(|w| ..)` method takes [`dma_con2::W`](W) writer structure"]
impl crate::Writable for DmaCon2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMA_CON2 to value 0"]
impl crate::Resettable for DmaCon2Spec {}
