# 内存管理与零拷贝

闭源库的内存管理策略，包括 DMA 内存分配、cache 同步、零拷贝机制。

> 逆向来源：`rknn_api.h`、`rknpu_ioctl.h`、`npu_interface.c`、`llama0.c`。

---

## 一、内存分配路径

### 1.1 闭源库内部分配

闭源库通过 DRM ioctl 分配 DMA 内存：

```
rknn_create_mem(ctx, size)
    ↓
ioctl(DRM_IOCTL_RKNPU_MEM_CREATE, {
    .size = size,
    .flags = flags
})
    ↓ 返回
{ handle, obj_addr, dma_addr }
    ↓
ioctl(DRM_IOCTL_RKNPU_MEM_MAP, { handle })
    ↓ 返回
{ offset }
    ↓
mmap(fd, size, offset)
    ↓ 返回
virt_addr
    ↓
填充 rknn_tensor_mem {
    .virt_addr = virt_addr,
    .phys_addr = dma_addr,    // 实际是 IOVA
    .fd = drm_fd,
    .size = size,
    .priv_data = 内部状态指针
}
```

### 1.2 内存类型标志

```c
enum e_rknpu_mem_type {
    RKNPU_MEM_CONTIGUOUS      = 0 << 0,  // 物理连续（默认）
    RKNPU_MEM_NON_CONTIGUOUS  = 1 << 0,  // 物理不连续
    RKNPU_MEM_NON_CACHEABLE   = 0 << 1,  // 不可缓存（默认）
    RKNPU_MEM_CACHEABLE       = 1 << 1,  // 可缓存
    RKNPU_MEM_WRITE_COMBINE   = 1 << 2,  // 写合并
    RKNPU_MEM_KERNEL_MAPPING  = 1 << 3,  // 内核态映射
    RKNPU_MEM_IOMMU           = 1 << 4,  // IOMMU 映射
    RKNPU_MEM_ZEROING         = 1 << 5,  // 零初始化
    RKNPU_MEM_SECURE          = 1 << 6,  // 安全缓冲
    RKNPU_MEM_NON_DMA32       = 1 << 7,  // 非 DMA32 区域
    RKNPU_MEM_TRY_ALLOC_SRAM  = 1 << 8,  // 尝试 SRAM 分配
};
```

### 1.3 闭源库的内存分配策略

| 用途 | flags | 原因 |
|:-----|:------|:-----|
| Task 数组 | `KERNEL_MAPPING` | 内核驱动需要直接读取 task 字段 |
| 命令流（regcmd） | 0 | 硬件通过 DMA 读取，不需要内核映射 |
| 权重 | `KERNEL_MAPPING` 或 0 | 取决于是否需要内核态访问 |
| 输入/输出 | 0 或 `CACHEABLE` | 用户频繁读写时用 CACHEABLE |
| 内部中间缓冲 | 0 | 仅硬件读写 |

**逆向证据**：`bench_mark.c` 中 tasks 用 `RKNPU_MEM_KERNEL_MAPPING`，其余用 0。

---

## 二、Cache 同步

### 2.1 同步模式

```c
enum e_rknpu_mem_sync_mode {
    RKNPU_MEM_SYNC_TO_DEVICE   = 1 << 0,  // CPU → 设备（flush）
    RKNPU_MEM_SYNC_FROM_DEVICE = 1 << 1,  // 设备 → CPU（invalidate）
};
```

### 2.2 同步时机

```
CPU 写入输入数据
    ↓
ioctl(MEM_SYNC, { flags=SYNC_TO_DEVICE, obj_addr, offset, size })
    ↓ flush CPU cache
NPU 可以安全读取
    ↓
NPU 执行完毕，写入输出
    ↓
ioctl(MEM_SYNC, { flags=SYNC_FROM_DEVICE, obj_addr, offset, size })
    ↓ invalidate CPU cache
CPU 可以安全读取输出
```

### 2.3 闭源库的 cache 优化标志

| 标志 | 效果 |
|:-----|:-----|
| `RKNN_FLAG_DISABLE_FLUSH_INPUT_MEM_CACHE` | 跳过输入 flush（用户自行保证） |
| `RKNN_FLAG_DISABLE_FLUSH_OUTPUT_MEM_CACHE` | 跳过输出 invalidate（输出由 GPU/RGA 消费） |

---

## 三、零拷贝机制

### 3.1 标准路径 vs 零拷贝路径

**标准路径**（`rknn_inputs_set` + `rknn_outputs_get`）：

```
用户缓冲 → [格式转换] → [类型转换] → memcpy → DMA 缓冲 → NPU
NPU → DMA 缓冲 → memcpy → [反量化] → [格式转换] → 用户缓冲
```

至少 2 次 memcpy + 可能的格式/类型转换。

**零拷贝路径**（`rknn_set_io_mem`）：

```
用户直接写入 DMA 缓冲（native layout）→ NPU
NPU → DMA 缓冲 → 用户直接读取
```

0 次 memcpy，但用户需要自行处理 native layout。

### 3.2 零拷贝 API

#### 内部分配

```c
rknn_tensor_mem* rknn_create_mem(rknn_context ctx, uint32_t size);
```

