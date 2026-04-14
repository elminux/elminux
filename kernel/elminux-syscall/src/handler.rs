//! Syscall handlers

use elminux_ipc::capability::Cap;
use elminux_ipc::message::Msg;

/// sys_yield: Voluntarily yield CPU
pub fn sys_yield(_a1: u64, _a2: u64, _a3: u64, _a4: u64, _a5: u64, _a6: u64) -> u64 {
    // TODO: Call scheduler to yield
    0
}

/// sys_exit: Exit current task with code
pub fn sys_exit(code: u64, _a2: u64, _a3: u64, _a4: u64, _a5: u64, _a6: u64) -> u64 {
    // TODO: Terminate current task
    // TODO: Never returns for this task
    code
}

/// sys_send: Send message on capability
pub fn sys_send(cap: u64, msg_ptr: u64, _a3: u64, _a4: u64, _a5: u64, _a6: u64) -> u64 {
    let _cap = Cap::from_raw(cap);
    // SAFETY: This dereferences a raw pointer from user space with NO validation.
    // FIXME(v0.4.0): Add user_pointer_valid() helper that checks:
    //   - pointer is non-null and aligned
    //   - range [msg_ptr, msg_ptr + size_of::<Msg>()) is in user address space
    //   - memory is readable (page table walk or copy_from_user pattern)
    //   - no kernel memory is accessible
    // Until fixed, any misaligned or kernel-mapped pointer causes UB.
    let _msg = unsafe { *(msg_ptr as *const Msg) };
    // TODO: Validate pointer, send message
    0
}

/// sys_recv: Receive message on capability
pub fn sys_recv(cap: u64, msg_ptr: u64, _a3: u64, _a4: u64, _a5: u64, _a6: u64) -> u64 {
    let _cap = Cap::from_raw(cap);
    let _msg = Msg::new(0);
    // TODO: Receive message, write to msg_ptr
    // SAFETY: Writing to unchecked user-supplied pointer. Same validation
    // requirements as sys_send, plus must verify target is writable.
    // FIXME(v0.4.0): Use copy_to_user() helper after pointer validation.
    unsafe { *(msg_ptr as *mut Msg) = _msg };
    0
}

/// sys_alloc_pages: Allocate physical pages
pub fn sys_alloc_pages(n: u64, _a2: u64, _a3: u64, _a4: u64, _a5: u64, _a6: u64) -> u64 {
    // TODO: Allocate n pages, return virtual address
    n * 4096 // Placeholder
}

/// sys_free_pages: Free allocated pages
pub fn sys_free_pages(addr: u64, n: u64, _a3: u64, _a4: u64, _a5: u64, _a6: u64) -> u64 {
    // TODO: Free n pages at addr
    let _ = (addr, n);
    0
}

/// sys_spawn: Spawn new process from manifest
pub fn sys_spawn(_manifest_ptr: u64, _a2: u64, _a3: u64, _a4: u64, _a5: u64, _a6: u64) -> u64 {
    // TODO: Parse manifest, create process
    // TODO: Return capability to new process
    0
}

/// sys_cap_drop: Drop capability
pub fn sys_cap_drop(cap: u64, _a2: u64, _a3: u64, _a4: u64, _a5: u64, _a6: u64) -> u64 {
    let _cap = Cap::from_raw(cap);
    // TODO: Remove from process cap table
    0
}
