#[doc = "Register `DST_SURF_STRIDE` reader"]
pub type R = crate::R<DstSurfStrideSpec>;
#[doc = "Register `DST_SURF_STRIDE` writer"]
pub type W = crate::W<DstSurfStrideSpec>;
#[doc = "Field `DST_SURF_STRIDE` reader - 输出 shape 的 surface 步长"]
pub type DstSurfStrideR = crate::FieldReader<u32>;
#[doc = "Field `DST_SURF_STRIDE` writer - 输出 shape 的 surface 步长"]
pub type DstSurfStrideW<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    #[doc = "Bits 4:31 - 输出 shape 的 surface 步长"]
    #[inline(always)]
    pub fn dst_surf_stride(&self) -> DstSurfStrideR {
        DstSurfStrideR::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bits 4:31 - 输出 shape 的 surface 步长"]
    #[inline(always)]
    pub fn dst_surf_stride(&mut self) -> DstSurfStrideW<'_, DstSurfStrideSpec> {
        DstSurfStrideW::new(self, 4)
    }
}
#[doc = "dst_surf_stride\n\nYou can [`read`](crate::Reg::read) this register and get [`dst_surf_stride::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dst_surf_stride::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DstSurfStrideSpec;
impl crate::RegisterSpec for DstSurfStrideSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dst_surf_stride::R`](R) reader structure"]
impl crate::Readable for DstSurfStrideSpec {}
#[doc = "`write(|w| ..)` method takes [`dst_surf_stride::W`](W) writer structure"]
impl crate::Writable for DstSurfStrideSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DST_SURF_STRIDE to value 0"]
impl crate::Resettable for DstSurfStrideSpec {}
