# DPU 寄存器块（Data Processing Unit）

**基址**：`CORE_BASE + 0x4000`　｜　**地址范围**：`0x4000 ~ 0x4FFF`

> 来源：RK3588 TRM §36.4.3 Detail Registers Description

DPU 负责后处理运算，包含三级流水线核心：BS CORE（Bias/Scale）→ BN CORE（Batch Norm）→ EW CORE（Element-Wise），以及输出转换器、LUT 引擎、转置/重组等功能。

---

## RKNN_dpu_s_status（0x4000）

执行器状态寄存器（只读）。

| Bit | 属性 | 复位值 | 字段名 | 描述 |
|:---:|:----:|:------:|:-------|:-----|
| 31:18 | RO | 0x0 | — | 保留 |
| 17:16 | RO | 0x0 | `status_1` | 执行器 1 状态。0：空闲；1：正在执行；2：正在执行且等待执行；3：保留 |
| 15:2 | RO | 0x0 | — | 保留 |
| 1:0 | RO | 0x0 | `status_0` | 执行器 0 状态。编码同 `status_1` |

---

## RKNN_dpu_s_pointer（0x4004）

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

## RKNN_dpu_operation_enable（0x4008）

操作使能。写入触发 DPU 执行，此寄存器及之后均为 ping-pong 影子寄存器。

| Bit | 属性 | 复位值 | 字段名 | 描述 |
|:---:|:----:|:------:|:-------|:-----|
| 31:1 | RO | 0x0 | — | 保留 |
| 0 | RW | 0x0 | `op_en` | DPU 操作使能。0：禁用；1：使能 |

---

## RKNN_dpu_feature_mode_cfg（0x400C）

特征模式配置：flying mode、输出目标、卷积模式、burst、非对齐、转置、重组。

| Bit | 属性 | 复位值 | 字段名 | 描述 |
|:---:|:----:|:------:|:-------|:-----|
| 31 | RW | 0x0 | `comb_use` | 组合使用，同 DPU_RDMA `comb_use[0]` |
| 30 | RW | 0x0 | `tp_en` | 转置使能 |
| 29:26 | RW | 0x0 | `rgp_type` | 重组类型。0：全部 128bit；1：4bit；2：8bit；3：16bit；4：32bit；5：64bit |
| 25 | RW | 0x0 | `nonalign` | 非对齐模式使能（输出数据流与输入相同时可用） |
| 24:9 | RW | 0x0 | `surf_len` | 非对齐模式下存储的 8 字节数 |
| 8:5 | RW | 0x0 | `burst_len` | Burst 长度。3：Burst4；7：Burst8；15：Burst16 |
| 4:3 | RW | 0x0 | `conv_mode` | 卷积模式。0：普通卷积；3：Depthwise |
| 2:1 | RW | 0x0 | `output_mode` | 输出目标。[0]：输出到 PPU；[1]：输出到外部 |
| 0 | RW | 0x0 | `flying_mode` | Flying 模式。0：主数据来自卷积输出；1：主数据来自 MRDMA |

---

## RKNN_dpu_data_format（0x4010）

数据格式配置：输入/输出/处理精度、负数移位值。

| Bit | 属性 | 复位值 | 字段名 | 描述 |
|:---:|:----:|:------:|:-------|:-----|
| 31:29 | RW | 0x0 | `out_precision` | 输出精度。0：int8；1：int16；2：fp16；3：bf16；4：int32；5：fp32；6：int4 |
| 28:26 | RW | 0x0 | `in_precision` | 输入精度（同 DPU_RDMA）。编码同上 |
| 25:16 | RW | 0x0 | `ew_truncate_neg` | EW CORE 负数移位值 |
| 15:10 | RW | 0x0 | `bn_mul_shift_value_neg` | BN CORE 负数移位值 |
| 9:4 | RW | 0x0 | `bs_mul_shift_value_neg` | BS CORE 负数移位值 |
| 3 | RW | 0x0 | `mc_surf_out` | 多 surface 输出。0：每像素 16 字节对齐；1：可输出 2/4 surface 串行 |
| 2:0 | RW | 0x0 | `proc_precision` | 处理精度。编码同 `out_precision` |

