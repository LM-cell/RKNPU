#[doc = "Register `BS_CFG` reader"]
pub type R = crate::R<BsCfgSpec>;
#[doc = "Register `BS_CFG` writer"]
pub type W = crate::W<BsCfgSpec>;
#[doc = "Field `BS_BYPASS` reader - 旁路整个 BS CORE"]
pub type BsBypassR = crate::BitReader;
#[doc = "Field `BS_BYPASS` writer - 旁路整个 BS CORE"]
pub type BsBypassW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BS_ALU_BYPASS` reader - 旁路 BS ALU"]
pub type BsAluBypassR = crate::BitReader;
#[doc = "Field `BS_ALU_BYPASS` writer - 旁路 BS ALU"]
pub type BsAluBypassW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BS_MUL_BYPASS` reader - 旁路 BS MUL"]
pub type BsMulBypassR = crate::BitReader;
#[doc = "Field `BS_MUL_BYPASS` writer - 旁路 BS MUL"]
pub type BsMulBypassW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BS_MUL_PRELU` reader - MUL PRELU 使能"]
pub type BsMulPreluR = crate::BitReader;
#[doc = "Field `BS_MUL_PRELU` writer - MUL PRELU 使能"]
pub type BsMulPreluW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BS_RELU_BYPASS` reader - 旁路 BS RELU。0：不旁路；1：旁路"]
pub type BsReluBypassR = crate::BitReader;
#[doc = "Field `BS_RELU_BYPASS` writer - 旁路 BS RELU。0：不旁路；1：旁路"]
pub type BsReluBypassW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BS_RELUX_EN` reader - RELUX 使能"]
pub type BsReluxEnR = crate::BitReader;
#[doc = "Field `BS_RELUX_EN` writer - RELUX 使能"]
pub type BsReluxEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BS_ALU_SRC` reader - ALU 操作数来源。0：寄存器；1：外部"]
pub type BsAluSrcR = crate::BitReader;
#[doc = "Field `BS_ALU_SRC` writer - ALU 操作数来源。0：寄存器；1：外部"]
pub type BsAluSrcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BS_ALU_ALGO` reader - BS ALU 运算类型。2：Add；4：Minus"]
pub type BsAluAlgoR = crate::FieldReader;
#[doc = "Field `BS_ALU_ALGO` writer - BS ALU 运算类型。2：Add；4：Minus"]
pub type BsAluAlgoW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - 旁路整个 BS CORE"]
    #[inline(always)]
    pub fn bs_bypass(&self) -> BsBypassR {
        BsBypassR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 旁路 BS ALU"]
    #[inline(always)]
    pub fn bs_alu_bypass(&self) -> BsAluBypassR {
        BsAluBypassR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - 旁路 BS MUL"]
    #[inline(always)]
    pub fn bs_mul_bypass(&self) -> BsMulBypassR {
        BsMulBypassR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - MUL PRELU 使能"]
    #[inline(always)]
    pub fn bs_mul_prelu(&self) -> BsMulPreluR {
        BsMulPreluR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 旁路 BS RELU。0：不旁路；1：旁路"]
    #[inline(always)]
    pub fn bs_relu_bypass(&self) -> BsReluBypassR {
        BsReluBypassR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - RELUX 使能"]
    #[inline(always)]
    pub fn bs_relux_en(&self) -> BsReluxEnR {
        BsReluxEnR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - ALU 操作数来源。0：寄存器；1：外部"]
    #[inline(always)]
    pub fn bs_alu_src(&self) -> BsAluSrcR {
        BsAluSrcR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 16:19 - BS ALU 运算类型。2：Add；4：Minus"]
    #[inline(always)]
    pub fn bs_alu_algo(&self) -> BsAluAlgoR {
        BsAluAlgoR::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 旁路整个 BS CORE"]
    #[inline(always)]
    pub fn bs_bypass(&mut self) -> BsBypassW<'_, BsCfgSpec> {
        BsBypassW::new(self, 0)
    }
    #[doc = "Bit 1 - 旁路 BS ALU"]
    #[inline(always)]
    pub fn bs_alu_bypass(&mut self) -> BsAluBypassW<'_, BsCfgSpec> {
        BsAluBypassW::new(self, 1)
    }
    #[doc = "Bit 4 - 旁路 BS MUL"]
    #[inline(always)]
    pub fn bs_mul_bypass(&mut self) -> BsMulBypassW<'_, BsCfgSpec> {
        BsMulBypassW::new(self, 4)
    }
    #[doc = "Bit 5 - MUL PRELU 使能"]
    #[inline(always)]
    pub fn bs_mul_prelu(&mut self) -> BsMulPreluW<'_, BsCfgSpec> {
        BsMulPreluW::new(self, 5)
    }
    #[doc = "Bit 6 - 旁路 BS RELU。0：不旁路；1：旁路"]
    #[inline(always)]
    pub fn bs_relu_bypass(&mut self) -> BsReluBypassW<'_, BsCfgSpec> {
        BsReluBypassW::new(self, 6)
    }
    #[doc = "Bit 7 - RELUX 使能"]
    #[inline(always)]
    pub fn bs_relux_en(&mut self) -> BsReluxEnW<'_, BsCfgSpec> {
        BsReluxEnW::new(self, 7)
    }
    #[doc = "Bit 8 - ALU 操作数来源。0：寄存器；1：外部"]
    #[inline(always)]
    pub fn bs_alu_src(&mut self) -> BsAluSrcW<'_, BsCfgSpec> {
        BsAluSrcW::new(self, 8)
    }
    #[doc = "Bits 16:19 - BS ALU 运算类型。2：Add；4：Minus"]
    #[inline(always)]
    pub fn bs_alu_algo(&mut self) -> BsAluAlgoW<'_, BsCfgSpec> {
        BsAluAlgoW::new(self, 16)
    }
}
#[doc = "bs_cfg\n\nYou can [`read`](crate::Reg::read) this register and get [`bs_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bs_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BsCfgSpec;
impl crate::RegisterSpec for BsCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bs_cfg::R`](R) reader structure"]
impl crate::Readable for BsCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`bs_cfg::W`](W) writer structure"]
impl crate::Writable for BsCfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BS_CFG to value 0"]
impl crate::Resettable for BsCfgSpec {}
