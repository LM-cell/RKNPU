#[doc = "Register `BS_OW_OP` reader"]
pub type R = crate::R<BsOwOpSpec>;
#[doc = "Register `BS_OW_OP` writer"]
pub type W = crate::W<BsOwOpSpec>;
#[doc = "Field `OW_OP` reader - CPEND 操作数"]
pub type OwOpR = crate::FieldReader<u16>;
#[doc = "Field `OW_OP` writer - CPEND 操作数"]
pub type OwOpW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - CPEND 操作数"]
    #[inline(always)]
    pub fn ow_op(&self) -> OwOpR {
        OwOpR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - CPEND 操作数"]
    #[inline(always)]
    pub fn ow_op(&mut self) -> OwOpW<'_, BsOwOpSpec> {
        OwOpW::new(self, 0)
    }
}
#[doc = "bs_ow_op\n\nYou can [`read`](crate::Reg::read) this register and get [`bs_ow_op::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bs_ow_op::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BsOwOpSpec;
impl crate::RegisterSpec for BsOwOpSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bs_ow_op::R`](R) reader structure"]
impl crate::Readable for BsOwOpSpec {}
#[doc = "`write(|w| ..)` method takes [`bs_ow_op::W`](W) writer structure"]
impl crate::Writable for BsOwOpSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BS_OW_OP to value 0"]
impl crate::Resettable for BsOwOpSpec {}
