//! GEM-style memory pool for DMA buffer management.
//!
//! The name comes from the Linux DRM/GPU stack, where a GEM object is a buffer
//! identified by an integer handle. The NPU driver uses the same idea:
//! userspace asks the driver to allocate DMA-visible memory, receives an opaque
//! handle, and later references that handle in ioctl calls.
//!
//! # Role in the NPU workflow
//!
//! ```text
//!  Userspace (RKNN runtime)             Kernel / driver
//!  ───────────────────────             ───────────────
//!  1. ioctl(MEM_CREATE, size)  ──────►  GemPool::create()
//!     ◄── handle=1, dma_addr           allocate DVec and return handle + bus address
//!
//!  2. Write input tensors into the returned `obj_addr`
//!
//!  3. ioctl(SUBMIT, task list)  ──────►  GemPool::comfirm_write_all()
//!                                        flush CPU cache so the NPU sees the data
//!                                        ... NPU runs ...
//!                                        GemPool::prepare_read_all()
//!                                        invalidate CPU cache so results become visible
//!
//!  4. Read output tensors from the same buffer
//!
//!  5. ioctl(MEM_DESTROY, handle) ────►  GemPool::destroy()
//! ```
//!
//! Each allocation is a contiguous, page-aligned, DMA-capable region
//! (`DVec<u8>`) accessible both by the CPU and by the NPU.

use alloc::collections::btree_map::BTreeMap;
use core::mem::size_of;

use dma_api::{DVec, Direction};

use crate::{
    RknpuError,
    ioctrl::{RknpuMemCreate, RknpuMemSync},
};

/// Pool of DMA buffers indexed by opaque `u32` handles.
///
/// Handles start at 1 and increase monotonically. The pool owns every
/// allocation and frees the backing memory when [`destroy`](GemPool::destroy)
/// is called or when the pool itself is dropped.
pub struct GemPool {
    /// Mapping from handle to DMA buffer.
    pool: BTreeMap<u32, DVec<u8>>,
    /// Next handle to allocate.
    handle_counter: u32,
}

impl GemPool {
    /// Create an empty GEM pool.
    pub const fn new() -> Self {
        GemPool {
            pool: BTreeMap::new(),
            handle_counter: 1,
        }
    }

    /// Allocate a new DMA buffer and return its handle and addresses.
    ///
    /// On success the driver fills:
    ///
    /// - `handle`: unique identifier for the allocation
    /// - `dma_addr`: bus address used by the NPU
    /// - `obj_addr`: CPU virtual address used by software
    /// - `sram_size`: actual allocated size
    pub fn create(&mut self, args: &mut RknpuMemCreate) -> Result<(), RknpuError> {
        let data = DVec::zeros(
            u32::MAX as _,
            args.size as _,
            0x1000,
            Direction::Bidirectional,
        )
        .unwrap();

        let handle = self.handle_counter;
        self.handle_counter = self.handle_counter.wrapping_add(1);

        args.handle = handle;
        args.sram_size = data.len() as _;
        args.dma_addr = data.bus_addr();
        args.obj_addr = data.as_ptr() as _;
        self.pool.insert(args.handle, data);
        let active_buffers = self.pool.len();
        let active_bytes = self.pool.values().map(|buffer| buffer.len()).sum::<usize>();
        warn!(
            "[NPU] MEM_CREATE: handle={}, size={:#x}, dma_addr={:#x}, active_buffers={}, active_bytes={:#x}",
            handle, args.size, args.dma_addr, active_buffers, active_bytes
        );
        Ok(())
    }

    /// Look up a buffer by handle and return its bus address and byte length.
    pub fn get_phys_addr_and_size(&self, handle: u32) -> Option<(u64, usize)> {
        self.pool
            .get(&handle)
            .map(|dvec| (dvec.bus_addr(), dvec.len()))
    }

    /// Read a `u64` command stream slice from a GEM buffer by DMA address.
    ///
    /// The submit path uses this when it needs to inspect the current task's
    /// regcmd payload from kernel context without treating a DMA address as a
    /// CPU virtual pointer.
    pub fn copy_regcmd_words(
        &self,
        dma_addr: u64,
        byte_offset: usize,
        word_count: usize,
    ) -> Option<alloc::vec::Vec<u64>> {
        let start = dma_addr.checked_add(byte_offset as u64)?;
        let byte_len = word_count.checked_mul(size_of::<u64>())?;

        self.pool.values().find_map(|dvec| {
            let base = dvec.bus_addr();
            let len = dvec.len();
            let end = base.checked_add(len as u64)?;
            if start < base || start.checked_add(byte_len as u64)? > end {
                return None;
            }

            let start_offset = (start - base) as usize;
            let src = unsafe { dvec.as_ptr().add(start_offset) as *const u64 };
            let mut out = alloc::vec::Vec::with_capacity(word_count);
            for idx in 0..word_count {
                out.push(unsafe { src.add(idx).read_unaligned() });
            }
            Some(out)
        })
    }

    /// Synchronize a buffer range between the CPU and the device.
    ///
    /// This is currently a no-op because the DMA allocator returns coherent
    /// memory. On a non-coherent platform this would flush or invalidate cache
    /// lines for the requested byte range.
    pub fn sync(&mut self, _args: &mut RknpuMemSync) {}

    /// Free a DMA buffer by handle. The handle becomes invalid afterwards.
    pub fn destroy(&mut self, handle: u32) {
        let removed = self.pool.remove(&handle);
        let active_buffers = self.pool.len();
        let active_bytes = self.pool.values().map(|buffer| buffer.len()).sum::<usize>();
        warn!(
            "[NPU] MEM_DESTROY: handle={}, removed={}, active_buffers={}, active_bytes={:#x}",
            handle,
            removed.is_some(),
            active_buffers,
            active_bytes
        );
    }

    /// Flush CPU-side cache state for all buffers before the NPU reads them.
    pub fn comfirm_write_all(&mut self) -> Result<(), RknpuError> {
        for data in self.pool.values_mut() {
            data.confirm_write_all();
        }
        Ok(())
    }

    /// Invalidate CPU-side cache state for all buffers after the NPU finishes.
    pub fn prepare_read_all(&mut self) -> Result<(), RknpuError> {
        for data in self.pool.values_mut() {
            data.prepare_read_all();
        }
        Ok(())
    }
}

impl Default for GemPool {
    /// Create an empty pool via [`GemPool::new`].
    fn default() -> Self {
        Self::new()
    }
}
