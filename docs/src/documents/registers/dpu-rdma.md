# DPU_RDMA 寄存器块

**基址**：`CORE_BASE + 0x5000`　｜　**地址范围**：`0x5000 ~ 0x5FFF`

> 来源：RK3588 TRM §36.4.3 Detail Registers Description

DPU_RDMA 负责为 DPU 从外部内存读取输入数据，包含四路 DMA 引擎：MRDMA（主数据）、BRDMA（BS 操作数）、NRDMA（BN 操作数）、ERDMA（EW 操作数）。

---

## RKNN_dpu_rdma_s_status（0x5000）

执行器状态寄存器（只读）。

| Bit | 属性 | 复位值 | 字段名 | 描述 |
|:---:|:----:|:------:|:-------|:-----|
| 31:18 | RO | 0x0 | — | 保留 |
| 17:16 | RO | 0x0 | `status_1` | 执行器 1 状态。0：空闲；1：正在执行；2：正在执行且等待执行；3：保留 |
| 15:2 | RO | 0x0 | — | 保留 |
| 1:0 | RO | 0x0 | `status_0` | 执行器 0 状态。编码同 `status_1` |

---

## RKNN_dpu_rdma_s_pointer（0x5004）

寄存器组指针与 ping-pong 控制。

| Bit | 属性 | 复位值 | 字段名 | 描述 |
|:---:|:----:|:------:|:-------|:-----|
| 31:17 | RO | 0x0 | — | 保留 |
| 16 | RO | 0x0 | `executer` | 当前使用的寄存器组。0：组 0；1：组 1 |
| 15:6 | RO | 0x0 | — | 保留 |
| 5 | W1C | 0x0 | `executer_pp_clear` | 清除执行器组指针，写 1 清零 |
| 4 | W1C | 0x0 | `pointer_pp_clear` | 清除寄存器组指针，写 1 清零 |
| 3 | RW | 0x0 | `pointer_pp_mode` | Ping-pong 模式。0：按执行器切换；1：按指针切换 |
| 2 | RW | 0x0 | `executer_pp_en` | 执行器组 ping-pong 使能 |
| 1 | RW | 0x0 | `pointer_pp_en` | 寄存器组 ping-pong 使能 |
| 0 | RW | 0x0 | `pointer` | 当前待设置的寄存器组。0：组 0；1：组 1 |

---

## RKNN_dpu_rdma_operation_enable（0x5008）

操作使能。写入触发 DPU_RDMA 执行，此寄存器及之后均为 ping-pong 影子寄存器。

| Bit | 属性 | 复位值 | 字段名 | 描述 |
|:---:|:----:|:------:|:-------|:-----|
| 31:1 | RO | 0x0 | — | 保留 |
| 0 | RW | 0x0 | `op_en` | DPU_RDMA 操作使能。0：禁用；1：使能 |

---

## RKNN_dpu_rdma_data_cube_width（0x500C）

输入特征宽度。

| Bit | 属性 | 复位值 | 字段名 | 描述 |
|:---:|:----:|:------:|:-------|:-----|
| 31:13 | RO | 0x0 | — | 保留 |
| 12:0 | RW | 0x0 | `width` | 输入特征宽度（需减 1） |

---

## RKNN_dpu_rdma_data_cube_height（0x5010）

输入特征高度 + EW line notch。

| Bit | 属性 | 复位值 | 字段名 | 描述 |
|:---:|:----:|:------:|:-------|:-----|
| 31:29 | RO | 0x0 | — | 保留 |
| 28:16 | RW | 0x0 | `ew_line_notch_addr` | EW 行 notch |
| 15:13 | RO | 0x0 | — | 保留 |
| 12:0 | RW | 0x0 | `height` | 输入特征高度（需减 1） |

---

## RKNN_dpu_rdma_data_cube_channel（0x5014）

输入特征通道数。

| Bit | 属性 | 复位值 | 字段名 | 描述 |
|:---:|:----:|:------:|:-------|:-----|
| 31:13 | RO | 0x0 | — | 保留 |
| 12:0 | RW | 0x0 | `channel` | 输入特征通道数（需减 1） |

---

## RKNN_dpu_rdma_src_base_addr（0x5018）

Flying 模式源地址。

