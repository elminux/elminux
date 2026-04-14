//! Task structure and management

use core::sync::atomic::{AtomicU64, Ordering};

/// Unique task ID
static NEXT_TID: AtomicU64 = AtomicU64::new(1);

/// Task states
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TaskState {
    Running,
    Ready,
    Blocked,
    Dead,
}

/// Task structure
#[derive(Debug)]
pub struct Task {
    pub id: u64,
    pub state: TaskState,
    pub stack: u64,
    pub priority: u8,
    // TODO: Register save area
}

impl Task {
    pub fn new() -> Self {
        Self {
            id: NEXT_TID.fetch_add(1, Ordering::SeqCst),
            state: TaskState::Ready,
            stack: 0,
            priority: 0,
        }
    }
}
