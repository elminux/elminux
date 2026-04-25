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

extern crate alloc;
use alloc::boxed::Box;
use alloc::vec::Vec;

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
global_asm!(include_str!("boot/pvh.s"), options(att_syntax));

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
use elminux_mm;

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

    // 5.1 Initialize physical memory manager with e820 memory map
    elminux_mm::init_from_e820(info.memmap_paddr, info.memmap_entries);
}

/// Stress test for the slab allocator (Milestone 5.4).
///
/// Allocates objects across all slab size classes (32–4096 bytes),
/// verifies read/write integrity, and deallocates in mixed order.
/// Panics on any allocation failure or data corruption.
fn heap_stress_test() {
    println!("[TEST] Heap stress test starting...");

    // Test 1: Box allocation of various sizes
    println!("[TEST] 1. Box allocations (8, 32, 128, 1024, 4096 bytes)...");
    {
        let b8: Box<u64> = Box::new(0xDEAD_BEEF_CAFE_BABE);
        assert_eq!(*b8, 0xDEAD_BEEF_CAFE_BABE, "8-byte Box corrupted");

        let b32: Box<[u8; 32]> = Box::new([0xAB; 32]);
        assert!(b32.iter().all(|&x| x == 0xAB), "32-byte Box corrupted");

        let b128: Box<[u64; 16]> = Box::new([0x1234_5678_9ABC_DEF0; 16]);
        assert!(
            b128.iter().all(|&x| x == 0x1234_5678_9ABC_DEF0),
            "128-byte Box corrupted"
        );

        let b1k: Box<[u8; 1024]> = Box::new([0x55; 1024]);
        assert!(b1k.iter().all(|&x| x == 0x55), "1024-byte Box corrupted");

        let b4k: Box<[u8; 4096]> = Box::new([0xAA; 4096]);
        assert!(b4k.iter().all(|&x| x == 0xAA), "4096-byte Box corrupted");

        // Explicit drops (not required but documents intent)
        drop(b8);
        drop(b32);
        drop(b128);
        drop(b1k);
        drop(b4k);
    }
    println!("[TEST]    Box allocations PASSED");

    // Test 2: Vec dynamic growth
    println!("[TEST] 2. Vec dynamic growth (push 256 elements)...");
    {
        let mut vec: Vec<u64> = Vec::new();
        for i in 0..256u64 {
            vec.push(i ^ 0xA5A5_A5A5_A5A5_A5A5);
        }
        for (i, &val) in vec.iter().enumerate() {
            let expected = (i as u64) ^ 0xA5A5_A5A5_A5A5_A5A5;
            assert_eq!(val, expected, "Vec element {} corrupted", i);
        }
        // Test removal from middle and end
        vec.truncate(128);
        assert_eq!(vec.len(), 128);
        vec.clear();
        assert!(vec.is_empty());
    }
    println!("[TEST]    Vec growth PASSED");

    // Test 3: Interleaved allocations of different sizes (mixed order free)
    println!("[TEST] 3. Interleaved allocations (mixed order free)...");
    {
        // Allocate in pattern: small, medium, large, small, medium, large...
        let mut boxes: Vec<Box<[u8]>> = Vec::new();
        let sizes = [32usize, 128, 512, 1024, 64, 256, 2048, 4096];

        for (round, &size) in sizes.iter().cycle().take(64).enumerate() {
            let pattern = ((round * 7 + 13) & 0xFF) as u8;
            let data: Vec<u8> = core::iter::repeat(pattern).take(size).collect();
            boxes.push(data.into_boxed_slice());
        }

        // Verify all allocations
        for (i, boxed) in boxes.iter().enumerate() {
            let pattern = ((i * 7 + 13) & 0xFF) as u8;
            assert!(
                boxed.iter().all(|&x| x == pattern),
                "Interleaved alloc {} corrupted",
                i
            );
        }

        // Free in reverse order (not allocation order) to stress coalescing
        while let Some(b) = boxes.pop() {
            drop(b);
        }
    }
    println!("[TEST]    Interleaved allocations PASSED");

    // Test 4: Many small allocations (exercise 32/64 byte slabs)
    println!("[TEST] 4. Many small allocations (100 x 32 bytes)...");
    {
        let mut small_boxes: Vec<Box<[u8; 32]>> = Vec::new();
        for i in 0..100 {
            let pattern = ((i * 31 + 17) & 0xFF) as u8;
            small_boxes.push(Box::new([pattern; 32]));
        }

        // Verify
        for (i, boxed) in small_boxes.iter().enumerate() {
            let pattern = ((i * 31 + 17) & 0xFF) as u8;
            assert!(
                boxed.iter().all(|&x| x == pattern),
                "Small alloc {} corrupted",
                i
            );
        }

        // Free every other one, then the rest (fragmentation stress)
        let mut i = 0;
        small_boxes.retain(|_| {
            i += 1;
            i % 2 == 0 // Keep even indices, drop odd
        });

        // Verify remaining
        for (idx, boxed) in small_boxes.iter().enumerate() {
            let original_idx = idx * 2 + 1; // Odd indices were kept (1, 3, 5...)
            let pattern = ((original_idx * 31 + 17) & 0xFF) as u8;
            assert!(
                boxed.iter().all(|&x| x == pattern),
                "Remaining alloc {} corrupted",
                idx
            );
        }

        drop(small_boxes);
    }
    println!("[TEST]    Small allocations PASSED");

    // Test 5: Large allocation boundary (4096 bytes = page size)
    println!("[TEST] 5. Page-sized allocations (10 x 4096 bytes)...");
    {
        let mut large_boxes: Vec<Box<[u8; 4096]>> = Vec::new();
        for i in 0..10 {
            let pattern = (0x10 + (i as u8)) & 0xFF;
            large_boxes.push(Box::new([pattern; 4096]));
        }

        for (i, boxed) in large_boxes.iter().enumerate() {
            let pattern = (0x10 + (i as u8)) & 0xFF;
            assert!(
                boxed.iter().all(|&x| x == pattern),
                "Large alloc {} corrupted",
                i
            );
        }

        drop(large_boxes);
    }
    println!("[TEST]    Page-sized allocations PASSED");

    println!("[TEST] Heap stress test PASSED — all 5 subtests complete");
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

    // 4.10 Identity-map teardown test (disabled: UART/APIC need higher-half mapping first)
    // TODO: Map UART/APIC at KERNEL_BASE + phys, then re-enable full teardown
    println!("[TEST] Identity teardown SKIPPED (UART/APIC need higher-half MMIO mapping)");
    // unsafe {
    //     elminux_mm::vmm::teardown_identity();
    // }

    // Test frame allocation (Milestone 5.4)
    println!("[TEST] Allocating 3 physical frames...");
    let frame1 = elminux_mm::pmm::alloc_frame();
    let frame2 = elminux_mm::pmm::alloc_frame();
    let frame3 = elminux_mm::pmm::alloc_frame();

    match (frame1, frame2, frame3) {
        (Some(f1), Some(f2), Some(f3)) => {
            println!("[TEST] Allocated frames:");
            println!("       Frame 1: {:#x}", f1);
            println!("       Frame 2: {:#x}", f2);
            println!("       Frame 3: {:#x}", f3);

            // Free the frames
            unsafe {
                elminux_mm::pmm::free_frame(f1);
                elminux_mm::pmm::free_frame(f2);
                elminux_mm::pmm::free_frame(f3);
            }
            println!("[TEST] All frames freed successfully");
            println!("[TEST] PMM test PASSED");
        }
        _ => {
            println!(
                "[TEST] PMM allocation FAILED (got {:?}, {:?}, {:?})",
                frame1, frame2, frame3
            );
        }
    }

    println!();

    // Heap stress test (Milestone 5.4)
    heap_stress_test();

    println!();

    // TODO: Initialize scheduler
    // TODO: Start init process

    // Halt loop - replace with scheduler when ready
    println!("[BOOT] Kernel initialization complete — entering halt loop");
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
