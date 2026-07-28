# 闭源库内部机制

命令流生成、Task 构造、多核切分、数据格式转换等闭源库内部逻辑的逆向分析。

> 逆向来源：开源 demo `npu_matmul.c`、`bench_mark.c`、`llama0.c`、内核驱动 `rknpu_job.c`、StarryOS Rust 驱动。

---

## 一、命令流生成

### 1.1 命令流格式

每条命令是一个 64-bit 值，编码为：

```
 63        48 47        16 15         0
┌────────────┬─────────────┬───────────┐
│  模块 ID   │  寄存器值    │ 寄存器偏移 │
│  (16-bit)  │  (32-bit)   │ (16-bit)  │
└────────────┴─────────────┴───────────┘
```

```c
#define NPUOP(op, value, reg) \
    (((uint64_t)(op & 0xffff)) << 48) | \
    (((uint64_t)(value & 0xffffffff)) << 16) | \
    (uint64_t)(reg & 0xffff)
```

### 1.2 模块 ID 编码

```c
#define BLOCK_PC       0x0100
#define BLOCK_CNA      0x0200
#define BLOCK_CORE     0x0800
#define BLOCK_DPU      0x1000
#define BLOCK_DPU_RDMA 0x2000
#define BLOCK_PPU      0x4000
#define BLOCK_PPU_RDMA 0x8000

#define PC_OP_01     0x01    // 寄存器写入标志
#define PC_OP_40     0x40    // 未知用途
#define PC_OP_ENABLE 0x80    // 使能标志

#define OP_REG_CNA  (BLOCK_CNA  | PC_OP_01)  // 0x0201
#define OP_REG_CORE (BLOCK_CORE | PC_OP_01)  // 0x0801
#define OP_REG_DPU  (BLOCK_DPU  | PC_OP_01)  // 0x1001
#define OP_ENABLE   (PC_OP_ENABLE | PC_OP_01) // 0x0081
```

### 1.3 命令流结构（以 matmul 为例）

一个典型的 CNA→CORE→DPU 流水线命令流包含约 108 条指令：

```
ops[0]     DPU_S_POINTER = 0xE          ← DPU 寄存器组指针
ops[1~28]  CNA 寄存器（卷积参数）         ← 28 条
ops[29~40] CNA 权重解压缩寄存器          ← 12 条
ops[41~42] CNA 补充寄存器                ← 2 条
ops[43]    CNA_S_POINTER = 0xE          ← CNA 寄存器组指针
ops[44~49] CORE 寄存器                   ← 6 条
ops[50]    CORE_S_POINTER = 0xE         ← CORE 寄存器组指针
ops[51~106] DPU 寄存器（后处理参数）      ← 56 条
ops[107]   PC_OPERATION_ENABLE           ← 全局使能
```

最后一条指令触发硬件开始执行：

```c
ops[107] = NPUOP(OP_ENABLE,
    PC_ENABLE_DPU | PC_ENABLE_CNA | PC_ENABLE,  // 0x0d
    PC_OPERATION_ENABLE);                         // 0x0008
```

### 1.4 S_POINTER 寄存器

每个模块有一个 `S_POINTER` 寄存器（偏移 `0x_004`），用于切换寄存器组（乒乓机制）：

```c
ops[0]  = NPUOP(OP_REG_DPU,  0xE, DPU_S_POINTER);   // 0x4004
ops[43] = NPUOP(OP_REG_CNA,  0xE, CNA_S_POINTER);   // 0x1004
ops[50] = NPUOP(OP_REG_CORE, 0xE, CORE_S_POINTER);  // 0x3004
```

值 `0xE` 的含义尚未完全逆向，推测与寄存器组选择和同步有关。

---

## 二、CNA 参数填充

CNA（Convolution Neural-network Accelerator）负责卷积计算的数据加载和 MAC 阵列控制。

### 2.1 CNA 描述符

