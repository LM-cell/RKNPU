#[doc = "Register `DATAOUT_SIZE_0` reader"]
pub type R = crate::R<DataoutSize0Spec>;
#[doc = "Register `DATAOUT_SIZE_0` writer"]
pub type W = crate::W<DataoutSize0Spec>;
#[doc = "Field `DATAOUT_WIDTH` reader - 激活后输出数据宽度"]
pub type DataoutWidthR = crate::FieldReader<u16>;
#[doc = "Field `DATAOUT_WIDTH` writer - 激活后输出数据宽度"]
pub type DataoutWidthW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `DATAOUT_HEIGHT` reader - 激活后输出数据高度"]
pub type DataoutHeightR = crate::FieldReader<u16>;
#[doc = "Field `DATAOUT_HEIGHT` writer - 激活后输出数据高度"]
pub type DataoutHeightW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 激活后输出数据宽度"]
    #[inline(always)]
    pub fn dataout_width(&self) -> DataoutWidthR {
        DataoutWidthR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 激活后输出数据高度"]
    #[inline(always)]
    pub fn dataout_height(&self) -> DataoutHeightR {
        DataoutHeightR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 激活后输出数据宽度"]
    #[inline(always)]
    pub fn dataout_width(&mut self) -> DataoutWidthW<'_, DataoutSize0Spec> {
        DataoutWidthW::new(self, 0)
    }
    #[doc = "Bits 16:31 - 激活后输出数据高度"]
    #[inline(always)]
    pub fn dataout_height(&mut self) -> DataoutHeightW<'_, DataoutSize0Spec> {
        DataoutHeightW::new(self, 16)
    }
}
#[doc = "dataout_size_0\n\nYou can [`read`](crate::Reg::read) this register and get [`dataout_size_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dataout_size_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DataoutSize0Spec;
impl crate::RegisterSpec for DataoutSize0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dataout_size_0::R`](R) reader structure"]
impl crate::Readable for DataoutSize0Spec {}
#[doc = "`write(|w| ..)` method takes [`dataout_size_0::W`](W) writer structure"]
impl crate::Writable for DataoutSize0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DATAOUT_SIZE_0 to value 0"]
impl crate::Resettable for DataoutSize0Spec {}
