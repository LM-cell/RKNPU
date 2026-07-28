#[doc = "Register `DATA_SIZE2` reader"]
pub type R = crate::R<DataSize2Spec>;
#[doc = "Register `DATA_SIZE2` writer"]
pub type W = crate::W<DataSize2Spec>;
#[doc = "Field `DATAOUT_WIDTH` reader - 卷积后数据宽度"]
pub type DataoutWidthR = crate::FieldReader<u16>;
#[doc = "Field `DATAOUT_WIDTH` writer - 卷积后数据宽度"]
pub type DataoutWidthW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:10 - 卷积后数据宽度"]
    #[inline(always)]
    pub fn dataout_width(&self) -> DataoutWidthR {
        DataoutWidthR::new((self.bits & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - 卷积后数据宽度"]
    #[inline(always)]
    pub fn dataout_width(&mut self) -> DataoutWidthW<'_, DataSize2Spec> {
        DataoutWidthW::new(self, 0)
    }
}
#[doc = "data_size2\n\nYou can [`read`](crate::Reg::read) this register and get [`data_size2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data_size2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DataSize2Spec;
impl crate::RegisterSpec for DataSize2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`data_size2::R`](R) reader structure"]
impl crate::Readable for DataSize2Spec {}
#[doc = "`write(|w| ..)` method takes [`data_size2::W`](W) writer structure"]
impl crate::Writable for DataSize2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DATA_SIZE2 to value 0"]
impl crate::Resettable for DataSize2Spec {}
