// RKNPU 测试观测与 Dynamic Submit 校验，最后修改日期：2026-08-18。
use alloc::vec;
use core::{convert::TryFrom, mem};

use crate::{
    RknpuAction, RknpuQueuedSubmit, RknpuTask, SubmitMeta,
    ioctrl::{
        RKNPU_SCHEDULE_EVENT_ALL, RKNPU_SCHEDULE_TRACE_CAPACITY,
        RKNPU_SCHEDULE_TRACE_CONFIG_RESET, RKNPU_SCHEDULE_TRACE_READ,
        RKNPU_SCHEDULE_TRACE_STATE, RKNPU_SUBMIT_TRACE_CAPACITY,
        RKNPU_SUBMIT_TRACE_READ, RKNPU_SUBMIT_TRACE_RESET, RknpuMemCreate,
        RknpuMemDestroy, RknpuMemMap, RknpuMemSync, RknpuScheduleTraceQuery,
        RknpuScheduleTraceRecord, RknpuSchedulerStateSnapshot, RknpuSubmit,
        RKNPU_WORKER_YIELD_TRACE_CONFIG_RESET, RKNPU_WORKER_YIELD_TRACE_DEFAULT_CAPACITY,
        RKNPU_WORKER_YIELD_TRACE_MAX_CAPACITY, RKNPU_WORKER_YIELD_TRACE_READ,
        RknpuSubmitTraceQuery, RknpuSubmitTraceRecord, RknpuWorkerYieldTraceQuery,
        RknpuWorkerYieldTraceRecord,
    },
};

use super::{RknpuPlatform, RknpuService, RknpuServiceError};

/// RKNPU driver-ioctl command numbers.
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
//rknpuCmdRKNPU驱动程序的ioctl命令编号。每个变体对应一个特定的ioctl操作，例如提交任务、创建内存、映射内存等，可以将历史的ioctl编号转换为内部命令枚举，从而在驱动程序中处理不同的ioctl请求。
pub enum RknpuCmd {
    /// Generic action/query command.
    Action = 0x00,
    /// Blocking submit command.
    Submit = 0x01,
    /// DMA buffer allocation command.
    MemCreate = 0x02,
    /// DMA buffer mmap offset lookup command.
    MemMap = 0x03,
    /// DMA buffer destruction command.
    MemDestroy = 0x04,
    /// DMA buffer sync command.
    MemSync = 0x05,
    /// 测试专用的 Submit 延迟记录命令。
    SubmitTrace = 0x06,
    /// 测试专用的调度事件配置、读取和资源状态命令。
    ScheduleTrace = 0x07,
    /// Experiment-only in-memory timestamps around Worker yield calls.
    WorkerYieldTrace = 0x08,
}

impl TryFrom<u32> for RknpuCmd {
    type Error = ();

    /// Decode the historical ioctl number into the internal command enum.
    fn try_from(nr: u32) -> Result<Self, Self::Error> {
        match nr {
            0x00 | 0x40 => Ok(Self::Action),
            0x01 | 0x41 => Ok(Self::Submit),
            0x02 | 0x42 => Ok(Self::MemCreate),
            0x03 | 0x43 => Ok(Self::MemMap),
            0x04 | 0x44 => Ok(Self::MemDestroy),
            0x05 | 0x45 => Ok(Self::MemSync),
            0x06 | 0x46 => Ok(Self::SubmitTrace),
            0x07 | 0x47 => Ok(Self::ScheduleTrace),
            0x08 | 0x48 => Ok(Self::WorkerYieldTrace),
            _ => Err(()),
        }
    }
}

/// Userspace action payload mirrored from the historical StarryOS ioctl path.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct RknpuUserAction {
    /// Requested raw action opcode copied from userspace.
    pub flags: u32,
    /// Input/output value associated with the action.
    pub value: u32,
}

impl Default for RknpuUserAction {
    /// Default to a harmless driver-version query with an empty value field.
    fn default() -> Self {
        Self {
            flags: RknpuAction::GetDrvVersion as u32,
            value: 0,
        }
    }
}

