//! Syscall dispatcher
//!
//! Entry point from SYSCALL instruction, dispatches to handlers.

use crate::Syscall;

/// Syscall handler function type
type HandlerFn = fn(u64, u64, u64, u64, u64, u64) -> u64;

/// Dispatcher table
static DISPATCHER: [Option<HandlerFn>; 8] = [
    Some(crate::handler::sys_yield),
    Some(crate::handler::sys_exit),
    Some(crate::handler::sys_send),
    Some(crate::handler::sys_recv),
    Some(crate::handler::sys_alloc_pages),
    Some(crate::handler::sys_free_pages),
    Some(crate::handler::sys_spawn),
    Some(crate::handler::sys_cap_drop),
];

/// Main syscall entry point (called by assembly stub)
#[no_mangle]
pub extern "C" fn syscall_entry(
    num: u64,
    arg1: u64,
    arg2: u64,
    arg3: u64,
    arg4: u64,
    arg5: u64,
) -> u64 {
    let syscall = match Syscall::from_number(num) {
        Some(s) => s,
        None => return -1i64 as u64, // Invalid syscall number
    };

    match DISPATCHER[syscall as usize] {
        Some(handler) => handler(arg1, arg2, arg3, arg4, arg5, 0),
        None => -1i64 as u64,
    }
}

/// Initialize syscall subsystem (setup MSRs)
pub fn init() {
    // TODO: Setup LSTAR, STAR, SFMASK MSRs
}
