//! elinit - Elminux Init System
//!
//! PID 1 equivalent: capability manifest reader, driver supervisor.

#![no_std]
#![no_main]

/// Driver spawn order
const DRIVER_ORDER: &[&str] = &["serial", "keyboard", "framebuffer", "block", "fs"];

fn main() {
    // TODO: Read capability manifest from kernel
    // TODO: Spawn driver servers in order
    for driver in DRIVER_ORDER {
        spawn_driver(driver);
    }

    // TODO: Wait for all drivers ready
    // TODO: Spawn modsh shell
    spawn_shell();

    // TODO: Supervise services (restart on crash)
    supervise();
}

fn spawn_driver(name: &str) {
    // TODO: Locate driver executable
    // TODO: sys_spawn with appropriate caps
    let _ = name;
}

fn spawn_shell() {
    // TODO: Spawn modsh as user shell
}

fn supervise() -> ! {
    loop {
        // TODO: Monitor driver capabilities
        // TODO: Restart crashed services with backoff
        // TODO: Handle shutdown requests
    }
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    main();
    loop {}
}
