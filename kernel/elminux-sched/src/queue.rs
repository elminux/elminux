//! Round-robin run queue

use crate::task::Task;
use alloc::collections::VecDeque;

/// Task run queue
pub struct RunQueue {
    queue: VecDeque<Task>,
}

impl RunQueue {
    pub const fn new() -> Self {
        Self {
            queue: VecDeque::new(),
        }
    }

    pub fn push_back(&mut self, task: Task) {
        self.queue.push_back(task);
    }

    pub fn pop_front(&mut self) -> Option<Task> {
        self.queue.pop_front()
    }

    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }
}
