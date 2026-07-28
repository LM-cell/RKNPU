#[doc = "Register `DATA_CUBE_IN_HEIGHT` reader"]
pub type R = crate::R<DataCubeInHeightSpec>;
#[doc = "Register `DATA_CUBE_IN_HEIGHT` writer"]
pub type W = crate::W<DataCubeInHeightSpec>;
#[doc = "Field `CUBE_IN_HEIGHT` reader - 池化输入高度（需减 1）"]
pub type CubeInHeightR = crate::FieldReader<u16>;
#[doc = "Field `CUBE_IN_HEIGHT` writer - 池化输入高度（需减 1）"]
pub type CubeInHeightW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    #[doc = "Bits 0:12 - 池化输入高度（需减 1）"]
    #[inline(always)]
    pub fn cube_in_height(&self) -> CubeInHeightR {
        CubeInHeightR::new((self.bits & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:12 - 池化输入高度（需减 1）"]
    #[inline(always)]
    pub fn cube_in_height(&mut self) -> CubeInHeightW<'_, DataCubeInHeightSpec> {
        CubeInHeightW::new(self, 0)
    }
}
#[doc = "data_cube_in_height\n\nYou can [`read`](crate::Reg::read) this register and get [`data_cube_in_height::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data_cube_in_height::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DataCubeInHeightSpec;
impl crate::RegisterSpec for DataCubeInHeightSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`data_cube_in_height::R`](R) reader structure"]
impl crate::Readable for DataCubeInHeightSpec {}
#[doc = "`write(|w| ..)` method takes [`data_cube_in_height::W`](W) writer structure"]
impl crate::Writable for DataCubeInHeightSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DATA_CUBE_IN_HEIGHT to value 0"]
impl crate::Resettable for DataCubeInHeightSpec {}
