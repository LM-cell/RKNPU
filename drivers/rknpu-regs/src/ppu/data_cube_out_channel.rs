#[doc = "Register `DATA_CUBE_OUT_CHANNEL` reader"]
pub type R = crate::R<DataCubeOutChannelSpec>;
#[doc = "Register `DATA_CUBE_OUT_CHANNEL` writer"]
pub type W = crate::W<DataCubeOutChannelSpec>;
#[doc = "Field `CUBE_OUT_CHANNEL` reader - 池化输出通道数（需减 1）"]
pub type CubeOutChannelR = crate::FieldReader<u16>;
#[doc = "Field `CUBE_OUT_CHANNEL` writer - 池化输出通道数（需减 1）"]
pub type CubeOutChannelW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    #[doc = "Bits 0:12 - 池化输出通道数（需减 1）"]
    #[inline(always)]
    pub fn cube_out_channel(&self) -> CubeOutChannelR {
        CubeOutChannelR::new((self.bits & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:12 - 池化输出通道数（需减 1）"]
    #[inline(always)]
    pub fn cube_out_channel(&mut self) -> CubeOutChannelW<'_, DataCubeOutChannelSpec> {
        CubeOutChannelW::new(self, 0)
    }
}
#[doc = "data_cube_out_channel\n\nYou can [`read`](crate::Reg::read) this register and get [`data_cube_out_channel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data_cube_out_channel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DataCubeOutChannelSpec;
impl crate::RegisterSpec for DataCubeOutChannelSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`data_cube_out_channel::R`](R) reader structure"]
impl crate::Readable for DataCubeOutChannelSpec {}
#[doc = "`write(|w| ..)` method takes [`data_cube_out_channel::W`](W) writer structure"]
impl crate::Writable for DataCubeOutChannelSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DATA_CUBE_OUT_CHANNEL to value 0"]
impl crate::Resettable for DataCubeOutChannelSpec {}
