#[doc = "Register `POOLING_KERNEL_CFG` reader"]
pub type R = crate::R<PoolingKernelCfgSpec>;
#[doc = "Register `POOLING_KERNEL_CFG` writer"]
pub type W = crate::W<PoolingKernelCfgSpec>;
#[doc = "Field `KERNEL_WIDTH` reader - Kernel 宽度（需减 1）"]
pub type KernelWidthR = crate::FieldReader;
#[doc = "Field `KERNEL_WIDTH` writer - Kernel 宽度（需减 1）"]
pub type KernelWidthW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `KERNEL_HEIGHT` reader - Kernel 高度（需减 1）"]
pub type KernelHeightR = crate::FieldReader;
#[doc = "Field `KERNEL_HEIGHT` writer - Kernel 高度（需减 1）"]
pub type KernelHeightW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `KERNEL_STRIDE_WIDTH` reader - Kernel 步长宽度（需减 1）"]
pub type KernelStrideWidthR = crate::FieldReader;
#[doc = "Field `KERNEL_STRIDE_WIDTH` writer - Kernel 步长宽度（需减 1）"]
pub type KernelStrideWidthW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `KERNEL_STRIDE_HEIGHT` reader - Kernel 步长高度（需减 1）"]
pub type KernelStrideHeightR = crate::FieldReader;
#[doc = "Field `KERNEL_STRIDE_HEIGHT` writer - Kernel 步长高度（需减 1）"]
pub type KernelStrideHeightW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Kernel 宽度（需减 1）"]
    #[inline(always)]
    pub fn kernel_width(&self) -> KernelWidthR {
        KernelWidthR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Kernel 高度（需减 1）"]
    #[inline(always)]
    pub fn kernel_height(&self) -> KernelHeightR {
        KernelHeightR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Kernel 步长宽度（需减 1）"]
    #[inline(always)]
    pub fn kernel_stride_width(&self) -> KernelStrideWidthR {
        KernelStrideWidthR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Kernel 步长高度（需减 1）"]
    #[inline(always)]
    pub fn kernel_stride_height(&self) -> KernelStrideHeightR {
        KernelStrideHeightR::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Kernel 宽度（需减 1）"]
    #[inline(always)]
    pub fn kernel_width(&mut self) -> KernelWidthW<'_, PoolingKernelCfgSpec> {
        KernelWidthW::new(self, 0)
    }
    #[doc = "Bits 8:11 - Kernel 高度（需减 1）"]
    #[inline(always)]
    pub fn kernel_height(&mut self) -> KernelHeightW<'_, PoolingKernelCfgSpec> {
        KernelHeightW::new(self, 8)
    }
    #[doc = "Bits 16:19 - Kernel 步长宽度（需减 1）"]
    #[inline(always)]
    pub fn kernel_stride_width(&mut self) -> KernelStrideWidthW<'_, PoolingKernelCfgSpec> {
        KernelStrideWidthW::new(self, 16)
    }
    #[doc = "Bits 20:23 - Kernel 步长高度（需减 1）"]
    #[inline(always)]
    pub fn kernel_stride_height(&mut self) -> KernelStrideHeightW<'_, PoolingKernelCfgSpec> {
        KernelStrideHeightW::new(self, 20)
    }
}
#[doc = "pooling_kernel_cfg\n\nYou can [`read`](crate::Reg::read) this register and get [`pooling_kernel_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pooling_kernel_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PoolingKernelCfgSpec;
impl crate::RegisterSpec for PoolingKernelCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pooling_kernel_cfg::R`](R) reader structure"]
impl crate::Readable for PoolingKernelCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`pooling_kernel_cfg::W`](W) writer structure"]
impl crate::Writable for PoolingKernelCfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets POOLING_KERNEL_CFG to value 0"]
impl crate::Resettable for PoolingKernelCfgSpec {}
