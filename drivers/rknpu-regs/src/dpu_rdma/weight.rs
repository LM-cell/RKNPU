#[doc = "Register `WEIGHT` reader"]
pub type R = crate::R<WeightSpec>;
#[doc = "Register `WEIGHT` writer"]
pub type W = crate::W<WeightSpec>;
#[doc = "Field `M_WEIGHT` reader - MRDMA 仲裁权重"]
pub type MWeightR = crate::FieldReader;
#[doc = "Field `M_WEIGHT` writer - MRDMA 仲裁权重"]
pub type MWeightW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `B_WEIGHT` reader - BRDMA 仲裁权重"]
pub type BWeightR = crate::FieldReader;
#[doc = "Field `B_WEIGHT` writer - BRDMA 仲裁权重"]
pub type BWeightW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `N_WEIGHT` reader - NRDMA 仲裁权重"]
pub type NWeightR = crate::FieldReader;
#[doc = "Field `N_WEIGHT` writer - NRDMA 仲裁权重"]
pub type NWeightW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `E_WEIGHT` reader - ERDMA 仲裁权重"]
pub type EWeightR = crate::FieldReader;
#[doc = "Field `E_WEIGHT` writer - ERDMA 仲裁权重"]
pub type EWeightW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - MRDMA 仲裁权重"]
    #[inline(always)]
    pub fn m_weight(&self) -> MWeightR {
        MWeightR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - BRDMA 仲裁权重"]
    #[inline(always)]
    pub fn b_weight(&self) -> BWeightR {
        BWeightR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - NRDMA 仲裁权重"]
    #[inline(always)]
    pub fn n_weight(&self) -> NWeightR {
        NWeightR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - ERDMA 仲裁权重"]
    #[inline(always)]
    pub fn e_weight(&self) -> EWeightR {
        EWeightR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - MRDMA 仲裁权重"]
    #[inline(always)]
    pub fn m_weight(&mut self) -> MWeightW<'_, WeightSpec> {
        MWeightW::new(self, 0)
    }
    #[doc = "Bits 8:15 - BRDMA 仲裁权重"]
    #[inline(always)]
    pub fn b_weight(&mut self) -> BWeightW<'_, WeightSpec> {
        BWeightW::new(self, 8)
    }
    #[doc = "Bits 16:23 - NRDMA 仲裁权重"]
    #[inline(always)]
    pub fn n_weight(&mut self) -> NWeightW<'_, WeightSpec> {
        NWeightW::new(self, 16)
    }
    #[doc = "Bits 24:31 - ERDMA 仲裁权重"]
    #[inline(always)]
    pub fn e_weight(&mut self) -> EWeightW<'_, WeightSpec> {
        EWeightW::new(self, 24)
    }
}
#[doc = "weight\n\nYou can [`read`](crate::Reg::read) this register and get [`weight::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`weight::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WeightSpec;
impl crate::RegisterSpec for WeightSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`weight::R`](R) reader structure"]
impl crate::Readable for WeightSpec {}
#[doc = "`write(|w| ..)` method takes [`weight::W`](W) writer structure"]
impl crate::Writable for WeightSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WEIGHT to value 0"]
impl crate::Resettable for WeightSpec {}
