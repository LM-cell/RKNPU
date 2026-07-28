#[doc = "Register `INTERRUPT_STATUS` reader"]
pub type R = crate::R<InterruptStatusSpec>;
#[doc = "Register `INTERRUPT_STATUS` writer"]
pub type W = crate::W<InterruptStatusSpec>;
#[doc = "Field `INT_ST` reader - 中断状态，与 mask 位做 AND（位定义同 `int_mask`）"]
pub type IntStR = crate::FieldReader<u32>;
#[doc = "Field `INT_ST` writer - 中断状态，与 mask 位做 AND（位定义同 `int_mask`）"]
pub type IntStW<'a, REG> = crate::FieldWriter<'a, REG, 17, u32>;
impl R {
    #[doc = "Bits 0:16 - 中断状态，与 mask 位做 AND（位定义同 `int_mask`）"]
    #[inline(always)]
    pub fn int_st(&self) -> IntStR {
        IntStR::new(self.bits & 0x0001_ffff)
    }
}
impl W {
    #[doc = "Bits 0:16 - 中断状态，与 mask 位做 AND（位定义同 `int_mask`）"]
    #[inline(always)]
    pub fn int_st(&mut self) -> IntStW<'_, InterruptStatusSpec> {
        IntStW::new(self, 0)
    }
}
#[doc = "interrupt_status\n\nYou can [`read`](crate::Reg::read) this register and get [`interrupt_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`interrupt_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct InterruptStatusSpec;
impl crate::RegisterSpec for InterruptStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`interrupt_status::R`](R) reader structure"]
impl crate::Readable for InterruptStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`interrupt_status::W`](W) writer structure"]
impl crate::Writable for InterruptStatusSpec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x0001_ffff;
}
#[doc = "`reset()` method sets INTERRUPT_STATUS to value 0"]
impl crate::Resettable for InterruptStatusSpec {}
