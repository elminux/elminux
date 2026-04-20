# Elminux — Architecture

Version: 0.1.1-draft
Target: x86_64 (QEMU first, bare metal second)

---

## Changes since 0.1.0-draft

- Clarified "hybrid" terminology — enumerated kernel-space exceptions (currently one: IPC fast path)
- Split driver framework description: shared trait/ABI crate vs. driver binaries (userspace)
- Documented early-boot vs. production serial ownership
- Added identity-map teardown step to boot sequence
- New section: **Capability Model** (derivation, revocation, CDT, notifications, IRQ delivery)
- New section: **Microarchitectural Security Posture** (KPTI/Spectre/L1TF/MDS/Retbleed stance)
- New section: **Multi-Architecture Strategy** (HAL split schedule)
- New section: **Build Reproducibility** (SLSA provenance, dated toolchain)

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

**What "hybrid" means in Elminux, concretely.** The default placement is
microkernel-shaped: scheduler, memory manager, capability + IPC primitives,
and HAL live in kernel space; all drivers, filesystem, network stack, and
services live in user space. "Hybrid" refers to a narrow, enumerated set of
performance-critical paths that are permitted to execute in kernel space.
As of v0.1.1-draft, exactly one such exception is sanctioned:

1. **IPC fast path** — synchronous rendezvous `send`/`recv` executes inline
   in the syscall dispatcher without invoking the scheduler when the receiver
   is already blocked on `sys_recv` (direct process switch).

Any additional kernel-space promotion requires a written rationale,
a benchmark proving the userspace implementation cannot meet its latency
target, and a security review. Promotions are tracked in
`docs/kernel-space-exceptions.md`.

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
- Notification primitive for async signaling (see *Capability Model* below)

**`elminux-syscall`** — Syscall ABI Definition
- Shared between kernel and userland (no duplication)
- Not POSIX — custom capability-based ABI
- Versioned from day one (`abi_version` in manifest)
- Syscall numbers are stable once published

**`elminux-drivers`** — Driver Framework (shared trait + ABI only)
- Driver trait and message-type definitions only — **no driver implementations**
- Shared between kernel (trait-object registry) and userland (driver implementations)
- Will be re-homed to `abi/elminux-driver-abi` in v0.5.0 to make the
  kernel/userland split structurally obvious
- Actual driver binaries live under `userland/drivers/*` (see below)

**Driver Servers (userspace, `userland/drivers/*`)** — Driver Implementations
- `serial` — UART 16550 (production path; distinct from kernel early-boot serial, see below)
- `keyboard` — PS/2 keyboard
- `vga` / `framebuffer` — basic display
- `ata` / `nvme` — storage (v0.6+)
- `virtio` — QEMU virtio block/net (v0.5)

**Early-boot vs. production serial ownership.** The kernel's
`elminux-hal::uart` drives COM1 directly for panic output, early-init logs,
and unrecoverable-fault reporting. After `elinit` spawns the userspace
`serial` driver server, ordinary logging transitions to the userspace path
via IPC. The kernel retains exclusive access to COM1 only along the panic
path (reclaimed via spinlock + `cli`). Documented in `docs/serial-ownership.md`.

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
    ├── elminux-hal::init()              — IDT, GDT, paging bootstrap
    ├── elminux-mm::init()               — frame allocator, heap
    ├── elminux-mm::teardown_identity()  — drop PVH trampoline identity
    │                                      0–4GB map; kernel now runs
    │                                      exclusively in higher half
    ├── elminux-ipc::init()              — capability table, CDT roots
    ├── elminux-sched::init()            — task queue
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

Extended syscall set (v0.5 ABI additions — see *Capability Model* and *IPC*):
```
sys_notify(cap: Cap)                       — set notification bits (non-blocking)
sys_wait(cap: Cap) -> u64                  — block until notification, return+clear word
sys_poll(cap: Cap) -> u64                  — non-blocking read+clear
sys_cap_grant(target: Pid, cap: Cap, rights: Rights) -> Cap
                                           — delegate narrowed cap to target process
sys_cap_revoke(cap: Cap)                   — cascading revoke via CDT
```

---

## Capability Model

The capability model is the kernel's single authorization mechanism. Every
syscall except `sys_yield` and `sys_exit` requires a capability argument.

### Capability Space Layout

- Per-process capability table (flat array, file-descriptor-like indexing)
- `Cap` is a `u32` index into the per-process table — unforgeable because
  the table is kernel-owned and indices without a backing entry are rejected
- Maximum 65 536 capabilities per process in v1 ABI (bumpable via ABI revision)
- Guarded-page-table layout (seL4-style) deferred to post-v1.0 if scaling demands it

### Derivation and Delegation

- A process holding a capability with the `grant` right may produce a derived
  capability for a child process via `sys_spawn` (transferred at spawn time) or
  via `sys_cap_grant(target, cap, new_rights)` (added in v0.5)
- Derived capabilities have `rights ⊆ parent_rights` (monotonic narrowing)
- The kernel maintains a **Capability Derivation Tree (CDT)** per kernel object:
  each node points to its parent cap. Used for cascading revocation.

### Revocation

- `sys_cap_revoke(cap)` — revokes the target cap and all capabilities
  derived from it (cascading, CDT walk). Synchronous: does not return until
  all derived caps across all processes are invalidated.
- `sys_cap_drop(cap)` — drops the caller's reference only (non-cascading).
- Stale references in other processes return `EBADCAP` on next use.

### Notification (Async Signal) Primitive

