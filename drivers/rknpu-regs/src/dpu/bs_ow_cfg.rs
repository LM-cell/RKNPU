#[doc = "Register `BS_OW_CFG` reader"]
pub type R = crate::R<BsOwCfgSpec>;
#[doc = "Register `BS_OW_CFG` writer"]
pub type W = crate::W<BsOwCfgSpec>;
#[doc = "Field `OW_SRC` reader - CPEND 操作数来源。0：寄存器；1：外部"]
pub type OwSrcR = crate::BitReader;
#[doc = "Field `OW_SRC` writer - CPEND 操作数来源。0：寄存器；1：外部"]
pub type OwSrcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OD_BYPASS` reader - 旁路 CPEND"]
pub type OdBypassR = crate::BitReader;
#[doc = "Field `OD_BYPASS` writer - 旁路 CPEND"]
pub type OdBypassW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SIZE_E_0` reader - 第一行输出每行 8 通道数（−1）"]
pub type SizeE0R = crate::FieldReader;
#[doc = "Field `SIZE_E_0` writer - 第一行输出每行 8 通道数（−1）"]
pub type SizeE0W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SIZE_E_1` reader - 中间行输出每行 8 通道数（−1）"]
pub type SizeE1R = crate::FieldReader;
#[doc = "Field `SIZE_E_1` writer - 中间行输出每行 8 通道数（−1）"]
pub type SizeE1W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SIZE_E_2` reader - 最后一行输出每行 8 通道数（−1）"]
pub type SizeE2R = crate::FieldReader;
#[doc = "Field `SIZE_E_2` writer - 最后一行输出每行 8 通道数（−1）"]
pub type SizeE2W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `TP_ORG_EN` reader - 原始转置使能"]
pub type TpOrgEnR = crate::BitReader;
#[doc = "Field `TP_ORG_EN` writer - 原始转置使能"]
pub type TpOrgEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RGP_CNTER` reader - 重组计数器。0：全选；1：每 2 选 1；2：每 4 选 1；3：每 8 选 1"]
pub type RgpCnterR = crate::FieldReader;
#[doc = "Field `RGP_CNTER` writer - 重组计数器。0：全选；1：每 2 选 1；2：每 4 选 1；3：每 8 选 1"]
pub type RgpCnterW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - CPEND 操作数来源。0：寄存器；1：外部"]
    #[inline(always)]
    pub fn ow_src(&self) -> OwSrcR {
        OwSrcR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 旁路 CPEND"]
    #[inline(always)]
    pub fn od_bypass(&self) -> OdBypassR {
        OdBypassR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:4 - 第一行输出每行 8 通道数（−1）"]
    #[inline(always)]
    pub fn size_e_0(&self) -> SizeE0R {
        SizeE0R::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bits 5:7 - 中间行输出每行 8 通道数（−1）"]
    #[inline(always)]
    pub fn size_e_1(&self) -> SizeE1R {
        SizeE1R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bits 8:10 - 最后一行输出每行 8 通道数（−1）"]
    #[inline(always)]
    pub fn size_e_2(&self) -> SizeE2R {
        SizeE2R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 27 - 原始转置使能"]
    #[inline(always)]
    pub fn tp_org_en(&self) -> TpOrgEnR {
        TpOrgEnR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:31 - 重组计数器。0：全选；1：每 2 选 1；2：每 4 选 1；3：每 8 选 1"]
    #[inline(always)]
    pub fn rgp_cnter(&self) -> RgpCnterR {
        RgpCnterR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - CPEND 操作数来源。0：寄存器；1：外部"]
    #[inline(always)]
    pub fn ow_src(&mut self) -> OwSrcW<'_, BsOwCfgSpec> {
        OwSrcW::new(self, 0)
    }
    #[doc = "Bit 1 - 旁路 CPEND"]
    #[inline(always)]
    pub fn od_bypass(&mut self) -> OdBypassW<'_, BsOwCfgSpec> {
        OdBypassW::new(self, 1)
    }
    #[doc = "Bits 2:4 - 第一行输出每行 8 通道数（−1）"]
    #[inline(always)]
    pub fn size_e_0(&mut self) -> SizeE0W<'_, BsOwCfgSpec> {
        SizeE0W::new(self, 2)
    }
    #[doc = "Bits 5:7 - 中间行输出每行 8 通道数（−1）"]
    #[inline(always)]
    pub fn size_e_1(&mut self) -> SizeE1W<'_, BsOwCfgSpec> {
        SizeE1W::new(self, 5)
    }
    #[doc = "Bits 8:10 - 最后一行输出每行 8 通道数（−1）"]
    #[inline(always)]
    pub fn size_e_2(&mut self) -> SizeE2W<'_, BsOwCfgSpec> {
        SizeE2W::new(self, 8)
    }
    #[doc = "Bit 27 - 原始转置使能"]
    #[inline(always)]
    pub fn tp_org_en(&mut self) -> TpOrgEnW<'_, BsOwCfgSpec> {
        TpOrgEnW::new(self, 27)
    }
    #[doc = "Bits 28:31 - 重组计数器。0：全选；1：每 2 选 1；2：每 4 选 1；3：每 8 选 1"]
    #[inline(always)]
    pub fn rgp_cnter(&mut self) -> RgpCnterW<'_, BsOwCfgSpec> {
        RgpCnterW::new(self, 28)
    }
}
#[doc = "bs_ow_cfg\n\nYou can [`read`](crate::Reg::read) this register and get [`bs_ow_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bs_ow_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BsOwCfgSpec;
impl crate::RegisterSpec for BsOwCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bs_ow_cfg::R`](R) reader structure"]
impl crate::Readable for BsOwCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`bs_ow_cfg::W`](W) writer structure"]
impl crate::Writable for BsOwCfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BS_OW_CFG to value 0"]
impl crate::Resettable for BsOwCfgSpec {}
