#[doc = "Register `S_POINTER` reader"]
pub type R = crate::R<SPointerSpec>;
#[doc = "Register `S_POINTER` writer"]
pub type W = crate::W<SPointerSpec>;
#[doc = "Field `POINTER` reader - 当前待设置的寄存器组。0：组 0；1：组 1"]
pub type PointerR = crate::BitReader;
#[doc = "Field `POINTER` writer - 当前待设置的寄存器组。0：组 0；1：组 1"]
pub type PointerW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POINTER_PP_EN` reader - 寄存器组 ping-pong 使能。0：禁用；1：使能"]
pub type PointerPpEnR = crate::BitReader;
#[doc = "Field `POINTER_PP_EN` writer - 寄存器组 ping-pong 使能。0：禁用；1：使能"]
pub type PointerPpEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXECUTER_PP_EN` reader - 执行器组 ping-pong 使能。0：禁用；1：使能"]
pub type ExecuterPpEnR = crate::BitReader;
#[doc = "Field `EXECUTER_PP_EN` writer - 执行器组 ping-pong 使能。0：禁用；1：使能"]
pub type ExecuterPpEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POINTER_PP_MODE` reader - Ping-pong 模式。0：按执行器切换；1：按指针切换"]
pub type PointerPpModeR = crate::BitReader;
#[doc = "Field `POINTER_PP_MODE` writer - Ping-pong 模式。0：按执行器切换；1：按指针切换"]
pub type PointerPpModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POINTER_PP_CLEAR` reader - 清除寄存器组指针，写 1 清零"]
pub type PointerPpClearR = crate::BitReader;
#[doc = "Field `POINTER_PP_CLEAR` writer - 清除寄存器组指针，写 1 清零"]
pub type PointerPpClearW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `EXECUTER_PP_CLEAR` reader - 清除执行器组指针，写 1 清零"]
pub type ExecuterPpClearR = crate::BitReader;
#[doc = "Field `EXECUTER_PP_CLEAR` writer - 清除执行器组指针，写 1 清零"]
pub type ExecuterPpClearW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `EXECUTER` reader - 当前使用的寄存器组。0：执行器组 0；1：执行器组 1"]
pub type ExecuterR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - 当前待设置的寄存器组。0：组 0；1：组 1"]
    #[inline(always)]
    pub fn pointer(&self) -> PointerR {
        PointerR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 寄存器组 ping-pong 使能。0：禁用；1：使能"]
    #[inline(always)]
    pub fn pointer_pp_en(&self) -> PointerPpEnR {
        PointerPpEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 执行器组 ping-pong 使能。0：禁用；1：使能"]
    #[inline(always)]
    pub fn executer_pp_en(&self) -> ExecuterPpEnR {
        ExecuterPpEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Ping-pong 模式。0：按执行器切换；1：按指针切换"]
    #[inline(always)]
    pub fn pointer_pp_mode(&self) -> PointerPpModeR {
        PointerPpModeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 清除寄存器组指针，写 1 清零"]
    #[inline(always)]
    pub fn pointer_pp_clear(&self) -> PointerPpClearR {
        PointerPpClearR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 清除执行器组指针，写 1 清零"]
    #[inline(always)]
    pub fn executer_pp_clear(&self) -> ExecuterPpClearR {
        ExecuterPpClearR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 16 - 当前使用的寄存器组。0：执行器组 0；1：执行器组 1"]
    #[inline(always)]
    pub fn executer(&self) -> ExecuterR {
        ExecuterR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 当前待设置的寄存器组。0：组 0；1：组 1"]
    #[inline(always)]
    pub fn pointer(&mut self) -> PointerW<'_, SPointerSpec> {
        PointerW::new(self, 0)
    }
    #[doc = "Bit 1 - 寄存器组 ping-pong 使能。0：禁用；1：使能"]
    #[inline(always)]
    pub fn pointer_pp_en(&mut self) -> PointerPpEnW<'_, SPointerSpec> {
        PointerPpEnW::new(self, 1)
    }
    #[doc = "Bit 2 - 执行器组 ping-pong 使能。0：禁用；1：使能"]
    #[inline(always)]
    pub fn executer_pp_en(&mut self) -> ExecuterPpEnW<'_, SPointerSpec> {
        ExecuterPpEnW::new(self, 2)
    }
    #[doc = "Bit 3 - Ping-pong 模式。0：按执行器切换；1：按指针切换"]
    #[inline(always)]
    pub fn pointer_pp_mode(&mut self) -> PointerPpModeW<'_, SPointerSpec> {
        PointerPpModeW::new(self, 3)
    }
    #[doc = "Bit 4 - 清除寄存器组指针，写 1 清零"]
    #[inline(always)]
    pub fn pointer_pp_clear(&mut self) -> PointerPpClearW<'_, SPointerSpec> {
        PointerPpClearW::new(self, 4)
    }
    #[doc = "Bit 5 - 清除执行器组指针，写 1 清零"]
    #[inline(always)]
    pub fn executer_pp_clear(&mut self) -> ExecuterPpClearW<'_, SPointerSpec> {
        ExecuterPpClearW::new(self, 5)
    }
}
#[doc = "s_pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`s_pointer::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`s_pointer::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPointerSpec;
impl crate::RegisterSpec for SPointerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`s_pointer::R`](R) reader structure"]
impl crate::Readable for SPointerSpec {}
#[doc = "`write(|w| ..)` method takes [`s_pointer::W`](W) writer structure"]
impl crate::Writable for SPointerSpec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x30;
}
#[doc = "`reset()` method sets S_POINTER to value 0"]
impl crate::Resettable for SPointerSpec {}
