use alloc::string::ToString;
use core::{
    any::Any,
    convert::TryFrom,
    ffi::{CStr, c_char, c_ulong},
    sync::atomic::{AtomicBool, Ordering},
};

use axerrno::AxError;
use axfs_ng_vfs::{DeviceId, NodeFlags, VfsError, VfsResult};
#[cfg(target_arch = "aarch64")]
use axtask::future::block_on;
use event_listener::Event;
use lazy_static::lazy_static;
use memory_addr::{MemoryAddr, PhysAddrRange};
use rknpu::service::{
    RknpuCmd, RknpuDeviceAccess, RknpuSchedulerRuntime, RknpuService, RknpuServiceError,
    RknpuSubmitWaiter, RknpuUserMemory, RknpuWorkerListener, RknpuWorkerSignal,
};
use starry_core::{
    futex::WaitQueue,
    vfs::{DeviceMmap, DeviceOps},
};

use crate::{
    card0::{copy_from_user, copy_to_user},
    drm::{DrmVersion, io_size, ioctl_nr, is_driver_ioctl},
};

/// Driver name for DRM device
const DRM1_NAME: &CStr = c"rknpu";
/// Driver date for DRM device
const DRM1_DATE: &CStr = c"20240828";
/// Driver description for DRM device
const DRM1_DESC: &CStr = c"RKNPU driver";

/// Device ID for /dev/dri/card1
pub const CARD1_SYSTEM_DEVICE_ID: DeviceId = DeviceId::new(0xe2, 1);

/// Device ID for /dev/rknpu (pick an unused major/minor)
pub const RKNPU_DEVICE_ID: DeviceId = DeviceId::new(251, 0);

/// Maximum ioctl command number
const MAX_IOCTL_NR: u32 = 0xcf;
/// Stack data buffer size
const STACK_DATA_SIZE: usize = 128;
/// DRM ioctl version command number
const DRM_IOCTL_VERSION_NR: u32 = 0;
/// DRM ioctl get unique command number
const DRM_IOCTL_GET_UNIQUE_NR: u32 = 1;
/// DRM ioctl gem flink command number
const DRM_IOCTL_GEM_FLINK_NR: u32 = 10;
/// DRM ioctl prime handle to fd command number
const DRM_IOCTL_PRIME_HANDLE_TO_FD_NR: u32 = 0x2d;

/// DRM_IOCTL_VERSION ioctl argument type
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct DrmUnique {
    /// Length of unique string identifier
    pub unique_len: c_ulong,
    /// Pointer to user-space buffer holding unique name for driver
    /// instantiation
    pub unique: *mut c_char,
}

const SUBMIT_WAIT_MASK: u32 = u32::MAX;

#[derive(Default, Clone, Copy)]
struct StarryPlatform;

struct StarrySubmitWaiter {
    done: AtomicBool,
    wait_queue: WaitQueue,
}

impl StarrySubmitWaiter {
    fn new() -> Self {
        Self {
            done: AtomicBool::new(false),
            wait_queue: WaitQueue::new(),
        }
    }
}

impl RknpuSubmitWaiter for StarrySubmitWaiter {
    fn wait(&self) -> Result<(), RknpuServiceError> {
        while !self.done.load(Ordering::Acquire) {
            self.wait_queue
                .wait_if(SUBMIT_WAIT_MASK, None, || {
                    !self.done.load(Ordering::Acquire)
                })
                .map_err(|err| match err {
                    AxError::Interrupted => RknpuServiceError::Interrupted,
                    _ => RknpuServiceError::InvalidData,
                })?;
        }
        Ok(())
    }

    fn complete(&self) {
        self.done.store(true, Ordering::Release);
        self.wait_queue.wake(usize::MAX, SUBMIT_WAIT_MASK);
    }
}

struct StarryWorkerSignal {
    event: Event,
}

struct StarryWorkerListener(event_listener::EventListener);

impl RknpuWorkerListener for StarryWorkerListener {
    fn wait(self) {
        #[cfg(target_arch = "aarch64")]
        let _ = block_on(self.0);
        #[cfg(not(target_arch = "aarch64"))]
        let _ = self.0;
    }
}

impl RknpuWorkerSignal for StarryWorkerSignal {
    type Listener = StarryWorkerListener;

    fn listen(&self) -> Self::Listener {
        StarryWorkerListener(self.event.listen())
    }

    fn notify_one(&self) {
        self.event.notify_relaxed(1);
    }
}

