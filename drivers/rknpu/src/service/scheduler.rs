// Submit 延迟、调度事件与 Worker 等待策略，最后修改日期：2026-08-19。
use alloc::{
    collections::{BTreeMap, BTreeSet, VecDeque},
    sync::Arc,
    vec::Vec,
};
use core::{
    mem,
    ptr::{addr_of, addr_of_mut},
    sync::atomic::{AtomicBool, AtomicU64, Ordering},
};

use spin::Mutex;

use crate::{
    NPU_MAX_CORES, RknpuError, RknpuQueueTask, RknpuQueueTaskId, RknpuQueuedSubmit, RknpuTask,
    ioctrl::{
        RKNPU_DISPATCH_SOURCE_NONE, RKNPU_DISPATCH_SOURCE_READY,
        RKNPU_DISPATCH_SOURCE_RUNNING,
        RKNPU_SCHEDULE_EVENT_COMPLETE, RKNPU_SCHEDULE_EVENT_DISPATCH,
        RKNPU_SCHEDULE_EVENT_ENQUEUE, RKNPU_SCHEDULE_EVENT_FAILED,
        RKNPU_SCHEDULE_TRACE_CAPACITY, RKNPU_SCHEDULE_TRACE_NO_PRIORITY,
        RKNPU_SUBMIT_TRACE_CAPACITY, RknpuScheduleTraceRecord,
        RknpuSchedulerStateSnapshot, RknpuSubmit, RknpuSubmitTraceRecord,
        RknpuWorkerYieldTraceRecord,
    },
    task::taskqueen::SubmitLatencyTrace,
};

use super::{
    RknpuPlatform, RknpuService, RknpuServiceError, RknpuSubmitWaiter, RknpuWorkerListener,
    RknpuWorkerSignal, RknpuWorkerWaitMode,
};

/// Monotonic task-id generator for queued blocking submits.
static NEXT_QUEUE_TASK_ID: AtomicU64 = AtomicU64::new(1);

/// Terminal scheduler result returned to the ioctl path.
///
/// This keeps queue-internal ownership private while still returning the
/// submit header, task shadow, and terminal error that ioctl callers must copy
/// back to userspace.
pub struct CompletedSubmit {
    /// Rebuilt ABI-facing submit header.
    pub submit: RknpuSubmit,
    /// Final task shadow, including harvested `int_status`.
    pub tasks: Vec<RknpuTask>,
    /// Last terminal error recorded for this submit.
    pub last_error: Option<RknpuError>,
    /// Coarse phase timestamps copied out with the terminal submit.
    pub(crate) latency: SubmitLatencyTrace,
}

#[cfg(test)]
mod worker_yield_trace_tests {
    use super::*;
    use crate::ioctrl::RKNPU_WORKER_YIELD_REASON_INFLIGHT;

    fn record(start: u64, end: u64) -> RknpuWorkerYieldTraceRecord {
        RknpuWorkerYieldTraceRecord {
            sequence: u64::MAX,
            queue_task: 7,
            yield_start_ns: start,
            yield_end_ns: end,
            reason: RKNPU_WORKER_YIELD_REASON_INFLIGHT,
            reserved: 0,
        }
    }

    #[test]
    fn disabled_worker_yield_trace_drops_records_without_allocating() {
        let mut trace = WorkerYieldTraceBuffer::new();
        trace.push(record(10, 20));

        assert!(!trace.enabled);
        assert_eq!(trace.records.capacity(), 0);
        assert!(trace.records.is_empty());
    }

    #[test]
    fn enabled_worker_yield_trace_assigns_monotonic_sequences() {
        let mut trace = WorkerYieldTraceBuffer::configured(true, 2).unwrap();
        trace.push(record(10, 20));
        trace.push(record(30, 50));
        trace.push(record(60, 90));

        assert_eq!(trace.records.len(), 2);
        assert_eq!(trace.capacity, 2);
        assert_eq!(trace.records[0].sequence, 0);
        assert_eq!(trace.records[1].sequence, 1);
        assert_eq!(trace.records[1].yield_end_ns - trace.records[1].yield_start_ns, 20);
        assert!(trace.overflowed);
    }
}

/// Scheduler-owned binding between one physical core and one running lane.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct CoreRunBinding {
    /// Live submit id that owns the dispatch.
    task_id: RknpuQueueTaskId,
    /// Logical lane inside that submit.
    lane_slot: u8,
    /// Absolute task index inside `RknpuQueueTask.tasks`.
    task_index: u32,
}

/// Ephemeral driver-call arguments prepared under the scheduler mutex.
#[derive(Clone, Copy)]
struct DispatchSetup {
    /// Target physical core for this driver call.
    core_slot: usize,
    /// Scheduler-owned submit/lane/task binding recorded for this core.
    binding: CoreRunBinding,
    /// Job-mode flags copied from the submit header.
    submit_flags: u32,
    /// Total number of task descriptors in this submit.
    task_total: u32,
    /// DMA base of the task descriptor array.
    task_array_dma_address: u64,
    /// Snapshot of the task descriptor used to program hardware.
    task: RknpuTask,
    /// Submit 调度优先级，数值越小越先被同类队列选择。
    priority: i32,
    /// 从 packed Task 安全复制出的操作编号，用于测试匹配事件。
    op_idx: u32,
    /// 本次 Dispatch 是 Running 续派还是 Ready 提升。
    dispatch_source: u32,
    /// 作出决策时空闲核心可执行的最高优先级 Ready Submit。
    ready_priority: i32,
}

/// Mutable scheduler state protected by one mutex.
///
/// `tasks` owns every live submit. `ready`, `running`, and `complete` are only
/// classification/lookup structures around that owner set.
struct NpuSchedulerState<W: RknpuSubmitWaiter> {
    /// Unique owner of every live submit that is not terminal yet.
    tasks: BTreeMap<RknpuQueueTaskId, RknpuQueueTask>,
    /// Ready buckets keyed by submit priority.
    ready: BTreeMap<i32, VecDeque<RknpuQueueTaskId>>,
    /// Running buckets keyed by submit priority.
    running: BTreeMap<i32, VecDeque<RknpuQueueTaskId>>,
    /// Physical core -> running submit/lane/task binding.
    core_binding: BTreeMap<usize, CoreRunBinding>,
    /// Terminal submits waiting for `take_terminal_submit(task_id)`.
    complete: BTreeMap<RknpuQueueTaskId, RknpuQueueTask>,
    /// Per-submit waiters keyed by queue task id.
    waiters: BTreeMap<RknpuQueueTaskId, Arc<W>>,
    /// Whether the singleton worker was already spawned.
    worker_started: bool,
}

