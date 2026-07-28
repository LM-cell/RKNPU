#[doc = "Register `NRDMA_CFG` reader"]
pub type R = crate::R<NrdmaCfgSpec>;
#[doc = "Register `NRDMA_CFG` writer"]
pub type W = crate::W<NrdmaCfgSpec>;
#[doc = "Field `NRDMA_DATA_USE` reader - 读取数据类型。\\[0\\]：ALU 操作数；\\[1\\]：CPEND 操作数（固定为 0，BN 无 CPEND）；\\[2\\]：MUL 操作数；\\[3\\]：TRT 操作数"]
pub type NrdmaDataUseR = crate::FieldReader;
#[doc = "Field `NRDMA_DATA_USE` writer - 读取数据类型。\\[0\\]：ALU 操作数；\\[1\\]：CPEND 操作数（固定为 0，BN 无 CPEND）；\\[2\\]：MUL 操作数；\\[3\\]：TRT 操作数"]
pub type NrdmaDataUseW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 1:4 - 读取数据类型。\\[0\\]：ALU 操作数；\\[1\\]：CPEND 操作数（固定为 0，BN 无 CPEND）；\\[2\\]：MUL 操作数；\\[3\\]：TRT 操作数"]
    #[inline(always)]
    pub fn nrdma_data_use(&self) -> NrdmaDataUseR {
        NrdmaDataUseR::new(((self.bits >> 1) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 1:4 - 读取数据类型。\\[0\\]：ALU 操作数；\\[1\\]：CPEND 操作数（固定为 0，BN 无 CPEND）；\\[2\\]：MUL 操作数；\\[3\\]：TRT 操作数"]
    #[inline(always)]
    pub fn nrdma_data_use(&mut self) -> NrdmaDataUseW<'_, NrdmaCfgSpec> {
        NrdmaDataUseW::new(self, 1)
    }
}
#[doc = "nrdma_cfg\n\nYou can [`read`](crate::Reg::read) this register and get [`nrdma_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nrdma_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NrdmaCfgSpec;
impl crate::RegisterSpec for NrdmaCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`nrdma_cfg::R`](R) reader structure"]
impl crate::Readable for NrdmaCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`nrdma_cfg::W`](W) writer structure"]
impl crate::Writable for NrdmaCfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets NRDMA_CFG to value 0"]
impl crate::Resettable for NrdmaCfgSpec {}
