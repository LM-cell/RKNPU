#[doc = "Register `CVT_CON4` reader"]
pub type R = crate::R<CvtCon4Spec>;
#[doc = "Register `CVT_CON4` writer"]
pub type W = crate::W<CvtCon4Spec>;
#[doc = "Field `CVT_OFFSET3` reader - CVT 偏移 3（第 4 通道加法操作数）"]
pub type CvtOffset3R = crate::FieldReader<u16>;
#[doc = "Field `CVT_OFFSET3` writer - CVT 偏移 3（第 4 通道加法操作数）"]
pub type CvtOffset3W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `CVT_SCALE3` reader - CVT 缩放 3（第 4 通道乘法操作数）"]
pub type CvtScale3R = crate::FieldReader<u16>;
#[doc = "Field `CVT_SCALE3` writer - CVT 缩放 3（第 4 通道乘法操作数）"]
pub type CvtScale3W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - CVT 偏移 3（第 4 通道加法操作数）"]
    #[inline(always)]
    pub fn cvt_offset3(&self) -> CvtOffset3R {
        CvtOffset3R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - CVT 缩放 3（第 4 通道乘法操作数）"]
    #[inline(always)]
    pub fn cvt_scale3(&self) -> CvtScale3R {
        CvtScale3R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - CVT 偏移 3（第 4 通道加法操作数）"]
    #[inline(always)]
    pub fn cvt_offset3(&mut self) -> CvtOffset3W<'_, CvtCon4Spec> {
        CvtOffset3W::new(self, 0)
    }
    #[doc = "Bits 16:31 - CVT 缩放 3（第 4 通道乘法操作数）"]
    #[inline(always)]
    pub fn cvt_scale3(&mut self) -> CvtScale3W<'_, CvtCon4Spec> {
        CvtScale3W::new(self, 16)
    }
}
#[doc = "cvt_con4\n\nYou can [`read`](crate::Reg::read) this register and get [`cvt_con4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cvt_con4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CvtCon4Spec;
impl crate::RegisterSpec for CvtCon4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cvt_con4::R`](R) reader structure"]
impl crate::Readable for CvtCon4Spec {}
#[doc = "`write(|w| ..)` method takes [`cvt_con4::W`](W) writer structure"]
impl crate::Writable for CvtCon4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CVT_CON4 to value 0"]
impl crate::Resettable for CvtCon4Spec {}
