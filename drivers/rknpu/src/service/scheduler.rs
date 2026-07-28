use alloc::{
    collections::{BTreeMap, BTreeSet, VecDeque},
    sync::Arc,
    vec::Vec,
};
use core::{
    ptr::addr_of_mut,
    sync::atomic::{AtomicU64, Ordering},
};

use spin::Mutex;

use crate::{
    NPU_MAX_CORES, RknpuError, RknpuQueueTask, RknpuQueueTaskId, RknpuQueuedSubmit, RknpuTask,
    ioctrl::RknpuSubmit,
};

use super::{
    RknpuPlatform, RknpuService, RknpuServiceError, RknpuSubmitWaiter, RknpuWorkerListener,
    RknpuWorkerSignal,
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
                if let Some((lane_slot, task_index)) = task.next_dispatchable_lane() {
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
        let (lane_slot, task_index) = task.next_dispatchable_lane()?;
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

        task.mark_lane_running(binding.lane_slot as usize);
        self.core_binding.insert(core_slot, binding);
        Some(DispatchSetup {
            core_slot,
            binding,
            submit_flags: meta.flags,
            task_total: task.tasks.len() as u32,
            task_array_dma_address: meta.task_array_dma_address,
            task: task_snapshot,
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
}

pub(super) struct RknpuScheduler<P: RknpuPlatform> {
    state: Mutex<NpuSchedulerState<P::Waiter>>,
    kick: P::WorkerSignal,
}

impl<P: RknpuPlatform> RknpuScheduler<P> {
    /// Build the scheduler state and worker wake-up primitive for one service.
    pub(super) fn new(platform: &P) -> Self {
        Self {
            state: Mutex::new(NpuSchedulerState::new()),
            kick: platform.new_worker_signal(),
        }
    }
}

impl<P: RknpuPlatform> RknpuService<P> {
    /// Enqueue one submit, install its waiter, and wake the worker if needed.
    pub fn enqueue_submit(
        &self,
        queued_submit: RknpuQueuedSubmit,
    ) -> Result<RknpuQueueTaskId, RknpuServiceError> {
        let waiter = Arc::new(self.inner.platform.new_waiter());
        let submit_snapshot = queued_submit.meta;
        let (task_id, spawn_worker) = {
            let mut state = self.inner.scheduler.state.lock();
            let spawn_worker = !state.worker_started;
            if spawn_worker {
                state.worker_started = true;
            }

            let task_id = state.enqueue_task(queued_submit);
            state.waiters.insert(task_id, waiter);
            (task_id, spawn_worker)
        };

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
        let tasks = task.tasks;
        let last_error = task.last_error;
        Ok(CompletedSubmit {
            submit,
            tasks,
            last_error,
        })
    }

    /// Run one low-level driver closure through the platform's locking policy.
    pub(crate) fn with_npu_driver<T, F>(&self, f: F) -> Result<T, RknpuServiceError>
    where
        F: FnOnce(&mut crate::Rknpu) -> Result<T, RknpuError>,
    {
        self.inner.platform.with_device(f)
    }

    /// Return true when the worker has any live submit to process.
    fn has_work(&self) -> bool {
        let state = self.inner.scheduler.state.lock();
        state.has_live_work()
    }

    /// Sleep until enqueue or completion activity makes work available.
    fn wait_for_work(&self) {
        loop {
            if self.has_work() {
                return;
            }

            let listener = self.inner.scheduler.kick.listen();
            if self.has_work() {
                return;
            }

            listener.wait();
        }
    }

    /// Complete waiters for terminal submits after optional read-side cache prep.
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
                task.fail_lane(binding.lane_slot as usize, err);
            }

            if let Some(task_id) = state.reclassify_task(binding.task_id) {
                terminal_ids.push(task_id);
            }

            terminal_ids
        };

        self.wake_terminal_tasks(terminal_ids, false);
    }

    /// Log enough inflight state to diagnose a worker that is waiting on hardware.
    fn log_inflight_snapshot(
        &self,
        inflight_yields: u32,
        harvest_rounds: u32,
        dispatch_rounds: u32,
    ) {
        let state = self.inner.scheduler.state.lock();
        if state.core_binding.is_empty() {
            debug!(
                "[rknpu-scheduler] inflight snapshot yields={} harvest_rounds={} \
                 dispatch_rounds={} no_inflight_entries=true",
                inflight_yields, harvest_rounds, dispatch_rounds
            );
            return;
        }

        for (core_slot, binding) in &state.core_binding {
            if let Some(task) = state.tasks.get(&binding.task_id) {
                debug!(
                    "[rknpu-scheduler] inflight snapshot yields={} harvest_rounds={} \
                     dispatch_rounds={} core={} queue_task={} lane={} task_index={} completed={} \
                     running_lanes={} dispatchable={} cursors={:?} lane_isrun={:?} last_error={:?}",
                    inflight_yields,
                    harvest_rounds,
                    dispatch_rounds,
                    core_slot,
                    binding.task_id,
                    binding.lane_slot,
                    binding.task_index,
                    task.completed_task_count(),
                    task.has_running_lanes(),
                    task.has_dispatchable_lane(),
                    task.subcore_cursors,
                    task.lane_isrun,
                    task.last_error
                );
            } else {
                debug!(
                    "[rknpu-scheduler] inflight snapshot yields={} harvest_rounds={} \
                     dispatch_rounds={} core={} queue_task={} lane={} task_index={} \
                     queue_task_missing=true",
                    inflight_yields,
                    harvest_rounds,
                    dispatch_rounds,
                    core_slot,
                    binding.task_id,
                    binding.lane_slot,
                    binding.task_index
                );
            }
        }
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

        let terminal_ids = {
            let mut state = self.inner.scheduler.state.lock();
            let mut terminal_ids = Vec::new();

            for completion in completions {
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

                if let Some(task_desc) = task.tasks.get_mut(binding.task_index as usize) {
                    let expected_irq_mask = task_desc.int_mask;
                    last_task_int_status = completion.observed_irq_status & expected_irq_mask;
                    task_error = completion.observed_irq_status != 0 && last_task_int_status == 0;

                    unsafe {
                        addr_of_mut!((*task_desc).int_status).write_unaligned(last_task_int_status);
                    }

                    task.complete_lane(binding.lane_slot as usize);
                    if task_error {
                        task.last_error = Some(RknpuError::TaskError);
                    }
                } else {
                    task.fail_lane(binding.lane_slot as usize, RknpuError::InvalidParameter);
                    debug!(
                        "[rknpu-scheduler] completion core={} queue_task={} invalid_task_index={}",
                        core_slot, binding.task_id, binding.task_index
                    );
                }

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
                    terminal_ids.push(task_id);
                }
            }

            terminal_ids
        };

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
                    let running_has_candidate = state.has_running_candidate_for_any(&idle_cores);
                    let mut prepared = None;

                    for core_slot in idle_cores {
                        if let Some(setup) = state.prepare_dispatch_from_running(core_slot) {
                            prepared = Some(setup);
                            break;
                        }

                        if !running_has_candidate {
                            if let Some(setup) = state.promote_ready_and_prepare_dispatch(core_slot)
                            {
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
                    self.fail_dispatch(setup.core_slot, setup.binding, err.to_driver_error());
                    continue;
                }
            }

            let submit_result = self.with_npu_driver(|rknpu_dev| {
                rknpu_dev.submit_ioctrl_step(
                    setup.core_slot,
                    setup.submit_flags,
                    setup.task_total,
                    setup.task_array_dma_address,
                    setup.binding.lane_slot,
                    setup.binding.task_index,
                    &mut setup.task,
                )
            });

            match submit_result {
                Ok(()) => {
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

    /// Singleton worker loop that alternates harvest, dispatch, and idle sleep.
    fn worker_main(self) {
        debug!("[rknpu-scheduler] worker thread started");
        loop {
            self.wait_for_work();
            let (has_inflight_on_wake, has_work_on_wake) = {
                let state = self.inner.scheduler.state.lock();
                (state.has_inflight(), state.has_live_work())
            };
            debug!(
                "[rknpu-scheduler] worker awakened has_inflight={} has_work={}",
                has_inflight_on_wake, has_work_on_wake
            );

            let mut harvest_rounds = 0u32;
            let mut dispatch_rounds = 0u32;
            let mut inflight_yields = 0u32;
            let mut stalled_yields = 0u32;

            loop {
                let harvested = self.harvest_completed_cores();
                let dispatched = self.dispatch_idle_cores();

                if harvested {
                    harvest_rounds = harvest_rounds.saturating_add(1);
                }
                if dispatched {
                    dispatch_rounds = dispatch_rounds.saturating_add(1);
                }

                if harvested || dispatched {
                    continue;
                }

                let (has_inflight, has_work) = {
                    let state = self.inner.scheduler.state.lock();
                    (state.has_inflight(), state.has_live_work())
                };

                if has_inflight {
                    inflight_yields = inflight_yields.saturating_add(1);
                    if inflight_yields == 1 || inflight_yields % 64 == 0 {
                        debug!(
                            "[rknpu-scheduler] worker waiting on inflight hardware yields={} \
                             harvest_rounds={} dispatch_rounds={}",
                            inflight_yields, harvest_rounds, dispatch_rounds
                        );
                        self.log_inflight_snapshot(
                            inflight_yields,
                            harvest_rounds,
                            dispatch_rounds,
                        );
                    }

                    self.inner.platform.yield_now();
                    continue;
                }

                if !has_work {
                    debug!(
                        "[rknpu-scheduler] worker drained current wakeup and is returning to sleep \
                         harvest_rounds={} dispatch_rounds={} inflight_yields={} stalled_yields={}",
                        harvest_rounds, dispatch_rounds, inflight_yields, stalled_yields
                    );
                    break;
                }

                stalled_yields = stalled_yields.saturating_add(1);
                if stalled_yields == 1 || stalled_yields % 64 == 0 {
                    debug!(
                        "[rknpu-scheduler] worker still has queued work but no dispatch was possible \
                         yields={} harvest_rounds={} dispatch_rounds={}",
                        stalled_yields, harvest_rounds, dispatch_rounds
                    );
                }

                self.inner.platform.yield_now();
            }
        }
    }

    #[cfg(test)]
    /// Test-only probe for whether the mock worker has issued hardware work.
    pub(super) fn has_inflight_dispatches(&self) -> bool {
        !self.inner.scheduler.state.lock().core_binding.is_empty()
    }
}
