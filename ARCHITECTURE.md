# Elminux — Architecture

Version: 0.1.0-draft
Target: x86_64 (QEMU first, bare metal second)

---

## Overview

Elminux is a hybrid-kernel operating system written entirely in Rust.
It has no Linux kernel, no GNU userland, no libc, and no POSIX ABI.
Every layer — from interrupt handling to the package manager — is
written in Rust with no C dependencies except at the absolute
hardware boundary, which is isolated in `elminux-hal`.

---

## Kernel Architecture: Hybrid Model

The hybrid model places a minimal trusted core in kernel space
and isolates drivers and services in user space via IPC.
Performance-critical paths may be promoted to kernel space
explicitly — but this is an opt-in exception, not the default.

```
┌─────────────────────────────────────────────────────────┐
│                    User Space                           │
│                                                         │
│  ┌──────────┐ ┌──────────┐ ┌──────────┐ ┌──────────┐  │
│  │  Driver  │ │    FS    │ │ Network  │ │  epkg    │  │
│  │ servers  │ │  server  │ │  stack   │ │  daemon  │  │
│  └────┬─────┘ └────┬─────┘ └────┬─────┘ └────┬─────┘  │
│       │             │             │             │        │
│  ─────┴─────────────┴─────────────┴─────────────┴────── │
│                 IPC / Capability Layer                   │
├─────────────────────────────────────────────────────────┤
│                   Kernel Space (Trusted Core)           │
│                                                         │
│  ┌────────────┐  ┌────────────┐  ┌────────────────────┐ │
│  │   Memory   │  │ Scheduler  │  │  Capability & IPC  │ │
│  │  Manager   │  │  (elminux- │  │  primitives        │ │
│  │ (elminux-  │  │   sched)   │  │  (elminux-ipc)     │ │
│  │    mm)     │  └────────────┘  └────────────────────┘ │
│  └────────────┘                                         │
│  ┌────────────────────────────────────────────────────┐ │
│  │         Hardware Abstraction Layer (elminux-hal)   │ │
│  │    x86_64: IDT │ GDT │ paging │ APIC │ ACPI       │ │
│  └────────────────────────────────────────────────────┘ │
├─────────────────────────────────────────────────────────┤
│                      Hardware                           │
│         x86_64 CPU │ RAM │ Storage │ NIC │ USB          │
└─────────────────────────────────────────────────────────┘
```

---

## Crate Responsibilities

### Kernel Space

**`elminux-hal`** — Hardware Abstraction Layer
- x86_64 only initially
- IDT (Interrupt Descriptor Table) setup
- GDT (Global Descriptor Table)
- 4-level page table management
- APIC (interrupt controller)
- ACPI (power management, device enumeration)
- Port I/O and MMIO primitives
- All `unsafe` in the project is contained here by policy

**`elminux-mm`** — Memory Manager
- Physical frame allocator (buddy allocator)
- Virtual memory manager (page table walker)
- Kernel heap allocator (slab allocator)
- User space memory regions
- No unsafe except via `elminux-hal` boundary

**`elminux-sched`** — Scheduler
- Preemptive, priority-based
- Round-robin initially → CFS-inspired in v0.5+
- Task/thread lifecycle (create, block, wake, terminate)
- Per-CPU run queues (SMP groundwork, single-CPU first)

**`elminux-ipc`** — IPC Primitives
- Synchronous message passing (primary mechanism)
- Capability tokens (unforgeable references to kernel objects)
- Channel pairs (bidirectional, typed)
- No shared memory between processes by default

**`elminux-syscall`** — Syscall ABI Definition
- Shared between kernel and userland (no duplication)
- Not POSIX — custom capability-based ABI
- Versioned from day one (`abi_version` in manifest)
- Syscall numbers are stable once published

**`elminux-drivers`** — Driver Framework
- Driver trait definitions
- Driver registry
- Initial drivers (all in user space via framework):
  - `serial` — UART 16550 (debug output first)
  - `keyboard` — PS/2 keyboard
  - `vga` / `framebuffer` — basic display
  - `ata` / `nvme` — storage (milestone 3+)
  - `virtio` — QEMU virtio block/net (milestone 2)

