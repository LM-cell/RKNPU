#[doc = "Register `BS_ALU_CFG` reader"]
pub type R = crate::R<BsAluCfgSpec>;
#[doc = "Register `BS_ALU_CFG` writer"]
pub type W = crate::W<BsAluCfgSpec>;
#[doc = "Field `BS_ALU_OPERAND` reader - BS CORE ALU 操作数"]
pub type BsAluOperandR = crate::FieldReader<u32>;
#[doc = "Field `BS_ALU_OPERAND` writer - BS CORE ALU 操作数"]
pub type BsAluOperandW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - BS CORE ALU 操作数"]
    #[inline(always)]
    pub fn bs_alu_operand(&self) -> BsAluOperandR {
        BsAluOperandR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - BS CORE ALU 操作数"]
    #[inline(always)]
    pub fn bs_alu_operand(&mut self) -> BsAluOperandW<'_, BsAluCfgSpec> {
        BsAluOperandW::new(self, 0)
    }
}
#[doc = "bs_alu_cfg\n\nYou can [`read`](crate::Reg::read) this register and get [`bs_alu_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bs_alu_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BsAluCfgSpec;
impl crate::RegisterSpec for BsAluCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bs_alu_cfg::R`](R) reader structure"]
impl crate::Readable for BsAluCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`bs_alu_cfg::W`](W) writer structure"]
impl crate::Writable for BsAluCfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BS_ALU_CFG to value 0"]
impl crate::Resettable for BsAluCfgSpec {}
