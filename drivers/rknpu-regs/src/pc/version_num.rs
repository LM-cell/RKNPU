#[doc = "Register `VERSION_NUM` reader"]
pub type R = crate::R<VersionNumSpec>;
#[doc = "Field `VERSION_NUM` reader - 硬件版本编号"]
pub type VersionNumR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - 硬件版本编号"]
    #[inline(always)]
    pub fn version_num(&self) -> VersionNumR {
        VersionNumR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "version_num\n\nYou can [`read`](crate::Reg::read) this register and get [`version_num::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VersionNumSpec;
impl crate::RegisterSpec for VersionNumSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`version_num::R`](R) reader structure"]
impl crate::Readable for VersionNumSpec {}
#[doc = "`reset()` method sets VERSION_NUM to value 0"]
impl crate::Resettable for VersionNumSpec {}
