#[doc = "Register `LUT_CFG` reader"]
pub type R = crate::R<LutCfgSpec>;
#[doc = "Register `LUT_CFG` writer"]
pub type W = crate::W<LutCfgSpec>;
#[doc = "Field `LUT_ROAD_SEL` reader - LUT 路径选择。0：第 1 路；1：第 2 路"]
pub type LutRoadSelR = crate::BitReader;
#[doc = "Field `LUT_ROAD_SEL` writer - LUT 路径选择。0：第 1 路；1：第 2 路"]
pub type LutRoadSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LUT_EXPAND_EN` reader - 扩展两个小 LUT 为一个大 LUT"]
pub type LutExpandEnR = crate::BitReader;
#[doc = "Field `LUT_EXPAND_EN` writer - 扩展两个小 LUT 为一个大 LUT"]
pub type LutExpandEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LUT_LO_LE_MUX` reader - LO/LE LUT 复用"]
pub type LutLoLeMuxR = crate::FieldReader;
#[doc = "Field `LUT_LO_LE_MUX` writer - LO/LE LUT 复用"]
pub type LutLoLeMuxW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `LUT_UFLOW_PRIORITY` reader - 下溢优先级。0：LE；1：LO"]
pub type LutUflowPriorityR = crate::BitReader;
#[doc = "Field `LUT_UFLOW_PRIORITY` writer - 下溢优先级。0：LE；1：LO"]
pub type LutUflowPriorityW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LUT_OFLOW_PRIORITY` reader - 上溢优先级。0：LE；1：LO"]
pub type LutOflowPriorityR = crate::BitReader;
#[doc = "Field `LUT_OFLOW_PRIORITY` writer - 上溢优先级。0：LE；1：LO"]
pub type LutOflowPriorityW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LUT_HYBRID_PRIORITY` reader - 混合流优先级。0：LE LUT；1：LO LUT"]
pub type LutHybridPriorityR = crate::BitReader;
#[doc = "Field `LUT_HYBRID_PRIORITY` writer - 混合流优先级。0：LE LUT；1：LO LUT"]
pub type LutHybridPriorityW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LUT_CAL_SEL` reader - LUT 计算选择（仅 `lut_expand_en=1` 时有效）"]
pub type LutCalSelR = crate::BitReader;
#[doc = "Field `LUT_CAL_SEL` writer - LUT 计算选择（仅 `lut_expand_en=1` 时有效）"]
pub type LutCalSelW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - LUT 路径选择。0：第 1 路；1：第 2 路"]
    #[inline(always)]
    pub fn lut_road_sel(&self) -> LutRoadSelR {
        LutRoadSelR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 扩展两个小 LUT 为一个大 LUT"]
    #[inline(always)]
    pub fn lut_expand_en(&self) -> LutExpandEnR {
        LutExpandEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - LO/LE LUT 复用"]
    #[inline(always)]
    pub fn lut_lo_le_mux(&self) -> LutLoLeMuxR {
        LutLoLeMuxR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - 下溢优先级。0：LE；1：LO"]
    #[inline(always)]
    pub fn lut_uflow_priority(&self) -> LutUflowPriorityR {
        LutUflowPriorityR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 上溢优先级。0：LE；1：LO"]
    #[inline(always)]
    pub fn lut_oflow_priority(&self) -> LutOflowPriorityR {
        LutOflowPriorityR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 混合流优先级。0：LE LUT；1：LO LUT"]
    #[inline(always)]
    pub fn lut_hybrid_priority(&self) -> LutHybridPriorityR {
        LutHybridPriorityR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - LUT 计算选择（仅 `lut_expand_en=1` 时有效）"]
    #[inline(always)]
    pub fn lut_cal_sel(&self) -> LutCalSelR {
        LutCalSelR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LUT 路径选择。0：第 1 路；1：第 2 路"]
    #[inline(always)]
    pub fn lut_road_sel(&mut self) -> LutRoadSelW<'_, LutCfgSpec> {
        LutRoadSelW::new(self, 0)
    }
    #[doc = "Bit 1 - 扩展两个小 LUT 为一个大 LUT"]
    #[inline(always)]
    pub fn lut_expand_en(&mut self) -> LutExpandEnW<'_, LutCfgSpec> {
        LutExpandEnW::new(self, 1)
    }
    #[doc = "Bits 2:3 - LO/LE LUT 复用"]
    #[inline(always)]
    pub fn lut_lo_le_mux(&mut self) -> LutLoLeMuxW<'_, LutCfgSpec> {
        LutLoLeMuxW::new(self, 2)
    }
    #[doc = "Bit 4 - 下溢优先级。0：LE；1：LO"]
    #[inline(always)]
    pub fn lut_uflow_priority(&mut self) -> LutUflowPriorityW<'_, LutCfgSpec> {
        LutUflowPriorityW::new(self, 4)
    }
    #[doc = "Bit 5 - 上溢优先级。0：LE；1：LO"]
    #[inline(always)]
    pub fn lut_oflow_priority(&mut self) -> LutOflowPriorityW<'_, LutCfgSpec> {
        LutOflowPriorityW::new(self, 5)
    }
    #[doc = "Bit 6 - 混合流优先级。0：LE LUT；1：LO LUT"]
    #[inline(always)]
    pub fn lut_hybrid_priority(&mut self) -> LutHybridPriorityW<'_, LutCfgSpec> {
        LutHybridPriorityW::new(self, 6)
    }
    #[doc = "Bit 7 - LUT 计算选择（仅 `lut_expand_en=1` 时有效）"]
    #[inline(always)]
    pub fn lut_cal_sel(&mut self) -> LutCalSelW<'_, LutCfgSpec> {
        LutCalSelW::new(self, 7)
    }
}
#[doc = "lut_cfg\n\nYou can [`read`](crate::Reg::read) this register and get [`lut_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lut_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LutCfgSpec;
impl crate::RegisterSpec for LutCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lut_cfg::R`](R) reader structure"]
impl crate::Readable for LutCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`lut_cfg::W`](W) writer structure"]
impl crate::Writable for LutCfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LUT_CFG to value 0"]
impl crate::Resettable for LutCfgSpec {}
