# 当前 NPU 队列调度器配合说明

这份文档讲的是当前这版 NPU 调度链路到底怎么配合。重点不是复盘历史方案，而是把已经落到代码里的职责边界讲清楚：`card1` 做什么，StarryOS 里的 scheduler 做什么，驱动和 IRQ 路径现在只剩下什么责任，以及一次 `Submit` 是怎么从入队一路走到返回用户态的。

这版实现保留了一个很明确的核心模型：

- 对外还是阻塞 `Submit` ioctl
- 对内是一整次 submit 入队
- 真正下发给硬件时是 per-core、per-task streaming dispatch
- 每个 submit 自己有 waiter
- 全局只有一个 kick，用来唤醒 worker
- in-flight dispatch 的 owner 只保留在 scheduler 一处

所以从用户态看，语义没变；从内核实现看，控制流已经不是“调用线程自己 loop 推完整次 submit”了，而是“调用线程阻塞，后台 worker 负责推进和补发”。

## 1. 先说结论

当前链路可以概括成四句话：

1. 用户线程发起 `Submit`，`card1` 先把 `RknpuSubmit` 和 `RknpuTask[]` 拷到内核 shadow。
2. 整次 submit 作为一个队列项入队，但真正发给硬件时是按 core、按 task 一次一次地下发。
3. 哪个 core 先完成，就先 harvest 哪个 core，再尽快给它补下一个可运行 task。
4. 原始提交线程睡在自己的 waiter 上，等 submit 进入 terminal 状态后被唤醒，再把最终结果拷回用户态。

这就是“阻塞 submit + 内部异步调度”的准确含义。

## 2. 为什么要收成现在这个形状

之前的问题主要有两个。

第一个问题是职责缠在一起。`card1` 既处理 ioctl，又自己循环推进 task，还要主动 `yield_now()` 给别的执行流机会。路径长，问题一多就很难沿着职责边界查。

第二个问题是内部状态太散。一份 submit 的信息会在 `card1 / scheduler / queue / driver` 之间换着名字存几次；in-flight dispatch 也会在 queue 和 driver 两边都留一份反向绑定，最后软件模型比真正需要的边界还重。

所以现在的收口原则很直接：

- `RknpuSubmit` 只留在 ioctl 边界和最终 copy-back 边界
- queue 只维护“这个 submit 还剩什么能调度”
- scheduler 独占 in-flight dispatch ownership
- driver / IRQ 只发布 per-core 原始完成状态

## 3. 各层现在分别负责什么

### 3.1 `card1`：阻塞 ioctl 入口

`card1` 现在只做三件事：

1. 从用户态拷贝 `RknpuSubmit` 和 `RknpuTask[]`
2. 构造 `RknpuQueuedSubmit` 并调用 `enqueue_submit(...)`
3. 等待完成，取回 `CompletedSubmit`，再统一拷回用户态

它不再自己步进调度，也不再负责保存任何调度期内部状态。它只保留阻塞设备调用入口该有的职责。

### 3.2 `NpuScheduler`：真正的调度器

StarryOS 里的 `NpuScheduler` 是当前链路的控制中心。它手里有四类核心状态：

- `queue: RknpuTaskQueue`
- `waiters: BTreeMap<RknpuQueueTaskId, Arc<NpuSubmitWaiter>>`
- `inflight: [Option<InflightDispatch>; NPU_MAX_CORES]`
- `kick: Event`

分工很明确：

- `queue` 管 submit 级进度
- `waiters` 管阻塞线程的睡眠和唤醒
- `inflight` 管当前每个硬件 core 在跑哪一个 dispatch
- `kick` 只负责把 worker 从睡眠态唤醒

也就是说，waiter 是 per-submit 的，kick 是全局的；二者分别解决“谁该睡”和“worker 什么时候该起来干活”两个完全不同的问题。

### 3.3 `drivers/rknpu`：单次硬件下发原语

驱动层现在退回到最小硬件编程层。它保留的核心原语就是：

- `submit_ioctrl_step(...)`
- `harvest_completed_dispatches()`

其中：

- `submit_ioctrl_step(...)` 只负责把一个 task 发给一个 core
- `harvest_completed_dispatches()` 只负责把 IRQ top-half 已经发布好的 raw completion 收回来

驱动不再负责维护完整 submit 生命周期，也不再保存 queue 语义的绑定关系。

### 3.4 IRQ top-half：只发布 raw per-core completion

