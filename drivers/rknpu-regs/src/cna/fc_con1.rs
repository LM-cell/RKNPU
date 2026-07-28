#[doc = "Register `FC_CON1` reader"]
pub type R = crate::R<FcCon1Spec>;
#[doc = "Register `FC_CON1` writer"]
pub type W = crate::W<FcCon1Spec>;
#[doc = "Field `DATA_OFFSET` reader - FC 零跳过模式下的特征数据偏移"]
pub type DataOffsetR = crate::FieldReader<u32>;
#[doc = "Field `DATA_OFFSET` writer - FC 零跳过模式下的特征数据偏移"]
pub type DataOffsetW<'a, REG> = crate::FieldWriter<'a, REG, 17, u32>;
impl R {
    #[doc = "Bits 0:16 - FC 零跳过模式下的特征数据偏移"]
    #[inline(always)]
    pub fn data_offset(&self) -> DataOffsetR {
        DataOffsetR::new(self.bits & 0x0001_ffff)
    }
}
impl W {
    #[doc = "Bits 0:16 - FC 零跳过模式下的特征数据偏移"]
    #[inline(always)]
    pub fn data_offset(&mut self) -> DataOffsetW<'_, FcCon1Spec> {
        DataOffsetW::new(self, 0)
    }
}
#[doc = "fc_con1\n\nYou can [`read`](crate::Reg::read) this register and get [`fc_con1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fc_con1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FcCon1Spec;
impl crate::RegisterSpec for FcCon1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fc_con1::R`](R) reader structure"]
impl crate::Readable for FcCon1Spec {}
#[doc = "`write(|w| ..)` method takes [`fc_con1::W`](W) writer structure"]
impl crate::Writable for FcCon1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FC_CON1 to value 0"]
impl crate::Resettable for FcCon1Spec {}