impl RknpuDeviceAccess for StarryPlatform {
    fn with_device<T, F>(&self, f: F) -> Result<T, RknpuServiceError>
    where
        F: FnOnce(&mut ::rknpu::Rknpu) -> Result<T, ::rknpu::RknpuError>,
    {
        let mut dev = npu().map_err(map_vfs_to_service_error)?;
        f(&mut dev).map_err(RknpuServiceError::from)
    }
}

impl RknpuUserMemory for StarryPlatform {
    fn copy_from_user(
        &self,
        dst: *mut u8,
        src: *const u8,
        size: usize,
    ) -> Result<(), RknpuServiceError> {
        copy_from_user(dst, src, size).map_err(|_| RknpuServiceError::InvalidData)
    }

    fn copy_to_user(
        &self,
        dst: *mut u8,
        src: *const u8,
        size: usize,
    ) -> Result<(), RknpuServiceError> {
        copy_to_user(dst, src, size).map_err(|_| RknpuServiceError::InvalidData)
    }
}

impl RknpuSchedulerRuntime for StarryPlatform {
    type Waiter = StarrySubmitWaiter;
    type WorkerSignal = StarryWorkerSignal;

    fn new_waiter(&self) -> Self::Waiter {
        StarrySubmitWaiter::new()
    }

    fn new_worker_signal(&self) -> Self::WorkerSignal {
        StarryWorkerSignal {
            event: Event::new(),
        }
    }

    fn spawn_worker<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        #[cfg(target_arch = "aarch64")]
        axtask::spawn(f, "rknpu-scheduler".to_string());
        #[cfg(not(target_arch = "aarch64"))]
        {
            let _ = f;
            warn!("rknpu scheduler worker is only supported on aarch64");
        }
    }

    fn yield_now(&self) {
        #[cfg(target_arch = "aarch64")]
        axtask::yield_now();
        #[cfg(not(target_arch = "aarch64"))]
        {}
    }
}

lazy_static! {
    static ref RKNPU_SERVICE: RknpuService<StarryPlatform> = RknpuService::new(StarryPlatform);
}

fn map_vfs_to_service_error(err: VfsError) -> RknpuServiceError {
    match err {
        VfsError::NotFound => RknpuServiceError::NotFound,
        VfsError::AddressInUse => RknpuServiceError::Busy,
        VfsError::BadIoctl => RknpuServiceError::BadIoctl,
        _ => RknpuServiceError::InvalidData,
    }
}

fn map_service_error(err: RknpuServiceError) -> VfsError {
    match err {
        RknpuServiceError::InvalidInput => VfsError::InvalidInput,
        RknpuServiceError::InvalidData => VfsError::InvalidData,
        RknpuServiceError::NotFound => VfsError::NotFound,
        RknpuServiceError::Busy => VfsError::AddressInUse,
        RknpuServiceError::Interrupted => AxError::Interrupted.into(),
        RknpuServiceError::BadIoctl => VfsError::BadIoctl,
        RknpuServiceError::Driver(_) | RknpuServiceError::Internal => VfsError::InvalidData,
    }
}

/// DRM card1 device implementation
pub struct Card1;

impl Card1 {
    /// Creates a new /dev/dri/card1 device.
    pub fn new() -> Card1 {
        Self
    }
}

impl Default for Card1 {
    fn default() -> Self {
        Self::new()
    }
}

impl DeviceOps for Card1 {
    /// Reads data from the device (not supported for card1)
    fn read_at(&self, _buf: &mut [u8], _offset: u64) -> VfsResult<usize> {
        trace!("dri: read_at called");
        // card1 heap devices are not meant to be read directly
        Err(VfsError::InvalidInput)
    }

    /// Writes data to the device (not supported for card1)
    fn write_at(&self, _buf: &[u8], _offset: u64) -> VfsResult<usize> {
        trace!("dri: write_at called");
        // card1 heap devices are not meant to be written directly
        Err(VfsError::InvalidInput)
    }

