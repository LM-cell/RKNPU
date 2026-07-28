//! Queue-side data structures for the RKNPU blocking-submit scheduler.
//!
//! This module intentionally stops at the submit/lane state-machine boundary.
//! It knows how one submit tracks lane progress, how a finished submit rebuilds
//! its ABI-facing `RknpuSubmit`, and how lane-level failures affect future
//! dispatch decisions. It does not own ready/running/complete buckets or any
//! per-core binding state; that belongs to the StarryOS scheduler layer.

#![allow(dead_code)]

use crate::{
    RKNPU_MAX_SUBCORE_TASKS, RknpuError, RknpuTask, core_mask_from_index,
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
}

impl RknpuQueuedSubmit {
    /// Build the queue-owned submit payload from userspace submit data.
    pub fn new(submit: RknpuSubmit, tasks: Vec<RknpuTask>) -> Self {
        let task_total = tasks.len() as u32;
        Self {
            meta: SubmitMeta::from_submit(&submit, task_total),
            reply: SubmitReplyState::from_submit(&submit),
            tasks,
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
/// No explicit ready/running/completed enum is stored here. Those states are
/// derived from `lane_isrun`, remaining dispatchable work, and `last_error`.
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
    /// Last scheduler or driver error seen by this submit.
    pub last_error: Option<RknpuError>,
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
            last_error: None,
        }
    }

    /// Return how many task completions make this submit terminal on success.
    fn completion_target(&self) -> u32 {
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

    /// Return true if at least one logical lane currently owns a hardware core.
    pub fn has_running_lanes(&self) -> bool {
        self.lane_isrun.iter().copied().any(|running| running)
    }

    /// Return true if the submit can still dispatch at least one more lane.
    pub fn has_dispatchable_lane(&self) -> bool {
        self.next_dispatchable_lane().is_some()
    }

    /// Return true if the submit finished successfully.
    pub fn is_terminal_success(&self) -> bool {
        self.last_error.is_none() && self.completed_task_count() >= self.completion_target()
    }

    /// Return true if the submit faulted and all running lanes already drained.
    pub fn is_terminal_fault(&self) -> bool {
        self.last_error.is_some() && !self.has_running_lanes()
    }

    /// Return true if this submit may use the specified physical core.
    pub fn allows_core(&self, core_slot: usize) -> bool {
        let mask = self.meta.core_mask;
        mask == 0 || (mask & core_mask_from_index(core_slot)) != 0
    }

    /// Return the next lane/task pair that may be dispatched right now.
    ///
    /// Once `last_error` is set, no new work may be issued. Existing running
    /// lanes are still allowed to drain through normal completion.
    pub fn next_dispatchable_lane(&self) -> Option<(usize, u32)> {
        if self.last_error.is_some() {
            return None;
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

    /// Mark one lane as owning an in-flight hardware dispatch.
    pub fn mark_lane_running(&mut self, lane_slot: usize) {
        if let Some(running) = self.lane_isrun.get_mut(lane_slot) {
            *running = true;
        }
    }

    /// Mark one dispatched lane as completed and advance its cursor by one task.
    pub fn complete_lane(&mut self, lane_slot: usize) {
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
    pub fn fail_lane(&mut self, lane_slot: usize, err: RknpuError) {
        if let Some(running) = self.lane_isrun.get_mut(lane_slot) {
            *running = false;
        }
        self.last_error = Some(err);
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
                self.tasks
                    .get_mut(&dispatch.task_id)?
                    .mark_lane_running(dispatch.lane_slot as usize);
                return Some(dispatch);
            }

            let task_id = self.pop_ready_candidate(core_slot)?;
            let priority = self.tasks.get(&task_id)?.meta.priority;
            Self::push_bucket(&mut self.running, priority, task_id);

            let dispatch = self.find_running_candidate(core_slot)?;
            self.tasks
                .get_mut(&dispatch.task_id)?
                .mark_lane_running(dispatch.lane_slot as usize);
            Some(dispatch)
        }

        /// Finish one reserved dispatch and reclassify the owning task.
        fn complete_dispatch(
            &mut self,
            dispatch: Dispatch,
            task_error: bool,
        ) -> Option<RknpuQueueTaskId> {
            if let Some(task) = self.tasks.get_mut(&dispatch.task_id) {
                task.complete_lane(dispatch.lane_slot as usize);
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
                task.fail_lane(dispatch.lane_slot as usize, err);
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
                    if let Some((lane_slot, task_index)) = task.next_dispatchable_lane() {
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
                            task.allows_core(core_slot) && task.has_dispatchable_lane()
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
            let has_running = task.has_running_lanes();
            let has_dispatchable = task.has_dispatchable_lane();
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
        let running = RknpuQueuedSubmit::new(
            fake_submit(2, 10, &[(0, 0, 1), (1, 1, 1)]),
            vec![fake_task(0x100), fake_task(0x200)],
        );
        let ready =
            RknpuQueuedSubmit::new(fake_submit(1, -10, &[(0, 0, 1)]), vec![fake_task(0x300)]);

        let running_id = queue.enqueue(running);
        let ready_id = queue.enqueue(ready);

        assert_eq!(queue.reserve_next_dispatch(0).unwrap().task_id, running_id);
        assert_eq!(queue.reserve_next_dispatch(1).unwrap().task_id, running_id);
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
}
