#[doc = "Register `WEIGHT_SIZE2` reader"]
pub type R = crate::R<WeightSize2Spec>;
#[doc = "Register `WEIGHT_SIZE2` writer"]
pub type W = crate::W<WeightSize2Spec>;
#[doc = "Field `WEIGHT_KERNELS` reader - Kernel 数量"]
pub type WeightKernelsR = crate::FieldReader<u16>;
#[doc = "Field `WEIGHT_KERNELS` writer - Kernel 数量"]
pub type WeightKernelsW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `WEIGHT_HEIGHT` reader - Kernel 高度"]
pub type WeightHeightR = crate::FieldReader;
#[doc = "Field `WEIGHT_HEIGHT` writer - Kernel 高度"]
pub type WeightHeightW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `WEIGHT_WIDTH` reader - Kernel 宽度"]
pub type WeightWidthR = crate::FieldReader;
#[doc = "Field `WEIGHT_WIDTH` writer - Kernel 宽度"]
pub type WeightWidthW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:13 - Kernel 数量"]
    #[inline(always)]
    pub fn weight_kernels(&self) -> WeightKernelsR {
        WeightKernelsR::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bits 16:20 - Kernel 高度"]
    #[inline(always)]
    pub fn weight_height(&self) -> WeightHeightR {
        WeightHeightR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - Kernel 宽度"]
    #[inline(always)]
    pub fn weight_width(&self) -> WeightWidthR {
        WeightWidthR::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:13 - Kernel 数量"]
    #[inline(always)]
    pub fn weight_kernels(&mut self) -> WeightKernelsW<'_, WeightSize2Spec> {
        WeightKernelsW::new(self, 0)
    }
    #[doc = "Bits 16:20 - Kernel 高度"]
    #[inline(always)]
    pub fn weight_height(&mut self) -> WeightHeightW<'_, WeightSize2Spec> {
        WeightHeightW::new(self, 16)
    }
    #[doc = "Bits 24:28 - Kernel 宽度"]
    #[inline(always)]
    pub fn weight_width(&mut self) -> WeightWidthW<'_, WeightSize2Spec> {
        WeightWidthW::new(self, 24)
    }
}
#[doc = "weight_size2\n\nYou can [`read`](crate::Reg::read) this register and get [`weight_size2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`weight_size2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WeightSize2Spec;
impl crate::RegisterSpec for WeightSize2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`weight_size2::R`](R) reader structure"]
impl crate::Readable for WeightSize2Spec {}
#[doc = "`write(|w| ..)` method takes [`weight_size2::W`](W) writer structure"]
impl crate::Writable for WeightSize2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WEIGHT_SIZE2 to value 0"]
impl crate::Resettable for WeightSize2Spec {}
