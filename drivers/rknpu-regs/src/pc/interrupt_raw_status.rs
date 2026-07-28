#[doc = "Register `INTERRUPT_RAW_STATUS` reader"]
pub type R = crate::R<InterruptRawStatusSpec>;
#[doc = "Register `INTERRUPT_RAW_STATUS` writer"]
pub type W = crate::W<InterruptRawStatusSpec>;
#[doc = "Field `INT_RAW_ST` reader - 中断原始状态（位定义同 `int_mask`）"]
pub type IntRawStR = crate::FieldReader<u32>;
#[doc = "Field `INT_RAW_ST` writer - 中断原始状态（位定义同 `int_mask`）"]
pub type IntRawStW<'a, REG> = crate::FieldWriter<'a, REG, 17, u32>;
impl R {
    #[doc = "Bits 0:16 - 中断原始状态（位定义同 `int_mask`）"]
    #[inline(always)]
    pub fn int_raw_st(&self) -> IntRawStR {
        IntRawStR::new(self.bits & 0x0001_ffff)
    }
}
impl W {
    #[doc = "Bits 0:16 - 中断原始状态（位定义同 `int_mask`）"]
    #[inline(always)]
    pub fn int_raw_st(&mut self) -> IntRawStW<'_, InterruptRawStatusSpec> {
        IntRawStW::new(self, 0)
    }
}
#[doc = "interrupt_raw_status\n\nYou can [`read`](crate::Reg::read) this register and get [`interrupt_raw_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`interrupt_raw_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct InterruptRawStatusSpec;
impl crate::RegisterSpec for InterruptRawStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`interrupt_raw_status::R`](R) reader structure"]
impl crate::Readable for InterruptRawStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`interrupt_raw_status::W`](W) writer structure"]
impl crate::Writable for InterruptRawStatusSpec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x0001_ffff;
}
#[doc = "`reset()` method sets INTERRUPT_RAW_STATUS to value 0"]
impl crate::Resettable for InterruptRawStatusSpec {}
