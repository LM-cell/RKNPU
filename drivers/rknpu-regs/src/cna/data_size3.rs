#[doc = "Register `DATA_SIZE3` reader"]
pub type R = crate::R<DataSize3Spec>;
#[doc = "Register `DATA_SIZE3` writer"]
pub type W = crate::W<DataSize3Spec>;
#[doc = "Field `DATAOUT_ATOMICS` reader - 卷积后输出总像素数"]
pub type DataoutAtomicsR = crate::FieldReader<u32>;
#[doc = "Field `DATAOUT_ATOMICS` writer - 卷积后输出总像素数"]
pub type DataoutAtomicsW<'a, REG> = crate::FieldWriter<'a, REG, 22, u32>;
#[doc = "Field `SURF_MODE` reader - Surface 串行模式。0/1：1 surf；2：2 surf；3：4 surf"]
pub type SurfModeR = crate::FieldReader;
#[doc = "Field `SURF_MODE` writer - Surface 串行模式。0/1：1 surf；2：2 surf；3：4 surf"]
pub type SurfModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:21 - 卷积后输出总像素数"]
    #[inline(always)]
    pub fn dataout_atomics(&self) -> DataoutAtomicsR {
        DataoutAtomicsR::new(self.bits & 0x003f_ffff)
    }
    #[doc = "Bits 22:23 - Surface 串行模式。0/1：1 surf；2：2 surf；3：4 surf"]
    #[inline(always)]
    pub fn surf_mode(&self) -> SurfModeR {
        SurfModeR::new(((self.bits >> 22) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:21 - 卷积后输出总像素数"]
    #[inline(always)]
    pub fn dataout_atomics(&mut self) -> DataoutAtomicsW<'_, DataSize3Spec> {
        DataoutAtomicsW::new(self, 0)
    }
    #[doc = "Bits 22:23 - Surface 串行模式。0/1：1 surf；2：2 surf；3：4 surf"]
    #[inline(always)]
    pub fn surf_mode(&mut self) -> SurfModeW<'_, DataSize3Spec> {
        SurfModeW::new(self, 22)
    }
}
#[doc = "data_size3\n\nYou can [`read`](crate::Reg::read) this register and get [`data_size3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data_size3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DataSize3Spec;
impl crate::RegisterSpec for DataSize3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`data_size3::R`](R) reader structure"]
impl crate::Readable for DataSize3Spec {}
#[doc = "`write(|w| ..)` method takes [`data_size3::W`](W) writer structure"]
impl crate::Writable for DataSize3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DATA_SIZE3 to value 0"]
impl crate::Resettable for DataSize3Spec {}