```c
typedef struct npu_cna_desc {
    uint8_t  conv_mode;          // 卷积模式（0=direct）
    uint8_t  in_precision;       // 输入精度（0=INT8, 2=FP16）
    uint8_t  proc_precision;     // 处理精度
    uint8_t  kernel_groups;      // 权重分组数
    uint16_t feature_grains;     // 特征粒度
    uint8_t  conv_x_stride;      // X 方向步长
    uint8_t  conv_y_stride;      // Y 方向步长
    uint16_t datain_width;       // 输入宽度
    uint16_t datain_height;      // 输入高度
    uint16_t datain_channel;     // 输入通道数
    uint16_t dataout_width;      // 输出宽度
    uint32_t dataout_atomics;    // 输出原子数
    uint32_t weight_bytes;       // 权重总字节数
    uint32_t weight_bytes_per_kernel; // 每个 kernel 的权重字节数
    uint8_t  weight_width;       // 权重宽度
    uint8_t  weight_height;      // 权重高度
    uint16_t weight_kernels;     // 权重 kernel 数
    uint8_t  weight_bank;        // 权重 CBUF bank 数
    uint8_t  data_bank;          // 数据 CBUF bank 数
    uint16_t data_entries;       // 数据 CBUF 条目数
    uint32_t feature_base_addr;  // 特征数据 DMA 基地址
    uint32_t line_stride;        // 行步长
    int32_t  surf_stride;        // 面步长
    // ... 更多字段
} npu_cna_desc;
```

### 2.2 CBUF Bank 分配

CBUF 是 CNA 内部的片上缓冲，共 12 个 bank，每个 32KB：

```c
#define NPU_CBUF_BANK_SIZE 32768   // 32KB
#define NPU_CBUF_BANKS     12

// 分配策略（从 gen_matmul_fp16 逆向）：
int weight_banks = ceil(weight_bytes / NPU_CBUF_BANK_SIZE);
int data_banks = NPU_CBUF_BANKS - weight_banks;
// 确保 data_banks >= 1
```

闭源库需要为每层计算最优的 bank 分配，平衡权重和数据的缓存需求。

### 2.3 Matmul → 卷积映射

NPU 没有专用的 matmul 单元，矩阵乘法通过卷积实现：

```
矩阵乘法 C[M×N] = A[M×K] × B[K×N]
    ↓ 映射为
1×1 卷积：
    输入特征: A 重排为 [M, K, 1, 1]（M 个样本，K 通道）
    权重:     B 重排为 [N, K, 1, 1]（N 个 1×1 kernel，K 通道）
    输出:     C 为 [M, N, 1, 1]
```

**逆向证据**：`gen_matmul_fp16()` 中设置 `conv_mode = direct_convolution`，`weight_width = weight_height = 1`，`conv_x_stride = conv_y_stride = 1`。

---

## 三、DPU 参数填充

DPU（Data Processing Unit）负责后处理：BS（Bias/Scale）、BN（Batch Norm）、EW（Element-wise）、LUT（激活函数）、输出转换。

### 3.1 DPU 描述符

```c
typedef struct npu_dpu_desc {
    uint8_t  flying_mode;       // 0=on-flying（从 CORE 直接接收）
    uint8_t  output_mode;       // 输出模式
    uint8_t  conv_mode;         // 卷积模式
    uint8_t  out_precision;     // 输出精度
    uint8_t  in_precision;      // 输入精度
    uint8_t  proc_precision;    // 处理精度
    uint32_t dst_base_addr;     // 输出 DMA 基地址
    uint32_t dst_surf_stride;   // 输出面步长
    uint16_t width, height;     // 输出尺寸
    uint16_t channel;           // 输出通道
    // BS 旁路控制
    uint8_t  bs_bypass;         // 1=旁路 BS
    uint8_t  bs_alu_bypass;     // 1=旁路 BS ALU
    uint8_t  bs_mul_bypass;     // 1=旁路 BS MUL
    uint8_t  bs_relu_bypass;    // 1=旁路 BS ReLU
    // BN 旁路控制
    uint8_t  bn_bypass;
    uint8_t  bn_alu_bypass;
    uint8_t  bn_mul_bypass;
    uint8_t  bn_relu_bypass;
    // EW 旁路控制
    uint8_t  ew_bypass;
    uint8_t  ew_op_bypass;
    uint8_t  ew_lut_bypass;
    uint8_t  ew_op_cvt_bypass;
    uint8_t  ew_relu_bypass;
    // 输出转换
    uint8_t  fp32tofp16_en;     // FP32→FP16 使能
    uint16_t out_cvt_scale;     // 输出缩放因子
    uint32_t surf_add;          // 面地址增量
} npu_dpu_desc;
```

### 3.2 旁路模式

对于简单的 matmul，DPU 大部分功能被旁路：

