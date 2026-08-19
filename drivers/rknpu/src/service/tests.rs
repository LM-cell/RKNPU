// Submit 延迟、优先级与 IRQ 唤醒仿真测试，最后修改日期：2026-08-17。
use alloc::{vec, vec::Vec};
use core::{
    ptr::NonNull,
    sync::atomic::{AtomicBool, AtomicU64, AtomicUsize, Ordering},
    time::Duration,
};

use super::{
    RknpuCmd, RknpuDeviceAccess, RknpuSchedulerRuntime, RknpuService, RknpuServiceError,
    RknpuSubmitWaiter, RknpuUserMemory, RknpuWorkerListener, RknpuWorkerSignal,
};
use crate::{
    Rknpu, RknpuAction, RknpuConfig, RknpuError, RknpuTask, RknpuType,
    ioctrl::{
        RKNPU_DISPATCH_SOURCE_READY, RKNPU_DISPATCH_SOURCE_RUNNING,
        RKNPU_SCHEDULE_EVENT_ALL, RKNPU_SCHEDULE_EVENT_COMPLETE,
        RKNPU_SCHEDULE_EVENT_DISPATCH, RKNPU_SCHEDULE_EVENT_ENQUEUE,
        RKNPU_SCHEDULE_TRACE_CONFIG_RESET, RKNPU_SCHEDULE_TRACE_READ,
        RKNPU_SCHEDULE_TRACE_STATE, RKNPU_SUBMIT_TRACE_CAPACITY,
        RKNPU_SUBMIT_TRACE_READ, RKNPU_SUBMIT_TRACE_RESET, RknpuMemCreate,
        RknpuMemDestroy, RknpuMemMap, RknpuScheduleTraceQuery,
        RknpuScheduleTraceRecord, RknpuSchedulerStateSnapshot, RknpuSubmit,
        RKNPU_WORKER_YIELD_TRACE_CONFIG_RESET, RKNPU_WORKER_YIELD_TRACE_DEFAULT_CAPACITY,
        RKNPU_WORKER_YIELD_TRACE_MAX_CAPACITY, RKNPU_WORKER_YIELD_TRACE_READ,
        RknpuSubmitTraceQuery, RknpuSubmitTraceRecord, RknpuWorkerYieldTraceQuery,
    },
};
use std::{
    collections::BTreeSet,
    sync::{Arc, Condvar, Mutex},
    thread::{self, JoinHandle},
};

const FAKE_MMIO_LEN: usize = 0x10000;

#[test]
fn submit_trace_drm_command_is_decoded() {
    // DRM 命令号包含 0x40 基值；确认两个测试 ioctl 都能映射到内部枚举，
    // 避免用户态头文件已经增加命令，而设备适配层仍返回 BadIoctl。
    assert_eq!(RknpuCmd::try_from(0x46), Ok(RknpuCmd::SubmitTrace));
    assert_eq!(RknpuCmd::try_from(0x47), Ok(RknpuCmd::ScheduleTrace));
    assert_eq!(RknpuCmd::try_from(0x48), Ok(RknpuCmd::WorkerYieldTrace));
}

#[test]
fn worker_yield_trace_config_and_empty_read_roundtrip() {
    let service = RknpuService::new(MockPlatform::new());
    let mut configure = RknpuWorkerYieldTraceQuery {
        operation: RKNPU_WORKER_YIELD_TRACE_CONFIG_RESET,
        enabled: 1,
        ..RknpuWorkerYieldTraceQuery::default()
    };
    service
        .driver_ioctl(
            RknpuCmd::WorkerYieldTrace,
            (&mut configure as *mut RknpuWorkerYieldTraceQuery) as usize,
        )
        .unwrap();

    let mut read = RknpuWorkerYieldTraceQuery {
        operation: RKNPU_WORKER_YIELD_TRACE_READ,
        ..RknpuWorkerYieldTraceQuery::default()
    };
    service
        .driver_ioctl(
            RknpuCmd::WorkerYieldTrace,
            (&mut read as *mut RknpuWorkerYieldTraceQuery) as usize,
        )
        .unwrap();
    assert_eq!(read.enabled, 1);
    assert_eq!(read.count, 0);
    assert_eq!(read.overflowed, 0);
    assert_eq!(configure.capacity as usize, RKNPU_WORKER_YIELD_TRACE_DEFAULT_CAPACITY);

    configure.enabled = 0;
    service
        .driver_ioctl(
            RknpuCmd::WorkerYieldTrace,
            (&mut configure as *mut RknpuWorkerYieldTraceQuery) as usize,
        )
        .unwrap();
}

#[test]
fn worker_yield_trace_rejects_capacity_above_safety_ceiling() {
    let service = RknpuService::new(MockPlatform::new());
    let mut configure = RknpuWorkerYieldTraceQuery {
        operation: RKNPU_WORKER_YIELD_TRACE_CONFIG_RESET,
        enabled: 1,
        capacity: RKNPU_WORKER_YIELD_TRACE_MAX_CAPACITY as u32 + 1,
        ..RknpuWorkerYieldTraceQuery::default()
    };

    assert_eq!(
        service.driver_ioctl(
            RknpuCmd::WorkerYieldTrace,
            (&mut configure as *mut RknpuWorkerYieldTraceQuery) as usize,
        ),
        Err(RknpuServiceError::InvalidInput)
    );
}

/// 为 `RknpuService` 提供调度仿真所需的平台能力。
///
/// 该平台组合了三组假 MMIO、用户内存复制、阻塞 Submit 等待器、worker 唤醒信号
/// 和确定性单调时钟。它用于验证调度状态机、优先级、事件归属和并发完成逻辑，
/// 不模拟真实 DMA、一致性维护、硬件执行耗时或 IRQ 控制器行为。
#[derive(Clone)]
struct MockPlatform {
    /// 由所有 Submit 线程和调度 worker 共享的假 NPU 设备。
    device: Arc<MockDevice>,
    /// 记录 worker 创建次数，用于确认每个 service 只启动一个调度 worker。
    spawn_count: Arc<AtomicUsize>,
    /// 控制新建等待器是否立即返回中断错误，用于覆盖 ioctl 等待失败路径。
    interrupt_wait: Arc<AtomicBool>,
    /// 每次读取增加 1000 ns 的假单调时钟，用于稳定验证时间戳先后关系。
    clock_ns: Arc<AtomicU64>,
    /// 记录调度 worker 实际进入阻塞等待的次数，用于区分 IRQ 唤醒与 yield 轮询。
    worker_wait_count: Arc<AtomicUsize>,
}

