# 寄存器地图

RK3588 NPU 每个核心（共 3 核）拥有独立的寄存器空间，内部按功能模块划分为以下区域：

## 地址空间总览

| Base[15:12] | 模块 | 大小 | 地址范围 | 功能 |
|:-----------:|:----:|:----:|:--------:|:-----|
| `4'h0` | **PC** | 4KB | `0x0000 ~ 0x0FFF` | 任务控制器 / 命令流引擎 |
| `4'h1` | **CNA** | 4KB | `0x1000 ~ 0x1FFF` | 卷积神经网络加速单元 |
| `4'h3` | **CORE** | 4KB | `0x3000 ~ 0x3FFF` | MAC 核心控制 |
| `4'h4` | **DPU** | 4KB | `0x4000 ~ 0x4FFF` | 数据后处理单元 |
| `4'h5` | **DPU_RDMA** | 4KB | `0x5000 ~ 0x5FFF` | DPU 读 DMA |
| `4'h6` | **PPU** | 4KB | `0x6000 ~ 0x6FFF` | 池化处理单元 |
| `4'h7` | **PPU_RDMA** | 4KB | `0x7000 ~ 0x7FFF` | PPU 读 DMA |
| `4'h8` | **DDMA** | 4KB | `0x8000 ~ 0x8FFF` | Data DMA 引擎 |
| `4'h9` | **SDMA** | 4KB | `0x9000 ~ 0x9FFF` | System DMA 引擎 |
| `4'hF` | **GLOBAL** | 4B | `0xF000 ~ 0xF004` | 全局使能掩码 |

> **来源说明**：地址映射来自 <span style="background:#e8f5e9;padding:1px 4px;border-radius:3px;font-size:0.85em">RK3588 TRM</span> Table 1-1 RKNN Address Mapping。

---
- [PC（任务控制器）](registers/pc.md)
- [CNA（卷积加速）](registers/cna.md)
- [CORE（MAC 核心）](registers/core.md)
- [DPU（数据后处理）](registers/dpu.md)
- [DPU_RDMA](registers/dpu-rdma.md)
- [PPU（池化）](registers/ppu.md)
- [PPU_RDMA](registers/ppu-rdma.md)
- [DDMA / SDMA](registers/ddma-sdma.md)
- [GLOBAL（全局使能）](registers/global.md)
