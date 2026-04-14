//! Elminux Memory Manager
//!
//! Physical memory management (buddy allocator), virtual memory (page tables),
//! and kernel heap allocator (slab allocator).

#![no_std]

extern crate alloc;

pub mod pmm;
pub mod vmm;
pub mod heap;

/// Initialize memory manager with Limine-provided memory map
pub fn init(_memory_map: &[MemoryRegion]) {
    // TODO: Parse memory map
    // TODO: Initialize buddy allocator
    // TODO: Initialize slab allocator for heap
}

/// Memory region descriptor
#[derive(Debug, Clone, Copy)]
pub struct MemoryRegion {
    pub base: u64,
    pub length: u64,
    pub typ: MemoryRegionType,
}

/// Memory region types
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MemoryRegionType {
    Usable,
    Reserved,
    AcpiReclaimable,
    AcpiNvs,
    BadMemory,
    BootloaderReclaimable,
    Kernel,
    Framebuffer,
}
