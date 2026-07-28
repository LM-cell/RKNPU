# 推理 API 参考（rknn_api.h）

> 来源：`test/starrynpu/demo/yolov8/3rdparty/rknpu2/include/rknn_api.h`
> 以下资料均来着StarryOS或社区收集。大多数暂未经过验证！。
---

## 一、类型定义

### 1.1 上下文句柄

```c
#ifdef __arm__
typedef uint32_t rknn_context;
#else
typedef uint64_t rknn_context;
```

所有 API 围绕 `rknn_context` 句柄操作，内部指向闭源库维护的不透明状态。

### 1.2 错误码

| 常量 | 值 | 说明 |
|:-----|:---|:-----|
| `RKNN_SUCC` | 0 | 成功 |
| `RKNN_ERR_FAIL` | -1 | 通用失败 |
| `RKNN_ERR_TIMEOUT` | -2 | 执行超时 |
| `RKNN_ERR_DEVICE_UNAVAILABLE` | -3 | 设备不可用 |
| `RKNN_ERR_MALLOC_FAIL` | -4 | 内存分配失败 |
| `RKNN_ERR_PARAM_INVALID` | -5 | 参数无效 |
| `RKNN_ERR_MODEL_INVALID` | -6 | 模型无效 |
| `RKNN_ERR_CTX_INVALID` | -7 | 上下文无效 |
| `RKNN_ERR_INPUT_INVALID` | -8 | 输入无效 |
| `RKNN_ERR_OUTPUT_INVALID` | -9 | 输出无效 |
| `RKNN_ERR_DEVICE_UNMATCH` | -10 | 设备不匹配（需更新 SDK/驱动） |
| `RKNN_ERR_INCOMPATILE_PRE_COMPILE_MODEL` | -11 | 预编译模型不兼容 |
| `RKNN_ERR_INCOMPATILE_OPTIMIZATION_LEVEL_VERSION` | -12 | 优化级别版本不兼容 |
| `RKNN_ERR_TARGET_PLATFORM_UNMATCH` | -13 | 目标平台不匹配 |

### 1.3 张量类型

```c
typedef enum _rknn_tensor_type {
    RKNN_TENSOR_FLOAT32 = 0,
    RKNN_TENSOR_FLOAT16,      // 1
    RKNN_TENSOR_INT8,         // 2
    RKNN_TENSOR_UINT8,        // 3
    RKNN_TENSOR_INT16,        // 4
    RKNN_TENSOR_UINT16,       // 5
    RKNN_TENSOR_INT32,        // 6
    RKNN_TENSOR_UINT32,       // 7
    RKNN_TENSOR_INT64,        // 8
    RKNN_TENSOR_BOOL,         // 9
    RKNN_TENSOR_INT4,         // 10
    RKNN_TENSOR_BFLOAT16,     // 11
} rknn_tensor_type;
```

### 1.4 量化类型

```c
typedef enum _rknn_tensor_qnt_type {
    RKNN_TENSOR_QNT_NONE = 0,              // 无量化
    RKNN_TENSOR_QNT_DFP,                   // 动态定点（fractional length）
    RKNN_TENSOR_QNT_AFFINE_ASYMMETRIC,     // 非对称仿射（zero_point + scale）
} rknn_tensor_qnt_type;
```

### 1.5 数据格式

```c
typedef enum _rknn_tensor_format {
    RKNN_TENSOR_NCHW = 0,
    RKNN_TENSOR_NHWC,
    RKNN_TENSOR_NC1HWC2,    // NPU 原生格式
    RKNN_TENSOR_UNDEFINED,
} rknn_tensor_format;
```

### 1.6 核心掩码

```c
typedef enum _rknn_core_mask {
    RKNN_NPU_CORE_AUTO   = 0,       // 自动选择
    RKNN_NPU_CORE_0      = 1,       // 核心 0
    RKNN_NPU_CORE_1      = 2,       // 核心 1
    RKNN_NPU_CORE_2      = 4,       // 核心 2
    RKNN_NPU_CORE_0_1    = 3,       // 核心 0+1 联合
    RKNN_NPU_CORE_0_1_2  = 7,       // 三核联合
    RKNN_NPU_CORE_ALL    = 0xffff,  // 平台自动选择多核
} rknn_core_mask;
```

