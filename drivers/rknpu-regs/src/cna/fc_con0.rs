#[doc = "Register `FC_CON0` reader"]
pub type R = crate::R<FcCon0Spec>;
#[doc = "Register `FC_CON0` writer"]
pub type W = crate::W<FcCon0Spec>;
#[doc = "Field `FC_SKIP_EN` reader - FC 零跳过使能。0：禁用；1：使能。当某像素特征数据为 0 时，跳过对应权重的取数"]
pub type FcSkipEnR = crate::BitReader;
#[doc = "Field `FC_SKIP_EN` writer - FC 零跳过使能。0：禁用；1：使能。当某像素特征数据为 0 时，跳过对应权重的取数"]
pub type FcSkipEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FC_SKIP_DATA` reader - FC 零跳过数据值（通常设为 0）"]
pub type FcSkipDataR = crate::FieldReader<u16>;
#[doc = "Field `FC_SKIP_DATA` writer - FC 零跳过数据值（通常设为 0）"]
pub type FcSkipDataW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - FC 零跳过使能。0：禁用；1：使能。当某像素特征数据为 0 时，跳过对应权重的取数"]
    #[inline(always)]
    pub fn fc_skip_en(&self) -> FcSkipEnR {
        FcSkipEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 16:31 - FC 零跳过数据值（通常设为 0）"]
    #[inline(always)]
    pub fn fc_skip_data(&self) -> FcSkipDataR {
        FcSkipDataR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - FC 零跳过使能。0：禁用；1：使能。当某像素特征数据为 0 时，跳过对应权重的取数"]
    #[inline(always)]
    pub fn fc_skip_en(&mut self) -> FcSkipEnW<'_, FcCon0Spec> {
        FcSkipEnW::new(self, 0)
    }
    #[doc = "Bits 16:31 - FC 零跳过数据值（通常设为 0）"]
    #[inline(always)]
    pub fn fc_skip_data(&mut self) -> FcSkipDataW<'_, FcCon0Spec> {
        FcSkipDataW::new(self, 16)
    }
}
#[doc = "fc_con0\n\nYou can [`read`](crate::Reg::read) this register and get [`fc_con0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fc_con0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FcCon0Spec;
impl crate::RegisterSpec for FcCon0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fc_con0::R`](R) reader structure"]
impl crate::Readable for FcCon0Spec {}
#[doc = "`write(|w| ..)` method takes [`fc_con0::W`](W) writer structure"]
impl crate::Writable for FcCon0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FC_CON0 to value 0"]
impl crate::Resettable for FcCon0Spec {}
