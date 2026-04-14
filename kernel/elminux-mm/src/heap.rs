//! Kernel Heap Allocator
//!
//! Slab allocator for fixed-size kernel objects.

use alloc::alloc::{GlobalAlloc, Layout};

/// Slab allocator for kernel heap
pub struct SlabAllocator;

unsafe impl GlobalAlloc for SlabAllocator {
    unsafe fn alloc(&self, _layout: Layout) -> *mut u8 {
        // FIXME(v0.2.0): This is a STUB that always returns null.
        // Any code using alloc (Box, Vec, String, etc.) will trigger
        // an OOM abort or undefined behavior at first allocation.
        //
        // Before use: Implement slab caches for common sizes (32, 64, 128,
        // 256, 512, 1024, 2048 bytes) backed by pmm::alloc_frame().
        // Must handle alignment requirements from layout.align().
        // TODO: Allocate from appropriate slab
        core::ptr::null_mut()
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        // TODO: Return to slab
    }
}

/// Global heap allocator instance
#[global_allocator]
static HEAP_ALLOCATOR: SlabAllocator = SlabAllocator;

/// Initialize heap allocator
pub fn init() {
    // TODO: Create slab caches for common sizes
}
