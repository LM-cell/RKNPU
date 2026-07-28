#[doc = "Register `SRC_LINE_STRIDE` reader"]
pub type R = crate::R<SrcLineStrideSpec>;
#[doc = "Register `SRC_LINE_STRIDE` writer"]
pub type W = crate::W<SrcLineStrideSpec>;
#[doc = "Field `SRC_LINE_STRIDE` reader - 池化 cube shape 宽度"]
pub type SrcLineStrideR = crate::FieldReader<u32>;
#[doc = "Field `SRC_LINE_STRIDE` writer - 池化 cube shape 宽度"]
pub type SrcLineStrideW<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    #[doc = "Bits 4:31 - 池化 cube shape 宽度"]
    #[inline(always)]
    pub fn src_line_stride(&self) -> SrcLineStrideR {
        SrcLineStrideR::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bits 4:31 - 池化 cube shape 宽度"]
    #[inline(always)]
    pub fn src_line_stride(&mut self) -> SrcLineStrideW<'_, SrcLineStrideSpec> {
        SrcLineStrideW::new(self, 4)
    }
}
#[doc = "src_line_stride\n\nYou can [`read`](crate::Reg::read) this register and get [`src_line_stride::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`src_line_stride::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrcLineStrideSpec;
impl crate::RegisterSpec for SrcLineStrideSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`src_line_stride::R`](R) reader structure"]
impl crate::Readable for SrcLineStrideSpec {}
#[doc = "`write(|w| ..)` method takes [`src_line_stride::W`](W) writer structure"]
impl crate::Writable for SrcLineStrideSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SRC_LINE_STRIDE to value 0"]
impl crate::Resettable for SrcLineStrideSpec {}