impl<W: RknpuSubmitWaiter> NpuSchedulerState<W> {
    /// Create an empty scheduler state with no worker marked as started.
    fn new() -> Self {
        Self {
            tasks: BTreeMap::new(),
            ready: BTreeMap::new(),
            running: BTreeMap::new(),
            core_binding: BTreeMap::new(),
            complete: BTreeMap::new(),
            waiters: BTreeMap::new(),
            worker_started: false,
        }
    }

    /// Return true while at least one submit still owns live scheduler state.
    fn has_live_work(&self) -> bool {
        !self.tasks.is_empty()
    }

    /// Return true while at least one physical core has an active binding.
    fn has_inflight(&self) -> bool {
        !self.core_binding.is_empty()
    }

    /// Insert one queued submit into the ready buckets and assign an id.
    fn enqueue_task(&mut self, queued_submit: RknpuQueuedSubmit) -> RknpuQueueTaskId {
        let task_id = NEXT_QUEUE_TASK_ID.fetch_add(1, Ordering::Relaxed);
        let task = RknpuQueueTask::new(task_id, queued_submit);
        let priority = task.meta.priority;
        self.tasks.insert(task_id, task);
        Self::push_bucket(&mut self.ready, priority, task_id);
        task_id
    }

    /// Remove one terminal submit from the completion map.
    fn take_terminal_task(&mut self, task_id: RknpuQueueTaskId) -> Option<RknpuQueueTask> {
        self.complete.remove(&task_id)
    }

    /// Return every physical core that does not currently own a dispatch.
    fn idle_cores(&self) -> Vec<usize> {
        (0..NPU_MAX_CORES)
            .filter(|core_slot| !self.core_binding.contains_key(core_slot))
            .collect()
    }

    /// Check whether any running submit can fill one of the listed idle cores.
    fn has_running_candidate_for_any(&self, idle_cores: &[usize]) -> bool {
        idle_cores
            .iter()
            .copied()
            .any(|core_slot| self.find_running_candidate_for_core(core_slot).is_some())
    }

    /// 返回任一空闲核心能够执行的最高优先级 Ready Submit。
    ///
    /// 该结果只写入调度 trace，用来判断 Running Submit 是否压住了更高
    /// 优先级的 Ready Submit；它不参与候选任务的选择，不改变调度策略。
    fn best_ready_priority_for_any(&self, idle_cores: &[usize]) -> Option<i32> {
        idle_cores
            .iter()
            .filter_map(|core_slot| self.best_ready_priority_for_core(*core_slot))
            .min()
    }

    /// 返回指定核心能够执行的最高优先级 Ready Submit。
    ///
    /// 必须同时满足 core mask 和存在可下发 lane，避免把不能在该核心执行的
    /// Submit 误报为优先级违规。
    fn best_ready_priority_for_core(&self, core_slot: usize) -> Option<i32> {
        self.ready.iter().find_map(|(priority, queue)| {
            queue
                .iter()
                .filter_map(|task_id| self.tasks.get(task_id))
                .any(|task| task.has_dispatchable_task() && task.allows_core(core_slot))
                .then_some(*priority)
        })
    }

    /// Find the next lane from an already-running submit that may use this core.
    fn find_running_candidate_for_core(&self, core_slot: usize) -> Option<CoreRunBinding> {
        for queue in self.running.values() {
            for task_id in queue {
                let Some(task) = self.tasks.get(task_id) else {
                    continue;
                };
                if !task.allows_core(core_slot) {
                    continue;
                }
                if let Some((lane_slot, task_index)) = task.next_dispatchable_task() {
                    return Some(CoreRunBinding {
                        task_id: *task_id,
                        lane_slot: lane_slot as u8,
                        task_index,
                    });
                }
            }
        }

        None
    }

    /// Pop the highest-priority ready submit that can dispatch on this core.
    fn pop_ready_candidate_for_core(&mut self, core_slot: usize) -> Option<RknpuQueueTaskId> {
        let priorities = self.ready.keys().copied().collect::<Vec<_>>();
        for priority in priorities {
            let mut selected = None;
            if let Some(queue) = self.ready.get(&priority) {
                selected = queue.iter().position(|task_id| {
                    self.tasks.get(task_id).is_some_and(|task| {
                        task.allows_core(core_slot) && task.has_dispatchable_task()
                    })
                });
            }

            if let Some(index) = selected {
                let task_id = self.ready.get_mut(&priority)?.remove(index)?;
                if self.ready.get(&priority).is_some_and(VecDeque::is_empty) {
                    self.ready.remove(&priority);
                }
                return Some(task_id);
            }

            if self.ready.get(&priority).is_some_and(VecDeque::is_empty) {
                self.ready.remove(&priority);
            }
        }

        None
    }

    /// Reserve a dispatch from an already-running submit.
    fn prepare_dispatch_from_running(&mut self, core_slot: usize) -> Option<DispatchSetup> {
        let binding = self.find_running_candidate_for_core(core_slot)?;
        self.prepare_dispatch(core_slot, binding)
    }

    /// Promote one ready submit to running and reserve its first dispatch.
    fn promote_ready_and_prepare_dispatch(&mut self, core_slot: usize) -> Option<DispatchSetup> {
        let task_id = self.pop_ready_candidate_for_core(core_slot)?;
        let priority = self.tasks.get(&task_id)?.meta.priority;
        Self::push_bucket(&mut self.running, priority, task_id);

        let task = self.tasks.get(&task_id)?;
        let (lane_slot, task_index) = task.next_dispatchable_task()?;
        let binding = CoreRunBinding {
            task_id,
            lane_slot: lane_slot as u8,
            task_index,
        };
        self.prepare_dispatch(core_slot, binding)
    }

