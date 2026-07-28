# PPU 寄存器块（Planar Processing Unit）

**基址**：`CORE_BASE + 0x6000`　｜　**地址范围**：`0x6000 ~ 0x6FFF`

> 来源：RK3588 TRM §36.4.3 Detail Registers Description

PPU 负责池化运算，支持平均池化、最大池化、最小池化，可与 DPU 流水线级联或独立 flying 模式运行。

---

## RKNN_ppu_s_status（0x6000）

执行器状态寄存器（只读）。

| Bit | 属性 | 复位值 | 字段名 | 描述 |
|:---:|:----:|:------:|:-------|:-----|
| 31:18 | RO | 0x0 | — | 保留 |
| 17:16 | RO | 0x0 | `status_1` | 执行器 1 状态。0：空闲；1：正在执行；2：正在执行且等待执行；3：保留 |
| 15:2 | RO | 0x0 | — | 保留 |
| 1:0 | RO | 0x0 | `status_0` | 执行器 0 状态。编码同 `status_1` |

---

## RKNN_ppu_s_pointer（0x6004）

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

## RKNN_ppu_operation_enable（0x6008）

操作使能。写入触发 PPU 执行，此寄存器及之后均为 ping-pong 影子寄存器。

| Bit | 属性 | 复位值 | 字段名 | 描述 |
|:---:|:----:|:------:|:-------|:-----|
| 31:1 | RO | 0x0 | — | 保留 |
| 0 | RW | 0x0 | `op_en` | PPU 操作使能。0：禁用；1：使能 |

---

## RKNN_ppu_data_cube_in_width（0x600C）

池化输入 cube 宽度。

| Bit | 属性 | 复位值 | 字段名 | 描述 |
|:---:|:----:|:------:|:-------|:-----|
| 31:13 | RO | 0x0 | — | 保留 |
| 12:0 | RW | 0x0 | `cube_in_width` | 池化输入宽度（需减 1） |

---

## RKNN_ppu_data_cube_in_height（0x6010）

池化输入 cube 高度。

| Bit | 属性 | 复位值 | 字段名 | 描述 |
|:---:|:----:|:------:|:-------|:-----|
| 31:13 | RO | 0x0 | — | 保留 |
| 12:0 | RW | 0x0 | `cube_in_height` | 池化输入高度（需减 1） |

---

## RKNN_ppu_data_cube_in_channel（0x6014）

池化输入 cube 通道数。

| Bit | 属性 | 复位值 | 字段名 | 描述 |
|:---:|:----:|:------:|:-------|:-----|
| 31:13 | RO | 0x0 | — | 保留 |
| 12:0 | RW | 0x0 | `cube_in_channel` | 池化输入通道数（需减 1） |

---

## RKNN_ppu_data_cube_out_width（0x6018）

池化输出 cube 宽度。

| Bit | 属性 | 复位值 | 字段名 | 描述 |
|:---:|:----:|:------:|:-------|:-----|
| 31:13 | RO | 0x0 | — | 保留 |
| 12:0 | RW | 0x0 | `cube_out_width` | 池化输出宽度（需减 1） |

---

## RKNN_ppu_data_cube_out_height（0x601C）

池化输出 cube 高度。

| Bit | 属性 | 复位值 | 字段名 | 描述 |
|:---:|:----:|:------:|:-------|:-----|
| 31:13 | RO | 0x0 | — | 保留 |
| 12:0 | RW | 0x0 | `cube_out_height` | 池化输出高度（需减 1） |

---

## RKNN_ppu_data_cube_out_channel（0x6020）

池化输出 cube 通道数。

| Bit | 属性 | 复位值 | 字段名 | 描述 |
|:---:|:----:|:------:|:-------|:-----|
| 31:13 | RO | 0x0 | — | 保留 |
| 12:0 | RW | 0x0 | `cube_out_channel` | 池化输出通道数（需减 1） |

---

## RKNN_ppu_operation_mode_cfg（0x6024）

操作模式配置：池化方法、flying mode、notch、索引输出。

| Bit | 属性 | 复位值 | 字段名 | 描述 |
|:---:|:----:|:------:|:-------|:-----|
| 31 | RO | 0x0 | — | 保留 |
| 30 | RW | 0x0 | `index_en` | 使能输出每个 kernel 的位置索引 |
| 29 | RO | 0x0 | — | 保留 |
| 28:16 | RW | 0x0 | `notch_addr` | 宽度末尾到 shape 行末的像素数 |
| 15:8 | RO | 0x0 | — | 保留 |
| 7:5 | RW | 0x0 | `use_cnt` | use_cnt |
| 4 | RW | 0x0 | `flying_mode` | 池化 cube 来源。0：DPU；1：外部 |
| 3:2 | RO | 0x0 | — | 保留 |
| 1:0 | RW | 0x0 | `pooling_method` | 池化方法。0：平均池化；1：最大池化；2：最小池化；3：保留 |

---

## RKNN_ppu_pooling_kernel_cfg（0x6034）

池化 kernel 大小与步长。

