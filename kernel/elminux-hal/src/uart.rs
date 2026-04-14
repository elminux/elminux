//! UART 16550 Serial Port Driver
//!
//! Basic serial output for early kernel debugging.

/// COM1 base port
pub const COM1: u16 = 0x3F8;

/// Initialize UART 16550
pub fn init() {
    // TODO: Configure baud rate, 8N1
    // TODO: Enable FIFO
}

/// Write a single byte to serial
pub fn write_byte(_byte: u8) {
    // TODO: Wait for transmit buffer, write byte
}

/// Write a string to serial
pub fn write_str(_s: &str) {
    // TODO: Write each byte
}
