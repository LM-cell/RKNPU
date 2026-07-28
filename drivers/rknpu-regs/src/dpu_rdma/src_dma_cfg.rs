#[doc = "Register `SRC_DMA_CFG` reader"]
pub type R = crate::R<SrcDmaCfgSpec>;
#[doc = "Register `SRC_DMA_CFG` writer"]
pub type W = crate::W<SrcDmaCfgSpec>;
#[doc = "Field `KERNEL_WIDTH` reader - 反池化 kernel 宽度（−1）"]
pub type KernelWidthR = crate::FieldReader;
#[doc = "Field `KERNEL_WIDTH` writer - 反池化 kernel 宽度（−1）"]
pub type KernelWidthW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `KERNEL_HEIGHT` reader - 反池化 kernel 高度（−1）"]
pub type KernelHeightR = crate::FieldReader;
#[doc = "Field `KERNEL_HEIGHT` writer - 反池化 kernel 高度（−1）"]
pub type KernelHeightW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `KERNEL_STRIDE_WIDTH` reader - 反池化 kernel 步长宽度（−1）"]
pub type KernelStrideWidthR = crate::FieldReader;
#[doc = "Field `KERNEL_STRIDE_WIDTH` writer - 反池化 kernel 步长宽度（−1）"]
pub type KernelStrideWidthW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `KERNEL_STRIDE_HEIGHT` reader - 反池化 kernel 步长高度（−1）"]
pub type KernelStrideHeightR = crate::FieldReader;
#[doc = "Field `KERNEL_STRIDE_HEIGHT` writer - 反池化 kernel 步长高度（−1）"]
pub type KernelStrideHeightW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `UNPOOLING_EN` reader - 反池化使能"]
pub type UnpoolingEnR = crate::BitReader;
#[doc = "Field `UNPOOLING_EN` writer - 反池化使能"]
pub type UnpoolingEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POOLING_METHOD` reader - 池化方法。0：平均池化（上采样可用此模式）；1：最小/最大池化"]
pub type PoolingMethodR = crate::BitReader;
#[doc = "Field `POOLING_METHOD` writer - 池化方法。0：平均池化（上采样可用此模式）；1：最小/最大池化"]
pub type PoolingMethodW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LINE_NOTCH_ADDR` reader - 宽度末尾到 shape 特征行末的像素数"]
pub type LineNotchAddrR = crate::FieldReader<u16>;
#[doc = "Field `LINE_NOTCH_ADDR` writer - 宽度末尾到 shape 特征行末的像素数"]
pub type LineNotchAddrW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    #[doc = "Bits 0:2 - 反池化 kernel 宽度（−1）"]
    #[inline(always)]
    pub fn kernel_width(&self) -> KernelWidthR {
        KernelWidthR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - 反池化 kernel 高度（−1）"]
    #[inline(always)]
    pub fn kernel_height(&self) -> KernelHeightR {
        KernelHeightR::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:8 - 反池化 kernel 步长宽度（−1）"]
    #[inline(always)]
    pub fn kernel_stride_width(&self) -> KernelStrideWidthR {
        KernelStrideWidthR::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 9:11 - 反池化 kernel 步长高度（−1）"]
    #[inline(always)]
    pub fn kernel_stride_height(&self) -> KernelStrideHeightR {
        KernelStrideHeightR::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bit 12 - 反池化使能"]
    #[inline(always)]
    pub fn unpooling_en(&self) -> UnpoolingEnR {
        UnpoolingEnR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 池化方法。0：平均池化（上采样可用此模式）；1：最小/最大池化"]
    #[inline(always)]
    pub fn pooling_method(&self) -> PoolingMethodR {
        PoolingMethodR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 19:31 - 宽度末尾到 shape 特征行末的像素数"]
    #[inline(always)]
    pub fn line_notch_addr(&self) -> LineNotchAddrR {
        LineNotchAddrR::new(((self.bits >> 19) & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:2 - 反池化 kernel 宽度（−1）"]
    #[inline(always)]
    pub fn kernel_width(&mut self) -> KernelWidthW<'_, SrcDmaCfgSpec> {
        KernelWidthW::new(self, 0)
    }
    #[doc = "Bits 3:5 - 反池化 kernel 高度（−1）"]
    #[inline(always)]
    pub fn kernel_height(&mut self) -> KernelHeightW<'_, SrcDmaCfgSpec> {
        KernelHeightW::new(self, 3)
    }
    #[doc = "Bits 6:8 - 反池化 kernel 步长宽度（−1）"]
    #[inline(always)]
    pub fn kernel_stride_width(&mut self) -> KernelStrideWidthW<'_, SrcDmaCfgSpec> {
        KernelStrideWidthW::new(self, 6)
    }
    #[doc = "Bits 9:11 - 反池化 kernel 步长高度（−1）"]
    #[inline(always)]
    pub fn kernel_stride_height(&mut self) -> KernelStrideHeightW<'_, SrcDmaCfgSpec> {
        KernelStrideHeightW::new(self, 9)
    }
    #[doc = "Bit 12 - 反池化使能"]
    #[inline(always)]
    pub fn unpooling_en(&mut self) -> UnpoolingEnW<'_, SrcDmaCfgSpec> {
        UnpoolingEnW::new(self, 12)
    }
    #[doc = "Bit 13 - 池化方法。0：平均池化（上采样可用此模式）；1：最小/最大池化"]
    #[inline(always)]
    pub fn pooling_method(&mut self) -> PoolingMethodW<'_, SrcDmaCfgSpec> {
        PoolingMethodW::new(self, 13)
    }
    #[doc = "Bits 19:31 - 宽度末尾到 shape 特征行末的像素数"]
    #[inline(always)]
    pub fn line_notch_addr(&mut self) -> LineNotchAddrW<'_, SrcDmaCfgSpec> {
        LineNotchAddrW::new(self, 19)
    }
}
#[doc = "src_dma_cfg\n\nYou can [`read`](crate::Reg::read) this register and get [`src_dma_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`src_dma_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrcDmaCfgSpec;
impl crate::RegisterSpec for SrcDmaCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`src_dma_cfg::R`](R) reader structure"]
impl crate::Readable for SrcDmaCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`src_dma_cfg::W`](W) writer structure"]
impl crate::Writable for SrcDmaCfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SRC_DMA_CFG to value 0"]
impl crate::Resettable for SrcDmaCfgSpec {}
