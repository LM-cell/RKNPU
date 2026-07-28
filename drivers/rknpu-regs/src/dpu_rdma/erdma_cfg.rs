#[doc = "Register `ERDMA_CFG` reader"]
pub type R = crate::R<ErdmaCfgSpec>;
#[doc = "Register `ERDMA_CFG` writer"]
pub type W = crate::W<ErdmaCfgSpec>;
#[doc = "Field `ERDMA_DISABLE` reader - 禁用 ERDMA。0：不禁用；1：禁用"]
pub type ErdmaDisableR = crate::BitReader;
#[doc = "Field `ERDMA_DISABLE` writer - 禁用 ERDMA。0：不禁用；1：禁用"]
pub type ErdmaDisableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OV4K_BYPASS` reader - 超 4K burst 拆分。0：使能；1：旁路"]
pub type Ov4kBypassR = crate::BitReader;
#[doc = "Field `OV4K_BYPASS` writer - 超 4K burst 拆分。0：使能；1：旁路"]
pub type Ov4kBypassW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERDMA_DATA_SIZE` reader - ERDMA 读取精度。0：4bit；1：8bit；2：16bit；3：32bit"]
pub type ErdmaDataSizeR = crate::FieldReader;
#[doc = "Field `ERDMA_DATA_SIZE` writer - ERDMA 读取精度。0：4bit；1：8bit；2：16bit；3：32bit"]
pub type ErdmaDataSizeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ERDMA_NONALIGN` reader - 非对齐模式。0：禁用；1：使能"]
pub type ErdmaNonalignR = crate::BitReader;
#[doc = "Field `ERDMA_NONALIGN` writer - 非对齐模式。0：禁用；1：使能"]
pub type ErdmaNonalignW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERDMA_SURF_MODE` reader - Surface 模式。0：1 surface 串行；1：2 surface 串行"]
pub type ErdmaSurfModeR = crate::BitReader;
#[doc = "Field `ERDMA_SURF_MODE` writer - Surface 模式。0：1 surface 串行；1：2 surface 串行"]
pub type ErdmaSurfModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERDMA_DATA_MODE` reader - 数据模式。0：按通道；1：按像素；2：按通道×像素；3：保留"]
pub type ErdmaDataModeR = crate::FieldReader;
#[doc = "Field `ERDMA_DATA_MODE` writer - 数据模式。0：按通道；1：按像素；2：按通道×像素；3：保留"]
pub type ErdmaDataModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - 禁用 ERDMA。0：不禁用；1：禁用"]
    #[inline(always)]
    pub fn erdma_disable(&self) -> ErdmaDisableR {
        ErdmaDisableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 超 4K burst 拆分。0：使能；1：旁路"]
    #[inline(always)]
    pub fn ov4k_bypass(&self) -> Ov4kBypassR {
        Ov4kBypassR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - ERDMA 读取精度。0：4bit；1：8bit；2：16bit；3：32bit"]
    #[inline(always)]
    pub fn erdma_data_size(&self) -> ErdmaDataSizeR {
        ErdmaDataSizeR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 28 - 非对齐模式。0：禁用；1：使能"]
    #[inline(always)]
    pub fn erdma_nonalign(&self) -> ErdmaNonalignR {
        ErdmaNonalignR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Surface 模式。0：1 surface 串行；1：2 surface 串行"]
    #[inline(always)]
    pub fn erdma_surf_mode(&self) -> ErdmaSurfModeR {
        ErdmaSurfModeR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bits 30:31 - 数据模式。0：按通道；1：按像素；2：按通道×像素；3：保留"]
    #[inline(always)]
    pub fn erdma_data_mode(&self) -> ErdmaDataModeR {
        ErdmaDataModeR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 禁用 ERDMA。0：不禁用；1：禁用"]
    #[inline(always)]
    pub fn erdma_disable(&mut self) -> ErdmaDisableW<'_, ErdmaCfgSpec> {
        ErdmaDisableW::new(self, 0)
    }
    #[doc = "Bit 1 - 超 4K burst 拆分。0：使能；1：旁路"]
    #[inline(always)]
    pub fn ov4k_bypass(&mut self) -> Ov4kBypassW<'_, ErdmaCfgSpec> {
        Ov4kBypassW::new(self, 1)
    }
    #[doc = "Bits 2:3 - ERDMA 读取精度。0：4bit；1：8bit；2：16bit；3：32bit"]
    #[inline(always)]
    pub fn erdma_data_size(&mut self) -> ErdmaDataSizeW<'_, ErdmaCfgSpec> {
        ErdmaDataSizeW::new(self, 2)
    }
    #[doc = "Bit 28 - 非对齐模式。0：禁用；1：使能"]
    #[inline(always)]
    pub fn erdma_nonalign(&mut self) -> ErdmaNonalignW<'_, ErdmaCfgSpec> {
        ErdmaNonalignW::new(self, 28)
    }
    #[doc = "Bit 29 - Surface 模式。0：1 surface 串行；1：2 surface 串行"]
    #[inline(always)]
    pub fn erdma_surf_mode(&mut self) -> ErdmaSurfModeW<'_, ErdmaCfgSpec> {
        ErdmaSurfModeW::new(self, 29)
    }
    #[doc = "Bits 30:31 - 数据模式。0：按通道；1：按像素；2：按通道×像素；3：保留"]
    #[inline(always)]
    pub fn erdma_data_mode(&mut self) -> ErdmaDataModeW<'_, ErdmaCfgSpec> {
        ErdmaDataModeW::new(self, 30)
    }
}
#[doc = "erdma_cfg\n\nYou can [`read`](crate::Reg::read) this register and get [`erdma_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`erdma_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ErdmaCfgSpec;
impl crate::RegisterSpec for ErdmaCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`erdma_cfg::R`](R) reader structure"]
impl crate::Readable for ErdmaCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`erdma_cfg::W`](W) writer structure"]
impl crate::Writable for ErdmaCfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ERDMA_CFG to value 0"]
impl crate::Resettable for ErdmaCfgSpec {}
