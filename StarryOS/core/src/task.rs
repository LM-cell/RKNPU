//! User task management.

mod stat;
#[path = "npu_context.rs"]
mod npu_context;

use alloc::{
    boxed::Box,
    string::String,
    sync::{Arc, Weak},
    vec::Vec,
};
use core::{
    alloc::Layout,
    cell::RefCell,
    ops::Deref,
    sync::atomic::{AtomicBool, AtomicI32, AtomicU32, AtomicUsize, Ordering},
};

use axdma::{DMAInfo, dealloc_coherent};
use axerrno::{AxError, AxResult};
use axmm::AddrSpace;
use axpoll::PollSet;
use axsync::{Mutex, spin::SpinNoIrq};
use axtask::{AxTaskRef, TaskExt, TaskInner, WeakAxTaskRef, current};
use extern_trait::extern_trait;
use hashbrown::HashMap;
use lazy_static::lazy_static;
use memory_addr::VirtAddr;
use scope_local::{ActiveScope, Scope};
use spin::RwLock;
use starry_process::{Pid, Process, ProcessGroup, Session};
use starry_signal::{
    SignalInfo, Signo,
    api::{ProcessSignalManager, SignalActions, ThreadSignalManager},
};
use weak_map::WeakMap;

pub use self::npu_context::{NpuContext, NpuContextError};
pub use self::stat::TaskStat;
use crate::{
    futex::{FutexKey, FutexTable},
    resources::Rlimits,
    time::{TimeManager, TimerState},
};

/// A wrapper type that assumes the inner type is `Sync`.
#[repr(transparent)]
pub struct AssumeSync<T>(pub T);

unsafe impl<T> Sync for AssumeSync<T> {}

impl<T> Deref for AssumeSync<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// The inner data of a thread.
pub struct ThreadInner {
    /// The process data shared by all threads in the process.
    pub proc_data: Arc<ProcessData>,

    /// The clear thread tid field.
    ///
    /// See <https://manpages.debian.org/unstable/manpages-dev/set_tid_address.2.en.html#clear_child_tid>.
    clear_child_tid: AtomicUsize,

    /// The head of the robust list.
    robust_list_head: AtomicUsize,

    /// The registered rseq area pointer (user address) for restartable sequences.
    rseq_area: AtomicUsize,

    /// The thread-level signal manager.
    pub signal: Arc<ThreadSignalManager>,

    /// Time manager.
    ///
    /// This is assumed to be `Sync` because it's only borrowed mutably during
    /// context switches, which is exclusive to the current thread.
    pub time: AssumeSync<RefCell<TimeManager>>,

    /// The OOM score adjustment value.
    oom_score_adj: AtomicI32,

    /// Ready to exit.
    exit: AtomicBool,

    /// 为后续 NPU 抢占/恢复预留的线程级上下文。
    pub npu_context: Mutex<NpuContext>,
}

impl ThreadInner {
    /// Create a new [`ThreadInner`].
    pub fn new(tid: u32, proc_data: Arc<ProcessData>) -> Self {
        ThreadInner {
            signal: ThreadSignalManager::new(tid, proc_data.signal.clone()),
            proc_data,
            clear_child_tid: AtomicUsize::new(0),
            robust_list_head: AtomicUsize::new(0),
            rseq_area: AtomicUsize::new(0),
            time: AssumeSync(RefCell::new(TimeManager::new())),
            oom_score_adj: AtomicI32::new(200),
            exit: AtomicBool::new(false),
            npu_context: Mutex::new(NpuContext::new()),
        }
    }

    /// Get the clear child tid field.
    pub fn clear_child_tid(&self) -> usize {
        self.clear_child_tid.load(Ordering::Relaxed)
    }

    /// Set the clear child tid field.
    pub fn set_clear_child_tid(&self, clear_child_tid: usize) {
        self.clear_child_tid
            .store(clear_child_tid, Ordering::Relaxed);
    }

    /// Get the robust list head.
    pub fn robust_list_head(&self) -> usize {
        self.robust_list_head.load(Ordering::SeqCst)
    }

    /// Set the robust list head.
    pub fn set_robust_list_head(&self, robust_list_head: usize) {
        self.robust_list_head
            .store(robust_list_head, Ordering::SeqCst);
    }