### 1.7 初始化标志

| 标志 | 值 | 说明 | 逆向推断的内部行为 |
|:-----|:---|:-----|:-------------------|
| `RKNN_FLAG_PRIOR_HIGH` | 0x0 | 高优先级（默认） | 设置 `nice -19` |
| `RKNN_FLAG_PRIOR_MEDIUM` | 0x1 | 中优先级 | — |
| `RKNN_FLAG_PRIOR_LOW` | 0x2 | 低优先级 | — |
| `RKNN_FLAG_ASYNC_MASK` | 0x4 | 异步模式 | `rknn_outputs_get` 返回上一帧结果 |
| `RKNN_FLAG_COLLECT_PERF_MASK` | 0x8 | 性能采集 | 启用逐层计时，降低帧率 |
| `RKNN_FLAG_MEM_ALLOC_OUTSIDE` | 0x10 | 外部内存分配 | 用户负责分配 weight/internal/IO 内存 |
| `RKNN_FLAG_SHARE_WEIGHT_MEM` | 0x20 | 权重共享 | 多上下文共享同一权重内存 |
| `RKNN_FLAG_FENCE_IN_OUTSIDE` | 0x40 | 外部输入 fence | 传入 DMA fence fd |
| `RKNN_FLAG_FENCE_OUT_OUTSIDE` | 0x80 | 外部输出 fence | 获取 DMA fence fd |
| `RKNN_FLAG_COLLECT_MODEL_INFO_ONLY` | 0x100 | 仅采集模型信息 | 不实际加载，仅查询 weight/internal 大小 |
| `RKNN_FLAG_INTERNAL_ALLOC_OUTSIDE` | 0x200 | 外部分配内部内存 | — |
| `RKNN_FLAG_EXECUTE_FALLBACK_PRIOR_DEVICE_GPU` | 0x400 | GPU 回退 | NPU 不支持的算子回退 GPU（OpenCL） |
| `RKNN_FLAG_ENABLE_SRAM` | 0x800 | 启用 SRAM | 尝试在 SRAM 分配缓冲 |
| `RKNN_FLAG_SHARE_SRAM` | 0x1000 | 共享 SRAM | 多上下文共享 SRAM |
| `RKNN_FLAG_DISABLE_PROC_HIGH_PRIORITY` | 0x2000 | 禁用高优先级 | 不设置 `nice -19` |
| `RKNN_FLAG_DISABLE_FLUSH_INPUT_MEM_CACHE` | 0x4000 | 禁用输入 cache flush | 用户自行保证 cache 一致性 |
| `RKNN_FLAG_DISABLE_FLUSH_OUTPUT_MEM_CACHE` | 0x8000 | 禁用输出 cache flush | 输出由 GPU/RGA 消费时使用 |
| `RKNN_FLAG_MODEL_BUFFER_ZERO_COPY` | 0x10000 | 模型缓冲零拷贝 | 模型数据由 NPU 直接访问 |

---

## 二、核心结构体

### 2.1 张量属性 `rknn_tensor_attr`

```c
typedef struct _rknn_tensor_attr {
    uint32_t index;                        // 输入/输出索引
    uint32_t n_dims;                       // 维度数
    uint32_t dims[RKNN_MAX_DIMS];          // 维度数组（最多 16 维）
    char     name[RKNN_MAX_NAME_LEN];      // 张量名（最长 256）
    uint32_t n_elems;                      // 元素总数
    uint32_t size;                         // 字节大小
    rknn_tensor_format fmt;                // 数据格式（NCHW/NHWC/NC1HWC2）
    rknn_tensor_type   type;               // 数据类型
    rknn_tensor_qnt_type qnt_type;         // 量化类型
    int8_t   fl;                           // DFP 小数位长度
    int32_t  zp;                           // 仿射量化零点
    float    scale;                        // 仿射量化缩放因子
    uint32_t w_stride;                     // 宽度方向步长（只读）
    uint32_t size_with_stride;             // 含步长的字节大小
    uint8_t  pass_through;                 // 直通模式标志
    uint32_t h_stride;                     // 高度方向步长（只写）
} rknn_tensor_attr;
```

