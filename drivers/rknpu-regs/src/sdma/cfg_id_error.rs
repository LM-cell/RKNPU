#[doc = "Register `CFG_ID_ERROR` reader"]
pub type R = crate::R<CfgIdErrorSpec>;
#[doc = "Register `CFG_ID_ERROR` writer"]
pub type W = crate::W<CfgIdErrorSpec>;
#[doc = "Field `RD_RESP_ID` reader - 错误读 ID"]
pub type RdRespIdR = crate::FieldReader;
#[doc = "Field `RD_RESP_ID` writer - 错误读 ID"]
pub type RdRespIdW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `WR_RESP_ID` reader - 错误写 ID"]
pub type WrRespIdR = crate::FieldReader;
#[doc = "Field `WR_RESP_ID` writer - 错误写 ID"]
pub type WrRespIdW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:4 - 错误读 ID"]
    #[inline(always)]
    pub fn rd_resp_id(&self) -> RdRespIdR {
        RdRespIdR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 6:9 - 错误写 ID"]
    #[inline(always)]
    pub fn wr_resp_id(&self) -> WrRespIdR {
        WrRespIdR::new(((self.bits >> 6) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - 错误读 ID"]
    #[inline(always)]
    pub fn rd_resp_id(&mut self) -> RdRespIdW<'_, CfgIdErrorSpec> {
        RdRespIdW::new(self, 0)
    }
    #[doc = "Bits 6:9 - 错误写 ID"]
    #[inline(always)]
    pub fn wr_resp_id(&mut self) -> WrRespIdW<'_, CfgIdErrorSpec> {
        WrRespIdW::new(self, 6)
    }
}
#[doc = "cfg_id_error\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_id_error::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_id_error::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgIdErrorSpec;
impl crate::RegisterSpec for CfgIdErrorSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_id_error::R`](R) reader structure"]
impl crate::Readable for CfgIdErrorSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_id_error::W`](W) writer structure"]
impl crate::Writable for CfgIdErrorSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CFG_ID_ERROR to value 0"]
impl crate::Resettable for CfgIdErrorSpec {}
