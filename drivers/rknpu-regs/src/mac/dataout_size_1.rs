#[doc = "Register `DATAOUT_SIZE_1` reader"]
pub type R = crate::R<DataoutSize1Spec>;
#[doc = "Register `DATAOUT_SIZE_1` writer"]
pub type W = crate::W<DataoutSize1Spec>;
#[doc = "Field `DATAOUT_CHANNEL` reader - 激活后输出数据通道数"]
pub type DataoutChannelR = crate::FieldReader<u16>;
#[doc = "Field `DATAOUT_CHANNEL` writer - 激活后输出数据通道数"]
pub type DataoutChannelW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 激活后输出数据通道数"]
    #[inline(always)]
    pub fn dataout_channel(&self) -> DataoutChannelR {
        DataoutChannelR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 激活后输出数据通道数"]
    #[inline(always)]
    pub fn dataout_channel(&mut self) -> DataoutChannelW<'_, DataoutSize1Spec> {
        DataoutChannelW::new(self, 0)
    }
}
#[doc = "dataout_size_1\n\nYou can [`read`](crate::Reg::read) this register and get [`dataout_size_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dataout_size_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DataoutSize1Spec;
impl crate::RegisterSpec for DataoutSize1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dataout_size_1::R`](R) reader structure"]
impl crate::Readable for DataoutSize1Spec {}
#[doc = "`write(|w| ..)` method takes [`dataout_size_1::W`](W) writer structure"]
impl crate::Writable for DataoutSize1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DATAOUT_SIZE_1 to value 0"]
impl crate::Resettable for DataoutSize1Spec {}
