//! modsh - Modular Shell
//!
//! Interactive shell for Elminux (ported from github.com/modsh-shell/modsh).

#![no_std]

pub mod builtin;
pub mod interactive;
pub mod pipeline;

/// Shell configuration
pub struct Config {
    pub interactive: bool,
    pub echo: bool,
}

impl Config {
    pub fn new() -> Self {
        Self {
            interactive: true,
            echo: false,
        }
    }
}

/// Run shell main loop
pub fn run(_config: Config) {
    // TODO: Initialize line editor
    // TODO: Load history
    // TODO: Main input loop
}
