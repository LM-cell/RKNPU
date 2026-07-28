#[doc = "Register `LUT_ACCESS_DATA` reader"]
pub type R = crate::R<LutAccessDataSpec>;
#[doc = "Register `LUT_ACCESS_DATA` writer"]
pub type W = crate::W<LutAccessDataSpec>;
#[doc = "Field `LUT_ACCESS_DATA` reader - LUT 访问数据"]
pub type LutAccessDataR = crate::FieldReader<u16>;
#[doc = "Field `LUT_ACCESS_DATA` writer - LUT 访问数据"]
pub type LutAccessDataW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - LUT 访问数据"]
    #[inline(always)]
    pub fn lut_access_data(&self) -> LutAccessDataR {
        LutAccessDataR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - LUT 访问数据"]
    #[inline(always)]
    pub fn lut_access_data(&mut self) -> LutAccessDataW<'_, LutAccessDataSpec> {
        LutAccessDataW::new(self, 0)
    }
}
#[doc = "lut_access_data\n\nYou can [`read`](crate::Reg::read) this register and get [`lut_access_data::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lut_access_data::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LutAccessDataSpec;
impl crate::RegisterSpec for LutAccessDataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lut_access_data::R`](R) reader structure"]
impl crate::Readable for LutAccessDataSpec {}
#[doc = "`write(|w| ..)` method takes [`lut_access_data::W`](W) writer structure"]
impl crate::Writable for LutAccessDataSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LUT_ACCESS_DATA to value 0"]
impl crate::Resettable for LutAccessDataSpec {}
