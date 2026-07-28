#![no_std]

//! # Some Serial - 嵌入式串口驱动集合
//!
//! 本库提供统一的串口驱动接口，支持多种硬件平台：
//! - ARM PL011 UART
//! - NS16550/16450 UART（IO Port 和 MMIO 版本）
//!
//! ## 特性
//!
//! - 🏗️ 统一抽象接口 - 基于 `rdif-serial` 的统一串口抽象
//! - 🛡️ 无标准库设计 (`no_std`) - 适用于裸机和嵌入式系统
//! - 📦 模块化架构 - 每个驱动独立模块，按需选择
//! - 🔒 类型安全 - 使用 Rust 类型系统确保内存安全
//! - ⚡ 高性能 - 零拷贝数据传输，直接硬件访问
//!
//! ## 支持的驱动
//!
//! ### ARM PL011 UART
//! - 广泛用于 ARM Cortex-A、Cortex-M、Cortex-R 系列
//! - 支持 FIFO、中断、回环等完整功能
//!
//! ### NS16550/16450 UART
//! - 经典 PC 串口控制器，广泛兼容
//! - 支持 IO Port（x86_64）和 MMIO（通用）两种访问方式
//! - 支持 16 字节 FIFO 缓冲
//!
//! ## 快速开始
//!
//! ```rust
//! use some_serial::{Serial, Config};
//! use some_serial::pl011::Pl011; // ARM PL011
//! use some_serial::ns16550::Ns16550Mmio; // NS16550 MMIO
//!
//! // 选择合适的驱动
//! #[cfg(target_arch = "aarch64")]
//! let mut uart = Pl011::new(
//!     NonNull::new(0x9000000 as *mut u8).unwrap(),
//!     24_000_000
//! );
//!
//! #[cfg(not(target_arch = "aarch64"))]
//! let mut uart = Ns16550Mmio::new(
//!     NonNull::new(0x9000000 as *mut u8).unwrap(),
//!     1_843_200
//! );
//!
//! // 配置串口
//! let config = Config::new()
//!     .baudrate(115200)
//!     .data_bits(some_serial::DataBits::Eight)
//!     .stop_bits(some_serial::StopBits::One)
//!     .parity(some_serial::Parity::None);
//!
//! uart.set_config(&config).unwrap();
//! uart.open().unwrap();
//! ```

// 导入核心模块
pub mod ns16550;
pub mod pl011;

// 重新导出 rdif-serial 的所有类型
pub use rdif_serial::*;
