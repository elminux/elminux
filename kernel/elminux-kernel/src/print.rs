//! Kernel print macros for serial output
//!
//! Provides `print!` and `println!` macros that output to COM1 serial port.
//! These macros are used for kernel debugging and boot messages.

use core::fmt;
use elminux_hal::uart;

/// A writer that outputs to the serial port
pub struct SerialWriter;

impl fmt::Write for SerialWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        uart::write_str(s);
        Ok(())
    }
}

/// Print a formatted string to the serial port
///
/// # Usage
/// ```
/// print!("Hello, {}!\n", "world");
/// ```
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        $crate::print::_print(format_args!($($arg)*))
    };
}

/// Print a formatted string to the serial port, followed by a newline
///
/// # Usage
/// ```
/// println!("Hello, {}!", "world");
/// ```
#[macro_export]
macro_rules! println {
    () => {
        $crate::print::_print(format_args!("\n"))
    };
    ($($arg:tt)*) => {
        $crate::print::_print(format_args!("{}\n", format_args!($($arg)*)))
    };
}

/// Internal function used by the print! macro
///
/// # Safety
/// This function assumes UART has been initialized. If called before
/// uart::init(), output will be lost or cause undefined behavior.
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    let mut writer = SerialWriter;
    let _ = writer.write_fmt(args);
}