### 2.2 张量内存 `rknn_tensor_mem`

```c
typedef struct _rknn_tensor_memory {
    void*    virt_addr;     // 虚拟地址
    uint64_t phys_addr;     // 物理地址
    int32_t  fd;            // DMA buffer fd
    int32_t  offset;        // 内存偏移
    uint32_t size;          // 缓冲大小
    uint32_t flags;         // 标志
    void*    priv_data;     // 私有数据（闭源库内部使用）
} rknn_tensor_mem;
```

**逆向推断**：`priv_data` 内部指向闭源库维护的 `rknpu_mem_create` 返回的 `obj_addr`，用于后续 `mem_destroy` 和 `mem_sync` 操作。

### 2.3 查询命令 `rknn_query_cmd`

| 命令 | 值 | 返回结构体 | 说明 |
|:-----|:---|:-----------|:-----|
| `RKNN_QUERY_IN_OUT_NUM` | 0 | `rknn_input_output_num` | 输入/输出数量 |
| `RKNN_QUERY_INPUT_ATTR` | 1 | `rknn_tensor_attr` | 输入张量属性 |
| `RKNN_QUERY_OUTPUT_ATTR` | 2 | `rknn_tensor_attr` | 输出张量属性 |
| `RKNN_QUERY_PERF_DETAIL` | 3 | `rknn_perf_detail` | 逐层性能（需 PERF_MASK） |
| `RKNN_QUERY_PERF_RUN` | 4 | `rknn_perf_run` | 推理总耗时 |
| `RKNN_QUERY_SDK_VERSION` | 5 | `rknn_sdk_version` | SDK/驱动版本 |
| `RKNN_QUERY_MEM_SIZE` | 6 | `rknn_mem_size` | 权重/内部内存大小 |
| `RKNN_QUERY_CUSTOM_STRING` | 7 | `rknn_custom_string` | 自定义字符串 |
| `RKNN_QUERY_NATIVE_INPUT_ATTR` | 8 | `rknn_tensor_attr` | 原生输入属性（NC1HWC2） |
| `RKNN_QUERY_NATIVE_OUTPUT_ATTR` | 9 | `rknn_tensor_attr` | 原生输出属性 |
| `RKNN_QUERY_DEVICE_MEM_INFO` | 12 | — | 设备内存信息 |
| `RKNN_QUERY_INPUT_DYNAMIC_RANGE` | 13 | `rknn_input_range` | 动态 shape 范围 |
| `RKNN_QUERY_CURRENT_INPUT_ATTR` | 14 | `rknn_tensor_attr` | 当前输入 shape（动态模型） |
| `RKNN_QUERY_CURRENT_OUTPUT_ATTR` | 15 | `rknn_tensor_attr` | 当前输出 shape（动态模型） |

---

## 三、函数签名与逆向还原

### 3.1 生命周期管理

#### `rknn_init`

```c
int rknn_init(rknn_context* context, void* model, uint32_t size, uint32_t flag, rknn_init_extend* extend);
```

| 参数 | 说明 |
|:-----|:-----|
| `context` | [out] 上下文句柄指针 |
| `model` | `size > 0` 时为模型数据指针；`size = 0` 时为模型文件路径 |
| `size` | 模型数据大小（0 表示从文件加载） |
| `flag` | 初始化标志组合（见 1.7） |
| `extend` | 扩展信息（可选，含 `real_model_offset`、`model_buffer_fd` 等） |

**逆向还原的内部流程**：

