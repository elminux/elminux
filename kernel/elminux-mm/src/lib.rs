//! Elminux Memory Manager
//!
//! Physical memory management (buddy allocator), virtual memory (page tables),
//! and kernel heap allocator (slab allocator).

#![no_std]

extern crate alloc;

pub mod heap;
pub mod pmm;
pub mod vmm;

/// e820 memory map entry (BIOS standard format, used by Xen PVH).
/// Each entry is 24 bytes: addr(8) + size(8) + type(4).
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct E820Entry {
    pub addr: u64,
    pub size: u64,
    pub typ: u32,
}

/// e820 memory region types.
pub mod e820 {
    /// Usable RAM (conventional memory).
    pub const USABLE: u32 = 1;
    /// Reserved (unusable).
    pub const RESERVED: u32 = 2;
    /// ACPI reclaimable (usable after ACPI tables parsed).
    pub const ACPI_RECLAIMABLE: u32 = 3;
    /// ACPI NVS (must preserve across sleep).
    pub const ACPI_NVS: u32 = 4;
    /// Bad memory (unusable).
    pub const BAD: u32 = 5;
}

/// Initialize memory manager with PVH-provided e820 memory map.
///
/// # Safety
/// `memmap_paddr` must be a valid physical address of an array of `E820Entry`
/// structures, identity-mapped. `entries` must be the correct count.
///
/// # Arguments
/// * `memmap_paddr` - Physical address of e820 array.
/// * `entries` - Number of entries in the array.
pub unsafe fn init_from_e820(memmap_paddr: u64, entries: u32) {
    if memmap_paddr == 0 || entries == 0 {
        // No memory map provided - cannot initialize PMM
        return;
    }

    let slice = core::slice::from_raw_parts(memmap_paddr as *const E820Entry, entries as usize);

    // Convert e820 entries to MemoryRegion format and find a suitable PMM region
    // We need a contiguous usable region that doesn't overlap with the kernel.
    // Kernel is loaded around 1MB, we'll start PMM at 16MB to be safe.
    const PMM_MIN_ADDR: u64 = 0x1000000; // 16MB - start of PMM-managed memory

    let mut total_usable_pages: usize = 0;
    let mut pmm_base: u64 = 0;
    let mut pmm_pages: usize = 0;

    for entry in slice {
        let typ = match entry.typ {
            e820::USABLE => MemoryRegionType::Usable,
            e820::RESERVED => MemoryRegionType::Reserved,
            e820::ACPI_RECLAIMABLE => MemoryRegionType::AcpiReclaimable,
            e820::ACPI_NVS => MemoryRegionType::AcpiNvs,
            e820::BAD => MemoryRegionType::BadMemory,
            _ => MemoryRegionType::Reserved,
        };

        if entry.typ == e820::USABLE && entry.size >= pmm::PAGE_SIZE as u64 {
            let pages = (entry.size as usize) / pmm::PAGE_SIZE;
            total_usable_pages += pages;

            // Find the best usable region for PMM (largest region above PMM_MIN_ADDR)
            let region_end = entry.addr + entry.size;
            if entry.addr < PMM_MIN_ADDR && region_end > PMM_MIN_ADDR {
                // Region straddles PMM_MIN_ADDR, use the part above
                let usable_start = PMM_MIN_ADDR;
                let usable_size = region_end - usable_start;
                let usable_pages = (usable_size as usize) / pmm::PAGE_SIZE;

                if usable_pages > pmm_pages {
                    pmm_base = usable_start;
                    pmm_pages = usable_pages;
                }
            } else if entry.addr >= PMM_MIN_ADDR && pages > pmm_pages {
                // Region is entirely above PMM_MIN_ADDR
                pmm_base = entry.addr;
                pmm_pages = pages;
            }
        }

        // TODO: Store regions for reservation tracking
        let _region = MemoryRegion {
            base: entry.addr,
            length: entry.size,
            typ,
        };
    }

    // Initialize buddy allocator if we found a suitable region
    if pmm_base != 0 && pmm_pages > 0 {
        // Reserve some pages for the PMM's own metadata (bitmap)
        // Bitmap needs 1 bit per page, so 1/64th of the pages for u64 words
        let bitmap_pages = (pmm_pages + 4095) / (4096 * 64) + 1;
        let adjusted_base = pmm_base + (bitmap_pages * pmm::PAGE_SIZE) as u64;
        let adjusted_pages = pmm_pages.saturating_sub(bitmap_pages);

        if adjusted_pages > 0 {
            pmm::init(adjusted_base, adjusted_pages);
        }
    }

    // Log memory stats
    elminux_hal::uart::write_str("[MM] Total usable memory: ");
    elminux_hal::uart::write_hex(total_usable_pages as u64);
    elminux_hal::uart::write_str(" pages (");
    elminux_hal::uart::write_hex((total_usable_pages * pmm::PAGE_SIZE) as u64);
    elminux_hal::uart::write_str(" bytes)\n");
}

/// Legacy init for Limine-provided memory map (kept for compatibility).
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
