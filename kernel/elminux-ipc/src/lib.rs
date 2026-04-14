//! Elminux IPC System
//!
//! Capability-based synchronous message passing with zero-copy fast path.

#![no_std]

pub mod capability;
pub mod message;
pub mod channel;

/// Capability rights flags
pub mod rights {
    pub const READ: u32 = 1 << 0;
    pub const WRITE: u32 = 1 << 1;
    pub const GRANT: u32 = 1 << 2;
    pub const REVOKE: u32 = 1 << 3;
}

/// Initialize IPC subsystem
pub fn init() {
    // TODO: Set up capability table management
    // TODO: Initialize message buffers
}
