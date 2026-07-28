#[doc = "Register `DATA_CUBE_OUT_WIDTH` reader"]
pub type R = crate::R<DataCubeOutWidthSpec>;
#[doc = "Register `DATA_CUBE_OUT_WIDTH` writer"]
pub type W = crate::W<DataCubeOutWidthSpec>;
#[doc = "Field `CUBE_OUT_WIDTH` reader - 池化输出宽度（需减 1）"]
pub type CubeOutWidthR = crate::FieldReader<u16>;
#[doc = "Field `CUBE_OUT_WIDTH` writer - 池化输出宽度（需减 1）"]
pub type CubeOutWidthW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    #[doc = "Bits 0:12 - 池化输出宽度（需减 1）"]
    #[inline(always)]
    pub fn cube_out_width(&self) -> CubeOutWidthR {
        CubeOutWidthR::new((self.bits & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:12 - 池化输出宽度（需减 1）"]
    #[inline(always)]
    pub fn cube_out_width(&mut self) -> CubeOutWidthW<'_, DataCubeOutWidthSpec> {
        CubeOutWidthW::new(self, 0)
    }
}
#[doc = "data_cube_out_width\n\nYou can [`read`](crate::Reg::read) this register and get [`data_cube_out_width::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data_cube_out_width::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DataCubeOutWidthSpec;
impl crate::RegisterSpec for DataCubeOutWidthSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`data_cube_out_width::R`](R) reader structure"]
impl crate::Readable for DataCubeOutWidthSpec {}
#[doc = "`write(|w| ..)` method takes [`data_cube_out_width::W`](W) writer structure"]
impl crate::Writable for DataCubeOutWidthSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DATA_CUBE_OUT_WIDTH to value 0"]
impl crate::Resettable for DataCubeOutWidthSpec {}