    /// Get the registered rseq area pointer.
    pub fn rseq_area(&self) -> usize {
        self.rseq_area.load(Ordering::SeqCst)
    }

    /// Set the registered rseq area pointer.
    pub fn set_rseq_area(&self, addr: usize) {
        self.rseq_area.store(addr, Ordering::SeqCst);
    }

    /// Get the oom score adjustment value.
    pub fn oom_score_adj(&self) -> i32 {
        self.oom_score_adj.load(Ordering::SeqCst)
    }

    /// Set the oom score adjustment value.
    pub fn set_oom_score_adj(&self, value: i32) {
        self.oom_score_adj.store(value, Ordering::SeqCst);
    }

    /// Check if the thread is ready to exit.
    pub fn pending_exit(&self) -> bool {
        self.exit.load(Ordering::Acquire)
    }

    /// Set the thread to exit.
    pub fn set_exit(&self) {
        self.exit.store(true, Ordering::Release);
    }
}

/// Extended thread data for the monolithic kernel.
pub struct Thread(Box<ThreadInner>);

impl Deref for Thread {
    type Target = ThreadInner;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[extern_trait]
unsafe impl TaskExt for Thread {
    fn on_enter(&self) {
        let scope = self.proc_data.scope.read();
        unsafe { ActiveScope::set(&scope) };
        core::mem::forget(scope);
    }

    fn on_leave(&self) {
        ActiveScope::set_global();
        unsafe { self.proc_data.scope.force_read_decrement() };
    }
}

/// Helper trait to access the thread from a task.
pub trait AsThread {
    /// Try to get the thread from the task.
    fn try_as_thread(&self) -> Option<&Thread>;

    /// Get the thread from the task, panicking if it is a kernel task.
    fn as_thread(&self) -> &Thread {
        self.try_as_thread().expect("kernel task")
    }
}

impl AsThread for TaskInner {
    fn try_as_thread(&self) -> Option<&Thread> {
        self.task_ext().map(|ext| unsafe { ext.downcast_ref() })
    }
}

impl Thread {
    /// Create a new [`Thread`].
    pub fn new(tid: u32, proc_data: Arc<ProcessData>) -> Self {
        Self(Box::new(ThreadInner::new(tid, proc_data)))
    }
}

#[derive(Debug, Clone, Copy)]
struct DmaAllocation {
    user_vaddr: usize,
    map_size: usize,
    layout: Layout,
    dma_info: DMAInfo,
}

unsafe impl Send for DmaAllocation{}
unsafe impl Sync for DmaAllocation{}

/// [`Process`]-shared data.
pub struct ProcessData {
    /// The process.
    pub proc: Arc<Process>,
    /// The executable path.
    pub exe_path: RwLock<String>,
    /// The command line arguments.
    pub cmdline: RwLock<Arc<Vec<String>>>,
    /// The virtual memory address space.
    pub aspace: Arc<Mutex<AddrSpace>>,
    /// The resource scope.
    pub scope: RwLock<Scope>,
    /// The user heap bottom.
    heap_bottom: AtomicUsize,
    /// The user heap top.
    heap_top: AtomicUsize,

    /// The resource limits.
    pub rlim: RwLock<Rlimits>,

    /// The child exit wait event.
    pub child_exit_event: Arc<PollSet>,
    /// Self exit event.
    pub exit_event: Arc<PollSet>,
    /// The exit signal of the thread.
    pub exit_signal: Option<Signo>,

    /// The process signal manager.
    pub signal: Arc<ProcessSignalManager>,

    /// The futex table.
    futex_table: Arc<FutexTable>,

    /// The default mask for file permissions.
    umask: AtomicU32,

