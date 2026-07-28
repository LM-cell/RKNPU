#[doc = "Register `DATA_FORMAT` reader"]
pub type R = crate::R<DataFormatSpec>;
#[doc = "Register `DATA_FORMAT` writer"]
pub type W = crate::W<DataFormatSpec>;
#[doc = "Field `PROC_PRECISION` reader - 处理精度"]
pub type ProcPrecisionR = crate::FieldReader;
#[doc = "Field `PROC_PRECISION` writer - 处理精度"]
pub type ProcPrecisionW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DPU_FLYIN` reader - 数据来自 DPU 且 DPU 数据来自外部时置 1"]
pub type DpuFlyinR = crate::BitReader;
#[doc = "Field `DPU_FLYIN` writer - 数据来自 DPU 且 DPU 数据来自外部时置 1"]
pub type DpuFlyinW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INDEX_ADD` reader - 若 `index_en` 使能，值为 `dst_surface_stride × cube surface 数`（每 surface 8 字节），否则等于 `dst_surface_stride`"]
pub type IndexAddR = crate::FieldReader<u32>;
#[doc = "Field `INDEX_ADD` writer - 若 `index_en` 使能，值为 `dst_surface_stride × cube surface 数`（每 surface 8 字节），否则等于 `dst_surface_stride`"]
pub type IndexAddW<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    #[doc = "Bits 0:2 - 处理精度"]
    #[inline(always)]
    pub fn proc_precision(&self) -> ProcPrecisionR {
        ProcPrecisionR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - 数据来自 DPU 且 DPU 数据来自外部时置 1"]
    #[inline(always)]
    pub fn dpu_flyin(&self) -> DpuFlyinR {
        DpuFlyinR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:31 - 若 `index_en` 使能，值为 `dst_surface_stride × cube surface 数`（每 surface 8 字节），否则等于 `dst_surface_stride`"]
    #[inline(always)]
    pub fn index_add(&self) -> IndexAddR {
        IndexAddR::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:2 - 处理精度"]
    #[inline(always)]
    pub fn proc_precision(&mut self) -> ProcPrecisionW<'_, DataFormatSpec> {
        ProcPrecisionW::new(self, 0)
    }
    #[doc = "Bit 3 - 数据来自 DPU 且 DPU 数据来自外部时置 1"]
    #[inline(always)]
    pub fn dpu_flyin(&mut self) -> DpuFlyinW<'_, DataFormatSpec> {
        DpuFlyinW::new(self, 3)
    }
    #[doc = "Bits 4:31 - 若 `index_en` 使能，值为 `dst_surface_stride × cube surface 数`（每 surface 8 字节），否则等于 `dst_surface_stride`"]
    #[inline(always)]
    pub fn index_add(&mut self) -> IndexAddW<'_, DataFormatSpec> {
        IndexAddW::new(self, 4)
    }
}
#[doc = "data_format\n\nYou can [`read`](crate::Reg::read) this register and get [`data_format::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data_format::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DataFormatSpec;
impl crate::RegisterSpec for DataFormatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`data_format::R`](R) reader structure"]
impl crate::Readable for DataFormatSpec {}
#[doc = "`write(|w| ..)` method takes [`data_format::W`](W) writer structure"]
impl crate::Writable for DataFormatSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DATA_FORMAT to value 0"]
impl crate::Resettable for DataFormatSpec {}
