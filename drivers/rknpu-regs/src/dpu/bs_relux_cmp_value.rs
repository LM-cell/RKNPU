#[doc = "Register `BS_RELUX_CMP_VALUE` reader"]
pub type R = crate::R<BsReluxCmpValueSpec>;
#[doc = "Register `BS_RELUX_CMP_VALUE` writer"]
pub type W = crate::W<BsReluxCmpValueSpec>;
#[doc = "Field `BS_RELUX_CMP_DAT` reader - RELUX 比较值"]
pub type BsReluxCmpDatR = crate::FieldReader<u32>;
#[doc = "Field `BS_RELUX_CMP_DAT` writer - RELUX 比较值"]
pub type BsReluxCmpDatW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - RELUX 比较值"]
    #[inline(always)]
    pub fn bs_relux_cmp_dat(&self) -> BsReluxCmpDatR {
        BsReluxCmpDatR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - RELUX 比较值"]
    #[inline(always)]
    pub fn bs_relux_cmp_dat(&mut self) -> BsReluxCmpDatW<'_, BsReluxCmpValueSpec> {
        BsReluxCmpDatW::new(self, 0)
    }
}
#[doc = "bs_relux_cmp_value\n\nYou can [`read`](crate::Reg::read) this register and get [`bs_relux_cmp_value::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bs_relux_cmp_value::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BsReluxCmpValueSpec;
impl crate::RegisterSpec for BsReluxCmpValueSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bs_relux_cmp_value::R`](R) reader structure"]
impl crate::Readable for BsReluxCmpValueSpec {}
#[doc = "`write(|w| ..)` method takes [`bs_relux_cmp_value::W`](W) writer structure"]
impl crate::Writable for BsReluxCmpValueSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BS_RELUX_CMP_VALUE to value 0"]
impl crate::Resettable for BsReluxCmpValueSpec {}