    /// Handles ioctl commands for the device
    fn ioctl(&self, cmd: u32, arg: usize) -> VfsResult<usize> {
        if arg == 0 {
            warn!("[rknpu]: ioctl received null arg pointer");
            return Err(VfsError::InvalidData);
        }
        let nr = ioctl_nr(cmd);
        info!("card1: cmd {cmd:#x}, nr {nr:#x}, arg {arg:#x}");

        let is_driver_ioctl = is_driver_ioctl(ioctl_nr(cmd));
        info!("card1: is_driver_ioctl = {}", is_driver_ioctl);

        if is_driver_ioctl {
            if let Ok(op) = RknpuCmd::try_from(nr) {
                rknpu_driver_ioctl(op, arg)?;
            } else {
                warn!("Unknown RKNPU cmd: {:#x}", cmd);
                return Err(VfsError::BadIoctl);
            }
        } else {
            assert!(nr <= MAX_IOCTL_NR, "card1: unsupported ioctl nr {nr}");
            let mut stack_data = [0u8; STACK_DATA_SIZE];

            let in_size = io_size(cmd) as usize;
            let out_size = in_size;

            copy_from_user(stack_data.as_mut_ptr(), arg as _, in_size)?;
            match nr {
                DRM_IOCTL_VERSION_NR => {
                    info!("drm get version");
                    drm_version(&mut stack_data)?;
                }
                DRM_IOCTL_GET_UNIQUE_NR => {
                    info!("drm get unique");
                    drm_get_unique(&mut stack_data)?;
                }
                DRM_IOCTL_GEM_FLINK_NR => {
                    drm_gem_flink_ioctl(&mut stack_data)?;
                }
                DRM_IOCTL_PRIME_HANDLE_TO_FD_NR => {
                    drm_prime_handle_to_fd_ioctl(&mut stack_data)?;
                }

                _ => {
                    panic!("card1: unsupported ioctl nr {nr:#x}");
                }
            }
            copy_to_user(arg as _, stack_data.as_mut_ptr(), out_size)?;
        }

        Ok(0)
    }

    /// Returns a reference to the object as Any for dynamic type checking
    fn as_any(&self) -> &dyn Any {
        self
    }

    /// Returns the node flags for the device
    fn flags(&self) -> NodeFlags {
        NodeFlags::NON_CACHEABLE
    }

    /// Maps device memory to user space
    fn mmap(&self, offset: u64) -> DeviceMmap {
        const PAGE_SHIFT: u32 = 12;
        const PAGE_SIZE: usize = 4096;

        let handle = (offset >> PAGE_SHIFT) as u32;

        with_npu(|rknpu_dev| {
            match rknpu_dev.get_phys_addr_and_size(handle) {
                Some((phys_addr, size)) => {
                    let range_size = if size < PAGE_SIZE {
                        PAGE_SIZE
                    } else {
                        size.align_up(PAGE_SIZE) // 向上对齐
                    };

                    info!(
                        "card1: mmap handle={}, phys={:#x}, orig_size={:#x}, range_size={:#x}",
                        handle, phys_addr, size, range_size
                    );

                    Ok(DeviceMmap::Physical(PhysAddrRange::new(
                        (phys_addr as usize).into(),
                        (phys_addr as usize + range_size).into(),
                    )))
                }
                None => {
                    warn!("card1: mmap invalid handle={}", handle);
                    Ok(DeviceMmap::None)
                }
            }
        })
        .unwrap_or(DeviceMmap::None)
    }
}

/// Gets a reference to the NPU device
pub fn npu() -> Result<rdrive::DeviceGuard<::rknpu::Rknpu>, VfsError> {
    rdrive::get_one()
        .ok_or(VfsError::NotFound)?
        .lock()
        .map_err(|_| VfsError::AddressInUse)
}

/// Executes a function with the NPU device
pub fn with_npu<F, R>(f: F) -> Result<R, VfsError>
where
    F: FnOnce(&mut ::rknpu::Rknpu) -> Result<R, VfsError>,
{
    let mut npu = npu()?;
    f(&mut npu)
}

/// Handles RKNPU action ioctl commands
pub fn rknpu_driver_ioctl(op: RknpuCmd, arg: usize) -> VfsResult<usize> {
    info!("rknpu_driver_ioctl: op = {:?}", op);
    RKNPU_SERVICE.driver_ioctl(op, arg).map_err(|err| {
        warn!("rknpu driver ioctl failed: {:?}", err);
        map_service_error(err)
    })
}

/// DRM_IOCTL_GEM_FLINK ioctl argument type
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
struct DrmGemFlink {
    /// GEM handle
    handle: u32,
    /// GEM name
    name: u32,
}

/// Handles DRM GEM flink ioctl command
fn drm_gem_flink_ioctl(data: &mut [u8]) -> VfsResult<usize> {
    let data = unsafe { &mut *(data.as_mut_ptr() as *mut DrmGemFlink) };
    info!("drm_gem_flink_ioctl called: {:#?}", data);
    Err(VfsError::NotFound)
}

/// DRM prime handle structure
#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct DrmPrimeHande {
    /// Handle
    handle: u32,
    /// Flags
    flags: u32,
    /// File descriptor
    fd: i32,
}

