# GLOBAL 寄存器块（全局使能）

**基址**：`CORE_BASE + 0xF000`　｜　**地址范围**：`0xF000 ~ 0xFFFF`

> 来源：RK3588 TRM §36.4.3 Detail Registers Description

GLOBAL 模块仅包含一个寄存器，用于一次性组合使能各功能模块的操作。

---

## RKNN_global_operation_enable（0xF008）

组合操作使能：一次写入同时触发多个模块开始执行。

| Bit | 属性 | 复位值 | 字段名 | 描述 |
|:---:|:----:|:------:|:-------|:-----|
| 31:7 | RO | 0x0 | — | 保留 |
| 6 | RW | 0x0 | `ppu_rdma_op_en` | PPU_RDMA 操作使能 |
| 5 | RW | 0x0 | `ppu_op_en` | PPU 操作使能 |
| 4 | RW | 0x0 | `dpu_rdma_op_en` | DPU_RDMA 操作使能 |
| 3 | RW | 0x0 | `dpu_op_en` | DPU 操作使能 |
| 2 | RW | 0x0 | `core_op_en` | CORE 操作使能 |
| 1 | RO | 0x0 | — | 保留 |
| 0 | RW | 0x0 | `cna_op_en` | CNA 操作使能 |
