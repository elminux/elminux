//! APIC (Advanced Programmable Interrupt Controller)
//!
//! Local APIC initialization, timer setup, I/O APIC configuration.
//!
//! This module handles:
//! - Disabling legacy PIC (8259)
//! - Initializing local APIC (MMIO at 0xFEE00000)
//! - APIC timer configuration for preemption

use crate::port::{inb, outb};

/// Local APIC base address (mapped by firmware/limine)
pub const LOCAL_APIC_BASE: u64 = 0xFEE00000;

/// Local APIC register offsets (from base address)
mod regs {
    pub const ID: u64 = 0x20; // Local APIC ID
    pub const VERSION: u64 = 0x30; // Local APIC Version
    pub const TPR: u64 = 0x80; // Task Priority Register
    pub const EOI: u64 = 0xB0; // End of Interrupt
    pub const LDR: u64 = 0xD0; // Logical Destination Register
    pub const DFR: u64 = 0xE0; // Destination Format Register
    pub const SPURIOUS: u64 = 0xF0; // Spurious Interrupt Vector
    pub const ICR_LOW: u64 = 0x300; // Interrupt Command Register (low)
    pub const ICR_HIGH: u64 = 0x310; // Interrupt Command Register (high)
    pub const LVT_TIMER: u64 = 0x320; // LVT Timer
    pub const LVT_THERMAL: u64 = 0x330; // LVT Thermal Sensor
    pub const LVT_PERF: u64 = 0x340; // LVT Performance Counter
    pub const LVT_LINT0: u64 = 0x350; // LVT LINT0
    pub const LVT_LINT1: u64 = 0x360; // LVT LINT1
    pub const LVT_ERROR: u64 = 0x370; // LVT Error
    pub const TIMER_INIT: u64 = 0x380; // Timer Initial Count
    pub const TIMER_CURR: u64 = 0x390; // Timer Current Count
    pub const TIMER_DIV: u64 = 0x3E0; // Timer Divide Configuration
}

/// Spurious interrupt vector flags
const SPURIOUS_ENABLE: u32 = 0x100; // Bit 8: APIC Software Enable
const SPURIOUS_VECTOR: u32 = 0xFF; // Vector 255 for spurious

/// LVT entry flags
const LVT_MASKED: u32 = 0x10000; // Bit 16: Masked
const LVT_PERIODIC: u32 = 0x20000; // Bit 17: Periodic mode
#[allow(dead_code)]
const LVT_ONESHOT: u32 = 0x00000; // One-shot mode (default)

/// Timer divide configuration values
#[allow(dead_code)]
const TIMER_DIV_1: u32 = 0b1011; // Divide by 1
#[allow(dead_code)]
const TIMER_DIV_2: u32 = 0b0000; // Divide by 2
#[allow(dead_code)]
const TIMER_DIV_4: u32 = 0b0001; // Divide by 4
#[allow(dead_code)]
const TIMER_DIV_8: u32 = 0b0010; // Divide by 8
const TIMER_DIV_16: u32 = 0b0011; // Divide by 16
#[allow(dead_code)]
const TIMER_DIV_32: u32 = 0b1000; // Divide by 32
#[allow(dead_code)]
const TIMER_DIV_64: u32 = 0b1001; // Divide by 64
#[allow(dead_code)]
const TIMER_DIV_128: u32 = 0b1010; // Divide by 128

/// PIC (8259) ports
const PIC1_COMMAND: u16 = 0x20;
const PIC1_DATA: u16 = 0x21;
const PIC2_COMMAND: u16 = 0xA0;
const PIC2_DATA: u16 = 0xA1;

/// PIC commands
const ICW1_INIT: u8 = 0x10;
const ICW1_ICW4: u8 = 0x01;
const ICW4_8086: u8 = 0x01;
const OCW3_READ_ISR: u8 = 0x0B;

/// Read 32-bit value from local APIC register
///
/// # Safety
/// Must only be called after local APIC is mapped and accessible.
/// Caller must ensure `offset` is a valid APIC register offset.
unsafe fn read_reg(offset: u64) -> u32 {
    let addr = LOCAL_APIC_BASE + offset;
    core::ptr::read_volatile(addr as *const u32)
}

