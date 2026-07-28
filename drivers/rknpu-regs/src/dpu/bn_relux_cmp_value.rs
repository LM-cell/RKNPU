#[doc = "Register `BN_RELUX_CMP_VALUE` reader"]
pub type R = crate::R<BnReluxCmpValueSpec>;
#[doc = "Register `BN_RELUX_CMP_VALUE` writer"]
pub type W = crate::W<BnReluxCmpValueSpec>;
#[doc = "Field `BN_RELUX_CMP_DAT` reader - BN RELUX 比较数据"]
pub type BnReluxCmpDatR = crate::FieldReader<u32>;
#[doc = "Field `BN_RELUX_CMP_DAT` writer - BN RELUX 比较数据"]
pub type BnReluxCmpDatW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - BN RELUX 比较数据"]
    #[inline(always)]
    pub fn bn_relux_cmp_dat(&self) -> BnReluxCmpDatR {
        BnReluxCmpDatR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - BN RELUX 比较数据"]
    #[inline(always)]
    pub fn bn_relux_cmp_dat(&mut self) -> BnReluxCmpDatW<'_, BnReluxCmpValueSpec> {
        BnReluxCmpDatW::new(self, 0)
    }
}
#[doc = "bn_relux_cmp_value\n\nYou can [`read`](crate::Reg::read) this register and get [`bn_relux_cmp_value::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bn_relux_cmp_value::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BnReluxCmpValueSpec;
impl crate::RegisterSpec for BnReluxCmpValueSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bn_relux_cmp_value::R`](R) reader structure"]
impl crate::Readable for BnReluxCmpValueSpec {}
#[doc = "`write(|w| ..)` method takes [`bn_relux_cmp_value::W`](W) writer structure"]
impl crate::Writable for BnReluxCmpValueSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BN_RELUX_CMP_VALUE to value 0"]
impl crate::Resettable for BnReluxCmpValueSpec {}
