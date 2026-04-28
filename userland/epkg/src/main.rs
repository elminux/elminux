//! epkg - Elminux Package Manager
//!
//! Offline-first, hash-addressed, capability-aware package manager.

#![no_std]
#![no_main]

use elminux_std::string::String;

/// Package manager commands
#[allow(dead_code)] // TODO: Parse and dispatch to handlers
enum Command {
    Install(String),
    Remove(String),
    List,
    Verify,
    Build(String),
}

fn main() {
    // TODO: Parse command line arguments
    // TODO: Dispatch to appropriate handler
}

#[allow(dead_code)] // TODO: Called when parsing Install command
fn cmd_install(_package: &str) {
    // TODO: Download package (if not cached)
    // TODO: Verify signature
    // TODO: Stage to store
    // TODO: Atomically link to active
}

#[allow(dead_code)] // TODO: Called when parsing Remove command
fn cmd_remove(_package: &str) {
    // TODO: Unlink from active
    // TODO: Check if still referenced
}

#[allow(dead_code)] // TODO: Called when parsing List command
fn cmd_list() {
    // TODO: Query active packages
    // TODO: Print name, version, hash
}

#[allow(dead_code)] // TODO: Called when parsing Verify command
fn cmd_verify() {
    // TODO: Verify all installed package signatures
    // TODO: Check for tampering
}

#[allow(dead_code)] // TODO: Called when parsing Build command
fn cmd_build(_manifest: &str) {
    // TODO: Parse epkg.toml
    // TODO: Build package contents
    // TODO: Sign with maintainer key
}

// Stub main for no_std
#[no_mangle]
pub extern "C" fn _start() -> ! {
    main();
    loop {
        core::hint::spin_loop();
    }
}