IRQ handler 现在只做最小动作：

- 读取硬件 IRQ 状态
- fuzz status
- 清硬件中断
- 把结果按 core 写入共享原子

它不直接继续调度，也不持有 queue 语义的数据结构。这样 top-half 足够短，行为也足够稳定。

## 4. 内部数据模型怎么收口

这版重构的核心不是“加了多少新类型”，而是把每个类型的边界收窄了。

### 4.1 `RknpuSubmit` 只留在边界

`RknpuSubmit` 现在只用于两处：

- ioctl 入口时承接用户态 header
- submit 终态时重新组装 copy-back 给用户态

调度期间不再把它当成一个全程可变的内部状态容器到处传、到处改。

### 4.2 `SubmitMeta` 是内部不可变输入视图

调度期真正使用的是 `SubmitMeta`。它只保留调度所需的固定字段：

- `flags`
- `priority`
- `core_mask`
- `task_dma_base`
- `lane_ranges`
- `task_total`

`lane_ranges` 在入队时就做一次归一化：

- 如果 `subcore_task[]` 全空，就默认变成 `slot0 = [0, task_total)`
- 否则只按已有非空 lane 解释

这样后面的 queue / scheduler 不需要再在运行期反复猜 lane layout。

### 4.3 `CompletedSubmit` 是 terminal 返回模型

调度器在 terminal 时返回的不是整个 queue entry，而是一个更明确的结果模型：

- `submit: RknpuSubmit`
- `tasks: Vec<RknpuTask>`
- `last_error: Option<RknpuError>`

这个边界很重要，因为 `card1` 真正需要的只有这三样，不需要知道 queue 内部状态机细节。

### 4.4 `InflightDispatch` 是唯一的 in-flight owner

每个正在硬件上飞的 dispatch 现在只在 scheduler 的 `inflight[core]` 里有一份记录，里面保存：

- `queue_task_id`
- `core_slot`
- `subcore_slot`
- `task_index`
- `task_ptr`
- `expected_irq_mask`

这份记录回答的就是一个问题：

“这个 core 当前跑的是哪个 submit 的哪个 task；如果 completion 回来，我该把结果记到哪里。”

queue 不再保存 per-core 反向绑定，driver 也不再保存 queue-facing 绑定。

### 4.5 `CoreCompletion` 是 driver 向 scheduler 提交的最小完成记录

driver harvest 之后只返回：

- `core_slot`
- `observed_irq_status`

scheduler 拿到它以后，再结合 `InflightDispatch.expected_irq_mask` 计算：

- `last_task_int_status`
- `task_error`

然后推进 queue 状态机。这一步把“硬件观测”与“队列语义”干净地分开了。

## 5. queue 是怎么维护游标和分发任务的

这部分是当前调度模型的核心。

### 5.1 queue 维护的是 submit 级进度，不是 per-core 绑定

`RknpuQueueTask` 里真正和调度推进相关的字段只有这些：

- `meta`
- `tasks`
- `subcore_cursors`
- `subcore_running_mask`
- `completed_task_count`
- `inflight_core_mask`
- `last_error`
- `ready_queued`

语义分别是：

- `subcore_cursors`：每个逻辑 lane 已经推进到该 lane 的第几个 task
- `subcore_running_mask`：某个 lane 当前是否已经有 task 在飞
- `completed_task_count`：这次 submit 已经完成了多少 task
- `inflight_core_mask`：当前有哪些物理 core 正在执行这个 submit
- `ready_queued`：这个 submit 当前是否已经挂在 ready 队列里，避免重复入队

### 5.2 `reserve_next_dispatch()` 只产出最小 reservation token

queue 不再给 scheduler 一整份“大计划”，而是一次只给一个最小 reservation：

- `queue_task_id`
- `subcore_slot`
- `task_index`

这意味着 queue 只负责说“下一个可以发的是谁”，不负责持有这个 dispatch 之后在 driver 侧的绑定信息。

### 5.3 游标推进规则

worker 调度时，queue 会按下面的规则挑任务：

1. 先按 priority 从 ready 队列里挑 submit
2. 再在这个 submit 内部按 `subcore_slot` 扫描可运行 lane
3. 跳过已经在飞的 lane
4. 用 `subcore_cursors[slot]` 算出当前 lane 的下一个 `task_index`
5. 成功 reservation 后，设置：
   - `subcore_running_mask`
   - `inflight_core_mask`

