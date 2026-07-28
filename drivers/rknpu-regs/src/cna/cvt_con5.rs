#[doc = "Register `CVT_CON5` reader"]
pub type R = crate::R<CvtCon5Spec>;
#[doc = "Register `CVT_CON5` writer"]
pub type W = crate::W<CvtCon5Spec>;
#[doc = "Field `PER_CHANNEL_CVT_EN` reader - 按通道使能 CVT 功能。int4 共 32 通道（128 bit），int8 共 16 通道"]
pub type PerChannelCvtEnR = crate::FieldReader<u32>;
#[doc = "Field `PER_CHANNEL_CVT_EN` writer - 按通道使能 CVT 功能。int4 共 32 通道（128 bit），int8 共 16 通道"]
pub type PerChannelCvtEnW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 按通道使能 CVT 功能。int4 共 32 通道（128 bit），int8 共 16 通道"]
    #[inline(always)]
    pub fn per_channel_cvt_en(&self) -> PerChannelCvtEnR {
        PerChannelCvtEnR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 按通道使能 CVT 功能。int4 共 32 通道（128 bit），int8 共 16 通道"]
    #[inline(always)]
    pub fn per_channel_cvt_en(&mut self) -> PerChannelCvtEnW<'_, CvtCon5Spec> {
        PerChannelCvtEnW::new(self, 0)
    }
}
#[doc = "cvt_con5\n\nYou can [`read`](crate::Reg::read) this register and get [`cvt_con5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cvt_con5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CvtCon5Spec;
impl crate::RegisterSpec for CvtCon5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cvt_con5::R`](R) reader structure"]
impl crate::Readable for CvtCon5Spec {}
#[doc = "`write(|w| ..)` method takes [`cvt_con5::W`](W) writer structure"]
impl crate::Writable for CvtCon5Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CVT_CON5 to value 0"]
impl crate::Resettable for CvtCon5Spec {}
