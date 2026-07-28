#[doc = "Register `REGISTER_AMOUNTS` reader"]
pub type R = crate::R<RegisterAmountsSpec>;
#[doc = "Register `REGISTER_AMOUNTS` writer"]
pub type W = crate::W<RegisterAmountsSpec>;
#[doc = "Field `PC_DATA_AMOUNT` reader - 数据量。一个 task 需要取的寄存器数量"]
pub type PcDataAmountR = crate::FieldReader<u16>;
#[doc = "Field `PC_DATA_AMOUNT` writer - 数据量。一个 task 需要取的寄存器数量"]
pub type PcDataAmountW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 数据量。一个 task 需要取的寄存器数量"]
    #[inline(always)]
    pub fn pc_data_amount(&self) -> PcDataAmountR {
        PcDataAmountR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 数据量。一个 task 需要取的寄存器数量"]
    #[inline(always)]
    pub fn pc_data_amount(&mut self) -> PcDataAmountW<'_, RegisterAmountsSpec> {
        PcDataAmountW::new(self, 0)
    }
}
#[doc = "register_amounts\n\nYou can [`read`](crate::Reg::read) this register and get [`register_amounts::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`register_amounts::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegisterAmountsSpec;
impl crate::RegisterSpec for RegisterAmountsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`register_amounts::R`](R) reader structure"]
impl crate::Readable for RegisterAmountsSpec {}
#[doc = "`write(|w| ..)` method takes [`register_amounts::W`](W) writer structure"]
impl crate::Writable for RegisterAmountsSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REGISTER_AMOUNTS to value 0"]
impl crate::Resettable for RegisterAmountsSpec {}