/// 带互斥保护的共享假硬件对象。
///
/// `_mmios` 持有三块 MMIO 后备内存，保证传给 `Rknpu` 的裸指针在整个测试期间有效；
/// `npu` 的互斥锁模拟适配层对同一设备寄存器访问的串行化。
struct MockDevice {
    _mmios: Vec<Vec<u8>>,
    npu: Mutex<Rknpu>,
}

impl MockPlatform {
    /// 创建带三组假 MMIO 的 RK3588 调度仿真平台。
    ///
    /// 每个 `Vec<u8>` 预先分配固定长度，取得基地址后不再改变容量；这些 Vec 随
    /// `MockDevice` 一直存活，因此 `Rknpu::new` 保存的三个 `NonNull` 基地址不会悬空。
    fn new() -> Self {
        let mut mmios = vec![vec![0_u8; FAKE_MMIO_LEN]; 3];
        let base_addrs = mmios
            .iter_mut()
            .map(|mmio| NonNull::new(mmio.as_mut_ptr()).unwrap())
            .collect::<Vec<_>>();
        let config = RknpuConfig {
            rknpu_type: RknpuType::Rk3588,
        };

        Self {
            device: Arc::new(MockDevice {
                _mmios: mmios,
                npu: Mutex::new(Rknpu::new(&base_addrs, config)),
            }),
            spawn_count: Arc::new(AtomicUsize::new(0)),
            interrupt_wait: Arc::new(AtomicBool::new(false)),
            clock_ns: Arc::new(AtomicU64::new(1_000)),
            worker_wait_count: Arc::new(AtomicUsize::new(0)),
        }
    }

    /// 向指定假核心的 IRQ 状态寄存器写入完成值。
    ///
    /// 这一步相当于测试主动制造一次硬件完成状态；随后由调度 worker 读取状态，
    /// 将完成事件匹配回对应的 Submit 和 Task。
    fn publish_completion_status(&self, core_slot: usize, irq_status: u32) {
        let dev = self.device.npu.lock().unwrap();
        dev.base[core_slot]
            .irq_status
            .store(irq_status, Ordering::Release);
    }

    /// 模拟完整 completion IRQ：先发布状态，再通过正式 Service 入口唤醒 worker。
    fn publish_completion(
        &self,
        service: &RknpuService<MockPlatform>,
        core_slot: usize,
        irq_status: u32,
    ) {
        self.publish_completion_status(core_slot, irq_status);
        service.notify_irq_completion();
    }

    /// 返回平台累计创建的调度 worker 数量。
    fn spawn_count(&self) -> usize {
        self.spawn_count.load(Ordering::SeqCst)
    }

    /// 返回调度 worker 调用阻塞等待器的累计次数。
    fn worker_wait_count(&self) -> usize {
        self.worker_wait_count.load(Ordering::SeqCst)
    }

}

impl RknpuDeviceAccess for MockPlatform {
    /// 在持有设备互斥锁时执行一次假 NPU 操作。
    ///
    /// 该互斥范围与适配层访问真实设备时的独占范围一致，避免多个测试线程同时修改
    /// 同一组假寄存器，使仿真仍能覆盖驱动规定的设备访问顺序。
    fn with_device<T, F>(&self, f: F) -> Result<T, RknpuServiceError>
    where
        F: FnOnce(&mut Rknpu) -> Result<T, RknpuError>,
    {
        let mut dev = self.device.npu.lock().unwrap();
        f(&mut dev).map_err(RknpuServiceError::from)
    }
}

impl RknpuUserMemory for MockPlatform {
    /// 模拟 copy-from-user：把测试调用方对象复制到驱动缓冲区。
    ///
    /// 测试传入的 `src` 必须来自当前进程仍然存活且至少包含 `size` 字节的对象；
    /// `dst` 由 service 分配，二者不重叠。该仿真实现只验证 ioctl 数据流，
    /// 不模拟内核页表检查、缺页处理或不可访问用户地址。
    fn copy_from_user(
        &self,
        dst: *mut u8,
        src: *const u8,
        size: usize,
    ) -> Result<(), RknpuServiceError> {
        // 安全性：上述测试约束保证源、目的地址在 `size` 范围内有效且不重叠。
        unsafe {
            core::ptr::copy_nonoverlapping(src, dst, size);
        }
        Ok(())
    }

    /// 模拟 copy-to-user：把驱动缓冲区复制回测试调用方对象。
    ///
    /// 测试传入的 `dst` 必须来自当前进程仍然存活且至少可写 `size` 字节的对象；
    /// `src` 由 service 持有，二者不重叠。真实系统中的用户地址校验由平台适配层负责。
    fn copy_to_user(
        &self,
        dst: *mut u8,
        src: *const u8,
        size: usize,
    ) -> Result<(), RknpuServiceError> {
        // 安全性：上述测试约束保证源、目的地址在 `size` 范围内有效且不重叠。
        unsafe {
            core::ptr::copy_nonoverlapping(src, dst, size);
        }
        Ok(())
    }
}

/// 用条件变量模拟一次 blocking Submit ioctl 的完成等待。
struct MockWaiter {
    /// 为 true 表示对应 Submit 已进入终态，可以让 ioctl 返回。
    done: Mutex<bool>,
    /// 调度 worker 完成 Submit 后通过该条件变量唤醒提交线程。
    cv: Condvar,
    /// 为 true 时模拟等待被信号中断，直接返回 `Interrupted`。
    interrupt: bool,
}

impl RknpuSubmitWaiter for MockWaiter {
    /// 阻塞到 Submit 完成；若测试启用了中断注入，则立即返回中断错误。
    fn wait(&self) -> Result<(), RknpuServiceError> {
        if self.interrupt {
            return Err(RknpuServiceError::Interrupted);
        }

        let mut done = self.done.lock().unwrap();
        while !*done {
            done = self.cv.wait(done).unwrap();
        }
        Ok(())
    }

