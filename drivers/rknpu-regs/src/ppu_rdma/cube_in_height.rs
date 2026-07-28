#[doc = "Register `CUBE_IN_HEIGHT` reader"]
pub type R = crate::R<CubeInHeightSpec>;
#[doc = "Register `CUBE_IN_HEIGHT` writer"]
pub type W = crate::W<CubeInHeightSpec>;
#[doc = "Field `CUBE_IN_HEIGHT` reader - 池化 cube 高度（需减 1）"]
pub type CubeInHeightR = crate::FieldReader<u16>;
#[doc = "Field `CUBE_IN_HEIGHT` writer - 池化 cube 高度（需减 1）"]
pub type CubeInHeightW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    #[doc = "Bits 0:12 - 池化 cube 高度（需减 1）"]
    #[inline(always)]
    pub fn cube_in_height(&self) -> CubeInHeightR {
        CubeInHeightR::new((self.bits & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:12 - 池化 cube 高度（需减 1）"]
    #[inline(always)]
    pub fn cube_in_height(&mut self) -> CubeInHeightW<'_, CubeInHeightSpec> {
        CubeInHeightW::new(self, 0)
    }
}
#[doc = "cube_in_height\n\nYou can [`read`](crate::Reg::read) this register and get [`cube_in_height::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cube_in_height::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CubeInHeightSpec;
impl crate::RegisterSpec for CubeInHeightSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cube_in_height::R`](R) reader structure"]
impl crate::Readable for CubeInHeightSpec {}
#[doc = "`write(|w| ..)` method takes [`cube_in_height::W`](W) writer structure"]
impl crate::Writable for CubeInHeightSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CUBE_IN_HEIGHT to value 0"]
impl crate::Resettable for CubeInHeightSpec {}
