#[doc = "Register `BRDMA_CFG` reader"]
pub type R = crate::R<BrdmaCfgSpec>;
#[doc = "Register `BRDMA_CFG` writer"]
pub type W = crate::W<BrdmaCfgSpec>;
#[doc = "Field `BRDMA_DATA_USE` reader - 读取数据类型。\\[0\\]：ALU 操作数；\\[1\\]：CPEND 操作数；\\[2\\]：MUL 操作数；\\[3\\]：TRT 操作数"]
pub type BrdmaDataUseR = crate::FieldReader;
#[doc = "Field `BRDMA_DATA_USE` writer - 读取数据类型。\\[0\\]：ALU 操作数；\\[1\\]：CPEND 操作数；\\[2\\]：MUL 操作数；\\[3\\]：TRT 操作数"]
pub type BrdmaDataUseW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 1:4 - 读取数据类型。\\[0\\]：ALU 操作数；\\[1\\]：CPEND 操作数；\\[2\\]：MUL 操作数；\\[3\\]：TRT 操作数"]
    #[inline(always)]
    pub fn brdma_data_use(&self) -> BrdmaDataUseR {
        BrdmaDataUseR::new(((self.bits >> 1) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 1:4 - 读取数据类型。\\[0\\]：ALU 操作数；\\[1\\]：CPEND 操作数；\\[2\\]：MUL 操作数；\\[3\\]：TRT 操作数"]
    #[inline(always)]
    pub fn brdma_data_use(&mut self) -> BrdmaDataUseW<'_, BrdmaCfgSpec> {
        BrdmaDataUseW::new(self, 1)
    }
}
#[doc = "brdma_cfg\n\nYou can [`read`](crate::Reg::read) this register and get [`brdma_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`brdma_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BrdmaCfgSpec;
impl crate::RegisterSpec for BrdmaCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`brdma_cfg::R`](R) reader structure"]
impl crate::Readable for BrdmaCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`brdma_cfg::W`](W) writer structure"]
impl crate::Writable for BrdmaCfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BRDMA_CFG to value 0"]
impl crate::Resettable for BrdmaCfgSpec {}
