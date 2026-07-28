#[doc = "Register `BN_CFG` reader"]
pub type R = crate::R<BnCfgSpec>;
#[doc = "Register `BN_CFG` writer"]
pub type W = crate::W<BnCfgSpec>;
#[doc = "Field `BN_BYPASS` reader - 旁路整个 BN CORE"]
pub type BnBypassR = crate::BitReader;
#[doc = "Field `BN_BYPASS` writer - 旁路整个 BN CORE"]
pub type BnBypassW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BN_ALU_BYPASS` reader - 旁路 BN ALU"]
pub type BnAluBypassR = crate::BitReader;
#[doc = "Field `BN_ALU_BYPASS` writer - 旁路 BN ALU"]
pub type BnAluBypassW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BN_MUL_BYPASS` reader - 旁路 BN MUL"]
pub type BnMulBypassR = crate::BitReader;
#[doc = "Field `BN_MUL_BYPASS` writer - 旁路 BN MUL"]
pub type BnMulBypassW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BN_MUL_PRELU` reader - MUL PRELU 使能"]
pub type BnMulPreluR = crate::BitReader;
#[doc = "Field `BN_MUL_PRELU` writer - MUL PRELU 使能"]
pub type BnMulPreluW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BN_RELU_BYPASS` reader - 旁路 BN RELU"]
pub type BnReluBypassR = crate::BitReader;
#[doc = "Field `BN_RELU_BYPASS` writer - 旁路 BN RELU"]
pub type BnReluBypassW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BN_RELUX_EN` reader - RELUX 使能"]
pub type BnReluxEnR = crate::BitReader;
#[doc = "Field `BN_RELUX_EN` writer - RELUX 使能"]
pub type BnReluxEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BN_ALU_SRC` reader - ALU 操作数来源。0：寄存器；1：外部"]
pub type BnAluSrcR = crate::BitReader;
#[doc = "Field `BN_ALU_SRC` writer - ALU 操作数来源。0：寄存器；1：外部"]
pub type BnAluSrcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BN_ALU_ALGO` reader - BN ALU 运算类型。2：Add；4：Minus"]
pub type BnAluAlgoR = crate::FieldReader;
#[doc = "Field `BN_ALU_ALGO` writer - BN ALU 运算类型。2：Add；4：Minus"]
pub type BnAluAlgoW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - 旁路整个 BN CORE"]
    #[inline(always)]
    pub fn bn_bypass(&self) -> BnBypassR {
        BnBypassR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 旁路 BN ALU"]
    #[inline(always)]
    pub fn bn_alu_bypass(&self) -> BnAluBypassR {
        BnAluBypassR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - 旁路 BN MUL"]
    #[inline(always)]
    pub fn bn_mul_bypass(&self) -> BnMulBypassR {
        BnMulBypassR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - MUL PRELU 使能"]
    #[inline(always)]
    pub fn bn_mul_prelu(&self) -> BnMulPreluR {
        BnMulPreluR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 旁路 BN RELU"]
    #[inline(always)]
    pub fn bn_relu_bypass(&self) -> BnReluBypassR {
        BnReluBypassR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - RELUX 使能"]
    #[inline(always)]
    pub fn bn_relux_en(&self) -> BnReluxEnR {
        BnReluxEnR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - ALU 操作数来源。0：寄存器；1：外部"]
    #[inline(always)]
    pub fn bn_alu_src(&self) -> BnAluSrcR {
        BnAluSrcR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 16:19 - BN ALU 运算类型。2：Add；4：Minus"]
    #[inline(always)]
    pub fn bn_alu_algo(&self) -> BnAluAlgoR {
        BnAluAlgoR::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 旁路整个 BN CORE"]
    #[inline(always)]
    pub fn bn_bypass(&mut self) -> BnBypassW<'_, BnCfgSpec> {
        BnBypassW::new(self, 0)
    }
    #[doc = "Bit 1 - 旁路 BN ALU"]
    #[inline(always)]
    pub fn bn_alu_bypass(&mut self) -> BnAluBypassW<'_, BnCfgSpec> {
        BnAluBypassW::new(self, 1)
    }
    #[doc = "Bit 4 - 旁路 BN MUL"]
    #[inline(always)]
    pub fn bn_mul_bypass(&mut self) -> BnMulBypassW<'_, BnCfgSpec> {
        BnMulBypassW::new(self, 4)
    }
    #[doc = "Bit 5 - MUL PRELU 使能"]
    #[inline(always)]
    pub fn bn_mul_prelu(&mut self) -> BnMulPreluW<'_, BnCfgSpec> {
        BnMulPreluW::new(self, 5)
    }
    #[doc = "Bit 6 - 旁路 BN RELU"]
    #[inline(always)]
    pub fn bn_relu_bypass(&mut self) -> BnReluBypassW<'_, BnCfgSpec> {
        BnReluBypassW::new(self, 6)
    }
    #[doc = "Bit 7 - RELUX 使能"]
    #[inline(always)]
    pub fn bn_relux_en(&mut self) -> BnReluxEnW<'_, BnCfgSpec> {
        BnReluxEnW::new(self, 7)
    }
    #[doc = "Bit 8 - ALU 操作数来源。0：寄存器；1：外部"]
    #[inline(always)]
    pub fn bn_alu_src(&mut self) -> BnAluSrcW<'_, BnCfgSpec> {
        BnAluSrcW::new(self, 8)
    }
    #[doc = "Bits 16:19 - BN ALU 运算类型。2：Add；4：Minus"]
    #[inline(always)]
    pub fn bn_alu_algo(&mut self) -> BnAluAlgoW<'_, BnCfgSpec> {
        BnAluAlgoW::new(self, 16)
    }
}
#[doc = "bn_cfg\n\nYou can [`read`](crate::Reg::read) this register and get [`bn_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bn_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BnCfgSpec;
impl crate::RegisterSpec for BnCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bn_cfg::R`](R) reader structure"]
impl crate::Readable for BnCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`bn_cfg::W`](W) writer structure"]
impl crate::Writable for BnCfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BN_CFG to value 0"]
impl crate::Resettable for BnCfgSpec {}