impl<P: RknpuPlatform> RknpuService<P> {
    /// Main service entry point used by OS device adapters.
    pub fn driver_ioctl(&self, op: RknpuCmd, arg: usize) -> Result<usize, RknpuServiceError> {
        match op {
            RknpuCmd::Submit => self.handle_submit_ioctl(arg),
            RknpuCmd::MemCreate => self.handle_mem_create_ioctl(arg),
            RknpuCmd::MemMap => self.handle_mem_map_ioctl(arg),
            RknpuCmd::MemDestroy => self.handle_mem_destroy_ioctl(arg),
            RknpuCmd::MemSync => self.handle_mem_sync_ioctl(arg),
            RknpuCmd::SubmitTrace => self.handle_submit_trace_ioctl(arg),
            RknpuCmd::ScheduleTrace => self.handle_schedule_trace_ioctl(arg),
            RknpuCmd::WorkerYieldTrace => self.handle_worker_yield_trace_ioctl(arg),
            RknpuCmd::Action => self.handle_action_ioctl(arg),
        }
    }

    /// Copy one typed ioctl payload from userspace into a kernel value.
    fn copy_from_user<T: Default>(&self, src: usize) -> Result<T, RknpuServiceError> {
        let mut value = T::default();
        self.inner.platform.copy_from_user(
            &mut value as *mut _ as *mut u8,
            src as *const u8,
            mem::size_of::<T>(),
        )?;
        Ok(value)
    }

    /// Copy one typed ioctl result from kernel memory back to userspace.
    fn copy_to_user<T>(&self, dst: usize, value: &T) -> Result<(), RknpuServiceError> {
        self.inner.platform.copy_to_user(
            dst as *mut u8,
            value as *const _ as *const u8,
            mem::size_of::<T>(),
        )
    }

    /// Handle a blocking submit ioctl from task-array copy-in to terminal copy-back.
    fn handle_submit_ioctl(&self, arg: usize) -> Result<usize, RknpuServiceError> {
        // t0：Submit ioctl 已进入驱动。
        let submit_enter_time_ns = self.inner.platform.monotonic_time_ns();
        let submit_args = self.copy_from_user::<RknpuSubmit>(arg)?;

        if submit_args.task_number == 0 || submit_args.task_array_cpu_address == 0 {
            debug!(
                "rknpu invalid submit header: task_number={}, task_array_cpu_address={:#x}, \
                 task_array_dma_address={:#x}",
                submit_args.task_number, submit_args.task_array_cpu_address, submit_args.task_array_dma_address,
            );
            return Err(RknpuServiceError::InvalidData);
        }

        // 动态模式在分配内核 Task 数组前完成全部布局校验，最后修改日期：2026-08-18。
        // 非法 flags、core mask、Lane 重叠、空洞或越界均不得进入调度器状态。
        if !SubmitMeta::dynamic_layout_is_valid(&submit_args) {
            debug!(
                "rknpu invalid dynamic submit layout: flags={:#x} core_mask={:#x} \
                 task_number={}",
                submit_args.flags, submit_args.core_mask, submit_args.task_number
            );
            return Err(RknpuServiceError::InvalidData);
        }

        if submit_args.task_array_dma_address == 0 {
            debug!(
                "rknpu submit header keeps legacy zero task_array_dma_address, scheduler will preserve \
                 zero DMA base"
            );
        }

        let user_task_array_cpu_address = submit_args.task_array_cpu_address;
        let task_bytes = (submit_args.task_number as usize)
            .checked_mul(mem::size_of::<RknpuTask>())
            .ok_or(RknpuServiceError::InvalidData)?;
        let mut tasks = vec![RknpuTask::default(); submit_args.task_number as usize];
        self.inner.platform.copy_from_user(
            tasks.as_mut_ptr() as *mut u8,
            user_task_array_cpu_address as *const u8,
            task_bytes,
        )?;

        debug!(
            "[rknpu-submit] queueing blocking submit task_number={} core_mask={:#x} \
             timeout={} task_array_dma_address={:#x} user_task_array_cpu_address={:#x}",
            submit_args.task_number,
            submit_args.core_mask,
            submit_args.timeout,
            submit_args.task_array_dma_address,
            user_task_array_cpu_address
        );
        let mut queued_submit = RknpuQueuedSubmit::new(submit_args.clone(), tasks);
        // Keep t0 with the scheduler-owned task.
        queued_submit.latency.t0_ns = submit_enter_time_ns;
        let queue_task_id = self.enqueue_submit(queued_submit)?;

        debug!(
            "[rknpu-submit] enqueued queue_task={} and entering blocking wait",
            queue_task_id
        );
        self.wait_for_submit(queue_task_id)?;

        debug!(
            "[rknpu-submit] blocking wait finished for queue_task={}, collecting terminal snapshot",
            queue_task_id
        );
        let finished = self.take_terminal_submit(queue_task_id)?;
        let latency = finished.latency;
        let mut finished_submit = finished.submit;
        finished_submit.task_array_cpu_address = user_task_array_cpu_address;

        debug!(
            "[rknpu-submit] terminal queue_task={} task_counter={} last_error={:?}",
            queue_task_id, finished_submit.task_counter, finished.last_error
        );

        self.inner.platform.copy_to_user(
            user_task_array_cpu_address as *mut u8,
            finished.tasks.as_ptr() as *const u8,
            task_bytes,
        )?;
        self.copy_to_user(arg, &finished_submit)?;

        if let Some(err) = finished.last_error {
            warn!("rknpu submit ioctl completed with driver error: {:?}", err);
            return Err(RknpuServiceError::Driver(err));
        }

        // t4：任务数组和 Submit 已经计算完完成并完整复制回用户空间。
        let submit_return_time_ns = self.inner.platform.monotonic_time_ns();
        self.record_submit_trace(RknpuSubmitTraceRecord {
            queue_task: queue_task_id,
            t0_ns: latency.t0_ns,
            t1_ns: latency.t1_ns,
            t2_ns: latency.t2_ns,
            t3_ns: latency.t3_ns,
            t4_ns: submit_return_time_ns,
        });

        Ok(0)
    }

