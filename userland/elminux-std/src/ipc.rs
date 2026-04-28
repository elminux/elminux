//! IPC bindings
//!
//! Safe wrappers around sys_send/sys_recv.

/// Safe wrapper for sending IPC messages
pub fn send(_cap: u64, _msg: &[u64]) -> Result<(), SendError> {
    // TODO: Call sys_send syscall
    Ok(())
}

/// Safe wrapper for receiving IPC messages
pub fn recv(_cap: u64, _buf: &mut [u64]) -> Result<(), RecvError> {
    // TODO: Call sys_recv syscall
    Ok(())
}

/// Typed channel for type-safe IPC
pub struct Channel<T> {
    #[allow(dead_code)] // Stored for future IPC syscall use
    cap: u64,
    _phantom: core::marker::PhantomData<T>,
}

impl<T> Channel<T> {
    pub fn new(cap: u64) -> Self {
        Self {
            cap,
            _phantom: core::marker::PhantomData,
        }
    }

    pub fn send(&self, _value: T) -> Result<(), SendError> {
        // TODO: Serialize and send
        Ok(())
    }

    pub fn recv(&self) -> Result<T, RecvError> {
        // TODO: Receive and deserialize
        Err(RecvError::WouldBlock)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SendError {
    InvalidCap,
    WouldBlock,
    Other,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RecvError {
    InvalidCap,
    WouldBlock,
    Other,
}