Pure synchronous rendezvous IPC cannot express "an event happened" without
blocking a receiver thread. Notifications are a separate primitive, modeled on
seL4 Notifications:

- `sys_notify(cap)` — sets one or more bits in a kernel notification word;
  never blocks sender; coalesces repeated signals
- `sys_wait(cap) -> u64` — blocks until notification word is non-zero,
  returns and clears the word atomically
- `sys_poll(cap) -> u64` — non-blocking read+clear

Notifications are used for IRQ delivery to userspace drivers (below) and
for cross-process signaling where blocking the sender is unacceptable.

### IRQ Delivery to Userspace Drivers

1. At boot, `elinit` requests an IRQ capability from the kernel for a given
   vector (e.g. IRQ1 for PS/2 keyboard), passing a notification capability.
2. On interrupt, the kernel ISR sets the corresponding bit in the notification
   word, acks the APIC, and returns from interrupt — no scheduler invocation,
   no IPC message construction on the hot path.
3. The userspace driver's IRQ thread is blocked on `sys_wait(notify_cap)`;
   when the notification fires it wakes, reads device registers (via its
   port-I/O or MMIO capability), and forwards a structured event message
   to downstream consumers via normal sync IPC.

This matches the seL4 / L4Re pattern and keeps the IRQ-to-userspace path
at ~1 µs in published benchmarks.

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
| IPC round-trip latency | < 500ns | kernel↔driver single message, same-core |
| Notification round-trip | < 200ns | `sys_notify` → `sys_wait` wake, same-core |
| Syscall overhead | < 200ns | `sys_yield()` round-trip |
| Boot to shell (QEMU) | < 2s | power-on to `modsh` prompt |
| Idle memory footprint | < 16MB | post-boot, no user apps |
| Kernel binary size | < 1MB | stripped ELF |

**Scope note**: latency targets apply to single-core execution until SMP
lands (post-v1.0, Backlog B.7). Cross-core numbers are tracked separately
once SMP is in tree.

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
| KPTI CR3-switch cost | Feature-gate per CPU (Meltdown-immune CPUs skip); measure both paths in bench harness |

### IPC Fast Path Design

The single most critical number is IPC round-trip latency.
A naïve implementation will be 2–10µs. Target is < 500ns.

Scope: this target applies to synchronous rendezvous `sys_send`/`sys_recv`
only. Notifications (`sys_notify`/`sys_wait`) are a distinct primitive with
their own target (< 200ns, same-core).

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

## Microarchitectural Security Posture

As of v0.1.1-draft, the following positions are explicit. Enabling/disabling
decisions are made at boot based on CPUID feature detection; each mitigation
has a matching TODO entry under v0.9.0.

| Mitigation | Position (v0.1–v1.0) | Rationale |
|---|---|---|
| KPTI (Meltdown) | Enabled by default on vulnerable CPUs; skip on Meltdown-immune CPUs | Mandatory for untrusted user code; ~100–200 cycles per syscall via CR3 switch |
| Spectre v1 (BCB) | `lfence` barriers at syscall-dispatch branch points; Rust bounds checks retained in hot paths | Rust safety does not extend to speculative side channels |
| Spectre v2 (BTI) | IBRS + IBPB on context switch; retpolines as fallback | Compiler `-Zretpoline` at build time |
| L1TF | PTE inversion for non-present entries | Default-on |
| MDS / TAA | VERW buffer clear on kernel exit | Default-on |
| Retbleed | Untrained return predictor clear on context switch | Default-on |
| SSB | Opt-in per-process via capability; off by default | Performance cost high for most workloads |

Userspace hardening (stack canaries, ASLR, W^X, CFI) is enumerated in
`docs/userspace-hardening.md`.

---

## Multi-Architecture Strategy

v0.1–v1.0 targets x86_64 only. Architectural layering is introduced from
day one to avoid retrofit cost when aarch64 and riscv64 arrive.

```
kernel/elminux-hal/              ← trait definitions only (arch-neutral)
kernel/elminux-hal-x86_64/       ← x86_64 implementation (current)
kernel/elminux-hal-aarch64/      ← planned post-v1.0
kernel/elminux-hal-riscv64/      ← planned post-v1.0
```

The top-level `elminux-hal` crate re-exports the active implementation via
`cfg(target_arch)` feature selection. Kernel code depends only on the
trait crate; per-arch crates provide the implementation.

**Refactor milestone**: split into trait + x86_64 crates at **v0.2.1**
(before more architectural assumptions calcify in shared code).
See TODO §4a.

---

## Build System

**Toolchain:** Rust nightly (`x86_64-unknown-none` target), pinned to a
**dated** nightly (e.g. `nightly-YYYY-MM-DD`) — never bare `nightly`, to
keep builds reproducible and CI stable against upstream churn.

```toml
# rust-toolchain.toml
[toolchain]
channel = "nightly-YYYY-MM-DD"  # bump deliberately, not drift
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

## Build Reproducibility

- `SOURCE_DATE_EPOCH` respected in all build scripts
- Linker version pinned (rust-lld from the pinned nightly toolchain)
- `rust-toolchain.toml` pinned to a dated nightly (above), not `nightly`
- Deterministic symbol mangling (`-Zremap-path-prefix`)
- Kernel ELF reproducible byte-for-byte across independent builds
  (verified in CI by comparing two independent build hashes)
- SLSA v1.0 provenance generated for every release artifact
  (kernel ELF, ISO, `.epkg` packages)
- Provenance signed with the Elminux release key; verification steps in
  `docs/verifying-builds.md`
- Submodules (including `userland/modsh`) pinned by commit SHA, not branch

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
