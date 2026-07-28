#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    s_status: SStatus,
    s_pointer: SPointer,
    operation_enable: OperationEnable,
    cube_in_width: CubeInWidth,
    cube_in_height: CubeInHeight,
    cube_in_channel: CubeInChannel,
    _reserved6: [u8; 0x04],
    src_base_addr: SrcBaseAddr,
    _reserved7: [u8; 0x04],
    src_line_stride: SrcLineStride,
    src_surf_stride: SrcSurfStride,
    _reserved9: [u8; 0x04],
    data_format: DataFormat,
}
impl RegisterBlock {
    #[doc = "0x00 - s_status"]
    #[inline(always)]
    pub const fn s_status(&self) -> &SStatus {
        &self.s_status
    }
    #[doc = "0x04 - s_pointer"]
    #[inline(always)]
    pub const fn s_pointer(&self) -> &SPointer {
        &self.s_pointer
    }
    #[doc = "0x08 - operation_enable"]
    #[inline(always)]
    pub const fn operation_enable(&self) -> &OperationEnable {
        &self.operation_enable
    }
    #[doc = "0x0c - cube_in_width"]
    #[inline(always)]
    pub const fn cube_in_width(&self) -> &CubeInWidth {
        &self.cube_in_width
    }
    #[doc = "0x10 - cube_in_height"]
    #[inline(always)]
    pub const fn cube_in_height(&self) -> &CubeInHeight {
        &self.cube_in_height
    }
    #[doc = "0x14 - cube_in_channel"]
    #[inline(always)]
    pub const fn cube_in_channel(&self) -> &CubeInChannel {
        &self.cube_in_channel
    }
    #[doc = "0x1c - src_base_addr"]
    #[inline(always)]
    pub const fn src_base_addr(&self) -> &SrcBaseAddr {
        &self.src_base_addr
    }
    #[doc = "0x24 - src_line_stride"]
    #[inline(always)]
    pub const fn src_line_stride(&self) -> &SrcLineStride {
        &self.src_line_stride
    }
    #[doc = "0x28 - src_surf_stride"]
    #[inline(always)]
    pub const fn src_surf_stride(&self) -> &SrcSurfStride {
        &self.src_surf_stride
    }
    #[doc = "0x30 - data_format"]
    #[inline(always)]
    pub const fn data_format(&self) -> &DataFormat {
        &self.data_format
    }
}
#[doc = "S_STATUS (r) register accessor: s_status\n\nYou can [`read`](crate::Reg::read) this register and get [`s_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s_status`] module"]
#[doc(alias = "S_STATUS")]
pub type SStatus = crate::Reg<s_status::SStatusSpec>;
#[doc = "s_status"]
pub mod s_status;
#[doc = "S_POINTER (rw) register accessor: s_pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`s_pointer::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`s_pointer::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s_pointer`] module"]
#[doc(alias = "S_POINTER")]
pub type SPointer = crate::Reg<s_pointer::SPointerSpec>;
#[doc = "s_pointer"]
pub mod s_pointer;
#[doc = "OPERATION_ENABLE (rw) register accessor: operation_enable\n\nYou can [`read`](crate::Reg::read) this register and get [`operation_enable::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`operation_enable::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@operation_enable`] module"]
#[doc(alias = "OPERATION_ENABLE")]
pub type OperationEnable = crate::Reg<operation_enable::OperationEnableSpec>;
#[doc = "operation_enable"]
pub mod operation_enable;
#[doc = "CUBE_IN_WIDTH (rw) register accessor: cube_in_width\n\nYou can [`read`](crate::Reg::read) this register and get [`cube_in_width::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cube_in_width::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cube_in_width`] module"]
#[doc(alias = "CUBE_IN_WIDTH")]
pub type CubeInWidth = crate::Reg<cube_in_width::CubeInWidthSpec>;
#[doc = "cube_in_width"]
pub mod cube_in_width;
#[doc = "CUBE_IN_HEIGHT (rw) register accessor: cube_in_height\n\nYou can [`read`](crate::Reg::read) this register and get [`cube_in_height::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cube_in_height::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cube_in_height`] module"]
#[doc(alias = "CUBE_IN_HEIGHT")]
pub type CubeInHeight = crate::Reg<cube_in_height::CubeInHeightSpec>;
#[doc = "cube_in_height"]
pub mod cube_in_height;
#[doc = "CUBE_IN_CHANNEL (rw) register accessor: cube_in_channel\n\nYou can [`read`](crate::Reg::read) this register and get [`cube_in_channel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cube_in_channel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cube_in_channel`] module"]
#[doc(alias = "CUBE_IN_CHANNEL")]
pub type CubeInChannel = crate::Reg<cube_in_channel::CubeInChannelSpec>;
#[doc = "cube_in_channel"]
pub mod cube_in_channel;
#[doc = "SRC_BASE_ADDR (rw) register accessor: src_base_addr\n\nYou can [`read`](crate::Reg::read) this register and get [`src_base_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`src_base_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@src_base_addr`] module"]
#[doc(alias = "SRC_BASE_ADDR")]
pub type SrcBaseAddr = crate::Reg<src_base_addr::SrcBaseAddrSpec>;
#[doc = "src_base_addr"]
pub mod src_base_addr;
#[doc = "SRC_LINE_STRIDE (rw) register accessor: src_line_stride\n\nYou can [`read`](crate::Reg::read) this register and get [`src_line_stride::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`src_line_stride::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@src_line_stride`] module"]
#[doc(alias = "SRC_LINE_STRIDE")]
pub type SrcLineStride = crate::Reg<src_line_stride::SrcLineStrideSpec>;
#[doc = "src_line_stride"]
pub mod src_line_stride;
#[doc = "SRC_SURF_STRIDE (rw) register accessor: src_surf_stride\n\nYou can [`read`](crate::Reg::read) this register and get [`src_surf_stride::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`src_surf_stride::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@src_surf_stride`] module"]
#[doc(alias = "SRC_SURF_STRIDE")]
pub type SrcSurfStride = crate::Reg<src_surf_stride::SrcSurfStrideSpec>;
#[doc = "src_surf_stride"]
pub mod src_surf_stride;
#[doc = "DATA_FORMAT (rw) register accessor: data_format\n\nYou can [`read`](crate::Reg::read) this register and get [`data_format::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data_format::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data_format`] module"]
#[doc(alias = "DATA_FORMAT")]
pub type DataFormat = crate::Reg<data_format::DataFormatSpec>;
#[doc = "data_format"]
pub mod data_format;
