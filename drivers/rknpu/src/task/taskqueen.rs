//! Queue-side data structures for the RKNPU blocking-submit scheduler.
//!
//! This module intentionally stops at the submit/lane state-machine boundary.
//! It knows how one submit tracks lane progress, how a finished submit rebuilds
//! its ABI-facing `RknpuSubmit`, and how lane-level failures affect future
//! dispatch decisions. It does not own ready/running/complete buckets or any
//! per-core binding state; that belongs to the StarryOS scheduler layer.

// Submit 延迟与 Dynamic Task 状态最后修改日期：2026-08-18。

#![allow(dead_code)]

use crate::{
    RKNPU_JOB_DYNAMIC_TASKS, RKNPU_JOB_HARDWARE_MASK, RKNPU_MAX_SUBCORE_TASKS,
    RKNPU_SUBMIT_ALLOWED_FLAGS, RKNPU_VALID_CORE_MASK, RknpuError, RknpuTask,
    core_mask_from_index,
    ioctrl::{RknpuSubcoreTask, RknpuSubmit},
};
use alloc::vec::Vec;

/// Unique identifier assigned to one queued blocking submit.
pub type RknpuQueueTaskId = u64;

/// Immutable scheduler-facing metadata derived from one submit ioctl.
///
/// `SubmitMeta` keeps only the fields that matter while the submit is live in
/// the scheduler. These values never change after enqueue:
///
/// - job-mode bits forwarded to the driver
/// - priority and core mask used for scheduling
/// - DMA base address used by the driver submit path
/// - normalized per-lane task ranges
/// - total task count
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SubmitMeta {
    /// Job-mode flags copied from `RknpuSubmit.flags`.
    pub flags: u32,
    /// Scheduler priority. Lower value means higher priority.
    pub priority: i32,
    /// Allowed hardware-core mask. Zero means "no restriction".
    pub core_mask: u32,
    /// DMA base of the task descriptor array as seen by the NPU.
    pub task_array_dma_address: u64,
    /// Normalized per-lane task layout.
    pub lane_ranges: [RknpuSubcoreTask; RKNPU_MAX_SUBCORE_TASKS],
    /// Total number of task descriptors in `tasks`.
    pub task_total: u32,
}

impl SubmitMeta {
    /// 校验动态模式的调度输入，最后修改日期：2026-08-18。
    ///
    /// 静态模式保持原兼容行为。动态模式要求 Lane 按下标连续、无重叠地覆盖
    /// 全部 Task，保证共享 Ready 集合不会遗漏或重复派发用户任务。
    pub fn dynamic_layout_is_valid(submit: &RknpuSubmit) -> bool {
        if submit.flags & RKNPU_JOB_DYNAMIC_TASKS == 0 {
            return true;
        }
        if submit.flags & !RKNPU_SUBMIT_ALLOWED_FLAGS != 0
            || submit.core_mask & !RKNPU_VALID_CORE_MASK != 0
            || submit.task_number == 0
        {
            return false;
        }

        let mut next_task = 0u32;
        for lane in submit.subcore_task {
            if lane.task_number == 0 {
                if lane.task_start != 0 {
                    return false;
                }
                continue;
            }
            if lane.task_start != next_task {
                return false;
            }
            let Some(end) = lane.task_start.checked_add(lane.task_number) else {
                return false;
            };
            if end > submit.task_number {
                return false;
            }
            next_task = end;
        }
        next_task == submit.task_number
    }

    /// Build immutable scheduler metadata from one ioctl submit.
    ///
    /// If userspace leaves every `subcore_task[]` entry empty, lane 0 is
    /// normalized to cover the whole task array so the scheduler always has a
    /// concrete single-lane layout to work with.
    pub fn from_submit(submit: &RknpuSubmit, task_total: u32) -> Self {
        let mut lane_ranges = submit.subcore_task;
        let all_empty = lane_ranges.iter().all(|lane| lane.task_number == 0);
        if all_empty && task_total > 0 {
            lane_ranges[0] = RknpuSubcoreTask {
                task_start: 0,
                task_number: task_total,
            };
        }

        Self {
            flags: submit.flags,
            priority: submit.priority,
            core_mask: submit.core_mask,
            task_array_dma_address: submit.task_array_dma_address,
            lane_ranges,
            task_total,
        }
    }

    /// Return the normalized task range owned by one logical lane.
    pub fn lane_range(&self, lane_slot: usize) -> Option<RknpuSubcoreTask> {
        self.lane_ranges
            .get(lane_slot)
            .copied()
            .filter(|lane| lane.task_number > 0)
    }

    /// 动态模式只改变 Task 选择，不改变 Submit 的阻塞完成语义。
    pub fn uses_dynamic_tasks(&self) -> bool {
        self.flags & RKNPU_JOB_DYNAMIC_TASKS != 0
    }

