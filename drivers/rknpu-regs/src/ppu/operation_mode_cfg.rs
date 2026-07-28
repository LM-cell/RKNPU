#[doc = "Register `OPERATION_MODE_CFG` reader"]
pub type R = crate::R<OperationModeCfgSpec>;
#[doc = "Register `OPERATION_MODE_CFG` writer"]
pub type W = crate::W<OperationModeCfgSpec>;
#[doc = "Field `POOLING_METHOD` reader - 池化方法。0：平均池化；1：最大池化；2：最小池化；3：保留"]
pub type PoolingMethodR = crate::FieldReader;
#[doc = "Field `POOLING_METHOD` writer - 池化方法。0：平均池化；1：最大池化；2：最小池化；3：保留"]
pub type PoolingMethodW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FLYING_MODE` reader - 池化 cube 来源。0：DPU；1：外部"]
pub type FlyingModeR = crate::BitReader;
#[doc = "Field `FLYING_MODE` writer - 池化 cube 来源。0：DPU；1：外部"]
pub type FlyingModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USE_CNT` reader - use_cnt"]
pub type UseCntR = crate::FieldReader;
#[doc = "Field `USE_CNT` writer - use_cnt"]
pub type UseCntW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `NOTCH_ADDR` reader - 宽度末尾到 shape 行末的像素数"]
pub type NotchAddrR = crate::FieldReader<u16>;
#[doc = "Field `NOTCH_ADDR` writer - 宽度末尾到 shape 行末的像素数"]
pub type NotchAddrW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
#[doc = "Field `INDEX_EN` reader - 使能输出每个 kernel 的位置索引"]
pub type IndexEnR = crate::BitReader;
#[doc = "Field `INDEX_EN` writer - 使能输出每个 kernel 的位置索引"]
pub type IndexEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - 池化方法。0：平均池化；1：最大池化；2：最小池化；3：保留"]
    #[inline(always)]
    pub fn pooling_method(&self) -> PoolingMethodR {
        PoolingMethodR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 4 - 池化 cube 来源。0：DPU；1：外部"]
    #[inline(always)]
    pub fn flying_mode(&self) -> FlyingModeR {
        FlyingModeR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:7 - use_cnt"]
    #[inline(always)]
    pub fn use_cnt(&self) -> UseCntR {
        UseCntR::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bits 16:28 - 宽度末尾到 shape 行末的像素数"]
    #[inline(always)]
    pub fn notch_addr(&self) -> NotchAddrR {
        NotchAddrR::new(((self.bits >> 16) & 0x1fff) as u16)
    }
    #[doc = "Bit 30 - 使能输出每个 kernel 的位置索引"]
    #[inline(always)]
    pub fn index_en(&self) -> IndexEnR {
        IndexEnR::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - 池化方法。0：平均池化；1：最大池化；2：最小池化；3：保留"]
    #[inline(always)]
    pub fn pooling_method(&mut self) -> PoolingMethodW<'_, OperationModeCfgSpec> {
        PoolingMethodW::new(self, 0)
    }
    #[doc = "Bit 4 - 池化 cube 来源。0：DPU；1：外部"]
    #[inline(always)]
    pub fn flying_mode(&mut self) -> FlyingModeW<'_, OperationModeCfgSpec> {
        FlyingModeW::new(self, 4)
    }
    #[doc = "Bits 5:7 - use_cnt"]
    #[inline(always)]
    pub fn use_cnt(&mut self) -> UseCntW<'_, OperationModeCfgSpec> {
        UseCntW::new(self, 5)
    }
    #[doc = "Bits 16:28 - 宽度末尾到 shape 行末的像素数"]
    #[inline(always)]
    pub fn notch_addr(&mut self) -> NotchAddrW<'_, OperationModeCfgSpec> {
        NotchAddrW::new(self, 16)
    }
    #[doc = "Bit 30 - 使能输出每个 kernel 的位置索引"]
    #[inline(always)]
    pub fn index_en(&mut self) -> IndexEnW<'_, OperationModeCfgSpec> {
        IndexEnW::new(self, 30)
    }
}
#[doc = "operation_mode_cfg\n\nYou can [`read`](crate::Reg::read) this register and get [`operation_mode_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`operation_mode_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OperationModeCfgSpec;
impl crate::RegisterSpec for OperationModeCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`operation_mode_cfg::R`](R) reader structure"]
impl crate::Readable for OperationModeCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`operation_mode_cfg::W`](W) writer structure"]
impl crate::Writable for OperationModeCfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OPERATION_MODE_CFG to value 0"]
impl crate::Resettable for OperationModeCfgSpec {}