    /// 标记 Submit 已完成，并唤醒等待该 Submit 的测试线程。
    fn complete(&self) {
        let mut done = self.done.lock().unwrap();
        *done = true;
        self.cv.notify_all();
    }
}

/// service 与调度 worker 共享的可克隆唤醒句柄。
#[derive(Clone)]
struct MockWorkerSignal {
    inner: Arc<MockWorkerSignalInner>,
}

/// 使用代数计数避免 worker 在“检查队列”和“开始休眠”之间丢失唤醒。
struct MockWorkerSignalInner {
    /// 每次通知递增；监听器只等待该值与创建时记录的值不同。
    generation: Mutex<u64>,
    /// 没有新工作时让 worker 休眠，有新 Submit 或完成事件时将其唤醒。
    cv: Condvar,
    /// 每次进入 `wait` 时递增，供 IRQ 唤醒仿真测试确认 worker 没有忙轮询。
    wait_count: Arc<AtomicUsize>,
}

/// 保存监听开始时的代数，等待后续通知推进代数。
struct MockWorkerListenerState {
    inner: Arc<MockWorkerSignalInner>,
    generation: u64,
}

impl RknpuWorkerListener for MockWorkerListenerState {
    /// 休眠到通知代数发生变化。
    ///
    /// 使用循环重新检查条件，可以同时处理条件变量的虚假唤醒。
    fn wait(self) {
        let mut generation_guard = self.inner.generation.lock().unwrap();
        while *generation_guard == self.generation {
            // 持有 generation 锁时通知端无法推进代数；计数后进入 Condvar::wait
            // 会原子地释放该锁，因此这里记录的确实是一次阻塞等待。
            self.inner.wait_count.fetch_add(1, Ordering::SeqCst);
            generation_guard = self.inner.cv.wait(generation_guard).unwrap();
        }
    }
}

impl RknpuWorkerSignal for MockWorkerSignal {
    type Listener = MockWorkerListenerState;

    /// 在 worker 再次检查调度队列之前保存当前通知代数。
    ///
    /// 若通知恰好发生在检查队列与调用 `wait` 之间，代数已经改变，监听器不会误睡。
    fn listen(&self) -> Self::Listener {
        let generation = *self.inner.generation.lock().unwrap();
        MockWorkerListenerState {
            inner: self.inner.clone(),
            generation,
        }
    }

    /// 推进通知代数并唤醒一个调度 worker。
    fn notify_one(&self) {
        let mut generation = self.inner.generation.lock().unwrap();
        *generation = generation.saturating_add(1);
        self.inner.cv.notify_one();
    }
}

impl RknpuSchedulerRuntime for MockPlatform {
    type Waiter = MockWaiter;
    type WorkerSignal = MockWorkerSignal;

    /// 按当前故障注入开关创建一个 Submit 等待器。
    fn new_waiter(&self) -> Self::Waiter {
        MockWaiter {
            done: Mutex::new(false),
            cv: Condvar::new(),
            interrupt: self.interrupt_wait.load(Ordering::Acquire),
        }
    }

    /// 为一个 service 实例创建独立的 worker 唤醒信号。
    fn new_worker_signal(&self) -> Self::WorkerSignal {
        MockWorkerSignal {
            inner: Arc::new(MockWorkerSignalInner {
                generation: Mutex::new(0),
                cv: Condvar::new(),
                wait_count: self.worker_wait_count.clone(),
            }),
        }
    }

    /// 返回确定性单调时间，每次调用固定前进 1000 ns。
    ///
    /// 固定步长只用于验证 t0～t4 的顺序与归属，不代表 RK3588 的真实阶段耗时。
    fn monotonic_time_ns(&self) -> u64 {
        self.clock_ns.fetch_add(1_000, Ordering::SeqCst)
    }

    /// 创建调度 worker，并累计创建次数供单 worker 测试断言。
    fn spawn_worker<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        self.spawn_count.fetch_add(1, Ordering::SeqCst);
        thread::spawn(f);
    }

    /// 仿真 stalled 分支时让出宿主线程；正常 completion 仍由 Mock IRQ 唤醒。
    /// 最后修改日期：2026-08-17。
    fn yield_now(&self) {
        thread::yield_now();
    }
}

/// 在有限时间内轮询异步调度条件。
///
/// 200 次、每次 2 ms 的上限会把死锁、漏唤醒或缺失事件转成明确失败，
/// 避免仿真测试因 worker 永久等待而挂住整个测试进程。
fn wait_until(mut condition: impl FnMut() -> bool) {
    for _ in 0..200 {
        if condition() {
            return;
        }
        thread::sleep(Duration::from_millis(2));
    }
    panic!("condition not reached before timeout");
}

/// 构造由测试线程持有 Task Vec 的最小合法单 Task Submit。
///
/// 默认限制在 core0 和 lane0，用于验证 blocking ioctl、copy-back、IRQ
/// 错误传播和 worker 生命周期，不引入多核选择因素。
fn build_single_task_submit(int_mask: u32) -> (RknpuSubmit, Vec<RknpuTask>) {
    let mut tasks = vec![RknpuTask {
        int_mask,
        ..RknpuTask::default()
    }];
    let mut submit = RknpuSubmit::default();
    submit.task_number = 1;
    submit.task_array_dma_address = 0x2000;
    submit.task_array_cpu_address = tasks.as_mut_ptr() as u64;
    submit.core_mask = 0x1;
    submit.subcore_task[0].task_start = 0;
    submit.subcore_task[0].task_number = 1;
    (submit, tasks)
}

/// 构造可指定优先级、核心掩码和 lane 布局的仿真 Submit。
///
/// 每个 Task 使用连续 `op_idx`，使测试能够在不依赖用户指针的情况下，
/// 将调度事件对应回具体 Submit 和 Task。所有 Task 使用同一预期 IRQ 位，
/// MockPlatform 只需向指定核心发布 `0x100` 即可模拟完成。
fn build_tagged_submit(
    task_count: u32,
    priority: i32,
    tag: u32,
    core_mask: u32,
    lane_counts: &[u32],
) -> (RknpuSubmit, Vec<RknpuTask>) {
    assert_eq!(lane_counts.iter().sum::<u32>(), task_count);
    let mut tasks = (0..task_count)
        .map(|index| RknpuTask {
            op_idx: tag + index,
            int_mask: 0x100,
            ..RknpuTask::default()
        })
        .collect::<Vec<_>>();
    let mut submit = RknpuSubmit {
        task_number: task_count,
        priority,
        task_array_cpu_address: tasks.as_mut_ptr() as u64,
        task_array_dma_address: 0x2000,
        core_mask,
        ..RknpuSubmit::default()
    };
    let mut task_start = 0;
    for (lane, task_number) in lane_counts.iter().copied().enumerate() {
        submit.subcore_task[lane].task_start = task_start;
        submit.subcore_task[lane].task_number = task_number;
        task_start += task_number;
    }
    (submit, tasks)
}

