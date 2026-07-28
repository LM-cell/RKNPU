#[doc = "Register `DCOMP_REGNUM` reader"]
pub type R = crate::R<DcompRegnumSpec>;
#[doc = "Register `DCOMP_REGNUM` writer"]
pub type W = crate::W<DcompRegnumSpec>;
#[doc = "Field `DCOMP_REGNUM` reader - 权重解压寄存器数量"]
pub type DcompRegnumR = crate::FieldReader<u32>;
#[doc = "Field `DCOMP_REGNUM` writer - 权重解压寄存器数量"]
pub type DcompRegnumW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 权重解压寄存器数量"]
    #[inline(always)]
    pub fn dcomp_regnum(&self) -> DcompRegnumR {
        DcompRegnumR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 权重解压寄存器数量"]
    #[inline(always)]
    pub fn dcomp_regnum(&mut self) -> DcompRegnumW<'_, DcompRegnumSpec> {
        DcompRegnumW::new(self, 0)
    }
}
#[doc = "dcomp_regnum\n\nYou can [`read`](crate::Reg::read) this register and get [`dcomp_regnum::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcomp_regnum::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DcompRegnumSpec;
impl crate::RegisterSpec for DcompRegnumSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcomp_regnum::R`](R) reader structure"]
impl crate::Readable for DcompRegnumSpec {}
#[doc = "`write(|w| ..)` method takes [`dcomp_regnum::W`](W) writer structure"]
impl crate::Writable for DcompRegnumSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DCOMP_REGNUM to value 0"]
impl crate::Resettable for DcompRegnumSpec {}
