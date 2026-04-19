//! Elminux Hardware Abstraction Layer
//!
//! Platform-specific code for x86_64: GDT, IDT, APIC, UART, ACPI, MMIO.

#![no_std]

pub mod acpi;
pub mod apic;
pub mod gdt;
pub mod idt;
pub mod mmio;
pub mod port;
pub mod uart;