    /// 剥离调度器专用标志，只把原有硬件位传入寄存器执行路径。
    pub fn hardware_flags(&self) -> u32 {
        self.flags & RKNPU_JOB_HARDWARE_MASK
    }
}

/// Submit fields that must survive until terminal copy-back, but are not used
/// by the live scheduler state machine.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct SubmitReplyState {
    /// Original submit timeout copied back at the terminal boundary.
    timeout: u32,
    /// Original legacy task start copied back at the terminal boundary.
    task_start: u32,
    /// Original IOMMU domain id copied back at the terminal boundary.
    iommu_domain_id: u32,
    /// Reserved ABI field copied back at the terminal boundary.
    reserved: u32,
    /// Fence fd copied back at the terminal boundary.
    fence_fd: i32,
}

impl SubmitReplyState {
    /// Snapshot the reply-only fields from one ioctl submit.
    fn from_submit(submit: &RknpuSubmit) -> Self {
        Self {
            timeout: submit.timeout,
            task_start: submit.task_start,
            iommu_domain_id: submit.iommu_domain_id,
            reserved: submit.reserved,
            fence_fd: submit.fence_fd,
        }
    }
}

/// Driver-side timestamps for the four coarse submit latency phases.
#[derive(Debug, Clone, Copy, Default)]
pub(crate) struct SubmitLatencyTrace {
    /// Submit ioctl entered the driver.
    pub t0_ns: u64,
    /// Submit was inserted into the scheduler queue.
    pub t1_ns: u64,
    /// The first NPU-core dispatch started.
    pub t2_ns: u64,
    /// The scheduler observed the final NPU-core completion.
    pub t3_ns: u64,
}

/// Queue-owned submit payload created at the ioctl boundary.
///
/// This is the hand-off object from `card1` to the scheduler. It splits the
/// ABI-facing `RknpuSubmit` into:
///
/// - immutable scheduling metadata
/// - reply-only fields used only at terminal copy-back
/// - the mutable `RknpuTask[]` shadow owned by the kernel
#[derive(Debug, Clone)]
pub struct RknpuQueuedSubmit {
    /// Immutable scheduling metadata.
    pub meta: SubmitMeta,
    /// Reply-only submit fields kept until terminal copy-back.
    reply: SubmitReplyState,
    /// Kernel-owned task shadow that the scheduler updates in place.
    pub tasks: Vec<RknpuTask>,
    /// Timestamps carried from ioctl entry into the live scheduler task.
    pub(crate) latency: SubmitLatencyTrace,
}

impl RknpuQueuedSubmit {
    /// Build the queue-owned submit payload from userspace submit data.
    pub fn new(submit: RknpuSubmit, tasks: Vec<RknpuTask>) -> Self {
        let task_total = tasks.len() as u32;
        Self {
            meta: SubmitMeta::from_submit(&submit, task_total),
            reply: SubmitReplyState::from_submit(&submit),
            tasks,
            latency: SubmitLatencyTrace::default(),
        }
    }
}

/// Live progress of one queued blocking submit.
///
/// The scheduler owns one `RknpuQueueTask` per live submit. It records:
///
/// - immutable scheduling metadata and reply-only fields
/// - the kernel-owned `RknpuTask[]` shadow
/// - one cursor per lane indicating how many tasks that lane already consumed
/// - one running bit per lane so the same lane cannot be dispatched twice
/// - the last terminal error, if any
///
/// 静态模式由 Lane 游标和运行位表示进度；动态模式只保存下一个 Task、运行数
/// 和完成数。Ready/Running/Complete Submit 桶仍由上层调度器统一维护。
#[derive(Debug, Clone)]
pub struct RknpuQueueTask {
    /// Stable scheduler-visible submit id.
    pub id: RknpuQueueTaskId,
    /// Immutable scheduling metadata.
    pub meta: SubmitMeta,
    /// Reply-only fields copied back into `RknpuSubmit` at terminal time.
    reply: SubmitReplyState,
    /// Mutable task shadow updated by the scheduler harvest path.
    pub tasks: Vec<RknpuTask>,
    /// Per-lane progress cursor. Each value counts completed tasks in that lane.
    pub subcore_cursors: [u32; RKNPU_MAX_SUBCORE_TASKS],
    /// Per-lane running bit. `true` means that lane already owns one in-flight
    /// dispatch on some hardware core.
    pub lane_isrun: [bool; RKNPU_MAX_SUBCORE_TASKS],
    /// 动态模式下，下一个尚未预留的 Task 下标。
    dynamic_next_task: u32,
    /// 动态模式下，已经绑定物理核心但尚未完成的 Task 数量。
    dynamic_running_tasks: u32,
    /// 动态模式下，已经由完成路径确认的 Task 数量。
    dynamic_completed_tasks: u32,
    /// Last scheduler or driver error seen by this submit.
    pub last_error: Option<RknpuError>,
    /// Coarse submit phase timestamps collected without changing scheduling.
    pub(crate) latency: SubmitLatencyTrace,
}