---

## RKNN_dpu_offset_pend（0x4014）

额外通道填充值。

| Bit | 属性 | 复位值 | 字段名 | 描述 |
|:---:|:----:|:------:|:-------|:-----|
| 31:16 | RO | 0x0 | — | 保留 |
| 15:0 | RW | 0x0 | `offset_pend` | 额外通道设置值 |

---

## RKNN_dpu_dst_base_addr（0x4020）

目标基址。

| Bit | 属性 | 复位值 | 字段名 | 描述 |
|:---:|:----:|:------:|:-------|:-----|
| 31:4 | RW | 0x0 | `dst_base_addr` | 目标基址 |
| 3:0 | RO | 0x0 | — | 保留 |

---

## RKNN_dpu_dst_surf_stride（0x4024）

输出 surface 步长。

| Bit | 属性 | 复位值 | 字段名 | 描述 |
|:---:|:----:|:------:|:-------|:-----|
| 31:4 | RW | 0x0 | `dst_surf_stride` | 输出 shape 的 surface 步长 |
| 3:0 | RO | 0x0 | — | 保留 |

---

## RKNN_dpu_data_cube_width（0x4030）

输入 cube 宽度。

| Bit | 属性 | 复位值 | 字段名 | 描述 |
|:---:|:----:|:------:|:-------|:-----|
| 31:13 | RO | 0x0 | — | 保留 |
| 12:0 | RW | 0x0 | `width` | 输入 cube 宽度 |

---

## RKNN_dpu_data_cube_height（0x4034）

输入 cube 高度 + minmax 控制。

| Bit | 属性 | 复位值 | 字段名 | 描述 |
|:---:|:----:|:------:|:-------|:-----|
| 31:25 | RO | 0x0 | — | 保留 |
| 24:22 | RW | 0x0 | `minmax_ctl` | MinMax 配置。[0]：使能；[1]：类型；[2]：仅概率 |
| 21:13 | RO | 0x0 | — | 保留 |
| 12:0 | RW | 0x0 | `height` | 输入 cube 高度 |

---

## RKNN_dpu_data_cube_notch_addr（0x4038）

Notch 地址（宽度末尾到 shape 行末的像素数）。

| Bit | 属性 | 复位值 | 字段名 | 描述 |
|:---:|:----:|:------:|:-------|:-----|
| 31:29 | RO | 0x0 | — | 保留 |
| 28:16 | RW | 0x0 | `notch_addr_1` | Notch 地址 1 |
| 15:13 | RO | 0x0 | — | 保留 |
| 12:0 | RW | 0x0 | `notch_addr_0` | Notch 地址 0 |

---

## RKNN_dpu_data_cube_channel（0x403C）

输入 cube 通道数。

| Bit | 属性 | 复位值 | 字段名 | 描述 |
|:---:|:----:|:------:|:-------|:-----|
| 31:29 | RO | 0x0 | — | 保留 |
| 28:16 | RW | 0x0 | `orig_channel` | 原始输出通道数 |
| 15:13 | RO | 0x0 | — | 保留 |
| 12:0 | RW | 0x0 | `channel` | Cube 通道数 |

---

## RKNN_dpu_bs_cfg（0x4040）

BS CORE 配置：ALU 算法、操作数来源、ReLU/PRELU/RELUX 控制。

