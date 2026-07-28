#[doc = "Register `INTERRUPT_CLEAR` reader"]
pub type R = crate::R<InterruptClearSpec>;
#[doc = "Register `INTERRUPT_CLEAR` writer"]
pub type W = crate::W<InterruptClearSpec>;
#[doc = "Field `INT_CLR` reader - 中断清除（位定义同 `int_mask`）"]
pub type IntClrR = crate::FieldReader<u32>;
#[doc = "Field `INT_CLR` writer - 中断清除（位定义同 `int_mask`）"]
pub type IntClrW<'a, REG> = crate::FieldWriter<'a, REG, 17, u32>;
impl R {
    #[doc = "Bits 0:16 - 中断清除（位定义同 `int_mask`）"]
    #[inline(always)]
    pub fn int_clr(&self) -> IntClrR {
        IntClrR::new(self.bits & 0x0001_ffff)
    }
}
impl W {
    #[doc = "Bits 0:16 - 中断清除（位定义同 `int_mask`）"]
    #[inline(always)]
    pub fn int_clr(&mut self) -> IntClrW<'_, InterruptClearSpec> {
        IntClrW::new(self, 0)
    }
}
#[doc = "interrupt_clear\n\nYou can [`read`](crate::Reg::read) this register and get [`interrupt_clear::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`interrupt_clear::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct InterruptClearSpec;
impl crate::RegisterSpec for InterruptClearSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`interrupt_clear::R`](R) reader structure"]
impl crate::Readable for InterruptClearSpec {}
#[doc = "`write(|w| ..)` method takes [`interrupt_clear::W`](W) writer structure"]
impl crate::Writable for InterruptClearSpec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x0001_ffff;
}
#[doc = "`reset()` method sets INTERRUPT_CLEAR to value 0"]
impl crate::Resettable for InterruptClearSpec {}
