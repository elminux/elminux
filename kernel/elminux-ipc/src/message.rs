//! IPC Message format
//!
//! Fixed-size message with register-sized fields for fast copying.

/// Maximum message payload size
pub const MSG_PAYLOAD_SIZE: usize = 112; // Fits in cache line

/// IPC message structure
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Msg {
    /// Message type/opcode
    pub code: u64,
    /// Sender capability
    pub sender: u64,
    /// Payload data
    pub payload: [u64; 14],
}

impl Msg {
    pub const fn new(code: u64) -> Self {
        Self {
            code,
            sender: 0,
            payload: [0; 14],
        }
    }

    pub fn with_payload(code: u64, payload: [u64; 14]) -> Self {
        Self {
            code,
            sender: 0,
            payload,
        }
    }
}