| Bit | 属性 | 复位值 | 字段名 | 描述 |
|:---:|:----:|:------:|:-------|:-----|
| 31:20 | RO | 0x0 | — | 保留 |
| 19:16 | RW | 0x0 | `bs_alu_algo` | BS ALU 运算类型。2：Add；4：Minus |
| 15:9 | RO | 0x0 | — | 保留 |
| 8 | RW | 0x0 | `bs_alu_src` | ALU 操作数来源。0：寄存器；1：外部 |
| 7 | RW | 0x0 | `bs_relux_en` | RELUX 使能 |
| 6 | RW | 0x0 | `bs_relu_bypass` | 旁路 BS RELU。0：不旁路；1：旁路 |
| 5 | RW | 0x0 | `bs_mul_prelu` | MUL PRELU 使能 |
| 4 | RW | 0x0 | `bs_mul_bypass` | 旁路 BS MUL |
| 3:2 | RO | 0x0 | — | 保留 |
| 1 | RW | 0x0 | `bs_alu_bypass` | 旁路 BS ALU |
| 0 | RW | 0x0 | `bs_bypass` | 旁路整个 BS CORE |

---

## RKNN_dpu_bs_alu_cfg（0x4044）

BS ALU 操作数。

| Bit | 属性 | 复位值 | 字段名 | 描述 |
|:---:|:----:|:------:|:-------|:-----|
| 31:0 | RW | 0x0 | `bs_alu_operand` | BS CORE ALU 操作数 |

---

## RKNN_dpu_bs_mul_cfg（0x4048）

BS MUL 配置：操作数、移位值、来源。

| Bit | 属性 | 复位值 | 字段名 | 描述 |
|:---:|:----:|:------:|:-------|:-----|
| 31:16 | RW | 0x0 | `bs_mul_operand` | BS MUL 操作数 |
| 15:14 | RO | 0x0 | — | 保留 |
| 13:8 | RW | 0x0 | `bs_mul_shift_value` | BS 正数移位值 |
| 7:2 | RO | 0x0 | — | 保留 |
| 1 | RW | 0x0 | `bs_truncate_src` | 移位值来源。0：寄存器；1：外部 |
| 0 | RW | 0x0 | `bs_mul_src` | MUL 操作数来源。0：寄存器；1：外部 |

---

## RKNN_dpu_bs_relux_cmp_value（0x404C）

BS RELUX 比较值。

| Bit | 属性 | 复位值 | 字段名 | 描述 |
|:---:|:----:|:------:|:-------|:-----|
| 31:0 | RW | 0x0 | `bs_relux_cmp_dat` | RELUX 比较值 |

---

## RKNN_dpu_bs_ow_cfg（0x4050）

BS OW（CPEND）配置 + 重组计数器 + 转置。

| Bit | 属性 | 复位值 | 字段名 | 描述 |
|:---:|:----:|:------:|:-------|:-----|
| 31:28 | RW | 0x0 | `rgp_cnter` | 重组计数器。0：全选；1：每 2 选 1；2：每 4 选 1；3：每 8 选 1 |
| 27 | RW | 0x0 | `tp_org_en` | 原始转置使能 |
| 26:11 | RO | 0x0 | — | 保留 |
| 10:8 | RW | 0x0 | `size_e_2` | 最后一行输出每行 8 通道数（−1） |
| 7:5 | RW | 0x0 | `size_e_1` | 中间行输出每行 8 通道数（−1） |
| 4:2 | RW | 0x0 | `size_e_0` | 第一行输出每行 8 通道数（−1） |
| 1 | RW | 0x0 | `od_bypass` | 旁路 CPEND |
| 0 | RW | 0x0 | `ow_src` | CPEND 操作数来源。0：寄存器；1：外部 |

---

## RKNN_dpu_bs_ow_op（0x4054）

CPEND 操作数。

| Bit | 属性 | 复位值 | 字段名 | 描述 |
|:---:|:----:|:------:|:-------|:-----|
| 31:16 | RO | 0x0 | — | 保留 |
| 15:0 | RW | 0x0 | `ow_op` | CPEND 操作数 |

---

## RKNN_dpu_wdma_size_0（0x4058）

DPU WDMA 尺寸 0：转置精度、通道。

