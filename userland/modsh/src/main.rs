//! modsh main entry point

#![no_std]
#![no_main]

use core::panic::PanicInfo;
use modsh::{run, Config};

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let config = Config::new();
    run(config);
    loop {}
}