/// 在线程中执行一个 blocking Submit，并把完整返回数据交给测试线程校验。
///
/// Task Vec 的地址必须在线程内部重新写入 Submit，因为 Vec 随闭包移动后，
/// ioctl copy-in 使用的地址应以最终所有者中的缓冲区为准。JoinHandle 同时
/// 保留 Submit 和 Task，保证 blocking ioctl 返回前用户缓冲区一直存活。
fn spawn_tagged_submit(
    service: RknpuService<MockPlatform>,
    task_count: u32,
    priority: i32,
    tag: u32,
    core_mask: u32,
    lane_counts: Vec<u32>,
) -> JoinHandle<(
    Result<usize, RknpuServiceError>,
    RknpuSubmit,
    Vec<RknpuTask>,
)> {
    thread::spawn(move || {
        let (mut submit, mut tasks) =
            build_tagged_submit(task_count, priority, tag, core_mask, &lane_counts);
        submit.task_array_cpu_address = tasks.as_mut_ptr() as u64;
        let result = service.driver_ioctl(
            RknpuCmd::Submit,
            (&mut submit as *mut RknpuSubmit) as usize,
        );
        (result, submit, tasks)
    })
}

/// 清空上一场景并开启全部调度事件记录。
///
/// 每个仿真测试使用独立 Service，但仍显式复位 trace，保证测试只依赖公开
/// ioctl 的行为，不依赖缓冲区初始实现细节。
fn configure_schedule_trace(service: &RknpuService<MockPlatform>) {
    let mut query = RknpuScheduleTraceQuery {
        operation: RKNPU_SCHEDULE_TRACE_CONFIG_RESET,
        event_mask: RKNPU_SCHEDULE_EVENT_ALL,
        ..RknpuScheduleTraceQuery::default()
    };
    service
        .driver_ioctl(
            RknpuCmd::ScheduleTrace,
            (&mut query as *mut RknpuScheduleTraceQuery) as usize,
        )
        .unwrap();
}

/// 通过测试 ioctl 读取当前调度事件快照。
///
/// 使用驱动公布的最大容量，任何 overflow 都视为测试数据不完整并立即
/// 失败；调用者因此可以安全地用事件数量和顺序作严格断言。
fn read_schedule_trace(
    service: &RknpuService<MockPlatform>,
) -> Vec<RknpuScheduleTraceRecord> {
    let mut records = vec![
        RknpuScheduleTraceRecord::default();
        crate::ioctrl::RKNPU_SCHEDULE_TRACE_CAPACITY
    ];
    let mut query = RknpuScheduleTraceQuery {
        operation: RKNPU_SCHEDULE_TRACE_READ,
        capacity: records.len() as u32,
        records_address: records.as_mut_ptr() as u64,
        ..RknpuScheduleTraceQuery::default()
    };
    service
        .driver_ioctl(
            RknpuCmd::ScheduleTrace,
            (&mut query as *mut RknpuScheduleTraceQuery) as usize,
        )
        .unwrap();
    assert_eq!(query.overflowed, 0);
    records.truncate(query.count as usize);
    records
}

/// 轮询时判断指定 `op_idx` 的某类事件是否已经出现。
///
/// 该帮助函数只读取快照，不清空记录，用于把异步 worker 推进到确定的
/// 调度状态后再发布下一次模拟中断。
fn has_schedule_event(
    service: &RknpuService<MockPlatform>,
    event_type: u32,
    op_idx: u32,
) -> bool {
    read_schedule_trace(service)
        .iter()
        .any(|record| record.event_type == event_type && record.op_idx == op_idx)
}

/// 返回指定 Task 实际下发到的 NPU 核心。
///
/// 测试必须向真实绑定核心写入模拟 IRQ，不能假定 Task 一定落在 core0。
fn dispatched_core(service: &RknpuService<MockPlatform>, op_idx: u32) -> usize {
    read_schedule_trace(service)
        .iter()
        .find(|record| {
            record.event_type == RKNPU_SCHEDULE_EVENT_DISPATCH && record.op_idx == op_idx
        })
        .map(|record| record.core_slot as usize)
        .expect("dispatch event not found")
}

/// 等待 blocking Submit 线程退出，并校验所有 Task 已计入 task_counter。
///
/// ioctl 错误、线程 panic、少完成 Task 都在这里转为明确的测试失败。
fn join_submit(
    handle: JoinHandle<(
        Result<usize, RknpuServiceError>,
        RknpuSubmit,
        Vec<RknpuTask>,
    )>,
) {
    let (result, submit, _tasks) = handle.join().unwrap();
    assert_eq!(result.unwrap(), 0);
    assert_eq!(submit.task_counter, submit.task_number);
}

#[test]
fn submit_ioctl_copies_back_terminal_state() {
    // 启动 blocking Submit，等 worker 建立核心绑定后发布模拟 IRQ。预期
    // task_counter、int_status 均复制回用户缓冲区，而且只创建一个 worker。
    let platform = MockPlatform::new();
    let service = RknpuService::new(platform.clone());

    let submitter = {
        let service = service.clone();
        thread::spawn(move || {
            let (mut submit, mut tasks) = build_single_task_submit(0x100);
            submit.task_array_cpu_address = tasks.as_mut_ptr() as u64;
            let result =
                service.driver_ioctl(RknpuCmd::Submit, (&mut submit as *mut RknpuSubmit) as usize);
            (result, submit, tasks)
        })
    };

    wait_until(|| service.has_inflight_dispatches());
    platform.publish_completion(&service, 0, 0x100);

    let (result, submit, tasks) = submitter.join().unwrap();
    assert_eq!(result.unwrap(), 0);
    assert_eq!(submit.task_counter, 1);
    // packed字段先复制到局部变量，避免断言宏创建未对齐引用。
    let int_status = tasks[0].int_status;
    assert_eq!(int_status, 0x100);
    assert_eq!(platform.spawn_count(), 1);
}

