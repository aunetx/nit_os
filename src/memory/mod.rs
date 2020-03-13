//! This module contains utilitaries to init and use memory.
//!

// extern crates
use x86_64::{structures::paging::OffsetPageTable, VirtAddr};

// public submodules
pub mod allocators;
pub mod heap;
pub mod mapping;

/// The divergent function that the kernel throws when it encounter an allocation
/// error.
#[alloc_error_handler]
fn alloc_error_handler(layout: alloc::alloc::Layout) -> ! {
    panic!(
        "allocation error: {:?}\nMake sure heap was initialized before using it.",
        layout
    )
}

/// Initialize memory of the kernel, must be used before any use of `alloc`.
///
/// The default steps are :
/// - get physical memory offset
/// - init mapping table
/// - init allocator
/// - init heap
///
/// Returns the mapper and the frame allocator.
pub fn init(
    boot_info: &'static bootloader::BootInfo,
) -> (OffsetPageTable<'static>, mapping::BootInfoFrameAllocator) {
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { mapping::init(phys_mem_offset) };
    let mut frame_allocator =
        unsafe { mapping::BootInfoFrameAllocator::init(&boot_info.memory_map) };

    heap::init(&mut mapper, &mut frame_allocator).expect("heap initialization failed");

    (mapper, frame_allocator)
}
