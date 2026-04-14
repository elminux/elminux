//! Environment access
//!
//! Capability-based environment variable access.

/// Get environment variable
pub fn var(_key: &str) -> Option<ElString> {
    // TODO: Query via IPC to environment service
    None
}

/// Set environment variable
pub fn set_var(_key: &str, _value: &str) {
    // TODO: Set via IPC to environment service
}

use crate::string::ElString;