#[test]
fn irq_notification_wakes_blocked_worker() {
    // 最后修改日期：2026-08-17。
    // 先确认 worker 已派发任务并进入阻塞等待，再只发布完成状态而不通知。
    // Submit 此时不能自行完成；模拟 IRQ 发出通知后，worker 必须恢复并回收任务。
    let platform = MockPlatform::new();
    let service = RknpuService::new(platform.clone());
    let submitter = spawn_tagged_submit(service.clone(), 1, 0, 50_000, 0x1, vec![1]);

    wait_until(|| service.has_inflight_dispatches());
    wait_until(|| platform.worker_wait_count() > 0);

    platform.publish_completion_status(0, 0x100);
    thread::sleep(Duration::from_millis(10));
    assert!(
        !submitter.is_finished(),
        "未发送 IRQ 唤醒通知时，阻塞中的 worker 不应完成 Submit"
    );

    service.notify_irq_completion();
    join_submit(submitter);
}

#[test]
fn yield_polling_worker_completes_without_irq_wake() {
    // 最后修改日期：2026-08-19。显式选择 YieldPolling 后只发布硬件完成状态，
    // 不发送 Event 通知；Worker 必须通过 yield 轮询回收 Task 并结束 Submit。
    let platform = MockPlatform::new();
    let service = RknpuService::new_with_worker_wait_mode(
        platform.clone(),
        super::RknpuWorkerWaitMode::YieldPolling,
    );
    let submitter = spawn_tagged_submit(service.clone(), 1, 0, 50_000, 0x1, vec![1]);

    wait_until(|| service.has_inflight_dispatches());
    assert_eq!(platform.worker_wait_count(), 0);
    platform.publish_completion_status(0, 0x100);
    join_submit(submitter);
}

#[test]
fn submit_trace_reset_and_read_roundtrip() {
    // 先复位 trace，再完成一个 Submit 并读取记录。除数量外，还要求
    // t0<=t1<=t2<=t3<=t4；再次复位后读取必须为空。
    let platform = MockPlatform::new();
    let service = RknpuService::new(platform.clone());
    let mut reset = RknpuSubmitTraceQuery {
        operation: RKNPU_SUBMIT_TRACE_RESET,
        ..RknpuSubmitTraceQuery::default()
    };
    assert_eq!(
        service
            .driver_ioctl(
                RknpuCmd::SubmitTrace,
                (&mut reset as *mut RknpuSubmitTraceQuery) as usize,
            )
            .unwrap(),
        0
    );

    let submitter = {
        let service = service.clone();
        thread::spawn(move || {
            let (mut submit, mut tasks) = build_single_task_submit(0x100);
            submit.task_array_cpu_address = tasks.as_mut_ptr() as u64;
            service.driver_ioctl(
                RknpuCmd::Submit,
                (&mut submit as *mut RknpuSubmit) as usize,
            )
        })
    };
    wait_until(|| service.has_inflight_dispatches());
    platform.publish_completion(&service, 0, 0x100);
    assert_eq!(submitter.join().unwrap().unwrap(), 0);

    let mut records = [RknpuSubmitTraceRecord::default(); 1];
    let mut read = RknpuSubmitTraceQuery {
        operation: RKNPU_SUBMIT_TRACE_READ,
        capacity: records.len() as u32,
        records_address: records.as_mut_ptr() as u64,
        ..RknpuSubmitTraceQuery::default()
    };
    assert_eq!(
        service
            .driver_ioctl(
                RknpuCmd::SubmitTrace,
                (&mut read as *mut RknpuSubmitTraceQuery) as usize,
            )
            .unwrap(),
        0
    );
    assert_eq!(read.count, 1);
    assert_eq!(read.overflowed, 0);
    let record = records[0];
    assert_ne!(record.queue_task, 0);
    assert!(record.t0_ns <= record.t1_ns);
    assert!(record.t1_ns <= record.t2_ns);
    assert!(record.t2_ns <= record.t3_ns);
    assert!(record.t3_ns <= record.t4_ns);

    reset.operation = RKNPU_SUBMIT_TRACE_RESET;
    service
        .driver_ioctl(
            RknpuCmd::SubmitTrace,
            (&mut reset as *mut RknpuSubmitTraceQuery) as usize,
        )
        .unwrap();
    service
        .driver_ioctl(
            RknpuCmd::SubmitTrace,
            (&mut read as *mut RknpuSubmitTraceQuery) as usize,
        )
        .unwrap();
    assert_eq!(read.count, 0);
    assert_eq!(read.overflowed, 0);
}

#[test]
fn submit_trace_rejects_invalid_read_bounds() {
    // 超过固定上限的 capacity 不能触发过量分配；capacity 非零时也不能使用
    // 空记录地址。两类输入必须在 snapshot 和 copy-out 前返回 InvalidInput。
    let service = RknpuService::new(MockPlatform::new());
    let mut too_large = RknpuSubmitTraceQuery {
        operation: RKNPU_SUBMIT_TRACE_READ,
        capacity: RKNPU_SUBMIT_TRACE_CAPACITY as u32 + 1,
        ..RknpuSubmitTraceQuery::default()
    };
    assert_eq!(
        service.driver_ioctl(
            RknpuCmd::SubmitTrace,
            (&mut too_large as *mut RknpuSubmitTraceQuery) as usize,
        ),
        Err(RknpuServiceError::InvalidInput)
    );

    let mut null_records = RknpuSubmitTraceQuery {
        operation: RKNPU_SUBMIT_TRACE_READ,
        capacity: 1,
        ..RknpuSubmitTraceQuery::default()
    };
    assert_eq!(
        service.driver_ioctl(
            RknpuCmd::SubmitTrace,
            (&mut null_records as *mut RknpuSubmitTraceQuery) as usize,
        ),
        Err(RknpuServiceError::InvalidInput)
    );
}