| Bit | 属性 | 复位值 | 字段名 | 描述 |
|:---:|:----:|:------:|:-------|:-----|
| 31:28 | RO | 0x0 | — | 保留 |
| 27 | RW | 0x0 | `tp_precision` | 转置精度。0：8bit；1：16bit |
| 26:16 | RW | 0x0 | `size_c_wdma` | WDMA 的 size_c |
| 15:13 | RO | 0x0 | — | 保留 |
| 12:0 | RW | 0x0 | `channel_wdma` | WDMA 通道数 |

---

## RKNN_dpu_wdma_size_1（0x405C）

DPU WDMA 尺寸 1：宽高。

| Bit | 属性 | 复位值 | 字段名 | 描述 |
|:---:|:----:|:------:|:-------|:-----|
| 31:29 | RO | 0x0 | — | 保留 |
| 28:16 | RW | 0x0 | `height_wdma` | WDMA 高度 |
| 15:13 | RO | 0x0 | — | 保留 |
| 12:0 | RW | 0x0 | `width_wdma` | WDMA 宽度 |

---

## RKNN_dpu_bn_cfg（0x4060）

BN CORE 配置：ALU 算法、操作数来源、ReLU/PRELU/RELUX 控制。

| Bit | 属性 | 复位值 | 字段名 | 描述 |
|:---:|:----:|:------:|:-------|:-----|
| 31:20 | RO | 0x0 | — | 保留 |
| 19:16 | RW | 0x0 | `bn_alu_algo` | BN ALU 运算类型。2：Add；4：Minus |
| 15:9 | RO | 0x0 | — | 保留 |
| 8 | RW | 0x0 | `bn_alu_src` | ALU 操作数来源。0：寄存器；1：外部 |
| 7 | RW | 0x0 | `bn_relux_en` | RELUX 使能 |
| 6 | RW | 0x0 | `bn_relu_bypass` | 旁路 BN RELU |
| 5 | RW | 0x0 | `bn_mul_prelu` | MUL PRELU 使能 |
| 4 | RW | 0x0 | `bn_mul_bypass` | 旁路 BN MUL |
| 3:2 | RO | 0x0 | — | 保留 |
| 1 | RW | 0x0 | `bn_alu_bypass` | 旁路 BN ALU |
| 0 | RW | 0x0 | `bn_bypass` | 旁路整个 BN CORE |

---

## RKNN_dpu_bn_alu_cfg（0x4064）

BN ALU 操作数。

| Bit | 属性 | 复位值 | 字段名 | 描述 |
|:---:|:----:|:------:|:-------|:-----|
| 31:0 | RW | 0x0 | `bn_alu_operand` | BN CORE ALU 操作数 |

---

## RKNN_dpu_bn_mul_cfg（0x4068）

BN MUL 配置。

| Bit | 属性 | 复位值 | 字段名 | 描述 |
|:---:|:----:|:------:|:-------|:-----|
| 31:16 | RW | 0x0 | `bn_mul_operand` | BN MUL 操作数 |
| 15:14 | RO | 0x0 | — | 保留 |
| 13:8 | RW | 0x0 | `bn_mul_shift_value` | BN 正数移位值 |
| 7:2 | RO | 0x0 | — | 保留 |
| 1 | RW | 0x0 | `bn_truncate_src` | 移位值来源。0：寄存器；1：外部 |
| 0 | RW | 0x0 | `bn_mul_src` | MUL 操作数来源。0：寄存器；1：外部 |

---

## RKNN_dpu_bn_relux_cmp_value（0x406C）

BN RELUX 比较值。

| Bit | 属性 | 复位值 | 字段名 | 描述 |
|:---:|:----:|:------:|:-------|:-----|
| 31:0 | RW | 0x0 | `bn_relux_cmp_dat` | BN RELUX 比较数据 |

---

## RKNN_dpu_ew_cfg（0x4070）

EW CORE 配置：ALU 算法（Max/Min/Add/Div/Minus/Abs/Neg/Floor/Ceil）、LUT、转换器、PRELU 等。

