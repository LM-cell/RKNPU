#[doc = "Register `CBUF_CON0` reader"]
pub type R = crate::R<CbufCon0Spec>;
#[doc = "Register `CBUF_CON0` writer"]
pub type W = crate::W<CbufCon0Spec>;
#[doc = "Field `DATA_BANK` reader - 特征数据占用的 Bank 数。0：Bank 0；1：Bank 0-1；…；6：Bank 0-6"]
pub type DataBankR = crate::FieldReader;
#[doc = "Field `DATA_BANK` writer - 特征数据占用的 Bank 数。0：Bank 0；1：Bank 0-1；…；6：Bank 0-6"]
pub type DataBankW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `WEIGHT_BANK` reader - 权重数据占用的 Bank 数。1：Bank 7；2：Bank 6-7；…；7：Bank 1-7"]
pub type WeightBankR = crate::FieldReader;
#[doc = "Field `WEIGHT_BANK` writer - 权重数据占用的 Bank 数。1：Bank 7；2：Bank 6-7；…；7：Bank 1-7"]
pub type WeightBankW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `FC_DATA_BANK` reader - FC 零跳过模式的特征数据 Bank 数。FC 零跳过模式设为 1，否则必须为 0"]
pub type FcDataBankR = crate::FieldReader;
#[doc = "Field `FC_DATA_BANK` writer - FC 零跳过模式的特征数据 Bank 数。FC 零跳过模式设为 1，否则必须为 0"]
pub type FcDataBankW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DATA_REUSE` reader - 特征数据复用使能。0：禁用；1：使能，直接从内部缓冲取数据"]
pub type DataReuseR = crate::BitReader;
#[doc = "Field `DATA_REUSE` writer - 特征数据复用使能。0：禁用；1：使能，直接从内部缓冲取数据"]
pub type DataReuseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WEIGHT_REUSE` reader - 权重数据复用使能。0：禁用；1：使能，直接从内部缓冲取权重"]
pub type WeightReuseR = crate::BitReader;
#[doc = "Field `WEIGHT_REUSE` writer - 权重数据复用使能。0：禁用；1：使能，直接从内部缓冲取权重"]
pub type WeightReuseW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - 特征数据占用的 Bank 数。0：Bank 0；1：Bank 0-1；…；6：Bank 0-6"]
    #[inline(always)]
    pub fn data_bank(&self) -> DataBankR {
        DataBankR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - 权重数据占用的 Bank 数。1：Bank 7；2：Bank 6-7；…；7：Bank 1-7"]
    #[inline(always)]
    pub fn weight_bank(&self) -> WeightBankR {
        WeightBankR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:10 - FC 零跳过模式的特征数据 Bank 数。FC 零跳过模式设为 1，否则必须为 0"]
    #[inline(always)]
    pub fn fc_data_bank(&self) -> FcDataBankR {
        FcDataBankR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 12 - 特征数据复用使能。0：禁用；1：使能，直接从内部缓冲取数据"]
    #[inline(always)]
    pub fn data_reuse(&self) -> DataReuseR {
        DataReuseR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 权重数据复用使能。0：禁用；1：使能，直接从内部缓冲取权重"]
    #[inline(always)]
    pub fn weight_reuse(&self) -> WeightReuseR {
        WeightReuseR::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - 特征数据占用的 Bank 数。0：Bank 0；1：Bank 0-1；…；6：Bank 0-6"]
    #[inline(always)]
    pub fn data_bank(&mut self) -> DataBankW<'_, CbufCon0Spec> {
        DataBankW::new(self, 0)
    }
    #[doc = "Bits 4:7 - 权重数据占用的 Bank 数。1：Bank 7；2：Bank 6-7；…；7：Bank 1-7"]
    #[inline(always)]
    pub fn weight_bank(&mut self) -> WeightBankW<'_, CbufCon0Spec> {
        WeightBankW::new(self, 4)
    }
    #[doc = "Bits 8:10 - FC 零跳过模式的特征数据 Bank 数。FC 零跳过模式设为 1，否则必须为 0"]
    #[inline(always)]
    pub fn fc_data_bank(&mut self) -> FcDataBankW<'_, CbufCon0Spec> {
        FcDataBankW::new(self, 8)
    }
    #[doc = "Bit 12 - 特征数据复用使能。0：禁用；1：使能，直接从内部缓冲取数据"]
    #[inline(always)]
    pub fn data_reuse(&mut self) -> DataReuseW<'_, CbufCon0Spec> {
        DataReuseW::new(self, 12)
    }
    #[doc = "Bit 13 - 权重数据复用使能。0：禁用；1：使能，直接从内部缓冲取权重"]
    #[inline(always)]
    pub fn weight_reuse(&mut self) -> WeightReuseW<'_, CbufCon0Spec> {
        WeightReuseW::new(self, 13)
    }
}
#[doc = "cbuf_con0\n\nYou can [`read`](crate::Reg::read) this register and get [`cbuf_con0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cbuf_con0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CbufCon0Spec;
impl crate::RegisterSpec for CbufCon0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cbuf_con0::R`](R) reader structure"]
impl crate::Readable for CbufCon0Spec {}
#[doc = "`write(|w| ..)` method takes [`cbuf_con0::W`](W) writer structure"]
impl crate::Writable for CbufCon0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CBUF_CON0 to value 0"]
impl crate::Resettable for CbufCon0Spec {}
