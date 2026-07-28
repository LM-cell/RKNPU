#[doc = "Register `DATA_SIZE1` reader"]
pub type R = crate::R<DataSize1Spec>;
#[doc = "Register `DATA_SIZE1` writer"]
pub type W = crate::W<DataSize1Spec>;
#[doc = "Field `DATAIN_CHANNEL` reader - 输入通道数。int8 须为 8 的整数倍；int16/fp16 须为 4 的整数倍"]
pub type DatainChannelR = crate::FieldReader<u16>;
#[doc = "Field `DATAIN_CHANNEL` writer - 输入通道数。int8 须为 8 的整数倍；int16/fp16 须为 4 的整数倍"]
pub type DatainChannelW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `DATAIN_CHANNEL_REAL` reader - 真实通道数。当输入通道不是 8（int8）或 4（int16/fp16）的整数倍时，设置此字段"]
pub type DatainChannelRealR = crate::FieldReader<u16>;
#[doc = "Field `DATAIN_CHANNEL_REAL` writer - 真实通道数。当输入通道不是 8（int8）或 4（int16/fp16）的整数倍时，设置此字段"]
pub type DatainChannelRealW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    #[doc = "Bits 0:15 - 输入通道数。int8 须为 8 的整数倍；int16/fp16 须为 4 的整数倍"]
    #[inline(always)]
    pub fn datain_channel(&self) -> DatainChannelR {
        DatainChannelR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:29 - 真实通道数。当输入通道不是 8（int8）或 4（int16/fp16）的整数倍时，设置此字段"]
    #[inline(always)]
    pub fn datain_channel_real(&self) -> DatainChannelRealR {
        DatainChannelRealR::new(((self.bits >> 16) & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 输入通道数。int8 须为 8 的整数倍；int16/fp16 须为 4 的整数倍"]
    #[inline(always)]
    pub fn datain_channel(&mut self) -> DatainChannelW<'_, DataSize1Spec> {
        DatainChannelW::new(self, 0)
    }
    #[doc = "Bits 16:29 - 真实通道数。当输入通道不是 8（int8）或 4（int16/fp16）的整数倍时，设置此字段"]
    #[inline(always)]
    pub fn datain_channel_real(&mut self) -> DatainChannelRealW<'_, DataSize1Spec> {
        DatainChannelRealW::new(self, 16)
    }
}
#[doc = "data_size1\n\nYou can [`read`](crate::Reg::read) this register and get [`data_size1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data_size1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DataSize1Spec;
impl crate::RegisterSpec for DataSize1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`data_size1::R`](R) reader structure"]
impl crate::Readable for DataSize1Spec {}
#[doc = "`write(|w| ..)` method takes [`data_size1::W`](W) writer structure"]
impl crate::Writable for DataSize1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DATA_SIZE1 to value 0"]
impl crate::Resettable for DataSize1Spec {}