#[test]
fn submit_trace_reports_fixed_buffer_overflow() {
    // 填满固定缓冲区后再追加一条。已有记录不能被覆盖，READ 必须返回固定
    // 数量并设置 overflowed，让用户态拒绝统计不完整实验。
    let service = RknpuService::new(MockPlatform::new());
    for queue_task in 1..=(RKNPU_SUBMIT_TRACE_CAPACITY as u64 + 1) {
        service.record_submit_trace(RknpuSubmitTraceRecord {
            queue_task,
            t0_ns: 1,
            t1_ns: 2,
            t2_ns: 3,
            t3_ns: 4,
            t4_ns: 5,
        });
    }

    let mut records = vec![RknpuSubmitTraceRecord::default(); RKNPU_SUBMIT_TRACE_CAPACITY];
    let mut read = RknpuSubmitTraceQuery {
        operation: RKNPU_SUBMIT_TRACE_READ,
        capacity: records.len() as u32,
        records_address: records.as_mut_ptr() as u64,
        ..RknpuSubmitTraceQuery::default()
    };
    service
        .driver_ioctl(
            RknpuCmd::SubmitTrace,
            (&mut read as *mut RknpuSubmitTraceQuery) as usize,
        )
        .unwrap();

    assert_eq!(read.count, RKNPU_SUBMIT_TRACE_CAPACITY as u32);
    assert_eq!(read.overflowed, 1);
    assert_eq!(records[0].queue_task, 1);
    assert_eq!(
        records[RKNPU_SUBMIT_TRACE_CAPACITY - 1].queue_task,
        RKNPU_SUBMIT_TRACE_CAPACITY as u64
    );
}

#[test]
fn schedule_trace_rejects_invalid_inputs() {
    // 该测试覆盖用户态可控的三个危险输入：未知事件位、越界容量和空地址。
    // 每个请求都必须在分配、读取调度状态或 copy-out 之前返回 InvalidInput。
    let service = RknpuService::new(MockPlatform::new());
    let mut bad_mask = RknpuScheduleTraceQuery {
        operation: RKNPU_SCHEDULE_TRACE_CONFIG_RESET,
        event_mask: 1 << 8,
        ..RknpuScheduleTraceQuery::default()
    };
    assert_eq!(
        service.driver_ioctl(
            RknpuCmd::ScheduleTrace,
            (&mut bad_mask as *mut RknpuScheduleTraceQuery) as usize,
        ),
        Err(RknpuServiceError::InvalidInput)
    );

    let mut too_large = RknpuScheduleTraceQuery {
        operation: RKNPU_SCHEDULE_TRACE_READ,
        capacity: crate::ioctrl::RKNPU_SCHEDULE_TRACE_CAPACITY as u32 + 1,
        records_address: 1,
        ..RknpuScheduleTraceQuery::default()
    };
    assert_eq!(
        service.driver_ioctl(
            RknpuCmd::ScheduleTrace,
            (&mut too_large as *mut RknpuScheduleTraceQuery) as usize,
        ),
        Err(RknpuServiceError::InvalidInput)
    );

    let mut null_records = RknpuScheduleTraceQuery {
        operation: RKNPU_SCHEDULE_TRACE_READ,
        capacity: 1,
        ..RknpuScheduleTraceQuery::default()
    };
    assert_eq!(
        service.driver_ioctl(
            RknpuCmd::ScheduleTrace,
            (&mut null_records as *mut RknpuScheduleTraceQuery) as usize,
        ),
        Err(RknpuServiceError::InvalidInput)
    );

    let mut null_state = RknpuScheduleTraceQuery {
        operation: RKNPU_SCHEDULE_TRACE_STATE,
        ..RknpuScheduleTraceQuery::default()
    };
    assert_eq!(
        service.driver_ioctl(
            RknpuCmd::ScheduleTrace,
            (&mut null_state as *mut RknpuScheduleTraceQuery) as usize,
        ),
        Err(RknpuServiceError::InvalidInput)
    );
}

#[test]
fn ready_submits_follow_priority_order() {
    // 场景构造：先用 blocker 占住唯一核心，再按低、普通、高的顺序入队。
    // 三个候选都停留在 Ready 后释放核心，预期实际下发顺序为高、普通、低。
    // 这样验证的是 Ready 桶排序，而不是线程启动顺序或偶然执行速度。
    let platform = MockPlatform::new();
    let service = RknpuService::new(platform.clone());
    configure_schedule_trace(&service);

    let blocker = spawn_tagged_submit(service.clone(), 1, 0, 100, 0x1, vec![1]);
    wait_until(|| has_schedule_event(&service, RKNPU_SCHEDULE_EVENT_DISPATCH, 100));

    let low = spawn_tagged_submit(service.clone(), 1, 10, 200, 0x1, vec![1]);
    wait_until(|| has_schedule_event(&service, RKNPU_SCHEDULE_EVENT_ENQUEUE, 200));
    let normal = spawn_tagged_submit(service.clone(), 1, 0, 300, 0x1, vec![1]);
    wait_until(|| has_schedule_event(&service, RKNPU_SCHEDULE_EVENT_ENQUEUE, 300));
    let high = spawn_tagged_submit(service.clone(), 1, -10, 400, 0x1, vec![1]);
    wait_until(|| has_schedule_event(&service, RKNPU_SCHEDULE_EVENT_ENQUEUE, 400));

    // 每观察到一个预期 Dispatch 才发布对应核心的完成 IRQ，逐步推进状态机，
    // 防止多个完成同时出现后掩盖 Ready 选择次序。
    platform.publish_completion(&service, dispatched_core(&service, 100), 0x100);
    wait_until(|| has_schedule_event(&service, RKNPU_SCHEDULE_EVENT_DISPATCH, 400));
    platform.publish_completion(&service, dispatched_core(&service, 400), 0x100);
    wait_until(|| has_schedule_event(&service, RKNPU_SCHEDULE_EVENT_DISPATCH, 300));
    platform.publish_completion(&service, dispatched_core(&service, 300), 0x100);
    wait_until(|| has_schedule_event(&service, RKNPU_SCHEDULE_EVENT_DISPATCH, 200));
    platform.publish_completion(&service, dispatched_core(&service, 200), 0x100);

    join_submit(blocker);
    join_submit(high);
    join_submit(normal);
    join_submit(low);

    // 排除 blocker 后，所有 Ready Submit 必须严格按 -10、0、10 下发。
    let dispatched = read_schedule_trace(&service)
        .iter()
        .filter(|record| {
            record.event_type == RKNPU_SCHEDULE_EVENT_DISPATCH && record.op_idx != 100
        })
        .map(|record| record.op_idx)
        .collect::<Vec<_>>();
    assert_eq!(dispatched, vec![400, 300, 200]);
}