```c
// gen_matmul_task() 中的典型设置：
dpu_desc.bs_bypass = 1;      // 无 bias
dpu_desc.bs_alu_bypass = 1;
dpu_desc.bs_mul_bypass = 1;
dpu_desc.bs_relu_bypass = 1;
dpu_desc.bn_bypass = 1;      // 无 batch norm
dpu_desc.bn_alu_bypass = 1;
dpu_desc.bn_mul_bypass = 1;
dpu_desc.bn_relu_bypass = 1;
dpu_desc.ew_bypass = 1;      // 无 element-wise
dpu_desc.ew_op_bypass = 1;
dpu_desc.ew_lut_bypass = 1;
dpu_desc.ew_op_cvt_bypass = 1;
dpu_desc.ew_relu_bypass = 1;
```

闭源库在编译复杂模型时，会根据每层的算子类型选择性启用这些功能。

---

## 四、Task 构造

### 4.1 Task 结构

```c
struct rknpu_task {
    uint32_t flags;           // 任务标志
    uint32_t op_idx;          // 算子索引
    uint32_t enable_mask;     // 模块使能掩码
    uint32_t int_mask;        // 期望的中断掩码
    uint32_t int_clear;       // 中断清除值
    uint32_t int_status;      // [内核写回] 实际中断状态
    uint32_t regcfg_amount;   // 命令流中的指令数量
    uint32_t regcfg_offset;   // 命令流偏移（字节）
    uint64_t regcmd_addr;     // 命令流 DMA 地址
};
```

### 4.2 关键字段计算

**`enable_mask`**（偏移 `0xF008`）：

```c
#define PC_ENABLE      0x01   // 全局使能
#define PC_ENABLE_CNA  0x04   // CNA 中断使能
#define PC_ENABLE_DPU  0x08   // DPU 中断使能
#define PC_ENABLE_PPU  0x10   // PPU 中断使能

// CNA + CORE + DPU 流水线：
enable_mask = PC_ENABLE | PC_ENABLE_CNA | PC_ENABLE_DPU;  // 0x0d
```

**`int_mask`**（偏移 `0x0020`）：

```c
// 中断位定义（从内核驱动 rknpu_ioctl.h 推断）：
// bit[8]  = DPU group 0 完成
// bit[9]  = DPU group 1 完成
// 对于单 task matmul：
int_mask = 0x300;  // 等待 DPU group 0 和 group 1 完成
```

**`int_clear`**：

```c
int_clear = 0x1ffff;  // 清除所有 17 位中断
```

**`regcfg_amount`**：

```c
// 命令流指令数 - 额外保留量 - 尾部保留
regcfg_amount = total_ops - RKNPU_PC_DATA_EXTRA_AMOUNT - 4;
// RKNPU_PC_DATA_EXTRA_AMOUNT = 4（内核驱动会额外加回）
```

**逆向证据**：内核驱动 `rknpu_job_subcore_commit_pc()` 中：
```c
amount = task->regcfg_amount + rknpu->config->pc_data_extra_amount;
```

### 4.3 多 Task 场景

对于多层网络，闭源库生成多个 task，每个 task 对应一层或一组层：

```
Task[0]: 第 1 层卷积（CNA+CORE+DPU）
    regcmd_addr → 命令流偏移 0
    regcfg_amount = 108
Task[1]: 第 2 层卷积
    regcmd_addr → 命令流偏移 108*8
    regcfg_amount = 108
...
Task[N-1]: 最后一层
```

---

## 五、多核切分

### 5.1 Submit 结构

```c
struct rknpu_submit {
    uint32_t flags;
    uint32_t timeout;
    uint32_t task_start;
    uint32_t task_number;        // 总 task 数
    uint32_t core_mask;          // 使用的核心掩码
    struct rknpu_subcore_task subcore_task[5]; // 每核心的 task 范围
};

struct rknpu_subcore_task {
    uint32_t task_start;         // 起始 task 索引
    uint32_t task_number;        // task 数量
};
```

### 5.2 单核模式

```c
submit.core_mask = 0x1;  // 仅核心 0
submit.subcore_task[0] = { .task_start = 0, .task_number = N };
submit.subcore_task[1] = { .task_start = N, .task_number = 0 };  // 哨兵
submit.subcore_task[2] = { .task_start = N, .task_number = 0 };
```

**逆向证据**：`bench_mark.c` 和 `llama0.c` 均使用此模式。

### 5.3 多核模式（闭源库独有）

闭源库将 task 数组切分到多个核心：

