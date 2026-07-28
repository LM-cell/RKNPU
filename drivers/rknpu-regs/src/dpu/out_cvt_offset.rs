#[doc = "Register `OUT_CVT_OFFSET` reader"]
pub type R = crate::R<OutCvtOffsetSpec>;
#[doc = "Register `OUT_CVT_OFFSET` writer"]
pub type W = crate::W<OutCvtOffsetSpec>;
#[doc = "Field `OUT_CVT_OFFSET` reader - 输出转换器偏移"]
pub type OutCvtOffsetR = crate::FieldReader<u32>;
#[doc = "Field `OUT_CVT_OFFSET` writer - 输出转换器偏移"]
pub type OutCvtOffsetW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 输出转换器偏移"]
    #[inline(always)]
    pub fn out_cvt_offset(&self) -> OutCvtOffsetR {
        OutCvtOffsetR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 输出转换器偏移"]
    #[inline(always)]
    pub fn out_cvt_offset(&mut self) -> OutCvtOffsetW<'_, OutCvtOffsetSpec> {
        OutCvtOffsetW::new(self, 0)
    }
}
#[doc = "out_cvt_offset\n\nYou can [`read`](crate::Reg::read) this register and get [`out_cvt_offset::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out_cvt_offset::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OutCvtOffsetSpec;
impl crate::RegisterSpec for OutCvtOffsetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`out_cvt_offset::R`](R) reader structure"]
impl crate::Readable for OutCvtOffsetSpec {}
#[doc = "`write(|w| ..)` method takes [`out_cvt_offset::W`](W) writer structure"]
impl crate::Writable for OutCvtOffsetSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OUT_CVT_OFFSET to value 0"]
impl crate::Resettable for OutCvtOffsetSpec {}