**`elminux-kernel`** — Kernel Entry + Orchestration
- Boot entry point (Limine protocol)
- Initializes hal → mm → sched → ipc in order
- Spawns initial user space process (elinit)
- Kernel panic handler

### User Space

**`elminux-std`** — Rust-Native Standard Library
- Replaces libc entirely
- Built on `core` + `alloc` (no_std foundation)
- Provides: allocator, string types, I/O traits,
  threading primitives, collections, time
- Targets `elminux-syscall` ABI directly
- No C FFI

**`elinit`** — Init System
- PID 1 equivalent
- Reads capability manifest at boot
- Spawns driver servers, filesystem server, network stack
- Service supervision (restart on crash)
- Structured in Rust, capability-aware

**`modsh`** — Shell
- Replaced by **modsh** (`github.com/modsh-shell/modsh`)
- Uses `modsh-core` + `modsh-interactive` crates (Apache-2.0, GPL-3.0 compatible)
- `modsh-ai` (BSL 1.1) excluded from OS bundle — available as optional `epkg` package
- POSIX-compatible core + structured data pipelines (nushell-inspired, opt-in)
- **Porting requirement:** modsh currently targets Rust `std` (POSIX/Linux syscalls)
  Must be ported to `elminux-std` once `elminux-syscall` ABI is stable (v0.4.0+)
- Until ported: modsh runs on host for development; bundled in OS at v0.7.0

**`epkg`** — Package Manager
- See Package Manager section below

---

## Boot Sequence

```
Power on
    │
    ▼
Limine bootloader
    │  (loads kernel ELF, sets up framebuffer, passes boot info)
    ▼
elminux-kernel entry (_start)
    │
    ├── elminux-hal::init()     — IDT, GDT, paging bootstrap
    ├── elminux-mm::init()      — frame allocator, heap
    ├── elminux-ipc::init()     — capability table
    ├── elminux-sched::init()   — task queue
    │
    ▼
Spawn elinit (first user process)
    │
    ├── Spawn serial driver server
    ├── Spawn keyboard driver server
    ├── Spawn framebuffer driver server
    ├── Spawn filesystem server
    │
    ▼
modsh (interactive shell, modsh-core + modsh-interactive)
    │
    ▼
[System ready]
```

---

## Memory Layout (x86_64)

```
0xFFFF_FFFF_FFFF_FFFF  ┐
                        │  Kernel space (higher half)
0xFFFF_8000_0000_0000  ┘
        (non-canonical gap)
0x0000_7FFF_FFFF_FFFF  ┐
                        │  User space
0x0000_0000_0010_0000  ┘
0x0000_0000_0000_0000     Reserved / NULL guard
```

- Kernel mapped in higher half (standard x86_64 convention)
- Each process has isolated page table (no shared kernel mappings in user range)
- KASLR: planned for v0.6+

---

## Syscall ABI

Elminux does not implement POSIX. The syscall ABI is:

- Capability-based: every operation requires a capability token
- Versioned: `abi_version` field in process manifest
- Minimal: syscalls are coarse operations on kernel objects
- Defined in `elminux-syscall` crate (shared crate, no duplication)

Initial syscall set (v0.1 ABI):
```
sys_yield()                          — yield CPU
sys_exit(code: i32)                  — terminate process
sys_send(cap: Cap, msg: Msg)         — send IPC message
sys_recv(cap: Cap) -> Msg            — receive IPC message
sys_alloc_pages(n: usize) -> VAddr   — allocate virtual pages
sys_free_pages(addr: VAddr, n: usize)
sys_spawn(manifest: &Manifest) -> Cap — spawn process
sys_cap_drop(cap: Cap)               — release capability
```

---

## Package Manager: epkg

**Format:** `.epkg` — tar.zst archive with TOML manifest

**Manifest (`epkg.toml`):**
```toml
[package]
name = "modsh"
version = "0.1.0"
abi_version = 1
author = "Elminux Project"
license = "GPL-3.0"

[capabilities]
required = ["fs.read", "tty.write"]
optional = ["net.connect"]

[dependencies]
elminux-std = "0.1"
```

