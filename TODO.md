# Elminux — TODO

Standard: OSS SDLC / DevSecOps
Versioning: SemVer 2.0.0
Language: Rust (nightly, x86_64-unknown-none)
Kernel model: Hybrid (trusted core in kernel space, drivers in user space)
Current release: v0.1.0 (2026-04-14)
Status tags: [ ] todo | [x] done | [~] in-progress | [!] blocked

---

## v0.1.0 — Governance & Toolchain

### 1. Legal & Governance
- [x] 1.1 Commit LICENSE (GPL-3.0)
- [x] 1.2 Write CONTRIBUTING.md (ECC v1.0 — Elminux Contributor Certificate, no CLA)
- [x] 1.3 Write CODE_OF_CONDUCT.md (Contributor Covenant 2.1)
- [x] 1.4 Write SECURITY.md (vulnerability disclosure + CVE SLA)
- [x] 1.5 Confirm project name: Elminux
- [x] 1.6 Register domain (elminux.org)
- [x] 1.7 Set up GitHub org (github.com/elminux)
- [x] 1.8 Configure branch protection (main, dev)
- [x] 1.9 Write issue templates (bug, kernel-panic, driver-request, abi-question)
- [x] 1.10 Write PR template with checklist

### 2. Toolchain & Workspace Setup
- [x] 2.1 Initialize Cargo workspace (`Cargo.toml`)
- [x] 2.2 Pin Rust nightly toolchain (`rust-toolchain.toml`)
  - [x] 2.2.1 Channel: nightly
  - [x] 2.2.2 Components: rust-src, llvm-tools-preview, rustfmt, clippy
  - [x] 2.2.3 Target: x86_64-unknown-none
- [x] 2.3 Add `cargo install cargo-binstall` to dev setup docs
- [x] 2.4 Add `cargo install cargo-skill` to dev setup docs
- [x] 2.5 Configure `.cargo/config.toml`
  - [x] 2.5.1 Default target: x86_64-unknown-none
  - [x] 2.5.2 Build std: `build-std = ["core", "alloc", "compiler_builtins"]`
  - [x] 2.5.3 Linker: rust-lld
- [x] 2.6 Configure linker script (`kernel/linker.ld`) — kernel memory layout
- [x] 2.7 Add Limine bootloader as build dependency
  - [x] 2.7.1 Pin Limine version
  - [x] 2.7.2 Write Limine config (`limine.cfg`)
- [x] 2.8 Write `Makefile` with targets
  - [x] 2.8.1 `make qemu` — build + run QEMU
  - [x] 2.8.2 `make iso` — produce bootable ISO
  - [x] 2.8.3 `make test` — QEMU headless unit tests
  - [x] 2.8.4 `make clippy` — lint all crates
  - [x] 2.8.5 `make doc` — generate docs
  - [x] 2.8.6 `make clean`
