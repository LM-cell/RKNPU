# rknpu-starry-adapter

`rknpu-starry-adapter` 是一个 **StarryOS <-> rknpu** 的桥接层（adapter）：

- 向上对接 StarryOS 的驱动注册/IRQ/MMIO/电源域框架（`rdrive` / `axklib` / `rockchip-pm`）
- 向下复用 `rknpu` 的核心能力（`Rknpu`、`RknpuIrqHandler`、提交与中断处理）

这个 crate 的职责是“接线”和“平台适配”，不是重写 NPU 算法逻辑。

## 设计定位

- `npuprobe.rs`：FDT 探测 + MMIO 映射 + IRQ 注册 + 驱动发布
- `irq.rs`：将平台 IRQ 回调转发到 `RknpuIrqHandler`
- `power.rs`：上电流程（NPU/NPUTOP/NPU1/NPU2）
- `tool.rs`：`iomap` 等平台工具胶水

## extern crate 约定（明确要求）

本适配层采用 `extern crate` 的方式引入关键依赖（`no_std` 场景下可读性和边界更清晰）：

```rust
#![no_std]

extern crate alloc;
#[macro_use]
extern crate log;
extern crate rknpu;
extern crate rdrive;
extern crate rockchip_pm;
extern crate axklib;
#[cfg(target_arch = "aarch64")]
extern crate axtask;
```

## 防止 extern 被优化掉：KeepAlive 宏

当开启 LTO/裁剪时，如果某个外部 crate 只被“间接使用”，可能被链接器激进裁剪。  
建议使用一个 `#[used]` 锚点宏显式保活符号：

```rust
#[macro_export]
macro_rules! keep_extern_symbol {
    ($keep_fn:ident, $keep_static:ident, $sym:path) => {
        #[inline(never)]
        fn $keep_fn() {
            let _ = $sym;
        }

        #[used]
        static $keep_static: fn() = $keep_fn;
    };
}

// 示例：保活来自 extern crate 的符号
keep_extern_symbol!(
    __keep_rdrive_get_one,
    __KEEP_RDRIVE_GET_ONE,
    rdrive::get_one::<rockchip_pm::RockchipPM>
);
```

说明：

- `#[used]` 保证静态锚点不会在编译期被移除
- 通过函数内 `let _ = $sym;` 建立对 extern 符号的显式引用
- 宏参数拆为函数名/静态名，避免重名冲突

## 编译建议

- 该 crate 应作为独立包编译：`cargo check --manifest-path drivers/rknpu-starry-adapter/Cargo.toml`
- 若目标是 `aarch64`，需确保 `axtask` 与 StarryOS 运行时环境可用
# rknpu-starry-adapter
