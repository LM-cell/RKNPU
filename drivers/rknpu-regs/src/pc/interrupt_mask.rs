#[doc = "Register `INTERRUPT_MASK` reader"]
pub type R = crate::R<InterruptMaskSpec>;
#[doc = "Register `INTERRUPT_MASK` writer"]
pub type W = crate::W<InterruptMaskSpec>;
#[doc = "Field `INT_MASK` reader - 中断掩码（见下表）"]
pub type IntMaskR = crate::FieldReader<u32>;
#[doc = "Field `INT_MASK` writer - 中断掩码（见下表）"]
pub type IntMaskW<'a, REG> = crate::FieldWriter<'a, REG, 17, u32>;
impl R {
    #[doc = "Bits 0:16 - 中断掩码（见下表）"]
    #[inline(always)]
    pub fn int_mask(&self) -> IntMaskR {
        IntMaskR::new(self.bits & 0x0001_ffff)
    }
}
impl W {
    #[doc = "Bits 0:16 - 中断掩码（见下表）"]
    #[inline(always)]
    pub fn int_mask(&mut self) -> IntMaskW<'_, InterruptMaskSpec> {
        IntMaskW::new(self, 0)
    }
}
#[doc = "interrupt_mask\n\nYou can [`read`](crate::Reg::read) this register and get [`interrupt_mask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`interrupt_mask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct InterruptMaskSpec;
impl crate::RegisterSpec for InterruptMaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`interrupt_mask::R`](R) reader structure"]
impl crate::Readable for InterruptMaskSpec {}
#[doc = "`write(|w| ..)` method takes [`interrupt_mask::W`](W) writer structure"]
impl crate::Writable for InterruptMaskSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INTERRUPT_MASK to value 0x0001_ffff"]
impl crate::Resettable for InterruptMaskSpec {
    const RESET_VALUE: u32 = 0x0001_ffff;
}
