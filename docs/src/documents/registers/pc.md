# PC 寄存器块（Program Counter / 任务控制器）

**基址**：`CORE_BASE + 0x0000`　｜　**地址范围**：`0x0000 ~ 0x0FFF`

> 来源：RK3588 TRM §36.4.3 Detail Registers Description

PC 是 NPU 的命令流执行引擎，负责：从 DMA 地址读取寄存器命令流 → 按序写入各功能模块寄存器 → 触发执行 → 产生完成中断。

---

## RKNN_pc_version（0x0000）

硬件版本寄存器（只读）。驱动通过 `version + (version_num & 0xFFFF)` 计算完整版本号。

| Bit | 属性 | 复位值 | 字段名 | 描述 |
|:---:|:----:|:------:|:-------|:-----|
| 31:0 | RO | 0x0 | `version` | 硬件版本标识 |

---

## RKNN_pc_version_num（0x0004）

硬件版本号寄存器（只读）。

| Bit | 属性 | 复位值 | 字段名 | 描述 |
|:---:|:----:|:------:|:-------|:-----|
| 31:16 | RO | 0x0 | — | 保留 |
| 15:0 | RO | 0x0 | `version_num` | 硬件版本编号 |

---

## RKNN_pc_operation_enable（0x0008）

操作使能寄存器。

| Bit | 属性 | 复位值 | 字段名 | 描述 |
|:---:|:----:|:------:|:-------|:-----|
| 31:1 | RO | 0x0 | — | 保留 |
| 0 | RW | 0x0 | `op_en` | PC 操作使能。0：禁用 PC 模块；1：使能 PC 模块，为每个 task 取寄存器配置 |

---

## RKNN_pc_base_address（0x0010）

PC 基址寄存器，指定 DMA 指令流所在的内存地址。

| Bit | 属性 | 复位值 | 字段名 | 描述 |
|:---:|:----:|:------:|:-------|:-----|
| 31:4 | RW | 0x0 | `pc_source_addr` | PC 基址。DMA 指令流所在的内存地址 |
| 3:1 | RO | 0x0 | — | 保留 |
| 0 | RW | 0x0 | `pc_sel` | PC 模式选择。0：PC 模式，通过 AXI DMA 取寄存器配置；1：Slave 模式，通过 AHB 设置寄存器 |

---

## RKNN_pc_register_amounts（0x0014）

每个 task 需要取的寄存器数量。

| Bit | 属性 | 复位值 | 字段名 | 描述 |
|:---:|:----:|:------:|:-------|:-----|
| 31:16 | RO | 0x0 | — | 保留 |
| 15:0 | RW | 0x0 | `pc_data_amount` | 数据量。一个 task 需要取的寄存器数量 |

每条寄存器指令占 64 bit，格式如下：

| 位域 | 含义 |
|:----:|:-----|
| `[63:48]` | 目标模块选择（哪个 block） |
| `[47:16]` | 寄存器值 |
| `[15:0]` | 各 block 内的偏移地址 |

**模块选择位**：

| Bit | 目标模块 |
|:---:|:---------|
| 56 | PC |
| 57 | CNA |
| 59 | CORE |
| 60 | DPU |
| 61 | DPU_RDMA |
| 62 | PPU |
| 63 | PPU_RDMA |
| 55 | 设置各 block 的 `op_en` |

> **示例**：`64'h0081_0000_007f_0008` 将设置各 block 的 op_en（CNA, CORE, ..., PPU_RDMA）。
>
> **注意**：`op_en` 强烈建议放在寄存器列表末尾。在 `op_en` 之前，必须先设置 `64'h0041_xxxx_xxxx_xxxx`。

---

## RKNN_pc_interrupt_mask（0x0020）

中断掩码寄存器。置 1 使能对应中断。

| Bit | 属性 | 复位值 | 字段名 | 描述 |
|:---:|:----:|:------:|:-------|:-----|
| 31:17 | RO | 0x0 | — | 保留 |
| 16:0 | RW | 0x1FFFF | `int_mask` | 中断掩码（见下表） |

| Bit | 中断源 |
|:---:|:-------|
| 0 | CNA feature group 0 |
| 1 | CNA feature group 1 |
| 2 | CNA weight group 0 |
| 3 | CNA weight group 1 |
| 4 | CNA csc group 0 |
| 5 | CNA csc group 1 |
| 6 | CORE group 0 |
| 7 | CORE group 1 |
| 8 | DPU group 0 |
| 9 | DPU group 1 |
| 10 | PPU group 0 |
| 11 | PPU group 1 |
| 12 | DMA read error |
| 13 | DMA write error |

> **注意**：在 PC 模式下，int_mask 设置的是最后一个 task 的中断掩码。

---

