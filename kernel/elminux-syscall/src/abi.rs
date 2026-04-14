//! Syscall ABI version and compatibility

/// ABI version constant
pub const ABI_VERSION: u32 = 1;

/// Minimum compatible ABI version
pub const ABI_VERSION_MIN: u32 = 1;

/// Check if an ABI version is compatible
pub fn is_compatible(version: u32) -> bool {
    version >= ABI_VERSION_MIN && version <= ABI_VERSION
}

/// ABI information structure (for user space query)
#[repr(C)]
pub struct AbiInfo {
    pub version: u32,
    pub version_min: u32,
    pub target: [u8; 16],
}

impl AbiInfo {
    pub const fn current() -> Self {
        Self {
            version: ABI_VERSION,
            version_min: ABI_VERSION_MIN,
            target: *b"x86_64-unknown\0\0",
        }
    }
}