    /// 复位或一次性读取 Submit 延迟记录。
    fn handle_submit_trace_ioctl(&self, arg: usize) -> Result<usize, RknpuServiceError> {
        let mut query = self.copy_from_user::<RknpuSubmitTraceQuery>(arg)?;

        match query.operation {
            RKNPU_SUBMIT_TRACE_RESET => {
                self.reset_submit_trace();
                query.count = 0;
                query.overflowed = 0;
            }
            RKNPU_SUBMIT_TRACE_READ => {
                let capacity = query.capacity as usize;
                if capacity > RKNPU_SUBMIT_TRACE_CAPACITY {
                    return Err(RknpuServiceError::InvalidInput);
                }
                if capacity > 0 && query.records_address == 0 {
                    return Err(RknpuServiceError::InvalidInput);
                }

                let (records, overflowed) = self.snapshot_submit_trace(capacity);
                let record_bytes = records
                    .len()
                    .checked_mul(mem::size_of::<RknpuSubmitTraceRecord>())
                    .ok_or(RknpuServiceError::InvalidInput)?;
                if record_bytes > 0 {
                    let records_address = usize::try_from(query.records_address)
                        .map_err(|_| RknpuServiceError::InvalidInput)?;
                    self.inner.platform.copy_to_user(
                        records_address as *mut u8,
                        records.as_ptr() as *const u8,
                        record_bytes,
                    )?;
                }
                query.count = records.len() as u32;
                query.overflowed = if overflowed { 1 } else { 0 };
            }
            _ => return Err(RknpuServiceError::InvalidInput),
        }

        self.copy_to_user(arg, &query)?;
        Ok(0)
    }

