//! I/O traits for elminux
//!
//! Read, Write, BufRead traits adapted for capability-based IPC.

/// Read trait
pub trait Read {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, ReadError>;
}

/// Write trait
pub trait Write {
    fn write(&mut self, buf: &[u8]) -> Result<usize, WriteError>;
    fn flush(&mut self) -> Result<(), WriteError>;
}

/// BufRead trait
pub trait BufRead: Read {
    fn fill_buf(&mut self) -> Result<&[u8], ReadError>;
    fn consume(&mut self, amt: usize);
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ReadError {
    Interrupted,
    WouldBlock,
    Other,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum WriteError {
    Interrupted,
    WouldBlock,
    Other,
}
