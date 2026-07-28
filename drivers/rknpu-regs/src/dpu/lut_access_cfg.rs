#[doc = "Register `LUT_ACCESS_CFG` reader"]
pub type R = crate::R<LutAccessCfgSpec>;
#[doc = "Register `LUT_ACCESS_CFG` writer"]
pub type W = crate::W<LutAccessCfgSpec>;
#[doc = "Field `LUT_ADDR` reader - 访问地址"]
pub type LutAddrR = crate::FieldReader<u16>;
#[doc = "Field `LUT_ADDR` writer - 访问地址"]
pub type LutAddrW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `LUT_TABLE_ID` reader - 访问 ID。0：LE LUT；1：LO LUT"]
pub type LutTableIdR = crate::BitReader;
#[doc = "Field `LUT_TABLE_ID` writer - 访问 ID。0：LE LUT；1：LO LUT"]
pub type LutTableIdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LUT_ACCESS_TYPE` reader - 访问类型。0：读；1：写"]
pub type LutAccessTypeR = crate::BitReader;
#[doc = "Field `LUT_ACCESS_TYPE` writer - 访问类型。0：读；1：写"]
pub type LutAccessTypeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:9 - 访问地址"]
    #[inline(always)]
    pub fn lut_addr(&self) -> LutAddrR {
        LutAddrR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 16 - 访问 ID。0：LE LUT；1：LO LUT"]
    #[inline(always)]
    pub fn lut_table_id(&self) -> LutTableIdR {
        LutTableIdR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 访问类型。0：读；1：写"]
    #[inline(always)]
    pub fn lut_access_type(&self) -> LutAccessTypeR {
        LutAccessTypeR::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - 访问地址"]
    #[inline(always)]
    pub fn lut_addr(&mut self) -> LutAddrW<'_, LutAccessCfgSpec> {
        LutAddrW::new(self, 0)
    }
    #[doc = "Bit 16 - 访问 ID。0：LE LUT；1：LO LUT"]
    #[inline(always)]
    pub fn lut_table_id(&mut self) -> LutTableIdW<'_, LutAccessCfgSpec> {
        LutTableIdW::new(self, 16)
    }
    #[doc = "Bit 17 - 访问类型。0：读；1：写"]
    #[inline(always)]
    pub fn lut_access_type(&mut self) -> LutAccessTypeW<'_, LutAccessCfgSpec> {
        LutAccessTypeW::new(self, 17)
    }
}
#[doc = "lut_access_cfg\n\nYou can [`read`](crate::Reg::read) this register and get [`lut_access_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lut_access_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LutAccessCfgSpec;
impl crate::RegisterSpec for LutAccessCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lut_access_cfg::R`](R) reader structure"]
impl crate::Readable for LutAccessCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`lut_access_cfg::W`](W) writer structure"]
impl crate::Writable for LutAccessCfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LUT_ACCESS_CFG to value 0"]
impl crate::Resettable for LutAccessCfgSpec {}
