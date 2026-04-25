//! Virtual Memory Manager
//!
//! 4-level page table walker and page mapping/unmapping.

pub const PML4_SHIFT: usize = 39;
pub const PDPT_SHIFT: usize = 30;
pub const PD_SHIFT: usize = 21;
pub const PT_SHIFT: usize = 12;

pub const PAGE_SIZE: usize = 4096;
pub const PAGE_SIZE_U64: u64 = 4096;

/// Higher-half base address for kernel mapping.
pub const KERNEL_BASE: u64 = 0xFFFF_8000_0000_0000;

use crate::pmm;

// ─── PTE flag bits (x86_64, 4-level paging) ─────────────────────────────────

pub const PTE_PRESENT: u64 = 1 << 0;
pub const PTE_WRITABLE: u64 = 1 << 1;
pub const PTE_USER: u64 = 1 << 2;
pub const PTE_WRITETHROUGH: u64 = 1 << 3;
pub const PTE_CACHEDISABLE: u64 = 1 << 4;
pub const PTE_ACCESSED: u64 = 1 << 5;
pub const PTE_DIRTY: u64 = 1 << 6;
pub const PTE_HUGE: u64 = 1 << 7;
pub const PTE_GLOBAL: u64 = 1 << 8;
pub const PTE_NOEXECUTE: u64 = 1 << 63;

// ─── Page table indices ─────────────────────────────────────────────────────

pub fn pml4_index(virt: u64) -> usize {
    ((virt >> 39) & 0x1FF) as usize
}

pub fn pdpt_index(virt: u64) -> usize {
    ((virt >> 30) & 0x1FF) as usize
}

pub fn pd_index(virt: u64) -> usize {
    ((virt >> 21) & 0x1FF) as usize
}

pub fn pt_index(virt: u64) -> usize {
    ((virt >> 12) & 0x1FF) as usize
}

// ─── CR3 helpers ────────────────────────────────────────────────────────────

/// Read the current CR3 register (physical address of PML4).
pub fn current_cr3() -> u64 {
    let cr3: u64;
    unsafe {
        core::arch::asm!("mov {}, cr3", out(reg) cr3);
    }
    cr3 & !0xFFF
}

// ─── Flag conversion ────────────────────────────────────────────────────────

/// Convert [`PageFlags`] into raw x86_64 PTE bits.
pub fn flags_to_bits(flags: PageFlags) -> u64 {
    let mut bits = 0u64;
    if flags.present {
        bits |= PTE_PRESENT;
    }
    if flags.writable {
        bits |= PTE_WRITABLE;
    }
    if flags.user {
        bits |= PTE_USER;
    }
    if flags.write_through {
        bits |= PTE_WRITETHROUGH;
    }
    if flags.cache_disable {
        bits |= PTE_CACHEDISABLE;
    }
    if flags.accessed {
        bits |= PTE_ACCESSED;
    }
    if flags.dirty {
        bits |= PTE_DIRTY;
    }
    if flags.huge {
        bits |= PTE_HUGE;
    }
    if flags.no_execute {
        bits |= PTE_NOEXECUTE;
    }
    bits
}

// ─── Page table walker ──────────────────────────────────────────────────────

/// Walk the 4-level hierarchy for `virt` and return a pointer to the leaf PTE.
///
/// Returns `None` if any non-present level is encountered before the leaf.
/// If the walk terminates early at a huge page (PDPT or PD level), the
/// pointer to that entry is returned.
///
/// # Safety
/// `pml4` must be a valid virtual-address pointer to the active PML4.
pub unsafe fn walk(pml4: *mut u64, virt: u64) -> Option<*mut u64> {
    let pml4e = unsafe { pml4.add(pml4_index(virt)) };
    if unsafe { *pml4e } & PTE_PRESENT == 0 {
        return None;
    }

    let pdpt = (unsafe { *pml4e } & !0xFFF) as *mut u64;
    let pdpte = unsafe { pdpt.add(pdpt_index(virt)) };
    if unsafe { *pdpte } & PTE_PRESENT == 0 {
        return None;
    }
    if unsafe { *pdpte } & PTE_HUGE != 0 {
        return Some(pdpte);
    }

    let pd = (unsafe { *pdpte } & !0xFFF) as *mut u64;
    let pde = unsafe { pd.add(pd_index(virt)) };
    if unsafe { *pde } & PTE_PRESENT == 0 {
        return None;
    }
    if unsafe { *pde } & PTE_HUGE != 0 {
        return Some(pde);
    }

    let pt = (unsafe { *pde } & !0xFFF) as *mut u64;
    let pte = unsafe { pt.add(pt_index(virt)) };
    Some(pte)
}

// ─── Allocation helper for page-table pages ─────────────────────────────────

/// Allocate a zeroed 4 KiB page table page via the PMM.
///
/// # Safety
/// PMM must be initialized.  The returned pointer is valid as a virtual
/// address only while the identity mapping is active (phys == virt).
unsafe fn alloc_table() -> Option<*mut u64> {
    let frame = pmm::alloc_frame()?;
    let ptr = frame as *mut u64;
    unsafe {
        core::ptr::write_bytes(ptr as *mut u8, 0, PAGE_SIZE);
    }
    Some(ptr)
}

