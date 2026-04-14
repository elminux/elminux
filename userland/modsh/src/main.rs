//! modsh main entry point

#![no_std]
#![no_main]

use modsh::{run, Config};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let config = Config::new();
    run(config);
    loop {}
}
