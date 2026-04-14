---
name: Kernel Panic
about: Report a kernel crash or unrecoverable error
title: '[PANIC] '
labels: bug, panic, critical
assignees: ''
---

## Panic Information

**Please fill out all sections. Kernel panics are high priority issues.**

### Panic Message

```
Paste the exact panic message here
```

### CPU State (if available)

```
RIP: 0x...  RSP: 0x...  RBP: 0x...
RAX: 0x...  RBX: 0x...  RCX: 0x...
RDX: 0x...  RSI: 0x...  RDI: 0x...
R8:  0x...  R9:  0x...  R10: 0x...
R11: 0x...  R12: 0x...  R13: 0x...
R14: 0x...  R15: 0x...
```

### Exception Information

- **Exception vector**: (e.g., 0x0E - Page Fault, 0x08 - Double Fault)
- **Error code**: (if applicable)
- **CR2**: (page fault address, if page fault)

## Environment

- **Elminux version**: (e.g., v0.2.0, commit hash)
- **Target platform**: (QEMU x86_64, specific hardware)
- **QEMU version**: (`qemu-system-x86_64 --version`)
- **Build configuration**: (debug/release, any special flags)

## Reproduction Steps

1. Boot with '...'
2. Execute '...'
3. Trigger action '...'
4. Panic occurs

### Reproducibility

- [ ] Always reproducible
- [ ] Intermittent (frequency: ___)
- [ ] One-time occurrence

## Stack Trace

If you have enabled frame pointers or captured a stack trace:

```
#0: 0x... in function_name
#1: 0x... in function_name
#2: 0x... in function_name
```

## Memory Map (if available)

```
Paste relevant memory map information from boot
```

## Additional Context

- Recent changes to the codebase
- Hardware specifications (if bare metal)
- Any custom patches or modifications
- Related issues or PRs

## Crash Dump

If you have a crash dump file, please attach it or provide instructions for reproducing.
