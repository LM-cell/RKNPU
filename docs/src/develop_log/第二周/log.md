# 第二周开发日志（2.1-2.7）

## 工作总结

- **开发板验证通过** — 在 OrangePi 5 Plus（RK3588）上部署 StarryOS（NPU 版本），成功跑通 RKNN 推理测例
- **闭源库逆向文档整理** — 整理了 `librknnrt.so` 逆向成果、完整寄存器语义、模型推理全链条，发布至 [GitHub Pages](https://qc-gpu-driver.github.io/starryos-pulsar/documents/docs.html)
- **驱动骨架搭建** — 创建 `axnpu-rknn` 独立 crate，编写 DTB 设备探测代码，通过 `rdrive` 框架自动匹配 NPU 设备
- **DRM 框架初步实现** — 创建 `axdrm` crate，实现 GEM 内存对象管理和 ioctl 编码解析与分发机制

---

## 开发板验证

搭载 RK3588 芯片的 **OrangePi 5 Plus** 开发板到了之后，将带有 NPU 支持的 StarryOS 版本（[atomgit 仓库](https://atomgit.com/aios-porting/3906cf9f2ccbf898bd3512d5862ef92e)）部署到开发板上，**成功跑通测例**，验证了 NPU 驱动在 StarryOS 上的可行性。

---

## 闭源库逆向与文档整理

花了 3 天时间系统整理了以下内容：

- **`librknnrt.so` 闭源库逆向成果** — 梳理了用户态库的内部调用流程、ioctl 命令、内存管理机制
- **寄存器语义** — 从官方 TRM 手册和 [RKNN 开源内核驱动](https://github.com/rockchip-linux/kernel)（C 语言版本）中，整理了完整的寄存器描述和语义
- **模型推理全链条** — 从 `rknn_init` 到 `rknn_destroy` 的完整生命周期，包括任务提交流程、状态机、DMA Fence 路径等

> **文档链接**：[https://qc-gpu-driver.github.io/starryos-pulsar/documents/docs.html](https://qc-gpu-driver.github.io/starryos-pulsar/documents/docs.html)

这份文档不仅方便自己开发时查阅，也相当于一份**社区参考文档**，我也会一直维护和验证的，方便更多对 RKNN NPU 驱动感兴趣的开发者查阅和使用。

---

## 驱动骨架编写

最初尝试直接在 `arceos/modules/` 下创建 `axnpu-rknn` 驱动模块，但遇到了 **workspace 循环依赖问题**——ArceOS 的 `axdriver_block` 锁定的版本缺少 `ahci` feature，而 Cargo workspace 会全量解析所有 members 的依赖，导致整个 workspace 编译失败。

参考 StarryOS（NPU 版本）的做法，将驱动模块移动到 Starry 根目录下独立开发，等完善后再集成回 arceos。

编写了 `dtbparse.rs`，利用 ArceOS 的 `rdrive` 框架通过设备树（DTB）自动探测 NPU 设备：

```rust
module_driver! {
    name: "RKNPU",
    level: ProbeLevel::PostKernel,
    priority: ProbePriority::DEFAULT,
    probe_kinds: &[
        ProbeKind::Fdt {
            compatibles: &["rockchip,rk3588-rknn"],
            on_probe: probe_rknpu
        }
    ],
}

fn probe_rknpu(info: FdtInfo<'_>, dev: PlatformDevice) -> Result<(), OnProbeError> {
    let name = info.node.name();
    // 提取 MMIO 基址: 0xfdab0000, 大小: 0x9000
    let mut regs = info.node.reg().ok_or_else(|| { /* ... */ })?;
    let base_reg = regs.next().ok_or_else(|| { /* ... */ })?;
    let mmio_base = base_reg.address as usize;
    let mmio_size = base_reg.size.unwrap_or(0x9000);
    // 提取中断号: SPI 110, 111, 112（3 个 NPU 核心）
    // ...
    Ok(())
}
```

`module_driver!` 宏会将驱动自动注册到 `rdrive` 框架，内核启动时遍历 DTB，匹配 `compatible = "rockchip,rk3588-rknn"` 后自动调用 `probe_rknpu`，无需手动调用。

---

## DRM 框架搭建

创建了 `driver/axdrm` crate，作为用户态接口适配层：

**GEM 内存对象管理** — 实现了 `GemNumberAllocator`（handle 编号分配/回收）、`GemObject`（物理地址、虚拟地址、fake offset）、`GemHandle`（自动回收的 RAII handle）

**ioctl 编码解析** — 实现了 DRM ioctl 命令的解码函数：

| 函数 | 作用 |
|------|------|
| `ioctl_nr(cmd)` | 提取 ioctl 编号（bits [7:0]） |
| `ioctl_type(cmd)` | 提取类型字节（DRM = `'d'` = 0x64） |
| `ioctl_size(cmd)` | 提取参数大小（bits [29:16]） |
| `is_driver_ioctl(cmd)` | 判断是否为驱动私有命令（nr >= 0x40） |

**ioctl 分发机制** — 驱动通过 `register_driver()` 注册自己的 ioctl handler，`dispatch_ioctl()` 根据命令编号自动分发到对应驱动。NPU 驱动只需注册一个回调函数即可处理私有 ioctl。

---
