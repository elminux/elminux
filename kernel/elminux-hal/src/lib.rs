//! Elminux Hardware Abstraction Layer
//!
//! Platform-specific code for x86_64: GDT, IDT, APIC, UART, ACPI parsing.

#![no_std]

pub mod gdt;
pub mod idt;
pub mod apic;
pub mod uart;
pub mod acpi;
pub mod port;
