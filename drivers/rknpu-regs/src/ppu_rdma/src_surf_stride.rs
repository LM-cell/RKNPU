#[doc = "Register `SRC_SURF_STRIDE` reader"]
pub type R = crate::R<SrcSurfStrideSpec>;
#[doc = "Register `SRC_SURF_STRIDE` writer"]
pub type W = crate::W<SrcSurfStrideSpec>;
#[doc = "Field `SRC_SURF_STRIDE` reader - 池化 cube shape 面积"]
pub type SrcSurfStrideR = crate::FieldReader<u32>;
#[doc = "Field `SRC_SURF_STRIDE` writer - 池化 cube shape 面积"]
pub type SrcSurfStrideW<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    #[doc = "Bits 4:31 - 池化 cube shape 面积"]
    #[inline(always)]
    pub fn src_surf_stride(&self) -> SrcSurfStrideR {
        SrcSurfStrideR::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bits 4:31 - 池化 cube shape 面积"]
    #[inline(always)]
    pub fn src_surf_stride(&mut self) -> SrcSurfStrideW<'_, SrcSurfStrideSpec> {
        SrcSurfStrideW::new(self, 4)
    }
}
#[doc = "src_surf_stride\n\nYou can [`read`](crate::Reg::read) this register and get [`src_surf_stride::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`src_surf_stride::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrcSurfStrideSpec;
impl crate::RegisterSpec for SrcSurfStrideSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`src_surf_stride::R`](R) reader structure"]
impl crate::Readable for SrcSurfStrideSpec {}
#[doc = "`write(|w| ..)` method takes [`src_surf_stride::W`](W) writer structure"]
impl crate::Writable for SrcSurfStrideSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SRC_SURF_STRIDE to value 0"]
impl crate::Resettable for SrcSurfStrideSpec {}
