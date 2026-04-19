//! Elminux Scheduler
//!
//! Round-robin task scheduler with voluntary and involuntary preemption.

#![no_std]

extern crate alloc;

pub mod context;
pub mod queue;
pub mod task;

/// Initialize scheduler
pub fn init() {
    // TODO: Create idle task
    // TODO: Initialize run queue
}

/// Schedule next task to run
pub fn schedule() {
    // TODO: Pick next task from run queue
    // TODO: Perform context switch
}

/// Current task yields CPU
pub fn yield_current() {
    // TODO: Move current task to back of queue
    // TODO: Call schedule()
}