    /// 为后续 NPU 上下文切换预留的脏位。
    npu_isdirty: AtomicBool,
    /// 记录当前进程通过 `dma_malloc` 建立的 DMA 映射。
    ///
    /// 这里同时保存三类地址：
    /// - `user_vaddr`: 返回给用户态的虚拟地址
    /// - `dma_info.cpu_addr`: 内核访问 coherent DMA 时使用的地址
    /// - `dma_info.bus_addr`: 设备访问这段内存时使用的 DMA/bus 地址
    ///
    /// 当前还没有 `dma_free` syscall，因此只在进程退出时统一回收。
    dma_allocations: Mutex<Vec<DmaAllocation>>,
}

impl ProcessData {
    /// Create a new [`ProcessData`].
    pub fn new(
        proc: Arc<Process>,
        exe_path: String,
        cmdline: Arc<Vec<String>>,
        aspace: Arc<Mutex<AddrSpace>>,
        signal_actions: Arc<SpinNoIrq<SignalActions>>,
        exit_signal: Option<Signo>,
    ) -> Arc<Self> {
        Arc::new(Self {
            proc,
            exe_path: RwLock::new(exe_path),
            cmdline: RwLock::new(cmdline),
            aspace,
            scope: RwLock::new(Scope::new()),
            heap_bottom: AtomicUsize::new(crate::config::USER_HEAP_BASE),
            heap_top: AtomicUsize::new(crate::config::USER_HEAP_BASE),

            rlim: RwLock::default(),

            child_exit_event: Arc::default(),
            exit_event: Arc::default(),
            exit_signal,

            signal: Arc::new(ProcessSignalManager::new(
                signal_actions,
                crate::config::SIGNAL_TRAMPOLINE,
            )),

            futex_table: Arc::new(FutexTable::new()),

            umask: AtomicU32::new(0o022),
            npu_isdirty: AtomicBool::new(false),
            dma_allocations: Mutex::new(Vec::new()),
        })
    }

    /// Get the bottom address of the user heap.
    pub fn get_heap_bottom(&self) -> usize {
        self.heap_bottom.load(Ordering::Acquire)
    }

    /// Set the bottom address of the user heap.
    pub fn set_heap_bottom(&self, bottom: usize) {
        self.heap_bottom.store(bottom, Ordering::Release)
    }

    /// Get the top address of the user heap.
    pub fn get_heap_top(&self) -> usize {
        self.heap_top.load(Ordering::Acquire)
    }

    /// Set the top address of the user heap.
    pub fn set_heap_top(&self, top: usize) {
        self.heap_top.store(top, Ordering::Release)
    }

    /// Linux manual: A "clone" child is one which delivers no signal, or a
    /// signal other than SIGCHLD to its parent upon termination.
    pub fn is_clone_child(&self) -> bool {
        self.exit_signal != Some(Signo::SIGCHLD)
    }

    /// Returns the futex table for the given key.
    pub fn futex_table_for(&self, key: &FutexKey) -> Arc<FutexTable> {
        match key {
            FutexKey::Private { .. } => self.futex_table.clone(),
            FutexKey::Shared { region, .. } => {
                let ptr = match region {
                    Ok(pages) => Weak::as_ptr(pages) as usize,
                    Err(key) => Weak::as_ptr(key) as usize,
                };
                SHARED_FUTEX_TABLES.lock().get_or_insert(ptr)
            }
        }
    }

    /// Get the umask.
    pub fn umask(&self) -> u32 {
        self.umask.load(Ordering::SeqCst)
    }

    /// Set the umask.
    pub fn set_umask(&self, umask: u32) {
        self.umask.store(umask, Ordering::SeqCst);
    }

    /// Set the umask and return the old value.
    pub fn replace_umask(&self, umask: u32) -> u32 {
        self.umask.swap(umask, Ordering::SeqCst)
    }

    /// 记录一次 DMA 分配，供进程退出时统一回收。
    pub fn record_dma_allocation(
        &self,
        user_vaddr: usize,
        map_size: usize,
        layout: Layout,
        dma_info: DMAInfo,
    ) {
        self.dma_allocations.lock().push(DmaAllocation {
            user_vaddr,
            map_size,
            layout,
            dma_info,
        });
    }

    /// 释放一条由 `dma_malloc` 建立的 DMA 映射。
    ///
    /// 参数 `user_vaddr` 必须是 `dma_malloc` 原样返回的用户虚拟地址：
    /// - 不能传入偏移后的地址
    /// - 不能跨进程释放
    ///
    /// 返回值：
    /// - `Ok(true)`: 成功找到并释放
    /// - `Ok(false)`: 当前进程下没有这条记录
    /// - `Err(_)`: 解除用户态映射失败，此时会把记录放回表中，避免丢失回收信息
    pub fn release_dma_allocation(&self, user_vaddr: usize) -> AxResult<bool> {
        let allocation = {
            let mut allocations = self.dma_allocations.lock();
            let Some(index) = allocations
                .iter()
                .position(|allocation| allocation.user_vaddr == user_vaddr)
            else {
                return Ok(false);
            };
            allocations.remove(index)
        };

        let unmap_result = {
            let mut aspace = self.aspace.lock();
            aspace.unmap(VirtAddr::from(allocation.user_vaddr), allocation.map_size)
        };

        if let Err(err) = unmap_result {
            self.dma_allocations.lock().push(allocation);
            return Err(err);
        }

        unsafe {
            dealloc_coherent(allocation.dma_info, allocation.layout);
        }

        Ok(true)
    }

