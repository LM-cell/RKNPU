#[doc = "Register `EW_OP_VALUE%s` reader"]
pub type R = crate::R<EwOpValueSpec>;
#[doc = "Register `EW_OP_VALUE%s` writer"]
pub type W = crate::W<EwOpValueSpec>;
#[doc = "Field `EW_OPERAND_N` reader - 第 N+1 个 EW 操作数"]
pub type EwOperandNR = crate::FieldReader<u32>;
#[doc = "Field `EW_OPERAND_N` writer - 第 N+1 个 EW 操作数"]
pub type EwOperandNW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 第 N+1 个 EW 操作数"]
    #[inline(always)]
    pub fn ew_operand_n(&self) -> EwOperandNR {
        EwOperandNR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 第 N+1 个 EW 操作数"]
    #[inline(always)]
    pub fn ew_operand_n(&mut self) -> EwOperandNW<'_, EwOpValueSpec> {
        EwOperandNW::new(self, 0)
    }
}
#[doc = "ew_op_value\n\nYou can [`read`](crate::Reg::read) this register and get [`ew_op_value::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ew_op_value::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EwOpValueSpec;
impl crate::RegisterSpec for EwOpValueSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ew_op_value::R`](R) reader structure"]
impl crate::Readable for EwOpValueSpec {}
#[doc = "`write(|w| ..)` method takes [`ew_op_value::W`](W) writer structure"]
impl crate::Writable for EwOpValueSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EW_OP_VALUE%s to value 0"]
impl crate::Resettable for EwOpValueSpec {}
