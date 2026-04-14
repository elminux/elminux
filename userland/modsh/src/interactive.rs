//! Interactive shell features
//!
//! Line editing, history, completion using keyboard/framebuffer drivers.

/// Line editor state
pub struct LineEditor {
    buffer: [u8; 256],
    pos: usize,
}

impl LineEditor {
    pub const fn new() -> Self {
        Self {
            buffer: [0; 256],
            pos: 0,
        }
    }

    pub fn insert(&mut self, ch: u8) {
        if self.pos < 256 {
            self.buffer[self.pos] = ch;
            self.pos += 1;
        }
    }

    pub fn backspace(&mut self) {
        if self.pos > 0 {
            self.pos -= 1;
            self.buffer[self.pos] = 0;
        }
    }

    pub fn clear(&mut self) {
        self.pos = 0;
    }

    pub fn as_str(&self) -> &str {
        unsafe { core::str::from_utf8_unchecked(&self.buffer[..self.pos]) }
    }
}

/// Read line from user input
pub fn read_line(_prompt: &str) -> LineEditor {
    // TODO: Display prompt via framebuffer driver
    // TODO: Read keys via keyboard driver
    // TODO: Handle line editing keys
    LineEditor::new()
}
