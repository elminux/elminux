//! Elminux synchronization primitives.
//!
//! `no_std`, no external dependencies.  Currently exports:
//! * [`Spinlock<T>`] — a simple test-and-set spinlock suitable for early
//!   boot and uniprocessor operation.
//!
//! This crate is the home for all cross-subsystem synchronization
//! primitives.  Raw unsafe (atomic operations, manual `Send` / `Sync`
//! assertions) is intentionally allowed here, carved out from the
//! "all `unsafe` lives in HAL" policy documented in `ARCHITECTURE.md`.
//!
//! # v0.5 roadmap
//! Per `ARCHITECTURE.md` §Known Risks, allocator contention will be
//! addressed in v0.5+ with per-CPU slab caches and lock-free free lists.
//! The current `Spinlock<T>` is the interim primitive.

#![no_std]
#![deny(unsafe_op_in_unsafe_fn)]

use core::cell::UnsafeCell;
use core::ops::{Deref, DerefMut};
use core::sync::atomic::{AtomicBool, Ordering};

/// A simple test-and-set spinlock.
///
/// Acquisition uses a compare-exchange CAS loop with `spin_loop()` between
/// attempts.  Release is a single `Release` store in the guard's `Drop`.
///
/// # Fairness
/// None.  A starving waiter is possible under contention; acceptable for
/// early-boot and uniprocessor use.  A ticket-lock variant will be added
/// when SMP lands.
///
/// # Interrupts
/// `Spinlock` does **not** disable interrupts.  Calling `lock()` from a
/// context that can be interrupted by a handler which also takes the same
/// lock will deadlock.  For IRQ-safe critical sections, use `IrqSpinlock`
/// (to be added when interrupt delivery to userspace lands).
pub struct Spinlock<T> {
    locked: AtomicBool,
    data: UnsafeCell<T>,
}

// SAFETY: `Spinlock<T>` mediates all access to `T` through a mutex
// acquired via atomic CAS, so shared references are race-free as long as
// `T: Send` (moving ownership across threads).
unsafe impl<T: Send> Sync for Spinlock<T> {}
unsafe impl<T: Send> Send for Spinlock<T> {}

impl<T> Spinlock<T> {
    /// Create a new unlocked `Spinlock` wrapping `value`.
    pub const fn new(value: T) -> Self {
        Self {
            locked: AtomicBool::new(false),
            data: UnsafeCell::new(value),
        }
    }

    /// Acquire the lock, spinning until it is free.
    #[inline]
    pub fn lock(&self) -> SpinlockGuard<'_, T> {
        while self
            .locked
            .compare_exchange_weak(false, true, Ordering::Acquire, Ordering::Relaxed)
            .is_err()
        {
            // Wait until the lock appears free before retrying the CAS.
            while self.locked.load(Ordering::Relaxed) {
                core::hint::spin_loop();
            }
        }
        SpinlockGuard { lock: self }
    }

    /// Try to acquire the lock without spinning.  Returns `None` if
    /// another holder currently owns it.
    #[inline]
    pub fn try_lock(&self) -> Option<SpinlockGuard<'_, T>> {
        match self
            .locked
            .compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)
        {
            Ok(_) => Some(SpinlockGuard { lock: self }),
            Err(_) => None,
        }
    }

    /// Consume the `Spinlock` and return the inner value.
    pub fn into_inner(self) -> T {
        self.data.into_inner()
    }

    /// Mutable access without locking, sound because `&mut self` proves
    /// exclusive access.
    pub fn get_mut(&mut self) -> &mut T {
        self.data.get_mut()
    }
}

/// RAII guard returned by [`Spinlock::lock`].  The lock is released when
/// the guard is dropped.
pub struct SpinlockGuard<'a, T> {
    lock: &'a Spinlock<T>,
}

impl<T> Deref for SpinlockGuard<'_, T> {
    type Target = T;
    #[inline]
    fn deref(&self) -> &T {
        // SAFETY: the guard exists, so we hold the lock; no other
        // reference to `data` can be alive.
        unsafe { &*self.lock.data.get() }
    }
}

impl<T> DerefMut for SpinlockGuard<'_, T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut T {
        // SAFETY: the guard exists, so we hold the lock exclusively.
        unsafe { &mut *self.lock.data.get() }
    }
}

impl<T> Drop for SpinlockGuard<'_, T> {
    #[inline]
    fn drop(&mut self) {
        self.lock.locked.store(false, Ordering::Release);
    }
}
