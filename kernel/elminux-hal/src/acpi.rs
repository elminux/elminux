//! ACPI Table Parsing
//!
//! Parse RSDP, RSDT/XSDT, MADT for interrupt routing.

/// RSDP (Root System Description Pointer)
#[repr(C, packed)]
pub struct Rsdp {
    signature: [u8; 8],
    checksum: u8,
    oem_id: [u8; 6],
    revision: u8,
    rsdt_address: u32,
    // ACPI 2.0+ fields...
}

/// Parse ACPI tables from Limine-provided RSDP
pub fn init(_rsdp: u64) {
    // TODO: Parse RSDP
    // TODO: Walk RSDT/XSDT
    // TODO: Parse MADT for APIC info
}
