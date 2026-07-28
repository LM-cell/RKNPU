#[doc = "Register `BN_MUL_CFG` reader"]
pub type R = crate::R<BnMulCfgSpec>;
#[doc = "Register `BN_MUL_CFG` writer"]
pub type W = crate::W<BnMulCfgSpec>;
#[doc = "Field `BN_MUL_SRC` reader - MUL 操作数来源。0：寄存器；1：外部"]
pub type BnMulSrcR = crate::BitReader;
#[doc = "Field `BN_MUL_SRC` writer - MUL 操作数来源。0：寄存器；1：外部"]
pub type BnMulSrcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BN_TRUNCATE_SRC` reader - 移位值来源。0：寄存器；1：外部"]
pub type BnTruncateSrcR = crate::BitReader;
#[doc = "Field `BN_TRUNCATE_SRC` writer - 移位值来源。0：寄存器；1：外部"]
pub type BnTruncateSrcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BN_MUL_SHIFT_VALUE` reader - BN 正数移位值"]
pub type BnMulShiftValueR = crate::FieldReader;
#[doc = "Field `BN_MUL_SHIFT_VALUE` writer - BN 正数移位值"]
pub type BnMulShiftValueW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `BN_MUL_OPERAND` reader - BN MUL 操作数"]
pub type BnMulOperandR = crate::FieldReader<u16>;
#[doc = "Field `BN_MUL_OPERAND` writer - BN MUL 操作数"]
pub type BnMulOperandW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - MUL 操作数来源。0：寄存器；1：外部"]
    #[inline(always)]
    pub fn bn_mul_src(&self) -> BnMulSrcR {
        BnMulSrcR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 移位值来源。0：寄存器；1：外部"]
    #[inline(always)]
    pub fn bn_truncate_src(&self) -> BnTruncateSrcR {
        BnTruncateSrcR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 8:13 - BN 正数移位值"]
    #[inline(always)]
    pub fn bn_mul_shift_value(&self) -> BnMulShiftValueR {
        BnMulShiftValueR::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:31 - BN MUL 操作数"]
    #[inline(always)]
    pub fn bn_mul_operand(&self) -> BnMulOperandR {
        BnMulOperandR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - MUL 操作数来源。0：寄存器；1：外部"]
    #[inline(always)]
    pub fn bn_mul_src(&mut self) -> BnMulSrcW<'_, BnMulCfgSpec> {
        BnMulSrcW::new(self, 0)
    }
    #[doc = "Bit 1 - 移位值来源。0：寄存器；1：外部"]
    #[inline(always)]
    pub fn bn_truncate_src(&mut self) -> BnTruncateSrcW<'_, BnMulCfgSpec> {
        BnTruncateSrcW::new(self, 1)
    }
    #[doc = "Bits 8:13 - BN 正数移位值"]
    #[inline(always)]
    pub fn bn_mul_shift_value(&mut self) -> BnMulShiftValueW<'_, BnMulCfgSpec> {
        BnMulShiftValueW::new(self, 8)
    }
    #[doc = "Bits 16:31 - BN MUL 操作数"]
    #[inline(always)]
    pub fn bn_mul_operand(&mut self) -> BnMulOperandW<'_, BnMulCfgSpec> {
        BnMulOperandW::new(self, 16)
    }
}
#[doc = "bn_mul_cfg\n\nYou can [`read`](crate::Reg::read) this register and get [`bn_mul_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bn_mul_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BnMulCfgSpec;
impl crate::RegisterSpec for BnMulCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bn_mul_cfg::R`](R) reader structure"]
impl crate::Readable for BnMulCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`bn_mul_cfg::W`](W) writer structure"]
impl crate::Writable for BnMulCfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BN_MUL_CFG to value 0"]
impl crate::Resettable for BnMulCfgSpec {}
