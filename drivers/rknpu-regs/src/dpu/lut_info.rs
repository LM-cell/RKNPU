#[doc = "Register `LUT_INFO` reader"]
pub type R = crate::R<LutInfoSpec>;
#[doc = "Register `LUT_INFO` writer"]
pub type W = crate::W<LutInfoSpec>;
#[doc = "Field `LUT_LE_INDEX_SELECT` reader - LE LUT 索引选择"]
pub type LutLeIndexSelectR = crate::FieldReader;
#[doc = "Field `LUT_LE_INDEX_SELECT` writer - LE LUT 索引选择"]
pub type LutLeIndexSelectW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `LUT_LO_INDEX_SELECT` reader - LO LUT 索引选择（索引生成器中选择哪些位作为索引）"]
pub type LutLoIndexSelectR = crate::FieldReader;
#[doc = "Field `LUT_LO_INDEX_SELECT` writer - LO LUT 索引选择（索引生成器中选择哪些位作为索引）"]
pub type LutLoIndexSelectW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 8:15 - LE LUT 索引选择"]
    #[inline(always)]
    pub fn lut_le_index_select(&self) -> LutLeIndexSelectR {
        LutLeIndexSelectR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - LO LUT 索引选择（索引生成器中选择哪些位作为索引）"]
    #[inline(always)]
    pub fn lut_lo_index_select(&self) -> LutLoIndexSelectR {
        LutLoIndexSelectR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 8:15 - LE LUT 索引选择"]
    #[inline(always)]
    pub fn lut_le_index_select(&mut self) -> LutLeIndexSelectW<'_, LutInfoSpec> {
        LutLeIndexSelectW::new(self, 8)
    }
    #[doc = "Bits 16:23 - LO LUT 索引选择（索引生成器中选择哪些位作为索引）"]
    #[inline(always)]
    pub fn lut_lo_index_select(&mut self) -> LutLoIndexSelectW<'_, LutInfoSpec> {
        LutLoIndexSelectW::new(self, 16)
    }
}
#[doc = "lut_info\n\nYou can [`read`](crate::Reg::read) this register and get [`lut_info::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lut_info::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LutInfoSpec;
impl crate::RegisterSpec for LutInfoSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lut_info::R`](R) reader structure"]
impl crate::Readable for LutInfoSpec {}
#[doc = "`write(|w| ..)` method takes [`lut_info::W`](W) writer structure"]
impl crate::Writable for LutInfoSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LUT_INFO to value 0"]
impl crate::Resettable for LutInfoSpec {}