    /// Bind one lane to a core and build the driver-call snapshot for it.
    fn prepare_dispatch(
        &mut self,
        core_slot: usize,
        binding: CoreRunBinding,
    ) -> Option<DispatchSetup> {
        if self.core_binding.contains_key(&core_slot) {
            return None;
        }

        let task = self.tasks.get_mut(&binding.task_id)?;
        let meta = task.meta;
        let task_desc = task.tasks.get_mut(binding.task_index as usize)?;
        unsafe {
            addr_of_mut!((*task_desc).int_status).write_unaligned(0);
        }
        let task_snapshot = *task_desc;
        let op_idx = unsafe { addr_of!((*task_desc).op_idx).read_unaligned() };

        if !task.reserve_dispatch(binding.lane_slot as usize, binding.task_index) {
            return None;
        }
        self.core_binding.insert(core_slot, binding);
        Some(DispatchSetup {
            core_slot,
            binding,
            // 动态 Task 标志只供调度器使用，最后修改日期：2026-08-18。
            // 底层寄存器路径仅接收原有硬件标志。
            submit_flags: meta.hardware_flags(),
            task_total: task.tasks.len() as u32,
            task_array_dma_address: meta.task_array_dma_address,
            task: task_snapshot,
            priority: meta.priority,
            op_idx,
            dispatch_source: RKNPU_DISPATCH_SOURCE_NONE,
            ready_priority: RKNPU_SCHEDULE_TRACE_NO_PRIORITY,
        })
    }

    /// Move a task between ready, running, and complete buckets after state changes.
    fn reclassify_task(&mut self, task_id: RknpuQueueTaskId) -> Option<RknpuQueueTaskId> {
        Self::remove_from_buckets(&mut self.ready, task_id);
        Self::remove_from_buckets(&mut self.running, task_id);

        let Some(task) = self.tasks.get(&task_id) else {
            return None;
        };
        let priority = task.meta.priority;
        let has_running = task.has_running_tasks();
        let has_dispatchable = task.has_dispatchable_task();
        let terminal_success = task.is_terminal_success();
        let terminal_fault = task.is_terminal_fault();

        if terminal_success || terminal_fault {
            if let Some(task) = self.tasks.remove(&task_id) {
                self.complete.insert(task_id, task);
                return Some(task_id);
            }
            return None;
        }

        if has_running {
            Self::push_bucket(&mut self.running, priority, task_id);
        } else if has_dispatchable {
            Self::push_bucket(&mut self.ready, priority, task_id);
        }

        None
    }

    /// Add a task id to a priority bucket without duplicating it.
    fn push_bucket(
        buckets: &mut BTreeMap<i32, VecDeque<RknpuQueueTaskId>>,
        priority: i32,
        task_id: RknpuQueueTaskId,
    ) {
        let queue = buckets.entry(priority).or_default();
        if !queue.iter().any(|existing| *existing == task_id) {
            queue.push_back(task_id);
        }
    }

    /// Remove a task id from all priority buckets and drop empty buckets.
    fn remove_from_buckets(
        buckets: &mut BTreeMap<i32, VecDeque<RknpuQueueTaskId>>,
        task_id: RknpuQueueTaskId,
    ) {
        let priorities = buckets.keys().copied().collect::<Vec<_>>();
        for priority in priorities {
            let mut remove_bucket = false;
            if let Some(queue) = buckets.get_mut(&priority) {
                if let Some(index) = queue.iter().position(|existing| *existing == task_id) {
                    let _ = queue.remove(index);
                }
                remove_bucket = queue.is_empty();
            }
            if remove_bucket {
                buckets.remove(&priority);
            }
        }
    }

    /// 在持有调度锁时复制资源计数，最后修改日期：2026-08-07。
    ///
    /// 返回值不包含任何指针或对象引用，因此调用者拿到结构后即可释放
    /// 调度锁，再单独获取设备锁读取 GEM 状态，避免形成嵌套锁顺序。
    fn snapshot(&self) -> RknpuSchedulerStateSnapshot {
        RknpuSchedulerStateSnapshot {
            live_submits: self.tasks.len() as u32,
            ready_entries: self.ready.values().map(VecDeque::len).sum::<usize>() as u32,
            running_entries: self.running.values().map(VecDeque::len).sum::<usize>() as u32,
            complete_entries: self.complete.len() as u32,
            waiters: self.waiters.len() as u32,
            core_bindings: self.core_binding.len() as u32,
            ..RknpuSchedulerStateSnapshot::default()
        }
    }
}

pub(super) struct RknpuScheduler<P: RknpuPlatform> {
    state: Mutex<NpuSchedulerState<P::Waiter>>,
    /// 固定上限的测试记录，与任务调度状态分开加锁。
    trace: Mutex<SubmitTraceBuffer>,
    /// 默认关闭的调度事件记录；独立锁保证记录读取不长期占用调度状态锁。
    schedule_trace: Mutex<ScheduleTraceBuffer>,
    /// 兼容旧测试 ioctl 的 yield 记录缓冲区；IRQ 阻塞等待路径不再写入新记录。
    worker_yield_trace: Mutex<WorkerYieldTraceBuffer>,
    /// Fast disabled-path check so normal scheduling does not take the trace lock.
    worker_yield_trace_enabled: AtomicBool,
    kick: P::WorkerSignal,
}

/// 固定容量的调度事件缓冲区。
///
/// 缓冲区只追加，不覆盖旧事件。达到上限后保留已有记录并设置
/// `overflowed`，让用户态拒绝使用不完整样本，而不是把首尾事件拼接起来。
struct ScheduleTraceBuffer {
    /// 按实际写入顺序保存的原始事件。
    records: Vec<RknpuScheduleTraceRecord>,
    /// 用户选择的事件类型；0 表示关闭调度事件记录。
    event_mask: u32,
    /// 下一条成功保存的事件序号。
    next_sequence: u64,
    /// 内核容量或用户读取容量不足时报告不完整数据。
    overflowed: bool,
}

impl ScheduleTraceBuffer {
    /// 创建默认关闭且不预分配大缓冲区的状态。
    fn new() -> Self {
        Self {
            records: Vec::new(),
            event_mask: 0,
            next_sequence: 0,
            overflowed: false,
        }
    }

