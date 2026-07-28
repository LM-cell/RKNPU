#[doc = "Register `S_STATUS` reader"]
pub type R = crate::R<SStatusSpec>;
#[doc = "Field `STATUS_0` reader - 执行器 0 状态。编码同 `status_1`"]
pub type Status0R = crate::FieldReader;
#[doc = "Field `STATUS_1` reader - 执行器 1 状态。0：空闲；1：正在执行；2：正在执行且等待执行；3：保留"]
pub type Status1R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:1 - 执行器 0 状态。编码同 `status_1`"]
    #[inline(always)]
    pub fn status_0(&self) -> Status0R {
        Status0R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 16:17 - 执行器 1 状态。0：空闲；1：正在执行；2：正在执行且等待执行；3：保留"]
    #[inline(always)]
    pub fn status_1(&self) -> Status1R {
        Status1R::new(((self.bits >> 16) & 3) as u8)
    }
}
#[doc = "s_status\n\nYou can [`read`](crate::Reg::read) this register and get [`s_status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SStatusSpec;
impl crate::RegisterSpec for SStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`s_status::R`](R) reader structure"]
impl crate::Readable for SStatusSpec {}
#[doc = "`reset()` method sets S_STATUS to value 0"]
impl crate::Resettable for SStatusSpec {}
