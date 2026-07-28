# RK3588 NPU 驱动开发路线图

## 进度总览
> **当前状态**：电源域管理 (Power Domain)
> **目标**：将添加了npu驱动的starry编译拷贝到开发板验证探测

## 实际计划

## 第一阶段：环境验证与基础初始化

### 1. 理论与环境准备
- [x] **阅读手册与 Demo**
    - [x] 研读 RK3588 技术参考手册中 NPU 章节。
    - [x] 分析 RKNN Toolkit2 的官方 Demo，理解从模型加载到推理的完整数据流
- [x] **实体机环境验证** (基于 StarryOS 比赛版)
    - [x] 编译并烧录比赛版本的 StarryOS 代码到 RK3588 实体板。
    - [x] 成功运行一个简单的文本生成模型，确保硬件和工具链正常。
- [x] **整理 StarryOS 对 RK3588 NPU 驱动的逆向成果**
    - [x] **寄存器地图**
        - [x] 按模块梳理：PC / CORE / CNA / DPU / PPU / SDMA / DDMA / GLOBAL
        - [x] 对每个寄存器记录：offset、字段含义、读写属性、默认值/复位值、关联流程
        - [x] 标注来源：TRM / Linux rknpu 驱动
    - [x] **提交协议与数据结构**
        - [x] 整理 `DRM_IOCTL_RKNPU_*` ioctl 列表与语义（Action / MemCreate / MemMap / MemDestroy / MemSync / Submit）
        - [x] 对齐结构体布局：`rknpu_mem_create` / `rknpu_mem_map` / `rknpu_task` / `rknpu_submit`（字段意义与对齐）
        - [x] 梳理 `mmap(offset)` 规则（handle 与 offset 的编码/解码约定）
    - [x] **任务提交流程（时序 + 状态机）**
        - [x] 从“用户态提交”到“硬件执行完成”的完整时序
        - [x] 失败路径：超时、异常中断状态、非法参数

## 第二阶段：异步推理支持

**目标**：将当前同步阻塞轮询的任务提交改为中断驱动的异步模式，提交任务后 CPU 不再空转等待，NPU 完成后通过中断通知。

### 实现情况

- [x] **中断处理** — 实现 IRQ 驱动模式，CPU 通过 WFI 指令进入低功耗等待，NPU 完成后触发中断唤醒 CPU
- [x] **异步等待机制** — 实现 `wait_all_npucore` 函数，支持多核心并行等待，每个核心独立触发中断
- [x] **并发安全** — 每个核心维护独立的 `irq_status` 原子变量，避免多核并发访问冲突
- [x] **超时与错误恢复** — 实现中断状态检查和错误传播机制，异常时正确上报错误码

---

## 第三阶段：NPU多任务并发运行

**目标**：支持多个NPU计算任务同时在NPU多核上进行

### 实现情况

- [x] **硬件资源隔离** — 验证 3 个 NPU 核心的 PC/CORE/CNA 等寄存器 per-core 独立，GLOBAL 寄存器通过 `core_idx` 参数隔离
- [x] **GEM 内存隔离** — 实现 per-core 的 regcmd 缓冲区分配，每个核心使用独立的 DMA 地址空间
- [x] **核心分配策略** — 实现 `subcore_task` 机制，支持灵活的核心分配，QKV 三核并行、Wo 单核等混合调度