#[doc = "Register `CFG_DMA_RD_CFG` reader"]
pub type R = crate::R<CfgDmaRdCfgSpec>;
#[doc = "Register `CFG_DMA_RD_CFG` writer"]
pub type W = crate::W<CfgDmaRdCfgSpec>;
#[doc = "Field `RD_ARSIZE` reader - AXI arsize"]
pub type RdArsizeR = crate::FieldReader;
#[doc = "Field `RD_ARSIZE` writer - AXI arsize"]
pub type RdArsizeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RD_ARBURST` reader - AXI arburst"]
pub type RdArburstR = crate::FieldReader;
#[doc = "Field `RD_ARBURST` writer - AXI arburst"]
pub type RdArburstW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RD_ARPROT` reader - AXI arprot"]
pub type RdArprotR = crate::FieldReader;
#[doc = "Field `RD_ARPROT` writer - AXI arprot"]
pub type RdArprotW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RD_ARCACHE` reader - AXI arcache"]
pub type RdArcacheR = crate::FieldReader;
#[doc = "Field `RD_ARCACHE` writer - AXI arcache"]
pub type RdArcacheW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RD_ARLOCK` reader - AXI arlock"]
pub type RdArlockR = crate::BitReader;
#[doc = "Field `RD_ARLOCK` writer - AXI arlock"]
pub type RdArlockW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - AXI arsize"]
    #[inline(always)]
    pub fn rd_arsize(&self) -> RdArsizeR {
        RdArsizeR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:4 - AXI arburst"]
    #[inline(always)]
    pub fn rd_arburst(&self) -> RdArburstR {
        RdArburstR::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bits 5:7 - AXI arprot"]
    #[inline(always)]
    pub fn rd_arprot(&self) -> RdArprotR {
        RdArprotR::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bits 8:11 - AXI arcache"]
    #[inline(always)]
    pub fn rd_arcache(&self) -> RdArcacheR {
        RdArcacheR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - AXI arlock"]
    #[inline(always)]
    pub fn rd_arlock(&self) -> RdArlockR {
        RdArlockR::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - AXI arsize"]
    #[inline(always)]
    pub fn rd_arsize(&mut self) -> RdArsizeW<'_, CfgDmaRdCfgSpec> {
        RdArsizeW::new(self, 0)
    }
    #[doc = "Bits 3:4 - AXI arburst"]
    #[inline(always)]
    pub fn rd_arburst(&mut self) -> RdArburstW<'_, CfgDmaRdCfgSpec> {
        RdArburstW::new(self, 3)
    }
    #[doc = "Bits 5:7 - AXI arprot"]
    #[inline(always)]
    pub fn rd_arprot(&mut self) -> RdArprotW<'_, CfgDmaRdCfgSpec> {
        RdArprotW::new(self, 5)
    }
    #[doc = "Bits 8:11 - AXI arcache"]
    #[inline(always)]
    pub fn rd_arcache(&mut self) -> RdArcacheW<'_, CfgDmaRdCfgSpec> {
        RdArcacheW::new(self, 8)
    }
    #[doc = "Bit 12 - AXI arlock"]
    #[inline(always)]
    pub fn rd_arlock(&mut self) -> RdArlockW<'_, CfgDmaRdCfgSpec> {
        RdArlockW::new(self, 12)
    }
}
#[doc = "cfg_dma_rd_cfg\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_dma_rd_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_dma_rd_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgDmaRdCfgSpec;
impl crate::RegisterSpec for CfgDmaRdCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_dma_rd_cfg::R`](R) reader structure"]
impl crate::Readable for CfgDmaRdCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_dma_rd_cfg::W`](W) writer structure"]
impl crate::Writable for CfgDmaRdCfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CFG_DMA_RD_CFG to value 0"]
impl crate::Resettable for CfgDmaRdCfgSpec {}
