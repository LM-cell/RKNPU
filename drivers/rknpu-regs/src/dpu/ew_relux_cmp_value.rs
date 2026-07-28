#[doc = "Register `EW_RELUX_CMP_VALUE` reader"]
pub type R = crate::R<EwReluxCmpValueSpec>;
#[doc = "Register `EW_RELUX_CMP_VALUE` writer"]
pub type W = crate::W<EwReluxCmpValueSpec>;
#[doc = "Field `EW_RELUX_CMP_DAT` reader - EW RELUX 比较数据"]
pub type EwReluxCmpDatR = crate::FieldReader<u32>;
#[doc = "Field `EW_RELUX_CMP_DAT` writer - EW RELUX 比较数据"]
pub type EwReluxCmpDatW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - EW RELUX 比较数据"]
    #[inline(always)]
    pub fn ew_relux_cmp_dat(&self) -> EwReluxCmpDatR {
        EwReluxCmpDatR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - EW RELUX 比较数据"]
    #[inline(always)]
    pub fn ew_relux_cmp_dat(&mut self) -> EwReluxCmpDatW<'_, EwReluxCmpValueSpec> {
        EwReluxCmpDatW::new(self, 0)
    }
}
#[doc = "ew_relux_cmp_value\n\nYou can [`read`](crate::Reg::read) this register and get [`ew_relux_cmp_value::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ew_relux_cmp_value::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EwReluxCmpValueSpec;
impl crate::RegisterSpec for EwReluxCmpValueSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ew_relux_cmp_value::R`](R) reader structure"]
impl crate::Readable for EwReluxCmpValueSpec {}
#[doc = "`write(|w| ..)` method takes [`ew_relux_cmp_value::W`](W) writer structure"]
impl crate::Writable for EwReluxCmpValueSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EW_RELUX_CMP_VALUE to value 0"]
impl crate::Resettable for EwReluxCmpValueSpec {}
