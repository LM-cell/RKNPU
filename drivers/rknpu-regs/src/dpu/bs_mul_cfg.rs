#[doc = "Register `BS_MUL_CFG` reader"]
pub type R = crate::R<BsMulCfgSpec>;
#[doc = "Register `BS_MUL_CFG` writer"]
pub type W = crate::W<BsMulCfgSpec>;
#[doc = "Field `BS_MUL_SRC` reader - MUL 操作数来源。0：寄存器；1：外部"]
pub type BsMulSrcR = crate::BitReader;
#[doc = "Field `BS_MUL_SRC` writer - MUL 操作数来源。0：寄存器；1：外部"]
pub type BsMulSrcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BS_TRUNCATE_SRC` reader - 移位值来源。0：寄存器；1：外部"]
pub type BsTruncateSrcR = crate::BitReader;
#[doc = "Field `BS_TRUNCATE_SRC` writer - 移位值来源。0：寄存器；1：外部"]
pub type BsTruncateSrcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BS_MUL_SHIFT_VALUE` reader - BS 正数移位值"]
pub type BsMulShiftValueR = crate::FieldReader;
#[doc = "Field `BS_MUL_SHIFT_VALUE` writer - BS 正数移位值"]
pub type BsMulShiftValueW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `BS_MUL_OPERAND` reader - BS MUL 操作数"]
pub type BsMulOperandR = crate::FieldReader<u16>;
#[doc = "Field `BS_MUL_OPERAND` writer - BS MUL 操作数"]
pub type BsMulOperandW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - MUL 操作数来源。0：寄存器；1：外部"]
    #[inline(always)]
    pub fn bs_mul_src(&self) -> BsMulSrcR {
        BsMulSrcR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 移位值来源。0：寄存器；1：外部"]
    #[inline(always)]
    pub fn bs_truncate_src(&self) -> BsTruncateSrcR {
        BsTruncateSrcR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 8:13 - BS 正数移位值"]
    #[inline(always)]
    pub fn bs_mul_shift_value(&self) -> BsMulShiftValueR {
        BsMulShiftValueR::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:31 - BS MUL 操作数"]
    #[inline(always)]
    pub fn bs_mul_operand(&self) -> BsMulOperandR {
        BsMulOperandR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - MUL 操作数来源。0：寄存器；1：外部"]
    #[inline(always)]
    pub fn bs_mul_src(&mut self) -> BsMulSrcW<'_, BsMulCfgSpec> {
        BsMulSrcW::new(self, 0)
    }
    #[doc = "Bit 1 - 移位值来源。0：寄存器；1：外部"]
    #[inline(always)]
    pub fn bs_truncate_src(&mut self) -> BsTruncateSrcW<'_, BsMulCfgSpec> {
        BsTruncateSrcW::new(self, 1)
    }
    #[doc = "Bits 8:13 - BS 正数移位值"]
    #[inline(always)]
    pub fn bs_mul_shift_value(&mut self) -> BsMulShiftValueW<'_, BsMulCfgSpec> {
        BsMulShiftValueW::new(self, 8)
    }
    #[doc = "Bits 16:31 - BS MUL 操作数"]
    #[inline(always)]
    pub fn bs_mul_operand(&mut self) -> BsMulOperandW<'_, BsMulCfgSpec> {
        BsMulOperandW::new(self, 16)
    }
}
#[doc = "bs_mul_cfg\n\nYou can [`read`](crate::Reg::read) this register and get [`bs_mul_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bs_mul_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BsMulCfgSpec;
impl crate::RegisterSpec for BsMulCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bs_mul_cfg::R`](R) reader structure"]
impl crate::Readable for BsMulCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`bs_mul_cfg::W`](W) writer structure"]
impl crate::Writable for BsMulCfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BS_MUL_CFG to value 0"]
impl crate::Resettable for BsMulCfgSpec {}
