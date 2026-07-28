#[doc = "Register `DATA_CUBE_OUT_HEIGHT` reader"]
pub type R = crate::R<DataCubeOutHeightSpec>;
#[doc = "Register `DATA_CUBE_OUT_HEIGHT` writer"]
pub type W = crate::W<DataCubeOutHeightSpec>;
#[doc = "Field `CUBE_OUT_HEIGHT` reader - 池化输出高度（需减 1）"]
pub type CubeOutHeightR = crate::FieldReader<u16>;
#[doc = "Field `CUBE_OUT_HEIGHT` writer - 池化输出高度（需减 1）"]
pub type CubeOutHeightW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    #[doc = "Bits 0:12 - 池化输出高度（需减 1）"]
    #[inline(always)]
    pub fn cube_out_height(&self) -> CubeOutHeightR {
        CubeOutHeightR::new((self.bits & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:12 - 池化输出高度（需减 1）"]
    #[inline(always)]
    pub fn cube_out_height(&mut self) -> CubeOutHeightW<'_, DataCubeOutHeightSpec> {
        CubeOutHeightW::new(self, 0)
    }
}
#[doc = "data_cube_out_height\n\nYou can [`read`](crate::Reg::read) this register and get [`data_cube_out_height::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data_cube_out_height::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DataCubeOutHeightSpec;
impl crate::RegisterSpec for DataCubeOutHeightSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`data_cube_out_height::R`](R) reader structure"]
impl crate::Readable for DataCubeOutHeightSpec {}
#[doc = "`write(|w| ..)` method takes [`data_cube_out_height::W`](W) writer structure"]
impl crate::Writable for DataCubeOutHeightSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DATA_CUBE_OUT_HEIGHT to value 0"]
impl crate::Resettable for DataCubeOutHeightSpec {}