    /// 为一次新实验创建干净缓冲区。
    ///
    /// 仅在 `event_mask != 0` 时分配 16,384 条记录空间，普通启动路径不会
    /// 为关闭的测试功能占用约 896 KiB 内存。
    fn configured(event_mask: u32) -> Self {
        Self {
            records: if event_mask == 0 {
                Vec::new()
            } else {
                Vec::with_capacity(RKNPU_SCHEDULE_TRACE_CAPACITY)
            },
            event_mask,
            next_sequence: 0,
            overflowed: false,
        }
    }

    /// 按事件掩码追加记录，并在这里统一分配连续序号。
    fn push(&mut self, mut record: RknpuScheduleTraceRecord) {
        if self.event_mask & record.event_type == 0 {
            return;
        }
        if self.records.len() >= RKNPU_SCHEDULE_TRACE_CAPACITY {
            self.overflowed = true;
            return;
        }
        record.sequence = self.next_sequence;
        self.next_sequence = self.next_sequence.wrapping_add(1);
        self.records.push(record);
    }
}

/// Bounded, append-only buffer for the focused yield-gap experiment.
///
/// It is empty and disabled during normal operation. CONFIG_RESET allocates
/// the complete backing storage before measurements start, so Worker writes
/// never allocate and never print to the serial console.
struct WorkerYieldTraceBuffer {
    records: Vec<RknpuWorkerYieldTraceRecord>,
    capacity: usize,
    enabled: bool,
    next_sequence: u64,
    overflowed: bool,
}

impl WorkerYieldTraceBuffer {
    fn new() -> Self {
        Self {
            records: Vec::new(),
            capacity: 0,
            enabled: false,
            next_sequence: 0,
            overflowed: false,
        }
    }

    fn configured(enabled: bool, capacity: usize) -> Result<Self, RknpuServiceError> {
        let mut records = Vec::new();
        if enabled {
            records
                .try_reserve_exact(capacity)
                .map_err(|_| RknpuServiceError::Driver(RknpuError::MemoryError))?;
        }

        Ok(Self {
            records,
            capacity: if enabled { capacity } else { 0 },
            enabled,
            next_sequence: 0,
            overflowed: false,
        })
    }

    fn push(&mut self, mut record: RknpuWorkerYieldTraceRecord) {
        if !self.enabled {
            return;
        }
        if self.records.len() >= self.capacity {
            self.overflowed = true;
            return;
        }
        record.sequence = self.next_sequence;
        self.next_sequence = self.next_sequence.wrapping_add(1);
        self.records.push(record);
    }
}

/// 保存 Submit 原始时间戳，缓冲区满后只标记溢出。
struct SubmitTraceBuffer {
    records: Vec<RknpuSubmitTraceRecord>,
    overflowed: bool,
}

impl SubmitTraceBuffer {
    fn new() -> Self {
        Self {
            records: Vec::with_capacity(RKNPU_SUBMIT_TRACE_CAPACITY),
            overflowed: false,
        }
    }

    fn reset(&mut self) {
        self.records.clear();
        self.overflowed = false;
    }

    fn push(&mut self, record: RknpuSubmitTraceRecord) {
        if self.records.len() >= RKNPU_SUBMIT_TRACE_CAPACITY {
            self.overflowed = true;
            return;
        }
        self.records.push(record);
    }

}

impl<P: RknpuPlatform> RknpuScheduler<P> {
    /// Build the scheduler state and worker wake-up primitive for one service.
    pub(super) fn new(platform: &P) -> Self {
        Self {
            state: Mutex::new(NpuSchedulerState::new()),
            trace: Mutex::new(SubmitTraceBuffer::new()),
            schedule_trace: Mutex::new(ScheduleTraceBuffer::new()),
            worker_yield_trace: Mutex::new(WorkerYieldTraceBuffer::new()),
            worker_yield_trace_enabled: AtomicBool::new(false),
            kick: platform.new_worker_signal(),
        }
    }
}

impl<P: RknpuPlatform> RknpuService<P> {
    /// 清空测试记录，供一次新实验开始前调用。
    pub(crate) fn reset_submit_trace(&self) {
        self.inner.scheduler.trace.lock().reset();
    }

    /// 复制指定数量的记录快照，释放锁后再向用户空间返回。
    pub(crate) fn snapshot_submit_trace(
        &self,
        capacity: usize,
    ) -> (Vec<RknpuSubmitTraceRecord>, bool) {
        // 先分配快照空间，避免在自旋锁内调用分配器。
        let mut records = Vec::with_capacity(capacity);
        let overflowed = {
            let trace = self.inner.scheduler.trace.lock();
            let count = trace.records.len().min(capacity);
            records.extend_from_slice(&trace.records[..count]);
            trace.overflowed || count < trace.records.len()
        };
        (records, overflowed)
    }

    /// 在 Submit 返回边界保存一条完整记录。
    pub(crate) fn record_submit_trace(&self, record: RknpuSubmitTraceRecord) {
        self.inner.scheduler.trace.lock().push(record);
    }

    /// Start a fresh worker-yield experiment or return to the disabled state.
    ///
    /// The backing allocation and old-buffer drop happen outside the trace
    /// lock. Enabling is published only after the fresh buffer is installed;
    /// disabling is published first so the Worker stops producing records
    /// before the old buffer is removed.
    pub(crate) fn configure_worker_yield_trace(
        &self,
        enabled: bool,
        capacity: usize,
    ) -> Result<(), RknpuServiceError> {
        if !enabled {
            self.inner
                .scheduler
                .worker_yield_trace_enabled
                .store(false, Ordering::Release);
        }

        let replacement = WorkerYieldTraceBuffer::configured(enabled, capacity)?;
        let previous = {
            let mut trace = self.inner.scheduler.worker_yield_trace.lock();
            mem::replace(&mut *trace, replacement)
        };
        drop(previous);

        if enabled {
            self.inner
                .scheduler
                .worker_yield_trace_enabled
                .store(true, Ordering::Release);
        }
        Ok(())
    }

