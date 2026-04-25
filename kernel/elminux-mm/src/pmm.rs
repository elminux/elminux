//! Physical Memory Manager
//!
//! Buddy allocator for 4KB frame management.
//!
//! The buddy allocator divides memory into power-of-2 sized blocks.
//! Minimum allocation unit is 4KB (order 0). Maximum block size is
//! 4MB (order 10, 1024 pages).
//!
//! All shared state lives under a single `Spinlock<BuddyAllocator>`.
//!
//! # Roadmap
//! Per `ARCHITECTURE.md` §Known Risks, v0.5+ replaces the single global
//! spinlock with per-CPU slab caches and a lock-free free list for hot
//! sizes.  The current design is intentionally simple and correct.

use elminux_sync::Spinlock;

/// Higher-half base used to translate physical addresses into virtual
/// addresses; mirrors `vmm::KERNEL_BASE` (kept here as a `const` to avoid
/// a `vmm → pmm → vmm` import cycle).
const KERNEL_BASE: u64 = 0xFFFF_8000_0000_0000;

#[inline]
fn phys_to_virt(phys: u64) -> u64 {
    KERNEL_BASE + phys
}

pub const PAGE_SIZE: usize = 4096;
pub const PAGE_SHIFT: usize = 12; // log2(4096)

/// Maximum order: 4MB blocks (1024 * 4KB)
pub const MAX_ORDER: usize = 10;

/// Maximum number of blocks we can track (supports up to ~4GB RAM).
pub const MAX_BLOCKS: usize = 1024 * 1024; // 1M blocks = 4GB at 4KB per block

/// Sentinel for an empty free list.
const FREE_LIST_EMPTY: u64 = u64::MAX;

/// Buddy allocator state.
pub struct BuddyAllocator {
    /// Base physical address of the managed memory region.
    base_addr: u64,
    /// Total number of 4KB frames managed.
    total_frames: usize,
    /// Free lists: one per order (0..=MAX_ORDER).  Each entry is the
    /// index of the first free block in that order, or `FREE_LIST_EMPTY`.
    free_lists: [u64; MAX_ORDER + 1],
    /// Bitmap tracking which blocks are allocated (1 = allocated).
    /// Each bit represents one 4KB block.
    allocated: [u64; MAX_BLOCKS / 64],
}

impl BuddyAllocator {
    const fn new() -> Self {
        Self {
            base_addr: 0,
            total_frames: 0,
            free_lists: [FREE_LIST_EMPTY; MAX_ORDER + 1],
            allocated: [0; MAX_BLOCKS / 64],
        }
    }
}

/// Global buddy allocator instance.
static BUDDY_ALLOC: Spinlock<BuddyAllocator> = Spinlock::new(BuddyAllocator::new());

/// Initialize the buddy allocator with a memory region.
///
/// # Safety
/// Must be called exactly once during early boot, before any allocations.
/// The memory region `[base, base + total_frames * PAGE_SIZE)` must be
/// identity-mapped and not already in use.
pub unsafe fn init(base: u64, total_frames: usize) {
    let mut alloc = BUDDY_ALLOC.lock();
    alloc.base_addr = base;
    alloc.total_frames = total_frames.min(MAX_BLOCKS);

    // Clear bookkeeping.
    for word in alloc.allocated.iter_mut() {
        *word = 0;
    }
    for list in alloc.free_lists.iter_mut() {
        *list = FREE_LIST_EMPTY;
    }

    // Add all frames to the order-0 free list.
    // TODO(v0.3+): coalesce contiguous frames into larger-order blocks.
    let frames = alloc.total_frames;
    for i in (0..frames).rev() {
        // SAFETY: the memory region is identity-mapped and exclusively
        // owned by the PMM.  Writing a u64 at each frame's base to thread
        // the free list is safe.
        unsafe { push_free_block_locked(&mut alloc, 0, i) };
    }

    elminux_hal::uart::write_str("[PMM] Buddy allocator initialized: ");
    elminux_hal::uart::write_hex(frames as u64);
    elminux_hal::uart::write_str(" frames @ ");
    elminux_hal::uart::write_hex(base);
    elminux_hal::uart::write_str("\n");
}

/// Allocate a single physical frame (4KB).
/// Returns the physical address, or `None` if out of memory.
pub fn alloc_frame() -> Option<u64> {
    let mut alloc = BUDDY_ALLOC.lock();
    // SAFETY: lock guarantees exclusive access to BuddyAllocator.
    let idx = unsafe { alloc_block_locked(&mut alloc, 0)? };
    Some(alloc.base_addr + (idx * PAGE_SIZE) as u64)
}

