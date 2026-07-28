#[doc = "Register `CFG_DMA_ARB` reader"]
pub type R = crate::R<CfgDmaArbSpec>;
#[doc = "Register `CFG_DMA_ARB` writer"]
pub type W = crate::W<CfgDmaArbSpec>;
#[doc = "Field `RD_FIX_ARB` reader - 读固定仲裁"]
pub type RdFixArbR = crate::FieldReader;
#[doc = "Field `RD_FIX_ARB` writer - 读固定仲裁"]
pub type RdFixArbW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `WR_FIX_ARB` reader - 写固定仲裁"]
pub type WrFixArbR = crate::FieldReader;
#[doc = "Field `WR_FIX_ARB` writer - 写固定仲裁"]
pub type WrFixArbW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RD_ARBIT_MODEL` reader - 读仲裁模型"]
pub type RdArbitModelR = crate::BitReader;
#[doc = "Field `RD_ARBIT_MODEL` writer - 读仲裁模型"]
pub type RdArbitModelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WR_ARBIT_MODEL` reader - 写仲裁模型"]
pub type WrArbitModelR = crate::BitReader;
#[doc = "Field `WR_ARBIT_MODEL` writer - 写仲裁模型"]
pub type WrArbitModelW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - 读固定仲裁"]
    #[inline(always)]
    pub fn rd_fix_arb(&self) -> RdFixArbR {
        RdFixArbR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - 写固定仲裁"]
    #[inline(always)]
    pub fn wr_fix_arb(&self) -> WrFixArbR {
        WrFixArbR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 8 - 读仲裁模型"]
    #[inline(always)]
    pub fn rd_arbit_model(&self) -> RdArbitModelR {
        RdArbitModelR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 写仲裁模型"]
    #[inline(always)]
    pub fn wr_arbit_model(&self) -> WrArbitModelR {
        WrArbitModelR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - 读固定仲裁"]
    #[inline(always)]
    pub fn rd_fix_arb(&mut self) -> RdFixArbW<'_, CfgDmaArbSpec> {
        RdFixArbW::new(self, 0)
    }
    #[doc = "Bits 4:6 - 写固定仲裁"]
    #[inline(always)]
    pub fn wr_fix_arb(&mut self) -> WrFixArbW<'_, CfgDmaArbSpec> {
        WrFixArbW::new(self, 4)
    }
    #[doc = "Bit 8 - 读仲裁模型"]
    #[inline(always)]
    pub fn rd_arbit_model(&mut self) -> RdArbitModelW<'_, CfgDmaArbSpec> {
        RdArbitModelW::new(self, 8)
    }
    #[doc = "Bit 9 - 写仲裁模型"]
    #[inline(always)]
    pub fn wr_arbit_model(&mut self) -> WrArbitModelW<'_, CfgDmaArbSpec> {
        WrArbitModelW::new(self, 9)
    }
}
#[doc = "cfg_dma_arb\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_dma_arb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_dma_arb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgDmaArbSpec;
impl crate::RegisterSpec for CfgDmaArbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_dma_arb::R`](R) reader structure"]
impl crate::Readable for CfgDmaArbSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_dma_arb::W`](W) writer structure"]
impl crate::Writable for CfgDmaArbSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CFG_DMA_ARB to value 0"]
impl crate::Resettable for CfgDmaArbSpec {}