    /// Copy a stable trace snapshot; userspace copy-out happens after unlock.
    pub(crate) fn snapshot_worker_yield_trace(
        &self,
        capacity: usize,
    ) -> Result<(Vec<RknpuWorkerYieldTraceRecord>, bool, bool), RknpuServiceError> {
        // Size the snapshot from the current record count, not the userspace
        // ceiling. A second lock makes concurrent reset/read conservative:
        // records appended between locks cause overflowed=true rather than an
        // allocation beyond the captured limit.
        let snapshot_limit = {
            let trace = self.inner.scheduler.worker_yield_trace.lock();
            trace.records.len().min(capacity)
        };
        let mut records = Vec::new();
        records
            .try_reserve_exact(snapshot_limit)
            .map_err(|_| RknpuServiceError::Driver(RknpuError::MemoryError))?;
        let (enabled, overflowed) = {
            let trace = self.inner.scheduler.worker_yield_trace.lock();
            let count = trace.records.len().min(capacity).min(snapshot_limit);
            records.extend_from_slice(&trace.records[..count]);
            (
                trace.enabled,
                trace.overflowed || count < trace.records.len(),
            )
        };
        Ok((records, enabled, overflowed))
    }

    /// 配置并清空调度事件记录。
    ///
    /// 新缓冲区的分配和旧缓冲区的释放都在自旋锁外完成；锁内只执行一次
    /// `mem::replace`，防止大内存分配延长不可抢占的临界区。
    pub(crate) fn configure_schedule_trace(&self, event_mask: u32) {
        // 在锁外分配和释放大缓冲区，锁内只替换状态。
        let replacement = ScheduleTraceBuffer::configured(event_mask);
        let previous = {
            let mut trace = self.inner.scheduler.schedule_trace.lock();
            mem::replace(&mut *trace, replacement)
        };
        drop(previous);
    }

    /// 复制调度事件到内核快照，释放 trace 锁后再向用户空间返回。
    ///
    /// 这样 copy-out 发生缺页或失败时不会阻塞调度事件写入，也不会让用户
    /// 指针访问发生在自旋锁保护范围内。
    pub(crate) fn snapshot_schedule_trace(
        &self,
        capacity: usize,
    ) -> (Vec<RknpuScheduleTraceRecord>, u32, bool) {
        let mut records = Vec::with_capacity(capacity);
        let (event_mask, overflowed) = {
            let trace = self.inner.scheduler.schedule_trace.lock();
            let count = trace.records.len().min(capacity);
            records.extend_from_slice(&trace.records[..count]);
            (
                trace.event_mask,
                trace.overflowed || count < trace.records.len(),
            )
        };
        (records, event_mask, overflowed)
    }

    /// 保存一条调度事件。
    ///
    /// 调用处不得持有调度器状态锁或设备锁，保持锁顺序为“业务状态处理
    /// 完成后再记录”，避免 trace 锁与调度锁、设备锁构成环路。
    fn record_schedule_event(&self, record: RknpuScheduleTraceRecord) {
        self.inner.scheduler.schedule_trace.lock().push(record);
    }

    /// 读取调度器状态；返回的是纯计数，函数返回时调度锁已经释放。
    pub(crate) fn snapshot_scheduler_state(&self) -> RknpuSchedulerStateSnapshot {
        self.inner.scheduler.state.lock().snapshot()
    }

    /// Enqueue one submit, install its waiter, and wake the worker if needed.
    pub fn enqueue_submit(
        &self,
        queued_submit: RknpuQueuedSubmit,
    ) -> Result<RknpuQueueTaskId, RknpuServiceError> {
        let waiter = Arc::new(self.inner.platform.new_waiter());
        let submit_snapshot = queued_submit.meta;
        // RknpuTask 使用 packed ABI；先按未对齐方式读出字段，后续 trace
        // 不再持有或引用用户任务数组。
        let first_op_idx = queued_submit
            .tasks
            .first()
            .map(|task| unsafe { addr_of!((*task).op_idx).read_unaligned() })
            .unwrap_or(0);
        let (task_id, spawn_worker, enqueue_time_ns) = {
            let mut state = self.inner.scheduler.state.lock();
            let spawn_worker = !state.worker_started;
            if spawn_worker {
                state.worker_started = true;
            }

            let task_id = state.enqueue_task(queued_submit);
            // t1：任务已经进入调度队列。
            let enqueue_time_ns = self.inner.platform.monotonic_time_ns();
            if let Some(task) = state.tasks.get_mut(&task_id) {
                task.mark_enqueued(enqueue_time_ns);
            }
            state.waiters.insert(task_id, waiter);
            (task_id, spawn_worker, enqueue_time_ns)
        };

        // 调度状态锁已经释放，此处只保存用于建立 queue_task 映射的入队事件。
        self.record_schedule_event(RknpuScheduleTraceRecord {
            timestamp_ns: enqueue_time_ns,
            queue_task: task_id,
            priority: submit_snapshot.priority,
            event_type: RKNPU_SCHEDULE_EVENT_ENQUEUE,
            op_idx: first_op_idx,
            task_index: 0,
            core_slot: u32::MAX,
            lane_slot: u32::MAX,
            ..RknpuScheduleTraceRecord::default()
        });

        debug!(
            "[rknpu-scheduler] enqueue queue_task={} priority={} task_number={} core_mask={:#x} \
             task_base_addr={:#x} spawn_worker={} subcore0=({}, {}) subcore1=({}, {}) \
             subcore2=({}, {})",
            task_id,
            submit_snapshot.priority,
            submit_snapshot.task_total,
            submit_snapshot.core_mask,
            submit_snapshot.task_array_dma_address,
            spawn_worker,
            submit_snapshot.lane_ranges[0].task_start,
            submit_snapshot.lane_ranges[0].task_number,
            submit_snapshot.lane_ranges[1].task_start,
            submit_snapshot.lane_ranges[1].task_number,
            submit_snapshot.lane_ranges[2].task_start,
            submit_snapshot.lane_ranges[2].task_number
        );

        if spawn_worker {
            debug!("[rknpu-scheduler] spawning worker thread");
            let service = self.clone();
            self.inner
                .platform
                .spawn_worker(move || service.worker_main());
        }

        self.inner.scheduler.kick.notify_one();
        Ok(task_id)
    }

