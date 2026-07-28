#[doc = "Register `CVT_CON3` reader"]
pub type R = crate::R<CvtCon3Spec>;
#[doc = "Register `CVT_CON3` writer"]
pub type W = crate::W<CvtCon3Spec>;
#[doc = "Field `CVT_OFFSET2` reader - CVT 偏移 2（第 3 通道加法操作数）"]
pub type CvtOffset2R = crate::FieldReader<u16>;
#[doc = "Field `CVT_OFFSET2` writer - CVT 偏移 2（第 3 通道加法操作数）"]
pub type CvtOffset2W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `CVT_SCALE2` reader - CVT 缩放 2（第 3 通道乘法操作数）"]
pub type CvtScale2R = crate::FieldReader<u16>;
#[doc = "Field `CVT_SCALE2` writer - CVT 缩放 2（第 3 通道乘法操作数）"]
pub type CvtScale2W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - CVT 偏移 2（第 3 通道加法操作数）"]
    #[inline(always)]
    pub fn cvt_offset2(&self) -> CvtOffset2R {
        CvtOffset2R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - CVT 缩放 2（第 3 通道乘法操作数）"]
    #[inline(always)]
    pub fn cvt_scale2(&self) -> CvtScale2R {
        CvtScale2R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - CVT 偏移 2（第 3 通道加法操作数）"]
    #[inline(always)]
    pub fn cvt_offset2(&mut self) -> CvtOffset2W<'_, CvtCon3Spec> {
        CvtOffset2W::new(self, 0)
    }
    #[doc = "Bits 16:31 - CVT 缩放 2（第 3 通道乘法操作数）"]
    #[inline(always)]
    pub fn cvt_scale2(&mut self) -> CvtScale2W<'_, CvtCon3Spec> {
        CvtScale2W::new(self, 16)
    }
}
#[doc = "cvt_con3\n\nYou can [`read`](crate::Reg::read) this register and get [`cvt_con3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cvt_con3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CvtCon3Spec;
impl crate::RegisterSpec for CvtCon3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cvt_con3::R`](R) reader structure"]
impl crate::Readable for CvtCon3Spec {}
#[doc = "`write(|w| ..)` method takes [`cvt_con3::W`](W) writer structure"]
impl crate::Writable for CvtCon3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CVT_CON3 to value 0"]
impl crate::Resettable for CvtCon3Spec {}
