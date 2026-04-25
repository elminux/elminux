//! Global Descriptor Table (GDT) setup
//!
//! Defines kernel and user code/data segments for x86_64.
//!
//! Segment selectors:
//! - 0x00: Null segment
//! - 0x08: Kernel code segment (ring 0)
//! - 0x10: Kernel data segment (ring 0)
//! - 0x18: User code segment (ring 3)
//! - 0x20: User data segment (ring 3)
//! - 0x28: TSS segment

use core::arch::asm;

/// GDT entry structure
#[repr(C, packed)]
pub struct GdtEntry {
    limit_low: u16,
    base_low: u16,
    base_middle: u8,
    access: u8,
    granularity: u8,
    base_high: u8,
}

/// Task State Segment (TSS)
#[repr(C, packed)]
pub struct Tss {
    reserved_1: u32,
    pub rsp0: u64,
    pub rsp1: u64,
    pub rsp2: u64,
    reserved_2: u64,
    pub ist1: u64,
    pub ist2: u64,
    pub ist3: u64,
    pub ist4: u64,
    pub ist5: u64,
    pub ist6: u64,
    pub ist7: u64,
    reserved_3: u64,
    reserved_4: u16,
    iomap_base: u16,
}

impl Tss {
    pub const fn new() -> Self {
        Self {
            reserved_1: 0,
            rsp0: 0,
            rsp1: 0,
            rsp2: 0,
            reserved_2: 0,
            ist1: 0,
            ist2: 0,
            ist3: 0,
            ist4: 0,
            ist5: 0,
            ist6: 0,
            ist7: 0,
            reserved_3: 0,
            reserved_4: 0,
            iomap_base: (core::mem::size_of::<Self>() as u16) - 1,
        }
    }

    /// Set the ring 0 stack pointer (for interrupts/syscalls)
    pub fn set_rsp0(&mut self, rsp0: u64) {
        self.rsp0 = rsp0;
    }
}

/// GDT descriptor for lgdt instruction
#[repr(C, packed)]
pub struct GdtDescriptor {
    pub limit: u16,
    pub base: u64,
}

/// Number of GDT entries
pub const GDT_ENTRIES: usize = 7; // Null + 4 segments + 2 for TSS (64-bit TSS uses 2 entries)

/// Complete GDT structure
pub struct Gdt {
    entries: [u64; GDT_ENTRIES],
}

/// Static TSS instance (must be static for GDT reference)
static mut TSS: Tss = Tss::new();

/// Static GDT instance
static mut GDT: Gdt = Gdt::new();

impl Gdt {
    pub const fn new() -> Self {
        Self {
            entries: [0; GDT_ENTRIES],
        }
    }

    /// Create a code/data segment descriptor
    fn create_segment(base: u32, limit: u32, access: u8, granularity: u8) -> u64 {
        let mut descriptor: u64 = 0;
        // Limit low (bits 0-15)
        descriptor |= (limit & 0xFFFF) as u64;
        // Base low (bits 16-39)
        descriptor |= ((base & 0xFFFFFF) as u64) << 16;
        // Access byte (bits 40-47)
        descriptor |= (access as u64) << 40;
        // Limit high nibble (bits 48-51) + flags (bits 52-55: G, D/B, L, AVL)
        descriptor |= (((limit >> 16) & 0xF) as u64) << 48;
        descriptor |= (granularity as u64) << 52;
        // Base high (bits 56-63)
        descriptor |= ((base >> 24) as u64) << 56;
        descriptor
    }

    /// Create a 64-bit TSS descriptor (uses 2 GDT entries)
    fn create_tss_descriptor(base: u64, limit: u32, access: u8) -> (u64, u64) {
        let limit = limit as u64;

        // First 64 bits of TSS descriptor
        let low = (limit & 0xFFFF)                    // bits  0-15: limit low
            | ((base & 0xFFFFFF) << 16)               // bits 16-39: base low + mid
            | ((access as u64) << 40)                 // bits 40-47: access byte
            | (((limit >> 16) & 0xF) << 48)           // bits 48-51: limit high nibble
            | (((base >> 24) & 0xFF) << 56); // bits 56-63: base high byte

        // Second 64 bits (high part of 64-bit TSS)
        let high = base >> 32;

        (low, high)
    }

