//! This module contains the `FixedSizeBlockAllocator`.
//!
//! It is probably the allocator you want to use, but this implementation is not
//! complete yet.
//!

// internal crate
use super::Locked;

// external crates
use alloc::alloc::GlobalAlloc;
use alloc::alloc::Layout;
use core::{mem, ptr};
use linked_list_allocator::Heap;

/// The block sizes to use.
///
/// The sizes must each be power of 2 because they are also used as
/// the block alignment (alignments must be always powers of 2).
const BLOCK_SIZES: &[usize] = &[8, 16, 32, 64, 128, 256, 512, 1024, 2048];

/// A node representing a memory block. Can either point to the next block or be
/// the last one.
struct ListNode {
    next: Option<&'static mut ListNode>,
}

/// A fixed-size blocks allocator.
///
/// This allocator is pretty simple and fast, it does not present important
/// drawbacks : it wastes some memory but presents good performances in
/// almost any cases.
///
/// Should be used for nearly everything.
pub struct FixedSizeBlockAllocator {
    /// A list of heads for each block size.
    ///
    /// If block is too big, it will use the `fallback_allocator`.
    list_heads: [Option<&'static mut ListNode>; BLOCK_SIZES.len()],

    /// Fallback allocator to use if size is too important to be contained in a block.
    ///
    /// The one actually used is from the `linked_list_allocator` crate, it could be
    /// switched to the `linked_list` module when it provides blocks merging.
    fallback_allocator: Heap,
}

impl FixedSizeBlockAllocator {
    /// Creates an empty `FixedSizeBlockAllocator`.
    pub const fn new() -> Self {
        FixedSizeBlockAllocator {
            list_heads: [None; BLOCK_SIZES.len()],
            fallback_allocator: linked_list_allocator::Heap::empty(),
        }
    }

    /// Initialize the allocator with the given heap bounds.
    ///
    /// ## Safety
    ///
    /// This function is unsafe because the caller must guarantee that the given
    /// heap bounds are valid and that the heap is unused. This method must be
    /// called only once.
    pub unsafe fn init(&mut self, heap_start: usize, heap_size: usize) {
        self.fallback_allocator.init(heap_start, heap_size);
    }

    /// Allocate using the fallback allocator.
    ///
    /// This implementation is adapted to the `linked_list_allocator` crate.
    fn fallback_alloc(&mut self, layout: Layout) -> *mut u8 {
        match self.fallback_allocator.allocate_first_fit(layout) {
            Ok(ptr) => ptr.as_ptr(),
            Err(_) => ptr::null_mut(),
        }
    }
}

/// Choose an appropriate block size for the given layout.
///
/// Returns an index into the `BLOCK_SIZES` array.
fn list_index(layout: &Layout) -> Option<usize> {
    let required_block_size = layout.size().max(layout.align());
    BLOCK_SIZES.iter().position(|&s| s >= required_block_size)
}

unsafe impl GlobalAlloc for Locked<FixedSizeBlockAllocator> {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let mut allocator = self.lock();
        match list_index(&layout) {
            Some(index) => {
                match allocator.list_heads[index].take() {
                    Some(node) => {
                        allocator.list_heads[index] = node.next.take();
                        node as *mut ListNode as *mut u8
                    }
                    None => {
                        // no block exists in list => allocate new block
                        let block_size = BLOCK_SIZES[index];
                        // only works if all block sizes are a power of 2
                        let block_align = block_size;
                        let layout = Layout::from_size_align(block_size, block_align).unwrap();
                        allocator.fallback_alloc(layout)
                    }
                }
            }
            None => allocator.fallback_alloc(layout),
        }
    }

    // FIXME understand this clippy lint
    #[allow(clippy::cast_ptr_alignment)]
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        let mut allocator = self.lock();
        match list_index(&layout) {
            Some(index) => {
                let new_node = ListNode {
                    next: allocator.list_heads[index].take(),
                };
                // verify that block has size and alignment required for storing node
                assert!(mem::size_of::<ListNode>() <= BLOCK_SIZES[index]);
                assert!(mem::align_of::<ListNode>() <= BLOCK_SIZES[index]);
                let new_node_ptr = ptr as *mut ListNode;
                new_node_ptr.write(new_node);
                allocator.list_heads[index] = Some(&mut *new_node_ptr);
            }
            None => {
                let ptr = ptr::NonNull::new(ptr).unwrap();
                allocator.fallback_allocator.deallocate(ptr, layout);
            }
        }
    }
}