| Bit | 属性 | 复位值 | 字段名 | 描述 |
|:---:|:----:|:------:|:-------|:-----|
| 31 | RW | 0x0 | `ew_cvt_type` | EW 输入转换类型。0：先乘后加；1：先加后乘 |
| 30 | RW | 0x0 | `ew_cvt_round` | EW 输入转换舍入。0：奇入偶不入；1：0.5 向上进 1 |
| 29:28 | RW | 0x0 | `ew_data_mode` | ERDMA 数据模式 |
| 27:24 | RO | 0x0 | — | 保留 |
| 23:22 | RW | 0x0 | `edata_size` | ERDMA cube 数据大小。0：4bit；1：8bit；2：16bit；3：32bit |
| 21 | RW | 0x0 | `ew_equal_en` | MinMax 相等使能 |
| 20 | RW | 0x0 | `ew_binary_en` | MinMax 二值使能 |
| 19:16 | RW | 0x0 | `ew_alu_algo` | EW ALU 运算。0：Max；1：Min；2：Add；3：Div；4：Minus；5：Abs；6：Neg；7：Floor；8：Ceil |
| 15:11 | RO | 0x0 | — | 保留 |
| 10 | RW | 0x0 | `ew_relux_en` | RELUX 使能 |
| 9 | RW | 0x0 | `ew_relu_bypass` | 旁路 EW RELU |
| 8 | RW | 0x0 | `ew_op_cvt_bypass` | 旁路 EW 输入转换器 |
| 7 | RW | 0x0 | `ew_lut_bypass` | 旁路 LUT |
| 6 | RW | 0x0 | `ew_op_src` | 操作数来源。0：寄存器；1：外部 |
| 5 | RW | 0x0 | `ew_mul_prelu` | MUL PRELU 使能 |
| 4:3 | RO | 0x0 | — | 保留 |
| 2 | RW | 0x0 | `ew_op_type` | 运算类型。0：ALU；1：MUL |
| 1 | RW | 0x0 | `ew_op_bypass` | 旁路 EW ALU 和 MUL |
| 0 | RW | 0x0 | `ew_bypass` | 旁路整个 EW CORE |

---

## RKNN_dpu_ew_cvt_offset_value（0x4074）

EW 输入转换偏移。

| Bit | 属性 | 复位值 | 字段名 | 描述 |
|:---:|:----:|:------:|:-------|:-----|
| 31:0 | RW | 0x0 | `ew_op_cvt_offset` | EW 转换偏移 |

---

## RKNN_dpu_ew_cvt_scale_value（0x4078）

EW 转换缩放与移位。

| Bit | 属性 | 复位值 | 字段名 | 描述 |
|:---:|:----:|:------:|:-------|:-----|
| 31:22 | RW | 0x0 | `ew_truncate` | EW CORE 移位值 |
| 21:16 | RW | 0x0 | `ew_op_cvt_shift` | EW 转换移位值 |
| 15:0 | RW | 0x0 | `ew_op_cvt_scale` | EW 转换缩放 |

---

## RKNN_dpu_ew_relux_cmp_value（0x407C）

EW RELUX 比较值。

| Bit | 属性 | 复位值 | 字段名 | 描述 |
|:---:|:----:|:------:|:-------|:-----|
| 31:0 | RW | 0x0 | `ew_relux_cmp_dat` | EW RELUX 比较数据 |

---

## RKNN_dpu_out_cvt_offset（0x4080）

输出转换偏移。

| Bit | 属性 | 复位值 | 字段名 | 描述 |
|:---:|:----:|:------:|:-------|:-----|
| 31:0 | RW | 0x0 | `out_cvt_offset` | 输出转换器偏移 |

---

## RKNN_dpu_out_cvt_scale（0x4084）

输出转换缩放 + fp32→fp16 使能。