/// Write 32-bit value to local APIC register
///
/// # Safety
/// Must only be called after local APIC is mapped and accessible.
/// Caller must ensure `offset` is a valid APIC register offset.
unsafe fn write_reg(offset: u64, value: u32) {
    let addr = LOCAL_APIC_BASE + offset;
    core::ptr::write_volatile(addr as *mut u32, value);
}

/// Disable legacy PIC (8259) by masking all interrupts
///
/// # Safety
/// Must be called before enabling local APIC. Safe to call multiple times.
pub unsafe fn disable_pic() {
    // Mask all interrupts on master PIC
    outb(PIC1_DATA, 0xFF);
    // Mask all interrupts on slave PIC
    outb(PIC2_DATA, 0xFF);

    // Wait for PIC to process (small delay)
    // This ensures the mask is applied before we continue
    inb(PIC1_DATA);
}

/// Remap and disable legacy PIC (alternative to simple disable)
///
/// Remaps PIC vectors to avoid overlap with CPU exceptions (0-31),
/// then masks all interrupts.
///
/// # Safety
/// Must be called before enabling local APIC.
#[allow(dead_code)]
pub unsafe fn remap_and_disable_pic() {
    // Start initialization sequence
    outb(PIC1_COMMAND, ICW1_INIT | ICW1_ICW4);
    outb(PIC2_COMMAND, ICW1_INIT | ICW1_ICW4);

    // ICW2: Vector offset
    outb(PIC1_DATA, 0x20); // Master: vectors 0x20-0x27 (32-39)
    outb(PIC2_DATA, 0x28); // Slave: vectors 0x28-0x2F (40-47)

    // ICW3: Cascade configuration
    outb(PIC1_DATA, 0x04); // Master has slave at IRQ2
    outb(PIC2_DATA, 0x02); // Slave is connected to IRQ2 of master

    // ICW4: 8086 mode
    outb(PIC1_DATA, ICW4_8086);
    outb(PIC2_DATA, ICW4_8086);

    // OCW1: Mask all interrupts
    outb(PIC1_DATA, 0xFF);
    outb(PIC2_DATA, 0xFF);
}

/// Enable the local APIC
///
/// # Safety
/// Must be called after:
/// - GDT is initialized (for proper segment handling in interrupts)
/// - IDT is initialized (to handle spurious interrupts)
/// - PIC is disabled (to avoid conflicts)
pub unsafe fn enable() {
    // Enable local APIC by setting bit 8 of spurious interrupt vector register
    let spurious = read_reg(regs::SPURIOUS);
    write_reg(regs::SPURIOUS, spurious | SPURIOUS_ENABLE | SPURIOUS_VECTOR);
}

/// Send End of Interrupt (EOI) to local APIC
///
/// Must be called at the end of every interrupt handler that goes
/// through the local APIC.
///
/// # Safety
/// Must only be called from within an interrupt handler.
pub unsafe fn eoi() {
    write_reg(regs::EOI, 0);
}

/// Get local APIC ID
///
/// # Safety
/// Must only be called after local APIC is enabled.
pub unsafe fn id() -> u32 {
    read_reg(regs::ID) >> 24
}

/// Get local APIC version
///
/// # Safety
/// Must only be called after local APIC is enabled.
#[allow(dead_code)]
pub unsafe fn version() -> u32 {
    read_reg(regs::VERSION) & 0xFF
}

/// Get max LVT entry count
///
/// # Safety
/// Must only be called after local APIC is enabled.
#[allow(dead_code)]
pub unsafe fn max_lvt() -> u32 {
    (read_reg(regs::VERSION) >> 16) & 0xFF
}