    /// 通知调度 worker 已有新的 NPU completion 状态可回收。
    ///
    /// 最后修改日期：2026-08-19。Event/Waker 模式转发到 IRQ-safe 唤醒对象；
    /// YieldPolling 模式由 Worker 轮询状态，不产生额外 Event 通知。两种模式都
    /// 不获取调度状态锁、不操作 Ready/Running 队列，也不直接派发任务。
    pub fn notify_irq_completion(&self) {
        if self.inner.worker_wait_mode == RknpuWorkerWaitMode::IrqEvent {
            self.inner.scheduler.kick.notify_one();
        }
    }

    /// Block the caller until the specified submit becomes terminal.
    pub fn wait_for_submit(&self, task_id: RknpuQueueTaskId) -> Result<(), RknpuServiceError> {
        let waiter = {
            let state = self.inner.scheduler.state.lock();
            state
                .waiters
                .get(&task_id)
                .cloned()
                .ok_or(RknpuServiceError::NotFound)?
        };

        debug!("[rknpu-scheduler] wait start queue_task={}", task_id);
        if let Err(err) = waiter.wait() {
            warn!(
                "[rknpu-scheduler] wait interrupted queue_task={} err={:?}",
                task_id, err
            );
            self.abort_wait(task_id);
            return Err(err);
        }
        debug!("[rknpu-scheduler] wait done queue_task={}", task_id);
        Ok(())
    }

    /// Consume one terminal submit and rebuild the ioctl-facing result.
    pub fn take_terminal_submit(
        &self,
        task_id: RknpuQueueTaskId,
    ) -> Result<CompletedSubmit, RknpuServiceError> {
        let mut state = self.inner.scheduler.state.lock();
        let task = state
            .take_terminal_task(task_id)
            .ok_or(RknpuServiceError::InvalidData)?;
        state.waiters.remove(&task_id);

        let submit = task.build_submit();
        let latency = task.latency;
        let tasks = task.tasks;
        let last_error = task.last_error;
        Ok(CompletedSubmit {
            submit,
            tasks,
            last_error,
            latency,
        })
    }

    /// Run one low-level driver closure through the platform's locking policy.
    pub(crate) fn with_npu_driver<T, F>(&self, f: F) -> Result<T, RknpuServiceError>
    where
        F: FnOnce(&mut crate::Rknpu) -> Result<T, RknpuError>,
    {
        self.inner.platform.with_device(f)
    }

    /// 唤醒 Submit 线程并返回
    fn wake_terminal_tasks(&self, task_ids: Vec<RknpuQueueTaskId>, prepare_read: bool) {
        if task_ids.is_empty() {
            return;
        }

        debug!(
            "[rknpu-scheduler] wake_terminal_tasks ids={:?} prepare_read={} count={}",
            task_ids,
            prepare_read,
            task_ids.len()
        );

        let prepare_error = if prepare_read {
            self.with_npu_driver(|rknpu_dev| rknpu_dev.prepare_read_all())
                .err()
        } else {
            None
        };

        let waiters = {
            let mut state = self.inner.scheduler.state.lock();
            let mut waiters = Vec::with_capacity(task_ids.len());

            for task_id in task_ids {
                if prepare_error.is_some() {
                    if let Some(task) = state.complete.get_mut(&task_id) {
                        task.last_error = Some(RknpuError::MemoryError);
                    }
                }

                if let Some(waiter) = state.waiters.get(&task_id).cloned() {
                    waiters.push(waiter);
                } else {
                    state.complete.remove(&task_id);
                }
            }

            waiters
        };

        for waiter in waiters {
            waiter.complete();
        }
    }

    /// Convert one failed dispatch into task state and wake it if terminal.
    fn fail_dispatch(&self, core_slot: usize, binding: CoreRunBinding, err: RknpuError) {
        let terminal_ids = {
            let mut state = self.inner.scheduler.state.lock();
            state.core_binding.remove(&core_slot);

            let mut terminal_ids = Vec::new();
            if let Some(task) = state.tasks.get_mut(&binding.task_id) {
                task.fail_dispatch(binding.lane_slot as usize, err);
            }

            if let Some(task_id) = state.reclassify_task(binding.task_id) {
                terminal_ids.push(task_id);
            }

            terminal_ids
        };

        self.wake_terminal_tasks(terminal_ids, false);
    }

