//! Elminux Kernel Entry Point
//!
//! The kernel is a hybrid design: trusted core runs in kernel space,
//! drivers run in user space with capability-based IPC.
//!
//! Boot protocol: PVH (Xen) via 32-bit trampoline. QEMU passes a
//! `hvm_start_info` structure pointer in %rdi (first SysV AMD64 arg).

#![no_std]
#![no_main]

use core::arch::global_asm;
use core::panic::PanicInfo;

/// PVH start info structure (Xen HVM start_info layout).
/// QEMU's -kernel boot provides this at the 32-bit entry point.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HvmStartInfo {
    /// Signature: "xen" (0x656e78) for Xen HVM, or 0 if not present
    pub magic: u32,
    /// Version of this structure (currently 1)
    pub version: u32,
    /// Flags (bit 0: 64-bit)
    pub flags: u32,
    /// Number of modules loaded
    pub nr_modules: u32,
    /// Physical address of module list (hvm_modlist_entry array)
    pub modlist_paddr: u64,
    /// Physical address of command line (null-terminated)
    pub cmdline_paddr: u64,
    /// Physical address of RSDP (ACPI root)
    pub rsdp_paddr: u64,
    /// Physical address of memory map (e820 entries)
    pub memmap_paddr: u64,
    /// Number of memory map entries
    pub memmap_entries: u32,
    /// Reserved
    pub reserved: u32,
}

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

/// Parse boot info and initialize ACPI if RSDP is provided.
///
/// # Safety
/// `boot_info` must be a valid physical address of an `HvmStartInfo`
/// structure provided by the PVH bootloader. Identity mapping required.
unsafe fn parse_boot_info(boot_info: u64) {
    if boot_info == 0 {
        println!("[BOOT] Warning: no hvm_start_info provided");
        return;
    }

    // Identity-mapped assumption: boot_info is a valid physical address.
    let info = &*(boot_info as *const HvmStartInfo);

    // Basic sanity check: non-zero version and magic
    if info.version == 0 {
        println!("[BOOT] Warning: invalid hvm_start_info (version=0)");
        return;
    }

    println!(
        "[BOOT] PVH start info v{}, flags={:08x}",
        info.version, info.flags
    );

    // Initialize ACPI if RSDP provided
    if info.rsdp_paddr != 0 {
        match elminux_hal::acpi::init(info.rsdp_paddr) {
            Some(apic_info) => {
                println!(
                    "[BOOT] ACPI: {} local APIC(s), {} IO-APIC(s)",
                    apic_info.processor_count, apic_info.io_apic_count
                );
            }
            None => {
                println!("[BOOT] Warning: ACPI init failed (continuing without)");
            }
        }
    } else {
        println!("[BOOT] No RSDP provided in boot info");
    }
}

/// Kernel entry point - called by PVH 32-bit trampoline.
///
/// # Arguments
/// * `boot_info` - Physical address of `HvmStartInfo` from QEMU.
///
/// # Safety
/// This is the first Rust code executed after the bootloader.
/// We must initialize CPU state before doing anything else.
#[no_mangle]
pub extern "C" fn _start(boot_info: u64) -> ! {
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

    // 4.6 Parse ACPI tables from boot info (best practice: early, before MM init)
    unsafe {
        parse_boot_info(boot_info);
    }

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
