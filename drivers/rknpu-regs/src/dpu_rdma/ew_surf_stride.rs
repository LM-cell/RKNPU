#[doc = "Register `EW_SURF_STRIDE` reader"]
pub type R = crate::R<EwSurfStrideSpec>;
#[doc = "Register `EW_SURF_STRIDE` writer"]
pub type W = crate::W<EwSurfStrideSpec>;
#[doc = "Field `EW_SURF_STRIDE` reader - EW 特征图 surface 步长。若 `erdma_data_mode` 为按通道模式，需设为 1"]
pub type EwSurfStrideR = crate::FieldReader<u32>;
#[doc = "Field `EW_SURF_STRIDE` writer - EW 特征图 surface 步长。若 `erdma_data_mode` 为按通道模式，需设为 1"]
pub type EwSurfStrideW<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    #[doc = "Bits 4:31 - EW 特征图 surface 步长。若 `erdma_data_mode` 为按通道模式，需设为 1"]
    #[inline(always)]
    pub fn ew_surf_stride(&self) -> EwSurfStrideR {
        EwSurfStrideR::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bits 4:31 - EW 特征图 surface 步长。若 `erdma_data_mode` 为按通道模式，需设为 1"]
    #[inline(always)]
    pub fn ew_surf_stride(&mut self) -> EwSurfStrideW<'_, EwSurfStrideSpec> {
        EwSurfStrideW::new(self, 4)
    }
}
#[doc = "ew_surf_stride\n\nYou can [`read`](crate::Reg::read) this register and get [`ew_surf_stride::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ew_surf_stride::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EwSurfStrideSpec;
impl crate::RegisterSpec for EwSurfStrideSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ew_surf_stride::R`](R) reader structure"]
impl crate::Readable for EwSurfStrideSpec {}
#[doc = "`write(|w| ..)` method takes [`ew_surf_stride::W`](W) writer structure"]
impl crate::Writable for EwSurfStrideSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EW_SURF_STRIDE to value 0"]
impl crate::Resettable for EwSurfStrideSpec {}
