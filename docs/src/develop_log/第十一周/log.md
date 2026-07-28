# 第十一周开发日志（4.6-4.12）

## 工作总结

本周的工作进入了收尾阶段，围绕已经完成的 `rknpu` 调度器做最后测试、benchmark 验证


---

## 本周进展

- 完成了多轮 benchmark 测试，覆盖 `tiny_dispatch`、`mid_balanced`、`throughput_heavy` 和 `llama_decode_like` 等场景，重点对比 1-core 与 3-core 的提交时间、吞吐和并行效率。
- 对调度器关键路径补充了更细的日志，覆盖 enqueue、worker 唤醒、dispatch、harvest、terminal wake 和 blocking wait 等节点。

---

## 遇到的问题与分析

本周最麻烦的问题，是 benchmark 过程中出现了**偶发性的卡住或返回不稳定**。

从现象上看，有些测试可以正常打印 `benchmark complete status=0` 并返回 shell；但也有一些运行过程中，会出现串口长时间停住、需要手动退出终端的情况。它并不是每次都能稳定复现。

结合当前benchmark日志，至少可以得到两个判断：

1. **小任务场景下，多核并行并不一定带来正收益**

   在 `tiny_dispatch` 这类尺寸很小的 workload 上，submit/scheduler 开销占比很高，3-core 的效率明显达不到理想值，甚至可能比 1-core 更慢。这说明当前瓶颈更多在调度与提交流程，而不是硬件算力本身。

2. **中大型任务场景下，多核并行已经能稳定带来正收益**

   结合这轮 benchmark 记录，可以直接看到几组比较明确的结果：

   - `mid_balanced` 场景下，`1-core avg submit` 从 `56.158 ms` 降到 `39.204 ms`，`speedup = 1.432x`，`parallel efficiency = 47.75%`
   - `throughput_heavy` 场景下，`1-core avg submit` 从 `76.605 ms` 降到 `40.824 ms`，`speedup = 1.876x`，`parallel efficiency = 62.55%`
   - `llama_decode_like` 场景下，`1-core avg submit` 从 `423.358 ms` 降到 `231.858 ms`，`speedup = 1.826x`，`parallel efficiency = 60.86%`
---

## 本周结论

本周完成了调度器收尾阶段最重要的一轮验证工作。当前版本已经能够支撑 benchmark 跑通，并能产出比较完整的日志和性能数据；


---