| Bit | 属性 | 复位值 | 字段名 | 描述 |
|:---:|:----:|:------:|:-------|:-----|
| 31:24 | RO | 0x0 | — | 保留 |
| 23:20 | RW | 0x0 | `kernel_stride_height` | Kernel 步长高度（需减 1） |
| 19:16 | RW | 0x0 | `kernel_stride_width` | Kernel 步长宽度（需减 1） |
| 15:12 | RO | 0x0 | — | 保留 |
| 11:8 | RW | 0x0 | `kernel_height` | Kernel 高度（需减 1） |
| 7:4 | RO | 0x0 | — | 保留 |
| 3:0 | RW | 0x0 | `kernel_width` | Kernel 宽度（需减 1） |

---

## RKNN_ppu_recip_kernel_width（0x6038）

Kernel 宽度倒数（用于平均池化计算）。

| Bit | 属性 | 复位值 | 字段名 | 描述 |
|:---:|:----:|:------:|:-------|:-----|
| 31:17 | RO | 0x0 | — | 保留 |
| 16:0 | RW | 0x0 | `recip_kernel_width` | Shape kernel 宽度的倒数 × 2^16 |

---

## RKNN_ppu_recip_kernel_height（0x603C）

Kernel 高度倒数。

| Bit | 属性 | 复位值 | 字段名 | 描述 |
|:---:|:----:|:------:|:-------|:-----|
| 31:17 | RO | 0x0 | — | 保留 |
| 16:0 | RW | 0x0 | `recip_kernel_height` | Shape kernel 高度的倒数 × 2^16 |

---

## RKNN_ppu_pooling_padding_cfg（0x6040）

池化四边 padding。

| Bit | 属性 | 复位值 | 字段名 | 描述 |
|:---:|:----:|:------:|:-------|:-----|
| 31:15 | RO | 0x0 | — | 保留 |
| 14:12 | RW | 0x0 | `pad_bottom` | 底部 pad |
| 11 | RO | 0x0 | — | 保留 |
| 10:8 | RW | 0x0 | `pad_right` | 右侧 pad |
| 7 | RO | 0x0 | — | 保留 |
| 6:4 | RW | 0x0 | `pad_top` | 顶部 pad |
| 3 | RO | 0x0 | — | 保留 |
| 2:0 | RW | 0x0 | `pad_left` | 左侧 pad |

---

## RKNN_ppu_padding_value_1_cfg（0x6044）

Pad 填充值低 32 位。

| Bit | 属性 | 复位值 | 字段名 | 描述 |
|:---:|:----:|:------:|:-------|:-----|
| 31:0 | RW | 0x0 | `pad_value_0` | pad_value×1 [31:0] |

---

## RKNN_ppu_padding_value_2_cfg（0x6048）

Pad 填充值高 3 位。

| Bit | 属性 | 复位值 | 字段名 | 描述 |
|:---:|:----:|:------:|:-------|:-----|
| 31:3 | RO | 0x0 | — | 保留 |
| 2:0 | RW | 0x0 | `pad_value_1` | pad_value×1 [34:32] |

---

## RKNN_ppu_dst_base_addr（0x6070）

输出 cube 目标基址。

| Bit | 属性 | 复位值 | 字段名 | 描述 |
|:---:|:----:|:------:|:-------|:-----|
| 31:4 | RW | 0x0 | `dst_base_addr` | 输出 cube 目标基址 |
| 3:0 | RO | 0x0 | — | 保留 |

---

## RKNN_ppu_dst_surf_stride（0x607C）

输出 surface 步长。

| Bit | 属性 | 复位值 | 字段名 | 描述 |
|:---:|:----:|:------:|:-------|:-----|
| 31:4 | RW | 0x0 | `dst_surf_stride` | 输出 shape 面积 |
| 3:0 | RO | 0x0 | — | 保留 |

---

## RKNN_ppu_data_format（0x6084）

数据格式配置。

| Bit | 属性 | 复位值 | 字段名 | 描述 |
|:---:|:----:|:------:|:-------|:-----|
| 31:4 | RW | 0x0 | `index_add` | 若 `index_en` 使能，值为 `dst_surface_stride × cube surface 数`（每 surface 8 字节），否则等于 `dst_surface_stride` |
| 3 | RW | 0x0 | `dpu_flyin` | 数据来自 DPU 且 DPU 数据来自外部时置 1 |
| 2:0 | RW | 0x0 | `proc_precision` | 处理精度 |

---

## RKNN_ppu_misc_ctrl（0x60DC）

杂项控制：非对齐模式、多 surface 输出、burst。

| Bit | 属性 | 复位值 | 字段名 | 描述 |
|:---:|:----:|:------:|:-------|:-----|
| 31:16 | RW | 0x0 | `surf_len` | Surface 计数长度 |
| 15:9 | RO | 0x0 | — | 保留 |
| 8 | RW | 0x0 | `mc_surf_out` | 多 surface 输出使能 |
| 7 | RW | 0x0 | `nonalign` | 非对齐模式使能 |
| 6:4 | RO | 0x0 | — | 保留 |
| 3:0 | RW | 0x0 | `burst_len` | Burst 长度。3：Burst4；7：Burst8；15：Burst16 |
