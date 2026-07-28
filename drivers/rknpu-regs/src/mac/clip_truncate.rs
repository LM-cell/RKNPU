#[doc = "Register `CLIP_TRUNCATE` reader"]
pub type R = crate::R<ClipTruncateSpec>;
#[doc = "Register `CLIP_TRUNCATE` writer"]
pub type W = crate::W<ClipTruncateSpec>;
#[doc = "Field `CLIP_TRUNCATE` reader - 截断位数"]
pub type ClipTruncateR = crate::FieldReader;
#[doc = "Field `CLIP_TRUNCATE` writer - 截断位数"]
pub type ClipTruncateW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `ROUND_TYPE` reader - 舍入类型。0：奇入偶不入；1：0.5 向上进 1"]
pub type RoundTypeR = crate::BitReader;
#[doc = "Field `ROUND_TYPE` writer - 舍入类型。0：奇入偶不入；1：0.5 向上进 1"]
pub type RoundTypeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:4 - 截断位数"]
    #[inline(always)]
    pub fn clip_truncate(&self) -> ClipTruncateR {
        ClipTruncateR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 6 - 舍入类型。0：奇入偶不入；1：0.5 向上进 1"]
    #[inline(always)]
    pub fn round_type(&self) -> RoundTypeR {
        RoundTypeR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - 截断位数"]
    #[inline(always)]
    pub fn clip_truncate(&mut self) -> ClipTruncateW<'_, ClipTruncateSpec> {
        ClipTruncateW::new(self, 0)
    }
    #[doc = "Bit 6 - 舍入类型。0：奇入偶不入；1：0.5 向上进 1"]
    #[inline(always)]
    pub fn round_type(&mut self) -> RoundTypeW<'_, ClipTruncateSpec> {
        RoundTypeW::new(self, 6)
    }
}
#[doc = "clip_truncate\n\nYou can [`read`](crate::Reg::read) this register and get [`clip_truncate::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clip_truncate::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClipTruncateSpec;
impl crate::RegisterSpec for ClipTruncateSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clip_truncate::R`](R) reader structure"]
impl crate::Readable for ClipTruncateSpec {}
#[doc = "`write(|w| ..)` method takes [`clip_truncate::W`](W) writer structure"]
impl crate::Writable for ClipTruncateSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CLIP_TRUNCATE to value 0"]
impl crate::Resettable for ClipTruncateSpec {}