```
rknn_init()
├── 1. 打开 /dev/dri/card* 或 /dev/rknpu（获取 fd）
├── 2. ioctl(ACTION, RKNPU_GET_HW_VERSION) → 检查硬件版本
├── 3. 解析 .rknn 模型文件头
│   ├── 提取网络拓扑（层数、连接关系）
│   ├── 提取权重数据
│   └── 提取预编译命令流（如果是 pre-compile 模型）
├── 4. ioctl(MEM_CREATE) × N → 分配权重/内部/IO 内存
│   ├── 权重内存（RKNPU_MEM_KERNEL_MAPPING）
│   ├── 内部中间缓冲
│   └── 命令流缓冲（regcmd）
├── 5. ioctl(MEM_MAP) + mmap() → 映射到用户空间
├── 6. 将权重数据拷贝到 DMA 内存（转换为 native layout）
├── 7. 编译模型 → 生成寄存器命令流
│   ├── 为每层生成 NPUOP 指令序列
│   ├── 计算 CBUF bank 分配
│   └── 生成 Task 数组
├── 8. 如果 flag & RKNN_FLAG_PRIOR_HIGH → ioctl(ACTION, RKNPU_SET_PROC_NICE)
└── 9. 返回 context 句柄
```

#### `rknn_destroy`

```c
int rknn_destroy(rknn_context context);
```

**逆向还原**：

```
rknn_destroy()
├── 1. 释放所有 Task 数组内存
├── 2. munmap() + ioctl(MEM_DESTROY) × N → 释放所有 DMA 内存
├── 3. close(drm_fd)
└── 4. 释放上下文结构体
```

#### `rknn_dup_context`

```c
int rknn_dup_context(rknn_context* context_in, rknn_context* context_out);
```

复制上下文，新上下文与原上下文共享权重内存（`RKNN_FLAG_SHARE_WEIGHT_MEM` 语义）。

---

### 3.2 推理流程

#### `rknn_inputs_set`

```c
int rknn_inputs_set(rknn_context context, uint32_t n_inputs, rknn_input inputs[]);
```

**逆向还原**：

```
rknn_inputs_set()
├── 1. 遍历 inputs[]
│   ├── 如果 pass_through == TRUE → 直接拷贝到输入 DMA 内存
│   └── 如果 pass_through == FALSE →
│       ├── 格式转换（NHWC → NCHW 或 NC1HWC2）
│       ├── 类型转换（FP32 → INT8/FP16，应用 scale/zp）
│       └── 拷贝到输入 DMA 内存
└── 2. ioctl(MEM_SYNC, SYNC_TO_DEVICE) → flush cache
```

#### `rknn_run`

```c
int rknn_run(rknn_context context, rknn_run_extend* extend);
```

| 参数 | 说明 |
|:-----|:-----|
| `extend->frame_id` | [out] 当前帧 ID |
| `extend->non_block` | 0=阻塞，1=非阻塞 |
| `extend->timeout_ms` | 阻塞模式超时（毫秒） |
| `extend->fence_fd` | 外部 fence fd |

**逆向还原**（核心路径）：

```
rknn_run()
├── 1. 构造 rknpu_submit 结构体
│   ├── flags = RKNPU_JOB_PC | RKNPU_JOB_BLOCK（或 NONBLOCK）| RKNPU_JOB_PINGPONG
│   ├── task_obj_addr = tasks DMA 对象地址
│   ├── core_mask = 根据 rknn_set_core_mask() 设置
│   ├── subcore_task[] = 根据多核切分策略填充
│   └── timeout = extend->timeout_ms 或默认值
├── 2. ioctl(DRM_IOCTL_RKNPU_SUBMIT, &submit)
│   └── 内核驱动：
│       ├── 分配 job → 调度到核心
│       ├── 写 PC 寄存器（base_addr, amount, task_control, op_enable）
│       ├── 等待中断
│       └── 写回 task[].int_status
└── 3. 如果阻塞模式 → 等待 ioctl 返回
    如果非阻塞 → 立即返回，后续 rknn_wait() 或 rknn_outputs_get() 等待
```

#### `rknn_wait`

```c
int rknn_wait(rknn_context context, rknn_run_extend* extend);
```

等待非阻塞推理完成。内部轮询或等待 fence 信号。

#### `rknn_outputs_get`

```c
int rknn_outputs_get(rknn_context context, uint32_t n_outputs, rknn_output outputs[], rknn_output_extend* extend);
```

**逆向还原**：

