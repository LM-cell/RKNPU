#[doc = "Register `FEATURE_MODE_CFG` reader"]
pub type R = crate::R<FeatureModeCfgSpec>;
#[doc = "Register `FEATURE_MODE_CFG` writer"]
pub type W = crate::W<FeatureModeCfgSpec>;
#[doc = "Field `FLYING_MODE` reader - Flying 模式。0：主数据来自卷积输出；1：主数据来自 MRDMA"]
pub type FlyingModeR = crate::BitReader;
#[doc = "Field `FLYING_MODE` writer - Flying 模式。0：主数据来自卷积输出；1：主数据来自 MRDMA"]
pub type FlyingModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTPUT_MODE` reader - 输出目标。\\[0\\]：输出到 PPU；\\[1\\]：输出到外部"]
pub type OutputModeR = crate::FieldReader;
#[doc = "Field `OUTPUT_MODE` writer - 输出目标。\\[0\\]：输出到 PPU；\\[1\\]：输出到外部"]
pub type OutputModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CONV_MODE` reader - 卷积模式。0：普通卷积；3：Depthwise"]
pub type ConvModeR = crate::FieldReader;
#[doc = "Field `CONV_MODE` writer - 卷积模式。0：普通卷积；3：Depthwise"]
pub type ConvModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BURST_LEN` reader - Burst 长度。3：Burst4；7：Burst8；15：Burst16"]
pub type BurstLenR = crate::FieldReader;
#[doc = "Field `BURST_LEN` writer - Burst 长度。3：Burst4；7：Burst8；15：Burst16"]
pub type BurstLenW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SURF_LEN` reader - 非对齐模式下存储的 8 字节数"]
pub type SurfLenR = crate::FieldReader<u16>;
#[doc = "Field `SURF_LEN` writer - 非对齐模式下存储的 8 字节数"]
pub type SurfLenW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `NONALIGN` reader - 非对齐模式使能（输出数据流与输入相同时可用）"]
pub type NonalignR = crate::BitReader;
#[doc = "Field `NONALIGN` writer - 非对齐模式使能（输出数据流与输入相同时可用）"]
pub type NonalignW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RGP_TYPE` reader - 重组类型。0：全部 128bit；1：4bit；2：8bit；3：16bit；4：32bit；5：64bit"]
pub type RgpTypeR = crate::FieldReader;
#[doc = "Field `RGP_TYPE` writer - 重组类型。0：全部 128bit；1：4bit；2：8bit；3：16bit；4：32bit；5：64bit"]
pub type RgpTypeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TP_EN` reader - 转置使能"]
pub type TpEnR = crate::BitReader;
#[doc = "Field `TP_EN` writer - 转置使能"]
pub type TpEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMB_USE` reader - 组合使用，同 DPU_RDMA `comb_use\\[0\\]`"]
pub type CombUseR = crate::BitReader;
#[doc = "Field `COMB_USE` writer - 组合使用，同 DPU_RDMA `comb_use\\[0\\]`"]
pub type CombUseW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Flying 模式。0：主数据来自卷积输出；1：主数据来自 MRDMA"]
    #[inline(always)]
    pub fn flying_mode(&self) -> FlyingModeR {
        FlyingModeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - 输出目标。\\[0\\]：输出到 PPU；\\[1\\]：输出到外部"]
    #[inline(always)]
    pub fn output_mode(&self) -> OutputModeR {
        OutputModeR::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bits 3:4 - 卷积模式。0：普通卷积；3：Depthwise"]
    #[inline(always)]
    pub fn conv_mode(&self) -> ConvModeR {
        ConvModeR::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bits 5:8 - Burst 长度。3：Burst4；7：Burst8；15：Burst16"]
    #[inline(always)]
    pub fn burst_len(&self) -> BurstLenR {
        BurstLenR::new(((self.bits >> 5) & 0x0f) as u8)
    }
    #[doc = "Bits 9:24 - 非对齐模式下存储的 8 字节数"]
    #[inline(always)]
    pub fn surf_len(&self) -> SurfLenR {
        SurfLenR::new(((self.bits >> 9) & 0xffff) as u16)
    }
    #[doc = "Bit 25 - 非对齐模式使能（输出数据流与输入相同时可用）"]
    #[inline(always)]
    pub fn nonalign(&self) -> NonalignR {
        NonalignR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bits 26:29 - 重组类型。0：全部 128bit；1：4bit；2：8bit；3：16bit；4：32bit；5：64bit"]
    #[inline(always)]
    pub fn rgp_type(&self) -> RgpTypeR {
        RgpTypeR::new(((self.bits >> 26) & 0x0f) as u8)
    }
    #[doc = "Bit 30 - 转置使能"]
    #[inline(always)]
    pub fn tp_en(&self) -> TpEnR {
        TpEnR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 组合使用，同 DPU_RDMA `comb_use\\[0\\]`"]
    #[inline(always)]
    pub fn comb_use(&self) -> CombUseR {
        CombUseR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Flying 模式。0：主数据来自卷积输出；1：主数据来自 MRDMA"]
    #[inline(always)]
    pub fn flying_mode(&mut self) -> FlyingModeW<'_, FeatureModeCfgSpec> {
        FlyingModeW::new(self, 0)
    }
    #[doc = "Bits 1:2 - 输出目标。\\[0\\]：输出到 PPU；\\[1\\]：输出到外部"]
    #[inline(always)]
    pub fn output_mode(&mut self) -> OutputModeW<'_, FeatureModeCfgSpec> {
        OutputModeW::new(self, 1)
    }
    #[doc = "Bits 3:4 - 卷积模式。0：普通卷积；3：Depthwise"]
    #[inline(always)]
    pub fn conv_mode(&mut self) -> ConvModeW<'_, FeatureModeCfgSpec> {
        ConvModeW::new(self, 3)
    }
    #[doc = "Bits 5:8 - Burst 长度。3：Burst4；7：Burst8；15：Burst16"]
    #[inline(always)]
    pub fn burst_len(&mut self) -> BurstLenW<'_, FeatureModeCfgSpec> {
        BurstLenW::new(self, 5)
    }
    #[doc = "Bits 9:24 - 非对齐模式下存储的 8 字节数"]
    #[inline(always)]
    pub fn surf_len(&mut self) -> SurfLenW<'_, FeatureModeCfgSpec> {
        SurfLenW::new(self, 9)
    }
    #[doc = "Bit 25 - 非对齐模式使能（输出数据流与输入相同时可用）"]
    #[inline(always)]
    pub fn nonalign(&mut self) -> NonalignW<'_, FeatureModeCfgSpec> {
        NonalignW::new(self, 25)
    }
    #[doc = "Bits 26:29 - 重组类型。0：全部 128bit；1：4bit；2：8bit；3：16bit；4：32bit；5：64bit"]
    #[inline(always)]
    pub fn rgp_type(&mut self) -> RgpTypeW<'_, FeatureModeCfgSpec> {
        RgpTypeW::new(self, 26)
    }
    #[doc = "Bit 30 - 转置使能"]
    #[inline(always)]
    pub fn tp_en(&mut self) -> TpEnW<'_, FeatureModeCfgSpec> {
        TpEnW::new(self, 30)
    }
    #[doc = "Bit 31 - 组合使用，同 DPU_RDMA `comb_use\\[0\\]`"]
    #[inline(always)]
    pub fn comb_use(&mut self) -> CombUseW<'_, FeatureModeCfgSpec> {
        CombUseW::new(self, 31)
    }
}
#[doc = "feature_mode_cfg\n\nYou can [`read`](crate::Reg::read) this register and get [`feature_mode_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`feature_mode_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FeatureModeCfgSpec;
impl crate::RegisterSpec for FeatureModeCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`feature_mode_cfg::R`](R) reader structure"]
impl crate::Readable for FeatureModeCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`feature_mode_cfg::W`](W) writer structure"]
impl crate::Writable for FeatureModeCfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FEATURE_MODE_CFG to value 0"]
impl crate::Resettable for FeatureModeCfgSpec {}
