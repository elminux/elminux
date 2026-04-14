//! Global Descriptor Table (GDT) setup
//!
//! Defines kernel and user code/data segments for x86_64.

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
            iomap_base: 0,
        }
    }
}