    /// Configure/reset or read the experiment-only Worker yield trace.
    ///
    /// All allocation happens during CONFIG_RESET or READ. The measured Worker
    /// path only appends pre/post timestamps to the already allocated buffer.
    fn handle_worker_yield_trace_ioctl(
        &self,
        arg: usize,
    ) -> Result<usize, RknpuServiceError> {
        let mut query = self.copy_from_user::<RknpuWorkerYieldTraceQuery>(arg)?;
        if query.reserved != 0 {
            return Err(RknpuServiceError::InvalidInput);
        }

        match query.operation {
            RKNPU_WORKER_YIELD_TRACE_CONFIG_RESET => {
                if query.enabled > 1 {
                    return Err(RknpuServiceError::InvalidInput);
                }
                let enabled = query.enabled != 0;
                let capacity = if !enabled {
                    0
                } else if query.capacity == 0 {
                    RKNPU_WORKER_YIELD_TRACE_DEFAULT_CAPACITY
                } else {
                    query.capacity as usize
                };
                if capacity > RKNPU_WORKER_YIELD_TRACE_MAX_CAPACITY {
                    return Err(RknpuServiceError::InvalidInput);
                }
                self.configure_worker_yield_trace(enabled, capacity)?;
                query.capacity = capacity as u32;
                query.count = 0;
                query.overflowed = 0;
            }
            RKNPU_WORKER_YIELD_TRACE_READ => {
                let capacity = query.capacity as usize;
                if capacity > RKNPU_WORKER_YIELD_TRACE_MAX_CAPACITY
                    || (capacity > 0 && query.records_address == 0)
                {
                    return Err(RknpuServiceError::InvalidInput);
                }

                let (records, enabled, overflowed) =
                    self.snapshot_worker_yield_trace(capacity)?;
                let record_bytes = records
                    .len()
                    .checked_mul(mem::size_of::<RknpuWorkerYieldTraceRecord>())
                    .ok_or(RknpuServiceError::InvalidInput)?;
                if record_bytes > 0 {
                    let records_address = usize::try_from(query.records_address)
                        .map_err(|_| RknpuServiceError::InvalidInput)?;
                    self.inner.platform.copy_to_user(
                        records_address as *mut u8,
                        records.as_ptr() as *const u8,
                        record_bytes,
                    )?;
                }
                query.enabled = if enabled { 1 } else { 0 };
                query.count = records.len() as u32;
                query.overflowed = if overflowed { 1 } else { 0 };
            }
            _ => return Err(RknpuServiceError::InvalidInput),
        }

        self.copy_to_user(arg, &query)?;
        Ok(0)
    }

    /// 配置、读取调度事件或获取资源状态，最后修改日期：2026-08-07。
    ///
    /// `query` 来自用户空间，操作码、事件位、容量和地址都不可信。该接口
    /// 先完成范围与乘法检查，再取得内核快照，最后在不持有调度器、设备或
    /// trace 锁的情况下 copy-out，避免非法指针破坏锁内状态。
    fn handle_schedule_trace_ioctl(&self, arg: usize) -> Result<usize, RknpuServiceError> {
        let mut query = self.copy_from_user::<RknpuScheduleTraceQuery>(arg)?;

        match query.operation {
            RKNPU_SCHEDULE_TRACE_CONFIG_RESET => {
                // 未定义事件位直接拒绝，避免用户态与内核对记录语义理解不同。
                if query.event_mask & !RKNPU_SCHEDULE_EVENT_ALL != 0 {
                    return Err(RknpuServiceError::InvalidInput);
                }
                // event_mask=0 表示关闭；非零表示开始一轮新的独立实验。
                self.configure_schedule_trace(query.event_mask);
                query.count = 0;
                query.overflowed = 0;
            }
            RKNPU_SCHEDULE_TRACE_READ => {
                let capacity = query.capacity as usize;
                // capacity 是元素数量。非零容量必须配套有效的用户数组地址。
                if capacity > RKNPU_SCHEDULE_TRACE_CAPACITY
                    || (capacity > 0 && query.records_address == 0)
                {
                    return Err(RknpuServiceError::InvalidInput);
                }

                // snapshot_schedule_trace 返回内核拥有的 Vec；后续不再持有 trace 锁。
                let (records, event_mask, overflowed) =
                    self.snapshot_schedule_trace(capacity);
                // 用户控制 capacity，字节数必须使用 checked_mul，禁止整数回绕。
                let record_bytes = records
                    .len()
                    .checked_mul(mem::size_of::<RknpuScheduleTraceRecord>())
                    .ok_or(RknpuServiceError::InvalidInput)?;
                if record_bytes > 0 {
                    let records_address = usize::try_from(query.records_address)
                        .map_err(|_| RknpuServiceError::InvalidInput)?;
                    self.inner.platform.copy_to_user(
                        records_address as *mut u8,
                        records.as_ptr() as *const u8,
                        record_bytes,
                    )?;
                }
                query.event_mask = event_mask;
                query.count = records.len() as u32;
                query.overflowed = if overflowed { 1 } else { 0 };
            }
            RKNPU_SCHEDULE_TRACE_STATE => {
                // STATE 固定复制一个结构，不接受空输出地址。
                if query.state_address == 0 {
                    return Err(RknpuServiceError::InvalidInput);
                }

                // 先取调度快照并释放调度锁，再获取设备锁读取 GEM，固定锁顺序。
                let mut state = self.snapshot_scheduler_state();
                let (gem_buffers, gem_bytes) = self.with_npu_driver(|rknpu_dev| {
                    Ok((
                        rknpu_dev.active_buffer_count(),
                        rknpu_dev.active_byte_count(),
                    ))
                })?;
                state.gem_buffers = gem_buffers as u32;
                state.gem_bytes = gem_bytes as u64;

                let state_address = usize::try_from(query.state_address)
                    .map_err(|_| RknpuServiceError::InvalidInput)?;
                self.copy_to_user::<RknpuSchedulerStateSnapshot>(state_address, &state)?;
                query.count = 0;
                query.overflowed = 0;
            }
            _ => return Err(RknpuServiceError::InvalidInput),
        }

        // 最后把 count、overflowed 和当前 event_mask 返回给调用者。
        self.copy_to_user(arg, &query)?;
        Ok(0)
    }