```
rknn_outputs_get()
├── 1. 如果异步模式 → 返回上一帧结果（不等待当前帧）
├── 2. ioctl(MEM_SYNC, SYNC_FROM_DEVICE) → invalidate output cache
├── 3. 遍历 outputs[]
│   ├── 如果 want_float == TRUE →
│   │   ├── 反量化（INT8 → FP32，应用 scale/zp）
│   │   └── 格式转换（NC1HWC2 → NCHW/NHWC）
│   ├── 如果 is_prealloc == TRUE → 拷贝到用户提供的 buf
│   └── 如果 is_prealloc == FALSE → 分配 buf 并拷贝
└── 4. extend->frame_id = 当前帧 ID
```

#### `rknn_outputs_release`

```c
int rknn_outputs_release(rknn_context context, uint32_t n_ouputs, rknn_output outputs[]);
```

释放 `rknn_outputs_get` 中 `is_prealloc == FALSE` 时分配的 buf。

---

### 3.3 查询接口

#### `rknn_query`

```c
int rknn_query(rknn_context context, rknn_query_cmd cmd, void* info, uint32_t size);
```

通用查询接口。`info` 指向对应结构体，`size` 为结构体大小。

**关键查询结构体**：

```c
typedef struct _rknn_input_output_num {
    uint32_t n_input;
    uint32_t n_output;
} rknn_input_output_num;

typedef struct _rknn_perf_detail {
    char*    perf_data;      // 性能数据字符串（闭源库内部分配）
    uint64_t data_len;
} rknn_perf_detail;

typedef struct _rknn_perf_run {
    int64_t run_duration;    // 推理耗时（微秒）
} rknn_perf_run;

typedef struct _rknn_sdk_version {
    char api_version[256];
    char drv_version[256];
} rknn_sdk_version;

typedef struct _rknn_mem_size {
    uint32_t total_weight_size;
    uint32_t total_internal_size;
    uint64_t total_dma_allocated_size;
    uint32_t total_sram_size;
    uint32_t free_sram_size;
    uint32_t reserved[10];
} rknn_mem_size;
```

---

### 3.4 多核控制

#### `rknn_set_core_mask`

```c
int rknn_set_core_mask(rknn_context context, rknn_core_mask core_mask);
```

**逆向推断**：设置后续 `rknn_run` 提交时 `rknpu_submit.core_mask` 的值。对于联合核心模式（`CORE_0_1`、`CORE_0_1_2`），闭源库内部会将 Task 数组切分到 `subcore_task[]` 中。

#### `rknn_set_batch_core_num`

```c
int rknn_set_batch_core_num(rknn_context context, int core_num);
```

设置批量推理时使用的核心数。

---

### 3.5 动态 Shape

#### `rknn_set_input_shapes`

```c
int rknn_set_input_shapes(rknn_context ctx, uint32_t n_inputs, rknn_tensor_attr attr[]);
```

设置所有输入张量的 shape。仅对动态 shape 模型有效。调用后闭源库内部会重新编译命令流。

#### `rknn_set_input_shape`（已废弃）

```c
int rknn_set_input_shape(rknn_context ctx, rknn_tensor_attr* attr);
```

---

### 3.6 扩展结构体

#### `rknn_init_extend`

```c
typedef struct _rknn_init_extend {
    rknn_context ctx;
    int32_t      real_model_offset;    // 模型文件内偏移（零拷贝模式）
    uint32_t     real_model_size;      // 模型实际大小
    int32_t      model_buffer_fd;      // 模型缓冲 fd
    uint32_t     model_buffer_flags;   // 模型缓冲标志
    uint8_t      reserved[112];
} rknn_init_extend;
```

#### `rknn_run_extend`

```c
typedef struct _rknn_run_extend {
    uint64_t frame_id;       // [out] 帧 ID
    int32_t  non_block;      // 0=阻塞，1=非阻塞
    int32_t  timeout_ms;     // 超时（毫秒）
    int32_t  fence_fd;       // 外部 fence fd
} rknn_run_extend;
```

#### `rknn_output_extend`

```c
typedef struct _rknn_output_extend {
    uint64_t frame_id;       // [out] 输出对应的帧 ID
} rknn_output_extend;
```
