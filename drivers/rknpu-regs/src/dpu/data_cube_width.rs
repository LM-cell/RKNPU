#[doc = "Register `DATA_CUBE_WIDTH` reader"]
pub type R = crate::R<DataCubeWidthSpec>;
#[doc = "Register `DATA_CUBE_WIDTH` writer"]
pub type W = crate::W<DataCubeWidthSpec>;
#[doc = "Field `WIDTH` reader - 输入 cube 宽度"]
pub type WidthR = crate::FieldReader<u16>;
#[doc = "Field `WIDTH` writer - 输入 cube 宽度"]
pub type WidthW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    #[doc = "Bits 0:12 - 输入 cube 宽度"]
    #[inline(always)]
    pub fn width(&self) -> WidthR {
        WidthR::new((self.bits & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:12 - 输入 cube 宽度"]
    #[inline(always)]
    pub fn width(&mut self) -> WidthW<'_, DataCubeWidthSpec> {
        WidthW::new(self, 0)
    }
}
#[doc = "data_cube_width\n\nYou can [`read`](crate::Reg::read) this register and get [`data_cube_width::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data_cube_width::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DataCubeWidthSpec;
impl crate::RegisterSpec for DataCubeWidthSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`data_cube_width::R`](R) reader structure"]
impl crate::Readable for DataCubeWidthSpec {}
#[doc = "`write(|w| ..)` method takes [`data_cube_width::W`](W) writer structure"]
impl crate::Writable for DataCubeWidthSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DATA_CUBE_WIDTH to value 0"]
impl crate::Resettable for DataCubeWidthSpec {}
