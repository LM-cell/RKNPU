#[doc = "Register `WEIGHT_SIZE1` reader"]
pub type R = crate::R<WeightSize1Spec>;
#[doc = "Register `WEIGHT_SIZE1` writer"]
pub type W = crate::W<WeightSize1Spec>;
#[doc = "Field `WEIGHT_BYTES_PER_KERNEL` reader - 单个 kernel 的权重字节数"]
pub type WeightBytesPerKernelR = crate::FieldReader<u32>;
#[doc = "Field `WEIGHT_BYTES_PER_KERNEL` writer - 单个 kernel 的权重字节数"]
pub type WeightBytesPerKernelW<'a, REG> = crate::FieldWriter<'a, REG, 19, u32>;
impl R {
    #[doc = "Bits 0:18 - 单个 kernel 的权重字节数"]
    #[inline(always)]
    pub fn weight_bytes_per_kernel(&self) -> WeightBytesPerKernelR {
        WeightBytesPerKernelR::new(self.bits & 0x0007_ffff)
    }
}
impl W {
    #[doc = "Bits 0:18 - 单个 kernel 的权重字节数"]
    #[inline(always)]
    pub fn weight_bytes_per_kernel(&mut self) -> WeightBytesPerKernelW<'_, WeightSize1Spec> {
        WeightBytesPerKernelW::new(self, 0)
    }
}
#[doc = "weight_size1\n\nYou can [`read`](crate::Reg::read) this register and get [`weight_size1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`weight_size1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WeightSize1Spec;
impl crate::RegisterSpec for WeightSize1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`weight_size1::R`](R) reader structure"]
impl crate::Readable for WeightSize1Spec {}
#[doc = "`write(|w| ..)` method takes [`weight_size1::W`](W) writer structure"]
impl crate::Writable for WeightSize1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WEIGHT_SIZE1 to value 0"]
impl crate::Resettable for WeightSize1Spec {}