#[test]
fn running_submit_precedes_ready_until_no_lane_can_dispatch() {
    // 低优先级 Submit A 在一个 lane 中包含两个 Task。高优先级 B 进入 Ready 后，
    // A 的首 Task 完成，调度器仍应先续派 A 的第二个 Task；A 无任务可续派后
    // 才能提升 B。最后修改日期：2026-08-07。
    let platform = MockPlatform::new();
    let service = RknpuService::new(platform.clone());
    configure_schedule_trace(&service);

    let running =
        spawn_tagged_submit(service.clone(), 2, 10, 1000, 0x1, vec![2]);
    wait_until(|| has_schedule_event(&service, RKNPU_SCHEDULE_EVENT_DISPATCH, 1000));

    let high_ready =
        spawn_tagged_submit(service.clone(), 1, -10, 2000, 0x1, vec![1]);
    wait_until(|| has_schedule_event(&service, RKNPU_SCHEDULE_EVENT_ENQUEUE, 2000));

    platform.publish_completion(&service, dispatched_core(&service, 1000), 0x100);
    wait_until(|| has_schedule_event(&service, RKNPU_SCHEDULE_EVENT_DISPATCH, 1001));
    // A 的第二个 Task 必须标记为 RUNNING 来源，同时记录等待中的最高 Ready=-10。
    let running_event = read_schedule_trace(&service)
        .into_iter()
        .find(|record| {
            record.event_type == RKNPU_SCHEDULE_EVENT_DISPATCH && record.op_idx == 1001
        })
        .unwrap();
    assert_eq!(running_event.dispatch_source, RKNPU_DISPATCH_SOURCE_RUNNING);
    assert_eq!(running_event.ready_priority, -10);
    assert!(!has_schedule_event(
        &service,
        RKNPU_SCHEDULE_EVENT_DISPATCH,
        2000
    ));

    platform.publish_completion(&service, dispatched_core(&service, 1001), 0x100);
    wait_until(|| has_schedule_event(&service, RKNPU_SCHEDULE_EVENT_DISPATCH, 2000));
    // A 暂时没有可续派 lane 后，B 必须从 READY 来源下发且自身就是最高优先级。
    let ready_event = read_schedule_trace(&service)
        .into_iter()
        .find(|record| {
            record.event_type == RKNPU_SCHEDULE_EVENT_DISPATCH && record.op_idx == 2000
        })
        .unwrap();
    assert_eq!(ready_event.dispatch_source, RKNPU_DISPATCH_SOURCE_READY);
    assert_eq!(ready_event.ready_priority, -10);
    platform.publish_completion(&service, dispatched_core(&service, 2000), 0x100);

    join_submit(high_ready);
    join_submit(running);
}

/// 用指定数量的并发 Submit 验证共享服务中的事件匹配和最终状态清理。
///
/// 每个线程提交一个单 Task Submit，所有 Submit 共用同一 Service，相当于板端
/// 共享 fd 的驱动状态。测试根据实际 Dispatch 记录向对应核心发布 IRQ，直到
/// 收到相同数量的 Complete，再检查：每个 op_idx 只下发和完成一次、事件序号
/// 连续、完成集合与下发集合一致、所有调度容器和 waiter 都已清空。
fn run_shared_submit_simulation(thread_count: usize) {
    let platform = MockPlatform::new();
    let service = RknpuService::new(platform.clone());
    configure_schedule_trace(&service);

    let handles = (0..thread_count)
        .map(|thread_id| {
            spawn_tagged_submit(
                service.clone(),
                1,
                0,
                10_000 + thread_id as u32,
                0x7,
                vec![1],
            )
        })
        .collect::<Vec<_>>();
    wait_until(|| {
        read_schedule_trace(&service)
            .iter()
            .filter(|record| record.event_type == RKNPU_SCHEDULE_EVENT_ENQUEUE)
            .count()
            == thread_count
    });

    // published 防止轮询同一快照时重复向一个 Task 发布完成 IRQ。
    let mut published = BTreeSet::new();
    wait_until(|| {
        let records = read_schedule_trace(&service);
        for record in records
            .iter()
            .filter(|record| record.event_type == RKNPU_SCHEDULE_EVENT_DISPATCH)
        {
            let key = (record.queue_task, record.task_index);
            if published.insert(key) {
                platform.publish_completion(&service, record.core_slot as usize, 0x100);
            }
        }
        records
            .iter()
            .filter(|record| record.event_type == RKNPU_SCHEDULE_EVENT_COMPLETE)
            .count()
            == thread_count
    });

    for handle in handles {
        join_submit(handle);
    }

    // sequence 检查固定缓冲区没有覆盖旧记录或产生重复序号。
    let records = read_schedule_trace(&service);
    for (index, record) in records.iter().enumerate() {
        assert_eq!(record.sequence, index as u64);
    }
    // 使用集合比较完成匹配；数量检查同时排除同一 op_idx 重复事件。
    let dispatched_tags = records
        .iter()
        .filter(|record| record.event_type == RKNPU_SCHEDULE_EVENT_DISPATCH)
        .map(|record| record.op_idx)
        .collect::<BTreeSet<_>>();
    let completed_tags = records
        .iter()
        .filter(|record| record.event_type == RKNPU_SCHEDULE_EVENT_COMPLETE)
        .map(|record| record.op_idx)
        .collect::<BTreeSet<_>>();
    assert_eq!(dispatched_tags.len(), thread_count);
    assert_eq!(completed_tags, dispatched_tags);

    // 所有 blocking ioctl 已 join，此时任何残留条目都表示生命周期清理错误。
    let mut state = RknpuSchedulerStateSnapshot::default();
    let mut query = RknpuScheduleTraceQuery {
        operation: RKNPU_SCHEDULE_TRACE_STATE,
        state_address: (&mut state as *mut RknpuSchedulerStateSnapshot) as u64,
        ..RknpuScheduleTraceQuery::default()
    };
    service
        .driver_ioctl(
            RknpuCmd::ScheduleTrace,
            (&mut query as *mut RknpuScheduleTraceQuery) as usize,
        )
        .unwrap();
    assert_eq!(state.live_submits, 0);
    assert_eq!(state.ready_entries, 0);
    assert_eq!(state.running_entries, 0);
    assert_eq!(state.complete_entries, 0);
    assert_eq!(state.waiters, 0);
    assert_eq!(state.core_bindings, 0);
    assert_eq!(state.gem_buffers, 0);
    assert_eq!(state.gem_bytes, 0);
}

