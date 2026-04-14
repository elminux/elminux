# Elminux

> **Local-first. Built to last.**
>
> A new operating system written from scratch in Rust —
> own hybrid kernel, no libc, own package manager.
> Built for constrained hardware, offline-first deployment,
> and digital sovereignty. No Linux. No GNU. No C.

---

## What This Is

Elminux is an experimental, from-scratch operating system targeting:

- **Modern safety** — Rust ownership model eliminates entire CVE classes at compile time
- **No legacy debt** — no POSIX, no libc, no C heritage, no 50-year-old ABI
- **Sovereignty** — verifiable, reproducible, no cloud dependency by design
- **Constrained hardware** — built to run well on minimal resources
- **Offline-first** — full functionality without internet connectivity

## Performance Philosophy

Lightweight and fast are not automatic — they are earned at every layer.

Elminux has structural advantages: no GC, no runtime, no libc bloat, no
legacy ABI translation, purpose-built allocator, and a clean syscall ABI.
These are necessary but not sufficient.

The IPC-based driver architecture trades some latency for isolation. Fast
requires deliberate design. Every layer has an explicit performance target:

| Metric | Target |
|---|---|
| IPC round-trip latency | < 500ns (kernel↔driver) |
| Syscall overhead | < 200ns |
| Boot to shell (QEMU) | < 2s |
| Idle memory footprint | < 16MB |
| Kernel binary size | < 1MB |

These targets are benchmarked explicitly at v0.3.0 and tracked on every
release. Missing a target is a bug, not a footnote.

This is not a Linux distribution. This is not a fork of anything.

---

## Architecture Summary

| Component | Approach |
|---|---|
| Kernel | Hybrid Rust kernel (trusted core in kernel space, drivers in user space) |
| libc | None — own `elminux-std` Rust-native standard library |
| Syscall ABI | Custom capability-based ABI, defined in `elminux-syscall` |
| Package manager | `epkg` — Rust-native, TOML manifests, cryptographically signed |
| Init system | `elinit` — Rust, capability-aware |
| Shell | `modsh` — Rust, POSIX-compatible + structured I/O pipelines (Apache-2.0 core) |
| Boot | Limine bootloader → Elminux kernel entry |
| Primary target | x86_64 (QEMU first, then bare metal) |

---

## Repository Structure

```
elminux/
├── Cargo.toml              # Workspace root
├── kernel/
│   ├── elminux-kernel/     # Hybrid kernel core
│   ├── elminux-hal/        # Hardware abstraction layer (x86_64)
│   ├── elminux-mm/         # Memory manager (physical + virtual)
│   ├── elminux-sched/      # Scheduler
│   ├── elminux-ipc/        # IPC primitives (message passing)
│   ├── elminux-drivers/    # Driver framework + initial drivers
│   └── elminux-syscall/    # Syscall ABI definition (shared kernel/user)
├── userland/
│   ├── elminux-std/        # Rust-native standard library (no libc)
│   ├── epkg/               # Package manager
│   ├── elinit/             # Init system
│   └── modsh/              # Shell (modsh-core + modsh-interactive, Apache-2.0)
└── tools/
    └── build-tools/        # ISO builder, QEMU runner, debug tooling
```

---

## Current Status

`v0.1.0-alpha` — Scaffold / Pre-development

**Nothing boots yet.** This is the governance and architecture foundation.
First milestone: boot to kernel entry in QEMU.

---

## License

GPL-3.0 — see [LICENSE](./LICENSE)

---

## Community

- Website: https://elminux.org
- GitHub: https://github.com/elminux
- Matrix: TBD
- Mailing list: TBD

---

## Warning

This is an experimental research-grade OS project. It has no users.
It is not suitable for any production purpose at this stage.
Breaking changes at every layer are expected and intentional.
