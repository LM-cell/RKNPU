#[doc = "Register `MISC_CFG` reader"]
pub type R = crate::R<MiscCfgSpec>;
#[doc = "Register `MISC_CFG` writer"]
pub type W = crate::W<MiscCfgSpec>;
#[doc = "Field `QD_EN` reader - 量化特征数据计算使能。0：禁用；1：使能"]
pub type QdEnR = crate::BitReader;
#[doc = "Field `QD_EN` writer - 量化特征数据计算使能。0：禁用；1：使能"]
pub type QdEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DW_EN` reader - 深度可分离模式使能。0：禁用；1：使能 Depthwise 模式"]
pub type DwEnR = crate::BitReader;
#[doc = "Field `DW_EN` writer - 深度可分离模式使能。0：禁用；1：使能 Depthwise 模式"]
pub type DwEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PROC_PRECISION` reader - 处理精度。0：int8；1：int16；2：fp16；3：bf16；6：int4；7：tf32"]
pub type ProcPrecisionR = crate::FieldReader;
#[doc = "Field `PROC_PRECISION` writer - 处理精度。0：int8；1：int16；2：fp16；3：bf16；6：int4；7：tf32"]
pub type ProcPrecisionW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SOFT_GATING` reader - 累加器软门控信号"]
pub type SoftGatingR = crate::FieldReader;
#[doc = "Field `SOFT_GATING` writer - 累加器软门控信号"]
pub type SoftGatingW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bit 0 - 量化特征数据计算使能。0：禁用；1：使能"]
    #[inline(always)]
    pub fn qd_en(&self) -> QdEnR {
        QdEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 深度可分离模式使能。0：禁用；1：使能 Depthwise 模式"]
    #[inline(always)]
    pub fn dw_en(&self) -> DwEnR {
        DwEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 8:10 - 处理精度。0：int8；1：int16；2：fp16；3：bf16；6：int4；7：tf32"]
    #[inline(always)]
    pub fn proc_precision(&self) -> ProcPrecisionR {
        ProcPrecisionR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 14:19 - 累加器软门控信号"]
    #[inline(always)]
    pub fn soft_gating(&self) -> SoftGatingR {
        SoftGatingR::new(((self.bits >> 14) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 量化特征数据计算使能。0：禁用；1：使能"]
    #[inline(always)]
    pub fn qd_en(&mut self) -> QdEnW<'_, MiscCfgSpec> {
        QdEnW::new(self, 0)
    }
    #[doc = "Bit 1 - 深度可分离模式使能。0：禁用；1：使能 Depthwise 模式"]
    #[inline(always)]
    pub fn dw_en(&mut self) -> DwEnW<'_, MiscCfgSpec> {
        DwEnW::new(self, 1)
    }
    #[doc = "Bits 8:10 - 处理精度。0：int8；1：int16；2：fp16；3：bf16；6：int4；7：tf32"]
    #[inline(always)]
    pub fn proc_precision(&mut self) -> ProcPrecisionW<'_, MiscCfgSpec> {
        ProcPrecisionW::new(self, 8)
    }
    #[doc = "Bits 14:19 - 累加器软门控信号"]
    #[inline(always)]
    pub fn soft_gating(&mut self) -> SoftGatingW<'_, MiscCfgSpec> {
        SoftGatingW::new(self, 14)
    }
}
#[doc = "misc_cfg\n\nYou can [`read`](crate::Reg::read) this register and get [`misc_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`misc_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MiscCfgSpec;
impl crate::RegisterSpec for MiscCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`misc_cfg::R`](R) reader structure"]
impl crate::Readable for MiscCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`misc_cfg::W`](W) writer structure"]
impl crate::Writable for MiscCfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MISC_CFG to value 0"]
impl crate::Resettable for MiscCfgSpec {}
