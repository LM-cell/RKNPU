#[doc = "Register `PADDING_VALUE_2_CFG` reader"]
pub type R = crate::R<PaddingValue2CfgSpec>;
#[doc = "Register `PADDING_VALUE_2_CFG` writer"]
pub type W = crate::W<PaddingValue2CfgSpec>;
#[doc = "Field `PAD_VALUE_1` reader - pad_value×1 \\[34:32\\]"]
pub type PadValue1R = crate::FieldReader;
#[doc = "Field `PAD_VALUE_1` writer - pad_value×1 \\[34:32\\]"]
pub type PadValue1W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - pad_value×1 \\[34:32\\]"]
    #[inline(always)]
    pub fn pad_value_1(&self) -> PadValue1R {
        PadValue1R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - pad_value×1 \\[34:32\\]"]
    #[inline(always)]
    pub fn pad_value_1(&mut self) -> PadValue1W<'_, PaddingValue2CfgSpec> {
        PadValue1W::new(self, 0)
    }
}
#[doc = "padding_value_2_cfg\n\nYou can [`read`](crate::Reg::read) this register and get [`padding_value_2_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`padding_value_2_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PaddingValue2CfgSpec;
impl crate::RegisterSpec for PaddingValue2CfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`padding_value_2_cfg::R`](R) reader structure"]
impl crate::Readable for PaddingValue2CfgSpec {}
#[doc = "`write(|w| ..)` method takes [`padding_value_2_cfg::W`](W) writer structure"]
impl crate::Writable for PaddingValue2CfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PADDING_VALUE_2_CFG to value 0"]
impl crate::Resettable for PaddingValue2CfgSpec {}
