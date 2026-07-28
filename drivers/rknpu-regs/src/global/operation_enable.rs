#[doc = "Register `OPERATION_ENABLE` reader"]
pub type R = crate::R<OperationEnableSpec>;
#[doc = "Register `OPERATION_ENABLE` writer"]
pub type W = crate::W<OperationEnableSpec>;
#[doc = "Field `CNA_OP_EN` reader - CNA 操作使能"]
pub type CnaOpEnR = crate::BitReader;
#[doc = "Field `CNA_OP_EN` writer - CNA 操作使能"]
pub type CnaOpEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CORE_OP_EN` reader - CORE 操作使能"]
pub type CoreOpEnR = crate::BitReader;
#[doc = "Field `CORE_OP_EN` writer - CORE 操作使能"]
pub type CoreOpEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DPU_OP_EN` reader - DPU 操作使能"]
pub type DpuOpEnR = crate::BitReader;
#[doc = "Field `DPU_OP_EN` writer - DPU 操作使能"]
pub type DpuOpEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DPU_RDMA_OP_EN` reader - DPU_RDMA 操作使能"]
pub type DpuRdmaOpEnR = crate::BitReader;
#[doc = "Field `DPU_RDMA_OP_EN` writer - DPU_RDMA 操作使能"]
pub type DpuRdmaOpEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PPU_OP_EN` reader - PPU 操作使能"]
pub type PpuOpEnR = crate::BitReader;
#[doc = "Field `PPU_OP_EN` writer - PPU 操作使能"]
pub type PpuOpEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PPU_RDMA_OP_EN` reader - PPU_RDMA 操作使能"]
pub type PpuRdmaOpEnR = crate::BitReader;
#[doc = "Field `PPU_RDMA_OP_EN` writer - PPU_RDMA 操作使能"]
pub type PpuRdmaOpEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - CNA 操作使能"]
    #[inline(always)]
    pub fn cna_op_en(&self) -> CnaOpEnR {
        CnaOpEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - CORE 操作使能"]
    #[inline(always)]
    pub fn core_op_en(&self) -> CoreOpEnR {
        CoreOpEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DPU 操作使能"]
    #[inline(always)]
    pub fn dpu_op_en(&self) -> DpuOpEnR {
        DpuOpEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - DPU_RDMA 操作使能"]
    #[inline(always)]
    pub fn dpu_rdma_op_en(&self) -> DpuRdmaOpEnR {
        DpuRdmaOpEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PPU 操作使能"]
    #[inline(always)]
    pub fn ppu_op_en(&self) -> PpuOpEnR {
        PpuOpEnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PPU_RDMA 操作使能"]
    #[inline(always)]
    pub fn ppu_rdma_op_en(&self) -> PpuRdmaOpEnR {
        PpuRdmaOpEnR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CNA 操作使能"]
    #[inline(always)]
    pub fn cna_op_en(&mut self) -> CnaOpEnW<'_, OperationEnableSpec> {
        CnaOpEnW::new(self, 0)
    }
    #[doc = "Bit 2 - CORE 操作使能"]
    #[inline(always)]
    pub fn core_op_en(&mut self) -> CoreOpEnW<'_, OperationEnableSpec> {
        CoreOpEnW::new(self, 2)
    }
    #[doc = "Bit 3 - DPU 操作使能"]
    #[inline(always)]
    pub fn dpu_op_en(&mut self) -> DpuOpEnW<'_, OperationEnableSpec> {
        DpuOpEnW::new(self, 3)
    }
    #[doc = "Bit 4 - DPU_RDMA 操作使能"]
    #[inline(always)]
    pub fn dpu_rdma_op_en(&mut self) -> DpuRdmaOpEnW<'_, OperationEnableSpec> {
        DpuRdmaOpEnW::new(self, 4)
    }
    #[doc = "Bit 5 - PPU 操作使能"]
    #[inline(always)]
    pub fn ppu_op_en(&mut self) -> PpuOpEnW<'_, OperationEnableSpec> {
        PpuOpEnW::new(self, 5)
    }
    #[doc = "Bit 6 - PPU_RDMA 操作使能"]
    #[inline(always)]
    pub fn ppu_rdma_op_en(&mut self) -> PpuRdmaOpEnW<'_, OperationEnableSpec> {
        PpuRdmaOpEnW::new(self, 6)
    }
}
#[doc = "operation_enable\n\nYou can [`read`](crate::Reg::read) this register and get [`operation_enable::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`operation_enable::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OperationEnableSpec;
impl crate::RegisterSpec for OperationEnableSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`operation_enable::R`](R) reader structure"]
impl crate::Readable for OperationEnableSpec {}
#[doc = "`write(|w| ..)` method takes [`operation_enable::W`](W) writer structure"]
impl crate::Writable for OperationEnableSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OPERATION_ENABLE to value 0"]
impl crate::Resettable for OperationEnableSpec {}
