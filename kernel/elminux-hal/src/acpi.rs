//! ACPI Table Parsing
//!
//! Parse RSDP, RSDT/XSDT, MADT for interrupt routing and platform info.
//!
//! This module provides:
//! - RSDP validation and parsing (ACPI 1.0 and 2.0+)
//! - RSDT/XSDT enumeration
//! - MADT parsing for local APIC and IO-APIC info

use core::mem;

/// RSDP (Root System Description Pointer) - ACPI 1.0 structure
///
/// Located in EBDA or BIOS memory area. Signature must be "RSD PTR ".
#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct RsdpV1 {
    pub signature: [u8; 8],
    pub checksum: u8,
    pub oem_id: [u8; 6],
    pub revision: u8,
    pub rsdt_address: u32,
}

/// RSDP Extended - ACPI 2.0+ structure
#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct RsdpV2 {
    pub v1: RsdpV1,
    pub length: u32,
    pub xsdt_address: u64,
    pub extended_checksum: u8,
    pub reserved: [u8; 3],
}

/// SDT (System Description Table) Header
///
/// Common header present in all ACPI tables.
#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct SdtHeader {
    pub signature: [u8; 4],
    pub length: u32,
    pub revision: u8,
    pub checksum: u8,
    pub oem_id: [u8; 6],
    pub oem_table_id: [u8; 8],
    pub oem_revision: u32,
    pub creator_id: u32,
    pub creator_revision: u32,
}

/// MADT (Multiple APIC Description Table) Header
#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct MadtHeader {
    pub header: SdtHeader,
    pub local_apic_addr: u32,
    pub flags: u32,
}

/// MADT Entry Type
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MadtEntryType {
    LocalApic = 0,
    IoApic = 1,
    InterruptSourceOverride = 2,
    NmiSource = 3,
    LocalApicNmi = 4,
    LocalApicAddressOverride = 5,
    IoSapic = 6,
    LocalSapic = 7,
    PlatformInterruptSources = 8,
    LocalX2Apic = 9,
    LocalX2ApicNmi = 10,
}

/// MADT Local APIC Entry
#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct MadtLocalApic {
    pub header: MadtEntryHeader,
    pub acpi_processor_id: u8,
    pub apic_id: u8,
    pub flags: u32,
}

/// MADT IO-APIC Entry
#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct MadtIoApic {
    pub header: MadtEntryHeader,
    pub io_apic_id: u8,
    pub reserved: u8,
    pub io_apic_address: u32,
    pub global_irq_base: u32,
}

/// MADT Entry Header (common to all entry types)
#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct MadtEntryHeader {
    pub entry_type: u8,
    pub length: u8,
}

/// APIC Information from MADT
#[derive(Debug, Clone, Copy)]
pub struct ApicInfo {
    pub local_apic_addr: u32,
    pub flags: u32,
    pub processor_count: usize,
    pub io_apic_count: usize,
}

impl Default for ApicInfo {
    fn default() -> Self {
        Self {
            local_apic_addr: 0xFEE00000,
            flags: 0,
            processor_count: 1,
            io_apic_count: 0,
        }
    }
}

/// Calculate checksum over a byte slice
///
/// ACPI checksum: sum of all bytes must equal 0 (mod 256)
fn calculate_checksum(data: &[u8]) -> u8 {
    data.iter().fold(0u8, |acc, &b| acc.wrapping_add(b))
}

/// Verify ACPI table checksum
fn verify_checksum(data: &[u8]) -> bool {
    calculate_checksum(data) == 0
}

/// Validate RSDP v1 signature
fn validate_rsdp_signature(rsdp: &RsdpV1) -> bool {
    &rsdp.signature == b"RSD PTR "
}

/// Parse and validate RSDP (ACPI 1.0 or 2.0+)
///
/// # Safety
/// `rsdp_addr` must point to a valid RSDP structure in memory.
/// The RSDP must be mapped and accessible.
pub unsafe fn parse_rsdp(rsdp_addr: u64) -> Option<(RsdpV1, Option<RsdpV2>)> {
    let v1_ptr = rsdp_addr as *const RsdpV1;
    let v1 = unsafe { *v1_ptr };

    // Validate signature
    if !validate_rsdp_signature(&v1) {
        return None;
    }

    // Validate v1 checksum
    let v1_size = mem::size_of::<RsdpV1>();
    let v1_slice = unsafe { core::slice::from_raw_parts(rsdp_addr as *const u8, v1_size) };
    if !verify_checksum(v1_slice) {
        return None;
    }

    // Check for ACPI 2.0+ (revision >= 2)
    let v2 = if v1.revision >= 2 {
        let v2_size = mem::size_of::<RsdpV2>();
        let v2_slice = unsafe { core::slice::from_raw_parts(rsdp_addr as *const u8, v2_size) };

        // Validate extended checksum
        if verify_checksum(v2_slice) {
            Some(unsafe { *(rsdp_addr as *const RsdpV2) })
        } else {
            None
        }
    } else {
        None
    };

    Some((v1, v2))
}

/// Get SDT header from physical address
///
/// # Safety
/// `addr` must point to a valid ACPI table.
unsafe fn get_sdt_header(addr: u64) -> SdtHeader {
    unsafe { *(addr as *const SdtHeader) }
}

