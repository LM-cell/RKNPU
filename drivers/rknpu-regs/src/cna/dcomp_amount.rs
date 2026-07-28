#[doc = "Register `DCOMP_AMOUNT%s` reader"]
pub type R = crate::R<DcompAmountSpec>;
#[doc = "Register `DCOMP_AMOUNT%s` writer"]
pub type W = crate::W<DcompAmountSpec>;
#[doc = "Field `DCOMP_AMOUNTN` reader - 第 N 次解压的权重数据量"]
pub type DcompAmountnR = crate::FieldReader<u32>;
#[doc = "Field `DCOMP_AMOUNTN` writer - 第 N 次解压的权重数据量"]
pub type DcompAmountnW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 第 N 次解压的权重数据量"]
    #[inline(always)]
    pub fn dcomp_amountn(&self) -> DcompAmountnR {
        DcompAmountnR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 第 N 次解压的权重数据量"]
    #[inline(always)]
    pub fn dcomp_amountn(&mut self) -> DcompAmountnW<'_, DcompAmountSpec> {
        DcompAmountnW::new(self, 0)
    }
}
#[doc = "dcomp_amount\n\nYou can [`read`](crate::Reg::read) this register and get [`dcomp_amount::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcomp_amount::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DcompAmountSpec;
impl crate::RegisterSpec for DcompAmountSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcomp_amount::R`](R) reader structure"]
impl crate::Readable for DcompAmountSpec {}
#[doc = "`write(|w| ..)` method takes [`dcomp_amount::W`](W) writer structure"]
impl crate::Writable for DcompAmountSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DCOMP_AMOUNT%s to value 0"]
impl crate::Resettable for DcompAmountSpec {}
