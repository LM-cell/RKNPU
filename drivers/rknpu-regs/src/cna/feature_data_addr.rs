#[doc = "Register `FEATURE_DATA_ADDR` reader"]
pub type R = crate::R<FeatureDataAddrSpec>;
#[doc = "Register `FEATURE_DATA_ADDR` writer"]
pub type W = crate::W<FeatureDataAddrSpec>;
#[doc = "Field `FEATURE_BASE_ADDR` reader - 特征数据地址"]
pub type FeatureBaseAddrR = crate::FieldReader<u32>;
#[doc = "Field `FEATURE_BASE_ADDR` writer - 特征数据地址"]
pub type FeatureBaseAddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 特征数据地址"]
    #[inline(always)]
    pub fn feature_base_addr(&self) -> FeatureBaseAddrR {
        FeatureBaseAddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 特征数据地址"]
    #[inline(always)]
    pub fn feature_base_addr(&mut self) -> FeatureBaseAddrW<'_, FeatureDataAddrSpec> {
        FeatureBaseAddrW::new(self, 0)
    }
}
#[doc = "feature_data_addr\n\nYou can [`read`](crate::Reg::read) this register and get [`feature_data_addr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`feature_data_addr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FeatureDataAddrSpec;
impl crate::RegisterSpec for FeatureDataAddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`feature_data_addr::R`](R) reader structure"]
impl crate::Readable for FeatureDataAddrSpec {}
#[doc = "`write(|w| ..)` method takes [`feature_data_addr::W`](W) writer structure"]
impl crate::Writable for FeatureDataAddrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FEATURE_DATA_ADDR to value 0"]
impl crate::Resettable for FeatureDataAddrSpec {}
