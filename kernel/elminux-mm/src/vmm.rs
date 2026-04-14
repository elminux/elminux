//! Virtual Memory Manager
//!
//! 4-level page table walker and page mapping/unmapping.

pub const PML4_SHIFT: usize = 39;
pub const PDPT_SHIFT: usize = 30;
pub const PD_SHIFT: usize = 21;
pub const PT_SHIFT: usize = 12;

/// Map a virtual page to a physical frame
pub fn map_page(_virt: u64, _phys: u64, _flags: PageFlags) {
    // TODO: Walk page tables, allocate if needed
}

/// Unmap a virtual page
pub fn unmap_page(_virt: u64) {
    // TODO: Walk page tables, clear entry, flush TLB
}

/// Page table entry flags
#[derive(Debug, Clone, Copy)]
pub struct PageFlags {
    pub present: bool,
    pub writable: bool,
    pub user: bool,
    pub write_through: bool,
    pub cache_disable: bool,
    pub accessed: bool,
    pub dirty: bool,
    pub huge: bool,
    pub no_execute: bool,
}

impl PageFlags {
    pub const fn new() -> Self {
        Self {
            present: true,
            writable: false,
            user: false,
            write_through: false,
            cache_disable: false,
            accessed: false,
            dirty: false,
            huge: false,
            no_execute: false,
        }
    }
}
