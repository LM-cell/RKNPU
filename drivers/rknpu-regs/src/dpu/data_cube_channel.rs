#[doc = "Register `DATA_CUBE_CHANNEL` reader"]
pub type R = crate::R<DataCubeChannelSpec>;
#[doc = "Register `DATA_CUBE_CHANNEL` writer"]
pub type W = crate::W<DataCubeChannelSpec>;
#[doc = "Field `CHANNEL` reader - Cube 通道数"]
pub type ChannelR = crate::FieldReader<u16>;
#[doc = "Field `CHANNEL` writer - Cube 通道数"]
pub type ChannelW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
#[doc = "Field `ORIG_CHANNEL` reader - 原始输出通道数"]
pub type OrigChannelR = crate::FieldReader<u16>;
#[doc = "Field `ORIG_CHANNEL` writer - 原始输出通道数"]
pub type OrigChannelW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    #[doc = "Bits 0:12 - Cube 通道数"]
    #[inline(always)]
    pub fn channel(&self) -> ChannelR {
        ChannelR::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bits 16:28 - 原始输出通道数"]
    #[inline(always)]
    pub fn orig_channel(&self) -> OrigChannelR {
        OrigChannelR::new(((self.bits >> 16) & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:12 - Cube 通道数"]
    #[inline(always)]
    pub fn channel(&mut self) -> ChannelW<'_, DataCubeChannelSpec> {
        ChannelW::new(self, 0)
    }
    #[doc = "Bits 16:28 - 原始输出通道数"]
    #[inline(always)]
    pub fn orig_channel(&mut self) -> OrigChannelW<'_, DataCubeChannelSpec> {
        OrigChannelW::new(self, 16)
    }
}
#[doc = "data_cube_channel\n\nYou can [`read`](crate::Reg::read) this register and get [`data_cube_channel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data_cube_channel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DataCubeChannelSpec;
impl crate::RegisterSpec for DataCubeChannelSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`data_cube_channel::R`](R) reader structure"]
impl crate::Readable for DataCubeChannelSpec {}
#[doc = "`write(|w| ..)` method takes [`data_cube_channel::W`](W) writer structure"]
impl crate::Writable for DataCubeChannelSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DATA_CUBE_CHANNEL to value 0"]
impl crate::Resettable for DataCubeChannelSpec {}
