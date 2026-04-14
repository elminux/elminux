//! Elminux Syscall ABI
//!
//! System call numbers, ABI version, and entry point.

#![no_std]

pub mod abi;
pub mod dispatcher;
pub mod handler;

/// Current ABI version
pub const ABI_VERSION: u32 = 1;

/// Syscall numbers (stable)
#[repr(u64)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Syscall {
    /// Yield current task
    Yield = 0,
    /// Exit task with code
    Exit = 1,
    /// Send message on capability
    Send = 2,
    /// Receive message on capability
    Recv = 3,
    /// Allocate pages
    AllocPages = 4,
    /// Free pages
    FreePages = 5,
    /// Spawn new process
    Spawn = 6,
    /// Drop capability
    CapDrop = 7,
}

impl Syscall {
    /// Convert syscall number to Syscall enum
    pub fn from_number(n: u64) -> Option<Self> {
        match n {
            0 => Some(Self::Yield),
            1 => Some(Self::Exit),
            2 => Some(Self::Send),
            3 => Some(Self::Recv),
            4 => Some(Self::AllocPages),
            5 => Some(Self::FreePages),
            6 => Some(Self::Spawn),
            7 => Some(Self::CapDrop),
            _ => None,
        }
    }
}
