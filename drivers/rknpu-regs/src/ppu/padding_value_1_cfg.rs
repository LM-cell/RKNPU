#[doc = "Register `PADDING_VALUE_1_CFG` reader"]
pub type R = crate::R<PaddingValue1CfgSpec>;
#[doc = "Register `PADDING_VALUE_1_CFG` writer"]
pub type W = crate::W<PaddingValue1CfgSpec>;
#[doc = "Field `PAD_VALUE_0` reader - pad_value×1 \\[31:0\\]"]
pub type PadValue0R = crate::FieldReader<u32>;
#[doc = "Field `PAD_VALUE_0` writer - pad_value×1 \\[31:0\\]"]
pub type PadValue0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - pad_value×1 \\[31:0\\]"]
    #[inline(always)]
    pub fn pad_value_0(&self) -> PadValue0R {
        PadValue0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - pad_value×1 \\[31:0\\]"]
    #[inline(always)]
    pub fn pad_value_0(&mut self) -> PadValue0W<'_, PaddingValue1CfgSpec> {
        PadValue0W::new(self, 0)
    }
}
#[doc = "padding_value_1_cfg\n\nYou can [`read`](crate::Reg::read) this register and get [`padding_value_1_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`padding_value_1_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PaddingValue1CfgSpec;
impl crate::RegisterSpec for PaddingValue1CfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`padding_value_1_cfg::R`](R) reader structure"]
impl crate::Readable for PaddingValue1CfgSpec {}
#[doc = "`write(|w| ..)` method takes [`padding_value_1_cfg::W`](W) writer structure"]
impl crate::Writable for PaddingValue1CfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PADDING_VALUE_1_CFG to value 0"]
impl crate::Resettable for PaddingValue1CfgSpec {}