impl RknpuQueueTask {
    /// Materialize one live submit from the queue-owned boundary payload.
    pub fn new(id: RknpuQueueTaskId, queued_submit: RknpuQueuedSubmit) -> Self {
        Self {
            id,
            meta: queued_submit.meta,
            reply: queued_submit.reply,
            tasks: queued_submit.tasks,
            subcore_cursors: [0; RKNPU_MAX_SUBCORE_TASKS],
            lane_isrun: [false; RKNPU_MAX_SUBCORE_TASKS],
            dynamic_next_task: 0,
            dynamic_running_tasks: 0,
            dynamic_completed_tasks: 0,
            last_error: None,
            latency: queued_submit.latency,
        }
    }

    /// Record when this submit entered the ready queue.
    pub(crate) fn mark_enqueued(&mut self, time_ns: u64) {
        self.latency.t1_ns = time_ns;
    }

    /// Record only the first core dispatch for this submit.
    pub(crate) fn mark_first_dispatch(&mut self, time_ns: u64) {
        if self.latency.t2_ns == 0 {
            self.latency.t2_ns = time_ns;
        }
    }

    /// Record when the scheduler observed terminal hardware completion.
    pub(crate) fn mark_terminal(&mut self, time_ns: u64) {
        self.latency.t3_ns = time_ns;
    }

    /// Return how many task completions make this submit terminal on success.
    fn completion_target(&self) -> u32 {
        if self.meta.uses_dynamic_tasks() {
            return self.meta.task_total;
        }
        let total = self
            .meta
            .lane_ranges
            .iter()
            .map(|lane| lane.task_number)
            .sum::<u32>();
        if total == 0 {
            self.meta.task_total
        } else {
            total.min(self.meta.task_total)
        }
    }

    /// Return how many task completions already happened across all lanes.
    pub fn completed_task_count(&self) -> u32 {
        if self.meta.uses_dynamic_tasks() {
            return self.dynamic_completed_tasks.min(self.meta.task_total);
        }
        self.subcore_cursors
            .iter()
            .copied()
            .sum::<u32>()
            .min(self.completion_target())
    }

    /// Rebuild the ABI-facing submit header at the terminal boundary.
    pub fn build_submit(&self) -> RknpuSubmit {
        let mut submit = RknpuSubmit::default();
        submit.flags = self.meta.flags;
        submit.timeout = self.reply.timeout;
        submit.task_start = self.reply.task_start;
        submit.task_number = self.meta.task_total;
        submit.task_counter = self.completed_task_count();
        submit.priority = self.meta.priority;
        submit.task_array_cpu_address = 0;
        submit.iommu_domain_id = self.reply.iommu_domain_id;
        submit.reserved = self.reply.reserved;
        submit.task_array_dma_address = self.meta.task_array_dma_address;
        submit.hw_elapse_time = if self.last_error.is_some() {
            -1
        } else if self.is_terminal_success() {
            self.completed_task_count() as i64
        } else {
            0
        };
        submit.core_mask = self.meta.core_mask;
        submit.fence_fd = self.reply.fence_fd;
        submit.subcore_task = self.meta.lane_ranges;
        submit
    }

    /// 返回是否至少有一个 Task 已经绑定物理核心且尚未完成。
    pub fn has_running_tasks(&self) -> bool {
        if self.meta.uses_dynamic_tasks() {
            self.dynamic_running_tasks != 0
        } else {
            self.lane_isrun.iter().copied().any(|running| running)
        }
    }

    /// 返回该 Submit 是否还有可以立即派发的 Task。
    pub fn has_dispatchable_task(&self) -> bool {
        self.next_dispatchable_task().is_some()
    }

    /// Return true if the submit finished successfully.
    pub fn is_terminal_success(&self) -> bool {
        self.last_error.is_none() && self.completed_task_count() >= self.completion_target()
    }

    /// Return true if the submit faulted and all running lanes already drained.
    pub fn is_terminal_fault(&self) -> bool {
        self.last_error.is_some() && !self.has_running_tasks()
    }

    /// Return true if this submit may use the specified physical core.
    pub fn allows_core(&self, core_slot: usize) -> bool {
        let mask = self.meta.core_mask;
        mask == 0 || (mask & core_mask_from_index(core_slot)) != 0
    }

    /// 返回当前可派发的“来源 Lane、Task 下标”。
    ///
    /// 出错后停止领取新 Task，已经运行的 Task 仍通过正常完成路径排空。
    pub fn next_dispatchable_task(&self) -> Option<(usize, u32)> {
        if self.last_error.is_some() {
            return None;
        }

        if self.meta.uses_dynamic_tasks() {
            let task_index = self.dynamic_next_task;
            if task_index >= self.meta.task_total || task_index as usize >= self.tasks.len() {
                return None;
            }
            return self.origin_lane_for_task(task_index).map(|lane| (lane, task_index));
        }

        for lane_slot in 0..RKNPU_MAX_SUBCORE_TASKS {
            if self.lane_isrun[lane_slot] {
                continue;
            }

            let Some(range) = self.meta.lane_range(lane_slot) else {
                continue;
            };
            let cursor = self.subcore_cursors[lane_slot];
            if cursor >= range.task_number {
                continue;
            }

            let task_index = range.task_start.saturating_add(cursor);
            if task_index >= self.meta.task_total || task_index as usize >= self.tasks.len() {
                continue;
            }
            return Some((lane_slot, task_index));
        }

        None
    }

