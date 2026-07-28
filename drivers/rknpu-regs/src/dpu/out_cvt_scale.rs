#[doc = "Register `OUT_CVT_SCALE` reader"]
pub type R = crate::R<OutCvtScaleSpec>;
#[doc = "Register `OUT_CVT_SCALE` writer"]
pub type W = crate::W<OutCvtScaleSpec>;
#[doc = "Field `OUT_CVT_SCALE` reader - 输出转换器缩放"]
pub type OutCvtScaleR = crate::FieldReader<u16>;
#[doc = "Field `OUT_CVT_SCALE` writer - 输出转换器缩放"]
pub type OutCvtScaleW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `FP32TOFP16_EN` reader - 使能输出 fp32→fp16 转换"]
pub type Fp32tofp16EnR = crate::BitReader;
#[doc = "Field `FP32TOFP16_EN` writer - 使能输出 fp32→fp16 转换"]
pub type Fp32tofp16EnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - 输出转换器缩放"]
    #[inline(always)]
    pub fn out_cvt_scale(&self) -> OutCvtScaleR {
        OutCvtScaleR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - 使能输出 fp32→fp16 转换"]
    #[inline(always)]
    pub fn fp32tofp16_en(&self) -> Fp32tofp16EnR {
        Fp32tofp16EnR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - 输出转换器缩放"]
    #[inline(always)]
    pub fn out_cvt_scale(&mut self) -> OutCvtScaleW<'_, OutCvtScaleSpec> {
        OutCvtScaleW::new(self, 0)
    }
    #[doc = "Bit 16 - 使能输出 fp32→fp16 转换"]
    #[inline(always)]
    pub fn fp32tofp16_en(&mut self) -> Fp32tofp16EnW<'_, OutCvtScaleSpec> {
        Fp32tofp16EnW::new(self, 16)
    }
}
#[doc = "out_cvt_scale\n\nYou can [`read`](crate::Reg::read) this register and get [`out_cvt_scale::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out_cvt_scale::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OutCvtScaleSpec;
impl crate::RegisterSpec for OutCvtScaleSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`out_cvt_scale::R`](R) reader structure"]
impl crate::Readable for OutCvtScaleSpec {}
#[doc = "`write(|w| ..)` method takes [`out_cvt_scale::W`](W) writer structure"]
impl crate::Writable for OutCvtScaleSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OUT_CVT_SCALE to value 0"]
impl crate::Resettable for OutCvtScaleSpec {}
