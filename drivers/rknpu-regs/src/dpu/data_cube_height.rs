#[doc = "Register `DATA_CUBE_HEIGHT` reader"]
pub type R = crate::R<DataCubeHeightSpec>;
#[doc = "Register `DATA_CUBE_HEIGHT` writer"]
pub type W = crate::W<DataCubeHeightSpec>;
#[doc = "Field `HEIGHT` reader - 输入 cube 高度"]
pub type HeightR = crate::FieldReader<u16>;
#[doc = "Field `HEIGHT` writer - 输入 cube 高度"]
pub type HeightW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
#[doc = "Field `MINMAX_CTL` reader - MinMax 配置。\\[0\\]：使能；\\[1\\]：类型；\\[2\\]：仅概率"]
pub type MinmaxCtlR = crate::FieldReader;
#[doc = "Field `MINMAX_CTL` writer - MinMax 配置。\\[0\\]：使能；\\[1\\]：类型；\\[2\\]：仅概率"]
pub type MinmaxCtlW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:12 - 输入 cube 高度"]
    #[inline(always)]
    pub fn height(&self) -> HeightR {
        HeightR::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bits 22:24 - MinMax 配置。\\[0\\]：使能；\\[1\\]：类型；\\[2\\]：仅概率"]
    #[inline(always)]
    pub fn minmax_ctl(&self) -> MinmaxCtlR {
        MinmaxCtlR::new(((self.bits >> 22) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:12 - 输入 cube 高度"]
    #[inline(always)]
    pub fn height(&mut self) -> HeightW<'_, DataCubeHeightSpec> {
        HeightW::new(self, 0)
    }
    #[doc = "Bits 22:24 - MinMax 配置。\\[0\\]：使能；\\[1\\]：类型；\\[2\\]：仅概率"]
    #[inline(always)]
    pub fn minmax_ctl(&mut self) -> MinmaxCtlW<'_, DataCubeHeightSpec> {
        MinmaxCtlW::new(self, 22)
    }
}
#[doc = "data_cube_height\n\nYou can [`read`](crate::Reg::read) this register and get [`data_cube_height::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data_cube_height::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DataCubeHeightSpec;
impl crate::RegisterSpec for DataCubeHeightSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`data_cube_height::R`](R) reader structure"]
impl crate::Readable for DataCubeHeightSpec {}
#[doc = "`write(|w| ..)` method takes [`data_cube_height::W`](W) writer structure"]
impl crate::Writable for DataCubeHeightSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DATA_CUBE_HEIGHT to value 0"]
impl crate::Resettable for DataCubeHeightSpec {}