    /// 预留一个 Task 并记录其核心绑定，最后修改日期：2026-08-18。
    ///
    /// 调用方必须传入刚从 `next_dispatchable_task()` 得到的二元组。本函数再次
    /// 比较期望值，防止调度器状态变化后重复预留同一个 Task。
    pub fn reserve_dispatch(&mut self, lane_slot: usize, task_index: u32) -> bool {
        if self.next_dispatchable_task() != Some((lane_slot, task_index)) {
            return false;
        }

        if self.meta.uses_dynamic_tasks() {
            self.dynamic_next_task += 1;
            self.dynamic_running_tasks += 1;
            true
        } else if let Some(running) = self.lane_isrun.get_mut(lane_slot) {
            *running = true;
            true
        } else {
            false
        }
    }

    /// 完成一个已绑定 Task；动态模式按 Task 计数，静态模式继续推进原 Lane。
    pub fn complete_dispatch(&mut self, lane_slot: usize) {
        if self.meta.uses_dynamic_tasks() {
            self.dynamic_running_tasks = self.dynamic_running_tasks.saturating_sub(1);
            self.dynamic_completed_tasks = self
                .dynamic_completed_tasks
                .saturating_add(1)
                .min(self.meta.task_total);
            return;
        }

        if let Some(running) = self.lane_isrun.get_mut(lane_slot) {
            *running = false;
        }

        let Some(range) = self.meta.lane_range(lane_slot) else {
            return;
        };
        let Some(cursor) = self.subcore_cursors.get_mut(lane_slot) else {
            return;
        };
        *cursor = cursor.saturating_add(1).min(range.task_number);
    }

    /// Mark one lane as failed before completion and record the error.
    ///
    /// This path is used for dispatch/setup failures. The lane's cursor is not
    /// advanced because the task never completed on hardware.
    pub fn fail_dispatch(&mut self, lane_slot: usize, err: RknpuError) {
        if self.meta.uses_dynamic_tasks() {
            self.dynamic_running_tasks = self.dynamic_running_tasks.saturating_sub(1);
            self.last_error = Some(err);
            return;
        }

        if let Some(running) = self.lane_isrun.get_mut(lane_slot) {
            *running = false;
        }
        self.last_error = Some(err);
    }

