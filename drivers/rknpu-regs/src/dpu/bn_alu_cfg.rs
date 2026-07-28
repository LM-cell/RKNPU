#[doc = "Register `BN_ALU_CFG` reader"]
pub type R = crate::R<BnAluCfgSpec>;
#[doc = "Register `BN_ALU_CFG` writer"]
pub type W = crate::W<BnAluCfgSpec>;
#[doc = "Field `BN_ALU_OPERAND` reader - BN CORE ALU 操作数"]
pub type BnAluOperandR = crate::FieldReader<u32>;
#[doc = "Field `BN_ALU_OPERAND` writer - BN CORE ALU 操作数"]
pub type BnAluOperandW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - BN CORE ALU 操作数"]
    #[inline(always)]
    pub fn bn_alu_operand(&self) -> BnAluOperandR {
        BnAluOperandR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - BN CORE ALU 操作数"]
    #[inline(always)]
    pub fn bn_alu_operand(&mut self) -> BnAluOperandW<'_, BnAluCfgSpec> {
        BnAluOperandW::new(self, 0)
    }
}
#[doc = "bn_alu_cfg\n\nYou can [`read`](crate::Reg::read) this register and get [`bn_alu_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bn_alu_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BnAluCfgSpec;
impl crate::RegisterSpec for BnAluCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bn_alu_cfg::R`](R) reader structure"]
impl crate::Readable for BnAluCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`bn_alu_cfg::W`](W) writer structure"]
impl crate::Writable for BnAluCfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BN_ALU_CFG to value 0"]
impl crate::Resettable for BnAluCfgSpec {}
