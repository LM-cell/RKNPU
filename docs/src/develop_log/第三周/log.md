# 第三周开发日志（2.8-2.14）

## 工作总结

- **修复 Axvisor 上 rsext4 文件系统导致的磁盘数据不一致问题** — 客户机写文件后重启出现 "block is free" 错误，定位为 rsext4 缓存未同步，在写操作后增加 `sync_to_disk` 调用修复 [![PR #368](https://img.shields.io/badge/PR-%23368_merged-8957e5?logo=github)](https://github.com/arceos-hypervisor/axvisor/pull/368)
- **svd2rust 寄存器库重构** — 基于 svd2rust 工具生成类型安全的寄存器访问库，重构 rknpu 驱动核心，减少手动操作寄存器的出错概率 [![rknpu-regs](https://img.shields.io/badge/📦_rknpu--regs-寄存器库-blue)](../../../drivers/rknpu-regs/) [![rknpu](https://img.shields.io/badge/📦_rknpu-NPU驱动-blue)](../../../drivers/rknpu/)
- **辅助注释与文档** — 利用 AI 为驱动代码添加详细注释，兼顾学习与开发效率
- **基于 WFI 的异步中断处理** — 实现了 NPU 中断的异步等待机制

---

## 修复 Axvisor 上 rsext4 文件系统磁盘数据不一致问题

> [![PR rgba(51, 95, 136, 1)](https://img.shields.io/badge/PR-%23368_merged-8957e5?logo=github)](https://github.com/arceos-hypervisor/axvisor/pull/368)

### 问题

1. 客户机执行文件写操作后重启，出现 `block is free`（双重释放，原来没有正确把`1`写回位图） （位图实际扫描使用的Block和超级块中的Block不一致）错误，磁盘数据损坏
2. 如果启动的是 Linux 客户机，Linux 在启动过程中会因文件系统完整性校验失败而无法启动

### 分析

Linux 客户机的问题是由于 Axvisor 在启动客户机的最后时刻调用 `mount_virtual_fs` 时没有刷新文件系统，导致文件系统不一致。Linux 启动时进行文件系统完整性校验，发现不一致后拒绝启动，随后关机导致缓存丢失，最终造成文件系统损坏。

根源在于 rsext4 为提升运行时性能设计了多级缓存机制，但 Axvisor 在集成 rsext4 时，没有在文件写操作后进行数据同步。如果客户机修改文件后立即关机，缓存仍停留在内存中，不可避免地导致磁盘数据不一致。

### 解决方案

在 rsext4 的 `ext4fs.rs` 访问层中，对所有写操作调用 `sync_to_disk` 函数，确保数据及时同步到磁盘。虽然会带来一定的性能损失，但保证了文件系统的一致性。

---

## 开发板环境踩坑

本周在板端遇到了一系列棘手的环境问题，花了不少时间排查：

### U-Boot 与启动链问题

- **U-Boot 缺少网卡驱动** — 板载 U-Boot 不支持网络传输，只能通过串口 `loady` 加载内核，debug 内核每次都要通过串口传内核很慢，每次修改都需要完整烧录StarryOS内核
- **SPI 残留环境变量** — 从 OrangePi 官方 Ubuntu 镜像中提取干净的 U-Boot，制作 SPI 镜像刷入开发板。但刷完后发现 SPI 中残留了旧的 U-Boot 环境变量，导致启动失败，只能重新刷回 SPI 镜像
- **eMMC 与 SD 卡混淆** — StarryOS 在开发板上扫描到板载 eMMC，但驱动却强制探测 SD 卡。起初误以为使用的是 SD 卡，经分析后确认实际使用的是板载 eMMC

### **后续 NPU 驱动工作开发路线决策**

最初尝试将 NPU 驱动集成到 StarryOS 主线，但 HAL 层架构 `axhal` 与 `somehal` 存在冲突，RK3588 平台级代码集成涉及整个架构的大修改。为专注于 NPU 驱动核心功能开发，决定先在 StarryOS NPU 版本上开发，待主要功能完成后再移植回主线。

---

## svd2rust 寄存器库重构

基于 svd2rust 工具重构 rknpu 驱动的寄存器访问层。

```rust
// 旧方式：手动偏移 + 裸指针
let status = unsafe { ptr.add(0x20).read_volatile() };
```

### 改进

通过 svd2rust 从 SVD 描述文件生成类型安全的寄存器库 `rknpu-regs`，每个寄存器字段都有明确的类型和文档：

```rust
// 新方式：类型安全 + 自动补全 + 文档
let status = core.pc().interrupt_status().read().bits();
core.pc().interrupt_clear().write(|w| unsafe { w.bits(INT_CLEAR_ALL) });
```

### 成果

- 生成了完整的寄存器定义，覆盖 PC、CNA、CORE、DPU、PPU、DDMA 等所有功能块
- `RknpuCore` 结构体封装了各功能块的访问方法，提供统一接口
- 编译期类型检查杜绝了寄存器偏移量写错、位域宽度搞混等常见错误

---

## AI 辅助注释

利用 AI 为 `axnpu` 驱动代码添加了详细的中文注释(关键函数的调用流程和参数说明)

既方便自己学习 NPU 硬件细节，也降低了后续开发和他人协作的门槛。

---

## 基于 WFI 的异步中断处理

实现了 NPU 任务完成的异步等待机制：

- NPU 核心执行完任务后触发中断
- CPU 通过 WFI（Wait For Interrupt）指令进入低功耗等待状态
- 中断到来时 CPU 被唤醒，读取中断状态寄存器，清除中断标志
- 返回任务执行结果

这为后续多核并发任务提交打下了基础——每个 NPU 核心可以独立触发中断，CPU 侧可以并行等待多个核心的完成通知。

---