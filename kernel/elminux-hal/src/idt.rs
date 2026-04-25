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
static mut IDT_DESCRIPTOR: IdtDescriptor = IdtDescriptor { limit: 0, base: 0 };

/// Initialize IDT with exception handlers
///
/// # Safety
/// Must be called after GDT is initialized and before interrupts are enabled.
/// This function modifies critical CPU state.
pub unsafe fn init() {
    let idt = core::ptr::addr_of_mut!(IDT);

    // Set up CPU exception handlers (0-31)
    // For now, use a generic handler stub that halts
    // TODO: Implement specific handlers for each exception type

    // Set handler for each exception
    for i in 0..32 {
        (*idt).set_handler(i, generic_exception_handler as *const () as u64);
    }

    // Override vector 14 (page fault) with dedicated handler for testing
    (*idt).set_handler(14, page_fault_handler as *const () as u64);

    // Set up IRQ stubs (32-255) - these will be used for hardware interrupts
    for i in 32..IDT_ENTRIES {
        (*idt).set_handler(i, generic_irq_handler as *const () as u64);
    }

    // Load IDT
    IDT_DESCRIPTOR = IdtDescriptor {
        limit: (core::mem::size_of::<Idt>() - 1) as u16,
        base: core::ptr::addr_of!(IDT) as u64,
    };

    core::arch::asm!(
        "lidt [{0}]",
        in(reg) core::ptr::addr_of!(IDT_DESCRIPTOR),
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
        "cli", // TODO: Save registers, print exception info, halt
        "2:", "hlt", "jmp 2b",
    );
}

/// Page fault handler (vector 14).
///
/// Page faults push an error code on the stack automatically.
/// We read CR2 (faulting address) and pass it to a Rust handler.
#[unsafe(naked)]
unsafe extern "C" fn page_fault_handler() {
    core::arch::naked_asm!(
        // Save scratch registers (System V AMD64 callee-saved are
        // rbx, rbp, r12-r15; we save the caller-saved ones)
        "push rax",
        "push rcx",
        "push rdx",
        "push rsi",
        "push rdi",
        "push r8",
        "push r9",
        "push r10",
        "push r11",
        // Pass CR2 (faulting linear address) as first argument in RDI
        "mov rdi, cr2",
        "call {handler}",
        // handler is diverging (-> !), but if it ever returns:
        "pop r11",
        "pop r10",
        "pop r9",
        "pop r8",
        "pop rdi",
        "pop rsi",
        "pop rdx",
        "pop rcx",
        "pop rax",
        // Pop the error code that the CPU pushed for #PF
        "add rsp, 8",
        "iretq",
        handler = sym page_fault_handler_impl,
    );
}

/// Rust implementation of page fault handler.
///
/// Prints the faulting address via UART and halts. In the identity-map
/// teardown test, a fault in the low 4GB range is expected and reported
/// as pass.
#[no_mangle]
extern "C" fn page_fault_handler_impl(addr: u64) -> ! {
    if addr < 0x1_0000_0000 {
        // Identity range fault — expected during teardown test
        crate::uart::write_str("[TEST] Page fault at ");
        crate::uart::write_hex(addr);
        crate::uart::write_str(" — identity map teardown working!\n");
    } else {
        crate::uart::write_str("[!!!] Unexpected page fault at ");
        crate::uart::write_hex(addr);
        crate::uart::write_str("\n");
    }
    loop {
        unsafe {
            core::arch::asm!("cli; hlt");
        }
    }
}

/// Generic IRQ handler stub (placeholder)
///
/// Called for all hardware interrupts (32+) until proper PIC/APIC
/// handling is implemented. Sends EOI to the local APIC so the next
/// interrupt can fire; otherwise the APIC keeps the IRQ in-service
/// forever and blocks all lower-priority interrupts.
#[unsafe(naked)]
unsafe extern "C" fn generic_irq_handler() {
    // SAFETY: Naked function - only inline assembly is allowed here.
    // Write 0 to Local APIC EOI register (0xFEE000B0) then iretq.
    core::arch::naked_asm!(
        "push rax",
        "mov rax, 0xFEE000B0",
        "mov dword ptr [rax], 0",
        "pop rax",
        "iretq",
    );
}

/// Load a specific handler into the IDT
///
/// # Safety
/// Must only be called after `init()` has been called.
/// Caller must ensure handler is a valid interrupt handler function.
pub unsafe fn set_handler(index: usize, handler: u64) {
    let idt = core::ptr::addr_of_mut!(IDT);
    (*idt).set_handler(index, handler);
}
