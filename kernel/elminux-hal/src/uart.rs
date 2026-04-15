//! UART 16550 Serial Port Driver
//!
//! Basic serial output for early kernel debugging on COM1.

use crate::port::{outb, inb};

/// COM1 base port
pub const COM1: u16 = 0x3F8;

/// UART register offsets
const REG_DATA: u16 = 0;       // Data register (read/write)
const REG_IER: u16 = 1;        // Interrupt Enable Register
#[allow(dead_code)]
const REG_IIR: u16 = 2;        // Interrupt Identification Register
const REG_FCR: u16 = 2;        // FIFO Control Register
const REG_LCR: u16 = 3;        // Line Control Register
const REG_MCR: u16 = 4;        // Modem Control Register
const REG_LSR: u16 = 5;        // Line Status Register
#[allow(dead_code)]
const REG_MSR: u16 = 6;        // Modem Status Register
#[allow(dead_code)]
const REG_SCR: u16 = 7;        // Scratch Register

/// Line Status Register bits
const LSR_THRE: u8 = 0x20;     // Transmitter Holding Register Empty
#[allow(dead_code)]
const LSR_TEMT: u8 = 0x40;     // Transmitter Empty

/// Initialize UART 16550 on COM1
///
/// Configures 115200 baud, 8 data bits, no parity, 1 stop bit (8N1),
/// with FIFO enabled.
pub fn init() {
    // Disable interrupts
    unsafe { outb(COM1 + REG_IER, 0x00); }

    // Enable DLAB (Divisor Latch Access Bit) to set baud rate
    unsafe { outb(COM1 + REG_LCR, 0x80); }

    // Set baud rate to 115200 (divisor = 1)
    // Divisor low byte
    unsafe { outb(COM1 + REG_DATA, 0x01); }
    // Divisor high byte
    unsafe { outb(COM1 + REG_IER, 0x00); }

    // 8 bits, no parity, one stop bit (8N1), clear DLAB
    unsafe { outb(COM1 + REG_LCR, 0x03); }

    // Enable FIFO, clear them, with 14-byte threshold
    unsafe { outb(COM1 + REG_FCR, 0xC7); }

    // IRQs enabled, RTS/DSR set (ready to receive)
    unsafe { outb(COM1 + REG_MCR, 0x0B); }
}

/// Check if transmit buffer is empty (ready to send)
fn transmit_empty() -> bool {
    (unsafe { inb(COM1 + REG_LSR) } & LSR_THRE) != 0
}

/// Write a single byte to serial
///
/// Blocks until the transmit buffer is ready.
pub fn write_byte(byte: u8) {
    // Wait for transmit buffer to be empty
    while !transmit_empty() {
        core::hint::spin_loop();
    }
    unsafe { outb(COM1 + REG_DATA, byte); }
}

/// Write a string to serial
///
/// Convenience function that writes a full string.
/// Automatically converts `\n` to `\r\n` for proper terminal display.
pub fn write_str(s: &str) {
    for byte in s.bytes() {
        if byte == b'\n' {
            write_byte(b'\r');
        }
        write_byte(byte);
    }
}


