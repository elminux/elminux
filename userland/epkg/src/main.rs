//! epkg - Elminux Package Manager
//!
//! Offline-first, hash-addressed, capability-aware package manager.

#![no_std]
#![no_main]

use elminux_std::io::Write;
use elminux_std::string::ElString;

/// Package manager commands
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

fn cmd_install(_package: &str) {
    // TODO: Download package (if not cached)
    // TODO: Verify signature
    // TODO: Stage to store
    // TODO: Atomically link to active
}

fn cmd_remove(_package: &str) {
    // TODO: Unlink from active
    // TODO: Check if still referenced
}

fn cmd_list() {
    // TODO: Query active packages
    // TODO: Print name, version, hash
}

fn cmd_verify() {
    // TODO: Verify all installed package signatures
    // TODO: Check for tampering
}

fn cmd_build(_manifest: &str) {
    // TODO: Parse epkg.toml
    // TODO: Build package contents
    // TODO: Sign with maintainer key
}

// Stub main for no_std
#[no_mangle]
pub extern "C" fn _start() -> ! {
    main();
    loop {}
}
