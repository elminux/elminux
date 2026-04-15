//! Interrupt Descriptor Table (IDT) setup
//!
//! CPU exception handlers (0-31) and IRQ stubs (32+).

/// IDT entry for x86_64
#[repr(C)]
#[derive(Copy, Clone)]
pub struct IdtEntry {
    offset_low: u16,
    selector: u16,
    ist: u8,
    type_attr: u8,
    offset_mid: u16,
    offset_high: u32,
    reserved: u32,
}

/// IDT descriptor structure
#[repr(C, packed)]
pub struct IdtDescriptor {
    limit: u16,
    base: u64,
}

/// Number of IDT entries (256 for x86_64)
pub const IDT_ENTRIES: usize = 256;

/// IDT structure
pub struct Idt {
    entries: [IdtEntry; IDT_ENTRIES],
}

impl Idt {
    pub const fn new() -> Self {
        Self {
            entries: [IdtEntry::new(); IDT_ENTRIES],
        }
    }

    pub fn set_handler(&mut self, index: usize, handler: u64) {
        self.entries[index].set_handler(handler);
    }
}

impl IdtEntry {
    pub const fn new() -> Self {
        Self {
            offset_low: 0,
            selector: 0,
            ist: 0,
            type_attr: 0,
            offset_mid: 0,
            offset_high: 0,
            reserved: 0,
        }
    }

    pub fn set_handler(&mut self, handler: u64) {
        self.offset_low = (handler & 0xFFFF) as u16;
        self.offset_mid = ((handler >> 16) & 0xFFFF) as u16;
        self.offset_high = ((handler >> 32) & 0xFFFFFFFF) as u32;
        self.selector = 0x08; // Kernel code segment
        self.type_attr = 0x8E; // Present, ring 0, interrupt gate
    }
}

/// Static IDT instance
static mut IDT: Idt = Idt::new();

/// IDT descriptor for lidt instruction
static mut IDT_DESCRIPTOR: IdtDescriptor = IdtDescriptor {
    limit: 0,
    base: 0,
};

/// Initialize IDT with exception handlers
///
/// # Safety
/// Must be called after GDT is initialized and before interrupts are enabled.
/// This function modifies critical CPU state.
pub unsafe fn init() {
    let idt = &mut IDT;

    // Set up CPU exception handlers (0-31)
    // For now, use a generic handler stub that halts
    // TODO: Implement specific handlers for each exception type

    // Set handler for each exception
    for i in 0..32 {
        idt.set_handler(i, generic_exception_handler as u64);
    }

    // Set up IRQ stubs (32-255) - these will be used for hardware interrupts
    for i in 32..IDT_ENTRIES {
        idt.set_handler(i, generic_irq_handler as u64);
    }

    // Load IDT
    IDT_DESCRIPTOR = IdtDescriptor {
        limit: (core::mem::size_of::<Idt>() - 1) as u16,
        base: core::ptr::addr_of!(IDT) as u64,
    };

    core::arch::asm!(
        "lidt [{0}]",
        in(reg) &IDT_DESCRIPTOR,
    );
}

/// Generic exception handler (placeholder)
///
/// This is called for all CPU exceptions (0-31) until specific
/// handlers are implemented.
#[unsafe(naked)]
unsafe extern "C" fn generic_exception_handler() {
    // SAFETY: Naked function - only inline assembly is allowed here.
    core::arch::naked_asm!(
        "cli",
        // TODO: Save registers, print exception info, halt
        "2:",
        "hlt",
        "jmp 2b",
    );
}

/// Generic IRQ handler stub (placeholder)
///
/// Called for all hardware interrupts (32+) until proper PIC/APIC
/// handling is implemented.
#[unsafe(naked)]
unsafe extern "C" fn generic_irq_handler() {
    // SAFETY: Naked function - only inline assembly is allowed here.
    core::arch::naked_asm!(
        "cli",
        // TODO: Save registers, send EOI, call handler
        "iretq",
    );
}

/// Load a specific handler into the IDT
///
/// # Safety
/// Must only be called after `init()` has been called.
/// Caller must ensure handler is a valid interrupt handler function.
pub unsafe fn set_handler(index: usize, handler: u64) {
    IDT.set_handler(index, handler);
}
