//! Elminux Kernel Entry Point
//!
//! The kernel is a hybrid design: trusted core runs in kernel space,
//! drivers run in user space with capability-based IPC.

#![no_std]
#![no_main]
#![feature(never_type)]

use core::panic::PanicInfo;

/// Kernel entry point - called by Limine bootloader
#[no_mangle]
pub extern "C" fn _start() -> ! {
    // TODO: Initialize HAL
    // TODO: Initialize memory manager
    // TODO: Initialize scheduler
    // TODO: Start init process

    loop {
        core::hint::spin_loop();
    }
}

/// Panic handler - required for no_std
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        core::hint::spin_loop();
    }
}
