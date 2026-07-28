#[doc = "Register `DATA_SIZE0` reader"]
pub type R = crate::R<DataSize0Spec>;
#[doc = "Register `DATA_SIZE0` writer"]
pub type W = crate::W<DataSize0Spec>;
#[doc = "Field `DATAIN_HEIGHT` reader - 输入特征数据高度"]
pub type DatainHeightR = crate::FieldReader<u16>;
#[doc = "Field `DATAIN_HEIGHT` writer - 输入特征数据高度"]
pub type DatainHeightW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `DATAIN_WIDTH` reader - 输入特征数据宽度"]
pub type DatainWidthR = crate::FieldReader<u16>;
#[doc = "Field `DATAIN_WIDTH` writer - 输入特征数据宽度"]
pub type DatainWidthW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:10 - 输入特征数据高度"]
    #[inline(always)]
    pub fn datain_height(&self) -> DatainHeightR {
        DatainHeightR::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 16:26 - 输入特征数据宽度"]
    #[inline(always)]
    pub fn datain_width(&self) -> DatainWidthR {
        DatainWidthR::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - 输入特征数据高度"]
    #[inline(always)]
    pub fn datain_height(&mut self) -> DatainHeightW<'_, DataSize0Spec> {
        DatainHeightW::new(self, 0)
    }
    #[doc = "Bits 16:26 - 输入特征数据宽度"]
    #[inline(always)]
    pub fn datain_width(&mut self) -> DatainWidthW<'_, DataSize0Spec> {
        DatainWidthW::new(self, 16)
    }
}
#[doc = "data_size0\n\nYou can [`read`](crate::Reg::read) this register and get [`data_size0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data_size0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DataSize0Spec;
impl crate::RegisterSpec for DataSize0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`data_size0::R`](R) reader structure"]
impl crate::Readable for DataSize0Spec {}
#[doc = "`write(|w| ..)` method takes [`data_size0::W`](W) writer structure"]
impl crate::Writable for DataSize0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DATA_SIZE0 to value 0"]
impl crate::Resettable for DataSize0Spec {}
