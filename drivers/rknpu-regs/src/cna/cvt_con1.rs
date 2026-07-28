#[doc = "Register `CVT_CON1` reader"]
pub type R = crate::R<CvtCon1Spec>;
#[doc = "Register `CVT_CON1` writer"]
pub type W = crate::W<CvtCon1Spec>;
#[doc = "Field `CVT_OFFSET0` reader - CVT 偏移 0（第 1 通道加法操作数）"]
pub type CvtOffset0R = crate::FieldReader<u16>;
#[doc = "Field `CVT_OFFSET0` writer - CVT 偏移 0（第 1 通道加法操作数）"]
pub type CvtOffset0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `CVT_SCALE0` reader - CVT 缩放 0（第 1 通道乘法操作数）"]
pub type CvtScale0R = crate::FieldReader<u16>;
#[doc = "Field `CVT_SCALE0` writer - CVT 缩放 0（第 1 通道乘法操作数）"]
pub type CvtScale0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - CVT 偏移 0（第 1 通道加法操作数）"]
    #[inline(always)]
    pub fn cvt_offset0(&self) -> CvtOffset0R {
        CvtOffset0R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - CVT 缩放 0（第 1 通道乘法操作数）"]
    #[inline(always)]
    pub fn cvt_scale0(&self) -> CvtScale0R {
        CvtScale0R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - CVT 偏移 0（第 1 通道加法操作数）"]
    #[inline(always)]
    pub fn cvt_offset0(&mut self) -> CvtOffset0W<'_, CvtCon1Spec> {
        CvtOffset0W::new(self, 0)
    }
    #[doc = "Bits 16:31 - CVT 缩放 0（第 1 通道乘法操作数）"]
    #[inline(always)]
    pub fn cvt_scale0(&mut self) -> CvtScale0W<'_, CvtCon1Spec> {
        CvtScale0W::new(self, 16)
    }
}
#[doc = "cvt_con1\n\nYou can [`read`](crate::Reg::read) this register and get [`cvt_con1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cvt_con1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CvtCon1Spec;
impl crate::RegisterSpec for CvtCon1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cvt_con1::R`](R) reader structure"]
impl crate::Readable for CvtCon1Spec {}
#[doc = "`write(|w| ..)` method takes [`cvt_con1::W`](W) writer structure"]
impl crate::Writable for CvtCon1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CVT_CON1 to value 0"]
impl crate::Resettable for CvtCon1Spec {}