| Bit | 属性 | 复位值 | 字段名 | 描述 |
|:---:|:----:|:------:|:-------|:-----|
| 31:0 | RW | 0x0 | `src_base_addr` | Flying 模式源地址 |

---

## RKNN_dpu_rdma_brdma_cfg（0x501C）

BRDMA（BS 操作数读取 DMA）配置。

| Bit | 属性 | 复位值 | 字段名 | 描述 |
|:---:|:----:|:------:|:-------|:-----|
| 31:5 | RO | 0x0 | — | 保留 |
| 4:1 | RW | 0x0 | `brdma_data_use` | 读取数据类型。[0]：ALU 操作数；[1]：CPEND 操作数；[2]：MUL 操作数；[3]：TRT 操作数 |
| 0 | RO | 0x0 | — | 保留 |

---

## RKNN_dpu_rdma_bs_base_addr（0x5020）

BS 操作数基址。

| Bit | 属性 | 复位值 | 字段名 | 描述 |
|:---:|:----:|:------:|:-------|:-----|
| 31:0 | RW | 0x0 | `bs_base_addr` | 读取 BS ALU、BS CPEND、BS MUL 操作数的基址 |

---

## RKNN_dpu_rdma_nrdma_cfg（0x5028）

NRDMA（BN 操作数读取 DMA）配置。

| Bit | 属性 | 复位值 | 字段名 | 描述 |
|:---:|:----:|:------:|:-------|:-----|
| 31:5 | RO | 0x0 | — | 保留 |
| 4:1 | RW | 0x0 | `nrdma_data_use` | 读取数据类型。[0]：ALU 操作数；[1]：CPEND 操作数（固定为 0，BN 无 CPEND）；[2]：MUL 操作数；[3]：TRT 操作数 |
| 0 | RO | 0x0 | — | 保留 |

---

## RKNN_dpu_rdma_bn_base_addr（0x502C）

BN 操作数基址。

| Bit | 属性 | 复位值 | 字段名 | 描述 |
|:---:|:----:|:------:|:-------|:-----|
| 31:0 | RW | 0x0 | `bn_base_addr` | 读取 BN ALU、BN MUL 操作数的基址 |

---

## RKNN_dpu_rdma_erdma_cfg（0x5034）

ERDMA（EW 操作数读取 DMA）配置。

| Bit | 属性 | 复位值 | 字段名 | 描述 |
|:---:|:----:|:------:|:-------|:-----|
| 31:30 | RW | 0x0 | `erdma_data_mode` | 数据模式。0：按通道；1：按像素；2：按通道×像素；3：保留 |
| 29 | RW | 0x0 | `erdma_surf_mode` | Surface 模式。0：1 surface 串行；1：2 surface 串行 |
| 28 | RW | 0x0 | `erdma_nonalign` | 非对齐模式。0：禁用；1：使能 |
| 27:4 | RO | 0x0 | — | 保留 |
| 3:2 | RW | 0x0 | `erdma_data_size` | ERDMA 读取精度。0：4bit；1：8bit；2：16bit；3：32bit |
| 1 | RW | 0x0 | `ov4k_bypass` | 超 4K burst 拆分。0：使能；1：旁路 |
| 0 | RW | 0x0 | `erdma_disable` | 禁用 ERDMA。0：不禁用；1：禁用 |

---

## RKNN_dpu_rdma_ew_base_addr（0x5038）

EW 操作数基址。

| Bit | 属性 | 复位值 | 字段名 | 描述 |
|:---:|:----:|:------:|:-------|:-----|
| 31:0 | RW | 0x0 | `ew_base_addr` | 读取 EW 操作数的基址 |

---

## RKNN_dpu_rdma_ew_surf_stride（0x5040）

EW 特征图 surface 步长。

| Bit | 属性 | 复位值 | 字段名 | 描述 |
|:---:|:----:|:------:|:-------|:-----|
| 31:4 | RW | 0x0 | `ew_surf_stride` | EW 特征图 surface 步长。若 `erdma_data_mode` 为按通道模式，需设为 1 |
| 3:0 | RO | 0x0 | — | 保留 |

---

## RKNN_dpu_rdma_feature_mode_cfg（0x5044）

特征模式配置：精度、burst、组合使用、flying mode、unpooling。

