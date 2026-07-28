#[doc = "Register `CUBE_IN_WIDTH` reader"]
pub type R = crate::R<CubeInWidthSpec>;
#[doc = "Register `CUBE_IN_WIDTH` writer"]
pub type W = crate::W<CubeInWidthSpec>;
#[doc = "Field `CUBE_IN_WIDTH` reader - 池化 cube 宽度（需减 1）"]
pub type CubeInWidthR = crate::FieldReader<u16>;
#[doc = "Field `CUBE_IN_WIDTH` writer - 池化 cube 宽度（需减 1）"]
pub type CubeInWidthW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    #[doc = "Bits 0:12 - 池化 cube 宽度（需减 1）"]
    #[inline(always)]
    pub fn cube_in_width(&self) -> CubeInWidthR {
        CubeInWidthR::new((self.bits & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:12 - 池化 cube 宽度（需减 1）"]
    #[inline(always)]
    pub fn cube_in_width(&mut self) -> CubeInWidthW<'_, CubeInWidthSpec> {
        CubeInWidthW::new(self, 0)
    }
}
#[doc = "cube_in_width\n\nYou can [`read`](crate::Reg::read) this register and get [`cube_in_width::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cube_in_width::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CubeInWidthSpec;
impl crate::RegisterSpec for CubeInWidthSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cube_in_width::R`](R) reader structure"]
impl crate::Readable for CubeInWidthSpec {}
#[doc = "`write(|w| ..)` method takes [`cube_in_width::W`](W) writer structure"]
impl crate::Writable for CubeInWidthSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CUBE_IN_WIDTH to value 0"]
impl crate::Resettable for CubeInWidthSpec {}
