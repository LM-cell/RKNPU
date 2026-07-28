//! NPU 上下文占位定义。
//!
//! 当前这一轮只先打通 DMA 分配链路，因此这里只保留一个最小可编译的
//! NPU 上下文结构，供线程结构体挂接。后续真正实现 NPU 抢占/恢复时，
//! 再把完整寄存器快照和状态机补进来。

/// 保存线程未来需要恢复的 NPU 状态。
#[derive(Debug, Clone, Default)]
pub struct NpuContext {
    /// 标记当前上下文里是否已经记录了待恢复的 NPU 状态。
    has_saved_state: bool,
}

impl NpuContext {
    /// 创建一个空的 NPU 上下文。
    pub const fn new() -> Self {
        Self {
            has_saved_state: false,
        }
    }

    /// 清空已经保存的 NPU 状态。
    pub fn clear(&mut self) {
        self.has_saved_state = false;
    }

    /// 判断当前上下文是否为空。
    pub const fn is_empty(&self) -> bool {
        !self.has_saved_state
    }
}

/// NPU 上下文相关错误。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NpuContextError {
    /// 当前内核版本还没有真正实现 NPU 上下文保存/恢复。
    Unsupported,
}