| Bit | 属性 | 复位值 | 字段名 | 描述 |
|:---:|:----:|:------:|:-------|:-----|
| 31:18 | RO | 0x0 | — | 保留 |
| 17:15 | RW | 0x0 | `in_precision` | 输入数据精度。0：int8；1：int16；2：fp16；3：bf16；4：int32；5：fp32；6：int4 |
| 14:11 | RW | 0x0 | `burst_len` | Burst 长度。3：Burst4；7：Burst8；15：Burst16 |
| 10:8 | RW | 0x0 | `comb_use` | 组合使用。[0]：MRDMA 和 ERDMA 读同一数据；[1]：数据送 MRDMA；[2]：数据送 ERDMA |
| 7:5 | RW | 0x0 | `proc_precision` | 处理精度。编码同 `in_precision` |
| 4 | RW | 0x0 | `mrdma_disable` | 禁用 MRDMA。0：不禁用；1：禁用 |
| 3 | RW | 0x0 | `mrdma_fp16tofp32_en` | 使能 DPU 输入 fp16→fp32 转换 |
| 2:1 | RW | 0x0 | `conv_mode` | 卷积模式。0：DC；3：Depthwise |
| 0 | RW | 0x0 | `flying_mode` | Flying 模式。0：主数据来自卷积输出；1：主数据来自 MRDMA |

---

## RKNN_dpu_rdma_src_dma_cfg（0x5048）

源 DMA 配置：line notch、unpooling 参数。

| Bit | 属性 | 复位值 | 字段名 | 描述 |
|:---:|:----:|:------:|:-------|:-----|
| 31:19 | RW | 0x0 | `line_notch_addr` | 宽度末尾到 shape 特征行末的像素数 |
| 18:14 | RO | 0x0 | — | 保留 |
| 13 | RW | 0x0 | `pooling_method` | 池化方法。0：平均池化（上采样可用此模式）；1：最小/最大池化 |
| 12 | RW | 0x0 | `unpooling_en` | 反池化使能 |
| 11:9 | RW | 0x0 | `kernel_stride_height` | 反池化 kernel 步长高度（−1） |
| 8:6 | RW | 0x0 | `kernel_stride_width` | 反池化 kernel 步长宽度（−1） |
| 5:3 | RW | 0x0 | `kernel_height` | 反池化 kernel 高度（−1） |
| 2:0 | RW | 0x0 | `kernel_width` | 反池化 kernel 宽度（−1） |

---

## RKNN_dpu_rdma_surf_notch（0x504C）

Surface notch。

| Bit | 属性 | 复位值 | 字段名 | 描述 |
|:---:|:----:|:------:|:-------|:-----|
| 31:4 | RW | 0x0 | `surf_notch_addr` | 当前处理特征图末尾到 shape 特征图末尾的像素数 |
| 3:0 | RO | 0x0 | — | 保留 |

---

## RKNN_dpu_rdma_pad_cfg（0x5064）

反池化 Pad 配置。

| Bit | 属性 | 复位值 | 字段名 | 描述 |
|:---:|:----:|:------:|:-------|:-----|
| 31:16 | RW | 0x0 | `pad_value` | Pad 填充值 |
| 15:7 | RO | 0x0 | — | 保留 |
| 6:4 | RW | 0x0 | `pad_top` | 反池化顶部 pad |
| 3 | RO | 0x0 | — | 保留 |
| 2:0 | RW | 0x0 | `pad_left` | 反池化左侧 pad |

---

## RKNN_dpu_rdma_weight（0x5068）

四路 DMA 仲裁权重。

| Bit | 属性 | 复位值 | 字段名 | 描述 |
|:---:|:----:|:------:|:-------|:-----|
| 31:24 | RW | 0x0 | `e_weight` | ERDMA 仲裁权重 |
| 23:16 | RW | 0x0 | `n_weight` | NRDMA 仲裁权重 |
| 15:8 | RW | 0x0 | `b_weight` | BRDMA 仲裁权重 |
| 7:0 | RW | 0x0 | `m_weight` | MRDMA 仲裁权重 |

---

## RKNN_dpu_rdma_ew_surf_notch（0x506C）

EW surface notch。

| Bit | 属性 | 复位值 | 字段名 | 描述 |
|:---:|:----:|:------:|:-------|:-----|
| 31:4 | RW | 0x0 | `ew_surf_notch` | EW surface notch |
| 3:0 | RO | 0x0 | — | 保留 |
