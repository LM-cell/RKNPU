#[doc = "Register `PAD_CON0` reader"]
pub type R = crate::R<PadCon0Spec>;
#[doc = "Register `PAD_CON0` writer"]
pub type W = crate::W<PadCon0Spec>;
#[doc = "Field `PAD_TOP` reader - 特征图顶部 pad 数"]
pub type PadTopR = crate::FieldReader;
#[doc = "Field `PAD_TOP` writer - 特征图顶部 pad 数"]
pub type PadTopW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PAD_LEFT` reader - 特征图左侧 pad 数"]
pub type PadLeftR = crate::FieldReader;
#[doc = "Field `PAD_LEFT` writer - 特征图左侧 pad 数"]
pub type PadLeftW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - 特征图顶部 pad 数"]
    #[inline(always)]
    pub fn pad_top(&self) -> PadTopR {
        PadTopR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - 特征图左侧 pad 数"]
    #[inline(always)]
    pub fn pad_left(&self) -> PadLeftR {
        PadLeftR::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 特征图顶部 pad 数"]
    #[inline(always)]
    pub fn pad_top(&mut self) -> PadTopW<'_, PadCon0Spec> {
        PadTopW::new(self, 0)
    }
    #[doc = "Bits 4:7 - 特征图左侧 pad 数"]
    #[inline(always)]
    pub fn pad_left(&mut self) -> PadLeftW<'_, PadCon0Spec> {
        PadLeftW::new(self, 4)
    }
}
#[doc = "pad_con0\n\nYou can [`read`](crate::Reg::read) this register and get [`pad_con0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pad_con0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PadCon0Spec;
impl crate::RegisterSpec for PadCon0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pad_con0::R`](R) reader structure"]
impl crate::Readable for PadCon0Spec {}
#[doc = "`write(|w| ..)` method takes [`pad_con0::W`](W) writer structure"]
impl crate::Writable for PadCon0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PAD_CON0 to value 0"]
impl crate::Resettable for PadCon0Spec {}
