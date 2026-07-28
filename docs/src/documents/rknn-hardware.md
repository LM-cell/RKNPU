# RKNN 硬件手册

> 来源：RK3588 TRM Chapter 36 RKNN

本章汇总 RK3588 NPU 硬件相关文档，包括硬件特性概览与完整寄存器参考。

## RKNN 硬件特性

- 三核 NPU，支持三核协同 / 双核协同 / 单核独立
- 每核 384KB 内部缓冲，AHB 配置接口 + AXI 数据接口
- 支持 INT4 / INT8 / INT16 / FP16 / BF16 / TF32 多精度推理
- 功能流水线：CNA（卷积）→ CORE（MAC）→ DPU（后处理）→ PPU（池化）
- 激活函数：ReLU / Leaky ReLU / ReLUx / Sigmoid / Tanh / Softmax
- 池化：Average / Max / Min Pooling

[完整硬件特性](./rknn-feature.md)

## 寄存器图

每个核心拥有独立 64KB 寄存器空间，按功能模块划分：

| 模块 | 地址范围 | 功能 |
|:----:|:--------:|:-----|
| **PC** | `0x0000 ~ 0x0FFF` | 任务控制器 / 命令流引擎 |
| **CNA** | `0x1000 ~ 0x1FFF` | 卷积神经网络加速单元 |
| **CORE** | `0x3000 ~ 0x3FFF` | MAC 核心控制 |
| **DPU** | `0x4000 ~ 0x4FFF` | 数据后处理单元 |
| **DPU_RDMA** | `0x5000 ~ 0x5FFF` | DPU 读 DMA |
| **PPU** | `0x6000 ~ 0x6FFF` | 池化处理单元 |
| **PPU_RDMA** | `0x7000 ~ 0x7FFF` | PPU 读 DMA |
| **DDMA** | `0x8000 ~ 0x8FFF` | Data DMA 引擎 |
| **SDMA** | `0x9000 ~ 0x9FFF` | System DMA 引擎 |
| **GLOBAL** | `0xF000 ~ 0xFFFF` | 全局使能掩码 |

[寄存器总览](./register-map.md)