#[doc = "Register `CFG_OUTSTANDING` reader"]
pub type R = crate::R<CfgOutstandingSpec>;
#[doc = "Register `CFG_OUTSTANDING` writer"]
pub type W = crate::W<CfgOutstandingSpec>;
#[doc = "Field `RD_OS_CNT` reader - 最大读 outstanding 数"]
pub type RdOsCntR = crate::FieldReader;
#[doc = "Field `RD_OS_CNT` writer - 最大读 outstanding 数"]
pub type RdOsCntW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `WR_OS_CNT` reader - 最大写 outstanding 数"]
pub type WrOsCntR = crate::FieldReader;
#[doc = "Field `WR_OS_CNT` writer - 最大写 outstanding 数"]
pub type WrOsCntW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 最大读 outstanding 数"]
    #[inline(always)]
    pub fn rd_os_cnt(&self) -> RdOsCntR {
        RdOsCntR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 最大写 outstanding 数"]
    #[inline(always)]
    pub fn wr_os_cnt(&self) -> WrOsCntR {
        WrOsCntR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 最大读 outstanding 数"]
    #[inline(always)]
    pub fn rd_os_cnt(&mut self) -> RdOsCntW<'_, CfgOutstandingSpec> {
        RdOsCntW::new(self, 0)
    }
    #[doc = "Bits 8:15 - 最大写 outstanding 数"]
    #[inline(always)]
    pub fn wr_os_cnt(&mut self) -> WrOsCntW<'_, CfgOutstandingSpec> {
        WrOsCntW::new(self, 8)
    }
}
#[doc = "cfg_outstanding\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_outstanding::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_outstanding::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgOutstandingSpec;
impl crate::RegisterSpec for CfgOutstandingSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_outstanding::R`](R) reader structure"]
impl crate::Readable for CfgOutstandingSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_outstanding::W`](W) writer structure"]
impl crate::Writable for CfgOutstandingSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CFG_OUTSTANDING to value 0"]
impl crate::Resettable for CfgOutstandingSpec {}
