#[doc = "Register `CLK_GATE` reader"]
pub type R = crate::R<ClkGateSpec>;
#[doc = "Register `CLK_GATE` writer"]
pub type W = crate::W<ClkGateSpec>;
#[doc = "Field `CNA_FEATURE_DISABLE_CLKGATE` reader - 特征取数自动时钟门控。0：使能；1：禁用"]
pub type CnaFeatureDisableClkgateR = crate::BitReader;
#[doc = "Field `CNA_FEATURE_DISABLE_CLKGATE` writer - 特征取数自动时钟门控。0：使能；1：禁用"]
pub type CnaFeatureDisableClkgateW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CNA_WEIGHT_DISABLE_CLKGATE` reader - 权重取数自动时钟门控。0：使能；1：禁用"]
pub type CnaWeightDisableClkgateR = crate::BitReader;
#[doc = "Field `CNA_WEIGHT_DISABLE_CLKGATE` writer - 权重取数自动时钟门控。0：使能；1：禁用"]
pub type CnaWeightDisableClkgateW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSC_DISABLE_CLKGATE` reader - CSC 自动时钟门控。0：使能；1：禁用 CSC 时钟门控"]
pub type CscDisableClkgateR = crate::BitReader;
#[doc = "Field `CSC_DISABLE_CLKGATE` writer - CSC 自动时钟门控。0：使能；1：禁用 CSC 时钟门控"]
pub type CscDisableClkgateW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CBUF_CS_DISABLE_CLKGATE` reader - CBUF 自动时钟门控。0：使能自动门控；1：禁用 CBUF 时钟门控"]
pub type CbufCsDisableClkgateR = crate::BitReader;
#[doc = "Field `CBUF_CS_DISABLE_CLKGATE` writer - CBUF 自动时钟门控。0：使能自动门控；1：禁用 CBUF 时钟门控"]
pub type CbufCsDisableClkgateW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 特征取数自动时钟门控。0：使能；1：禁用"]
    #[inline(always)]
    pub fn cna_feature_disable_clkgate(&self) -> CnaFeatureDisableClkgateR {
        CnaFeatureDisableClkgateR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 权重取数自动时钟门控。0：使能；1：禁用"]
    #[inline(always)]
    pub fn cna_weight_disable_clkgate(&self) -> CnaWeightDisableClkgateR {
        CnaWeightDisableClkgateR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CSC 自动时钟门控。0：使能；1：禁用 CSC 时钟门控"]
    #[inline(always)]
    pub fn csc_disable_clkgate(&self) -> CscDisableClkgateR {
        CscDisableClkgateR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - CBUF 自动时钟门控。0：使能自动门控；1：禁用 CBUF 时钟门控"]
    #[inline(always)]
    pub fn cbuf_cs_disable_clkgate(&self) -> CbufCsDisableClkgateR {
        CbufCsDisableClkgateR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 特征取数自动时钟门控。0：使能；1：禁用"]
    #[inline(always)]
    pub fn cna_feature_disable_clkgate(&mut self) -> CnaFeatureDisableClkgateW<'_, ClkGateSpec> {
        CnaFeatureDisableClkgateW::new(self, 0)
    }
    #[doc = "Bit 1 - 权重取数自动时钟门控。0：使能；1：禁用"]
    #[inline(always)]
    pub fn cna_weight_disable_clkgate(&mut self) -> CnaWeightDisableClkgateW<'_, ClkGateSpec> {
        CnaWeightDisableClkgateW::new(self, 1)
    }
    #[doc = "Bit 2 - CSC 自动时钟门控。0：使能；1：禁用 CSC 时钟门控"]
    #[inline(always)]
    pub fn csc_disable_clkgate(&mut self) -> CscDisableClkgateW<'_, ClkGateSpec> {
        CscDisableClkgateW::new(self, 2)
    }
    #[doc = "Bit 4 - CBUF 自动时钟门控。0：使能自动门控；1：禁用 CBUF 时钟门控"]
    #[inline(always)]
    pub fn cbuf_cs_disable_clkgate(&mut self) -> CbufCsDisableClkgateW<'_, ClkGateSpec> {
        CbufCsDisableClkgateW::new(self, 4)
    }
}
#[doc = "clk_gate\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_gate::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_gate::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkGateSpec;
impl crate::RegisterSpec for ClkGateSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk_gate::R`](R) reader structure"]
impl crate::Readable for ClkGateSpec {}
#[doc = "`write(|w| ..)` method takes [`clk_gate::W`](W) writer structure"]
impl crate::Writable for ClkGateSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CLK_GATE to value 0"]
impl crate::Resettable for ClkGateSpec {}