| Bit | 属性 | 复位值 | 字段名 | 描述 |
|:---:|:----:|:------:|:-------|:-----|
| 31:17 | RO | 0x0 | — | 保留 |
| 16 | RW | 0x0 | `fp32tofp16_en` | 使能输出 fp32→fp16 转换 |
| 15:0 | RW | 0x0 | `out_cvt_scale` | 输出转换器缩放 |

---

## RKNN_dpu_out_cvt_shift（0x4088）

输出转换移位、舍入、指数。

| Bit | 属性 | 复位值 | 字段名 | 描述 |
|:---:|:----:|:------:|:-------|:-----|
| 31 | RW | 0x0 | `cvt_type` | 输出转换类型。0：先乘后加；1：先加后乘 |
| 30 | RW | 0x0 | `cvt_round` | 输出转换舍入。0：奇入偶不入；1：0.5 向上进 1 |
| 29:20 | RO | 0x0 | — | 保留 |
| 19:12 | RW | 0x0 | `minus_exp` | 输出 CVT 减指数 |
| 11:0 | RW | 0x0 | `out_cvt_shift` | 输出转换器移位 |

---

## RKNN_dpu_ew_op_value_0~7（0x4090 ~ 0x40AC）

EW CORE 操作数寄存器，共 8 个，偏移 `0x4090 + N×4`（N = 0~7）。

| Bit | 属性 | 复位值 | 字段名 | 描述 |
|:---:|:----:|:------:|:-------|:-----|
| 31:0 | RW | 0x0 | `ew_operand_N` | 第 N+1 个 EW 操作数 |

---

## RKNN_dpu_surface_add（0x40C0）

Surface 加法器。

| Bit | 属性 | 复位值 | 字段名 | 描述 |
|:---:|:----:|:------:|:-------|:-----|
| 31:4 | RW | 0x0 | `surf_add` | 一行中有多少个 surface |
| 3:0 | RO | 0x0 | — | 保留 |

---

## RKNN_dpu_lut_access_cfg（0x4100）

LUT 访问配置。

| Bit | 属性 | 复位值 | 字段名 | 描述 |
|:---:|:----:|:------:|:-------|:-----|
| 31:18 | RO | 0x0 | — | 保留 |
| 17 | RW | 0x0 | `lut_access_type` | 访问类型。0：读；1：写 |
| 16 | RW | 0x0 | `lut_table_id` | 访问 ID。0：LE LUT；1：LO LUT |
| 15:10 | RO | 0x0 | — | 保留 |
| 9:0 | RW | 0x0 | `lut_addr` | 访问地址 |

---

## RKNN_dpu_lut_access_data（0x4104）

LUT 访问数据。

| Bit | 属性 | 复位值 | 字段名 | 描述 |
|:---:|:----:|:------:|:-------|:-----|
| 31:16 | RO | 0x0 | — | 保留 |
| 15:0 | RW | 0x0 | `lut_access_data` | LUT 访问数据 |

---

## RKNN_dpu_lut_cfg（0x4108）

LUT 配置。

| Bit | 属性 | 复位值 | 字段名 | 描述 |
|:---:|:----:|:------:|:-------|:-----|
| 31:8 | RO | 0x0 | — | 保留 |
| 7 | RW | 0x0 | `lut_cal_sel` | LUT 计算选择（仅 `lut_expand_en=1` 时有效） |
| 6 | RW | 0x0 | `lut_hybrid_priority` | 混合流优先级。0：LE LUT；1：LO LUT |
| 5 | RW | 0x0 | `lut_oflow_priority` | 上溢优先级。0：LE；1：LO |
| 4 | RW | 0x0 | `lut_uflow_priority` | 下溢优先级。0：LE；1：LO |
| 3:2 | RW | 0x0 | `lut_lo_le_mux` | LO/LE LUT 复用 |
| 1 | RW | 0x0 | `lut_expand_en` | 扩展两个小 LUT 为一个大 LUT |
| 0 | RW | 0x0 | `lut_road_sel` | LUT 路径选择。0：第 1 路；1：第 2 路 |