    /// Allocate a driver GEM object and return its handle to userspace.
    fn handle_mem_create_ioctl(&self, arg: usize) -> Result<usize, RknpuServiceError> {
        let mut mem_create_args = self.copy_from_user::<RknpuMemCreate>(arg)?;
        self.with_npu_driver(|rknpu_dev| rknpu_dev.create(&mut mem_create_args))?;
        self.copy_to_user(arg, &mem_create_args)?;
        Ok(0)
    }

    /// Convert a GEM handle into the legacy mmap offset expected by userspace.
    fn handle_mem_map_ioctl(&self, arg: usize) -> Result<usize, RknpuServiceError> {
        const PAGE_SHIFT: u32 = 12;

        let mut mem_map = self.copy_from_user::<RknpuMemMap>(arg)?;
        self.with_npu_driver(|rknpu_dev| {
            if rknpu_dev.get_phys_addr_and_size(mem_map.handle).is_some() {
                mem_map.offset = (mem_map.handle as u64) << PAGE_SHIFT;
                Ok(())
            } else {
                Err(crate::RknpuError::InvalidHandle)
            }
        })?;
        self.copy_to_user(arg, &mem_map)?;
        Ok(0)
    }

    /// Destroy a GEM object if the supplied handle still exists.
    fn handle_mem_destroy_ioctl(&self, arg: usize) -> Result<usize, RknpuServiceError> {
        let mem_destroy = self.copy_from_user::<RknpuMemDestroy>(arg)?;
        self.with_npu_driver(|rknpu_dev| {
            if rknpu_dev
                .get_phys_addr_and_size(mem_destroy.handle)
                .is_none()
            {
                warn!(
                    "[rknpu] mem_destroy ignored unknown handle={} obj_addr={:#x}",
                    mem_destroy.handle, mem_destroy.obj_addr
                );
                return Ok(());
            }

            rknpu_dev.destroy(mem_destroy.handle);
            Ok(())
        })?;
        Ok(0)
    }

    /// Run cache synchronization for a userspace-visible GEM object.
    fn handle_mem_sync_ioctl(&self, arg: usize) -> Result<usize, RknpuServiceError> {
        let mut mem_sync = self.copy_from_user::<RknpuMemSync>(arg)?;
        self.with_npu_driver(|rknpu_dev| {
            rknpu_dev.sync(&mut mem_sync);
            Ok(())
        })?;
        Ok(0)
    }

    /// Execute a small driver action query/update and copy the result value back.
    fn handle_action_ioctl(&self, arg: usize) -> Result<usize, RknpuServiceError> {
        let mut action = self.copy_from_user::<RknpuUserAction>(arg)?;
        let action_code =
            RknpuAction::try_from(action.flags).map_err(|_| RknpuServiceError::BadIoctl)?;
        self.with_npu_driver(|rknpu_dev| {
            let val = rknpu_dev.action(action_code, action.value)?;
            action.value = val;
            Ok(())
        })?;
        self.copy_to_user(arg, &action)?;
        Ok(0)
    }
}