**Design principles:**
- Cryptographically signed (Ed25519)
- Immutable packages (hash-addressed store)
- Atomic installs (no partial state)
- Capability-aware (package declares required capabilities)
- Offline-first (local cache, no network required)
- Reproducible builds (manifest pins all dependency hashes)

**Package store layout:**
```
/pkg/
├── store/          # Immutable, hash-addressed package blobs
├── active/         # Symlinks to current active versions
├── manifests/      # TOML manifests for installed packages
└── keys/           # Trusted signing keys
```

---

## Performance Design

Lightweight and fast are explicit design constraints, not aspirational.
Every architectural decision is evaluated against the performance targets below.

### Targets

| Metric | Target | Measurement Point |
|---|---|---|
| IPC round-trip latency | < 500ns | kernel↔driver single message |
| Syscall overhead | < 200ns | `sys_yield()` round-trip |
| Boot to shell (QEMU) | < 2s | power-on to `modsh` prompt |
| Idle memory footprint | < 16MB | post-boot, no user apps |
| Kernel binary size | < 1MB | stripped ELF |

### Structural Advantages

- No GC, no runtime, no VM — same floor as C, safer ceiling
- No libc — no glibc startup overhead, no dynamic linker for kernel
- Clean syscall ABI — no POSIX translation layer
- Purpose-built allocator — tuned for actual allocation patterns
- Hybrid model — performance-critical paths promotable to kernel space

### Known Risks and Mitigations

| Risk | Mitigation |
|---|---|
| IPC overhead per driver call | Zero-copy message passing where possible; async IPC paths for high-throughput drivers; batch operations |
| Bounds check overhead | Profile-guided elimination; use `get_unchecked` only inside `elminux-hal` with documented invariants |
| Monomorphization bloat | Audit generic usage in hot paths; prefer dynamic dispatch at stable ABI boundaries |
| Allocator contention | Per-CPU slab caches (v0.5+); lock-free free list for hot sizes |
| Scheduler latency | Round-robin only for v0.2; priority preemption + CFS-inspired fairness by v0.5 |

### IPC Fast Path Design

The single most critical number is IPC round-trip latency.
A naïve implementation will be 2–10µs. Target is < 500ns.

Design requirements for the fast path:
- Synchronous rendezvous: sender blocks, kernel directly copies registers to receiver — no buffer allocation
- Kernel-side channel table: O(1) cap lookup, no hash collision
- No allocation on the IPC hot path — all state is pre-allocated per channel
- Benchmarked in CI on every build (regression = blocker)

---

## Security Model

| Control | Implementation |
|---|---|
| Memory safety | Rust ownership — enforced at compile time |
| Process isolation | Separate page tables, capability-based access |
| Driver isolation | Drivers run in user space, kernel boundary via IPC |
| No ambient authority | Every operation requires explicit capability |
| Signed packages | Ed25519 signatures on all `.epkg` artifacts |
| No libc | Eliminates entire class of C runtime vulnerabilities |
| Reproducible builds | Hash-pinned dependency graph |
| Audit log | Kernel-level capability audit trail (planned v0.5+) |

---

## Build System

**Toolchain:** Rust nightly (`x86_64-unknown-none` target)

```toml
# rust-toolchain.toml
[toolchain]
channel = "nightly"
components = ["rust-src", "llvm-tools-preview", "rustfmt", "clippy"]
targets = ["x86_64-unknown-none"]
```

**Build targets:**
```
make qemu          — build + run in QEMU (primary dev workflow)
make iso           — produce bootable ISO
make test          — run kernel unit tests (QEMU headless)
make clippy        — lint all crates
make doc           — generate crate docs
```

**QEMU invocation:**
```bash
qemu-system-x86_64 \
  -cdrom elminux.iso \
  -m 512M \
  -serial stdio \
  -display gtk \
  -no-reboot
```

---

## Versioning

SemVer 2.0.0:
- `MAJOR` — syscall ABI break or kernel ABI break
- `MINOR` — new subsystem, new driver, new userland component
- `PATCH` — bug fixes, security patches

Kernel ABI and syscall ABI are versioned independently.
ABI stability is not guaranteed until v1.0.0.

---

## What This Is Not

- Not a Linux distribution
- Not POSIX-compatible
- Not production-ready
- Not suitable for end users at this stage