```
假设 12 个 task，3 核心模式：
submit.core_mask = 0x7;  // 核心 0+1+2
submit.subcore_task[0] = { 0, 4 };   // 核心 0: task 0~3
submit.subcore_task[1] = { 4, 4 };   // 核心 1: task 4~7
submit.subcore_task[2] = { 8, 4 };   // 核心 2: task 8~11
```

切分策略是闭源库的核心竞争力之一，涉及：
- 层间数据依赖分析
- 计算量均衡
- 内存带宽分配
- CBUF 冲突避免

### 5.4 乒乓模式

`RKNPU_JOB_PINGPONG` 标志启用硬件乒乓机制：

```
Task[0] 在 group 0 执行
    ↓ 完成，触发 group 0 中断
Task[1] 在 group 1 执行（与 Task[0] 的 DPU 输出重叠）
    ↓ 完成，触发 group 1 中断
Task[2] 在 group 0 执行
    ...
```

这允许流水线执行：当 Task[N] 在 DPU 阶段输出时，Task[N+1] 已经在 CNA 阶段加载数据。

---

## 六、数据格式转换

### 6.1 特征数据排列（`feature_data`）

从 `npu_matmul.h` 导出的函数，将行主序坐标转换为 NPU native layout 索引：

```c
int feature_data(int C, int H, int W, int C2, int c, int h, int w);
```

| 参数 | 说明 |
|:-----|:-----|
| C | 通道总数 |
| H | 高度 |
| W | 宽度 |
| C2 | 通道分组大小（FP16=8, INT8=16, FP32=4） |
| c, h, w | 1-indexed 坐标 |

**逆向推断的公式**：

```
native_index = (c-1)/C2 * (H*W*C2) + (h-1)*W*C2 + (w-1)*C2 + (c-1)%C2
```

这对应 `NC1HWC2` 格式：`(N, ceil(C/C2), H, W, C2)`。

### 6.2 权重排列

#### FP16 权重（`weight_fp16`）

```c
int weight_fp16(int C, int k, int c);
```

| 参数 | 说明 |
|:-----|:-----|
| C | 输入通道数（K 维度） |
| k | kernel 索引（1-indexed） |
| c | 通道索引（1-indexed） |

对应 native layout `(N/16, K/32, 16, 32)` 的 FP16 变体。

#### INT8 权重（`weight_int8`）

```c
int weight_int8(int C, int k, int c);
```

对应 native layout `(N/32, K/32, 32, 32)`。

### 6.3 闭源库的格式转换链

```
用户输入（NHWC, UINT8）
    ↓ rknn_inputs_set()
    ├── NHWC → NCHW（如果模型需要）
    ├── UINT8 → INT8（减去 zp）
    └── NCHW → NC1HWC2（NPU native）
    ↓
NPU 执行
    ↓
NPU 输出（NC1HWC2, INT8/FP16）
    ↓ rknn_outputs_get()
    ├── NC1HWC2 → NCHW/NHWC
    ├── INT8 → FP32（(val - zp) * scale）
    └── 拷贝到用户缓冲
```

零拷贝路径跳过所有转换，用户直接操作 native layout。

---

## 七、闭源库 vs 裸 ioctl 对照表

| 闭源库内部操作 | 裸 ioctl demo 对应 | 文件 |
|:--------------|:-------------------|:-----|
| 模型解析 | 无（用户手动定义参数） | — |
| CNA 描述符填充 | `gen_matmul_fp16()` | `npu_matmul.c` |
| DPU 描述符填充 | `gen_matmul_task()` | `npu_matmul.c` |
| NPUOP 编码 | `NPUOP()` 宏 | `npu_hw.h` |
| Task 构造 | 手动填充 `tasks[0]` | `bench_mark.c` |
| Submit 构造 | 手动填充 `submit` | `bench_mark.c` |
| 权重转换 | `weight_fp16()` / `weight_int8()` | `npu_matmul.c` |
| 特征排列 | `feature_data()` | `npu_matmul.c` |
| 内存分配 | `mem_allocate()` | `npu_interface.c` |
| 内存释放 | `mem_destroy()` | `npu_interface.c` |
| 设备打开 | `npu_open()` | `npu_interface.c` |
| 设备复位 | `npu_reset()` | `npu_interface.c` |
| Buffer 池化 | `NPUBuffer` 池 | `llama0.c` |
| 权重缓存 | `NPUWeightCache` | `llama0.c` |
