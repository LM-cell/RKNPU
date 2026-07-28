#[doc = "Register `CONV_CON2` reader"]
pub type R = crate::R<ConvCon2Spec>;
#[doc = "Register `CONV_CON2` writer"]
pub type W = crate::W<ConvCon2Spec>;
#[doc = "Field `CMD_FIFO_SRST` reader - 命令 FIFO 软复位（调试用）"]
pub type CmdFifoSrstR = crate::BitReader;
#[doc = "Field `CMD_FIFO_SRST` writer - 命令 FIFO 软复位（调试用）"]
pub type CmdFifoSrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSC_DO_EN` reader - 数据扫描控制。0：使能 CSC 输出特征数据到 CORE；1：禁用"]
pub type CscDoEnR = crate::BitReader;
#[doc = "Field `CSC_DO_EN` writer - 数据扫描控制。0：使能 CSC 输出特征数据到 CORE；1：禁用"]
pub type CscDoEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSC_WO_EN` reader - 权重扫描控制。0：使能 CSC 输出权重到 CORE；1：禁用"]
pub type CscWoEnR = crate::BitReader;
#[doc = "Field `CSC_WO_EN` writer - 权重扫描控制。0：使能 CSC 输出权重到 CORE；1：禁用"]
pub type CscWoEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FEATURE_GRAINS` reader - 卷积开始前需缓冲的特征数据行数。建议设为 y_stride + weight_height + 1"]
pub type FeatureGrainsR = crate::FieldReader<u16>;
#[doc = "Field `FEATURE_GRAINS` writer - 卷积开始前需缓冲的特征数据行数。建议设为 y_stride + weight_height + 1"]
pub type FeatureGrainsW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `KERNEL_GROUP` reader - Kernel 分组数。int8 下 32 个 kernel 为 1 组，int16/fp16 下 16 个为 1 组。例：256 个 kernel，int8 下设为 256/32−1=7"]
pub type KernelGroupR = crate::FieldReader;
#[doc = "Field `KERNEL_GROUP` writer - Kernel 分组数。int8 下 32 个 kernel 为 1 组，int16/fp16 下 16 个为 1 组。例：256 个 kernel，int8 下设为 256/32−1=7"]
pub type KernelGroupW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - 命令 FIFO 软复位（调试用）"]
    #[inline(always)]
    pub fn cmd_fifo_srst(&self) -> CmdFifoSrstR {
        CmdFifoSrstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 数据扫描控制。0：使能 CSC 输出特征数据到 CORE；1：禁用"]
    #[inline(always)]
    pub fn csc_do_en(&self) -> CscDoEnR {
        CscDoEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 权重扫描控制。0：使能 CSC 输出权重到 CORE；1：禁用"]
    #[inline(always)]
    pub fn csc_wo_en(&self) -> CscWoEnR {
        CscWoEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:13 - 卷积开始前需缓冲的特征数据行数。建议设为 y_stride + weight_height + 1"]
    #[inline(always)]
    pub fn feature_grains(&self) -> FeatureGrainsR {
        FeatureGrainsR::new(((self.bits >> 4) & 0x03ff) as u16)
    }
    #[doc = "Bits 16:23 - Kernel 分组数。int8 下 32 个 kernel 为 1 组，int16/fp16 下 16 个为 1 组。例：256 个 kernel，int8 下设为 256/32−1=7"]
    #[inline(always)]
    pub fn kernel_group(&self) -> KernelGroupR {
        KernelGroupR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 命令 FIFO 软复位（调试用）"]
    #[inline(always)]
    pub fn cmd_fifo_srst(&mut self) -> CmdFifoSrstW<'_, ConvCon2Spec> {
        CmdFifoSrstW::new(self, 0)
    }
    #[doc = "Bit 1 - 数据扫描控制。0：使能 CSC 输出特征数据到 CORE；1：禁用"]
    #[inline(always)]
    pub fn csc_do_en(&mut self) -> CscDoEnW<'_, ConvCon2Spec> {
        CscDoEnW::new(self, 1)
    }
    #[doc = "Bit 2 - 权重扫描控制。0：使能 CSC 输出权重到 CORE；1：禁用"]
    #[inline(always)]
    pub fn csc_wo_en(&mut self) -> CscWoEnW<'_, ConvCon2Spec> {
        CscWoEnW::new(self, 2)
    }
    #[doc = "Bits 4:13 - 卷积开始前需缓冲的特征数据行数。建议设为 y_stride + weight_height + 1"]
    #[inline(always)]
    pub fn feature_grains(&mut self) -> FeatureGrainsW<'_, ConvCon2Spec> {
        FeatureGrainsW::new(self, 4)
    }
    #[doc = "Bits 16:23 - Kernel 分组数。int8 下 32 个 kernel 为 1 组，int16/fp16 下 16 个为 1 组。例：256 个 kernel，int8 下设为 256/32−1=7"]
    #[inline(always)]
    pub fn kernel_group(&mut self) -> KernelGroupW<'_, ConvCon2Spec> {
        KernelGroupW::new(self, 16)
    }
}
#[doc = "conv_con2\n\nYou can [`read`](crate::Reg::read) this register and get [`conv_con2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`conv_con2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ConvCon2Spec;
impl crate::RegisterSpec for ConvCon2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`conv_con2::R`](R) reader structure"]
impl crate::Readable for ConvCon2Spec {}
#[doc = "`write(|w| ..)` method takes [`conv_con2::W`](W) writer structure"]
impl crate::Writable for ConvCon2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CONV_CON2 to value 0"]
impl crate::Resettable for ConvCon2Spec {}
