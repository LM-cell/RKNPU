# 矩阵乘法 API（rknn_matmul_api.h）

> 来源：`test/starrynpu/demo/yolov8/3rdparty/rknpu2/include/rknn_matmul_api.h`（544 行）

独立于模型推理的通用矩阵乘法加速接口。公式：**C = A × B**。

---

## 一、支持的计算类型

```c
typedef enum _rknn_matmul_type {
    RKNN_FLOAT16_MM_FLOAT16_TO_FLOAT32 = 1,   // FP16 × FP16 → FP32
    RKNN_INT8_MM_INT8_TO_INT32         = 2,   // INT8 × INT8 → INT32
    RKNN_INT8_MM_INT8_TO_INT8          = 3,   // INT8 × INT8 → INT8
    RKNN_FLOAT16_MM_FLOAT16_TO_FLOAT16 = 4,   // FP16 × FP16 → FP16
    RKNN_FLOAT16_MM_INT8_TO_FLOAT32    = 5,   // FP16 × INT8 → FP32
    RKNN_FLOAT16_MM_INT8_TO_FLOAT16    = 6,   // FP16 × INT8 → FP16
    RKNN_FLOAT16_MM_INT4_TO_FLOAT32    = 7,   // FP16 × INT4 → FP32
    RKNN_FLOAT16_MM_INT4_TO_FLOAT16    = 8,   // FP16 × INT4 → FP16
    RKNN_INT8_MM_INT8_TO_FLOAT32       = 9,   // INT8 × INT8 → FP32
    RKNN_INT4_MM_INT4_TO_INT16         = 10,  // INT4 × INT4 → INT16
    RKNN_INT8_MM_INT4_TO_INT32         = 11,  // INT8 × INT4 → INT32
    RKNN_FLOAT16_MM_INT4_TO_BFLOAT16   = 12,  // FP16 × INT4 → BF16
    RKNN_INT8_MM_INT4_TO_FLOAT16       = 15,  // INT8 × INT4 → FP16
} rknn_matmul_type;
```

---

## 二、对齐要求

### RK3588 / RK3576

| 精度 | K 对齐 | N 对齐 | K 最大值 |
|:-----|:-------|:-------|:---------|
| INT4 | 32 字节 | 64 字节 | 10240 |
| INT8 | 32 字节 | 32 字节 | 10240 |
| FP16 | 32 字节 | 16 字节 | 10240 |

### RK3566 / RK3568

| 精度 | K 对齐 | N 对齐 |
|:-----|:-------|:-------|
| INT8 | 32 字节 | 16 字节 |
| FP16 | 16 字节 | 8 字节 |

### RK3562

| 精度 | K 对齐 | N 对齐 |
|:-----|:-------|:-------|
| INT8 | 32 字节 | 16 字节 |
| FP16 | 32 字节 | 8 字节 |

---

## 三、Native Layout 规范

硬件要求输入/输出数据按特定分块格式排列。使用 native layout 可避免闭源库内部的格式转换开销。

### 3.1 A 矩阵（M × K）

**Normal layout**：`(M, K)` — 行主序。

**Native layout（RK3588/3576）**：

| 精度 | 布局 | 说明 |
|:-----|:-----|:-----|
| INT4 | `(K/32, M, 32)` | 每 32 个 K 元素为一组 |
| INT8 | `(K/16, M, 16)` | 每 16 个 K 元素为一组 |
| FP16 | `(K/8, M, 8)` | 每 8 个 K 元素为一组 |

示例（FP16）：
```
[K1M1, K2M1, ..., K8M1,
 K1M2, K2M2, ..., K8M2,
 ...
 K(k-7)Mm, K(k-6)Mm, ..., KkMm]
```

### 3.2 B 矩阵（K × N）

**Normal layout**：`(K, N)` — 行主序。

**Native layout（RK3588/3576）**：

| 精度 | 布局 | 说明 |
|:-----|:-----|:-----|
| INT4 | `(N/64, K/32, 64, 32)` | 64×32 分块 |
| INT8 | `(N/32, K/32, 32, 32)` | 32×32 分块 |
| FP16 | `(N/16, K/32, 16, 32)` | 16×32 分块 |

示例（INT8, RK3588）：
```
[K1N1,  K2N1,  ..., K32N1,
 K1N2,  K2N2,  ..., K32N2,
 ...
 K1N32, K2N32, ..., K32N32,    ← 第一个 32×32 块
 K33N1, K34N1, ..., K64N1,
 ...
 K(k-31)N32, ..., KkN32,      ← 第二个 K 块
 K1N33, K2N33, ..., K32N33,    ← 第二个 N 块
 ...]
```

### 3.3 C 矩阵（M × N）

**Normal layout**：`(M, N)` — 行主序。

**Native layout**：

| 平台 | 精度 | 布局 |
|:-----|:-----|:-----|
| 通用 | 通用 | `(N/4, M, 4)` |
| RK3588 | INT4 | `(N/8, M, 8)` |

### 3.4 K 分段规则

当 K 超过硬件限制时，B 矩阵自动分段：

| 平台 | 分段阈值 | 分段数 |
|:-----|:---------|:-------|
| RK3588 | K > 8192 | `T = ceil(K / 8192)` |
| RK3576 | K > 4096 | `T = ceil(K / 4096)` |

分段后每段独立按 native layout 排列。推荐使用 `rknn_B_normal_layout_to_native_layout()` 进行自动转换。

---

## 四、函数签名

### 4.1 创建与销毁

#### `rknn_matmul_create`