闭源库内部调用 `ioctl(MEM_CREATE)` + `ioctl(MEM_MAP)` + `mmap()`。

#### 带标志分配

```c
rknn_tensor_mem* rknn_create_mem2(rknn_context ctx, uint64_t size, uint64_t alloc_flags);
```

| alloc_flags | 说明 |
|:------------|:-----|
| `RKNN_FLAG_MEMORY_CACHEABLE` | 可缓存内存 |
| `RKNN_FLAG_MEMORY_NON_CACHEABLE` | 不可缓存内存 |
| `RKNN_FLAG_MEMORY_TRY_ALLOC_SRAM` | 尝试 SRAM |

#### 从外部 fd 创建

```c
rknn_tensor_mem* rknn_create_mem_from_fd(rknn_context ctx,
    int32_t fd, void* virt_addr, uint32_t size, int32_t offset);
```

用于导入其他设备（如 camera、GPU）分配的 DMA buffer。

#### 从物理地址创建

```c
rknn_tensor_mem* rknn_create_mem_from_phys(rknn_context ctx,
    uint64_t phys_addr, void* virt_addr, uint32_t size);
```

#### 从 mb_blk 创建

```c
rknn_tensor_mem* rknn_create_mem_from_mb_blk(rknn_context ctx,
    void* mb_blk, int32_t offset);
```

用于 Rockchip 多媒体框架的内存块。

#### 销毁

```c
int rknn_destroy_mem(rknn_context ctx, rknn_tensor_mem* mem);
```

**逆向推断**：内部根据 `mem->flags` 判断是否需要 `munmap()` 和 `ioctl(MEM_DESTROY)`：
- `RKNN_TENSOR_MEMORY_FLAGS_ALLOC_INSIDE` → 完整释放
- `RKNN_TENSOR_MEMORY_FLAGS_FROM_FD` → 仅释放包装结构
- `RKNN_TENSOR_MEMORY_FLAGS_FROM_PHYS` → 仅释放包装结构

#### Cache 同步

```c
int rknn_mem_sync(rknn_context context, rknn_tensor_mem* mem, rknn_mem_sync_mode mode);
```

| mode | 说明 |
|:-----|:-----|
| `RKNN_MEMORY_SYNC_TO_DEVICE` | CPU 写完后调用 |
| `RKNN_MEMORY_SYNC_FROM_DEVICE` | 读取设备输出前调用 |
| `RKNN_MEMORY_SYNC_BIDIRECTIONAL` | 双向同步 |

**逆向推断**：内部调用 `ioctl(DRM_IOCTL_RKNPU_MEM_SYNC, { flags, obj_addr, offset, size })`。

### 3.3 IO 内存绑定

```c
int rknn_set_io_mem(rknn_context ctx, rknn_tensor_mem* mem, rknn_tensor_attr* attr);
```

将用户分配的内存绑定为模型的输入或输出。`attr->index` 指定输入/输出索引。

```c
int rknn_set_weight_mem(rknn_context ctx, rknn_tensor_mem* mem);
int rknn_set_internal_mem(rknn_context ctx, rknn_tensor_mem* mem);
```

绑定权重和内部内存。需配合 `RKNN_FLAG_MEM_ALLOC_OUTSIDE` 使用。

---

## 四、SRAM 管理

RK3588 NPU 有片上 SRAM，可用于减少 DDR 带宽：

```c
// 查询 SRAM 大小
rknn_mem_size mem_size;
rknn_query(ctx, RKNN_QUERY_MEM_SIZE, &mem_size, sizeof(mem_size));
// mem_size.total_sram_size — 总 SRAM 大小
// mem_size.free_sram_size  — 空闲 SRAM 大小

// 也可通过 ioctl 查询
struct rknpu_action action = { .flags = RKNPU_GET_TOTAL_SRAM_SIZE };
ioctl(fd, DRM_IOCTL_RKNPU_ACTION, &action);
```

启用 SRAM 分配：
- 初始化时设置 `RKNN_FLAG_ENABLE_SRAM`
- 内存分配时使用 `RKNPU_MEM_TRY_ALLOC_SRAM` 或 `RKNN_FLAG_MEMORY_TRY_ALLOC_SRAM`

---

## 五、npu_llama 的 Buffer 池化策略

`llama0.c` 展示了一种用户态 buffer 池化方案，避免频繁的 DMA 内存分配/释放：

```c
#define MAX_BUFFER_POOL_SIZE 8

typedef struct {
    void*    data;
    uint64_t dma;
    uint64_t obj;
    uint32_t handle;
    size_t   size;
    int      in_use;
} NPUBuffer;

// 预分配不同大小的 buffer
size_t sizes[] = { 512KB, 1MB, 2MB, 512KB, 256KB, 256KB, 128KB, 128KB };
for (int i = 0; i < 8; i++) {
    pool[i].data = mem_allocate(fd, sizes[i], &pool[i].dma, ...);
}

// 运行时从池中取用
NPUBuffer* buf = get_buffer_from_pool(t, required_size);
// ... 使用 buf->data / buf->dma ...
release_buffer_to_pool(buf);
```

闭源库内部很可能也使用类似的池化策略来管理内部中间缓冲。
