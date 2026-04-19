//! Elminux Standard Library
//!
//! User space foundation: I/O traits, IPC bindings, threading primitives.

#![no_std]

pub mod env;
pub mod io;
pub mod ipc;
pub mod string;
pub mod thread;

// Re-export alloc collections
extern crate alloc;
pub use alloc::boxed;
pub use alloc::collections;
pub use alloc::rc;
pub use alloc::string;
pub use alloc::sync;
pub use alloc::vec;

/// Initialize standard library
pub fn init() {
    // TODO: Setup IPC connection to system services
}

/// Panic handler for user space
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    // TODO: Print panic message via debug IPC
    // TODO: Call sys_exit(1)
    loop {
        core::hint::spin_loop();
    }
}
