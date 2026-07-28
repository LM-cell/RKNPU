use alloc::vec;
use core::{convert::TryFrom, mem};

use crate::{
    RknpuAction, RknpuQueuedSubmit, RknpuTask,
    ioctrl::{RknpuMemCreate, RknpuMemDestroy, RknpuMemMap, RknpuMemSync, RknpuSubmit},
};

use super::{RknpuPlatform, RknpuService, RknpuServiceError};

/// RKNPU driver-ioctl command numbers.
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
        let submit_args = self.copy_from_user::<RknpuSubmit>(arg)?;

        if submit_args.task_number == 0 || submit_args.task_array_cpu_address == 0 {
            debug!(
                "rknpu invalid submit header: task_number={}, task_array_cpu_address={:#x}, \
                 task_array_dma_address={:#x}",
                submit_args.task_number, submit_args.task_array_cpu_address, submit_args.task_array_dma_address,
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
        let queue_task_id =
            self.enqueue_submit(RknpuQueuedSubmit::new(submit_args.clone(), tasks))?;

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
