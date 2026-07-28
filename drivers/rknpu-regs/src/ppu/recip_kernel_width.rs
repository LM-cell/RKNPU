#[doc = "Register `RECIP_KERNEL_WIDTH` reader"]
pub type R = crate::R<RecipKernelWidthSpec>;
#[doc = "Register `RECIP_KERNEL_WIDTH` writer"]
pub type W = crate::W<RecipKernelWidthSpec>;
#[doc = "Field `RECIP_KERNEL_WIDTH` reader - Shape kernel 宽度的倒数 × 2^16"]
pub type RecipKernelWidthR = crate::FieldReader<u32>;
#[doc = "Field `RECIP_KERNEL_WIDTH` writer - Shape kernel 宽度的倒数 × 2^16"]
pub type RecipKernelWidthW<'a, REG> = crate::FieldWriter<'a, REG, 17, u32>;
impl R {
    #[doc = "Bits 0:16 - Shape kernel 宽度的倒数 × 2^16"]
    #[inline(always)]
    pub fn recip_kernel_width(&self) -> RecipKernelWidthR {
        RecipKernelWidthR::new(self.bits & 0x0001_ffff)
    }
}
impl W {
    #[doc = "Bits 0:16 - Shape kernel 宽度的倒数 × 2^16"]
    #[inline(always)]
    pub fn recip_kernel_width(&mut self) -> RecipKernelWidthW<'_, RecipKernelWidthSpec> {
        RecipKernelWidthW::new(self, 0)
    }
}
#[doc = "recip_kernel_width\n\nYou can [`read`](crate::Reg::read) this register and get [`recip_kernel_width::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`recip_kernel_width::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RecipKernelWidthSpec;
impl crate::RegisterSpec for RecipKernelWidthSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`recip_kernel_width::R`](R) reader structure"]
impl crate::Readable for RecipKernelWidthSpec {}
#[doc = "`write(|w| ..)` method takes [`recip_kernel_width::W`](W) writer structure"]
impl crate::Writable for RecipKernelWidthSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RECIP_KERNEL_WIDTH to value 0"]
impl crate::Resettable for RecipKernelWidthSpec {}
