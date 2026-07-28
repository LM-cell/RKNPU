#[doc = "Register `CONV_CON1` reader"]
pub type R = crate::R<ConvCon1Spec>;
#[doc = "Register `CONV_CON1` writer"]
pub type W = crate::W<ConvCon1Spec>;
#[doc = "Field `CONV_MODE` reader - 卷积模式。0：直接卷积；3：深度可分离卷积（Depthwise）"]
pub type ConvModeR = crate::FieldReader;
#[doc = "Field `CONV_MODE` writer - 卷积模式。0：直接卷积；3：深度可分离卷积（Depthwise）"]
pub type ConvModeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `IN_PRECISION` reader - 输入精度。编码同 `proc_precision`"]
pub type InPrecisionR = crate::FieldReader;
#[doc = "Field `IN_PRECISION` writer - 输入精度。编码同 `proc_precision`"]
pub type InPrecisionW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PROC_PRECISION` reader - 处理精度。0：int8；1：int16；2：fp16；3：bf16；6：int4；7：tf32"]
pub type ProcPrecisionR = crate::FieldReader;
#[doc = "Field `PROC_PRECISION` writer - 处理精度。0：int8；1：int16；2：fp16；3：bf16；6：int4；7：tf32"]
pub type ProcPrecisionW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ARGB_IN` reader - 非对齐通道层控制。8：1 通道输入；9：2 通道；10：3 通道；11：4 通道"]
pub type ArgbInR = crate::FieldReader;
#[doc = "Field `ARGB_IN` writer - 非对齐通道层控制。8：1 通道输入；9：2 通道；10：3 通道；11：4 通道"]
pub type ArgbInW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DECONV` reader - 反卷积使能。0：禁用；1：使能"]
pub type DeconvR = crate::BitReader;
#[doc = "Field `DECONV` writer - 反卷积使能。0：禁用；1：使能"]
pub type DeconvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GROUP_LINE_OFF` reader - 组行取数关闭。0：使能组行取数；1：禁用（仅影响取数效率）"]
pub type GroupLineOffR = crate::BitReader;
#[doc = "Field `GROUP_LINE_OFF` writer - 组行取数关闭。0：使能组行取数；1：禁用（仅影响取数效率）"]
pub type GroupLineOffW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NONALIGN_DMA` reader - CNA DMA 非对齐模式。0：禁用；1：使能（ARGB 模式下请开启，使 DMA 连续取特征数据）"]
pub type NonalignDmaR = crate::BitReader;
#[doc = "Field `NONALIGN_DMA` writer - CNA DMA 非对齐模式。0：禁用；1：使能（ARGB 模式下请开启，使 DMA 连续取特征数据）"]
pub type NonalignDmaW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - 卷积模式。0：直接卷积；3：深度可分离卷积（Depthwise）"]
    #[inline(always)]
    pub fn conv_mode(&self) -> ConvModeR {
        ConvModeR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:6 - 输入精度。编码同 `proc_precision`"]
    #[inline(always)]
    pub fn in_precision(&self) -> InPrecisionR {
        InPrecisionR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 7:9 - 处理精度。0：int8；1：int16；2：fp16；3：bf16；6：int4；7：tf32"]
    #[inline(always)]
    pub fn proc_precision(&self) -> ProcPrecisionR {
        ProcPrecisionR::new(((self.bits >> 7) & 7) as u8)
    }
    #[doc = "Bits 12:15 - 非对齐通道层控制。8：1 通道输入；9：2 通道；10：3 通道；11：4 通道"]
    #[inline(always)]
    pub fn argb_in(&self) -> ArgbInR {
        ArgbInR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bit 16 - 反卷积使能。0：禁用；1：使能"]
    #[inline(always)]
    pub fn deconv(&self) -> DeconvR {
        DeconvR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 29 - 组行取数关闭。0：使能组行取数；1：禁用（仅影响取数效率）"]
    #[inline(always)]
    pub fn group_line_off(&self) -> GroupLineOffR {
        GroupLineOffR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - CNA DMA 非对齐模式。0：禁用；1：使能（ARGB 模式下请开启，使 DMA 连续取特征数据）"]
    #[inline(always)]
    pub fn nonalign_dma(&self) -> NonalignDmaR {
        NonalignDmaR::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - 卷积模式。0：直接卷积；3：深度可分离卷积（Depthwise）"]
    #[inline(always)]
    pub fn conv_mode(&mut self) -> ConvModeW<'_, ConvCon1Spec> {
        ConvModeW::new(self, 0)
    }
    #[doc = "Bits 4:6 - 输入精度。编码同 `proc_precision`"]
    #[inline(always)]
    pub fn in_precision(&mut self) -> InPrecisionW<'_, ConvCon1Spec> {
        InPrecisionW::new(self, 4)
    }
    #[doc = "Bits 7:9 - 处理精度。0：int8；1：int16；2：fp16；3：bf16；6：int4；7：tf32"]
    #[inline(always)]
    pub fn proc_precision(&mut self) -> ProcPrecisionW<'_, ConvCon1Spec> {
        ProcPrecisionW::new(self, 7)
    }
    #[doc = "Bits 12:15 - 非对齐通道层控制。8：1 通道输入；9：2 通道；10：3 通道；11：4 通道"]
    #[inline(always)]
    pub fn argb_in(&mut self) -> ArgbInW<'_, ConvCon1Spec> {
        ArgbInW::new(self, 12)
    }
    #[doc = "Bit 16 - 反卷积使能。0：禁用；1：使能"]
    #[inline(always)]
    pub fn deconv(&mut self) -> DeconvW<'_, ConvCon1Spec> {
        DeconvW::new(self, 16)
    }
    #[doc = "Bit 29 - 组行取数关闭。0：使能组行取数；1：禁用（仅影响取数效率）"]
    #[inline(always)]
    pub fn group_line_off(&mut self) -> GroupLineOffW<'_, ConvCon1Spec> {
        GroupLineOffW::new(self, 29)
    }
    #[doc = "Bit 30 - CNA DMA 非对齐模式。0：禁用；1：使能（ARGB 模式下请开启，使 DMA 连续取特征数据）"]
    #[inline(always)]
    pub fn nonalign_dma(&mut self) -> NonalignDmaW<'_, ConvCon1Spec> {
        NonalignDmaW::new(self, 30)
    }
}
#[doc = "conv_con1\n\nYou can [`read`](crate::Reg::read) this register and get [`conv_con1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`conv_con1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ConvCon1Spec;
impl crate::RegisterSpec for ConvCon1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`conv_con1::R`](R) reader structure"]
impl crate::Readable for ConvCon1Spec {}
#[doc = "`write(|w| ..)` method takes [`conv_con1::W`](W) writer structure"]
impl crate::Writable for ConvCon1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CONV_CON1 to value 0"]
impl crate::Resettable for ConvCon1Spec {}
