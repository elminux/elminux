//! Physical Memory Manager
//!
//! Buddy allocator for 4KB frame management.

pub const PAGE_SIZE: usize = 4096;

/// Allocate a single physical frame
pub fn alloc_frame() -> Option<u64> {
    // TODO: Implement buddy allocator
    None
}

/// Free a physical frame
pub fn free_frame(_frame: u64) {
    // TODO: Return to buddy allocator
}
