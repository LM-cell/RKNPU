#[doc = "Register `POOLING_PADDING_CFG` reader"]
pub type R = crate::R<PoolingPaddingCfgSpec>;
#[doc = "Register `POOLING_PADDING_CFG` writer"]
pub type W = crate::W<PoolingPaddingCfgSpec>;
#[doc = "Field `PAD_LEFT` reader - 左侧 pad"]
pub type PadLeftR = crate::FieldReader;
#[doc = "Field `PAD_LEFT` writer - 左侧 pad"]
pub type PadLeftW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PAD_TOP` reader - 顶部 pad"]
pub type PadTopR = crate::FieldReader;
#[doc = "Field `PAD_TOP` writer - 顶部 pad"]
pub type PadTopW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PAD_RIGHT` reader - 右侧 pad"]
pub type PadRightR = crate::FieldReader;
#[doc = "Field `PAD_RIGHT` writer - 右侧 pad"]
pub type PadRightW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PAD_BOTTOM` reader - 底部 pad"]
pub type PadBottomR = crate::FieldReader;
#[doc = "Field `PAD_BOTTOM` writer - 底部 pad"]
pub type PadBottomW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - 左侧 pad"]
    #[inline(always)]
    pub fn pad_left(&self) -> PadLeftR {
        PadLeftR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - 顶部 pad"]
    #[inline(always)]
    pub fn pad_top(&self) -> PadTopR {
        PadTopR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - 右侧 pad"]
    #[inline(always)]
    pub fn pad_right(&self) -> PadRightR {
        PadRightR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:14 - 底部 pad"]
    #[inline(always)]
    pub fn pad_bottom(&self) -> PadBottomR {
        PadBottomR::new(((self.bits >> 12) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - 左侧 pad"]
    #[inline(always)]
    pub fn pad_left(&mut self) -> PadLeftW<'_, PoolingPaddingCfgSpec> {
        PadLeftW::new(self, 0)
    }
    #[doc = "Bits 4:6 - 顶部 pad"]
    #[inline(always)]
    pub fn pad_top(&mut self) -> PadTopW<'_, PoolingPaddingCfgSpec> {
        PadTopW::new(self, 4)
    }
    #[doc = "Bits 8:10 - 右侧 pad"]
    #[inline(always)]
    pub fn pad_right(&mut self) -> PadRightW<'_, PoolingPaddingCfgSpec> {
        PadRightW::new(self, 8)
    }
    #[doc = "Bits 12:14 - 底部 pad"]
    #[inline(always)]
    pub fn pad_bottom(&mut self) -> PadBottomW<'_, PoolingPaddingCfgSpec> {
        PadBottomW::new(self, 12)
    }
}
#[doc = "pooling_padding_cfg\n\nYou can [`read`](crate::Reg::read) this register and get [`pooling_padding_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pooling_padding_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PoolingPaddingCfgSpec;
impl crate::RegisterSpec for PoolingPaddingCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pooling_padding_cfg::R`](R) reader structure"]
impl crate::Readable for PoolingPaddingCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`pooling_padding_cfg::W`](W) writer structure"]
impl crate::Writable for PoolingPaddingCfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets POOLING_PADDING_CFG to value 0"]
impl crate::Resettable for PoolingPaddingCfgSpec {}
