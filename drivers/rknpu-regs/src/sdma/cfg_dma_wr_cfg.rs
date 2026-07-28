#[doc = "Register `CFG_DMA_WR_CFG` reader"]
pub type R = crate::R<CfgDmaWrCfgSpec>;
#[doc = "Register `CFG_DMA_WR_CFG` writer"]
pub type W = crate::W<CfgDmaWrCfgSpec>;
#[doc = "Field `WR_AWSIZE` reader - AXI awsize"]
pub type WrAwsizeR = crate::FieldReader;
#[doc = "Field `WR_AWSIZE` writer - AXI awsize"]
pub type WrAwsizeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `WR_AWBURST` reader - AXI awburst"]
pub type WrAwburstR = crate::FieldReader;
#[doc = "Field `WR_AWBURST` writer - AXI awburst"]
pub type WrAwburstW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `WR_AWPROT` reader - AXI awprot"]
pub type WrAwprotR = crate::FieldReader;
#[doc = "Field `WR_AWPROT` writer - AXI awprot"]
pub type WrAwprotW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `WR_AWCACHE` reader - AXI awcache"]
pub type WrAwcacheR = crate::FieldReader;
#[doc = "Field `WR_AWCACHE` writer - AXI awcache"]
pub type WrAwcacheW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `WR_AWLOCK` reader - AXI awlock"]
pub type WrAwlockR = crate::BitReader;
#[doc = "Field `WR_AWLOCK` writer - AXI awlock"]
pub type WrAwlockW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - AXI awsize"]
    #[inline(always)]
    pub fn wr_awsize(&self) -> WrAwsizeR {
        WrAwsizeR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:4 - AXI awburst"]
    #[inline(always)]
    pub fn wr_awburst(&self) -> WrAwburstR {
        WrAwburstR::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bits 5:7 - AXI awprot"]
    #[inline(always)]
    pub fn wr_awprot(&self) -> WrAwprotR {
        WrAwprotR::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bits 8:11 - AXI awcache"]
    #[inline(always)]
    pub fn wr_awcache(&self) -> WrAwcacheR {
        WrAwcacheR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - AXI awlock"]
    #[inline(always)]
    pub fn wr_awlock(&self) -> WrAwlockR {
        WrAwlockR::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - AXI awsize"]
    #[inline(always)]
    pub fn wr_awsize(&mut self) -> WrAwsizeW<'_, CfgDmaWrCfgSpec> {
        WrAwsizeW::new(self, 0)
    }
    #[doc = "Bits 3:4 - AXI awburst"]
    #[inline(always)]
    pub fn wr_awburst(&mut self) -> WrAwburstW<'_, CfgDmaWrCfgSpec> {
        WrAwburstW::new(self, 3)
    }
    #[doc = "Bits 5:7 - AXI awprot"]
    #[inline(always)]
    pub fn wr_awprot(&mut self) -> WrAwprotW<'_, CfgDmaWrCfgSpec> {
        WrAwprotW::new(self, 5)
    }
    #[doc = "Bits 8:11 - AXI awcache"]
    #[inline(always)]
    pub fn wr_awcache(&mut self) -> WrAwcacheW<'_, CfgDmaWrCfgSpec> {
        WrAwcacheW::new(self, 8)
    }
    #[doc = "Bit 12 - AXI awlock"]
    #[inline(always)]
    pub fn wr_awlock(&mut self) -> WrAwlockW<'_, CfgDmaWrCfgSpec> {
        WrAwlockW::new(self, 12)
    }
}
#[doc = "cfg_dma_wr_cfg\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_dma_wr_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_dma_wr_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgDmaWrCfgSpec;
impl crate::RegisterSpec for CfgDmaWrCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_dma_wr_cfg::R`](R) reader structure"]
impl crate::Readable for CfgDmaWrCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_dma_wr_cfg::W`](W) writer structure"]
impl crate::Writable for CfgDmaWrCfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CFG_DMA_WR_CFG to value 0"]
impl crate::Resettable for CfgDmaWrCfgSpec {}