    /// Initialize and load the GDT
    ///
    /// # Safety
    /// Must be called exactly once during kernel initialization before
    /// enabling interrupts or entering user mode.
    pub unsafe fn init() {
        let gdt = core::ptr::addr_of_mut!(GDT);

        // Entry 0: Null segment
        (*gdt).entries[0] = 0;

        // Entry 1 (0x08): Kernel code segment - ring 0, execute/read, 64-bit
        // Access: Present(1) | DPL 0(00) | S(1) | Code(1) | Conforming(0) | Readable(1) | Accessed(0) = 0x9A
        // Flags nibble (bits 52-55): G=1 | D/B=0 | L=1 (64-bit) | AVL=0 = 0xA
        (*gdt).entries[1] = Self::create_segment(0, 0xFFFFF, 0x9A, 0xA);

        // Entry 2 (0x10): Kernel data segment - ring 0, read/write
        // Access: Present(1) | DPL 0(00) | S(1) | Data(0) | Expand-down(0) | Writable(1) | Accessed(0) = 0x92
        // Flags nibble (bits 52-55): G=1 | D/B=1 | L=0 | AVL=0 = 0xC
        (*gdt).entries[2] = Self::create_segment(0, 0xFFFFF, 0x92, 0xC);

        // Entry 3 (0x18): User code segment - ring 3, execute/read, 64-bit
        // Access: Present(1) | DPL 3(11) | S(1) | Code(1) | Conforming(0) | Readable(1) | Accessed(0) = 0xFA
        // Flags nibble (bits 52-55): G=1 | D/B=0 | L=1 (64-bit) | AVL=0 = 0xA
        (*gdt).entries[3] = Self::create_segment(0, 0xFFFFF, 0xFA, 0xA);

        // Entry 4 (0x20): User data segment - ring 3, read/write
        // Access: Present(1) | DPL 3(11) | S(1) | Data(0) | Expand-down(0) | Writable(1) | Accessed(0) = 0xF2
        // Flags nibble (bits 52-55): G=1 | D/B=1 | L=0 | AVL=0 = 0xC
        (*gdt).entries[4] = Self::create_segment(0, 0xFFFFF, 0xF2, 0xC);

        // Entries 5-6: TSS (64-bit TSS requires 2 entries)
        let tss_base = core::ptr::addr_of!(TSS) as u64;
        let tss_limit = (core::mem::size_of::<Tss>() - 1) as u32;
        // Access: Present(1) | DPL 0(00) | Type(1001) = 0x89 for available 64-bit TSS
        let (tss_low, tss_high) = Self::create_tss_descriptor(tss_base, tss_limit, 0x89);
        (*gdt).entries[5] = tss_low;
        (*gdt).entries[6] = tss_high;

        // Load GDT
        let gdt_descriptor = GdtDescriptor {
            limit: (core::mem::size_of::<Gdt>() - 1) as u16,
            base: core::ptr::addr_of!(GDT) as u64,
        };

        // Load GDT and reload segment registers
        // Note: We use a far return to reload CS properly
        let data_segment: u16 = 0x10;
        asm!(
            "lgdt [{gdt_desc}]",
            "mov ax, {data_seg:x}",
            "mov ds, ax",
            "mov es, ax",
            "mov fs, ax",
            "mov gs, ax",
            "mov ss, ax",
            "push 0x08",          // Kernel code segment selector
            "lea {tmp}, [rip + 2f]",
            "push {tmp}",
            "retfq",
            "2:",
            gdt_desc = in(reg) &gdt_descriptor,
            data_seg = in(reg) data_segment,
            tmp = lateout(reg) _,
        );

        // Load TSS
        asm!(
            "ltr {0:x}",
            in(reg) 0x28u16, // TSS selector (entry 5 * 8)
        );
    }

    /// Set the kernel stack pointer in TSS (for interrupt handlers)
    ///
    /// # Safety
    /// Mutates a global `static mut TSS`. Caller must ensure no concurrent
    /// access from another CPU or interrupt handler (data race). This will
    /// need a proper lock once SMP is enabled.
    pub unsafe fn set_kernel_stack(rsp0: u64) {
        let tss = &raw mut TSS;
        unsafe { (*tss).set_rsp0(rsp0) };
    }

    /// Get a raw pointer to the current TSS.
    ///
    /// Returns a raw pointer rather than `&'static mut` because constructing
    /// a `&mut` to a `static mut` trivially violates aliasing rules — any
    /// second call (or any shared reference) would cause UB.
    ///
    /// # Safety
    /// Caller must serialize access and never dereference concurrently.
    pub unsafe fn tss_ptr() -> *mut Tss {
        &raw mut TSS
    }
}

/// Initialize GDT with default segments
///
/// # Safety
/// Must be called once during early kernel initialization.
/// This function modifies critical CPU state.
pub unsafe fn init() {
    Gdt::init();
}
