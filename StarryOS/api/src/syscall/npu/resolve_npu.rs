use axerrno::{AxError, AxResult};

/// `resolve_npu` 还没有在这一轮实现。
///
/// 当前先只打通 DMA 分配链路，真正的 NPU 协处理器调度后续再补。
pub fn sys_resolve_npu(_submit_dma_ptr: usize, _arg1: usize, _arg2: usize) -> AxResult<isize> {
    Err(AxError::Unsupported)
}
