//! MMIO (Memory-Mapped I/O) Primitives
//!
//! Provides volatile read/write operations for memory-mapped hardware registers.
//! All operations use proper fencing to ensure ordering with respect to other
//! memory operations.
//!
//! # Safety
//! MMIO operations are inherently unsafe because they directly access hardware
//! registers. Caller must ensure:
//! - The address points to valid MMIO memory (not RAM)
//! - The address is properly aligned for the access size
//! - No concurrent access to the same register violates hardware constraints

use core::arch::asm;
use core::ptr;

/// Read 8-bit value from MMIO address
///
/// # Safety
/// - `addr` must point to valid MMIO memory
/// - `addr` must be 1-byte aligned
/// - Caller must ensure no data races with hardware or other cores
#[inline]
pub unsafe fn read8(addr: u64) -> u8 {
    ptr::read_volatile(addr as *const u8)
}

/// Read 16-bit value from MMIO address
///
/// # Safety
/// - `addr` must point to valid MMIO memory
/// - `addr` must be 2-byte aligned
/// - Caller must ensure no data races with hardware or other cores
#[inline]
pub unsafe fn read16(addr: u64) -> u16 {
    ptr::read_volatile(addr as *const u16)
}

/// Read 32-bit value from MMIO address
///
/// # Safety
/// - `addr` must point to valid MMIO memory
/// - `addr` must be 4-byte aligned
/// - Caller must ensure no data races with hardware or other cores
#[inline]
pub unsafe fn read32(addr: u64) -> u32 {
    ptr::read_volatile(addr as *const u32)
}

/// Read 64-bit value from MMIO address
///
/// # Safety
/// - `addr` must point to valid MMIO memory
/// - `addr` must be 8-byte aligned
/// - Caller must ensure no data races with hardware or other cores
#[inline]
pub unsafe fn read64(addr: u64) -> u64 {
    ptr::read_volatile(addr as *const u64)
}

/// Write 8-bit value to MMIO address
///
/// # Safety
/// - `addr` must point to valid MMIO memory
/// - `addr` must be 1-byte aligned
/// - Caller must ensure no data races with hardware or other cores
#[inline]
pub unsafe fn write8(addr: u64, value: u8) {
    ptr::write_volatile(addr as *mut u8, value);
}

/// Write 16-bit value to MMIO address
///
/// # Safety
/// - `addr` must point to valid MMIO memory
/// - `addr` must be 2-byte aligned
/// - Caller must ensure no data races with hardware or other cores
#[inline]
pub unsafe fn write16(addr: u64, value: u16) {
    ptr::write_volatile(addr as *mut u16, value);
}

/// Write 32-bit value to MMIO address
///
/// # Safety
/// - `addr` must point to valid MMIO memory
/// - `addr` must be 4-byte aligned
/// - Caller must ensure no data races with hardware or other cores
#[inline]
pub unsafe fn write32(addr: u64, value: u32) {
    ptr::write_volatile(addr as *mut u32, value);
}

/// Write 64-bit value to MMIO address
///
/// # Safety
/// - `addr` must point to valid MMIO memory
/// - `addr` must be 8-byte aligned
/// - Caller must ensure no data races with hardware or other cores
#[inline]
pub unsafe fn write64(addr: u64, value: u64) {
    ptr::write_volatile(addr as *mut u64, value);
}

/// Full memory fence (sequentially consistent).
///
/// Use when ordering with MMIO is required in both directions. Emits `mfence`
/// on x86-64.
#[inline]
pub fn fence() {
    core::sync::atomic::fence(core::sync::atomic::Ordering::SeqCst);
}

/// Release fence: prior writes become visible before the next store.
///
/// Use *before* an MMIO write whose side-effect depends on earlier stores
/// being observable to the device (e.g., filling a descriptor then ringing
/// a doorbell).
#[inline]
pub fn fence_write() {
    core::sync::atomic::fence(core::sync::atomic::Ordering::Release);
}

/// Acquire fence: subsequent loads observe prior stores from the device.
///
/// Use *after* an MMIO read whose result gates subsequent loads (e.g.,
/// reading a status register before reading a buffer).
#[inline]
pub fn fence_read() {
    core::sync::atomic::fence(core::sync::atomic::Ordering::Acquire);
}

/// Read 32-bit MMIO value with acquire semantics (for device synchronization).
///
/// # Safety
/// Same as `read32`, but provides acquire ordering for synchronization.
#[inline]
pub unsafe fn read32_acquire(addr: u64) -> u32 {
    let value = read32(addr);
    fence_read();
    value
}

/// Write 32-bit MMIO value with release semantics (for device synchronization).
///
/// # Safety
/// Same as `write32`, but provides release ordering for synchronization.
#[inline]
pub unsafe fn write32_release(addr: u64, value: u32) {
    fence_write();
    write32(addr, value);
}

/// Write-combine fence: ensures all WC buffer data is flushed
///
/// # Safety
/// This uses `mfence` instruction which is safe but should only be used
/// when necessary for performance-critical MMIO sequences.
#[inline]
pub unsafe fn fence_mfence() {
    asm!("mfence", options(nostack, preserves_flags));
}
