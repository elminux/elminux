//! Kernel Heap Allocator
//!
//! Fixed-size slab caches backed by 4 KiB PMM frames.  Supports
//! allocations up to 4 KiB; larger requests return null.
//!
//! Shared state lives under a `Spinlock<[SlabCache; N]>`.
//!
//! # Roadmap
//! Per `ARCHITECTURE.md` §Known Risks, v0.5+ replaces this with per-CPU
//! slab caches and a lock-free free list for hot sizes.

use crate::pmm;
use alloc::alloc::{GlobalAlloc, Layout};
use elminux_sync::Spinlock;

// ─── Slab sizes (power-of-two, naturally aligned) ───────────────────────────

const SLAB_SIZES: [usize; 8] = [32, 64, 128, 256, 512, 1024, 2048, 4096];
const MIN_SLAB_SHIFT: u32 = 5; // 2^5 == 32 == SLAB_SIZES[0]

/// One slab cache: linked list of free objects within PMM frames.
struct SlabCache {
    size: usize,
    free_list: *mut u8,
}

// SAFETY: SlabCache only lives inside the Spinlock; concurrent access is
// mediated by the lock.  Send is required to move the wrapper across
// threads, which the lock makes safe.
unsafe impl Send for SlabCache {}

impl SlabCache {
    const fn new(size: usize) -> Self {
        Self {
            size,
            free_list: core::ptr::null_mut(),
        }
    }

    /// Grow this cache by carving a new PMM frame into objects.
    ///
    /// Returns `false` on PMM OOM.
    fn grow(&mut self) -> bool {
        let frame = match pmm::alloc_frame() {
            Some(f) => f,
            None => return false,
        };
        let page = frame as *mut u8;
        let count = pmm::PAGE_SIZE / self.size;

        for i in (0..count).rev() {
            // SAFETY: each slot lies inside the freshly-allocated frame.
            unsafe {
                let obj = page.add(i * self.size);
                *(obj as *mut *mut u8) = self.free_list;
                self.free_list = obj;
            }
        }
        true
    }

    /// Pop one object from the free list, growing on demand.
    fn alloc(&mut self) -> *mut u8 {
        if self.free_list.is_null() && !self.grow() {
            return core::ptr::null_mut();
        }
        let obj = self.free_list;
        // SAFETY: obj is non-null and points to a slot inside a PMM frame.
        unsafe {
            self.free_list = *(obj as *mut *mut u8);
        }
        obj
    }

    /// Push an object back onto the free list.
    ///
    /// # Safety
    /// `ptr` must have been returned by [`Self::alloc`] from this cache.
    unsafe fn dealloc(&mut self, ptr: *mut u8) {
        // SAFETY: per contract, `ptr` is a slot of size `self.size`
        // inside a PMM frame owned by this cache.
        unsafe {
            *(ptr as *mut *mut u8) = self.free_list;
        }
        self.free_list = ptr;
    }
}

// ─── Global cache table ─────────────────────────────────────────────────────

static SLAB_CACHES: Spinlock<[SlabCache; 8]> = Spinlock::new([
    SlabCache::new(32),
    SlabCache::new(64),
    SlabCache::new(128),
    SlabCache::new(256),
    SlabCache::new(512),
    SlabCache::new(1024),
    SlabCache::new(2048),
    SlabCache::new(4096),
]);

/// Compute the slab index required for `layout`, or `None` if the
/// requested size/alignment exceeds the largest slab.
fn slab_index_for(layout: Layout) -> Option<usize> {
    let size = layout.size().max(1);
    let needed = size.next_power_of_two().max(layout.align());
    let largest = *SLAB_SIZES.last().expect("SLAB_SIZES non-empty");
    if needed > largest {
        return None;
    }
    let needed_pow2 = needed.next_power_of_two();
    let shift = needed_pow2.trailing_zeros();
    Some((shift.saturating_sub(MIN_SLAB_SHIFT)) as usize)
}

// ─── GlobalAlloc implementation ─────────────────────────────────────────────

/// Slab allocator for the kernel heap.
pub struct SlabAllocator;

unsafe impl GlobalAlloc for SlabAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let idx = match slab_index_for(layout) {
            Some(i) => i,
            None => return core::ptr::null_mut(),
        };
        let mut caches = SLAB_CACHES.lock();
        caches[idx].alloc()
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        let idx = match slab_index_for(layout) {
            Some(i) => i,
            None => return,
        };
        let mut caches = SLAB_CACHES.lock();
        // SAFETY: caller satisfies GlobalAlloc::dealloc contract — `ptr`
        // came from a previous alloc with the same layout, hence the
        // same slab index.
        unsafe {
            caches[idx].dealloc(ptr);
        }
    }
}

/// Global heap allocator instance.
#[global_allocator]
static HEAP_ALLOCATOR: SlabAllocator = SlabAllocator;

/// Initialise heap allocator.  Currently a no-op; slab caches grow on
/// demand.  Reserved for future warm-up logic.
pub fn init() {}
