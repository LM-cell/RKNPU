use axerrno::{AxError, AxResult};
use axtask::current;
use starry_core::task::AsThread;

/// 释放当前进程通过 `dma_malloc` 申请的一段 DMA 内存。
///
/// 注意：
/// - `user_ptr` 必须是 `dma_malloc` 原样返回的首地址
/// - 不允许传入偏移后的地址
/// - 不允许跨进程释放
pub fn sys_dma_free(user_ptr: usize) -> AxResult<isize> {
    if user_ptr == 0 {
        return Err(AxError::InvalidInput);
    }

    let curr = current();
    let proc_data = &curr.as_thread().proc_data;

    if !proc_data.release_dma_allocation(user_ptr)? {
        return Err(AxError::BadAddress);
    }

    Ok(0)
}
