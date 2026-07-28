#[doc = "Register `RECIP_KERNEL_HEIGHT` reader"]
pub type R = crate::R<RecipKernelHeightSpec>;
#[doc = "Register `RECIP_KERNEL_HEIGHT` writer"]
pub type W = crate::W<RecipKernelHeightSpec>;
#[doc = "Field `RECIP_KERNEL_HEIGHT` reader - Shape kernel 高度的倒数 × 2^16"]
pub type RecipKernelHeightR = crate::FieldReader<u32>;
#[doc = "Field `RECIP_KERNEL_HEIGHT` writer - Shape kernel 高度的倒数 × 2^16"]
pub type RecipKernelHeightW<'a, REG> = crate::FieldWriter<'a, REG, 17, u32>;
impl R {
    #[doc = "Bits 0:16 - Shape kernel 高度的倒数 × 2^16"]
    #[inline(always)]
    pub fn recip_kernel_height(&self) -> RecipKernelHeightR {
        RecipKernelHeightR::new(self.bits & 0x0001_ffff)
    }
}
impl W {
    #[doc = "Bits 0:16 - Shape kernel 高度的倒数 × 2^16"]
    #[inline(always)]
    pub fn recip_kernel_height(&mut self) -> RecipKernelHeightW<'_, RecipKernelHeightSpec> {
        RecipKernelHeightW::new(self, 0)
    }
}
#[doc = "recip_kernel_height\n\nYou can [`read`](crate::Reg::read) this register and get [`recip_kernel_height::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`recip_kernel_height::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RecipKernelHeightSpec;
impl crate::RegisterSpec for RecipKernelHeightSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`recip_kernel_height::R`](R) reader structure"]
impl crate::Readable for RecipKernelHeightSpec {}
#[doc = "`write(|w| ..)` method takes [`recip_kernel_height::W`](W) writer structure"]
impl crate::Writable for RecipKernelHeightSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RECIP_KERNEL_HEIGHT to value 0"]
impl crate::Resettable for RecipKernelHeightSpec {}