completion 回来以后，再清掉对应 bit，并把该 lane 的 cursor 加一。

### 5.4 为什么同一个 submit 能同时占多个 core

因为 `subcore_running_mask` 限制的是“同一个 lane 不能并发重复下发”，不是“同一个 submit 只能占一个 core”。

所以只要：

- 这个 submit 还有别的 lane 可跑
- 目标 core 在 `core_mask` 允许范围内
- 当前有空闲 core

那么同一个 submit 完全可以在一轮里占多个 core。

### 5.5 为什么 faulted submit 还要等 inflight core 排空

`Faulted` 不等于立即 terminal。

当前实现里，一个 submit 即便已经 faulted，只要 `inflight_core_mask != 0`，它还不能算 terminal。因为还有别的 core 上的 in-flight task 没收干净，waiter 这时不能提前醒。

所以 terminal 的判定是：

- `Completed`
- 或者 `Faulted && inflight_core_mask == 0`

## 6. 一次完整的 `Submit` 现在怎么走

### 第 1 步：用户态发起 ioctl

用户态传入 `RknpuSubmit` 和 `RknpuTask[]`。`card1` 把它们拷进内核 shadow。

### 第 2 步：构造 `RknpuQueuedSubmit`

`card1` 把原始 submit 拆成两部分：

- 边界专用的 `RknpuSubmit`
- 调度专用的 `SubmitMeta + Vec<RknpuTask>`

然后包装成 `RknpuQueuedSubmit` 入队。

### 第 3 步：提交线程阻塞

`enqueue_submit(...)` 为这个 submit 建立自己的 `NpuSubmitWaiter`。随后提交线程执行 `wait_for_submit(...)`，睡到 waiter 的 `WaitQueue` 上。

### 第 4 步：worker 被 kick 唤醒

新 submit 入队后，scheduler 调一次 `kick.notify_relaxed(1)`。worker 如果在睡眠，就会被唤醒开始工作。

这里的 `kick` 不是“某个 submit 的完成事件”，它只是一个全局“有活了，起来看队列”的唤醒信号。

### 第 5 步：worker 给空闲 core 分发任务

worker 会循环做两件事：

- `harvest_completed_cores()`
- `dispatch_idle_cores()`

在 dispatch 路径里，它会：

1. 找一个当前没有 inflight 的 core
2. 让 queue 给出一个 reservation token
3. 从 queue task 里取出对应 task，组装 `InflightDispatch`
4. 先写入 `state.inflight[core]`
5. 如果这是本轮第一次给这个 submit 发任务，就做一次 `confirm_write_all()`
6. 调用 driver 的 `submit_ioctrl_step(...)`

如果 driver dispatch 失败，就回滚 `state.inflight[core]` 和 queue reservation。

### 第 6 步：IRQ 到来，只发布原始完成状态

哪个 core 先完成，就哪个 core 的 top-half 先把 `observed_irq_status` 发布出来。此时不会直接继续调度。

### 第 7 步：worker harvest completion

worker 下次循环里会：

1. 从 driver 拿到 `CoreCompletion`
2. 通过 `state.inflight[core]` 找到对应 `InflightDispatch`
3. 计算：
   - `last_task_int_status = observed_irq_status & expected_irq_mask`
   - `task_error`
4. 回写 `task.int_status`
5. 调用 queue 的 `complete_dispatch(...)` 推进 submit 状态

如果 submit 已经 terminal，就把 task_id 记到 terminal 列表里。

### 第 8 步：terminal path 唤醒 waiter

terminal path 里，scheduler 会：

- 先做一次 `prepare_read_all()`
- 然后唤醒该 submit 对应的 waiter

提交线程被唤醒后，再从 scheduler 取回 `CompletedSubmit`，并把最终的 task 数组和 submit header 一起拷回用户态。

## 7. 现在比旧模型少了什么

少掉的其实就是这次刻意拿掉的那些冗余状态。

### 7.1 不再有 submit 信息的多份影子副本

现在不会再在内部反复保存这些重复字段：

- `task_dma_addr` 和 `submit.task_base_addr` 的双存
- `submit.task_obj_addr` 的内部影子
- `submit.task_counter` / `submit.hw_elapse_time` 的调度期原地写回版本
- queue 上额外存一份 `priority`

这些都被收回到更单一的边界里了。

### 7.2 不再有 queue 和 driver 的双重反向绑定

以前 queue 和 driver 都想回答“这个 core 当前到底在跑谁”。现在只有 scheduler 的 `InflightDispatch` 回答这个问题。

