#[doc = "Register `PAD_CFG` reader"]
pub type R = crate::R<PadCfgSpec>;
#[doc = "Register `PAD_CFG` writer"]
pub type W = crate::W<PadCfgSpec>;
#[doc = "Field `PAD_LEFT` reader - 反池化左侧 pad"]
pub type PadLeftR = crate::FieldReader;
#[doc = "Field `PAD_LEFT` writer - 反池化左侧 pad"]
pub type PadLeftW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PAD_TOP` reader - 反池化顶部 pad"]
pub type PadTopR = crate::FieldReader;
#[doc = "Field `PAD_TOP` writer - 反池化顶部 pad"]
pub type PadTopW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PAD_VALUE` reader - Pad 填充值"]
pub type PadValueR = crate::FieldReader<u16>;
#[doc = "Field `PAD_VALUE` writer - Pad 填充值"]
pub type PadValueW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:2 - 反池化左侧 pad"]
    #[inline(always)]
    pub fn pad_left(&self) -> PadLeftR {
        PadLeftR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - 反池化顶部 pad"]
    #[inline(always)]
    pub fn pad_top(&self) -> PadTopR {
        PadTopR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 16:31 - Pad 填充值"]
    #[inline(always)]
    pub fn pad_value(&self) -> PadValueR {
        PadValueR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:2 - 反池化左侧 pad"]
    #[inline(always)]
    pub fn pad_left(&mut self) -> PadLeftW<'_, PadCfgSpec> {
        PadLeftW::new(self, 0)
    }
    #[doc = "Bits 4:6 - 反池化顶部 pad"]
    #[inline(always)]
    pub fn pad_top(&mut self) -> PadTopW<'_, PadCfgSpec> {
        PadTopW::new(self, 4)
    }
    #[doc = "Bits 16:31 - Pad 填充值"]
    #[inline(always)]
    pub fn pad_value(&mut self) -> PadValueW<'_, PadCfgSpec> {
        PadValueW::new(self, 16)
    }
}
#[doc = "pad_cfg\n\nYou can [`read`](crate::Reg::read) this register and get [`pad_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pad_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PadCfgSpec;
impl crate::RegisterSpec for PadCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pad_cfg::R`](R) reader structure"]
impl crate::Readable for PadCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`pad_cfg::W`](W) writer structure"]
impl crate::Writable for PadCfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PAD_CFG to value 0"]
impl crate::Resettable for PadCfgSpec {}