/// Free a physical frame.
///
/// # Safety
/// The frame must have been allocated by [`alloc_frame`] and not already freed.
pub unsafe fn free_frame(frame: u64) {
    let mut alloc = BUDDY_ALLOC.lock();
    let base = alloc.base_addr;
    let total = alloc.total_frames;
    let offset = frame.saturating_sub(base) as usize;
    if offset % PAGE_SIZE != 0 {
        return; // misaligned — ignore
    }
    let idx = offset / PAGE_SIZE;
    if idx >= total {
        return; // out of range — ignore
    }
    // SAFETY: lock guarantees exclusive access; caller asserts the frame is
    // a valid, once-allocated frame.
    unsafe { free_block_locked(&mut alloc, 0, idx) };
}

// ─── Locked helpers (require the caller to hold BUDDY_ALLOC) ────────────────

/// Allocate a block of the given order.  Returns the block index.
///
/// # Safety
/// Caller holds the `BUDDY_ALLOC` lock (i.e. has `&mut BuddyAllocator`).
unsafe fn alloc_block_locked(alloc: &mut BuddyAllocator, order: usize) -> Option<usize> {
    let order = order.min(MAX_ORDER);
    // SAFETY: see function doc.
    let idx = unsafe { pop_free_block_locked(alloc, order)? };
    mark_allocated_locked(alloc, idx, true);
    Some(idx)
}

/// Free a block of the given order at the given index.
///
/// # Safety
/// Caller holds the `BUDDY_ALLOC` lock.
unsafe fn free_block_locked(alloc: &mut BuddyAllocator, order: usize, idx: usize) {
    let order = order.min(MAX_ORDER);
    mark_allocated_locked(alloc, idx, false);
    // TODO(v0.3+): coalesce with buddy before pushing to free list.
    // SAFETY: see function doc.
    unsafe { push_free_block_locked(alloc, order, idx) };
}

/// Push a block onto the free list for the given order.
///
/// Uses the first 8 bytes of each free block to store the next-index.
///
/// # Safety
/// Caller holds the lock; the frame's memory is identity-mapped and
/// owned by the PMM.
unsafe fn push_free_block_locked(alloc: &mut BuddyAllocator, order: usize, idx: usize) {
    let order = order.min(MAX_ORDER);
    if idx >= alloc.total_frames {
        return;
    }

    let old_head = alloc.free_lists[order];
    let block_addr = alloc.base_addr + (idx * PAGE_SIZE) as u64;
    // Address frames via the higher-half mapping so the free list
    // remains accessible after the identity map has been dropped.
    let next_ptr = phys_to_virt(block_addr) as *mut u64;
    // SAFETY: PMM-owned frame, exclusive access via the lock.
    unsafe { next_ptr.write(old_head) };

    alloc.free_lists[order] = idx as u64;
}

/// Pop a block from the free list for the given order.
///
/// # Safety
/// Caller holds the lock.
unsafe fn pop_free_block_locked(alloc: &mut BuddyAllocator, order: usize) -> Option<usize> {
    let order = order.min(MAX_ORDER);
    let idx = alloc.free_lists[order];
    if idx == FREE_LIST_EMPTY {
        return None;
    }

    let block_addr = alloc.base_addr + (idx as usize * PAGE_SIZE) as u64;
    let next_ptr = phys_to_virt(block_addr) as *const u64;
    // SAFETY: PMM-owned frame; written by push_free_block_locked.
    let next = unsafe { next_ptr.read() };
    alloc.free_lists[order] = next;

    Some(idx as usize)
}

/// Mark a block as allocated or free in the bitmap.
fn mark_allocated_locked(alloc: &mut BuddyAllocator, idx: usize, allocated: bool) {
    if idx >= alloc.total_frames {
        return;
    }
    let word_idx = idx / 64;
    let bit_idx = idx % 64;
    if word_idx >= alloc.allocated.len() {
        return;
    }
    let mask = 1u64 << bit_idx;
    if allocated {
        alloc.allocated[word_idx] |= mask;
    } else {
        alloc.allocated[word_idx] &= !mask;
    }
}

/// Check whether a block is marked allocated.
#[allow(dead_code)]
fn is_allocated_locked(alloc: &BuddyAllocator, idx: usize) -> bool {
    if idx >= alloc.total_frames {
        return false;
    }
    let word_idx = idx / 64;
    let bit_idx = idx % 64;
    if word_idx >= alloc.allocated.len() {
        return false;
    }
    (alloc.allocated[word_idx] & (1u64 << bit_idx)) != 0
}
