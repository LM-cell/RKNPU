#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    version: Version,
    version_num: VersionNum,
    operation_enable: OperationEnable,
    _reserved3: [u8; 0x04],
    base_address: BaseAddress,
    register_amounts: RegisterAmounts,
    _reserved5: [u8; 0x08],
    interrupt_mask: InterruptMask,
    interrupt_clear: InterruptClear,
    interrupt_status: InterruptStatus,
    interrupt_raw_status: InterruptRawStatus,
    task_con: TaskCon,
    task_dma_base_addr: TaskDmaBaseAddr,
    _reserved11: [u8; 0x04],
    task_status: TaskStatus,
}
impl RegisterBlock {
    #[doc = "0x00 - version"]
    #[inline(always)]
    pub const fn version(&self) -> &Version {
        &self.version
    }
    #[doc = "0x04 - version_num"]
    #[inline(always)]
    pub const fn version_num(&self) -> &VersionNum {
        &self.version_num
    }
    #[doc = "0x08 - operation_enable"]
    #[inline(always)]
    pub const fn operation_enable(&self) -> &OperationEnable {
        &self.operation_enable
    }
    #[doc = "0x10 - base_address"]
    #[inline(always)]
    pub const fn base_address(&self) -> &BaseAddress {
        &self.base_address
    }
    #[doc = "0x14 - register_amounts"]
    #[inline(always)]
    pub const fn register_amounts(&self) -> &RegisterAmounts {
        &self.register_amounts
    }
    #[doc = "0x20 - interrupt_mask"]
    #[inline(always)]
    pub const fn interrupt_mask(&self) -> &InterruptMask {
        &self.interrupt_mask
    }
    #[doc = "0x24 - interrupt_clear"]
    #[inline(always)]
    pub const fn interrupt_clear(&self) -> &InterruptClear {
        &self.interrupt_clear
    }
    #[doc = "0x28 - interrupt_status"]
    #[inline(always)]
    pub const fn interrupt_status(&self) -> &InterruptStatus {
        &self.interrupt_status
    }
    #[doc = "0x2c - interrupt_raw_status"]
    #[inline(always)]
    pub const fn interrupt_raw_status(&self) -> &InterruptRawStatus {
        &self.interrupt_raw_status
    }
    #[doc = "0x30 - task_con"]
    #[inline(always)]
    pub const fn task_con(&self) -> &TaskCon {
        &self.task_con
    }
    #[doc = "0x34 - task_dma_base_addr"]
    #[inline(always)]
    pub const fn task_dma_base_addr(&self) -> &TaskDmaBaseAddr {
        &self.task_dma_base_addr
    }
    #[doc = "0x3c - task_status"]
    #[inline(always)]
    pub const fn task_status(&self) -> &TaskStatus {
        &self.task_status
    }
}
#[doc = "VERSION (r) register accessor: version\n\nYou can [`read`](crate::Reg::read) this register and get [`version::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@version`] module"]
#[doc(alias = "VERSION")]
pub type Version = crate::Reg<version::VersionSpec>;
#[doc = "version"]
pub mod version;
#[doc = "VERSION_NUM (r) register accessor: version_num\n\nYou can [`read`](crate::Reg::read) this register and get [`version_num::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@version_num`] module"]
#[doc(alias = "VERSION_NUM")]
pub type VersionNum = crate::Reg<version_num::VersionNumSpec>;
#[doc = "version_num"]
pub mod version_num;
#[doc = "OPERATION_ENABLE (rw) register accessor: operation_enable\n\nYou can [`read`](crate::Reg::read) this register and get [`operation_enable::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`operation_enable::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@operation_enable`] module"]
#[doc(alias = "OPERATION_ENABLE")]
pub type OperationEnable = crate::Reg<operation_enable::OperationEnableSpec>;
#[doc = "operation_enable"]
pub mod operation_enable;
#[doc = "BASE_ADDRESS (rw) register accessor: base_address\n\nYou can [`read`](crate::Reg::read) this register and get [`base_address::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`base_address::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@base_address`] module"]
#[doc(alias = "BASE_ADDRESS")]
pub type BaseAddress = crate::Reg<base_address::BaseAddressSpec>;
#[doc = "base_address"]
pub mod base_address;
#[doc = "REGISTER_AMOUNTS (rw) register accessor: register_amounts\n\nYou can [`read`](crate::Reg::read) this register and get [`register_amounts::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`register_amounts::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@register_amounts`] module"]
#[doc(alias = "REGISTER_AMOUNTS")]
pub type RegisterAmounts = crate::Reg<register_amounts::RegisterAmountsSpec>;
#[doc = "register_amounts"]
pub mod register_amounts;
#[doc = "INTERRUPT_MASK (rw) register accessor: interrupt_mask\n\nYou can [`read`](crate::Reg::read) this register and get [`interrupt_mask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`interrupt_mask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@interrupt_mask`] module"]
#[doc(alias = "INTERRUPT_MASK")]
pub type InterruptMask = crate::Reg<interrupt_mask::InterruptMaskSpec>;
#[doc = "interrupt_mask"]
pub mod interrupt_mask;
#[doc = "INTERRUPT_CLEAR (rw) register accessor: interrupt_clear\n\nYou can [`read`](crate::Reg::read) this register and get [`interrupt_clear::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`interrupt_clear::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@interrupt_clear`] module"]
#[doc(alias = "INTERRUPT_CLEAR")]
pub type InterruptClear = crate::Reg<interrupt_clear::InterruptClearSpec>;
#[doc = "interrupt_clear"]
pub mod interrupt_clear;
#[doc = "INTERRUPT_STATUS (rw) register accessor: interrupt_status\n\nYou can [`read`](crate::Reg::read) this register and get [`interrupt_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`interrupt_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@interrupt_status`] module"]
#[doc(alias = "INTERRUPT_STATUS")]
pub type InterruptStatus = crate::Reg<interrupt_status::InterruptStatusSpec>;
#[doc = "interrupt_status"]
pub mod interrupt_status;
#[doc = "INTERRUPT_RAW_STATUS (rw) register accessor: interrupt_raw_status\n\nYou can [`read`](crate::Reg::read) this register and get [`interrupt_raw_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`interrupt_raw_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@interrupt_raw_status`] module"]
#[doc(alias = "INTERRUPT_RAW_STATUS")]
pub type InterruptRawStatus = crate::Reg<interrupt_raw_status::InterruptRawStatusSpec>;
#[doc = "interrupt_raw_status"]
pub mod interrupt_raw_status;
#[doc = "TASK_CON (rw) register accessor: task_con\n\nYou can [`read`](crate::Reg::read) this register and get [`task_con::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`task_con::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@task_con`] module"]
#[doc(alias = "TASK_CON")]
pub type TaskCon = crate::Reg<task_con::TaskConSpec>;
#[doc = "task_con"]
pub mod task_con;
#[doc = "TASK_DMA_BASE_ADDR (rw) register accessor: task_dma_base_addr\n\nYou can [`read`](crate::Reg::read) this register and get [`task_dma_base_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`task_dma_base_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@task_dma_base_addr`] module"]
#[doc(alias = "TASK_DMA_BASE_ADDR")]
pub type TaskDmaBaseAddr = crate::Reg<task_dma_base_addr::TaskDmaBaseAddrSpec>;
#[doc = "task_dma_base_addr"]
pub mod task_dma_base_addr;
#[doc = "TASK_STATUS (rw) register accessor: task_status\n\nYou can [`read`](crate::Reg::read) this register and get [`task_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`task_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@task_status`] module"]
#[doc(alias = "TASK_STATUS")]
pub type TaskStatus = crate::Reg<task_status::TaskStatusSpec>;
#[doc = "task_status"]
pub mod task_status;
