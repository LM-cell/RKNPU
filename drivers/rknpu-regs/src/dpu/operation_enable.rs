#[doc = "Register `OPERATION_ENABLE` reader"]
pub type R = crate::R<OperationEnableSpec>;
#[doc = "Register `OPERATION_ENABLE` writer"]
pub type W = crate::W<OperationEnableSpec>;
#[doc = "Field `OP_EN` reader - DPU 操作使能。0：禁用；1：使能"]
pub type OpEnR = crate::BitReader;
#[doc = "Field `OP_EN` writer - DPU 操作使能。0：禁用；1：使能"]
pub type OpEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - DPU 操作使能。0：禁用；1：使能"]
    #[inline(always)]
    pub fn op_en(&self) -> OpEnR {
        OpEnR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DPU 操作使能。0：禁用；1：使能"]
    #[inline(always)]
    pub fn op_en(&mut self) -> OpEnW<'_, OperationEnableSpec> {
        OpEnW::new(self, 0)
    }
}
#[doc = "operation_enable\n\nYou can [`read`](crate::Reg::read) this register and get [`operation_enable::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`operation_enable::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OperationEnableSpec;
impl crate::RegisterSpec for OperationEnableSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`operation_enable::R`](R) reader structure"]
impl crate::Readable for OperationEnableSpec {}
#[doc = "`write(|w| ..)` method takes [`operation_enable::W`](W) writer structure"]
impl crate::Writable for OperationEnableSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OPERATION_ENABLE to value 0"]
impl crate::Resettable for OperationEnableSpec {}
