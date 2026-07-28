#[doc = "Register `MISC_CTRL` reader"]
pub type R = crate::R<MiscCtrlSpec>;
#[doc = "Register `MISC_CTRL` writer"]
pub type W = crate::W<MiscCtrlSpec>;
#[doc = "Field `BURST_LEN` reader - Burst 长度。3：Burst4；7：Burst8；15：Burst16"]
pub type BurstLenR = crate::FieldReader;
#[doc = "Field `BURST_LEN` writer - Burst 长度。3：Burst4；7：Burst8；15：Burst16"]
pub type BurstLenW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `NONALIGN` reader - 非对齐模式使能"]
pub type NonalignR = crate::BitReader;
#[doc = "Field `NONALIGN` writer - 非对齐模式使能"]
pub type NonalignW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MC_SURF_OUT` reader - 多 surface 输出使能"]
pub type McSurfOutR = crate::BitReader;
#[doc = "Field `MC_SURF_OUT` writer - 多 surface 输出使能"]
pub type McSurfOutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SURF_LEN` reader - Surface 计数长度"]
pub type SurfLenR = crate::FieldReader<u16>;
#[doc = "Field `SURF_LEN` writer - Surface 计数长度"]
pub type SurfLenW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:3 - Burst 长度。3：Burst4；7：Burst8；15：Burst16"]
    #[inline(always)]
    pub fn burst_len(&self) -> BurstLenR {
        BurstLenR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 7 - 非对齐模式使能"]
    #[inline(always)]
    pub fn nonalign(&self) -> NonalignR {
        NonalignR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 多 surface 输出使能"]
    #[inline(always)]
    pub fn mc_surf_out(&self) -> McSurfOutR {
        McSurfOutR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 16:31 - Surface 计数长度"]
    #[inline(always)]
    pub fn surf_len(&self) -> SurfLenR {
        SurfLenR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:3 - Burst 长度。3：Burst4；7：Burst8；15：Burst16"]
    #[inline(always)]
    pub fn burst_len(&mut self) -> BurstLenW<'_, MiscCtrlSpec> {
        BurstLenW::new(self, 0)
    }
    #[doc = "Bit 7 - 非对齐模式使能"]
    #[inline(always)]
    pub fn nonalign(&mut self) -> NonalignW<'_, MiscCtrlSpec> {
        NonalignW::new(self, 7)
    }
    #[doc = "Bit 8 - 多 surface 输出使能"]
    #[inline(always)]
    pub fn mc_surf_out(&mut self) -> McSurfOutW<'_, MiscCtrlSpec> {
        McSurfOutW::new(self, 8)
    }
    #[doc = "Bits 16:31 - Surface 计数长度"]
    #[inline(always)]
    pub fn surf_len(&mut self) -> SurfLenW<'_, MiscCtrlSpec> {
        SurfLenW::new(self, 16)
    }
}
#[doc = "misc_ctrl\n\nYou can [`read`](crate::Reg::read) this register and get [`misc_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`misc_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MiscCtrlSpec;
impl crate::RegisterSpec for MiscCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`misc_ctrl::R`](R) reader structure"]
impl crate::Readable for MiscCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`misc_ctrl::W`](W) writer structure"]
impl crate::Writable for MiscCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MISC_CTRL to value 0"]
impl crate::Resettable for MiscCtrlSpec {}
