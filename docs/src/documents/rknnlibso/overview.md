# RKNN 用户库手册

## 概述

`librknnrt.so` 是 Rockchip 提供的**闭源** NPU 运行时库，负责将神经网络模型编译为硬件命令流并提交给内核驱动执行。它是用户态与 NPU 硬件之间的桥梁。

## 架构层次

```
┌─────────────────────────────────────────────────┐
│              用户应用程序                          │
├─────────────────────────────────────────────────┤
│  rknn_api.h          │  rknn_matmul_api.h       │  ← 闭源库公开头文件
├──────────────────────┼──────────────────────────┤
│  librknnrt.so（闭源）                             │
│  ┌──────────┐ ┌──────────┐ ┌──────────────────┐ │
│  │模型解析器 │ │图优化/编译│ │命令流生成 + Task │ │
│  └──────────┘ └──────────┘ └──────────────────┘ │
├─────────────────────────────────────────────────┤
│  DRM ioctl (DRM_IOCTL_RKNPU_*)                  │  ← 开源 ABI
├─────────────────────────────────────────────────┤
│  rknpu 内核驱动（开源 GPL）                       │
│  任务调度 │ GEM 内存 │ 中断处理 │ PC 寄存器写入   │
├─────────────────────────────────────────────────┤
│  NPU 硬件（CNA / CORE / DPU / PPU）              │
└─────────────────────────────────────────────────┘
```

## 两条 NPU 使用路径

| 路径 | 依赖 | 操作方式 | 代表项目 |
|:-----|:-----|:---------|:---------|
| **闭源 API 路径** | `librknnrt.so` | `rknn_init → rknn_run → rknn_outputs_get` | yolov8 demo |
| **裸 ioctl 路径** | 仅 `libdrm` + 内核驱动 | 用户自行构造命令流 + `DRM_IOCTL_RKNPU_SUBMIT` | npu_benchmark / npu_llama |

闭源库封装了命令流生成、内存规划、多核调度等全部复杂逻辑；裸 ioctl 路径则要求用户自己实现这些。

## 闭源库提供的三套 API

| 头文件 | 功能 | 说明 |
|:-------|:-----|:-----|
| `rknn_api.h` | 模型推理 | 加载 .rknn 模型 → 设置输入 → 推理 → 获取输出 |
| `rknn_matmul_api.h` | 矩阵乘法加速 | 独立于模型推理的通用 matmul 接口 |
| `rknn_custom_op.h` | 自定义算子 | 注册用户实现的 CPU/GPU 算子回调 |

## 闭源库二进制

仓库中 `test/starrynpu/demo/yolov8/3rdparty/` 包含实际二进制：

| 文件 | 说明 |
|:-----|:-----|
| `rknpu2/Linux/aarch64/librknnrt.so` | RKNPU2 闭源运行时（当前版本） |
| `rknpu1/Linux/aarch64/librknn_api.so` | RKNPU1 旧版 API 库 |
| `rknpu2/include/rknn_api.h` | 推理 API 头文件（805 行） |
| `rknpu2/include/rknn_matmul_api.h` | Matmul API 头文件（544 行） |
| `rknpu2/include/rknn_custom_op.h` | 自定义算子 API 头文件（145 行） |

## 文档结构

- **[推理 API 参考](./rknn-api.md)** — `rknn_api.h` 全部函数签名与逆向还原
- **[矩阵乘法 API](./rknn-matmul.md)** — `rknn_matmul_api.h` 与 Native Layout 规范
- **[自定义算子 API](./custom-op.md)** — `rknn_custom_op.h` 回调机制
- **[模型推理全链条](./lifecycle.md)** — 从加载到运行到结束的完整逻辑链条
- **[内存管理与零拷贝](./memory.md)** — 闭源库的内存分配策略
- **[闭源库内部机制](./internals.md)** — 命令流生成、Task 构造、多核切分
