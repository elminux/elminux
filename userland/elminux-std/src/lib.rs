//! Elminux Standard Library
//!
//! User space foundation: I/O traits, IPC bindings, threading primitives.

#![no_std]

use core::alloc::{GlobalAlloc, Layout};

/// Placeholder global allocator - panics until real allocator implemented (milestone 5.3)
pub struct PlaceholderAlloc;

unsafe impl GlobalAlloc for PlaceholderAlloc {
    unsafe fn alloc(&self, _layout: Layout) -> *mut u8 {
        panic!("Global allocator not yet implemented - milestone 5.3 pending")
    }
    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {}
}

#[global_allocator]
static ALLOCATOR: PlaceholderAlloc = PlaceholderAlloc;

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
pub use alloc::sync;
pub use alloc::vec;
// Note: alloc::string not re-exported - local string module provides String

/// Initialize standard library
pub fn init() {
    // TODO: Setup IPC connection to system services
}

/// Panic handler for user space
#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    // TODO: Print panic message via debug IPC
    // TODO: Call sys_exit(1)
    loop {
        core::hint::spin_loop();
    }
}