/// Map a virtual page to a physical frame, allocating missing page-table
/// levels on demand.
///
/// # Panics
/// Panics on out-of-memory while allocating a page-table page, or if the
/// walk encounters an existing huge page that would overlap the target.
///
/// # Safety
/// `pml4` must be a valid virtual-address pointer to the root page table.
/// PMM must be initialized.  This function is not re-entrant; caller must
/// serialize concurrent modifications.
pub unsafe fn map_page(pml4: *mut u64, virt: u64, phys: u64, flags: PageFlags) {
    // Intermediate entries need PRESENT + WRITABLE, and USER if the leaf is user-accessible.
    let mut intermediate = PTE_PRESENT | PTE_WRITABLE;
    if flags.user {
        intermediate |= PTE_USER;
    }

    let pml4e = unsafe { pml4.add(pml4_index(virt)) };
    if unsafe { *pml4e } & PTE_PRESENT == 0 {
        let table = alloc_table().expect("OOM allocating PDPT");
        unsafe { *pml4e = (table as u64) | intermediate };
    } else if flags.user && (unsafe { *pml4e } & PTE_USER == 0) {
        unsafe { *pml4e |= PTE_USER };
    }

    let pdpt = (unsafe { *pml4e } & !0xFFF) as *mut u64;
    let pdpte = unsafe { pdpt.add(pdpt_index(virt)) };
    if unsafe { *pdpte } & PTE_PRESENT == 0 {
        let table = alloc_table().expect("OOM allocating PD");
        unsafe { *pdpte = (table as u64) | intermediate };
    } else if flags.user && (unsafe { *pdpte } & PTE_USER == 0) {
        unsafe { *pdpte |= PTE_USER };
    }
    if unsafe { *pdpte } & PTE_HUGE != 0 {
        panic!(
            "map_page: virt {:#x} collides with existing 1 GiB huge page",
            virt
        );
    }

    let pd = (unsafe { *pdpte } & !0xFFF) as *mut u64;
    let pde = unsafe { pd.add(pd_index(virt)) };
    if unsafe { *pde } & PTE_PRESENT == 0 {
        let table = alloc_table().expect("OOM allocating PT");
        unsafe { *pde = (table as u64) | intermediate };
    } else if flags.user && (unsafe { *pde } & PTE_USER == 0) {
        unsafe { *pde |= PTE_USER };
    }
    if unsafe { *pde } & PTE_HUGE != 0 {
        panic!(
            "map_page: virt {:#x} collides with existing 2 MiB huge page",
            virt
        );
    }

    let pt = (unsafe { *pde } & !0xFFF) as *mut u64;
    let pte = unsafe { pt.add(pt_index(virt)) };
    unsafe { *pte = (phys & !0xFFF) | flags_to_bits(flags) };

    flush_tlb(virt);
}

/// Unmap a virtual page and flush the TLB entry.
///
/// # Safety
/// `pml4` must be a valid virtual-address pointer to the root page table.
pub unsafe fn unmap_page(pml4: *mut u64, virt: u64) {
    if let Some(pte) = walk(pml4, virt) {
        // Refuse to zero a huge-page entry (1 GiB or 2 MiB) — that would
        // silently unmap far more than a single 4 KiB page.
        if unsafe { *pte } & PTE_HUGE != 0 {
            return;
        }
        unsafe { core::ptr::write_volatile(pte, 0) };
        flush_tlb(virt);
    }
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

// ─── TLB flush ──────────────────────────────────────────────────────────────

/// Flush the TLB entry for a single virtual address.
pub unsafe fn flush_tlb(virt: u64) {
    core::arch::asm!("invlpg [{}]", in(reg) virt, options(nostack));
}

/// Flush the entire TLB by reloading CR3.
pub unsafe fn flush_tlb_all() {
    let cr3: u64;
    unsafe {
        core::arch::asm!("mov {}, cr3", out(reg) cr3);
        core::arch::asm!("mov cr3, {}", in(reg) cr3);
    }
}

// ─── Higher-half kernel mapping ─────────────────────────────────────────────

/// Map the kernel physical region `[phys_start, phys_end)` into the higher
/// half at `KERNEL_BASE + phys`.
///
/// # Safety
/// `pml4` must be valid.  PMM must be initialized.
pub unsafe fn map_kernel_higher_half(pml4: *mut u64, phys_start: u64, phys_end: u64) {
    let mut addr = phys_start & !0xFFF;
    let end = (phys_end + 0xFFF) & !0xFFF;

    while addr < end {
        let virt = KERNEL_BASE + addr;
        let flags = PageFlags {
            present: true,
            writable: true,
            user: false,
            write_through: false,
            cache_disable: false,
            accessed: false,
            dirty: false,
            huge: false,
            no_execute: false,
        };
        map_page(pml4, virt, addr, flags);
        addr += PAGE_SIZE_U64;
    }
}

// ─── Identity-map teardown ──────────────────────────────────────────────────

/// Tear down the PVH identity map (0–4 GB).
///
/// Clears the four 1 GiB huge-page entries in the PDPT set up by the PVH
/// trampoline, then flushes the TLB via CR3 reload.
///
/// # Safety
/// Must only be called after all identity-mapped accesses (boot info,
/// ACPI tables, e820 map) are complete.  The kernel itself must not
/// rely on `virt == phys` after this point.
pub unsafe fn teardown_identity() {
    let pml4_phys = current_cr3();
    let pml4 = pml4_phys as *mut u64;
    let pml4e = unsafe { *pml4.add(0) };

    if pml4e & PTE_PRESENT != 0 {
        let pdpt = (pml4e & !0xFFF) as *mut u64;
        unsafe {
            core::ptr::write_volatile(pdpt.add(0), 0);
            core::ptr::write_volatile(pdpt.add(1), 0);
            core::ptr::write_volatile(pdpt.add(2), 0);
            core::ptr::write_volatile(pdpt.add(3), 0);
        }
    }

    flush_tlb_all();

    elminux_hal::uart::write_str(
        "[VMM] Identity map 0–4GB torn down (PDPT cleared, TLB flushed)\n",
    );
}
