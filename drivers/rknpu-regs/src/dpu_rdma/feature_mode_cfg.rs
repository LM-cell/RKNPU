#[doc = "Register `FEATURE_MODE_CFG` reader"]
pub type R = crate::R<FeatureModeCfgSpec>;
#[doc = "Register `FEATURE_MODE_CFG` writer"]
pub type W = crate::W<FeatureModeCfgSpec>;
#[doc = "Field `FLYING_MODE` reader - Flying 模式。0：主数据来自卷积输出；1：主数据来自 MRDMA"]
pub type FlyingModeR = crate::BitReader;
#[doc = "Field `FLYING_MODE` writer - Flying 模式。0：主数据来自卷积输出；1：主数据来自 MRDMA"]
pub type FlyingModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CONV_MODE` reader - 卷积模式。0：DC；3：Depthwise"]
pub type ConvModeR = crate::FieldReader;
#[doc = "Field `CONV_MODE` writer - 卷积模式。0：DC；3：Depthwise"]
pub type ConvModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MRDMA_FP16TOFP32_EN` reader - 使能 DPU 输入 fp16→fp32 转换"]
pub type MrdmaFp16tofp32EnR = crate::BitReader;
#[doc = "Field `MRDMA_FP16TOFP32_EN` writer - 使能 DPU 输入 fp16→fp32 转换"]
pub type MrdmaFp16tofp32EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MRDMA_DISABLE` reader - 禁用 MRDMA。0：不禁用；1：禁用"]
pub type MrdmaDisableR = crate::BitReader;
#[doc = "Field `MRDMA_DISABLE` writer - 禁用 MRDMA。0：不禁用；1：禁用"]
pub type MrdmaDisableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PROC_PRECISION` reader - 处理精度。编码同 `in_precision`"]
pub type ProcPrecisionR = crate::FieldReader;
#[doc = "Field `PROC_PRECISION` writer - 处理精度。编码同 `in_precision`"]
pub type ProcPrecisionW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `COMB_USE` reader - 组合使用。\\[0\\]：MRDMA 和 ERDMA 读同一数据；\\[1\\]：数据送 MRDMA；\\[2\\]：数据送 ERDMA"]
pub type CombUseR = crate::FieldReader;
#[doc = "Field `COMB_USE` writer - 组合使用。\\[0\\]：MRDMA 和 ERDMA 读同一数据；\\[1\\]：数据送 MRDMA；\\[2\\]：数据送 ERDMA"]
pub type CombUseW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `BURST_LEN` reader - Burst 长度。3：Burst4；7：Burst8；15：Burst16"]
pub type BurstLenR = crate::FieldReader;
#[doc = "Field `BURST_LEN` writer - Burst 长度。3：Burst4；7：Burst8；15：Burst16"]
pub type BurstLenW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `IN_PRECISION` reader - 输入数据精度。0：int8；1：int16；2：fp16；3：bf16；4：int32；5：fp32；6：int4"]
pub type InPrecisionR = crate::FieldReader;
#[doc = "Field `IN_PRECISION` writer - 输入数据精度。0：int8；1：int16；2：fp16；3：bf16；4：int32；5：fp32；6：int4"]
pub type InPrecisionW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 0 - Flying 模式。0：主数据来自卷积输出；1：主数据来自 MRDMA"]
    #[inline(always)]
    pub fn flying_mode(&self) -> FlyingModeR {
        FlyingModeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - 卷积模式。0：DC；3：Depthwise"]
    #[inline(always)]
    pub fn conv_mode(&self) -> ConvModeR {
        ConvModeR::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3 - 使能 DPU 输入 fp16→fp32 转换"]
    #[inline(always)]
    pub fn mrdma_fp16tofp32_en(&self) -> MrdmaFp16tofp32EnR {
        MrdmaFp16tofp32EnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 禁用 MRDMA。0：不禁用；1：禁用"]
    #[inline(always)]
    pub fn mrdma_disable(&self) -> MrdmaDisableR {
        MrdmaDisableR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:7 - 处理精度。编码同 `in_precision`"]
    #[inline(always)]
    pub fn proc_precision(&self) -> ProcPrecisionR {
        ProcPrecisionR::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bits 8:10 - 组合使用。\\[0\\]：MRDMA 和 ERDMA 读同一数据；\\[1\\]：数据送 MRDMA；\\[2\\]：数据送 ERDMA"]
    #[inline(always)]
    pub fn comb_use(&self) -> CombUseR {
        CombUseR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:14 - Burst 长度。3：Burst4；7：Burst8；15：Burst16"]
    #[inline(always)]
    pub fn burst_len(&self) -> BurstLenR {
        BurstLenR::new(((self.bits >> 11) & 0x0f) as u8)
    }
    #[doc = "Bits 15:17 - 输入数据精度。0：int8；1：int16；2：fp16；3：bf16；4：int32；5：fp32；6：int4"]
    #[inline(always)]
    pub fn in_precision(&self) -> InPrecisionR {
        InPrecisionR::new(((self.bits >> 15) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Flying 模式。0：主数据来自卷积输出；1：主数据来自 MRDMA"]
    #[inline(always)]
    pub fn flying_mode(&mut self) -> FlyingModeW<'_, FeatureModeCfgSpec> {
        FlyingModeW::new(self, 0)
    }
    #[doc = "Bits 1:2 - 卷积模式。0：DC；3：Depthwise"]
    #[inline(always)]
    pub fn conv_mode(&mut self) -> ConvModeW<'_, FeatureModeCfgSpec> {
        ConvModeW::new(self, 1)
    }
    #[doc = "Bit 3 - 使能 DPU 输入 fp16→fp32 转换"]
    #[inline(always)]
    pub fn mrdma_fp16tofp32_en(&mut self) -> MrdmaFp16tofp32EnW<'_, FeatureModeCfgSpec> {
        MrdmaFp16tofp32EnW::new(self, 3)
    }
    #[doc = "Bit 4 - 禁用 MRDMA。0：不禁用；1：禁用"]
    #[inline(always)]
    pub fn mrdma_disable(&mut self) -> MrdmaDisableW<'_, FeatureModeCfgSpec> {
        MrdmaDisableW::new(self, 4)
    }
    #[doc = "Bits 5:7 - 处理精度。编码同 `in_precision`"]
    #[inline(always)]
    pub fn proc_precision(&mut self) -> ProcPrecisionW<'_, FeatureModeCfgSpec> {
        ProcPrecisionW::new(self, 5)
    }
    #[doc = "Bits 8:10 - 组合使用。\\[0\\]：MRDMA 和 ERDMA 读同一数据；\\[1\\]：数据送 MRDMA；\\[2\\]：数据送 ERDMA"]
    #[inline(always)]
    pub fn comb_use(&mut self) -> CombUseW<'_, FeatureModeCfgSpec> {
        CombUseW::new(self, 8)
    }
    #[doc = "Bits 11:14 - Burst 长度。3：Burst4；7：Burst8；15：Burst16"]
    #[inline(always)]
    pub fn burst_len(&mut self) -> BurstLenW<'_, FeatureModeCfgSpec> {
        BurstLenW::new(self, 11)
    }
    #[doc = "Bits 15:17 - 输入数据精度。0：int8；1：int16；2：fp16；3：bf16；4：int32；5：fp32；6：int4"]
    #[inline(always)]
    pub fn in_precision(&mut self) -> InPrecisionW<'_, FeatureModeCfgSpec> {
        InPrecisionW::new(self, 15)
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
