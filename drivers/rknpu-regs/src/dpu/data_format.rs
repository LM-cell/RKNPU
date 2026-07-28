#[doc = "Register `DATA_FORMAT` reader"]
pub type R = crate::R<DataFormatSpec>;
#[doc = "Register `DATA_FORMAT` writer"]
pub type W = crate::W<DataFormatSpec>;
#[doc = "Field `PROC_PRECISION` reader - 处理精度。编码同 `out_precision`"]
pub type ProcPrecisionR = crate::FieldReader;
#[doc = "Field `PROC_PRECISION` writer - 处理精度。编码同 `out_precision`"]
pub type ProcPrecisionW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `MC_SURF_OUT` reader - 多 surface 输出。0：每像素 16 字节对齐；1：可输出 2/4 surface 串行"]
pub type McSurfOutR = crate::BitReader;
#[doc = "Field `MC_SURF_OUT` writer - 多 surface 输出。0：每像素 16 字节对齐；1：可输出 2/4 surface 串行"]
pub type McSurfOutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BS_MUL_SHIFT_VALUE_NEG` reader - BS CORE 负数移位值"]
pub type BsMulShiftValueNegR = crate::FieldReader;
#[doc = "Field `BS_MUL_SHIFT_VALUE_NEG` writer - BS CORE 负数移位值"]
pub type BsMulShiftValueNegW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `BN_MUL_SHIFT_VALUE_NEG` reader - BN CORE 负数移位值"]
pub type BnMulShiftValueNegR = crate::FieldReader;
#[doc = "Field `BN_MUL_SHIFT_VALUE_NEG` writer - BN CORE 负数移位值"]
pub type BnMulShiftValueNegW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `EW_TRUNCATE_NEG` reader - EW CORE 负数移位值"]
pub type EwTruncateNegR = crate::FieldReader<u16>;
#[doc = "Field `EW_TRUNCATE_NEG` writer - EW CORE 负数移位值"]
pub type EwTruncateNegW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `IN_PRECISION` reader - 输入精度（同 DPU_RDMA）。编码同上"]
pub type InPrecisionR = crate::FieldReader;
#[doc = "Field `IN_PRECISION` writer - 输入精度（同 DPU_RDMA）。编码同上"]
pub type InPrecisionW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `OUT_PRECISION` reader - 输出精度。0：int8；1：int16；2：fp16；3：bf16；4：int32；5：fp32；6：int4"]
pub type OutPrecisionR = crate::FieldReader;
#[doc = "Field `OUT_PRECISION` writer - 输出精度。0：int8；1：int16；2：fp16；3：bf16；4：int32；5：fp32；6：int4"]
pub type OutPrecisionW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - 处理精度。编码同 `out_precision`"]
    #[inline(always)]
    pub fn proc_precision(&self) -> ProcPrecisionR {
        ProcPrecisionR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - 多 surface 输出。0：每像素 16 字节对齐；1：可输出 2/4 surface 串行"]
    #[inline(always)]
    pub fn mc_surf_out(&self) -> McSurfOutR {
        McSurfOutR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:9 - BS CORE 负数移位值"]
    #[inline(always)]
    pub fn bs_mul_shift_value_neg(&self) -> BsMulShiftValueNegR {
        BsMulShiftValueNegR::new(((self.bits >> 4) & 0x3f) as u8)
    }
    #[doc = "Bits 10:15 - BN CORE 负数移位值"]
    #[inline(always)]
    pub fn bn_mul_shift_value_neg(&self) -> BnMulShiftValueNegR {
        BnMulShiftValueNegR::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bits 16:25 - EW CORE 负数移位值"]
    #[inline(always)]
    pub fn ew_truncate_neg(&self) -> EwTruncateNegR {
        EwTruncateNegR::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    #[doc = "Bits 26:28 - 输入精度（同 DPU_RDMA）。编码同上"]
    #[inline(always)]
    pub fn in_precision(&self) -> InPrecisionR {
        InPrecisionR::new(((self.bits >> 26) & 7) as u8)
    }
    #[doc = "Bits 29:31 - 输出精度。0：int8；1：int16；2：fp16；3：bf16；4：int32；5：fp32；6：int4"]
    #[inline(always)]
    pub fn out_precision(&self) -> OutPrecisionR {
        OutPrecisionR::new(((self.bits >> 29) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - 处理精度。编码同 `out_precision`"]
    #[inline(always)]
    pub fn proc_precision(&mut self) -> ProcPrecisionW<'_, DataFormatSpec> {
        ProcPrecisionW::new(self, 0)
    }
    #[doc = "Bit 3 - 多 surface 输出。0：每像素 16 字节对齐；1：可输出 2/4 surface 串行"]
    #[inline(always)]
    pub fn mc_surf_out(&mut self) -> McSurfOutW<'_, DataFormatSpec> {
        McSurfOutW::new(self, 3)
    }
    #[doc = "Bits 4:9 - BS CORE 负数移位值"]
    #[inline(always)]
    pub fn bs_mul_shift_value_neg(&mut self) -> BsMulShiftValueNegW<'_, DataFormatSpec> {
        BsMulShiftValueNegW::new(self, 4)
    }
    #[doc = "Bits 10:15 - BN CORE 负数移位值"]
    #[inline(always)]
    pub fn bn_mul_shift_value_neg(&mut self) -> BnMulShiftValueNegW<'_, DataFormatSpec> {
        BnMulShiftValueNegW::new(self, 10)
    }
    #[doc = "Bits 16:25 - EW CORE 负数移位值"]
    #[inline(always)]
    pub fn ew_truncate_neg(&mut self) -> EwTruncateNegW<'_, DataFormatSpec> {
        EwTruncateNegW::new(self, 16)
    }
    #[doc = "Bits 26:28 - 输入精度（同 DPU_RDMA）。编码同上"]
    #[inline(always)]
    pub fn in_precision(&mut self) -> InPrecisionW<'_, DataFormatSpec> {
        InPrecisionW::new(self, 26)
    }
    #[doc = "Bits 29:31 - 输出精度。0：int8；1：int16；2：fp16；3：bf16；4：int32；5：fp32；6：int4"]
    #[inline(always)]
    pub fn out_precision(&mut self) -> OutPrecisionW<'_, DataFormatSpec> {
        OutPrecisionW::new(self, 29)
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
