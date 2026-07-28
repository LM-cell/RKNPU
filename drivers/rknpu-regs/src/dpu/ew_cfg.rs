#[doc = "Register `EW_CFG` reader"]
pub type R = crate::R<EwCfgSpec>;
#[doc = "Register `EW_CFG` writer"]
pub type W = crate::W<EwCfgSpec>;
#[doc = "Field `EW_BYPASS` reader - 旁路整个 EW CORE"]
pub type EwBypassR = crate::BitReader;
#[doc = "Field `EW_BYPASS` writer - 旁路整个 EW CORE"]
pub type EwBypassW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EW_OP_BYPASS` reader - 旁路 EW ALU 和 MUL"]
pub type EwOpBypassR = crate::BitReader;
#[doc = "Field `EW_OP_BYPASS` writer - 旁路 EW ALU 和 MUL"]
pub type EwOpBypassW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EW_OP_TYPE` reader - 运算类型。0：ALU；1：MUL"]
pub type EwOpTypeR = crate::BitReader;
#[doc = "Field `EW_OP_TYPE` writer - 运算类型。0：ALU；1：MUL"]
pub type EwOpTypeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EW_MUL_PRELU` reader - MUL PRELU 使能"]
pub type EwMulPreluR = crate::BitReader;
#[doc = "Field `EW_MUL_PRELU` writer - MUL PRELU 使能"]
pub type EwMulPreluW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EW_OP_SRC` reader - 操作数来源。0：寄存器；1：外部"]
pub type EwOpSrcR = crate::BitReader;
#[doc = "Field `EW_OP_SRC` writer - 操作数来源。0：寄存器；1：外部"]
pub type EwOpSrcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EW_LUT_BYPASS` reader - 旁路 LUT"]
pub type EwLutBypassR = crate::BitReader;
#[doc = "Field `EW_LUT_BYPASS` writer - 旁路 LUT"]
pub type EwLutBypassW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EW_OP_CVT_BYPASS` reader - 旁路 EW 输入转换器"]
pub type EwOpCvtBypassR = crate::BitReader;
#[doc = "Field `EW_OP_CVT_BYPASS` writer - 旁路 EW 输入转换器"]
pub type EwOpCvtBypassW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EW_RELU_BYPASS` reader - 旁路 EW RELU"]
pub type EwReluBypassR = crate::BitReader;
#[doc = "Field `EW_RELU_BYPASS` writer - 旁路 EW RELU"]
pub type EwReluBypassW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EW_RELUX_EN` reader - RELUX 使能"]
pub type EwReluxEnR = crate::BitReader;
#[doc = "Field `EW_RELUX_EN` writer - RELUX 使能"]
pub type EwReluxEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EW_ALU_ALGO` reader - EW ALU 运算。0：Max；1：Min；2：Add；3：Div；4：Minus；5：Abs；6：Neg；7：Floor；8：Ceil"]
pub type EwAluAlgoR = crate::FieldReader;
#[doc = "Field `EW_ALU_ALGO` writer - EW ALU 运算。0：Max；1：Min；2：Add；3：Div；4：Minus；5：Abs；6：Neg；7：Floor；8：Ceil"]
pub type EwAluAlgoW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EW_BINARY_EN` reader - MinMax 二值使能"]
pub type EwBinaryEnR = crate::BitReader;
#[doc = "Field `EW_BINARY_EN` writer - MinMax 二值使能"]
pub type EwBinaryEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EW_EQUAL_EN` reader - MinMax 相等使能"]
pub type EwEqualEnR = crate::BitReader;
#[doc = "Field `EW_EQUAL_EN` writer - MinMax 相等使能"]
pub type EwEqualEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EDATA_SIZE` reader - ERDMA cube 数据大小。0：4bit；1：8bit；2：16bit；3：32bit"]
pub type EdataSizeR = crate::FieldReader;
#[doc = "Field `EDATA_SIZE` writer - ERDMA cube 数据大小。0：4bit；1：8bit；2：16bit；3：32bit"]
pub type EdataSizeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EW_DATA_MODE` reader - ERDMA 数据模式"]
pub type EwDataModeR = crate::FieldReader;
#[doc = "Field `EW_DATA_MODE` writer - ERDMA 数据模式"]
pub type EwDataModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EW_CVT_ROUND` reader - EW 输入转换舍入。0：奇入偶不入；1：0.5 向上进 1"]
pub type EwCvtRoundR = crate::BitReader;
#[doc = "Field `EW_CVT_ROUND` writer - EW 输入转换舍入。0：奇入偶不入；1：0.5 向上进 1"]
pub type EwCvtRoundW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EW_CVT_TYPE` reader - EW 输入转换类型。0：先乘后加；1：先加后乘"]
pub type EwCvtTypeR = crate::BitReader;
#[doc = "Field `EW_CVT_TYPE` writer - EW 输入转换类型。0：先乘后加；1：先加后乘"]
pub type EwCvtTypeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 旁路整个 EW CORE"]
    #[inline(always)]
    pub fn ew_bypass(&self) -> EwBypassR {
        EwBypassR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 旁路 EW ALU 和 MUL"]
    #[inline(always)]
    pub fn ew_op_bypass(&self) -> EwOpBypassR {
        EwOpBypassR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 运算类型。0：ALU；1：MUL"]
    #[inline(always)]
    pub fn ew_op_type(&self) -> EwOpTypeR {
        EwOpTypeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - MUL PRELU 使能"]
    #[inline(always)]
    pub fn ew_mul_prelu(&self) -> EwMulPreluR {
        EwMulPreluR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 操作数来源。0：寄存器；1：外部"]
    #[inline(always)]
    pub fn ew_op_src(&self) -> EwOpSrcR {
        EwOpSrcR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 旁路 LUT"]
    #[inline(always)]
    pub fn ew_lut_bypass(&self) -> EwLutBypassR {
        EwLutBypassR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 旁路 EW 输入转换器"]
    #[inline(always)]
    pub fn ew_op_cvt_bypass(&self) -> EwOpCvtBypassR {
        EwOpCvtBypassR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 旁路 EW RELU"]
    #[inline(always)]
    pub fn ew_relu_bypass(&self) -> EwReluBypassR {
        EwReluBypassR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - RELUX 使能"]
    #[inline(always)]
    pub fn ew_relux_en(&self) -> EwReluxEnR {
        EwReluxEnR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 16:19 - EW ALU 运算。0：Max；1：Min；2：Add；3：Div；4：Minus；5：Abs；6：Neg；7：Floor；8：Ceil"]
    #[inline(always)]
    pub fn ew_alu_algo(&self) -> EwAluAlgoR {
        EwAluAlgoR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 20 - MinMax 二值使能"]
    #[inline(always)]
    pub fn ew_binary_en(&self) -> EwBinaryEnR {
        EwBinaryEnR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - MinMax 相等使能"]
    #[inline(always)]
    pub fn ew_equal_en(&self) -> EwEqualEnR {
        EwEqualEnR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 22:23 - ERDMA cube 数据大小。0：4bit；1：8bit；2：16bit；3：32bit"]
    #[inline(always)]
    pub fn edata_size(&self) -> EdataSizeR {
        EdataSizeR::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 28:29 - ERDMA 数据模式"]
    #[inline(always)]
    pub fn ew_data_mode(&self) -> EwDataModeR {
        EwDataModeR::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bit 30 - EW 输入转换舍入。0：奇入偶不入；1：0.5 向上进 1"]
    #[inline(always)]
    pub fn ew_cvt_round(&self) -> EwCvtRoundR {
        EwCvtRoundR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - EW 输入转换类型。0：先乘后加；1：先加后乘"]
    #[inline(always)]
    pub fn ew_cvt_type(&self) -> EwCvtTypeR {
        EwCvtTypeR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 旁路整个 EW CORE"]
    #[inline(always)]
    pub fn ew_bypass(&mut self) -> EwBypassW<'_, EwCfgSpec> {
        EwBypassW::new(self, 0)
    }
    #[doc = "Bit 1 - 旁路 EW ALU 和 MUL"]
    #[inline(always)]
    pub fn ew_op_bypass(&mut self) -> EwOpBypassW<'_, EwCfgSpec> {
        EwOpBypassW::new(self, 1)
    }
    #[doc = "Bit 2 - 运算类型。0：ALU；1：MUL"]
    #[inline(always)]
    pub fn ew_op_type(&mut self) -> EwOpTypeW<'_, EwCfgSpec> {
        EwOpTypeW::new(self, 2)
    }
    #[doc = "Bit 5 - MUL PRELU 使能"]
    #[inline(always)]
    pub fn ew_mul_prelu(&mut self) -> EwMulPreluW<'_, EwCfgSpec> {
        EwMulPreluW::new(self, 5)
    }
    #[doc = "Bit 6 - 操作数来源。0：寄存器；1：外部"]
    #[inline(always)]
    pub fn ew_op_src(&mut self) -> EwOpSrcW<'_, EwCfgSpec> {
        EwOpSrcW::new(self, 6)
    }
    #[doc = "Bit 7 - 旁路 LUT"]
    #[inline(always)]
    pub fn ew_lut_bypass(&mut self) -> EwLutBypassW<'_, EwCfgSpec> {
        EwLutBypassW::new(self, 7)
    }
    #[doc = "Bit 8 - 旁路 EW 输入转换器"]
    #[inline(always)]
    pub fn ew_op_cvt_bypass(&mut self) -> EwOpCvtBypassW<'_, EwCfgSpec> {
        EwOpCvtBypassW::new(self, 8)
    }
    #[doc = "Bit 9 - 旁路 EW RELU"]
    #[inline(always)]
    pub fn ew_relu_bypass(&mut self) -> EwReluBypassW<'_, EwCfgSpec> {
        EwReluBypassW::new(self, 9)
    }
    #[doc = "Bit 10 - RELUX 使能"]
    #[inline(always)]
    pub fn ew_relux_en(&mut self) -> EwReluxEnW<'_, EwCfgSpec> {
        EwReluxEnW::new(self, 10)
    }
    #[doc = "Bits 16:19 - EW ALU 运算。0：Max；1：Min；2：Add；3：Div；4：Minus；5：Abs；6：Neg；7：Floor；8：Ceil"]
    #[inline(always)]
    pub fn ew_alu_algo(&mut self) -> EwAluAlgoW<'_, EwCfgSpec> {
        EwAluAlgoW::new(self, 16)
    }
    #[doc = "Bit 20 - MinMax 二值使能"]
    #[inline(always)]
    pub fn ew_binary_en(&mut self) -> EwBinaryEnW<'_, EwCfgSpec> {
        EwBinaryEnW::new(self, 20)
    }
    #[doc = "Bit 21 - MinMax 相等使能"]
    #[inline(always)]
    pub fn ew_equal_en(&mut self) -> EwEqualEnW<'_, EwCfgSpec> {
        EwEqualEnW::new(self, 21)
    }
    #[doc = "Bits 22:23 - ERDMA cube 数据大小。0：4bit；1：8bit；2：16bit；3：32bit"]
    #[inline(always)]
    pub fn edata_size(&mut self) -> EdataSizeW<'_, EwCfgSpec> {
        EdataSizeW::new(self, 22)
    }
    #[doc = "Bits 28:29 - ERDMA 数据模式"]
    #[inline(always)]
    pub fn ew_data_mode(&mut self) -> EwDataModeW<'_, EwCfgSpec> {
        EwDataModeW::new(self, 28)
    }
    #[doc = "Bit 30 - EW 输入转换舍入。0：奇入偶不入；1：0.5 向上进 1"]
    #[inline(always)]
    pub fn ew_cvt_round(&mut self) -> EwCvtRoundW<'_, EwCfgSpec> {
        EwCvtRoundW::new(self, 30)
    }
    #[doc = "Bit 31 - EW 输入转换类型。0：先乘后加；1：先加后乘"]
    #[inline(always)]
    pub fn ew_cvt_type(&mut self) -> EwCvtTypeW<'_, EwCfgSpec> {
        EwCvtTypeW::new(self, 31)
    }
}
#[doc = "ew_cfg\n\nYou can [`read`](crate::Reg::read) this register and get [`ew_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ew_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EwCfgSpec;
impl crate::RegisterSpec for EwCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ew_cfg::R`](R) reader structure"]
impl crate::Readable for EwCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`ew_cfg::W`](W) writer structure"]
impl crate::Writable for EwCfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EW_CFG to value 0"]
impl crate::Resettable for EwCfgSpec {}
