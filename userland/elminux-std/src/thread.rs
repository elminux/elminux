//! Threading primitives
//!
//! Thread spawn, IPC-based Mutex (no spinlock in user space).

use core::cell::UnsafeCell;

/// Thread handle
pub struct Thread;

impl Thread {
    /// Spawn a new thread
    pub fn spawn<F>(_f: F) -> Thread
    where
        F: FnOnce() + Send + 'static,
    {
        // TODO: Call sys_spawn
        Thread
    }

    /// Wait for thread to complete
    pub fn join(self) {
        // TODO: Wait via IPC
        let _ = self;
    }
}

/// IPC-based mutex (no user-space spinlock)
pub struct Mutex<T> {
    cap: u64,
    data: UnsafeCell<T>,
}

unsafe impl<T: Send> Send for Mutex<T> {}
unsafe impl<T: Send> Sync for Mutex<T> {}

impl<T> Mutex<T> {
    pub const fn new(value: T) -> Self {
        Self {
            cap: 0,
            data: UnsafeCell::new(value),
        }
    }

    pub fn lock(&self) -> MutexGuard<T> {
        // FIXME(v0.4.0): This is a STUB - no actual lock is acquired.
        // The returned MutexGuard provides DerefMut access without any
        // mutual exclusion. Multiple threads can simultaneously obtain
        // guards and modify data, causing data races.
        //
        // Before use: Implement IPC to kernel mutex service or use
        // atomic-based user-space mutex once we have futex-like support.
        // TODO: Acquire via IPC to kernel mutex service
        MutexGuard { mutex: self }
    }
}

pub struct MutexGuard<'a, T> {
    mutex: &'a Mutex<T>,
}

impl<'a, T> core::ops::Deref for MutexGuard<'a, T> {
    type Target = T;
    fn deref(&self) -> &T {
        unsafe { &*self.mutex.data.get() }
    }
}

impl<'a, T> core::ops::DerefMut for MutexGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.mutex.data.get() }
    }
}

impl<'a, T> Drop for MutexGuard<'a, T> {
    fn drop(&mut self) {
        // TODO: Release via IPC
    }
}
