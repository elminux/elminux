//! Driver registry (kernel-side)

use crate::driver::Driver;

/// Registered driver entry
pub struct DriverEntry {
    pub name: &'static str,
    pub driver: &'static dyn Driver,
}

/// Global driver registry
pub struct Registry {
    entries: [Option<DriverEntry>; 16],
    count: usize,
}

impl Registry {
    pub const fn new() -> Self {
        Self {
            entries: [const { None }; 16],
            count: 0,
        }
    }

    #[allow(clippy::result_unit_err)] // Simple error type for stub implementation
    pub fn register(&mut self, name: &'static str, driver: &'static dyn Driver) -> Result<(), ()> {
        if self.count >= 16 {
            return Err(());
        }
        self.entries[self.count] = Some(DriverEntry { name, driver });
        self.count += 1;
        Ok(())
    }

    pub fn find(&self, name: &str) -> Option<&DriverEntry> {
        for i in 0..self.count {
            if let Some(ref entry) = self.entries[i] {
                if entry.name == name {
                    return Some(entry);
                }
            }
        }
        None
    }
}

impl Default for Registry {
    fn default() -> Self {
        Self::new()
    }
}

/// Global driver registry instance
// FIXME(v1.0-SMP): `static mut` requires unsafe on every access and has no
// interior mutability protection. Replace with Mutex<Registry> or spin::Once
// before adding multi-core support to avoid data races.
static mut REGISTRY: Registry = Registry::new();

/// Register a driver with the system
#[allow(clippy::result_unit_err)] // Simple error type for stub implementation
pub fn register(name: &'static str, driver: &'static dyn Driver) -> Result<(), ()> {
    unsafe { REGISTRY.register(name, driver) }
}

/// Find a registered driver by name
pub fn find(name: &str) -> Option<&'static DriverEntry> {
    unsafe { REGISTRY.find(name) }
}
