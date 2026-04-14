//! APIC (Advanced Programmable Interrupt Controller)
//!
//! Local APIC initialization, timer setup, I/O APIC configuration.

/// Local APIC base address
pub const LOCAL_APIC_BASE: u64 = 0xFEE00000;

/// Initialize the local APIC
pub fn init() {
    // TODO: Enable local APIC
    // TODO: Configure timer
    // TODO: Set up spurious interrupt vector
}