    /// 返回 Task 原本所属的逻辑 Lane，仅用于 trace 和静态/动态对照。
    fn origin_lane_for_task(&self, task_index: u32) -> Option<usize> {
        self.meta.lane_ranges.iter().enumerate().find_map(|(lane_slot, lane)| {
            let end = lane.task_start.checked_add(lane.task_number)?;
            (lane.task_number > 0 && task_index >= lane.task_start && task_index < end)
                .then_some(lane_slot)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::{
        collections::{BTreeMap, VecDeque},
        vec,
        vec::Vec,
    };

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    struct Dispatch {
        task_id: RknpuQueueTaskId,
        lane_slot: u8,
        task_index: u32,
    }

    #[derive(Default)]
    struct TestBuckets {
        next_task_id: RknpuQueueTaskId,
        tasks: BTreeMap<RknpuQueueTaskId, RknpuQueueTask>,
        ready: BTreeMap<i32, VecDeque<RknpuQueueTaskId>>,
        running: BTreeMap<i32, VecDeque<RknpuQueueTaskId>>,
        complete: BTreeMap<RknpuQueueTaskId, RknpuQueueTask>,
    }

    impl TestBuckets {
        /// Create empty test buckets with deterministic ids.
        fn new() -> Self {
            Self {
                next_task_id: 1,
                ..Self::default()
            }
        }

        /// Insert one queued submit into the test ready queue.
        fn enqueue(&mut self, queued_submit: RknpuQueuedSubmit) -> RknpuQueueTaskId {
            let task_id = self.next_task_id;
            self.next_task_id = self.next_task_id.saturating_add(1);

            let task = RknpuQueueTask::new(task_id, queued_submit);
            let priority = task.meta.priority;
            self.tasks.insert(task_id, task);
            Self::push_bucket(&mut self.ready, priority, task_id);
            task_id
        }

        /// Return true if the task has reached the test completion bucket.
        fn is_terminal(&self, task_id: RknpuQueueTaskId) -> bool {
            self.complete.contains_key(&task_id)
        }

        /// Remove one finished task from the test completion bucket.
        fn take_terminal_task(&mut self, task_id: RknpuQueueTaskId) -> Option<RknpuQueueTask> {
            self.complete.remove(&task_id)
        }

        /// Reserve the next dispatch using the same running-before-ready policy.
        fn reserve_next_dispatch(&mut self, core_slot: usize) -> Option<Dispatch> {
            if let Some(dispatch) = self.find_running_candidate(core_slot) {
                let reserved = self.tasks
                    .get_mut(&dispatch.task_id)?
                    .reserve_dispatch(dispatch.lane_slot as usize, dispatch.task_index);
                if !reserved {
                    return None;
                }
                return Some(dispatch);
            }

            let task_id = self.pop_ready_candidate(core_slot)?;
            let priority = self.tasks.get(&task_id)?.meta.priority;
            Self::push_bucket(&mut self.running, priority, task_id);

            let dispatch = self.find_running_candidate(core_slot)?;
            let reserved = self.tasks
                .get_mut(&dispatch.task_id)?
                .reserve_dispatch(dispatch.lane_slot as usize, dispatch.task_index);
            if !reserved {
                return None;
            }
            Some(dispatch)
        }

        /// Finish one reserved dispatch and reclassify the owning task.
        fn complete_dispatch(
            &mut self,
            dispatch: Dispatch,
            task_error: bool,
        ) -> Option<RknpuQueueTaskId> {
            if let Some(task) = self.tasks.get_mut(&dispatch.task_id) {
                task.complete_dispatch(dispatch.lane_slot as usize);
                if task_error {
                    task.last_error = Some(RknpuError::TaskError);
                }
            }

            self.reclassify(dispatch.task_id)
        }

        /// Fail one reserved dispatch and reclassify the owning task.
        fn fail_dispatch(
            &mut self,
            dispatch: Dispatch,
            err: RknpuError,
        ) -> Option<RknpuQueueTaskId> {
            if let Some(task) = self.tasks.get_mut(&dispatch.task_id) {
                task.fail_dispatch(dispatch.lane_slot as usize, err);
            }

            self.reclassify(dispatch.task_id)
        }

        /// Find work from already-running submits that can use this core.
        fn find_running_candidate(&self, core_slot: usize) -> Option<Dispatch> {
            for queue in self.running.values() {
                for task_id in queue {
                    let Some(task) = self.tasks.get(task_id) else {
                        continue;
                    };
                    if !task.allows_core(core_slot) {
                        continue;
                    }
                    if let Some((lane_slot, task_index)) = task.next_dispatchable_task() {
                        return Some(Dispatch {
                            task_id: *task_id,
                            lane_slot: lane_slot as u8,
                            task_index,
                        });
                    }
                }
            }

            None
        }

        /// Pop the highest-priority ready submit that can run on this core.
        fn pop_ready_candidate(&mut self, core_slot: usize) -> Option<RknpuQueueTaskId> {
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

        /// Move a task to ready, running, or complete after a dispatch event.
        fn reclassify(&mut self, task_id: RknpuQueueTaskId) -> Option<RknpuQueueTaskId> {
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

        /// Insert a task id into a priority bucket once.
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

        /// Remove a task id from every bucket and clean up empty queues.
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
    }

    /// Build a submit descriptor with explicit per-lane ranges.
    fn fake_submit(
        task_number: u32,
        priority: i32,
        subcore_tasks: &[(usize, u32, u32)],
    ) -> RknpuSubmit {
        let mut submit = RknpuSubmit::default();
        submit.timeout = 1000;
        submit.priority = priority;
        submit.task_number = task_number;
        for (slot, start, number) in subcore_tasks {
            submit.subcore_task[*slot].task_start = *start;
            submit.subcore_task[*slot].task_number = *number;
        }
        submit
    }

    /// Build a submit descriptor that is restricted to a physical-core mask.
    fn fake_submit_with_core_mask(
        task_number: u32,
        priority: i32,
        core_mask: u32,
        subcore_tasks: &[(usize, u32, u32)],
    ) -> RknpuSubmit {
        let mut submit = fake_submit(task_number, priority, subcore_tasks);
        submit.core_mask = core_mask;
        submit
    }

    /// 构造显式启用动态 Task 领取的 Submit。
    fn fake_dynamic_submit(task_number: u32, ranges: &[(usize, u32, u32)]) -> RknpuSubmit {
        let mut submit = fake_submit(task_number, 0, ranges);
        submit.flags = crate::RKNPU_JOB_PC | RKNPU_JOB_DYNAMIC_TASKS;
        submit.core_mask = RKNPU_VALID_CORE_MASK;
        submit
    }

    /// Build a fake task descriptor with the expected interrupt mask set.
    fn fake_task(int_mask: u32) -> RknpuTask {
        RknpuTask {
            int_mask,
            ..RknpuTask::default()
        }
    }

    #[test]
    fn queued_submit_builds_submit_meta() {
        let tasks = vec![RknpuTask::default(), RknpuTask::default()];
        let queued = RknpuQueuedSubmit::new(fake_submit(2, 0, &[(0, 0, 2)]), tasks);

        assert_eq!(queued.meta.task_total, 2);
        assert_eq!(queued.meta.task_array_dma_address, 0);
        assert_eq!(queued.meta.lane_range(0).unwrap().task_number, 2);
    }

    #[test]
    fn empty_subcore_layout_defaults_to_slot_zero_range() {
        let mut queue = TestBuckets::new();
        let submit = RknpuQueuedSubmit::new(
            fake_submit(2, 0, &[]),
            vec![fake_task(0x100), fake_task(0x200)],
        );

        queue.enqueue(submit);
        let first = queue.reserve_next_dispatch(0).unwrap();

        assert_eq!(first.lane_slot, 0);
        assert_eq!(first.task_index, 0);
    }

    #[test]
    fn lower_priority_value_runs_first() {
        let mut queue = TestBuckets::new();

        let low = RknpuQueuedSubmit::new(fake_submit(1, 10, &[(0, 0, 1)]), vec![fake_task(0x100)]);
        let high = RknpuQueuedSubmit::new(fake_submit(1, -5, &[(0, 0, 1)]), vec![fake_task(0x200)]);

        let low_id = queue.enqueue(low);
        let high_id = queue.enqueue(high);

        assert_eq!(queue.reserve_next_dispatch(0).unwrap().task_id, high_id);
        assert_eq!(queue.reserve_next_dispatch(1).unwrap().task_id, low_id);
    }

    #[test]
    fn same_submit_can_fill_multiple_idle_cores() {
        let mut queue = TestBuckets::new();
        let submit = RknpuQueuedSubmit::new(
            fake_submit(2, 0, &[(0, 0, 1), (1, 1, 1)]),
            vec![fake_task(0x100), fake_task(0x200)],
        );

        let task_id = queue.enqueue(submit);
        let first = queue.reserve_next_dispatch(0).unwrap();
        let second = queue.reserve_next_dispatch(1).unwrap();

        assert_eq!(first.task_id, task_id);
        assert_eq!(second.task_id, task_id);
        assert_ne!(first.lane_slot, second.lane_slot);
    }

    #[test]
    fn core_mask_binds_submit_to_matching_physical_core() {
        let mut queue = TestBuckets::new();
        let submit = RknpuQueuedSubmit::new(
            fake_submit_with_core_mask(1, 0, 0x2, &[(0, 0, 1)]),
            vec![fake_task(0x100)],
        );

        queue.enqueue(submit);

        assert!(queue.reserve_next_dispatch(0).is_none());
        let dispatch = queue.reserve_next_dispatch(1).unwrap();
        assert_eq!(dispatch.task_index, 0);
    }

    #[test]
    fn running_submit_is_prioritized_before_new_ready_submit() {
        let mut queue = TestBuckets::new();

        // 旧 Submit：2 个 Task，2 条 Lane
        let running = RknpuQueuedSubmit::new(
            fake_submit(2, 10, &[(0, 0, 1), (1, 1, 1)]),
            vec![fake_task(0x100), fake_task(0x200)],
        );

        // 新 Submit：优先级更高，但当前还是 Ready
        let ready = RknpuQueuedSubmit::new(
            fake_submit(1, -10, &[(0, 0, 1)]),
            vec![fake_task(0x300)],
        );

        // 1. 旧 Submit 先进入 Ready 队列
        let running_id = queue.enqueue(running);

        // 2. 向 core0 派发第一条 Lane
        //    此时旧 Submit 从 Ready → Running
        assert_eq!(
            queue.reserve_next_dispatch(0).unwrap().task_id,
            running_id
        );

        // 3. 此时再加入新的高优先级 Ready Submit
        let ready_id = queue.enqueue(ready);

        // 4. core1 请求任务
        //    按当前策略：Running Submit 优先于新的 Ready Submit
        assert_eq!(
            queue.reserve_next_dispatch(1).unwrap().task_id,
            running_id
        );

        // 两个 Submit 必须是不同对象
        assert_ne!(running_id, ready_id);
    }

    #[test]
    fn same_lane_is_not_dispatched_twice_concurrently() {
        let mut queue = TestBuckets::new();
        let submit = RknpuQueuedSubmit::new(
            fake_submit(2, 0, &[(0, 0, 2)]),
            vec![fake_task(0x100), fake_task(0x100)],
        );

        queue.enqueue(submit);
        let first = queue.reserve_next_dispatch(0).unwrap();

        assert_eq!(first.lane_slot, 0);
        assert!(queue.reserve_next_dispatch(1).is_none());
    }

    #[test]
    fn dynamic_mode_allows_multiple_cores_to_take_one_origin_lane() {
        let mut queue = TestBuckets::new();
        let submit = RknpuQueuedSubmit::new(
            fake_dynamic_submit(4, &[(0, 0, 4)]),
            vec![fake_task(0x100); 4],
        );

        queue.enqueue(submit);
        let first = queue.reserve_next_dispatch(0).unwrap();
        let second = queue.reserve_next_dispatch(1).unwrap();
        let third = queue.reserve_next_dispatch(2).unwrap();

        assert_eq!((first.lane_slot, first.task_index), (0, 0));
        assert_eq!((second.lane_slot, second.task_index), (0, 1));
        assert_eq!((third.lane_slot, third.task_index), (0, 2));
    }

    #[test]
    fn dynamic_mode_completes_out_of_order_without_duplicate_tasks() {
        let mut queue = TestBuckets::new();
        let submit = RknpuQueuedSubmit::new(
            fake_dynamic_submit(4, &[(0, 0, 2), (1, 2, 2)]),
            vec![fake_task(0x100); 4],
        );
        let task_id = queue.enqueue(submit);
        let first = queue.reserve_next_dispatch(0).unwrap();
        let second = queue.reserve_next_dispatch(1).unwrap();
        let third = queue.reserve_next_dispatch(2).unwrap();

        assert!(queue.complete_dispatch(second, false).is_none());
        let fourth = queue.reserve_next_dispatch(1).unwrap();
        assert_eq!(fourth.task_index, 3);
        assert_eq!(fourth.lane_slot, 1);
        assert!(queue.complete_dispatch(first, false).is_none());
        assert!(queue.complete_dispatch(fourth, false).is_none());
        assert_eq!(queue.complete_dispatch(third, false), Some(task_id));

        let finished = queue.take_terminal_task(task_id).unwrap();
        assert_eq!(finished.build_submit().task_counter, 4);
        assert!(finished.is_terminal_success());
    }

    #[test]
    fn dynamic_mode_dispatches_all_48_tasks_once() {
        let mut queue = TestBuckets::new();
        let submit = RknpuQueuedSubmit::new(
            fake_dynamic_submit(48, &[(0, 0, 16), (1, 16, 16), (2, 32, 16)]),
            vec![fake_task(0x100); 48],
        );
        let task_id = queue.enqueue(submit);
        let mut active = [
            queue.reserve_next_dispatch(0),
            queue.reserve_next_dispatch(1),
            queue.reserve_next_dispatch(2),
        ];
        let mut completed = [false; 48];
        let mut terminal = None;

        // Core0 模拟较快核心：每轮先完成并立即领取；随后再处理 Core1、Core2。
        while active.iter().any(Option::is_some) {
            for core in [0usize, 0, 1, 0, 2] {
                let Some(dispatch) = active[core].take() else {
                    continue;
                };
                assert!(!completed[dispatch.task_index as usize]);
                completed[dispatch.task_index as usize] = true;
                terminal = queue.complete_dispatch(dispatch, false).or(terminal);
                active[core] = queue.reserve_next_dispatch(core);
            }
        }

        assert!(completed.iter().all(|done| *done));
        assert_eq!(terminal, Some(task_id));
        assert_eq!(
            queue
                .take_terminal_task(task_id)
                .unwrap()
                .build_submit()
                .task_counter,
            48
        );
    }

    #[test]
    fn ready_submit_fills_tail_when_dynamic_running_submit_has_no_ready_task() {
        let mut queue = TestBuckets::new();
        let running = RknpuQueuedSubmit::new(
            fake_dynamic_submit(2, &[(0, 0, 1), (1, 1, 1)]),
            vec![fake_task(0x100); 2],
        );
        let ready = RknpuQueuedSubmit::new(
            fake_submit(1, -10, &[(0, 0, 1)]),
            vec![fake_task(0x200)],
        );
        let running_id = queue.enqueue(running);

        assert_eq!(queue.reserve_next_dispatch(0).unwrap().task_id, running_id);
        assert_eq!(queue.reserve_next_dispatch(1).unwrap().task_id, running_id);
        let ready_id = queue.enqueue(ready);

        assert_eq!(queue.reserve_next_dispatch(2).unwrap().task_id, ready_id);
    }

    #[test]
    fn dynamic_layout_rejects_gaps_overlap_and_unknown_bits() {
        let valid = fake_dynamic_submit(4, &[(0, 0, 2), (1, 2, 2)]);
        let gap = fake_dynamic_submit(4, &[(0, 0, 1), (1, 2, 2)]);
        let overlap = fake_dynamic_submit(4, &[(0, 0, 3), (1, 2, 2)]);
        let mut unknown_flag = valid.clone();
        unknown_flag.flags |= 1 << 12;
        let mut invalid_core = valid.clone();
        invalid_core.core_mask = 1 << 8;

        assert!(SubmitMeta::dynamic_layout_is_valid(&valid));
        assert!(!SubmitMeta::dynamic_layout_is_valid(&gap));
        assert!(!SubmitMeta::dynamic_layout_is_valid(&overlap));
        assert!(!SubmitMeta::dynamic_layout_is_valid(&unknown_flag));
        assert!(!SubmitMeta::dynamic_layout_is_valid(&invalid_core));
        assert_eq!(
            SubmitMeta::from_submit(&valid, 4).hardware_flags(),
            crate::RKNPU_JOB_PC
        );
    }

    #[test]
    fn static_mode_keeps_legacy_partial_lane_layout() {
        let submit = fake_submit(4, 0, &[(0, 1, 2)]);

        assert!(SubmitMeta::dynamic_layout_is_valid(&submit));
    }

    #[test]
    fn completion_requeues_followup_work_and_terminal_task_can_be_taken() {
        let mut queue = TestBuckets::new();
        let submit = RknpuQueuedSubmit::new(
            fake_submit(2, 0, &[(0, 0, 2)]),
            vec![fake_task(0x100), fake_task(0x100)],
        );

        let task_id = queue.enqueue(submit);
        let first = queue.reserve_next_dispatch(0).unwrap();

        assert!(queue.complete_dispatch(first, false).is_none());

        let second = queue.reserve_next_dispatch(0).unwrap();
        let terminal = queue.complete_dispatch(second, false);

        assert_eq!(terminal, Some(task_id));
        let finished = queue.take_terminal_task(task_id).unwrap();
        assert!(finished.is_terminal_success());
        assert_eq!(finished.build_submit().task_counter, 2);
    }

    #[test]
    fn faulted_submit_waits_for_other_running_lanes_before_becoming_terminal() {
        let mut queue = TestBuckets::new();
        let submit = RknpuQueuedSubmit::new(
            fake_submit(2, 0, &[(0, 0, 1), (1, 1, 1)]),
            vec![fake_task(0x100), fake_task(0x200)],
        );

        let task_id = queue.enqueue(submit);
        let first = queue.reserve_next_dispatch(0).unwrap();
        let second = queue.reserve_next_dispatch(1).unwrap();

        assert!(
            queue
                .fail_dispatch(first, RknpuError::InternalError)
                .is_none()
        );
        assert!(!queue.is_terminal(task_id));

        let terminal = queue.complete_dispatch(second, false);

        assert_eq!(terminal, Some(task_id));
        let finished = queue.take_terminal_task(task_id).unwrap();
        assert!(finished.is_terminal_fault());
        assert_eq!(finished.last_error, Some(RknpuError::InternalError));
    }

    #[test]
    fn completed_submit_preserves_reply_fields() {
        let mut submit = fake_submit(2, -3, &[(0, 0, 2)]);
        submit.flags = 0x55aa;
        submit.timeout = 77;
        submit.task_start = 9;
        submit.iommu_domain_id = 11;
        submit.reserved = 12;
        submit.task_array_dma_address = 0x8800;
        submit.core_mask = 0x7;
        submit.fence_fd = 13;

        let queued = RknpuQueuedSubmit::new(submit, vec![fake_task(0x100), fake_task(0x200)]);
        let task = RknpuQueueTask::new(1, queued);
        let rebuilt = task.build_submit();

        assert_eq!(rebuilt.flags, 0x55aa);
        assert_eq!(rebuilt.timeout, 77);
        assert_eq!(rebuilt.task_start, 9);
        assert_eq!(rebuilt.priority, -3);
        assert_eq!(rebuilt.iommu_domain_id, 11);
        assert_eq!(rebuilt.reserved, 12);
        assert_eq!(rebuilt.task_array_dma_address, 0x8800);
        assert_eq!(rebuilt.core_mask, 0x7);
        assert_eq!(rebuilt.fence_fd, 13);
        assert_eq!(rebuilt.task_array_cpu_address, 0);
    }

    #[test]
    fn partial_lane_layout_keeps_original_completion_target() {
        let queued = RknpuQueuedSubmit::new(
            fake_submit(4, 0, &[(0, 1, 2)]),
            vec![
                fake_task(0x100),
                fake_task(0x100),
                fake_task(0x100),
                fake_task(0x100),
            ],
        );
        let task = RknpuQueueTask::new(1, queued);

        assert_eq!(task.completion_target(), 2);
    }

    #[test]
    fn submit_latency_keeps_the_first_core_dispatch_time() {
        let mut queued = RknpuQueuedSubmit::new(
            fake_submit(1, 0, &[(0, 0, 1)]),
            vec![fake_task(0x100)],
        );
        queued.latency.t0_ns = 10;

        let mut task = RknpuQueueTask::new(1, queued);
        task.mark_enqueued(20);
        task.mark_first_dispatch(30);
        // Keep t2 from the first core dispatch.
        task.mark_first_dispatch(35);
        task.mark_terminal(40);

        assert_eq!(task.latency.t0_ns, 10);
        assert_eq!(task.latency.t1_ns, 20);
        assert_eq!(task.latency.t2_ns, 30);
        assert_eq!(task.latency.t3_ns, 40);
    }
}
