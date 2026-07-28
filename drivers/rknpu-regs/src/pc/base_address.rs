#[doc = "Register `BASE_ADDRESS` reader"]
pub type R = crate::R<BaseAddressSpec>;
#[doc = "Register `BASE_ADDRESS` writer"]
pub type W = crate::W<BaseAddressSpec>;
#[doc = "Field `PC_SEL` reader - PC 模式选择。0：PC 模式，通过 AXI DMA 取寄存器配置；1：Slave 模式，通过 AHB 设置寄存器"]
pub type PcSelR = crate::BitReader;
#[doc = "Field `PC_SEL` writer - PC 模式选择。0：PC 模式，通过 AXI DMA 取寄存器配置；1：Slave 模式，通过 AHB 设置寄存器"]
pub type PcSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PC_SOURCE_ADDR` reader - PC 基址。DMA 指令流所在的内存地址"]
pub type PcSourceAddrR = crate::FieldReader<u32>;
#[doc = "Field `PC_SOURCE_ADDR` writer - PC 基址。DMA 指令流所在的内存地址"]
pub type PcSourceAddrW<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    #[doc = "Bit 0 - PC 模式选择。0：PC 模式，通过 AXI DMA 取寄存器配置；1：Slave 模式，通过 AHB 设置寄存器"]
    #[inline(always)]
    pub fn pc_sel(&self) -> PcSelR {
        PcSelR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:31 - PC 基址。DMA 指令流所在的内存地址"]
    #[inline(always)]
    pub fn pc_source_addr(&self) -> PcSourceAddrR {
        PcSourceAddrR::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - PC 模式选择。0：PC 模式，通过 AXI DMA 取寄存器配置；1：Slave 模式，通过 AHB 设置寄存器"]
    #[inline(always)]
    pub fn pc_sel(&mut self) -> PcSelW<'_, BaseAddressSpec> {
        PcSelW::new(self, 0)
    }
    #[doc = "Bits 4:31 - PC 基址。DMA 指令流所在的内存地址"]
    #[inline(always)]
    pub fn pc_source_addr(&mut self) -> PcSourceAddrW<'_, BaseAddressSpec> {
        PcSourceAddrW::new(self, 4)
    }
}
#[doc = "base_address\n\nYou can [`read`](crate::Reg::read) this register and get [`base_address::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`base_address::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BaseAddressSpec;
impl crate::RegisterSpec for BaseAddressSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`base_address::R`](R) reader structure"]
impl crate::Readable for BaseAddressSpec {}
#[doc = "`write(|w| ..)` method takes [`base_address::W`](W) writer structure"]
impl crate::Writable for BaseAddressSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BASE_ADDRESS to value 0"]
impl crate::Resettable for BaseAddressSpec {}
