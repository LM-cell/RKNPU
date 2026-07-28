#[doc = "Register `CUBE_IN_CHANNEL` reader"]
pub type R = crate::R<CubeInChannelSpec>;
#[doc = "Register `CUBE_IN_CHANNEL` writer"]
pub type W = crate::W<CubeInChannelSpec>;
#[doc = "Field `CUBE_IN_CHANNEL` reader - 池化 cube 通道数（需减 1）"]
pub type CubeInChannelR = crate::FieldReader<u16>;
#[doc = "Field `CUBE_IN_CHANNEL` writer - 池化 cube 通道数（需减 1）"]
pub type CubeInChannelW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    #[doc = "Bits 0:12 - 池化 cube 通道数（需减 1）"]
    #[inline(always)]
    pub fn cube_in_channel(&self) -> CubeInChannelR {
        CubeInChannelR::new((self.bits & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:12 - 池化 cube 通道数（需减 1）"]
    #[inline(always)]
    pub fn cube_in_channel(&mut self) -> CubeInChannelW<'_, CubeInChannelSpec> {
        CubeInChannelW::new(self, 0)
    }
}
#[doc = "cube_in_channel\n\nYou can [`read`](crate::Reg::read) this register and get [`cube_in_channel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cube_in_channel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CubeInChannelSpec;
impl crate::RegisterSpec for CubeInChannelSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cube_in_channel::R`](R) reader structure"]
impl crate::Readable for CubeInChannelSpec {}
#[doc = "`write(|w| ..)` method takes [`cube_in_channel::W`](W) writer structure"]
impl crate::Writable for CubeInChannelSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CUBE_IN_CHANNEL to value 0"]
impl crate::Resettable for CubeInChannelSpec {}