- [x] 2.9 Scaffold all crates (empty `lib.rs` / `main.rs`, `Cargo.toml`)
  - [x] 2.9.1 `kernel/elminux-kernel`
  - [x] 2.9.2 `kernel/elminux-hal`
  - [x] 2.9.3 `kernel/elminux-mm`
  - [x] 2.9.4 `kernel/elminux-sched`
  - [x] 2.9.5 `kernel/elminux-ipc`
  - [x] 2.9.6 `kernel/elminux-drivers`
  - [x] 2.9.7 `kernel/elminux-syscall`
  - [x] 2.9.8 `userland/elminux-std`
  - [x] 2.9.9 `userland/epkg`
  - [x] 2.9.10 `userland/elinit`
  - [x] 2.9.11 `userland/modsh` (submodule: [modsh](https://github.com/modsh-shell/modsh))
  - [x] 2.9.12 `tools/build-tools`
- [x] 2.10 Verify workspace builds cleanly (no code yet — structure only)

### 3. CI/CD Pipeline
- [x] 3.1 GitHub Actions: build + clippy on every PR
- [x] 3.2 GitHub Actions: QEMU boot smoke test (headless)
- [x] 3.3 GitHub Actions: doc build verification
- [x] 3.4 GitHub Actions: `deny.toml` — license + advisory check (cargo-deny)
- [x] 3.5 GitHub Actions: SBOM generation (cargo-cyclonedx)
- [x] 3.6 Release pipeline: tag → build ISO → GPG sign → publish

---

## v0.2.0 — Kernel Boot (QEMU)

### 4. Hardware Abstraction Layer (`elminux-hal`)
- [x] 4.1 Implement kernel entry point (`_start`) — Limine protocol
- [x] 4.2 Implement GDT (Global Descriptor Table)
  - [x] 4.2.1 Kernel code + data segments (0x08, 0x10)
  - [x] 4.2.2 User code + data segments (0x18, 0x20)
  - [x] 4.2.3 TSS (Task State Segment at 0x28)
- [x] 4.3 Implement IDT (Interrupt Descriptor Table)
  - [x] 4.3.1 CPU exception handlers (0–31) — generic handler stubs
  - [x] 4.3.2 Panic handler for unhandled exceptions — via #[panic_handler]
  - [x] 4.3.3 IRQ stubs (32+) — placeholder handlers
- [x] 4.4 Implement basic serial output (UART 16550)
  - [x] 4.4.1 `write_byte`, `write_str` to COM1 at 115200 baud
  - [x] 4.4.2 Kernel `print!` / `println!` macros via serial
- [x] 4.5 Implement APIC (Advanced Programmable Interrupt Controller)
  - [x] 4.5.1 Disable legacy PIC (8259) via masking
  - [x] 4.5.2 Initialize local APIC at 0xFEE00000
  - [x] 4.5.3 APIC timer configuration (calibrate + periodic mode)
- [x] 4.6 Basic ACPI table parsing (RSDP → RSDT/XSDT)
  - [x] 4.6.1 Locate RSDP from Limine — parse RSDP v1/v2 with validation
  - [x] 4.6.2 Parse MADT — Local APIC + IO-APIC enumeration
- [x] 4.7 Port I/O primitives (`inb`, `outb`, `inw`, `outw`, `inl`, `outl`)
  - All 6 primitives implemented in `port.rs` with inline assembly
  - 8-bit, 16-bit, 32-bit variants with proper register constraints
- [ ] 4.8 MMIO read/write primitives (volatile, fenced)
- [ ] 4.9 Milestone: kernel boots in QEMU, prints "Elminux v0.2.0" via serial

### 5. Memory Manager (`elminux-mm`)
- [ ] 5.1 Physical memory manager
  - [ ] 5.1.1 Parse memory map from Limine boot info
  - [ ] 5.1.2 Implement buddy allocator (frame granularity: 4KB)
  - [ ] 5.1.3 `alloc_frame()` / `free_frame()`
  - [ ] 5.1.4 Track reserved regions (kernel, firmware, ACPI)
- [ ] 5.2 Virtual memory manager
  - [ ] 5.2.1 4-level page table walker (PML4 → PDPT → PD → PT)
  - [ ] 5.2.2 `map_page(virt, phys, flags)`
  - [ ] 5.2.3 `unmap_page(virt)`
  - [ ] 5.2.4 Higher-half kernel mapping
  - [ ] 5.2.5 TLB flush on unmap
- [ ] 5.3 Kernel heap allocator
  - [ ] 5.3.1 Slab allocator for fixed-size kernel objects
  - [ ] 5.3.2 Global allocator registration (`#[global_allocator]`)
  - [ ] 5.3.3 `alloc` crate available in kernel
- [ ] 5.4 Milestone: kernel allocates heap objects, no panics

---

## v0.3.0 — Scheduling & Process Model

### 6. Scheduler (`elminux-sched`)
- [ ] 6.1 Define `Task` struct (id, state, stack, registers, priority)
- [ ] 6.2 Implement kernel stack allocation per task
- [ ] 6.3 Implement context switch (save/restore registers, x86_64 ABI)
- [ ] 6.4 Implement round-robin scheduler (initial)
  - [ ] 6.4.1 Run queue (VecDeque of ready tasks)
  - [ ] 6.4.2 `schedule()` — pick next task
  - [ ] 6.4.3 APIC timer interrupt → `schedule()`
- [ ] 6.5 Task states: Running, Ready, Blocked, Dead
- [ ] 6.6 `sys_yield()` — voluntary preemption
- [ ] 6.7 `sys_exit()` — task termination + cleanup
- [ ] 6.8 Idle task (runs when no other task is ready)
- [ ] 6.9 Milestone: two kernel tasks context-switching in QEMU

### 7. Performance Benchmarks (v0.3.0 gate)
- [ ] 7.0 Set up benchmark harness (QEMU serial output, TSC-based timing)
- [ ] 7.1 Benchmark: syscall round-trip (`sys_yield()`)
  - [ ] 7.1.1 Target: < 200ns
  - [ ] 7.1.2 Baseline recorded, added to CI performance log
- [ ] 7.2 Benchmark: IPC round-trip (send + recv single message)
  - [ ] 7.2.1 Target: < 500ns
  - [ ] 7.2.2 Baseline recorded, added to CI performance log
- [ ] 7.3 Benchmark: `alloc_frame()` + `free_frame()` cycle
  - [ ] 7.3.1 Target: < 100ns
- [ ] 7.4 Benchmark: page map + unmap cycle
  - [ ] 7.4.1 Target: < 500ns
- [ ] 7.5 Benchmark: boot to kernel entry (QEMU, TSC delta)
  - [ ] 7.5.1 Target: < 500ms to kernel `_start`
- [ ] 7.6 Measure kernel binary size (stripped ELF)
  - [ ] 7.6.1 Target: < 1MB
- [ ] 7.7 Measure idle memory footprint post-boot
  - [ ] 7.7.1 Target: < 16MB
- [ ] 7.8 Add performance regression check to CI
  - [ ] 7.8.1 >20% regression on any target = build failure
  - [ ] 7.8.2 Publish benchmark results in release notes

### 8. IPC Primitives (`elminux-ipc`)
- [ ] 8.1 Define capability model
  - [ ] 8.1.1 `Cap` type — unforgeable token (kernel-managed integer)
  - [ ] 8.1.2 Capability table per process
  - [ ] 8.1.3 Capability rights flags (read, write, grant, revoke)
- [ ] 8.2 Define `Msg` type — fixed-size message (register-sized fields)
- [ ] 8.3 Implement synchronous message passing
  - [ ] 8.3.1 `sys_send(cap, msg)` — blocks until receiver calls recv
  - [ ] 8.3.2 `sys_recv(cap) -> Msg` — blocks until sender calls send
  - [ ] 8.3.3 Rendezvous semantics (no buffering in kernel by default)
- [ ] 8.4 Implement fast path (zero-copy, no allocation on hot path)
- [ ] 8.5 Implement channel pairs (bidirectional)
- [ ] 8.6 Milestone: two processes exchange messages via IPC in QEMU, latency < 500ns

---

## v0.4.0 — Syscall ABI & User Space Foundation

### 8. Syscall ABI (`elminux-syscall`)
- [ ] 8.1 Define ABI version constant (`ABI_VERSION = 1`)
- [ ] 8.2 Define syscall numbers (stable enum)
- [ ] 8.3 Implement syscall entry (SYSCALL/SYSRET, x86_64)
  - [ ] 8.3.1 Set up LSTAR, STAR, SFMASK MSRs
  - [ ] 8.3.2 Kernel syscall dispatcher
- [ ] 8.4 Implement initial syscall set
  - [ ] 8.4.1 `sys_yield()`
  - [ ] 8.4.2 `sys_exit(code)`
  - [ ] 8.4.3 `sys_send(cap, msg)`
  - [ ] 8.4.4 `sys_recv(cap) -> Msg`
  - [ ] 8.4.5 `sys_alloc_pages(n) -> VAddr`
  - [ ] 8.4.6 `sys_free_pages(addr, n)`
  - [ ] 8.4.7 `sys_spawn(manifest) -> Cap`
  - [ ] 8.4.8 `sys_cap_drop(cap)`
- [ ] 8.5 User space enters via ELF loader (kernel-side)
  - [ ] 8.5.1 Minimal ELF64 loader in kernel
  - [ ] 8.5.2 Set up user stack
  - [ ] 8.5.3 Jump to user entry point (SYSRET to ring 3)
- [ ] 8.6 Milestone: first user space process runs, calls sys_exit(0)

### 9. Elminux Standard Library (`elminux-std`)
- [ ] 9.1 Foundation layer
  - [ ] 9.1.1 Re-export `core` and `alloc` primitives
  - [ ] 9.1.2 Custom global allocator (calls `sys_alloc_pages`)
  - [ ] 9.1.3 Panic handler (calls `sys_exit(1)` + print)
- [ ] 9.2 I/O traits
  - [ ] 9.2.1 `Read` trait
  - [ ] 9.2.2 `Write` trait
  - [ ] 9.2.3 `BufRead` trait
- [ ] 9.3 IPC bindings
  - [ ] 9.3.1 Safe wrappers around `sys_send` / `sys_recv`
  - [ ] 9.3.2 Typed channel API
- [ ] 9.4 Threading primitives
  - [ ] 9.4.1 `Thread::spawn()` → `sys_spawn()`
  - [ ] 9.4.2 `Mutex<T>` (IPC-based, no spinlock in user space)
- [ ] 9.5 String types (`ElString`, UTF-8, no C string)
- [ ] 9.6 Collections (re-export `alloc` collections)
- [ ] 9.7 Milestone: user space program uses `elminux-std`, prints via IPC

---

## v0.5.0 — Drivers & Device Layer

### 10. Driver Framework (`elminux-drivers`)
- [ ] 10.1 Define `Driver` trait
  - [ ] 10.1.1 `init() -> Result<Cap>`
  - [ ] 10.1.2 `handle_msg(msg: Msg) -> Msg`
- [ ] 10.2 Driver registry (kernel-side)
- [ ] 10.3 Implement serial driver (user space server)
  - [ ] 10.3.1 UART 16550 via port I/O capability
  - [ ] 10.3.2 IPC interface: `write(bytes)`, `read() -> bytes`
- [ ] 10.4 Implement keyboard driver (user space server)
  - [ ] 10.4.1 PS/2 keyboard via IRQ1
  - [ ] 10.4.2 Scancode → keycode translation
  - [ ] 10.4.3 IPC interface: `read_key() -> KeyEvent`
- [ ] 10.5 Implement framebuffer driver
  - [ ] 10.5.1 Limine framebuffer protocol
  - [ ] 10.5.2 Pixel write, rect fill, blit
  - [ ] 10.5.3 Basic text rendering (bitmap font)
- [ ] 10.6 Implement VirtIO block driver (QEMU virtual disk)
  - [ ] 10.6.1 VirtIO MMIO transport
  - [ ] 10.6.2 Read/write sectors
  - [ ] 10.6.3 IPC interface: `read_block(lba)`, `write_block(lba, data)`
- [ ] 10.7 Milestone: keyboard input displayed on framebuffer in QEMU

---

## v0.6.0 — Filesystem

### 11. Filesystem Server
- [ ] 11.1 Define filesystem IPC protocol
  - [ ] 11.1.1 `open(path, flags) -> Cap`
  - [ ] 11.1.2 `read(cap, buf, len) -> usize`
  - [ ] 11.1.3 `write(cap, buf, len) -> usize`
  - [ ] 11.1.4 `close(cap)`
  - [ ] 11.1.5 `stat(path) -> Stat`
  - [ ] 11.1.6 `readdir(path) -> [DirEntry]`
- [ ] 11.2 Implement in-memory filesystem (initramfs)
  - [ ] 11.2.1 Embedded in kernel image at build time
  - [ ] 11.2.2 Read-only initially
  - [ ] 11.2.3 Stores: `elinit`, `modsh`, initial `epkg` store
- [ ] 11.3 Implement on-disk filesystem (custom, simple)
  - [ ] 11.3.1 Define Elminux FS format (extent-based, append-friendly)
  - [ ] 11.3.2 Read support
  - [ ] 11.3.3 Write support
  - [ ] 11.3.4 `fsck` equivalent tool
- [ ] 11.4 VFS layer (routes FS calls to correct server)
- [ ] 11.5 Milestone: read/write files on VirtIO disk in QEMU

---

## v0.7.0 — Init System & Shell

### 12. Init System (`elinit`)
- [ ] 12.1 PID 1 equivalent — first user process spawned by kernel
- [ ] 12.2 Read capability manifest at boot
- [ ] 12.3 Spawn driver servers in order
  - [ ] 12.3.1 serial → keyboard → framebuffer → block → fs
- [ ] 12.4 Service supervision
  - [ ] 12.4.1 Monitor driver server caps
  - [ ] 12.4.2 Restart crashed services (with backoff)
- [ ] 12.5 Spawn `modsh` after all drivers ready
- [ ] 12.6 Milestone: full boot sequence to shell prompt in QEMU

### 13. Shell — modsh Integration
- [ ] 13.1 Add modsh as git submodule (`userland/modsh`)
  - [ ] 13.1.1 Pin to stable modsh release tag
  - [ ] 13.1.2 Verify Apache-2.0 license compliance (modsh-core + modsh-interactive only)
  - [ ] 13.1.3 Exclude modsh-ai from OS bundle (BSL 1.1 — GPL incompatible)
- [ ] 13.2 Port modsh-core to `elminux-std`
  - [ ] 13.2.1 Audit all `std` dependencies in modsh-core
  - [ ] 13.2.2 Replace `std::fs` calls → elminux filesystem IPC
  - [ ] 13.2.3 Replace `std::process` → `sys_spawn` capability
  - [ ] 13.2.4 Replace `std::io` → `elminux-std` I/O traits
  - [ ] 13.2.5 Replace `std::env` → elminux environment capability
- [ ] 13.3 Port modsh-interactive to `elminux-std`
  - [ ] 13.3.1 Replace terminal I/O → keyboard + framebuffer driver IPC
  - [ ] 13.3.2 Verify line editing, history, completion on elminux
- [ ] 13.4 Verify modsh built-ins work on elminux
  - [ ] 13.4.1 `ls`, `cat`, `echo`, `cd`, `exit`
  - [ ] 13.4.2 Structured pipeline output via elminux IPC
- [ ] 13.5 Milestone: modsh interactive prompt usable in QEMU on elminux
- [ ] 13.6 modsh-ai (optional epkg package — post v1.0.0)
  - [ ] 13.6.1 Package as `modsh-ai.epkg` (BSL 1.1, separate install)
  - [ ] 13.6.2 Document license difference clearly in epkg manifest

---

## v0.8.0 — Package Manager (`epkg`)

### 14. Package Format
- [ ] 14.1 Define `.epkg` format (tar.zst + `epkg.toml` manifest)
- [ ] 14.2 Define `epkg.toml` schema
  - [ ] 14.2.1 `[package]` — name, version, abi_version, license
  - [ ] 14.2.2 `[capabilities]` — required, optional
  - [ ] 14.2.3 `[dependencies]` — name + hash-pinned version
- [ ] 14.3 Define package signing (Ed25519)
  - [ ] 14.3.1 Key generation tool
  - [ ] 14.3.2 Sign command
  - [ ] 14.3.3 Verify on install

### 15. Package Store
- [ ] 15.1 Define store layout (`/pkg/store`, `/pkg/active`, `/pkg/keys`)
- [ ] 15.2 Implement hash-addressed store (blake3)
- [ ] 15.3 Implement atomic install (stage → verify → link)
- [ ] 15.4 Implement rollback (relink previous version)

### 16. epkg CLI
- [ ] 16.1 `epkg install <package>`
- [ ] 16.2 `epkg remove <package>`
- [ ] 16.3 `epkg list`
- [ ] 16.4 `epkg verify` — verify all installed package signatures
- [ ] 16.5 `epkg build` — build `.epkg` from source manifest
- [ ] 16.6 Offline-first: all operations work without network
- [ ] 16.7 Milestone: install/remove a package in QEMU

---

## v0.9.0 — Security Hardening & Testing

### 17. Security
- [ ] 17.1 Audit all `unsafe` blocks in `elminux-hal` — document each
- [ ] 17.2 Add `#![forbid(unsafe_code)]` to all non-hal crates
- [ ] 17.3 Implement capability audit log (append-only, kernel-side)
- [ ] 17.4 Add KASLR (kernel address space layout randomization)
- [ ] 17.5 Stack canaries in kernel (via compiler flag)
- [ ] 17.6 Implement `cargo-deny` policy (no GPL-incompatible deps)
- [ ] 17.7 SBOM generation on every release (cargo-cyclonedx)

### 18. Testing
- [ ] 18.1 Kernel unit tests (QEMU headless, `x86_64-unknown-none`)
  - [ ] 18.1.1 Memory allocator tests
  - [ ] 18.1.2 Page table tests
  - [ ] 18.1.3 IPC message passing tests
  - [ ] 18.1.4 Scheduler round-robin tests
- [ ] 18.2 Integration tests (full boot in QEMU, scripted)
  - [ ] 18.2.1 Boot to shell prompt
  - [ ] 18.2.2 File read/write
  - [ ] 18.2.3 Package install/remove
  - [ ] 18.2.4 Driver crash + restart
- [ ] 18.3 Fuzzing
  - [ ] 18.3.1 Syscall fuzzer (cargo-fuzz)
  - [ ] 18.3.2 IPC message fuzzer
  - [ ] 18.3.3 epkg manifest parser fuzzer

---

## v1.0.0 — Stable Experimental Release

### 19. Pre-release
- [ ] 19.1 All kernel unit tests passing in CI
- [ ] 19.2 Full boot sequence stable in QEMU (100 consecutive boots, no panic)
- [ ] 19.3 Syscall ABI frozen at v1 (no breaking changes after this)
- [ ] 19.4 SBOM published
- [ ] 19.5 All `unsafe` blocks documented with safety invariants
- [ ] 19.6 External review of kernel memory safety (academic partner)

### 20. Release
- [ ] 20.1 Tag v1.0.0
- [ ] 20.2 Publish signed ISO (QEMU-tested)
- [ ] 20.3 Publish SBOM
- [ ] 20.4 Publish ABI v1 specification document
- [ ] 20.5 Announce (OSDev community, Rust forums, elminux.org)

---

## Post v1.0.0 Backlog

### Hardware Expansion
- [ ] B.1 Bare metal x86_64 boot (real hardware, start with one tested board)
- [ ] B.2 NVMe driver
- [ ] B.3 USB (xHCI) driver
- [ ] B.4 AHCI (SATA) driver
- [ ] B.5 Intel/AMD NIC driver (e1000 / virtio-net first)
- [ ] B.6 aarch64 port (Raspberry Pi 4)

### System
- [ ] B.7 SMP (multi-core scheduler)
- [ ] B.8 Network stack (TCP/IP in Rust, user space server)
- [ ] B.9 TLS (rustls-based)
- [ ] B.10 Display server (Wayland-inspired, no X11)

### Userland
- [ ] B.11 epkg repository server
- [ ] B.12 Port Rust std applications to elminux-std
- [ ] B.13 Text editor (port helix or build minimal)
- [ ] B.14 Local AI layer (llama.cpp-equivalent in Rust, 2GB RAM target)

### Mission Profiles
- [ ] B.15 Education profile
- [ ] B.16 Medical profile
- [ ] B.17 Agricultural profile
- [ ] B.18 Sovereignty profile (airgap, audit-ready)

### Governance
- [ ] B.19 Foundation/legal entity
- [ ] B.20 Sovereignty certification pathway (ANSSI-compatible)
- [ ] B.21 NGO hardware distribution partnerships
