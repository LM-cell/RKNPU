# 自定义算子 API（rknn_custom_op.h）

> 来源：`test/starrynpu/demo/yolov8/3rdparty/rknpu2/include/rknn_custom_op.h`（145 行）

当模型中包含 NPU 不支持的算子时，闭源库提供自定义算子机制，允许用户注册 CPU 或 GPU（OpenCL）回调函数来实现这些算子。

---

## 一、核心概念

```
模型推理流程中遇到不支持的算子：
    NPU 执行 layer 0~5
    → 自定义算子回调执行 layer 6（CPU/GPU）
    → NPU 继续执行 layer 7~N
```

闭源库在推理过程中自动调度：NPU 可执行的层在 NPU 上运行，不支持的层调用用户注册的回调。

---

## 二、类型定义

### 2.1 执行后端

```c
typedef enum _rknn_target_type {
    RKNN_TARGET_TYPE_CPU = 1,    // CPU 后端
    RKNN_TARGET_TYPE_GPU = 2,    // GPU 后端（OpenCL）
} rknn_target_type;
```

### 2.2 GPU 上下文

```c
typedef struct _rknn_gpu_op_context {
    void* cl_context;          // OpenCL context
    void* cl_command_queue;    // OpenCL command queue
    void* cl_kernel;           // OpenCL kernel
} rknn_gpu_op_context;
```

### 2.3 算子上下文

```c
typedef struct _rknn_custom_op_context {
    rknn_target_type               target;        // 后端类型
    rknn_custom_op_interal_context internal_ctx;   // 闭源库内部上下文
    rknn_gpu_op_context            gpu_ctx;        // GPU 上下文
    void*                          priv_data;      // 用户私有数据
} rknn_custom_op_context;
```

### 2.4 算子张量

```c
typedef struct _rknn_custom_op_tensor {
    rknn_tensor_attr attr;    // 张量属性（维度、类型、量化参数）
    rknn_tensor_mem  mem;     // 张量内存（虚拟地址、物理地址、fd）
} rknn_custom_op_tensor;
```

### 2.5 算子属性

```c
typedef struct _rknn_custom_op_attr {
    char             name[RKNN_MAX_NAME_LEN];   // 属性名
    rknn_tensor_type dtype;                      // 数据类型
    uint32_t         n_elems;                    // 元素数量
    void*            data;                       // 属性数据指针
} rknn_custom_op_attr;
```

---

## 三、算子注册结构体

```c
typedef struct _rknn_custom_op {
    uint32_t         version;                         // 版本号
    rknn_target_type target;                          // CPU 或 GPU
    char             op_type[RKNN_MAX_NAME_LEN];      // 算子类型名

    // GPU（OpenCL）专用字段
    char     cl_kernel_name[RKNN_MAX_NAME_LEN];       // OpenCL kernel 名
    char*    cl_kernel_source;                         // kernel 源码或文件路径
    uint64_t cl_source_size;                           // 源码大小（0=文件路径）
    char     cl_build_options[RKNN_MAX_NAME_LEN];     // 编译选项

    // 回调函数
    int (*init)(rknn_custom_op_context* op_ctx,
                rknn_custom_op_tensor* inputs, uint32_t n_inputs,
                rknn_custom_op_tensor* outputs, uint32_t n_outputs);
                // [可选] 初始化回调

    int (*prepare)(rknn_custom_op_context* op_ctx,
                   rknn_custom_op_tensor* inputs, uint32_t n_inputs,
                   rknn_custom_op_tensor* outputs, uint32_t n_outputs);
                   // [可选] 准备回调

    int (*compute)(rknn_custom_op_context* op_ctx,
                   rknn_custom_op_tensor* inputs, uint32_t n_inputs,
                   rknn_custom_op_tensor* outputs, uint32_t n_outputs);
                   // [必须] 计算回调

    int (*compute_native)(rknn_custom_op_context* op_ctx,
                          rknn_custom_op_tensor* inputs, uint32_t n_inputs,
                          rknn_custom_op_tensor* outputs, uint32_t n_outputs);
                          // [可选] 原生属性计算回调（当前不支持）

    int (*destroy)(rknn_custom_op_context* op_ctx);
                   // [可选] 销毁回调
} rknn_custom_op;
```

**特殊返回值**：`init` 回调返回 `RKNN_WARNING_SKIP_CUSTOM_OP_COMPUTE`（-14）时，如果该算子类型被 RKNN 内部支持，则使用内部实现而非自定义回调。

---

## 四、函数签名

### `rknn_register_custom_ops`

```c
int rknn_register_custom_ops(rknn_context ctx, rknn_custom_op* op, uint32_t custom_op_num);
```

| 参数 | 说明 |
|:-----|:-----|
| `ctx` | rknn 上下文（必须在 `rknn_init` 之后调用） |
| `op` | 自定义算子数组 |
| `custom_op_num` | 数组长度 |

**使用步骤**：
1. 创建 `rknn_custom_op` 结构体数组
2. 填写 `op_type`、`target`、回调函数
3. 在 `rknn_init()` 之后调用 `rknn_register_custom_ops()`

### `rknn_custom_op_get_op_attr`

```c
void rknn_custom_op_get_op_attr(rknn_custom_op_context* op_ctx,
                                 const char* attr_name,
                                 rknn_custom_op_attr* op_attr);
```

在回调函数内部调用，获取模型中定义的算子属性（如 kernel_size、stride 等）。

---

## 五、动态加载机制

闭源库支持通过 `dlopen` 加载自定义算子 `.so`：

```c
typedef rknn_custom_op* (*get_custom_op_func)();
```

自定义算子 `.so` 需导出 `get_custom_op_func` 类型的函数，返回 `rknn_custom_op` 指针。需使用 `RKNN_CUSTOM_OP_EXPORT`（`__attribute__((visibility("default")))`）标记导出函数。
