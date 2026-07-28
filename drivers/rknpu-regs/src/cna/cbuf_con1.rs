#[doc = "Register `CBUF_CON1` reader"]
pub type R = crate::R<CbufCon1Spec>;
#[doc = "Register `CBUF_CON1` writer"]
pub type W = crate::W<CbufCon1Spec>;
#[doc = "Field `DATA_ENTRIES` reader - 存储一行特征图所需的 Bank 空间"]
pub type DataEntriesR = crate::FieldReader<u16>;
#[doc = "Field `DATA_ENTRIES` writer - 存储一行特征图所需的 Bank 空间"]
pub type DataEntriesW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    #[doc = "Bits 0:12 - 存储一行特征图所需的 Bank 空间"]
    #[inline(always)]
    pub fn data_entries(&self) -> DataEntriesR {
        DataEntriesR::new((self.bits & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:12 - 存储一行特征图所需的 Bank 空间"]
    #[inline(always)]
    pub fn data_entries(&mut self) -> DataEntriesW<'_, CbufCon1Spec> {
        DataEntriesW::new(self, 0)
    }
}
#[doc = "cbuf_con1\n\nYou can [`read`](crate::Reg::read) this register and get [`cbuf_con1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cbuf_con1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CbufCon1Spec;
impl crate::RegisterSpec for CbufCon1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cbuf_con1::R`](R) reader structure"]
impl crate::Readable for CbufCon1Spec {}
#[doc = "`write(|w| ..)` method takes [`cbuf_con1::W`](W) writer structure"]
impl crate::Writable for CbufCon1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CBUF_CON1 to value 0"]
impl crate::Resettable for CbufCon1Spec {}
