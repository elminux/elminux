# Elminux Development Setup

This guide explains how to set up your development environment for Elminux kernel development.

## Prerequisites

- **OS**: Linux (x86_64) recommended. macOS and Windows (WSL2) may work but are not officially supported.
- **Memory**: 8GB RAM minimum, 16GB recommended
- **Disk**: 20GB free space

## Required Tools

### 1. Rust Toolchain

Install Rust using [rustup](https://rustup.rs/):

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

The Elminux workspace uses a pinned nightly toolchain. The correct version will be installed automatically when you run `cargo` commands.

### 2. QEMU

QEMU is used for testing the kernel:

```bash
# Ubuntu/Debian
sudo apt-get install qemu-system-x86

# Fedora
sudo dnf install qemu-system-x86

# Arch Linux
sudo pacman -S qemu-full
```

### 3. Build Dependencies

```bash
# Ubuntu/Debian
sudo apt-get install build-essential lld llvm xorriso git

# Fedora
sudo dnf install gcc lld llvm xorriso git

# Arch Linux
sudo pacman -S base-devel lld llvm-libs libisoburn git
```

### 4. cargo-binstall (Recommended)

[cargo-binstall](https://github.com/cargo-bins/cargo-binstall) provides fast, pre-built binary installation for Rust tools:

```bash
cargo install cargo-binstall
```

Use it to install other tools quickly:

```bash
cargo binstall cargo-deny cargo-cyclonedx cargo-fuzz
```

### 5. cargo-skill (Required)

[cargo-skill](https://crates.io/crates/cargo-skill) provides AI-assisted development for this codebase:

```bash
cargo install cargo-skill
cargo skill init   # One-time setup per repository
cargo skill write  # Update context after major changes
```

## Building Elminux

### First Build

```bash
# Clone the repository
git clone https://github.com/elminux/elminux.git
cd elminux

# The correct nightly toolchain will be installed automatically
cargo build --release -p elminux-kernel
```

### Common Build Commands

```bash
# Build debug kernel
cargo build -p elminux-kernel

# Build release kernel (optimized)
cargo build --release -p elminux-kernel

# Run in QEMU
make qemu

# Create bootable ISO
make iso

# Run linting
make clippy

# Run tests
make test

# Generate documentation
make doc
```

## IDE Setup

### Windsurf / VS Code

The repository includes Windsurf configuration. Recommended extensions:

- rust-analyzer
- CodeLLDB (for debugging)
- Even Better TOML

### rust-analyzer Configuration

Add to your editor settings:

```json
{
  "rust-analyzer.cargo.target": "x86_64-unknown-none",
  "rust-analyzer.cargo.features": "all",
  "rust-analyzer.checkOnSave.command": "clippy"
}
```

## Project Structure

```
elminux/
├── kernel/           # Kernel-space code
│   ├── elminux-kernel    # Kernel entry point
│   ├── elminux-hal       # Hardware abstraction (GDT, IDT, APIC, etc.)
│   ├── elminux-mm        # Memory management
│   ├── elminux-sched     # Task scheduler
│   ├── elminux-ipc       # IPC system
│   ├── elminux-drivers   # Driver framework
│   └── elminux-syscall   # Syscall ABI
├── userland/         # User-space code
│   ├── elminux-std       # Standard library
│   ├── epkg              # Package manager
│   ├── elinit            # Init system
│   └── modsh             # Shell
└── tools/            # Build tools
```

## Troubleshooting

### "compiler_builtins" build errors

If you see SSE-related errors, ensure `.cargo/config.toml` has `+soft-float` (not `-soft-float`).

### QEMU not found

Ensure QEMU is in your PATH:

```bash
which qemu-system-x86_64
```

### Missing rust-src component

The `rust-toolchain.toml` should auto-install this. If not:

```bash
rustup component add rust-src
```

## Next Steps

- Read [ARCHITECTURE.md](../ARCHITECTURE.md) for system design
- Check [TODO.md](../TODO.md) for current development priorities
- See [CONTRIBUTORS.md](../CONTRIBUTORS.md) for contribution guidelines
