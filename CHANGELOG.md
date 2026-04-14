# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0] - 2026-04-14

### Governance & Legal
- Add GPL-3.0 LICENSE
- Add CONTRIBUTORS.md with DCO sign-off process (no CLA)
- Add CODE_OF_CONDUCT.md (Contributor Covenant 2.1)
- Add SECURITY.md with vulnerability disclosure policy and CVE SLA
- Configure branch protection rules for `main` and `dev` branches
- Add GitHub issue templates (bug report, kernel panic, driver request, ABI question)
- Add PR template with comprehensive checklist

### Toolchain & Workspace
- Initialize Cargo workspace with 12 crates
- Pin Rust nightly toolchain with required components
- Configure x86_64-unknown-none target with rust-lld linker
- Add kernel linker script (higher-half at 0xffffffff80000000)
- Add Limine bootloader configuration
- Add Makefile with qemu, iso, test, clippy, doc, clean targets
- Add development setup documentation

### Kernel Infrastructure
- **elminux-kernel**: Kernel entry point with panic handler
- **elminux-hal**: Hardware abstraction layer (GDT, IDT, APIC, UART, ACPI, port I/O)
- **elminux-mm**: Memory management (PMM, VMM, slab heap allocator)
- **elminux-sched**: Task scheduler (task, context, round-robin queue)
- **elminux-ipc**: IPC system (capability, message, channel)
- **elminux-drivers**: Driver framework (Driver trait, registry)
- **elminux-syscall**: Syscall ABI (dispatcher, handlers for yield, exit, send, recv, alloc, free, spawn, cap_drop)

### Userland Foundation
- **elminux-std**: Standard library with I/O traits, IPC bindings, threading primitives
- **epkg**: Package manager CLI scaffolding
- **elinit**: Init system (PID 1) scaffolding
- **modsh**: Shell with builtins, interactive features, pipeline support

### Build Tools
- **build-tools**: image-builder for creating bootable ISOs and disk images

### Notes
This is a structural release — no kernel boots yet. All crates are scaffolded with TODO stubs for implementation in v0.2.0+.

[Unreleased]: https://github.com/elminux/elminux/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/elminux/elminux/releases/tag/v0.1.0
