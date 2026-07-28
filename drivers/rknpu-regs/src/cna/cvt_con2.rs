#[doc = "Register `CVT_CON2` reader"]
pub type R = crate::R<CvtCon2Spec>;
#[doc = "Register `CVT_CON2` writer"]
pub type W = crate::W<CvtCon2Spec>;
#[doc = "Field `CVT_OFFSET1` reader - CVT 偏移 1（第 2 通道加法操作数）"]
pub type CvtOffset1R = crate::FieldReader<u16>;
#[doc = "Field `CVT_OFFSET1` writer - CVT 偏移 1（第 2 通道加法操作数）"]
pub type CvtOffset1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `CVT_SCALE1` reader - CVT 缩放 1（第 2 通道乘法操作数）"]
pub type CvtScale1R = crate::FieldReader<u16>;
#[doc = "Field `CVT_SCALE1` writer - CVT 缩放 1（第 2 通道乘法操作数）"]
pub type CvtScale1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - CVT 偏移 1（第 2 通道加法操作数）"]
    #[inline(always)]
    pub fn cvt_offset1(&self) -> CvtOffset1R {
        CvtOffset1R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - CVT 缩放 1（第 2 通道乘法操作数）"]
    #[inline(always)]
    pub fn cvt_scale1(&self) -> CvtScale1R {
        CvtScale1R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - CVT 偏移 1（第 2 通道加法操作数）"]
    #[inline(always)]
    pub fn cvt_offset1(&mut self) -> CvtOffset1W<'_, CvtCon2Spec> {
        CvtOffset1W::new(self, 0)
    }
    #[doc = "Bits 16:31 - CVT 缩放 1（第 2 通道乘法操作数）"]
    #[inline(always)]
    pub fn cvt_scale1(&mut self) -> CvtScale1W<'_, CvtCon2Spec> {
        CvtScale1W::new(self, 16)
    }
}
#[doc = "cvt_con2\n\nYou can [`read`](crate::Reg::read) this register and get [`cvt_con2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cvt_con2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CvtCon2Spec;
impl crate::RegisterSpec for CvtCon2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cvt_con2::R`](R) reader structure"]
impl crate::Readable for CvtCon2Spec {}
#[doc = "`write(|w| ..)` method takes [`cvt_con2::W`](W) writer structure"]
impl crate::Writable for CvtCon2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CVT_CON2 to value 0"]
impl crate::Resettable for CvtCon2Spec {}
