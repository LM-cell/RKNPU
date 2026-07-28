use core::alloc::Layout;

use axdma::{DMAInfo, alloc_coherent, dealloc_coherent};
use axerrno::{AxError, AxResult};
use axhal::{mem::virt_to_phys, paging::MappingFlags};
use axtask::current;
use memory_addr::{MemoryAddr, PAGE_SIZE_4K, VirtAddr, VirtAddrRange};
use starry_core::task::AsThread;

use crate::mm::UserPtr;

fn aligned_dma_size(size: usize) -> AxResult<usize> {
    if size == 0 {
        return Err(AxError::InvalidInput);
    }

    size.checked_add(PAGE_SIZE_4K - 1)
        .map(|value| value & !(PAGE_SIZE_4K - 1))
        .ok_or(AxError::NoMemory)
}

fn cleanup_dma_mapping(user_start: VirtAddr, map_size: usize, layout: Layout, dma_info: DMAInfo) {
    let curr = current();
    let proc_data = &curr.as_thread().proc_data;

    if let Err(err) = proc_data.aspace.lock().unmap(user_start, map_size) {
        warn!(
            "failed to rollback DMA user mapping at {:#x}: {:?}",
            user_start.as_usize(),
            err
        );
    }

    unsafe {
        dealloc_coherent(dma_info, layout);
    }
}

/// 为用户态分配一段连续 coherent DMA 内存。
///
/// 返回值是用户态虚拟地址，第二个参数回填设备访问这段内存时使用的 DMA/bus
/// 地址。
pub fn sys_dma_malloc(size: usize, dma_addr_out: usize) -> AxResult<isize> {
    let map_size = aligned_dma_size(size)?;
    let layout =
        Layout::from_size_align(map_size, PAGE_SIZE_4K).map_err(|_| AxError::InvalidInput)?;

    // coherent DMA 分配器返回的是内核可访问虚拟地址和 DMA/bus 地址。
    let dma_info = unsafe { alloc_coherent(layout) }.map_err(|_| AxError::NoMemory)?;

    // 用户态映射需要用物理地址，而不是 DMA/bus 地址。
    let phys_addr = virt_to_phys((dma_info.cpu_addr.as_ptr() as usize).into());
    let curr = current();
    let proc_data = &curr.as_thread().proc_data;

    // 这段映射只暴露给当前进程，不做跨进程共享。
    let user_start = match (|| {
        let mut aspace = proc_data.aspace.lock();
        let user_range = VirtAddrRange::new(aspace.base(), aspace.end());
        let start = aspace
            .find_free_area(aspace.base(), map_size, user_range)
            .ok_or(AxError::NoMemory)?;

        // 这里建立的是“用户虚拟地址 -> 物理地址”的线性映射。
        // 设备依旧通过 DMA/bus 地址访问这片内存，CPU 用户态则通过返回的 user_ptr 访问。
        aspace.map_linear(
            start,
            phys_addr,
            map_size,
            MappingFlags::READ | MappingFlags::WRITE | MappingFlags::USER,
        )?;
        Ok::<VirtAddr, AxError>(start)
    })() {
        Ok(start) => start,
        Err(err) => {
            unsafe {
                dealloc_coherent(dma_info, layout);
            }
            return Err(err);
        }
    };

    if let Err(err) = (|| {
        let dma_addr = UserPtr::<u64>::from(dma_addr_out).get_as_mut()?;
        *dma_addr = dma_info.bus_addr.as_u64();
        Ok::<(), AxError>(())
    })() {
        cleanup_dma_mapping(user_start, map_size, layout, dma_info);
        return Err(err);
    }

    // 当前还没有 dma_free syscall，因此把分配记录挂到进程上，等进程退出时统一回收。
    proc_data.record_dma_allocation(user_start.as_usize(), map_size, layout, dma_info);

    Ok(user_start.as_usize() as isize)
}
