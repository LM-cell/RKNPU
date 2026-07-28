# DDMA / SDMA 寄存器块（Data DMA / System DMA）

**DDMA 基址**：`CORE_BASE + 0x8000`　｜　**地址范围**：`0x8000 ~ 0x8FFF`
**SDMA 基址**：`CORE_BASE + 0x9000`　｜　**地址范围**：`0x9000 ~ 0x9FFF`

> 来源：RK3588 TRM §36.4.3 Detail Registers Description

DDMA 和 SDMA 寄存器布局完全一致，仅基址不同（DDMA `0x8xxx`，SDMA `0x9xxx`）。DDMA 用于数据搬运，SDMA 用于系统级搬运。以下以 DDMA 为例展开位域，SDMA 将偏移 `0x8xxx` 替换为 `0x9xxx` 即可。

---

## cfg_outstanding（DDMA: 0x8000 / SDMA: 0x9000）

读写 outstanding 数配置。

| Bit | 属性 | 复位值 | 字段名 | 描述 |
|:---:|:----:|:------:|:-------|:-----|
| 31:16 | RO | 0x0 | — | 保留 |
| 15:8 | RW | 0x0 | `wr_os_cnt` | 最大写 outstanding 数 |
| 7:0 | RW | 0x0 | `rd_os_cnt` | 最大读 outstanding 数 |

---

## rd_weight_0（DDMA: 0x8004 / SDMA: 0x9004）

读仲裁权重 0：各模块读 burst 权重。

| Bit | 属性 | 复位值 | 字段名 | 描述 |
|:---:|:----:|:------:|:-------|:-----|
| 31:24 | RW | 0x0 | `rd_weight_pdp` | PPU 读 burst 权重 |
| 23:16 | RW | 0x0 | `rd_weight_dpu` | DPU 读 burst 权重 |
| 15:8 | RW | 0x0 | `rd_weight_kernel` | 权重读 burst 权重 |
| 7:0 | RW | 0x0 | `rd_weight_feature` | 特征读 burst 权重 |

---

## wr_weight_0（DDMA: 0x8008 / SDMA: 0x9008）

写仲裁权重。

| Bit | 属性 | 复位值 | 字段名 | 描述 |
|:---:|:----:|:------:|:-------|:-----|
| 31:16 | RO | 0x0 | — | 保留 |
| 15:8 | RW | 0x0 | `wr_weight_pdp` | PPU 写权重 |
| 7:0 | RW | 0x0 | `wr_weight_dpu` | DPU 写权重 |

---

## cfg_id_error（DDMA: 0x800C / SDMA: 0x900C）

错误 ID 记录。

| Bit | 属性 | 复位值 | 字段名 | 描述 |
|:---:|:----:|:------:|:-------|:-----|
| 31:10 | RO | 0x0 | — | 保留 |
| 9:6 | RW | 0x0 | `wr_resp_id` | 错误写 ID |
| 5 | RO | 0x0 | — | 保留 |
| 4:0 | RW | 0x0 | `rd_resp_id` | 错误读 ID |

---

## rd_weight_1（DDMA: 0x8010 / SDMA: 0x9010）

读仲裁权重 1：PC 读 burst 权重。

| Bit | 属性 | 复位值 | 字段名 | 描述 |
|:---:|:----:|:------:|:-------|:-----|
| 31:8 | RO | 0x0 | — | 保留 |
| 7:0 | RW | 0x0 | `rd_weight_pc` | PC 读 burst 权重 |

---

## cfg_dma_fifo_clr（DDMA: 0x8014 / SDMA: 0x9014）

清除 DMA FIFO。

| Bit | 属性 | 复位值 | 字段名 | 描述 |
|:---:|:----:|:------:|:-------|:-----|
| 31:1 | RO | 0x0 | — | 保留 |
| 0 | RW | 0x0 | `dma_fifo_clr` | 清除 DMA FIFO |

---

## cfg_dma_arb（DDMA: 0x8018 / SDMA: 0x9018）

DMA 仲裁模式配置。

| Bit | 属性 | 复位值 | 字段名 | 描述 |
|:---:|:----:|:------:|:-------|:-----|
| 31:10 | RO | 0x0 | — | 保留 |
| 9 | RW | 0x0 | `wr_arbit_model` | 写仲裁模型 |
| 8 | RW | 0x0 | `rd_arbit_model` | 读仲裁模型 |
| 7 | RO | 0x0 | — | 保留 |
| 6:4 | RW | 0x0 | `wr_fix_arb` | 写固定仲裁 |
| 3 | RO | 0x0 | — | 保留 |
| 2:0 | RW | 0x0 | `rd_fix_arb` | 读固定仲裁 |

