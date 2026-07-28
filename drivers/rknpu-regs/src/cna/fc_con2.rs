#[doc = "Register `FC_CON2` reader"]
pub type R = crate::R<FcCon2Spec>;
#[doc = "Register `FC_CON2` writer"]
pub type W = crate::W<FcCon2Spec>;
#[doc = "Field `WEIGHT_OFFSET` reader - 权重数据地址偏移"]
pub type WeightOffsetR = crate::FieldReader<u32>;
#[doc = "Field `WEIGHT_OFFSET` writer - 权重数据地址偏移"]
pub type WeightOffsetW<'a, REG> = crate::FieldWriter<'a, REG, 17, u32>;
impl R {
    #[doc = "Bits 0:16 - 权重数据地址偏移"]
    #[inline(always)]
    pub fn weight_offset(&self) -> WeightOffsetR {
        WeightOffsetR::new(self.bits & 0x0001_ffff)
    }
}
impl W {
    #[doc = "Bits 0:16 - 权重数据地址偏移"]
    #[inline(always)]
    pub fn weight_offset(&mut self) -> WeightOffsetW<'_, FcCon2Spec> {
        WeightOffsetW::new(self, 0)
    }
}
#[doc = "fc_con2\n\nYou can [`read`](crate::Reg::read) this register and get [`fc_con2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fc_con2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FcCon2Spec;
impl crate::RegisterSpec for FcCon2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fc_con2::R`](R) reader structure"]
impl crate::Readable for FcCon2Spec {}
#[doc = "`write(|w| ..)` method takes [`fc_con2::W`](W) writer structure"]
impl crate::Writable for FcCon2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FC_CON2 to value 0"]
impl crate::Resettable for FcCon2Spec {}
