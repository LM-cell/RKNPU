#[doc = "Register `DATA_CUBE_HEIGHT` reader"]
pub type R = crate::R<DataCubeHeightSpec>;
#[doc = "Register `DATA_CUBE_HEIGHT` writer"]
pub type W = crate::W<DataCubeHeightSpec>;
#[doc = "Field `HEIGHT` reader - 输入特征高度（需减 1）"]
pub type HeightR = crate::FieldReader<u16>;
#[doc = "Field `HEIGHT` writer - 输入特征高度（需减 1）"]
pub type HeightW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
#[doc = "Field `EW_LINE_NOTCH_ADDR` reader - EW 行 notch"]
pub type EwLineNotchAddrR = crate::FieldReader<u16>;
#[doc = "Field `EW_LINE_NOTCH_ADDR` writer - EW 行 notch"]
pub type EwLineNotchAddrW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    #[doc = "Bits 0:12 - 输入特征高度（需减 1）"]
    #[inline(always)]
    pub fn height(&self) -> HeightR {
        HeightR::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bits 16:28 - EW 行 notch"]
    #[inline(always)]
    pub fn ew_line_notch_addr(&self) -> EwLineNotchAddrR {
        EwLineNotchAddrR::new(((self.bits >> 16) & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:12 - 输入特征高度（需减 1）"]
    #[inline(always)]
    pub fn height(&mut self) -> HeightW<'_, DataCubeHeightSpec> {
        HeightW::new(self, 0)
    }
    #[doc = "Bits 16:28 - EW 行 notch"]
    #[inline(always)]
    pub fn ew_line_notch_addr(&mut self) -> EwLineNotchAddrW<'_, DataCubeHeightSpec> {
        EwLineNotchAddrW::new(self, 16)
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
