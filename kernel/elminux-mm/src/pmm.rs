//! Physical Memory Manager
//!
//! Buddy allocator for 4KB frame management.
//!
//! The buddy allocator divides memory into power-of-2 sized blocks.
//! Minimum allocation unit is 4KB (order 0). Maximum block size is
//! 4MB (order 10, 1024 pages).

use core::sync::atomic::{AtomicU64, Ordering};

pub const PAGE_SIZE: usize = 4096;
pub const PAGE_SHIFT: usize = 12; // log2(4096)

/// Maximum order: 4MB blocks (1024 * 4KB)
pub const MAX_ORDER: usize = 10;

/// Maximum number of blocks we can track (supports up to ~4GB RAM)
pub const MAX_BLOCKS: usize = 1024 * 1024; // 1M blocks = 4GB at 4KB per block

/// Buddy allocator state
pub struct BuddyAllocator {
    /// Base physical address of the managed memory region
    base_addr: u64,
    /// Total number of 4KB frames managed
    total_frames: usize,
    /// Free lists: one per order (0..=MAX_ORDER)
    /// Each entry is the index of the first free block in that order,
    /// or usize::MAX if the list is empty.
    free_lists: [AtomicU64; MAX_ORDER + 1],
    /// Bitmap tracking which blocks are allocated (1 = allocated, 0 = free)
    /// Each bit represents one 4KB block.
    allocated: [AtomicU64; MAX_BLOCKS / 64],
}

/// Global buddy allocator instance (statically allocated)
static mut BUDDY_ALLOC: BuddyAllocator = BuddyAllocator {
    base_addr: 0,
    total_frames: 0,
    free_lists: [const { AtomicU64::new(u64::MAX) }; MAX_ORDER + 1],
    allocated: [const { AtomicU64::new(0) }; MAX_BLOCKS / 64],
};

/// Initialize the buddy allocator with a memory region.
///
/// # Safety
/// Must be called exactly once during early boot, before any allocations.
/// The memory region [base, base + total_frames * PAGE_SIZE) must be
/// identity-mapped and not already in use.
pub unsafe fn init(base: u64, total_frames: usize) {
    let alloc = &raw mut BUDDY_ALLOC;
    (*alloc).base_addr = base;
    (*alloc).total_frames = total_frames.min(MAX_BLOCKS);

    // Clear all allocation bits
    for word in (*alloc).allocated.iter_mut() {
        word.store(0, Ordering::Relaxed);
    }

    // Clear free lists
    for list in (*alloc).free_lists.iter_mut() {
        list.store(u64::MAX, Ordering::Relaxed);
    }

    // Add all frames to free list at the highest possible order
    // For simplicity, we add them as order 0 (single pages)
    // TODO: Coalesce contiguous regions into larger order blocks
    let frames = (*alloc).total_frames;
    for i in (0..frames).rev() {
        unsafe {
            push_free_block(0, i);
        }
    }

    elminux_hal::uart::write_str("[PMM] Buddy allocator initialized: ");
    elminux_hal::uart::write_hex(frames as u64);
    elminux_hal::uart::write_str(" frames @ ");
    elminux_hal::uart::write_hex(base);
    elminux_hal::uart::write_str("\n");
}

/// Allocate a single physical frame (4KB).
/// Returns the physical address, or None if out of memory.
pub fn alloc_frame() -> Option<u64> {
    // Try to allocate from order 0 (single page)
    unsafe {
        alloc_block(0).map(|idx| {
            let base = (*(&raw const BUDDY_ALLOC)).base_addr;
            base + (idx * PAGE_SIZE) as u64
        })
    }
}

