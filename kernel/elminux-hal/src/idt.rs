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