/// Find an ACPI table by signature
///
/// # Arguments
/// * `rsdp_addr` - Physical address of RSDP
/// * `signature` - 4-byte table signature (e.g., "APIC", "HPET")
///
/// # Returns
/// Physical address of table header, or None if not found
///
/// # Safety
/// `rsdp_addr` must be a valid RSDP.
pub unsafe fn find_table(rsdp_addr: u64, signature: &[u8; 4]) -> Option<u64> {
    let (rsdp_v1, rsdp_v2) = parse_rsdp(rsdp_addr)?;

    // Use XSDT if available (ACPI 2.0+) with non-zero address, otherwise fall back to RSDT.
    // Some firmware has rsdp.revision >= 2 with a valid checksum but xsdt_address == 0.
    let use_xsdt = rsdp_v2.map(|v2| v2.xsdt_address != 0).unwrap_or(false);

    if use_xsdt {
        // XSDT contains 64-bit physical addresses
        let xsdt_addr = rsdp_v2.unwrap().xsdt_address;
        let xsdt_header = get_sdt_header(xsdt_addr);

        // Calculate number of entries: (total size - header size) / 8
        let xsdt_len = xsdt_header.length as usize;
        if xsdt_len < mem::size_of::<SdtHeader>() {
            return None;
        }
        let entry_count = (xsdt_len - mem::size_of::<SdtHeader>()) / 8;

        for i in 0..entry_count {
            let entry_addr = xsdt_addr + mem::size_of::<SdtHeader>() as u64 + (i * 8) as u64;
            // XSDT entries start at offset 36 → not 8-byte aligned. Use read_unaligned.
            let table_addr = unsafe { core::ptr::read_unaligned(entry_addr as *const u64) };
            if table_addr == 0 {
                continue;
            }
            let table_header = get_sdt_header(table_addr);

            if &table_header.signature == signature {
                return Some(table_addr);
            }
        }
    } else {
        // RSDT contains 32-bit physical addresses
        let rsdt_addr = rsdp_v1.rsdt_address as u64;
        if rsdt_addr == 0 {
            return None;
        }
        let rsdt_header = get_sdt_header(rsdt_addr);

        // Calculate number of entries: (total size - header size) / 4
        let rsdt_len = rsdt_header.length as usize;
        if rsdt_len < mem::size_of::<SdtHeader>() {
            return None;
        }
        let entry_count = (rsdt_len - mem::size_of::<SdtHeader>()) / 4;

        for i in 0..entry_count {
            let entry_addr = rsdt_addr + mem::size_of::<SdtHeader>() as u64 + (i * 4) as u64;
            let table_addr = unsafe { *(entry_addr as *const u32) } as u64;
            if table_addr == 0 {
                continue;
            }
            let table_header = get_sdt_header(table_addr);

            if &table_header.signature == signature {
                return Some(table_addr);
            }
        }
    }

    None
}

/// Parse MADT (Multiple APIC Description Table)
///
/// # Safety
/// `madt_addr` must point to a valid MADT table.
pub unsafe fn parse_madt(madt_addr: u64) -> Option<ApicInfo> {
    let madt_header = unsafe { *(madt_addr as *const MadtHeader) };

    // Verify MADT signature
    if &madt_header.header.signature != b"APIC" {
        return None;
    }

    // Verify checksum
    let madt_slice = unsafe {
        core::slice::from_raw_parts(madt_addr as *const u8, madt_header.header.length as usize)
    };
    if !verify_checksum(madt_slice) {
        return None;
    }

    let mut info = ApicInfo {
        local_apic_addr: madt_header.local_apic_addr,
        flags: madt_header.flags,
        processor_count: 0,
        io_apic_count: 0,
    };

    // Parse MADT entries
    let entries_start = madt_addr + mem::size_of::<MadtHeader>() as u64;
    let entries_end = madt_addr + madt_header.header.length as u64;
    let mut current = entries_start;

    while current + (mem::size_of::<MadtEntryHeader>() as u64) <= entries_end {
        let entry_header = unsafe { *(current as *const MadtEntryHeader) };

        // Bounds-check: ensure the full entry fits inside the table.
        let entry_len = entry_header.length as u64;
        if entry_len < mem::size_of::<MadtEntryHeader>() as u64 || current + entry_len > entries_end
        {
            break;
        }

        match entry_header.entry_type {
            0 => {
                // Local APIC entry
                let entry = unsafe { *(current as *const MadtLocalApic) };
                // Check if processor is enabled (bit 0 of flags)
                if entry.flags & 1 != 0 {
                    info.processor_count += 1;
                }
            }
            1 => {
                // IO-APIC entry
                info.io_apic_count += 1;
            }
            _ => {
                // Other entry types - skip
            }
        }

        current += entry_len;
    }

    Some(info)
}

/// Parse ACPI tables from Limine-provided RSDP
///
/// # Safety
/// `rsdp` must be a valid physical address of the RSDP.
pub unsafe fn init(rsdp: u64) -> Option<ApicInfo> {
    // Parse RSDP
    let (_rsdp_v1, rsdp_v2) = parse_rsdp(rsdp)?;

    // Log ACPI version
    let version = if rsdp_v2.is_some() { "2.0+" } else { "1.0" };
    crate::uart::write_str("[ACPI] Found RSDP (version ");
    crate::uart::write_str(version);
    crate::uart::write_str(")\n");

    // Find MADT
    let madt_addr = find_table(rsdp, b"APIC")?;
    crate::uart::write_str("[ACPI] Found MADT\n");

    // Parse MADT
    parse_madt(madt_addr)
}
