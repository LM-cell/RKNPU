#[doc = "Register `DATA_CUBE_NOTCH_ADDR` reader"]
pub type R = crate::R<DataCubeNotchAddrSpec>;
#[doc = "Register `DATA_CUBE_NOTCH_ADDR` writer"]
pub type W = crate::W<DataCubeNotchAddrSpec>;
#[doc = "Field `NOTCH_ADDR_0` reader - Notch 地址 0"]
pub type NotchAddr0R = crate::FieldReader<u16>;
#[doc = "Field `NOTCH_ADDR_0` writer - Notch 地址 0"]
pub type NotchAddr0W<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
#[doc = "Field `NOTCH_ADDR_1` reader - Notch 地址 1"]
pub type NotchAddr1R = crate::FieldReader<u16>;
#[doc = "Field `NOTCH_ADDR_1` writer - Notch 地址 1"]
pub type NotchAddr1W<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    #[doc = "Bits 0:12 - Notch 地址 0"]
    #[inline(always)]
    pub fn notch_addr_0(&self) -> NotchAddr0R {
        NotchAddr0R::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bits 16:28 - Notch 地址 1"]
    #[inline(always)]
    pub fn notch_addr_1(&self) -> NotchAddr1R {
        NotchAddr1R::new(((self.bits >> 16) & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:12 - Notch 地址 0"]
    #[inline(always)]
    pub fn notch_addr_0(&mut self) -> NotchAddr0W<'_, DataCubeNotchAddrSpec> {
        NotchAddr0W::new(self, 0)
    }
    #[doc = "Bits 16:28 - Notch 地址 1"]
    #[inline(always)]
    pub fn notch_addr_1(&mut self) -> NotchAddr1W<'_, DataCubeNotchAddrSpec> {
        NotchAddr1W::new(self, 16)
    }
}
#[doc = "data_cube_notch_addr\n\nYou can [`read`](crate::Reg::read) this register and get [`data_cube_notch_addr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data_cube_notch_addr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DataCubeNotchAddrSpec;
impl crate::RegisterSpec for DataCubeNotchAddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`data_cube_notch_addr::R`](R) reader structure"]
impl crate::Readable for DataCubeNotchAddrSpec {}
#[doc = "`write(|w| ..)` method takes [`data_cube_notch_addr::W`](W) writer structure"]
impl crate::Writable for DataCubeNotchAddrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DATA_CUBE_NOTCH_ADDR to value 0"]
impl crate::Resettable for DataCubeNotchAddrSpec {}
