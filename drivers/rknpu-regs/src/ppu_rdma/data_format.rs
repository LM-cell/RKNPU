#[doc = "Register `DATA_FORMAT` reader"]
pub type R = crate::R<DataFormatSpec>;
#[doc = "Register `DATA_FORMAT` writer"]
pub type W = crate::W<DataFormatSpec>;
#[doc = "Field `IN_PRECISION` reader - 输入精度。0：4bit；1：8bit；2：16bit；3：32bit"]
pub type InPrecisionR = crate::FieldReader;
#[doc = "Field `IN_PRECISION` writer - 输入精度。0：4bit；1：8bit；2：16bit；3：32bit"]
pub type InPrecisionW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - 输入精度。0：4bit；1：8bit；2：16bit；3：32bit"]
    #[inline(always)]
    pub fn in_precision(&self) -> InPrecisionR {
        InPrecisionR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - 输入精度。0：4bit；1：8bit；2：16bit；3：32bit"]
    #[inline(always)]
    pub fn in_precision(&mut self) -> InPrecisionW<'_, DataFormatSpec> {
        InPrecisionW::new(self, 0)
    }
}
#[doc = "data_format\n\nYou can [`read`](crate::Reg::read) this register and get [`data_format::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data_format::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DataFormatSpec;
impl crate::RegisterSpec for DataFormatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`data_format::R`](R) reader structure"]
impl crate::Readable for DataFormatSpec {}
#[doc = "`write(|w| ..)` method takes [`data_format::W`](W) writer structure"]
impl crate::Writable for DataFormatSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DATA_FORMAT to value 0"]
impl crate::Resettable for DataFormatSpec {}