    /// 返回当前进程预留的 NPU 脏位。
    pub fn npu_isdirty(&self) -> bool {
        self.npu_isdirty.load(Ordering::Acquire)
    }

    /// 更新当前进程预留的 NPU 脏位。
    pub fn set_npu_isdirty(&self, value: bool) {
        self.npu_isdirty.store(value, Ordering::Release);
    }
}

impl Drop for ProcessData {
    fn drop(&mut self) {
        let allocations = core::mem::take(&mut *self.dma_allocations.lock());
        if allocations.is_empty() {
            return;
        }

        {
            let mut aspace = self.aspace.lock();
            for allocation in &allocations {
                if let Err(err) =
                    aspace.unmap(VirtAddr::from(allocation.user_vaddr), allocation.map_size)
                {
                    warn!(
                        "failed to unmap DMA memory for process {} at {:#x}: {:?}",
                        self.proc.pid(),
                        allocation.user_vaddr,
                        err
                    );
                }
            }
        }

        for allocation in allocations {
            // 先撤销用户空间映射，再归还 coherent DMA 内存。
            unsafe {
                dealloc_coherent(allocation.dma_info, allocation.layout);
            }
        }
    }
}

struct FutexTables {
    map: HashMap<usize, Arc<FutexTable>>,
    operations: usize,
}

impl FutexTables {
    fn new() -> Self {
        Self {
            map: HashMap::new(),
            operations: 0,
        }
    }

