#[doc = "Register `DCOMP_CTRL` reader"]
pub type R = crate::R<DcompCtrlSpec>;
#[doc = "Register `DCOMP_CTRL` writer"]
pub type W = crate::W<DcompCtrlSpec>;
#[doc = "Field `DECOMP_CONTROL` reader - 权重解压控制"]
pub type DecompControlR = crate::FieldReader;
#[doc = "Field `DECOMP_CONTROL` writer - 权重解压控制"]
pub type DecompControlW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `WT_DEC_BYPASS` reader - 旁路权重解压"]
pub type WtDecBypassR = crate::BitReader;
#[doc = "Field `WT_DEC_BYPASS` writer - 旁路权重解压"]
pub type WtDecBypassW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - 权重解压控制"]
    #[inline(always)]
    pub fn decomp_control(&self) -> DecompControlR {
        DecompControlR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - 旁路权重解压"]
    #[inline(always)]
    pub fn wt_dec_bypass(&self) -> WtDecBypassR {
        WtDecBypassR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - 权重解压控制"]
    #[inline(always)]
    pub fn decomp_control(&mut self) -> DecompControlW<'_, DcompCtrlSpec> {
        DecompControlW::new(self, 0)
    }
    #[doc = "Bit 3 - 旁路权重解压"]
    #[inline(always)]
    pub fn wt_dec_bypass(&mut self) -> WtDecBypassW<'_, DcompCtrlSpec> {
        WtDecBypassW::new(self, 3)
    }
}
#[doc = "dcomp_ctrl\n\nYou can [`read`](crate::Reg::read) this register and get [`dcomp_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcomp_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DcompCtrlSpec;
impl crate::RegisterSpec for DcompCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcomp_ctrl::R`](R) reader structure"]
impl crate::Readable for DcompCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`dcomp_ctrl::W`](W) writer structure"]
impl crate::Writable for DcompCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DCOMP_CTRL to value 0"]
impl crate::Resettable for DcompCtrlSpec {}
