#[doc = "Register `CVT_CON0` reader"]
pub type R = crate::R<CvtCon0Spec>;
#[doc = "Register `CVT_CON0` writer"]
pub type W = crate::W<CvtCon0Spec>;
#[doc = "Field `CVT_BYPASS` reader - 旁路输入转换。0：使能 CVT；1：禁用 CVT"]
pub type CvtBypassR = crate::BitReader;
#[doc = "Field `CVT_BYPASS` writer - 旁路输入转换。0：使能 CVT；1：禁用 CVT"]
pub type CvtBypassW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CVT_TYPE` reader - 转换运算顺序。0：先乘后加；1：先加后乘"]
pub type CvtTypeR = crate::BitReader;
#[doc = "Field `CVT_TYPE` writer - 转换运算顺序。0：先乘后加；1：先加后乘"]
pub type CvtTypeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ROUND_TYPE` reader - 舍入类型。0：奇入偶不入；1：0.5 向上进 1"]
pub type RoundTypeR = crate::BitReader;
#[doc = "Field `ROUND_TYPE` writer - 舍入类型。0：奇入偶不入；1：0.5 向上进 1"]
pub type RoundTypeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATA_SIGN` reader - 特征数据符号。0：无符号；1：有符号"]
pub type DataSignR = crate::BitReader;
#[doc = "Field `DATA_SIGN` writer - 特征数据符号。0：无符号；1：有符号"]
pub type DataSignW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CVT_TRUNCATE_0` reader - CVT 截断值 0"]
pub type CvtTruncate0R = crate::FieldReader;
#[doc = "Field `CVT_TRUNCATE_0` writer - CVT 截断值 0"]
pub type CvtTruncate0W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `CVT_TRUNCATE_1` reader - CVT 截断值 1"]
pub type CvtTruncate1R = crate::FieldReader;
#[doc = "Field `CVT_TRUNCATE_1` writer - CVT 截断值 1"]
pub type CvtTruncate1W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `CVT_TRUNCATE_2` reader - CVT 截断值 2"]
pub type CvtTruncate2R = crate::FieldReader;
#[doc = "Field `CVT_TRUNCATE_2` writer - CVT 截断值 2"]
pub type CvtTruncate2W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `CVT_TRUNCATE_3` reader - CVT 截断值 3"]
pub type CvtTruncate3R = crate::FieldReader;
#[doc = "Field `CVT_TRUNCATE_3` writer - CVT 截断值 3"]
pub type CvtTruncate3W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bit 0 - 旁路输入转换。0：使能 CVT；1：禁用 CVT"]
    #[inline(always)]
    pub fn cvt_bypass(&self) -> CvtBypassR {
        CvtBypassR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 转换运算顺序。0：先乘后加；1：先加后乘"]
    #[inline(always)]
    pub fn cvt_type(&self) -> CvtTypeR {
        CvtTypeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 舍入类型。0：奇入偶不入；1：0.5 向上进 1"]
    #[inline(always)]
    pub fn round_type(&self) -> RoundTypeR {
        RoundTypeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 特征数据符号。0：无符号；1：有符号"]
    #[inline(always)]
    pub fn data_sign(&self) -> DataSignR {
        DataSignR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:9 - CVT 截断值 0"]
    #[inline(always)]
    pub fn cvt_truncate_0(&self) -> CvtTruncate0R {
        CvtTruncate0R::new(((self.bits >> 4) & 0x3f) as u8)
    }
    #[doc = "Bits 10:15 - CVT 截断值 1"]
    #[inline(always)]
    pub fn cvt_truncate_1(&self) -> CvtTruncate1R {
        CvtTruncate1R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - CVT 截断值 2"]
    #[inline(always)]
    pub fn cvt_truncate_2(&self) -> CvtTruncate2R {
        CvtTruncate2R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 22:27 - CVT 截断值 3"]
    #[inline(always)]
    pub fn cvt_truncate_3(&self) -> CvtTruncate3R {
        CvtTruncate3R::new(((self.bits >> 22) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 旁路输入转换。0：使能 CVT；1：禁用 CVT"]
    #[inline(always)]
    pub fn cvt_bypass(&mut self) -> CvtBypassW<'_, CvtCon0Spec> {
        CvtBypassW::new(self, 0)
    }
    #[doc = "Bit 1 - 转换运算顺序。0：先乘后加；1：先加后乘"]
    #[inline(always)]
    pub fn cvt_type(&mut self) -> CvtTypeW<'_, CvtCon0Spec> {
        CvtTypeW::new(self, 1)
    }
    #[doc = "Bit 2 - 舍入类型。0：奇入偶不入；1：0.5 向上进 1"]
    #[inline(always)]
    pub fn round_type(&mut self) -> RoundTypeW<'_, CvtCon0Spec> {
        RoundTypeW::new(self, 2)
    }
    #[doc = "Bit 3 - 特征数据符号。0：无符号；1：有符号"]
    #[inline(always)]
    pub fn data_sign(&mut self) -> DataSignW<'_, CvtCon0Spec> {
        DataSignW::new(self, 3)
    }
    #[doc = "Bits 4:9 - CVT 截断值 0"]
    #[inline(always)]
    pub fn cvt_truncate_0(&mut self) -> CvtTruncate0W<'_, CvtCon0Spec> {
        CvtTruncate0W::new(self, 4)
    }
    #[doc = "Bits 10:15 - CVT 截断值 1"]
    #[inline(always)]
    pub fn cvt_truncate_1(&mut self) -> CvtTruncate1W<'_, CvtCon0Spec> {
        CvtTruncate1W::new(self, 10)
    }
    #[doc = "Bits 16:21 - CVT 截断值 2"]
    #[inline(always)]
    pub fn cvt_truncate_2(&mut self) -> CvtTruncate2W<'_, CvtCon0Spec> {
        CvtTruncate2W::new(self, 16)
    }
    #[doc = "Bits 22:27 - CVT 截断值 3"]
    #[inline(always)]
    pub fn cvt_truncate_3(&mut self) -> CvtTruncate3W<'_, CvtCon0Spec> {
        CvtTruncate3W::new(self, 22)
    }
}
#[doc = "cvt_con0\n\nYou can [`read`](crate::Reg::read) this register and get [`cvt_con0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cvt_con0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CvtCon0Spec;
impl crate::RegisterSpec for CvtCon0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cvt_con0::R`](R) reader structure"]
impl crate::Readable for CvtCon0Spec {}
#[doc = "`write(|w| ..)` method takes [`cvt_con0::W`](W) writer structure"]
impl crate::Writable for CvtCon0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CVT_CON0 to value 0"]
impl crate::Resettable for CvtCon0Spec {}
