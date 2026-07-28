# PPU_RDMA 寄存器块

**基址**：`CORE_BASE + 0x7000`　｜　**地址范围**：`0x7000 ~ 0x7FFF`

> 来源：RK3588 TRM §36.4.3 Detail Registers Description

PPU_RDMA 负责为 PPU 从外部内存读取池化输入特征数据（flying 模式下使用）。

---

## RKNN_ppu_rdma_s_status（0x7000）

执行器状态寄存器（只读）。

| Bit | 属性 | 复位值 | 字段名 | 描述 |
|:---:|:----:|:------:|:-------|:-----|
| 31:18 | RO | 0x0 | — | 保留 |
| 17:16 | RO | 0x0 | `status_1` | 执行器 1 状态。0：空闲；1：正在执行；2：正在执行且等待执行；3：保留 |
| 15:2 | RO | 0x0 | — | 保留 |
| 1:0 | RO | 0x0 | `status_0` | 执行器 0 状态。编码同 `status_1` |

---

## RKNN_ppu_rdma_s_pointer（0x7004）

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

## RKNN_ppu_rdma_operation_enable（0x7008）

操作使能。写入触发 PPU_RDMA 执行，此寄存器及之后均为 ping-pong 影子寄存器。

| Bit | 属性 | 复位值 | 字段名 | 描述 |
|:---:|:----:|:------:|:-------|:-----|
| 31:1 | RO | 0x0 | — | 保留 |
| 0 | RW | 0x0 | `op_en` | PPU_RDMA 操作使能。0：禁用；1：使能 |

---

## RKNN_ppu_rdma_cube_in_width（0x700C）

池化输入 cube 宽度。

| Bit | 属性 | 复位值 | 字段名 | 描述 |
|:---:|:----:|:------:|:-------|:-----|
| 31:13 | RO | 0x0 | — | 保留 |
| 12:0 | RW | 0x0 | `cube_in_width` | 池化 cube 宽度（需减 1） |

---

## RKNN_ppu_rdma_cube_in_height（0x7010）

池化输入 cube 高度。

| Bit | 属性 | 复位值 | 字段名 | 描述 |
|:---:|:----:|:------:|:-------|:-----|
| 31:13 | RO | 0x0 | — | 保留 |
| 12:0 | RW | 0x0 | `cube_in_height` | 池化 cube 高度（需减 1） |

---

## RKNN_ppu_rdma_cube_in_channel（0x7014）

池化输入 cube 通道数。

| Bit | 属性 | 复位值 | 字段名 | 描述 |
|:---:|:----:|:------:|:-------|:-----|
| 31:13 | RO | 0x0 | — | 保留 |
| 12:0 | RW | 0x0 | `cube_in_channel` | 池化 cube 通道数（需减 1） |

---

## RKNN_ppu_rdma_src_base_addr（0x701C）

池化 cube 源基址。

| Bit | 属性 | 复位值 | 字段名 | 描述 |
|:---:|:----:|:------:|:-------|:-----|
| 31:0 | RW | 0x0 | `src_base_addr` | 池化 cube 基址 |

---

## RKNN_ppu_rdma_src_line_stride（0x7024）

源行步长（shape 宽度）。

| Bit | 属性 | 复位值 | 字段名 | 描述 |
|:---:|:----:|:------:|:-------|:-----|
| 31:4 | RW | 0x0 | `src_line_stride` | 池化 cube shape 宽度 |
| 3:0 | RO | 0x0 | — | 保留 |

---

## RKNN_ppu_rdma_src_surf_stride（0x7028）

源 surface 步长（shape 面积）。

| Bit | 属性 | 复位值 | 字段名 | 描述 |
|:---:|:----:|:------:|:-------|:-----|
| 31:4 | RW | 0x0 | `src_surf_stride` | 池化 cube shape 面积 |
| 3:0 | RO | 0x0 | — | 保留 |

---

## RKNN_ppu_rdma_data_format（0x7030）

输入数据格式。

| Bit | 属性 | 复位值 | 字段名 | 描述 |
|:---:|:----:|:------:|:-------|:-----|
| 31:2 | RO | 0x0 | — | 保留 |
| 1:0 | RW | 0x0 | `in_precision` | 输入精度。0：4bit；1：8bit；2：16bit；3：32bit |