/// Free a physical frame.
/// # Safety
/// The frame must have been allocated by `alloc_frame` and not already freed.
pub unsafe fn free_frame(frame: u64) {
    let base = (*(&raw const BUDDY_ALLOC)).base_addr;
    let total = (*(&raw const BUDDY_ALLOC)).total_frames;
    let offset = frame.saturating_sub(base) as usize;
    if offset % PAGE_SIZE != 0 {
        // Misaligned frame - ignore
        return;
    }
    let idx = offset / PAGE_SIZE;
    if idx >= total {
        // Out of range - ignore
        return;
    }
    free_block(0, idx);
}

/// Allocate a block of given order (2^order pages).
unsafe fn alloc_block(order: usize) -> Option<usize> {
    let order = order.min(MAX_ORDER);

    // Try to get a block from the free list
    let idx = pop_free_block(order)?;

    // Mark as allocated
    mark_allocated(idx, true);

    Some(idx)
}

/// Free a block of given order at the specified index.
unsafe fn free_block(order: usize, idx: usize) {
    let order = order.min(MAX_ORDER);

    // Mark as free
    mark_allocated(idx, false);

    // Try to coalesce with buddy
    // For now, just add to free list without coalescing
    // TODO: Implement buddy coalescing
    push_free_block(order, idx);
}

/// Push a block onto the free list for the given order.
/// Uses the first 8 bytes of each free block to store the next pointer.
unsafe fn push_free_block(order: usize, idx: usize) {
    let order = order.min(MAX_ORDER);
    unsafe {
        let alloc = &raw mut BUDDY_ALLOC;

        if idx >= (*alloc).total_frames {
            return;
        }

        // Get current head
        let list = &(*alloc).free_lists[order];
        let old_head = list.load(Ordering::Relaxed);

        // Write old head as next pointer into the free block's first 8 bytes
        // Block address = base_addr + idx * PAGE_SIZE
        let block_addr = (*alloc).base_addr + (idx * PAGE_SIZE) as u64;
        let next_ptr = block_addr as *mut u64;
        next_ptr.write(old_head);

        // Update list head to point to this block
        list.store(idx as u64, Ordering::Relaxed);
    }
}

/// Pop a block from the free list for the given order.
unsafe fn pop_free_block(order: usize) -> Option<usize> {
    let order = order.min(MAX_ORDER);
    unsafe {
        let alloc = &raw mut BUDDY_ALLOC;

        let list = &(*alloc).free_lists[order];
        let idx = list.load(Ordering::Relaxed);

        if idx == u64::MAX {
            return None;
        }

        // Read next pointer from the block's first 8 bytes
        let block_addr = (*alloc).base_addr + (idx as usize * PAGE_SIZE) as u64;
        let next_ptr = block_addr as *const u64;
        let next = next_ptr.read();

        // Update list head to point to next block
        list.store(next, Ordering::Relaxed);

        Some(idx as usize)
    }
}

/// Mark a block as allocated or free.
unsafe fn mark_allocated(idx: usize, allocated: bool) {
    unsafe {
        let alloc = &raw mut BUDDY_ALLOC;

        if idx >= (*alloc).total_frames {
            return;
        }

        let word_idx = idx / 64;
        let bit_idx = idx % 64;

        if word_idx >= (*alloc).allocated.len() {
            return;
        }

        let word = &(*alloc).allocated[word_idx];
        let mask = 1u64 << bit_idx;

        if allocated {
            word.fetch_or(mask, Ordering::Relaxed);
        } else {
            word.fetch_and(!mask, Ordering::Relaxed);
        }
    }
}

/// Check if a block is allocated.
#[allow(dead_code)]
unsafe fn is_allocated(idx: usize) -> bool {
    unsafe {
        let alloc = &raw const BUDDY_ALLOC;

        if idx >= (*alloc).total_frames {
            return false;
        }

        let word_idx = idx / 64;
        let bit_idx = idx % 64;

        if word_idx >= (*alloc).allocated.len() {
            return false;
        }

        let word = (*alloc).allocated[word_idx].load(Ordering::Relaxed);
        let mask = 1u64 << bit_idx;

        (word & mask) != 0
    }
}
