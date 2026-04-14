//! Capability management
//!
//! Unforgeable tokens representing access rights.

use core::sync::atomic::{AtomicU64, Ordering};

/// Global capability ID counter
static NEXT_CAP_ID: AtomicU64 = AtomicU64::new(1);

/// Capability handle (unforgeable token)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cap(u64);

/// Capability entry in process table
pub struct CapEntry {
    pub id: Cap,
    pub target: CapTarget,
    pub rights: u32,
}

/// What a capability points to
pub enum CapTarget {
    Process(u64),
    Memory(u64),
    Device(u64),
    Channel(u64),
}

impl Cap {
    /// Mint a new unique capability (kernel use only)
    pub fn new() -> Self {
        Self(NEXT_CAP_ID.fetch_add(1, Ordering::SeqCst))
    }

    /// Reconstruct a Cap from a raw integer received from user space.
    /// Caller must validate this value against the process capability table
    /// before use — never trust raw user-supplied IDs.
    pub fn from_raw(id: u64) -> Self {
        Self(id)
    }

    pub fn as_u64(&self) -> u64 {
        self.0
    }
}
