# RK3588 RKNPU 开发文档

本文档基于 RockChip官方开源仓库和 StarryOS 仓库中对 RK3588 NPU 运行时库（闭源 `librknnrt.so`）及内核侧 RKNPU DRM 驱动的逆向与复现成果，整理为三个核心章节：

## 文档结构

<table>
<tr>
<th>章节</th><th>内容</th><th>适用场景</th>
</tr>
<tr>
<td><a href="./rknn-feature.html"><strong>RKNN 硬件特性</strong></a></td>
<td>三核架构、数据精度与算力、支持的推理框架</td>
<td>了解硬件能力边界</td>
</tr>
<tr>
<td><a href="./register-map.html"><strong>寄存器地图</strong></a></td>
<td>按模块梳理rknpu内部各个寄存器的位域、读写属性</td>
<td>写驱动、调试硬件交互</td>
</tr>
<tr>
<td><a href="./ioctl-protocol.html"><strong>IOCTL协议与数据结构</strong></a></td>
<td>DRM_IOCTL_RKNPU_* 命令表，结构体布局，flags枚举，mmap 规则</td>
<td>实现ioctl分发，对齐用户态ABI</td>
</tr>
<tr>
<td><a href="./submit-flow.html"><strong>任务提交流程</strong></a></td>
<td>从用户态提交到硬件执行完成的完整时序逻辑，包含状态机和各种失败路径</td>
<td>查询job生命周期</td>
</tr>
<tr>
<td><a href="./submit-irq-snapshot-restore.html"><strong>当前提交-IRQ 边界快照恢复系统</strong></a></td>
<td>总结 step-submit、owner 切换、IRQ 边界快照/写坏/恢复/校验，以及相关 struct 的职责</td>
<td>整理当前实验实现、查数据结构分工、回顾抢占边界语义</td>
</tr>
</table>

## 来源标注约定

文档中对每条信息标注来源，使用以下标记：

- <span style="background:#e8f5e9;padding:2px 6px;border-radius:3px;font-size:0.85em">Linux rknpu 驱动</span> — 来自Rockchip官方仓库中rk3588-npu内核驱动代码
- <span style="background:#e3f2fd;padding:2px 6px;border-radius:3px;font-size:0.85em">rknpu-ioctl.h</span> — 来自 Linux rknpu 驱动include目录的 ioctl 头文件
- <span style="background:#fff3e0;padding:2px 6px;border-radius:3px;font-size:0.85em">StarryOS Rust 驱动</span> — 来自 `drivers/rknpu/src/` 的 Rust 复刻实现
- <span style="background:#fce4ec;padding:2px 6px;border-radius:3px;font-size:0.85em">逆向推断</span> — 基于代码行为推断，无官方文档确认

## 术语速查

| 术语 | 含义 |
|------|------|
| **GEM** | Graphics Execution Manager，DRM 子系统的内存对象管理框架 |
| **PC** | Program Counter / 任务控制器，NPU 的命令流执行引擎 |
| **CNA** | Convolution Neural-network Accelerator，卷积加速单元 |
| **DPU** | Data Processing Unit，数据后处理单元 |
| **PPU** | Pooling Processing Unit，池化处理单元 |
| **DDMA/SDMA** | Data DMA / System DMA，数据搬运引擎 |
| **IOVA** | I/O Virtual Address，IOMMU 映射后的设备侧虚拟地址 |
| **fence** | DMA fence，用于 job 完成通知与跨设备同步的内核原语 |