---

## cfg_dma_rd_qos（DDMA: 0x8020 / SDMA: 0x9020）

各模块读 QoS 配置。

| Bit | 属性 | 复位值 | 字段名 | 描述 |
|:---:|:----:|:------:|:-------|:-----|
| 31:10 | RO | 0x0 | — | 保留 |
| 9:8 | RW | 0x0 | `rd_pc_qos` | PC 读 QoS |
| 7:6 | RW | 0x0 | `rd_ppu_qos` | PPU 读 QoS |
| 5:4 | RW | 0x0 | `rd_dpu_qos` | DPU 读 QoS |
| 3:2 | RW | 0x0 | `rd_kernel_qos` | Kernel 读 QoS |
| 1:0 | RW | 0x0 | `rd_feature_qos` | Feature 读 QoS |

---

## cfg_dma_rd_cfg（DDMA: 0x8024 / SDMA: 0x9024）

AXI 读通道信号配置。

| Bit | 属性 | 复位值 | 字段名 | 描述 |
|:---:|:----:|:------:|:-------|:-----|
| 31:13 | RO | 0x0 | — | 保留 |
| 12 | RW | 0x0 | `rd_arlock` | AXI arlock |
| 11:8 | RW | 0x0 | `rd_arcache` | AXI arcache |
| 7:5 | RW | 0x0 | `rd_arprot` | AXI arprot |
| 4:3 | RW | 0x0 | `rd_arburst` | AXI arburst |
| 2:0 | RW | 0x0 | `rd_arsize` | AXI arsize |

---

## cfg_dma_wr_cfg（DDMA: 0x8028 / SDMA: 0x9028）

AXI 写通道信号配置。

| Bit | 属性 | 复位值 | 字段名 | 描述 |
|:---:|:----:|:------:|:-------|:-----|
| 31:13 | RO | 0x0 | — | 保留 |
| 12 | RW | 0x0 | `wr_awlock` | AXI awlock |
| 11:8 | RW | 0x0 | `wr_awcache` | AXI awcache |
| 7:5 | RW | 0x0 | `wr_awprot` | AXI awprot |
| 4:3 | RW | 0x0 | `wr_awburst` | AXI awburst |
| 2:0 | RW | 0x0 | `wr_awsize` | AXI awsize |

---

## cfg_dma_wstrb（DDMA: 0x802C / SDMA: 0x902C）

AXI 写选通。

| Bit | 属性 | 复位值 | 字段名 | 描述 |
|:---:|:----:|:------:|:-------|:-----|
| 31:0 | RW | 0x0 | `wr_wstrb` | AXI 写选通信号 |

---

## cfg_status（DDMA: 0x8030 / SDMA: 0x9030）

DMA 状态。

| Bit | 属性 | 复位值 | 字段名 | 描述 |
|:---:|:----:|:------:|:-------|:-----|
| 31:9 | RO | 0x0 | — | 保留 |
| 8 | RW | 0x0 | `idel` | 空闲状态 |
| 7:0 | RO | 0x0 | — | 保留 |

---

## dt_wr_amount（DDMA: 0x8034 / SDMA: 0x9034）

数据写入量统计。用于 ioctl `GetDtWrAmount`。

| Bit | 属性 | 复位值 | 字段名 | 描述 |
|:---:|:----:|:------:|:-------|:-----|
| 31:0 | RO | 0x0 | `dt_wr_amount` | 数据写入量 |

---

## dt_rd_amount（DDMA: 0x8038 / SDMA: 0x9038）

数据读取量统计。用于 ioctl `GetDtRdAmount`。

| Bit | 属性 | 复位值 | 字段名 | 描述 |
|:---:|:----:|:------:|:-------|:-----|
| 31:0 | RO | 0x0 | `dt_rd_amount` | 数据读取量 |

---

## wt_rd_amount（DDMA: 0x803C / SDMA: 0x903C）

权重读取量统计。用于 ioctl `GetWtRdAmount`。

| Bit | 属性 | 复位值 | 字段名 | 描述 |
|:---:|:----:|:------:|:-------|:-----|
| 31:0 | RO | 0x0 | `wt_rd_amount` | 权重读取量 |

---

> **清除操作**：写 `rd_weight_1`(0x8010) 值 `0x80000101` 再写 `0x00000101`（两次写入进行 latch/clear）。