/// Mask all LVT entries during initialization
///
/// # Safety
/// Must only be called after local APIC is enabled but before
/// configuring specific LVT entries.
unsafe fn mask_all_lvts() {
    write_reg(regs::LVT_TIMER, LVT_MASKED);
    write_reg(regs::LVT_THERMAL, LVT_MASKED);
    write_reg(regs::LVT_PERF, LVT_MASKED);
    write_reg(regs::LVT_LINT0, LVT_MASKED);
    write_reg(regs::LVT_LINT1, LVT_MASKED);
    write_reg(regs::LVT_ERROR, LVT_MASKED);
}

/// Configure APIC timer
///
/// Sets up the APIC timer in periodic mode with the specified
/// initial count and vector.
///
/// # Arguments
/// * `vector` - Interrupt vector number for timer (typically 0x20-0xFF)
/// * `initial_count` - Initial count value (decrements at bus frequency / divider)
/// * `periodic` - If true, timer reloads after reaching 0; if false, one-shot
///
/// # Safety
/// Must only be called after:
/// - Local APIC is enabled
/// - IDT has a handler installed for the specified vector
pub unsafe fn configure_timer(vector: u8, initial_count: u32, periodic: bool) {
    // Mask timer during configuration
    write_reg(regs::LVT_TIMER, LVT_MASKED);

    // Set timer divide configuration (divide by 16)
    write_reg(regs::TIMER_DIV, TIMER_DIV_16);

    // Set initial count
    write_reg(regs::TIMER_INIT, initial_count);

    // Configure LVT timer entry
    let mut lvt = vector as u32;
    if periodic {
        lvt |= LVT_PERIODIC;
    }
    write_reg(regs::LVT_TIMER, lvt);
}

/// Stop the APIC timer
///
/// # Safety
/// Must only be called after local APIC is enabled.
pub unsafe fn stop_timer() {
    write_reg(regs::LVT_TIMER, LVT_MASKED);
    write_reg(regs::TIMER_INIT, 0);
}

/// Get current timer count
///
/// # Safety
/// Must only be called after local APIC is enabled.
#[allow(dead_code)]
pub unsafe fn timer_count() -> u32 {
    read_reg(regs::TIMER_CURR)
}

/// Calibrate APIC timer using PIT or fixed delay
///
/// This is a placeholder for timer calibration. In a real implementation,
/// you would:
/// 1. Set up the PIT for a known delay (e.g., 10ms)
/// 2. Start APIC timer with max count
/// 3. Wait for PIT delay
/// 4. Read remaining APIC count
/// 5. Calculate ticks per second
///
/// # Returns
/// Estimated APIC timer frequency in Hz (at current divider setting)
///
/// # Safety
/// Must only be called after local APIC is enabled.
#[allow(dead_code)]
pub unsafe fn calibrate_timer() -> u64 {
    // Placeholder: assumes 100MHz bus frequency with divide-by-16
    // Real implementation needs actual calibration
    let bus_frequency = 100_000_000u64; // 100 MHz typical
    let divider = 16u64;
    bus_frequency / divider
}

/// Initialize the local APIC
///
/// This is the main entry point for APIC initialization.
/// Disables PIC, enables local APIC, and masks all LVT entries.
///
/// # Safety
/// Must be called after:
/// - GDT is initialized
/// - IDT is initialized (at least spurious vector handler)
pub unsafe fn init() {
    // Disable legacy PIC
    disable_pic();

    // Enable local APIC
    enable();

    // Mask all LVT entries by default
    mask_all_lvts();

    // Set task priority to 0 (accept all interrupts)
    write_reg(regs::TPR, 0);
}

/// Initialize APIC timer for preemption
///
/// Sets up the APIC timer in periodic mode for scheduling.
///
/// # Arguments
/// * `vector` - Interrupt vector for timer (default: 0x20)
/// * `hz` - Desired timer frequency in Hz (e.g., 100 for 10ms ticks)
///
/// # Safety
/// Must be called after `init()` and after installing
/// the timer interrupt handler in IDT.
pub unsafe fn init_timer(vector: u8, hz: u64) {
    // Calibrate or estimate timer frequency
    let timer_freq = calibrate_timer();

    // Calculate initial count for desired frequency
    let initial_count = (timer_freq / hz) as u32;

    // Configure timer in periodic mode
    configure_timer(vector, initial_count, true);
}
