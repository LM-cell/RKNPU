#[doc = "Register `PAD_CON1` reader"]
pub type R = crate::R<PadCon1Spec>;
#[doc = "Register `PAD_CON1` writer"]
pub type W = crate::W<PadCon1Spec>;
#[doc = "Field `PAD_VALUE` reader - Pad 填充值"]
pub type PadValueR = crate::FieldReader<u32>;
#[doc = "Field `PAD_VALUE` writer - Pad 填充值"]
pub type PadValueW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Pad 填充值"]
    #[inline(always)]
    pub fn pad_value(&self) -> PadValueR {
        PadValueR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Pad 填充值"]
    #[inline(always)]
    pub fn pad_value(&mut self) -> PadValueW<'_, PadCon1Spec> {
        PadValueW::new(self, 0)
    }
}
#[doc = "pad_con1\n\nYou can [`read`](crate::Reg::read) this register and get [`pad_con1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pad_con1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PadCon1Spec;
impl crate::RegisterSpec for PadCon1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pad_con1::R`](R) reader structure"]
impl crate::Readable for PadCon1Spec {}
#[doc = "`write(|w| ..)` method takes [`pad_con1::W`](W) writer structure"]
impl crate::Writable for PadCon1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PAD_CON1 to value 0"]
impl crate::Resettable for PadCon1Spec {}
