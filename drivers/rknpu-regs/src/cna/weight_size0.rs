#[doc = "Register `WEIGHT_SIZE0` reader"]
pub type R = crate::R<WeightSize0Spec>;
#[doc = "Register `WEIGHT_SIZE0` writer"]
pub type W = crate::W<WeightSize0Spec>;
#[doc = "Field `WEIGHT_BYTES` reader - 本次卷积的权重总字节数"]
pub type WeightBytesR = crate::FieldReader<u32>;
#[doc = "Field `WEIGHT_BYTES` writer - 本次卷积的权重总字节数"]
pub type WeightBytesW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 本次卷积的权重总字节数"]
    #[inline(always)]
    pub fn weight_bytes(&self) -> WeightBytesR {
        WeightBytesR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 本次卷积的权重总字节数"]
    #[inline(always)]
    pub fn weight_bytes(&mut self) -> WeightBytesW<'_, WeightSize0Spec> {
        WeightBytesW::new(self, 0)
    }
}
#[doc = "weight_size0\n\nYou can [`read`](crate::Reg::read) this register and get [`weight_size0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`weight_size0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WeightSize0Spec;
impl crate::RegisterSpec for WeightSize0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`weight_size0::R`](R) reader structure"]
impl crate::Readable for WeightSize0Spec {}
#[doc = "`write(|w| ..)` method takes [`weight_size0::W`](W) writer structure"]
impl crate::Writable for WeightSize0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WEIGHT_SIZE0 to value 0"]
impl crate::Resettable for WeightSize0Spec {}
