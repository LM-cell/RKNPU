# 第五周开发日志（2.22-2.28）

## 工作总结

- **RK3588 NPU 三核并行矩阵乘法实现** — 扩展 Transformer 结构体支持 3 套 regcmd 缓冲区，实现 `matmul_npu_3core_qkv` 函数，将 QKV 三个矩阵乘法并行提交到 3 个 NPU 核心，板端验证推理成功
- **驱动多核提交流程重构** — 修改 `submit_ioctrl` 实现批量任务分配，支持一次 ioctl 向多个核心提交任务，新增 `wait_all_npucore` 并行等待机制

---

### 验证结果

板端运行日志显示多核并行成功：

```
[261.640936] Total tasks to submit: 3, active cores: 3, max batch size: 4095
[261.650710] Total tasks to submit: 1, active cores: 1, max batch size: 4095
[262.295035] Total tasks to submit: 3, active cores: 3, max batch size: 4095
...
Once upon a time...
```

- `3 tasks, 3 cores` 表示 QKV 三核并行成功
- `1 tasks, 1 core` 表示后续的 `wo` 矩阵乘法（单核）
- 模型推理输出 "Once upon a time..." 验证结果正确

---

## 驱动多核提交流程重构

### 批量任务分配

重构 `submit_ioctrl` 函数，支持将用户空间的任务数组自动分配到多个 NPU 核心：

```rust
pub fn submit_ioctrl(&mut self, args: &mut RknpuSubmit) -> Result<(), RknpuError> {
    // 1. 刷新缓存，确保 NPU 能看到 CPU 写入的数据
    self.gem.comfirm_write_all()?;
    
    // 2. 提取活跃的核心任务
    let active_subcore: Vec<&RknpuSubcoreTask> = args.subcore_task.iter()
        .filter(|s| s.task_number > 0).collect();
    
    // 3. 批量提交到多个核心
    while task_iter < task_iter_end {
        let task_batch = active_subcore.len().min(task_iter_end - task_iter);
        let submit_tasks = unsafe { 
            core::slice::from_raw_parts_mut(task_ptr.add(task_iter), task_batch) 
        };
        
        // 并行启动每个核心的任务
        for idx in 0..active_subcore.len().min(task_batch) {
            self.base[idx].start_execute_one(idx, &self.data, &mut submit_tasks[idx], args)?;
        }
        
        // 并行等待所有核心完成
        self.wait_all_npucore(self.wait_fn, int_mask, submit_tasks)?;
        task_iter += task_batch;
    }
    
    // 4. 使缓存无效，确保 CPU 能读取 NPU 写入的结果
    self.gem.prepare_read_all()?;
    Ok(())
}
```

### 并行等待机制

新增 `wait_all_npucore` 函数，实现多核心并行等待：

```rust
pub fn wait_all_npucore(&self, normal_wait_fn: Option<fn()>, 
                        int_mask: Vec<u32>, 
                        submit_tasks: &mut [RknpuTask]) -> Result<(), RknpuError> {
    let mut done: [bool; 3] = [false; 3];
    
    if let Some(wait) = normal_wait_fn {
        // IRQ+WFI 模式：CPU 休眠等待中断
        loop {
            let status: Vec<u32> = self.base.iter()
                .map(|core| core.irq_status.load(Ordering::Acquire)).collect();
            
            // 检查每个核心的完成状态
            for idx in 0..submit_tasks.len() {
                if status[idx] & int_mask[idx] > 0 {
                    self.base[idx].clean_interrupts();
                    self.base[idx].irq_status.store(0, Ordering::Release);
                    submit_tasks[idx].int_status = int_mask[idx] & status[idx];
                    done[idx] = true;
                }
            }
            
            // 所有核心都完成则退出
            if done[..submit_tasks.len()].iter().filter(|&d| !d).count() == 0 {
                break;
            }
            
            // CPU 进入低功耗等待
            (wait)();
        }
    } else {
        panic!("[NPU] busy-poll mode not implemented for multi-core wait");
    }
    Ok(())
}
```