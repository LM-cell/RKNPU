#[doc = "Register `SURF_NOTCH` reader"]
pub type R = crate::R<SurfNotchSpec>;
#[doc = "Register `SURF_NOTCH` writer"]
pub type W = crate::W<SurfNotchSpec>;
#[doc = "Field `SURF_NOTCH_ADDR` reader - 当前处理特征图末尾到 shape 特征图末尾的像素数"]
pub type SurfNotchAddrR = crate::FieldReader<u32>;
#[doc = "Field `SURF_NOTCH_ADDR` writer - 当前处理特征图末尾到 shape 特征图末尾的像素数"]
pub type SurfNotchAddrW<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    #[doc = "Bits 4:31 - 当前处理特征图末尾到 shape 特征图末尾的像素数"]
    #[inline(always)]
    pub fn surf_notch_addr(&self) -> SurfNotchAddrR {
        SurfNotchAddrR::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bits 4:31 - 当前处理特征图末尾到 shape 特征图末尾的像素数"]
    #[inline(always)]
    pub fn surf_notch_addr(&mut self) -> SurfNotchAddrW<'_, SurfNotchSpec> {
        SurfNotchAddrW::new(self, 4)
    }
}
#[doc = "surf_notch\n\nYou can [`read`](crate::Reg::read) this register and get [`surf_notch::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`surf_notch::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SurfNotchSpec;
impl crate::RegisterSpec for SurfNotchSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`surf_notch::R`](R) reader structure"]
impl crate::Readable for SurfNotchSpec {}
#[doc = "`write(|w| ..)` method takes [`surf_notch::W`](W) writer structure"]
impl crate::Writable for SurfNotchSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SURF_NOTCH to value 0"]
impl crate::Resettable for SurfNotchSpec {}