```c
int rknn_matmul_create(rknn_matmul_ctx* ctx, rknn_matmul_info* info, rknn_matmul_io_attr* io_attr);
```

| 参数 | 说明 |
|:-----|:-----|
| `ctx` | [out] matmul 上下文句柄 |
| `info` | [in] matmul 配置（M/K/N、类型、布局、量化） |
| `io_attr` | [out] 输入/输出属性（含实际维度和大小） |

#### `rknn_matmul_create_dynamic_shape`

```c
int rknn_matmul_create_dynamic_shape(rknn_matmul_ctx* ctx, rknn_matmul_info* info,
    int shape_num, rknn_matmul_shape dynamic_shapes[], rknn_matmul_io_attr io_attrs[]);
```

创建支持动态 M/K/N 的 matmul 上下文。`info.M/K/N` 无效，以 `dynamic_shapes[]` 为准。

#### `rknn_matmul_destroy`

```c
int rknn_matmul_destroy(rknn_matmul_ctx ctx);
```

### 4.2 IO 绑定

#### `rknn_matmul_set_io_mem`

```c
int rknn_matmul_set_io_mem(rknn_matmul_ctx ctx, rknn_tensor_mem* mem, rknn_matmul_tensor_attr* attr);
```

绑定 A/B/C 矩阵的内存。`attr` 来自 `rknn_matmul_create` 返回的 `io_attr`。

### 4.3 执行

#### `rknn_matmul_run`

```c
int rknn_matmul_run(rknn_matmul_ctx ctx);
```

阻塞执行矩阵乘法。

**逆向推断的内部流程**：

```
rknn_matmul_run()
├── 1. 根据 M/K/N 和精度生成命令流
│   ├── 填充 CNA 描述符（卷积参数映射为 matmul）
│   ├── 填充 CORE 描述符
│   └── 填充 DPU 描述符（输出转换）
├── 2. 构造 rknpu_task 数组
│   ├── enable_mask = 0xd（CNA + CORE + DPU）
│   ├── int_mask = 0x300（DPU 完成中断）
│   └── regcmd_addr = 命令流 DMA 地址
├── 3. 构造 rknpu_submit
│   ├── flags = RKNPU_JOB_PC | RKNPU_JOB_BLOCK | RKNPU_JOB_PINGPONG
│   └── core_mask = 根据 rknn_matmul_set_core_mask() 设置
└── 4. ioctl(DRM_IOCTL_RKNPU_SUBMIT)
```

### 4.4 核心控制

#### `rknn_matmul_set_core_mask`

```c
int rknn_matmul_set_core_mask(rknn_matmul_ctx context, rknn_core_mask core_mask);
```

### 4.5 量化参数

#### `rknn_matmul_set_quant_params`

```c
int rknn_matmul_set_quant_params(rknn_matmul_ctx context, rknn_quant_params* params);
```

设置量化参数。仅支持 `INT8_MM_INT8_TO_INT8` 和 `INT8_MM_INT8_TO_INT32` 类型。

#### `rknn_matmul_get_quant_params`

```c
int rknn_matmul_get_quant_params(rknn_matmul_ctx ctx, rknn_quant_params* params, float* scale);
```

### 4.6 动态 Shape

#### `rknn_matmul_set_dynamic_shape`

```c
int rknn_matmul_set_dynamic_shape(rknn_matmul_ctx ctx, rknn_matmul_shape* shape);
```

运行时切换 M/K/N（目前仅支持 M 动态）。

### 4.7 布局转换

#### `rknn_B_normal_layout_to_native_layout`

```c
int rknn_B_normal_layout_to_native_layout(void* B_input, void* B_output, int K, int N, rknn_matmul_info* info);
```

将 B 矩阵从 normal layout 转换为 native layout。处理 K 分段和平台差异。

---

## 五、配置结构体

### `rknn_matmul_info`

```c
typedef struct rknn_matmul_info_t {
    int32_t M;
    int32_t K;
    int32_t N;
    rknn_matmul_type type;       // 计算类型
    int16_t B_layout;            // 0=normal, 1=native
    int16_t B_quant_type;        // 0=per-layer, 1=per-channel, 2=per-group
    int16_t AC_layout;           // 0=normal, 1=native
    int16_t AC_quant_type;       // 仅支持 0
    int32_t iommu_domain_id;     // IOMMU 域 ID
    int16_t group_size;          // per-group 量化的组大小
    int8_t  reserved[34];
} rknn_matmul_info;
```

### `rknn_quant_params`

```c
typedef struct _rknn_quant_params {
    char     name[RKNN_MAX_NAME_LEN];
    float*   scale;              // 缩放因子数组
    int32_t  scale_len;
    int32_t* zp;                 // 零点数组
    int32_t  zp_len;
} rknn_quant_params;
```

---

## 六、与裸 ioctl demo 的对应关系

`npu_benchmark` 和 `npu_llama` 中的手工 matmul 实现，本质上是 `rknn_matmul_run()` 内部逻辑的开源复刻：

| 闭源 API | 裸 ioctl demo 对应 |
|:---------|:-------------------|
| `rknn_matmul_create()` | `gen_matmul_fp16()` / `gen_matmul_int8()` 生成命令流 |
| `rknn_matmul_set_io_mem()` | `mem_allocate()` + `weight_fp16()` / `feature_data()` 排列数据 |
| `rknn_matmul_run()` | 填充 `rknpu_task` + `rknpu_submit` + `ioctl(SUBMIT)` |
| `rknn_matmul_destroy()` | `munmap()` + `mem_destroy()` |
| native layout | `weight_fp16()` / `weight_int8()` / `feature_data()` 函数 |
