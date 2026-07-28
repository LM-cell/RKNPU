#![no_std]

extern crate alloc;
#[macro_use]
extern crate log;
extern crate axklib;
extern crate rdrive;
extern crate rknpu;
extern crate rockchip_pm;
#[cfg(target_arch = "aarch64")]
extern crate axtask;

pub use core::ptr::NonNull;
pub use rknpu::{Rknpu, RknpuConfig, RknpuIrqHandler, RknpuType};

pub mod card0;
pub mod card1;
pub mod devfs;
pub mod drm;

#[path = "../irq.rs"]
pub mod irq;
#[path = "../npuprobe.rs"]
pub mod npuprobe;
#[path = "../power.rs"]
pub mod power;
#[path = "../tool.rs"]
pub mod tool;

pub use npuprobe::rknpu_probe;
pub use power::enable_pm;