    /// Harvest completed cores, update task shadows, and wake terminal submits.
    fn harvest_completed_cores(&self) -> bool {
        let completions =
            match self.with_npu_driver(|rknpu_dev| Ok(rknpu_dev.harvest_completed_dispatches())) {
                Ok(completions) => completions,
                Err(err) => {
                    warn!(
                        "[rknpu-scheduler] failed to harvest completed cores: {:?}",
                        err
                    );
                    return false;
                }
            };

        if completions.is_empty() {
            return false;
        }

        // 先在调度锁内更新完成状态并构造事件，释放调度锁后统一写 trace。
        let mut schedule_events = Vec::with_capacity(completions.len());
        let terminal_ids = {
            let mut state = self.inner.scheduler.state.lock();
            let mut terminal_ids = Vec::new();
            for completion in completions {
                // 当前核心完成，用于判断整个 Submit 的 t3。
                let completion_time_ns = self.inner.platform.monotonic_time_ns();
                let core_slot = completion.core_slot as usize;
                let Some(binding) = state.core_binding.remove(&core_slot) else {
                    debug!(
                        "[rknpu-scheduler] harvested completion on core={} without core binding \
                         observed={:#x}",
                        core_slot, completion.observed_irq_status
                    );
                    continue;
                };

                let mut last_task_int_status = 0u32;
                let mut task_error = false;

                let Some(task) = state.tasks.get_mut(&binding.task_id) else {
                    debug!(
                        "[rknpu-scheduler] completion core={} queue_task={} missing_live_task=true",
                        core_slot, binding.task_id
                    );
                    continue;
                };
                let priority = task.meta.priority;
                let mut op_idx = 0;
                if let Some(task_desc) = task.tasks.get_mut(binding.task_index as usize) {
                    // packed Task 字段按值读取，避免创建未对齐引用。
                    op_idx = unsafe { addr_of!((*task_desc).op_idx).read_unaligned() };
                    let expected_irq_mask = task_desc.int_mask;
                    last_task_int_status = completion.observed_irq_status & expected_irq_mask;
                    task_error =
                        completion.observed_irq_status != 0 && last_task_int_status == 0;

                    unsafe {
                        addr_of_mut!((*task_desc).int_status).write_unaligned(last_task_int_status);
                    }
                    task.complete_dispatch(binding.lane_slot as usize);
                    if task_error {
                        task.last_error = Some(RknpuError::TaskError);
                    }
                } else {
                    task.fail_dispatch(
                        binding.lane_slot as usize,
                        RknpuError::InvalidParameter,
                    );
                    debug!(
                        "[rknpu-scheduler] completion core={} queue_task={} invalid_task_index={}",
                        core_slot, binding.task_id, binding.task_index
                    );
                }

                schedule_events.push(RknpuScheduleTraceRecord {
                    timestamp_ns: completion_time_ns,
                    queue_task: binding.task_id,
                    priority,
                    event_type: RKNPU_SCHEDULE_EVENT_COMPLETE,
                    op_idx,
                    task_index: binding.task_index,
                    core_slot: core_slot as u32,
                    lane_slot: binding.lane_slot as u32,
                    ..RknpuScheduleTraceRecord::default()
                });

                debug!(
                    "[rknpu-scheduler] harvested completion core={} queue_task={} task_index={} \
                     lane={} observed={:#x} last_task_int_status={:#x} task_error={}",
                    core_slot,
                    binding.task_id,
                    binding.task_index,
                    binding.lane_slot,
                    completion.observed_irq_status,
                    last_task_int_status,
                    task_error
                );

                if let Some(task_id) = state.reclassify_task(binding.task_id) {
                    if let Some(task) = state.complete.get_mut(&task_id) {
                        // t3：最后一个 Task 已完成，整个 Submit 进入终态。
                        task.mark_terminal(completion_time_ns);
                    }
                    terminal_ids.push(task_id);
                }
            }

            terminal_ids
        };

        for event in schedule_events {
            self.record_schedule_event(event);
        }

        self.wake_terminal_tasks(terminal_ids, true);
        true
    }

    /// Dispatch queued work onto idle cores until no more immediate dispatch fits.
    fn dispatch_idle_cores(&self) -> bool {
        let mut dispatched = false;
        let mut confirmed_submit_ids = BTreeSet::new();
        loop {
            let setup = {
                let mut state = self.inner.scheduler.state.lock();
                let idle_cores = state.idle_cores();
                if idle_cores.is_empty() {
                    None
                } else {
                    // 原调度策略保持不变：只要 Running Submit 还能续派，
                    // 就不从 Ready 提升新 Submit；ready_priority 仅用于记录当时竞争者。
                    let running_has_candidate = state.has_running_candidate_for_any(&idle_cores);
                    let ready_priority = state
                        .best_ready_priority_for_any(&idle_cores)
                        .unwrap_or(RKNPU_SCHEDULE_TRACE_NO_PRIORITY);
                    let mut prepared = None;

                    for core_slot in idle_cores {
                        if let Some(mut setup) = state.prepare_dispatch_from_running(core_slot) {
                            setup.dispatch_source = RKNPU_DISPATCH_SOURCE_RUNNING;
                            setup.ready_priority = ready_priority;
                            prepared = Some(setup);
                            break;
                        }

                        if !running_has_candidate {
                            let core_ready_priority = state
                                .best_ready_priority_for_core(core_slot)
                                .unwrap_or(RKNPU_SCHEDULE_TRACE_NO_PRIORITY);
                            if let Some(mut setup) =
                                state.promote_ready_and_prepare_dispatch(core_slot)
                            {
                                setup.dispatch_source = RKNPU_DISPATCH_SOURCE_READY;
                                setup.ready_priority = core_ready_priority;
                                prepared = Some(setup);
                                break;
                            }
                        }
                    }

                    prepared
                }
            };

            let Some(mut setup) = setup else {
                break;
            };

            // 同一轮派发中，每个 Submit 只执行一次下发前 DMA 写同步。
            if confirmed_submit_ids.insert(setup.binding.task_id) {
                debug!(
                    "[rknpu-scheduler] confirm_write_all queue_task={}",
                    setup.binding.task_id
                );
                if let Err(err) = self.with_npu_driver(|rknpu_dev| rknpu_dev.comfirm_write_all()) {
                    warn!(
                        "[rknpu-scheduler] confirm_write_all failed for queue_task={}: {:?}",
                        setup.binding.task_id, err
                    );
                    // 先记录具体失败 Task，再解除核心绑定并唤醒可能终止的 Submit。
                    self.record_schedule_event(RknpuScheduleTraceRecord {
                        timestamp_ns: self.inner.platform.monotonic_time_ns(),
                        queue_task: setup.binding.task_id,
                        priority: setup.priority,
                        event_type: RKNPU_SCHEDULE_EVENT_FAILED,
                        op_idx: setup.op_idx,
                        task_index: setup.binding.task_index,
                        core_slot: setup.core_slot as u32,
                        lane_slot: setup.binding.lane_slot as u32,
                        dispatch_source: setup.dispatch_source,
                        ready_priority: setup.ready_priority,
                        ..RknpuScheduleTraceRecord::default()
                    });
                    self.fail_dispatch(setup.core_slot, setup.binding, err.to_driver_error());
                    continue;
                }
            }

            let submit_result = self.with_npu_driver(|rknpu_dev| {
                // t2：下一步直接进入NPU寄存器下发。
                let dispatch_time_ns = self.inner.platform.monotonic_time_ns();
                rknpu_dev.submit_ioctrl_step(
                    setup.core_slot,
                    setup.submit_flags,
                    setup.task_total,
                    setup.task_array_dma_address,
                    setup.binding.lane_slot,
                    setup.binding.task_index,
                    &mut setup.task,
                )?;
                Ok(dispatch_time_ns)
            });

            match submit_result {
                Ok(dispatch_time_ns) => {
                    // 设备锁已经释放；此处只在第一次成功下发时保存 Submit 的 t2。
                    {
                        let mut state = self.inner.scheduler.state.lock();
                        if let Some(task) = state.tasks.get_mut(&setup.binding.task_id) {
                            task.mark_first_dispatch(dispatch_time_ns);
                        }
                    }
                    self.record_schedule_event(RknpuScheduleTraceRecord {
                        timestamp_ns: dispatch_time_ns,
                        queue_task: setup.binding.task_id,
                        priority: setup.priority,
                        event_type: RKNPU_SCHEDULE_EVENT_DISPATCH,
                        op_idx: setup.op_idx,
                        task_index: setup.binding.task_index,
                        core_slot: setup.core_slot as u32,
                        lane_slot: setup.binding.lane_slot as u32,
                        dispatch_source: setup.dispatch_source,
                        ready_priority: setup.ready_priority,
                        ..RknpuScheduleTraceRecord::default()
                    });
                    debug!(
                        "[rknpu-scheduler] dispatched queue_task={} core={} lane={} task_index={}",
                        setup.binding.task_id,
                        setup.core_slot,
                        setup.binding.lane_slot,
                        setup.binding.task_index
                    );
                    dispatched = true;
                }
                Err(err) => {
                    warn!(
                        "[rknpu-scheduler] dispatch failed for queue_task={} core={} task={}: {:?}",
                        setup.binding.task_id, setup.core_slot, setup.binding.task_index, err
                    );
                    self.record_schedule_event(RknpuScheduleTraceRecord {
                        timestamp_ns: self.inner.platform.monotonic_time_ns(),
                        queue_task: setup.binding.task_id,
                        priority: setup.priority,
                        event_type: RKNPU_SCHEDULE_EVENT_FAILED,
                        op_idx: setup.op_idx,
                        task_index: setup.binding.task_index,
                        core_slot: setup.core_slot as u32,
                        lane_slot: setup.binding.lane_slot as u32,
                        dispatch_source: setup.dispatch_source,
                        ready_priority: setup.ready_priority,
                        ..RknpuScheduleTraceRecord::default()
                    });
                    self.fail_dispatch(setup.core_slot, setup.binding, err.to_driver_error());
                }
            }
        }

        dispatched
    }