## RKNN_pc_interrupt_clear（0x0024）

中断清除寄存器。写 1 清除对应中断位。

| Bit | 属性 | 复位值 | 字段名 | 描述 |
|:---:|:----:|:------:|:-------|:-----|
| 31:17 | RO | 0x0 | — | 保留 |
| 16:0 | W1C | 0x0 | `int_clr` | 中断清除（位定义同 `int_mask`） |

> **`INT_CLEAR_ALL = 0x1FFFF`**（清除 bit0~bit16 全部中断）<span style="background:#e3f2fd;padding:1px 4px;border-radius:3px;font-size:0.8em">rknpu-ioctl.h</span>

---

## RKNN_pc_interrupt_status（0x0028）

中断状态寄存器（经过 mask 后的状态）。

| Bit | 属性 | 复位值 | 字段名 | 描述 |
|:---:|:----:|:------:|:-------|:-----|
| 31:17 | RO | 0x0 | — | 保留 |
| 16:0 | W1C | 0x0 | `int_st` | 中断状态，与 mask 位做 AND（位定义同 `int_mask`） |

---

## RKNN_pc_interrupt_raw_status（0x002C）

中断原始状态寄存器（未经 mask 的原始状态）。

| Bit | 属性 | 复位值 | 字段名 | 描述 |
|:---:|:----:|:------:|:-------|:-----|
| 31:17 | RO | 0x0 | — | 保留 |
| 16:0 | W1C | 0x0 | `int_raw_st` | 中断原始状态（位定义同 `int_mask`） |

---

## RKNN_pc_task_con（0x0030）

任务控制寄存器。

| Bit | 属性 | 复位值 | 字段名 | 描述 |
|:---:|:----:|:------:|:-------|:-----|
| 31:14 | RO | 0x0 | — | 保留 |
| 13 | W1C | 0x0 | `task_count_clear` | 任务计数器清除。清除当前 task 计数器，建议在 task 启动前清除 |
| 12 | RW | 0x0 | `task_pp_en` | Ping-pong 模式使能。0：关闭，第二组寄存器在第一组 task 完成后才取；1：开启，第二组寄存器在第一组取完后立即开始取 |
| 11:0 | RW | 0x0 | `task_number` | 要执行的 task 总数 |

---

## RKNN_pc_task_dma_base_addr（0x0034）

任务 DMA 基址寄存器。

| Bit | 属性 | 复位值 | 字段名 | 描述 |
|:---:|:----:|:------:|:-------|:-----|
| 31:4 | RW | 0x0 | `dma_base_addr` | 任务基址。各 DMA（feature DMA、weight DMA、DPU DMA、PPU DMA）的地址设为偏移地址，AXI 总线上的最终地址 = 基址 + 偏移地址 |
| 3:0 | RO | 0x0 | — | 保留 |

---

## RKNN_pc_task_status（0x003C）

任务状态寄存器（只读）。

| Bit | 属性 | 复位值 | 字段名 | 描述 |
|:---:|:----:|:------:|:-------|:-----|
| 31:28 | RO | 0x0 | — | 保留 |
| 27:0 | RW | 0x0 | `task_status` | 任务状态（见下表） |

| 位域 | 含义 |
|:----:|:-----|
| [11:0] | 当前 task 计数器值 |
| [12] | 指示第一个 task 正在执行 / 第一个 task 的寄存器正在取 |
| [13] | 指示最后一个 task 正在执行 / 最后一个 task 的寄存器正在取 |

---

## 附：驱动层补充

### 中断状态归一化（`rknpu_fuzz_status()`）

<span style="background:#fff3e0;padding:1px 4px;border-radius:3px;font-size:0.8em">StarryOS Rust 驱动</span> 在判定完成前，对 `interrupt_status` 做如下归一化处理：

| 位组 | 掩码 | 归一化规则 | 对应模块 |
|------|------|-----------|---------|
| bit[1:0] | `0x03` | 任一非零 → 置 `0x03` | CNA_FG |
| bit[3:2] | `0x0C` | 任一非零 → 置 `0x0C` | CNA_WG |
| bit[5:4] | `0x30` | 任一非零 → 置 `0x30` | CNA_CSC |
| bit[7:6] | `0xC0` | 任一非零 → 置 `0xC0` | CORE |
| bit[9:8] | `0x300` | 任一非零 → 置 `0x300` | DPU |
| bit[11:10] | `0xC00` | 任一非零 → 置 `0xC00` | PPU |

**含义** <span style="background:#fce4ec;padding:1px 4px;border-radius:3px;font-size:0.8em">逆向推断</span>：每个功能模块有 2 个中断 bit（G0/G1），硬件可能只置其中一个，但驱动判定完成时需要两个都为 1，因此做归一化。
