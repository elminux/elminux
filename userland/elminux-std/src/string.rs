//! String types
//!
//! UTF-8 string, no C string compatibility.

pub use alloc::string::String;

use alloc::vec::Vec;

/// Elminux custom string type (UTF-8) - alternative to standard String
pub struct ElString {
    bytes: Vec<u8>,
}

impl ElString {
    pub fn new() -> Self {
        Self { bytes: Vec::new() }
    }

    pub fn from_str(s: &str) -> Self {
        Self {
            bytes: s.as_bytes().to_vec(),
        }
    }

    pub fn as_str(&self) -> &str {
        unsafe { core::str::from_utf8_unchecked(&self.bytes) }
    }

    pub fn push(&mut self, ch: char) {
        let mut buf = [0; 4];
        let bytes = ch.encode_utf8(&mut buf);
        self.bytes.extend_from_slice(bytes.as_bytes());
    }

    pub fn push_str(&mut self, s: &str) {
        self.bytes.extend_from_slice(s.as_bytes());
    }

    pub fn len(&self) -> usize {
        self.bytes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.bytes.is_empty()
    }
}

impl Default for ElString {
    fn default() -> Self {
        Self::new()
    }
}