#[test]
fn shared_submit_simulation_covers_three_and_six_threads() {
    // 分别覆盖目标实验的 3 线程和 6 线程规模，验证共享状态不会随并发度覆盖。
    run_shared_submit_simulation(3);
    run_shared_submit_simulation(6);
}

#[test]
fn submit_ioctl_reports_terminal_task_error() {
    // 发布与 Task int_mask 不相交的 IRQ，模拟非预期硬件中断。调度器必须
    // 结束 waiter 并向 ioctl 返回 TaskError，不能让 blocking 线程死锁。
    let platform = MockPlatform::new();
    let service = RknpuService::new(platform.clone());

    let submitter = {
        let service = service.clone();
        thread::spawn(move || {
            let (mut submit, mut tasks) = build_single_task_submit(0x100);
            submit.task_array_cpu_address = tasks.as_mut_ptr() as u64;
            let result =
                service.driver_ioctl(RknpuCmd::Submit, (&mut submit as *mut RknpuSubmit) as usize);
            (result, submit, tasks)
        })
    };

    wait_until(|| service.has_inflight_dispatches());
    platform.publish_completion(&service, 0, 0x200);

    let (result, submit, tasks) = submitter.join().unwrap();
    assert_eq!(
        result,
        Err(RknpuServiceError::Driver(RknpuError::TaskError))
    );
    assert_eq!(submit.task_counter, 1);
    // packed字段先复制到局部变量，避免断言宏创建未对齐引用。
    let int_status = tasks[0].int_status;
    assert_eq!(int_status, 0);
}

#[test]
fn worker_spawns_only_once_across_multiple_submits() {
    // 顺序完成两个 Submit，第二次入队必须复用现有 worker。若重复创建 worker，
    // 多个执行体可能竞争同一核心绑定并破坏调度状态的单一所有权。
    let platform = MockPlatform::new();
    let service = RknpuService::new(platform.clone());

    for _ in 0..2 {
        let submitter = {
            let service = service.clone();
            thread::spawn(move || {
                let (mut submit, mut tasks) = build_single_task_submit(0x100);
                submit.task_array_cpu_address = tasks.as_mut_ptr() as u64;
                let result = service
                    .driver_ioctl(RknpuCmd::Submit, (&mut submit as *mut RknpuSubmit) as usize);
                (result, submit, tasks)
            })
        };

        wait_until(|| service.has_inflight_dispatches());
        platform.publish_completion(&service, 0, 0x100);
        let (result, submit, tasks) = submitter.join().unwrap();
        assert_eq!(result.unwrap(), 0);
        assert_eq!(submit.task_counter, 1);
        // packed字段先复制到局部变量，避免断言宏创建未对齐引用。
        let int_status = tasks[0].int_status;
        assert_eq!(int_status, 0x100);
    }

    assert_eq!(platform.spawn_count(), 1);
}

#[test]
fn mem_and_action_ioctls_roundtrip() {
    // 依次验证 GEM 创建、mmap offset 查询、驱动 action 和 GEM 销毁，确保
    // 用户结构 copy-in/copy-out 与底层 DMA 对象生命周期保持一致。
    let platform = MockPlatform::new();
    let service = RknpuService::new(platform.clone());

    let mut mem_create = RknpuMemCreate {
        size: 0x1000,
        ..RknpuMemCreate::default()
    };
    assert_eq!(
        service
            .driver_ioctl(
                RknpuCmd::MemCreate,
                (&mut mem_create as *mut RknpuMemCreate) as usize,
            )
            .unwrap(),
        0
    );
    assert_ne!(mem_create.handle, 0);
    assert_ne!(mem_create.obj_addr, 0);

    let mut mem_map = RknpuMemMap {
        handle: mem_create.handle,
        ..RknpuMemMap::default()
    };
    assert_eq!(
        service
            .driver_ioctl(
                RknpuCmd::MemMap,
                (&mut mem_map as *mut RknpuMemMap) as usize,
            )
            .unwrap(),
        0
    );
    assert_eq!(mem_map.offset, (mem_create.handle as u64) << 12);

    let mut action = super::RknpuUserAction {
        flags: RknpuAction::GetDrvVersion as u32,
        value: 0,
    };
    assert_eq!(
        service
            .driver_ioctl(
                RknpuCmd::Action,
                (&mut action as *mut super::RknpuUserAction) as usize,
            )
            .unwrap(),
        0
    );
    assert_ne!(action.value, 0);

    let mut mem_destroy = RknpuMemDestroy {
        handle: mem_create.handle,
        obj_addr: mem_create.obj_addr,
        ..RknpuMemDestroy::default()
    };
    assert_eq!(
        service
            .driver_ioctl(
                RknpuCmd::MemDestroy,
                (&mut mem_destroy as *mut RknpuMemDestroy) as usize,
            )
            .unwrap(),
        0
    );

    let exists_after_destroy = platform
        .with_device(|dev| Ok(dev.get_phys_addr_and_size(mem_create.handle).is_some()))
        .unwrap();
    assert!(!exists_after_destroy);
}

#[test]
fn action_ioctl_rejects_unknown_opcode() {
    // 未定义 action 必须在命令解码边界返回 BadIoctl，不能到达底层设备。
    let platform = MockPlatform::new();
    let service = RknpuService::new(platform);

    let mut action = super::RknpuUserAction {
        flags: u32::MAX,
        value: 0,
    };

    assert_eq!(
        service.driver_ioctl(
            RknpuCmd::Action,
            (&mut action as *mut super::RknpuUserAction) as usize,
        ),
        Err(RknpuServiceError::BadIoctl)
    );
}