### 7.3 不再有只负责搬字段的 helper

像旧的 submit view 刷新、ready bit 翻转、active core 反查这类 helper，现在都不需要了。保留下来的 helper 都是状态机规则本身的一部分。

## 8. 当前实现里还需要注意的点

### 8.1 worker 还不是 IRQ 直接唤醒

当前 worker 在有 inflight core 时，还是通过轻量 `yield_now()` 轮询 harvest，而不是让 IRQ 直接把 worker 唤醒。

这不影响语义正确性，但如果以后继续优化内核态延迟，这里还有继续收紧的空间。

### 8.2 `confirm_write_all()` / `prepare_read_all()` 还是全局同步

现在还是按 GEM 池全局同步，不是按单 submit 精细同步。这版先保证路径和 ownership 干净，性能细化可以后面再做。

### 8.3 当前切换边界仍然是 task 完成后的 IRQ 边界

这版已经是多 submit、per-core streaming dispatch，但它仍然不是“任意时刻抢占正在执行的 NPU 指令流”。真正稳定的切换点还是 task 完成后的 IRQ 边界。

## 9. 队列相关结构体作用

这一节单独列一下和 queue / scheduler 直接相关的结构体，后面查代码时可以快速对照。

### 9.1 `SubmitMeta`

作用：submit 的内部不可变调度视图。

它只保留调度期真正要用的元数据：flag、priority、core_mask、DMA base、lane_ranges、task_total。它的意义是把 `RknpuSubmit` 从“内部运行时状态对象”降回“边界协议头”。

### 9.2 `RknpuQueuedSubmit`

作用：从 ioctl 边界进入 queue 时的输入容器。

它持有：

- `meta`
- reply-only 的 submit 字段
- shadow `tasks`

它只活在“刚入队”这道边界上，目的就是把边界输入整理成 queue 可以直接接管的形式。

### 9.3 `RknpuQueueTask`

作用：一个已经入队、正在被调度器管理的 submit 实体。

它维护 submit 的生命周期状态、lane cursor、running mask、完成计数、错误状态，以及最终把终态重新组装回 `RknpuSubmit` 所需的 reply 信息。

### 9.4 `RknpuTaskQueue`

作用：scheduler 内部的 submit 容器和 ready 选择器。

它维护：

- 全部 queue task 总表
- 按 priority 分桶的 ready 队列
- 下一个 queue task id

它只做 submit 选择和 reservation，不做 per-core in-flight ownership。

### 9.5 `RknpuDispatchReservation`

作用：queue 交给 scheduler 的最小“可发任务 token”。

它只说明：

- 哪个 submit
- 哪个 lane
- 哪个 task_index

不带 driver 绑定，不带 task 指针，不带 IRQ 期望值。

### 9.6 `InflightDispatch`

作用：scheduler 独占的 per-core in-flight 记录。

它把 reservation 扩成真正能完成一次硬件 dispatch 和一次 completion 匹配所需的最小信息，包括 task 指针和 expected IRQ mask。

### 9.7 `CoreCompletion`

作用：driver 向 scheduler 返回的最小 raw completion。

它只表达“哪个 core 观察到了什么 IRQ 状态”，不掺杂 queue 语义。

### 9.8 `CompletedSubmit`

作用：terminal submit 返回给 `card1` 的结果模型。

它把 scheduler 内部状态机收敛成用户态真正需要的三样结果：

- 最终 `submit`
- 最终 `tasks`
- 最终 `last_error`

### 9.9 `NpuSubmitWaiter`

作用：一个 submit 对应一个阻塞原语。

原始 ioctl 线程就睡在这里，直到 terminal path 把它唤醒。它解决的是“这个 submit 的提交线程什么时候返回”。

### 9.10 `kick: Event`

作用：全局 worker 唤醒器。

它不对应某个 submit，也不携带完成结果。它只负责在“有新活可做”时把 worker 从睡眠态叫起来。

## 10. 用一句话收尾

这版实现真正做成的事情其实很具体：把 `Submit` 从“调用线程自己 loop 推完整次 submit 的独占路径”，改成了“对外阻塞、对内由 queue + worker 按 core streaming dispatch 推进”的模型。queue 只管 submit 进度，scheduler 独占 in-flight ownership，driver 和 IRQ 只保留原始硬件完成边界，最终结果再由原始提交线程统一回填用户态。
