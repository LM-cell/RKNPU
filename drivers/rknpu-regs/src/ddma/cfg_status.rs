#[doc = "Register `CFG_STATUS` reader"]
pub type R = crate::R<CfgStatusSpec>;
#[doc = "Register `CFG_STATUS` writer"]
pub type W = crate::W<CfgStatusSpec>;
#[doc = "Field `IDEL` reader - 空闲状态"]
pub type IdelR = crate::BitReader;
#[doc = "Field `IDEL` writer - 空闲状态"]
pub type IdelW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 8 - 空闲状态"]
    #[inline(always)]
    pub fn idel(&self) -> IdelR {
        IdelR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - 空闲状态"]
    #[inline(always)]
    pub fn idel(&mut self) -> IdelW<'_, CfgStatusSpec> {
        IdelW::new(self, 8)
    }
}
#[doc = "cfg_status\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgStatusSpec;
impl crate::RegisterSpec for CfgStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_status::R`](R) reader structure"]
impl crate::Readable for CfgStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_status::W`](W) writer structure"]
impl crate::Writable for CfgStatusSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CFG_STATUS to value 0"]
impl crate::Resettable for CfgStatusSpec {}