---

## RKNN_dpu_lut_info（0x410C）

LUT 索引选择。

| Bit | 属性 | 复位值 | 字段名 | 描述 |
|:---:|:----:|:------:|:-------|:-----|
| 31:24 | RO | 0x0 | — | 保留 |
| 23:16 | RW | 0x0 | `lut_lo_index_select` | LO LUT 索引选择（索引生成器中选择哪些位作为索引） |
| 15:8 | RW | 0x0 | `lut_le_index_select` | LE LUT 索引选择 |
| 7:0 | RO | 0x0 | — | 保留 |

---

## RKNN_dpu_lut_le_start（0x4110）

| Bit | 属性 | 复位值 | 字段名 | 描述 |
|:---:|:----:|:------:|:-------|:-----|
| 31:0 | RW | 0x0 | `lut_le_start` | LE LUT 起始点 |

---

## RKNN_dpu_lut_le_end（0x4114）

| Bit | 属性 | 复位值 | 字段名 | 描述 |
|:---:|:----:|:------:|:-------|:-----|
| 31:0 | RW | 0x0 | `lut_le_end` | LE LUT 终止点 |

---

## RKNN_dpu_lut_lo_start（0x4118）

| Bit | 属性 | 复位值 | 字段名 | 描述 |
|:---:|:----:|:------:|:-------|:-----|
| 31:0 | RW | 0x0 | `lut_lo_start` | LO LUT 起始点 |

---

## RKNN_dpu_lut_lo_end（0x411C）

| Bit | 属性 | 复位值 | 字段名 | 描述 |
|:---:|:----:|:------:|:-------|:-----|
| 31:0 | RW | 0x0 | `lut_lo_end` | LO LUT 终止点 |

---

## RKNN_dpu_lut_le_slope_scale（0x4120）

LE LUT 斜率缩放（上溢/下溢）。

| Bit | 属性 | 复位值 | 字段名 | 描述 |
|:---:|:----:|:------:|:-------|:-----|
| 31:16 | RW | 0x0 | `lut_le_slope_oflow_scale` | LE LUT 上溢斜率缩放 |
| 15:0 | RW | 0x0 | `lut_le_slope_uflow_scale` | LE LUT 下溢斜率缩放 |

---

## RKNN_dpu_lut_le_slope_shift（0x4124）

LE LUT 斜率移位。

| Bit | 属性 | 复位值 | 字段名 | 描述 |
|:---:|:----:|:------:|:-------|:-----|
| 31:10 | RO | 0x0 | — | 保留 |
| 9:5 | RW | 0x0 | `lut_le_slope_oflow_shift` | LE LUT 上溢斜率移位 |
| 4:0 | RW | 0x0 | `lut_le_slope_uflow_shift` | LE LUT 下溢斜率移位 |

---

## RKNN_dpu_lut_lo_slope_scale（0x4128）

LO LUT 斜率缩放。

| Bit | 属性 | 复位值 | 字段名 | 描述 |
|:---:|:----:|:------:|:-------|:-----|
| 31:16 | RW | 0x0 | `lut_lo_slope_oflow_scale` | LO LUT 上溢斜率缩放 |
| 15:0 | RW | 0x0 | `lut_lo_slope_uflow_scale` | LO LUT 下溢斜率缩放 |

---

## RKNN_dpu_lut_lo_slope_shift（0x412C）

LO LUT 斜率移位。

| Bit | 属性 | 复位值 | 字段名 | 描述 |
|:---:|:----:|:------:|:-------|:-----|
| 31:10 | RO | 0x0 | — | 保留 |
| 9:5 | RW | 0x0 | `lut_lo_slope_oflow_shift` | LO LUT 上溢斜率移位 |
| 4:0 | RW | 0x0 | `lut_lo_slope_uflow_shift` | LO LUT 下溢斜率移位 |
