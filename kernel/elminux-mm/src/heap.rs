//! Kernel Heap Allocator
//!
//! Fixed-size slab caches backed by 4 KiB PMM frames.
//! Supports allocations up to 4 KiB; larger requests return null.

use crate::pmm;
use alloc::alloc::{GlobalAlloc, Layout};

// ─── Slab sizes (power-of-two, naturally aligned) ───────────────────────────

const SLAB_SIZES: [usize; 8] = [32, 64, 128, 256, 512, 1024, 2048, 4096];

/// One slab cache: linked list of free objects within PMM frames.
struct SlabCache {
    size: usize,
    free_list: *mut u8,
}

impl SlabCache {
    const fn new(size: usize) -> Self {
        Self {
            size,
            free_list: core::ptr::null_mut(),
        }
    }

    /// Grow this cache by carving a new PMM frame into objects.
    ///
    /// # Safety
    /// PMM must be initialized.  Not re-entrant.
    unsafe fn grow(&mut self) -> bool {
        let frame = match pmm::alloc_frame() {
            Some(f) => f,
            None => return false,
        };
        let page = frame as *mut u8;
        let count = pmm::PAGE_SIZE / self.size;

        // Push every object onto the free list.
        // The first bytes of each free slot hold the next pointer.
        for i in (0..count).rev() {
            let obj = unsafe { page.add(i * self.size) };
            unsafe {
                *(obj as *mut *mut u8) = self.free_list;
            }
            self.free_list = obj;
        }
        true
    }

    /// Pop one object from the free list.
    ///
    /// # Safety
    /// Caller must ensure exclusive access.
    unsafe fn alloc(&mut self) -> *mut u8 {
        if self.free_list.is_null() && !self.grow() {
            return core::ptr::null_mut();
        }
        let obj = self.free_list;
        self.free_list = unsafe { *(obj as *mut *mut u8) };
        obj
    }

    /// Push an object back onto the free list.
    ///
    /// # Safety
    /// `ptr` must have been returned by `alloc` from this cache.
    unsafe fn dealloc(&mut self, ptr: *mut u8) {
        unsafe {
            *(ptr as *mut *mut u8) = self.free_list;
        }
        self.free_list = ptr;
    }
}

// ─── Static caches ──────────────────────────────────────────────────────────

static mut SLAB_CACHES: [SlabCache; 8] = [
    SlabCache::new(32),
    SlabCache::new(64),
    SlabCache::new(128),
    SlabCache::new(256),
    SlabCache::new(512),
    SlabCache::new(1024),
    SlabCache::new(2048),
    SlabCache::new(4096),
];

/// Compute the slab size required for `layout`.
fn slab_size_for(layout: Layout) -> Option<usize> {
    let needed = layout.size().next_power_of_two().max(layout.align());
    for size in SLAB_SIZES {
        if size >= needed {
            return Some(size);
        }
    }
    None
}

/// Find the cache matching `size` without creating references to mutable statics.
unsafe fn cache_for_size(size: usize) -> Option<*mut SlabCache> {
    let caches = core::ptr::addr_of_mut!(SLAB_CACHES) as *mut SlabCache;
    for i in 0..SLAB_SIZES.len() {
        let cache = unsafe { caches.add(i) };
        if unsafe { (*cache).size } == size {
            return Some(cache);
        }
    }
    None
}

// ─── GlobalAlloc implementation ───────────────────────────────────────────

/// Slab allocator for kernel heap.
///
/// # Safety note
/// This allocator is **not** currently SMP-safe.  It relies on the kernel
/// being single-threaded during early boot.  A future spinlock wrapper
/// must be added before enabling pre-emption or additional cores.
pub struct SlabAllocator;

unsafe impl GlobalAlloc for SlabAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let size = match slab_size_for(layout) {
            Some(s) => s,
            None => return core::ptr::null_mut(),
        };
        let cache = match cache_for_size(size) {
            Some(c) => c,
            None => return core::ptr::null_mut(),
        };
        unsafe { (*cache).alloc() }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        let size = match slab_size_for(layout) {
            Some(s) => s,
            None => return,
        };
        let cache = match cache_for_size(size) {
            Some(c) => c,
            None => return,
        };
        unsafe { (*cache).dealloc(ptr) };
    }
}

/// Global heap allocator instance.
#[global_allocator]
static HEAP_ALLOCATOR: SlabAllocator = SlabAllocator;

/// Initialise heap allocator (pre-warm if desired).
///
/// Currently a no-op; slab caches grow on first demand via `grow()`.
pub fn init() {
    // Slab caches are lazily populated on first allocation.
}