    /// Drop a waiter after an interrupted blocking wait and prevent later copy-back.
    fn abort_wait(&self, task_id: RknpuQueueTaskId) {
        let mut state = self.inner.scheduler.state.lock();
        state.waiters.remove(&task_id);

        if state.complete.remove(&task_id).is_some() {
            return;
        }

        if let Some(task) = state.tasks.get_mut(&task_id) {
            task.last_error = Some(RknpuError::Interrupted);
        }

        if state.reclassify_task(task_id).is_some() {
            state.complete.remove(&task_id);
        }
    }

    /// 单 worker 调度循环：回收完成任务、派发 Ready 任务，再按三种状态等待或重试。
    ///
    /// 最后修改日期：2026-08-19。inflight completion 可选择 Event/Waker 或
    /// `yield_now()` 轮询；无 live work 时始终等待新 Submit，避免空转。
    fn worker_main(self) {
        debug!("[rknpu-scheduler] worker thread started");
        let mut stalled_yields = 0u32;
        loop {
            // Event/Waker 模式必须在检查 completion 前注册监听，避免 IRQ 落在
            // “检查状态”和“开始等待”之间。YieldPolling 模式不创建无用监听器。
            let listener = match self.inner.worker_wait_mode {
                RknpuWorkerWaitMode::IrqEvent => Some(self.inner.scheduler.kick.listen()),
                RknpuWorkerWaitMode::YieldPolling => None,
            };
            let harvested = self.harvest_completed_cores();
            let dispatched = self.dispatch_idle_cores();

            if harvested || dispatched {
                stalled_yields = 0;
                continue;
            }

            let (has_inflight, has_work) = {
                let state = self.inner.scheduler.state.lock();
                (state.has_inflight(), state.has_live_work())
            };
            if has_inflight {
                // 最后修改日期：2026-08-19。两种版本只在这个等待点分流，
                // completion 回收和后续 refill 仍走完全相同的调度流程。
                match self.inner.worker_wait_mode {
                    RknpuWorkerWaitMode::IrqEvent => {
                        debug!(
                            "[rknpu-scheduler] worker sleeping reason=inflight_completion \
                             has_inflight={} has_work={}",
                            has_inflight, has_work
                        );
                        // IrqEvent 模式在本轮入口必定已经建立 listener。
                        listener.unwrap().wait();
                    }
                    RknpuWorkerWaitMode::YieldPolling => {
                        // 轮询热路径不打印日志，避免日志级别判断影响基线性能。
                        self.inner.platform.yield_now();
                    }
                }
                continue;
            }

            if !has_work {
                // 当前没有 Submit，等待下一次 enqueue 通知。YieldPolling 模式
                // 此时才注册监听并复查状态，覆盖“先入队、后监听”的竞争窗口。
                let listener = match listener {
                    Some(listener) => listener,
                    None => {
                        let listener = self.inner.scheduler.kick.listen();
                        if self.inner.scheduler.state.lock().has_live_work() {
                            continue;
                        }
                        listener
                    }
                };
                debug!(
                    "[rknpu-scheduler] worker sleeping reason=idle_enqueue \
                     has_inflight={} has_work={}",
                    has_inflight, has_work
                );
                listener.wait();
                continue;
            }

            // 最后修改日期：2026-08-17。存在 live Submit、但既没有 inflight
            // Task 也没有成功派发时，不得等待 IRQ；此时没有任何事件保证会再次唤醒。
            stalled_yields = stalled_yields.saturating_add(1);
            if stalled_yields == 1 || stalled_yields % 64 == 0 {
                debug!(
                    "[rknpu-scheduler] worker retrying reason=scheduler_stalled \
                     has_inflight={} has_work={} yields={}",
                    has_inflight, has_work, stalled_yields
                );
            }
            self.inner.platform.yield_now();
        }
    }

    #[cfg(test)]
    /// Test-only probe for whether the mock worker has issued hardware work.
    pub(super) fn has_inflight_dispatches(&self) -> bool {
        !self.inner.scheduler.state.lock().core_binding.is_empty()
    }

}
