# 第八周开发日志（3.15-3.21）

## 工作总结

- **阶段性代码提交** — 本周完成一次阶段性代码提交 `1d4e8fe`，核心是把 `rknpu` 提交路径收敛为“单 task 边界步进 + 外层调度器轮转推进”，并补齐最小 DMA syscall 与用户态试验链路。提交链接：[Commit](https://github.com/qc-gpu-driver/starryos-pulsar/commit/1d4e8fea8a5459bef7cff3d9eb0ec2a7dadfa472)
- **NPU 提交路径支持单 task 边界切换** — 驱动只保留 `submit_ioctrl_step_with_owner()` 这条步进入口，在每个 task 完成 IRQ 边界返回，让多个进程可以轮流推进各自未完成的 submit
- **DMA syscall 与用户态辅助库打通** — 保留 `sys_dma_malloc`、`sys_dma_free` 和对应轻量用户态封装，用于板端分配与释放 DMA 缓冲区
- **新增多进程/混合工作负载验证** — 新增 `matmul_multi_process` 与 `matmul_llama_concurrent` 两个测试程序，板端验证多进程交错提交和 llama 推理并发场景
- **寄存器级保存/恢复路径彻底下线** — 当前主线不再保留完整寄存器快照、恢复镜像、毒值写坏和读回校验逻辑，IRQ 路径只更新最小 owner/task 状态

---

## 单 task 边界调度改造

本周核心工作是把原来“一次 ioctl 一口气跑完整批任务”的同步提交逻辑，改造成“每次只推进一个当前 task-batch，完成后返回调度器”的步进式路径。

当前驱动只保留两类最小状态：

- **`NpuOwnerIds`** — 由外层调度器提供的 owner 标识，使用“线程 ID + 进程 ID + 地址空间指针”唯一标定一次 submit 的归属
- **每核 owner 槽位** — 每个硬件 core 只保留一个 `NpuOwnerState`，记录该 core 当前绑定的 owner、当前 task、task 指针、task 索引，以及最近一次 IRQ 的观测结果

不再保留：

- resident owner
- owner 级 submit/context 快照
- `(owner, core)` 共享状态表
- 完整寄存器保存/恢复镜像

新的路径大致如下：

1. `card1` 在进入 `RKNPU_SUBMIT` 时，用“当前线程 ID + 当前进程 ID + 当前地址空间指针”组装 owner
2. 驱动执行 `submit_ioctrl_step_with_owner()`，本次最多只推进一个 task-batch
3. 每个参与的 core 在下发前先绑定一个最小 `NpuOwnerState`
4. 每个核心收到预期完成中断后，驱动更新对应 task 的 `int_status`，同步回写 owner 槽位里的 IRQ 字段，然后立即清空该 core 槽位
5. 如果整次 submit 还没结束，就 `yield_now()` 主动让出，允许别的 owner 进来推进它自己的 submit
6. 下次同一个 owner 再进来时，只根据 `task_counter` 从上次稳定完成的位置继续

这样之后，进程级切换点就固定在了“单个 task 完成 IRQ 边界”上，而不是“整个 submit 全部完成”之后。

板端验证已经可以观察到这种交错推进：

```text
[NPU]   batch done: int_status=0x300
...
matmul_multi_process: pass
```

这说明当前已经不是“一个进程一次性独占跑完整次 submit”，而是由外层 loop 在 task 边界轮流推进多个 owner。

---


## 多进程与混合负载测试

为了验证当前这套“单 task 边界切换”不是只在单个 demo 里自娱自乐，本周新增了两个更贴近真实使用方式的测试程序。

### 1. `matmul_multi_process`

这个测试会同时启动 4 个子进程。每个子进程只发起 **1 次 submit**，但这次 submit 里串了 20 个相关联的 matmul task：

- task 0 的输出作为 task 1 的输入
- task 1 的输出再作为 task 2 的输入
- …
- 最终做逐 task、逐元素 CPU 参考校验

它验证的不是“多个短任务抢一个锁”，而是：

- 单个 submit 内还有剩余 task 时，驱动能否在 IRQ 边界切出去
- 之后切回来时，能否从正确的 `task_counter` 继续推进
- 多个进程交错执行后，中间状态和最终输出是否仍然正确

最终板端结果为：

```text
matmul_multi_process: pass
```

---

## 中断边界实验的收敛

本周原本也同步尝试了更激进的路线：在 IRQ 完成边界读取 live 寄存器快照，把准备恢复的寄存器先写入固定毒值，再按镜像恢复，最后读回校验。

但在板端验证时发现，至少有一部分 **task-window 配置寄存器** 在“任务刚完成、IRQ 已到达”的这个边界之后，并不保证还能按提交时的原值稳定读回。例如曾经观察到：

```text
first_task_shadow_mismatch={ offset=0x100c, expected=0x120, got=0x0 }
first_task_shadow_mismatch={ offset=0x4058, expected=0xf, got=0x0 }
```

这说明两个事实：

- 当前边界对“task 已完成”是稳定的
- 但它不一定对“所有任务窗口寄存器还能按原值读回”稳定

因此当前版本做了一个更明确的收敛：

- 不再尝试保存/恢复整套寄存器镜像
- 不再保留毒值验证、恢复镜像、读回校验的主线逻辑
- IRQ 路径只保留“读取 live IRQ、清中断、更新最小 owner/task 槽位状态”

也就是说，当前主线解决的是“可运行、可验证、可继续扩展”的 task 粒度协作式调度基础设施；**任意寄存器级/算子级抢占** 不在本次提交范围内。

---

## 本周结论

本周最重要的进展，是把 **单任务级边界（核 IRQ 中断边界）切换 + 多进程交错推进 + 结果正确** 这一层跑通，并进一步把驱动内部状态压缩到了最小模型。

当前已经可以确认：

- 抢占/切换粒度已经从“整次 submit”下沉到了“单个 task 完成 IRQ 边界”
- 多个进程提交各自含有多 task 的 submit 时，驱动可以轮流推进
- 驱动内部不再承担 owner 上下文保存/恢复，也不再提供完整 NPU 状态导出接口

下一步的重点，是继续在当前 task 粒度调度稳定的基础上，围绕更细的调度与恢复能力评估真实可行的硬件边界，而不是继续维护一套过重的软件镜像模型。
