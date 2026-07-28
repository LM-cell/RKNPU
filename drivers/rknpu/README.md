# rknpu

`rknpu` 是一个 `no_std` 的 Rockchip NPU（RK3588）驱动 crate，提供两层能力：

- 底层 `Rknpu`：MMIO 寄存器访问、任务下发、IRQ 完成回收、DMA/GEM 内存管理。
- 高层 `service`：封装 RKNPU ioctl 协议和阻塞提交调度，把 OS 适配点收敛到 trait。

## 功能范围

- RK3588 多核 NPU 驱动对象创建与硬件初始化入口。
- `RknpuTask` / `RknpuSubmit` / `RknpuMem*` ioctl ABI 结构。
- `submit_ioctrl_step()` 单步 dispatch 原语（每次下发一个 task 到一个 core）。
- IRQ completion 影子状态发布与 harvest。
- GEM/DMA 缓冲区 `create/map/destroy/sync`。
- `RknpuService<P>` 阻塞提交调度与 ioctl 处理。

## 代码结构

```text
src/
├── lib.rs                // 顶层驱动对象 Rknpu
├── ioctrl.rs             // ioctl ABI 与 submit step
├── gem.rs                // DMA/GEM 内存池
├── job.rs                // job/submit 辅助结构
├── registers/            // 寄存器访问包装
├── status/               // 中断与状态码
├── task/                 // 任务结构与 op 描述
├── osal.rs               // OS 抽象边界
└── service/
    ├── mod.rs            // RknpuService 入口
    ├── ioctl.rs          // ioctl 分发与 payload copy
    ├── platform.rs       // 平台 trait 定义
    ├── scheduler.rs      // 阻塞提交调度器
    └── error.rs          // service 错误类型
```

## 集成方式

### 1. 创建 `Rknpu`

平台先完成 MMIO 映射，再把每个 core 的基地址传入 `Rknpu::new()`：

```rust
use core::ptr::NonNull;
use rknpu::{Rknpu, RknpuConfig, RknpuType};

let base_addrs: [NonNull<u8>; 3] = [/* mapped MMIO ptrs */];
let config = RknpuConfig { rknpu_type: RknpuType::Rk3588 };
let mut npu = Rknpu::new(&base_addrs, config);
npu.open()?;
```

### 2. 注册 IRQ 并启用中断等待

给每个 core 注册中断回调，回调里调用 `RknpuIrqHandler::handle()`。
在支持的平台可启用中断辅助等待：

```rust
npu.set_wait_fn(platform_wait_fn);
```

`platform_wait_fn` 需要在中断到来时返回（例如 WFI/yield 路径）。

### 3. 推荐通过 `RknpuService<P>` 接入 OS

`service` 层把与 OS 耦合的点抽成 trait，平台实现后即可复用统一 ioctl 语义：

- `RknpuDeviceAccess`
- `RknpuUserMemory`
- `RknpuSubmitWaiter`
- `RknpuWorkerSignal`
- `RknpuSchedulerRuntime`

然后使用：

```rust
use rknpu::service::RknpuService;
let service = RknpuService::new(platform);
```

调用：

```rust
service.driver_ioctl(op, arg)?;
```

## ioctl 命令

`service::RknpuCmd` 当前支持：

- `Action`
- `Submit`
- `MemCreate`
- `MemMap`
- `MemDestroy`
- `MemSync`

其中 `Submit` 为阻塞语义：调用线程在 per-submit waiter 上等待，后台 worker 负责 dispatch/harvest。

## 构建与检查

```bash
cargo check
```

## 备注

- 依赖了本地路径 crate：`rknpu-regs = { path = "../rknpu-regs" }`。
- `src/lib.rs` 里有 `#[cfg(feature = "starryos")]` 分支，但当前 `Cargo.toml` 未声明该 feature；如果要启用这条分支，需要先在 features 中补齐定义。
