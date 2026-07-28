#[doc = "Register `OUT_CVT_SHIFT` reader"]
pub type R = crate::R<OutCvtShiftSpec>;
#[doc = "Register `OUT_CVT_SHIFT` writer"]
pub type W = crate::W<OutCvtShiftSpec>;
#[doc = "Field `OUT_CVT_SHIFT` reader - 输出转换器移位"]
pub type OutCvtShiftR = crate::FieldReader<u16>;
#[doc = "Field `OUT_CVT_SHIFT` writer - 输出转换器移位"]
pub type OutCvtShiftW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `MINUS_EXP` reader - 输出 CVT 减指数"]
pub type MinusExpR = crate::FieldReader;
#[doc = "Field `MINUS_EXP` writer - 输出 CVT 减指数"]
pub type MinusExpW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CVT_ROUND` reader - 输出转换舍入。0：奇入偶不入；1：0.5 向上进 1"]
pub type CvtRoundR = crate::BitReader;
#[doc = "Field `CVT_ROUND` writer - 输出转换舍入。0：奇入偶不入；1：0.5 向上进 1"]
pub type CvtRoundW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CVT_TYPE` reader - 输出转换类型。0：先乘后加；1：先加后乘"]
pub type CvtTypeR = crate::BitReader;
#[doc = "Field `CVT_TYPE` writer - 输出转换类型。0：先乘后加；1：先加后乘"]
pub type CvtTypeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:11 - 输出转换器移位"]
    #[inline(always)]
    pub fn out_cvt_shift(&self) -> OutCvtShiftR {
        OutCvtShiftR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:19 - 输出 CVT 减指数"]
    #[inline(always)]
    pub fn minus_exp(&self) -> MinusExpR {
        MinusExpR::new(((self.bits >> 12) & 0xff) as u8)
    }
    #[doc = "Bit 30 - 输出转换舍入。0：奇入偶不入；1：0.5 向上进 1"]
    #[inline(always)]
    pub fn cvt_round(&self) -> CvtRoundR {
        CvtRoundR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 输出转换类型。0：先乘后加；1：先加后乘"]
    #[inline(always)]
    pub fn cvt_type(&self) -> CvtTypeR {
        CvtTypeR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - 输出转换器移位"]
    #[inline(always)]
    pub fn out_cvt_shift(&mut self) -> OutCvtShiftW<'_, OutCvtShiftSpec> {
        OutCvtShiftW::new(self, 0)
    }
    #[doc = "Bits 12:19 - 输出 CVT 减指数"]
    #[inline(always)]
    pub fn minus_exp(&mut self) -> MinusExpW<'_, OutCvtShiftSpec> {
        MinusExpW::new(self, 12)
    }
    #[doc = "Bit 30 - 输出转换舍入。0：奇入偶不入；1：0.5 向上进 1"]
    #[inline(always)]
    pub fn cvt_round(&mut self) -> CvtRoundW<'_, OutCvtShiftSpec> {
        CvtRoundW::new(self, 30)
    }
    #[doc = "Bit 31 - 输出转换类型。0：先乘后加；1：先加后乘"]
    #[inline(always)]
    pub fn cvt_type(&mut self) -> CvtTypeW<'_, OutCvtShiftSpec> {
        CvtTypeW::new(self, 31)
    }
}
#[doc = "out_cvt_shift\n\nYou can [`read`](crate::Reg::read) this register and get [`out_cvt_shift::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out_cvt_shift::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OutCvtShiftSpec;
impl crate::RegisterSpec for OutCvtShiftSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`out_cvt_shift::R`](R) reader structure"]
impl crate::Readable for OutCvtShiftSpec {}
#[doc = "`write(|w| ..)` method takes [`out_cvt_shift::W`](W) writer structure"]
impl crate::Writable for OutCvtShiftSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OUT_CVT_SHIFT to value 0"]
impl crate::Resettable for OutCvtShiftSpec {}