    fn get_or_insert(&mut self, key: usize) -> Arc<FutexTable> {
        self.operations += 1;
        if self.operations == 100 {
            self.operations = 0;
            self.map
                .retain(|_, table| Arc::strong_count(table) > 1 || !table.is_empty());
        }
        self.map
            .entry(key)
            .or_insert_with(|| Arc::new(FutexTable::new()))
            .clone()
    }
}

lazy_static! {
    static ref SHARED_FUTEX_TABLES: Mutex<FutexTables> = Mutex::new(FutexTables::new());
}

static TASK_TABLE: RwLock<WeakMap<Pid, WeakAxTaskRef>> = RwLock::new(WeakMap::new());

static PROCESS_TABLE: RwLock<WeakMap<Pid, Weak<ProcessData>>> = RwLock::new(WeakMap::new());

static PROCESS_GROUP_TABLE: RwLock<WeakMap<Pid, Weak<ProcessGroup>>> = RwLock::new(WeakMap::new());

static SESSION_TABLE: RwLock<WeakMap<Pid, Weak<Session>>> = RwLock::new(WeakMap::new());

/// Cleanup expired entries in the task tables.
///
/// This function is intended to be used during memory leak analysis to remove
/// possible noise caused by expired entries in the [`WeakMap`].
pub fn cleanup_task_tables() {
    TASK_TABLE.write().cleanup();
    PROCESS_TABLE.write().cleanup();
    PROCESS_GROUP_TABLE.write().cleanup();
    SESSION_TABLE.write().cleanup();
}

/// Add the task, the thread and possibly its process, process group and session
/// to the corresponding tables.
pub fn add_task_to_table(task: &AxTaskRef) {
    let tid = task.id().as_u64() as Pid;

    let mut task_table = TASK_TABLE.write();
    task_table.insert(tid, task);

    let proc_data = &task.as_thread().proc_data;
    let proc = &proc_data.proc;
    let pid = proc.pid();
    let mut proc_table = PROCESS_TABLE.write();
    if proc_table.contains_key(&pid) {
        return;
    }
    proc_table.insert(pid, proc_data);

    let pg = proc.group();
    let mut pg_table = PROCESS_GROUP_TABLE.write();
    if pg_table.contains_key(&pg.pgid()) {
        return;
    }
    pg_table.insert(pg.pgid(), &pg);

    let session = pg.session();
    let mut session_table = SESSION_TABLE.write();
    if session_table.contains_key(&session.sid()) {
        return;
    }
    session_table.insert(session.sid(), &session);
}

/// Lists all tasks.
pub fn tasks() -> Vec<AxTaskRef> {
    TASK_TABLE.read().values().collect()
}

/// Finds the task with the given TID.
pub fn get_task(tid: Pid) -> AxResult<AxTaskRef> {
    if tid == 0 {
        return Ok(current().clone());
    }
    TASK_TABLE.read().get(&tid).ok_or(AxError::NoSuchProcess)
}

/// Lists all processes.
pub fn processes() -> Vec<Arc<ProcessData>> {
    PROCESS_TABLE.read().values().collect()
}

/// Finds the process with the given PID.
pub fn get_process_data(pid: Pid) -> AxResult<Arc<ProcessData>> {
    if pid == 0 {
        return Ok(current().as_thread().proc_data.clone());
    }
    PROCESS_TABLE.read().get(&pid).ok_or(AxError::NoSuchProcess)
}

/// Finds the process group with the given PGID.
pub fn get_process_group(pgid: Pid) -> AxResult<Arc<ProcessGroup>> {
    PROCESS_GROUP_TABLE
        .read()
        .get(&pgid)
        .ok_or(AxError::NoSuchProcess)
}

/// Finds the session with the given SID.
pub fn get_session(sid: Pid) -> AxResult<Arc<Session>> {
    SESSION_TABLE.read().get(&sid).ok_or(AxError::NoSuchProcess)
}

/// Poll the timer.
pub fn poll_timer(task: &TaskInner) {
    let Some(thr) = task.try_as_thread() else {
        return;
    };
    let Ok(mut time) = thr.time.try_borrow_mut() else {
        // reentrant borrow, likely IRQ
        return;
    };
    time.poll(|signo| {
        send_signal_thread_inner(task, thr, SignalInfo::new_kernel(signo));
    });
}

/// Sets the timer state.
pub fn set_timer_state(task: &TaskInner, state: TimerState) {
    let Some(thr) = task.try_as_thread() else {
        return;
    };
    let Ok(mut time) = thr.time.try_borrow_mut() else {
        // reentrant borrow, likely IRQ
        return;
    };
    time.poll(|signo| {
        send_signal_thread_inner(task, thr, SignalInfo::new_kernel(signo));
    });
    time.set_state(state);
}

fn send_signal_thread_inner(task: &TaskInner, thr: &Thread, sig: SignalInfo) {
    if thr.signal.send_signal(sig) {
        task.interrupt();
    }
}

/// Sends a signal to a thread.
pub fn send_signal_to_thread(tgid: Option<Pid>, tid: Pid, sig: Option<SignalInfo>) -> AxResult<()> {
    let task = get_task(tid)?;
    let thread = task.try_as_thread().ok_or(AxError::OperationNotPermitted)?;
    if tgid.is_some_and(|tgid| thread.proc_data.proc.pid() != tgid) {
        return Err(AxError::NoSuchProcess);
    }

    if let Some(sig) = sig {
        info!("Send signal {:?} to thread {}", sig.signo(), tid);
        send_signal_thread_inner(&task, thread, sig);
    }

    Ok(())
}

/// Sends a signal to a process.
pub fn send_signal_to_process(pid: Pid, sig: Option<SignalInfo>) -> AxResult<()> {
    let proc_data = get_process_data(pid)?;

    if let Some(sig) = sig {
        let signo = sig.signo();
        info!("Send signal {:?} to process {}", signo, pid);
        if let Some(tid) = proc_data.signal.send_signal(sig)
            && let Ok(task) = get_task(tid)
        {
            task.interrupt();
        }
    }

    Ok(())
}

/// Sends a signal to a process group.
pub fn send_signal_to_process_group(pgid: Pid, sig: Option<SignalInfo>) -> AxResult<()> {
    let pg = get_process_group(pgid)?;

    if let Some(sig) = sig {
        info!("Send signal {:?} to process group {}", sig.signo(), pgid);
        for proc in pg.processes() {
            send_signal_to_process(proc.pid(), Some(sig.clone()))?;
        }
    }

    Ok(())
}
