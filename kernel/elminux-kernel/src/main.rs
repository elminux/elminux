//! Elminux Kernel Entry Point
//!
//! The kernel is a hybrid design: trusted core runs in kernel space,
//! drivers run in user space with capability-based IPC.

#![no_std]
#![no_main]

use core::arch::global_asm;
use core::panic::PanicInfo;

// PVH 32-bit entry trampoline (enables long mode before calling _start)
global_asm!(include_str!("boot/pvh.s"));

// PVH ELF Note for QEMU direct kernel boot (-kernel flag)
// QEMU uses the Xen PVH protocol: XEN_ELFNOTE_PHYS32_ENTRY (type 18)
// Entry point is pvh_start (32-bit protected mode trampoline).
global_asm!(
    r#"
    .section .note.pvh, "a", @note
    .align 4
    .long 4           // namesz: "Xen\0" = 4 bytes
    .long 4           // descsz: 4 bytes (32-bit entry address)
    .long 18          // type: XEN_ELFNOTE_PHYS32_ENTRY = 18
    .asciz "Xen"      // name: "Xen\0"
    .align 4
    .long pvh_start   // desc: 32-bit physical address of trampoline
    "#
);
use elminux_hal::apic;
use elminux_hal::gdt;
use elminux_hal::idt;
use elminux_hal::uart;

mod print;

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

        // 4.5 Initialize APIC (disable PIC, enable local APIC)
        apic::init();
    }

    // Print boot banner
    println!();
    println!("========================================");
    println!("  Elminux Kernel v0.2.0");
    println!("  Hybrid microkernel for x86_64");
    println!("========================================");
    println!();
    println!("[BOOT] GDT initialized");
    println!("[BOOT] IDT initialized");
    println!("[BOOT] UART initialized");
    println!("[BOOT] APIC initialized (PIC disabled, local APIC enabled)");
    println!("[BOOT] Kernel boot sequence complete");
    println!();

    // TODO: Initialize memory manager
    // TODO: Initialize scheduler
    // TODO: Start init process

    // Halt loop - replace with scheduler when ready
    loop {
        unsafe {
            core::arch::asm!("hlt");
        }
    }
}

/// Panic handler - required for no_std
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    // Try to print panic info if UART is available
    println!("\n[!!!] KERNEL PANIC");
    // TODO: Format and print panic message with _info
    println!("[!!!] Panic occurred");

    // Halt forever
    loop {
        unsafe {
            core::arch::asm!("cli; hlt");
        }
    }
}