/// Handles DRM prime handle to fd ioctl command
fn drm_prime_handle_to_fd_ioctl(data: &mut [u8]) -> VfsResult<usize> {
    let data = unsafe { &mut *(data.as_mut_ptr() as *mut DrmPrimeHande) };
    info!("drm_prime_handle_to_fd_ioctl {data:#x?}");
    data.fd = 1; // 返回一个假的 fd
    Ok(0)
}

/// Rust implementation of Linux kernel's drm_copy_field function
///
/// This function safely copies a string value to user space buffer,
/// similar to the Linux kernel implementation with proper error handling.
unsafe fn drm_copy_field(
    buf: *mut u8,
    buf_len: &mut c_ulong,
    value: *const u8,
) -> VfsResult<()> {
    // Handle NULL value case - same as kernel's WARN_ONCE check
    if value.is_null() {
        warn!("[drm_copy_field] BUG: the value to copy was not set!");
        *buf_len = 0;
        return Ok(());
    }

    // Calculate actual string length using C string semantics
    let mut len = 0;
    unsafe {
        let mut ptr = value;
        while *ptr != 0 {
            len += 1;
            ptr = ptr.add(1);
        }
    }

    // Get the original buffer size
    let original_buf_len = *buf_len;

    // Update user's buffer length with actual string length (same as kernel)
    *buf_len = len;

    // Don't overflow user buffer - limit copy to available space
    let copy_len = if len > original_buf_len {
        original_buf_len
    } else {
        len
    };

    // Finally, try filling in the userbuf (same logic as kernel)
    if copy_len > 0 && !buf.is_null() {
        copy_to_user(buf as _, value, copy_len as _)?;
    }

    Ok(())
}

/// Sets the DRM version information for the device
pub fn drm_version(data: &mut [u8]) -> VfsResult<()> {
    let data = unsafe { &mut *(data.as_mut_ptr() as *mut DrmVersion) };
    info!("drm_version called: {:?}", data);

    // Set version information
    data.version_major = 0;
    data.version_minor = 9;
    data.version_patchlevel = 8;

    // Use drm_copy_field to handle string copying properly
    unsafe {
        // Copy driver name
        let ret = drm_copy_field(data.name, &mut data.name_len, DRM1_NAME.as_ptr());
        if let Err(e) = ret {
            warn!("[drm_version] Failed to copy driver name: {:?}", e);
            return Err(VfsError::InvalidData);
        }

        // Copy driver date
        let ret = drm_copy_field(
            data.date as *mut u8,
            &mut data.date_len,
            DRM1_DATE.as_ptr() as *const u8,
        );
        if let Err(e) = ret {
            warn!("[drm_version] Failed to copy driver date: {:?}", e);
            return Err(VfsError::InvalidData);
        }

        // Copy driver description
        let ret = drm_copy_field(data.desc, &mut data.desc_len, DRM1_DESC.as_ptr());
        if let Err(e) = ret {
            warn!("[drm_version] Failed to copy driver description: {:?}", e);
            return Err(VfsError::InvalidData);
        }
    }

    info!(
        "[drm_version] Set driver info: name_len={}, date_len={}, desc_len={}",
        data.name_len, data.date_len, data.desc_len
    );

    Ok(())
}

/// DRM_GET_UNIQUE ioctl handler
///
/// This function handles DRM_IOCTL_GET_UNIQUE requests, returning the unique
/// identifier for the DRM device (typically a bus ID or similar identifier).
pub fn drm_get_unique(data: &mut [u8]) -> VfsResult<()> {
    let unique_data = unsafe { &mut *(data.as_mut_ptr() as *mut DrmUnique) };
    info!("drm_get_unique called: {:?}", unique_data);

    unique_data.unique_len = 0;

    Ok(())
}

/// DRM_SET_UNIQUE ioctl handler (stub implementation)
///
/// This function handles DRM_IOCTL_SET_UNIQUE requests. For this
/// implementation, we return success but don't actually set the unique
/// identifier, as this is typically not used/needed in embedded systems.
pub fn drm_set_unique(data: &mut [u8]) -> VfsResult<()> {
    let unique_data = unsafe { &*(data.as_ptr() as *const DrmUnique) };
    info!("drm_set_unique called: {:?}", unique_data);

    // For this implementation, we just log the attempt and return success
    // In a real implementation, this would validate and store the unique ID
    warn!("[drm_set_unique] Setting unique identifier is not supported in this implementation");

    Ok(())
}
