//! Elminux Kernel Entry Point
//!
//! The kernel is a hybrid design: trusted core runs in kernel space,
//! drivers run in user space with capability-based IPC.

#![no_std]
#![no_main]

use core::panic::PanicInfo;
use elminux_hal::gdt;
use elminux_hal::idt;
use elminux_hal::uart;

/// Kernel entry point - called by Limine bootloader
///
/// # Safety
/// This is the first Rust code executed after the bootloader.
/// We must initialize CPU state before doing anything else.
#[no_mangle]
pub extern "C" fn _start() -> ! {
    // 4.1 Initialize HAL
    // SAFETY: We are in early boot, single-threaded, with no concurrent access
    unsafe {
        // 4.2 Load GDT with kernel/user segments and TSS
        gdt::init();

        // 4.3 Load IDT with exception handlers
        idt::init();

        // 4.4 Initialize serial output for debugging
        uart::init();
    }

    // Print boot banner
    uart::puts_str("\n");
    uart::puts_str("========================================\n");
    uart::puts_str("  Elminux Kernel v0.2.0\n");
    uart::puts_str("  Hybrid microkernel for x86_64\n");
    uart::puts_str("========================================\n");
    uart::puts_str("\n");
    uart::puts_str("[BOOT] GDT initialized\n");
    uart::puts_str("[BOOT] IDT initialized\n");
    uart::puts_str("[BOOT] UART initialized\n");
    uart::puts_str("[BOOT] Kernel boot sequence complete\n");
    uart::puts_str("\n");

    // TODO: Initialize memory manager
    // TODO: Initialize scheduler
    // TODO: Start init process

    // Halt loop - replace with scheduler when ready
    loop {
        unsafe { core::arch::asm!("hlt"); }
    }
}

/// Panic handler - required for no_std
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    // Try to print panic info if UART is available
    uart::puts_str("\n[!!!] KERNEL PANIC\n");
    // TODO: Format and print panic message with _info
    uart::puts_str("[!!!] Panic occurred\n");

    // Halt forever
    loop {
        unsafe { core::arch::asm!("cli; hlt"); }
    }
}
