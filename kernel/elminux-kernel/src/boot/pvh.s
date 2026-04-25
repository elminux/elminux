// PVH entry trampoline for QEMU direct kernel boot
//
// QEMU enters here in 32-bit protected mode with paging disabled.
// We must set up a minimal GDT, enable PAE, enable long mode via EFER,
// set up identity-mapped page tables, then enable paging and jump to
// the 64-bit _start.
//
// Entry state (from Xen PVH spec):
//   CS: 32-bit flat code segment
//   DS/ES/SS: 32-bit flat data segment
//   CR0: PE=1, PG=0
//   EBX: physical address of hvm_start_info struct

.code32
.section .boot.text, "ax"
.global pvh_start

pvh_start:
    // Disable interrupts
    cli

    // Save hvm_start_info pointer to %ebp (not clobbered by long-mode switch)
    movl %ebx, %ebp

    // Load our minimal 64-bit GDT
    lgdtl (pvh_gdt_descriptor)

    // Set up PAE (CR4.PAE = 1) required for long mode
    movl %cr4, %eax
    orl $0x20, %eax
    movl %eax, %cr4

    // Point CR3 to our minimal PML4 page tables
    movl $pvh_pml4, %eax
    movl %eax, %cr3

    // Enable long mode via EFER.LME
    movl $0xC0000080, %ecx   // EFER MSR
    rdmsr
    orl $0x100, %eax         // Set LME bit
    wrmsr

    // Enable paging + protected mode (CR0.PG = CR0.PE = 1)
    movl %cr0, %eax
    orl $0x80000001, %eax
    movl %eax, %cr0

    // Far jump to 64-bit code segment to enter long mode
    ljmpl $0x08, $pvh_long_mode

.code64
pvh_long_mode:
    // Set up data segments
    mov $0x10, %ax
    mov %ax, %ds
    mov %ax, %es
    mov %ax, %ss
    xor %ax, %ax
    mov %ax, %fs
    mov %ax, %gs

    // Set up a boot stack (still using identity-mapped low VA — pvh_boot_stack
    // lives in .boot.text which is linked at low phys).
    lea (pvh_boot_stack_top), %rsp

    // Far-jump to the higher-half entry so all subsequent fetches use
    // higher-half virtual addresses.  PML4[256] aliases 0–4GB to the
    // same PDPT (see static page tables below).
    movabs $higher_half_entry, %rax
    jmp *%rax

// ─── .text section (linked at higher-half VA) ──────────────────────────────
.section .text, "ax"
higher_half_entry:
    // Zero BSS now — __bss_start/__bss_end are higher-half symbols only
    // reachable in 64-bit mode after the higher-half jump.
    movabs $__bss_start, %rdi
    movabs $__bss_end, %rcx
    sub %rdi, %rcx
    xor %al, %al
    rep stosb

    // Pass hvm_start_info in %rdi (first SysV arg) — zero-extend from %ebp.
    movl %ebp, %edi
    xorl %ebp, %ebp         // clear %rbp to mark outermost stack frame
    call _start

    // Should never return
1:  hlt
    jmp 1b

// ─── back to .boot.text for static GDT / page tables / boot stack ──────────
.section .boot.text, "ax"

// ─── Minimal GDT for 64-bit boot ────────────────────────────────────────────
.align 8
pvh_gdt:
    .quad 0x0000000000000000  // 0x00: null
    .quad 0x00AF9A000000FFFF  // 0x08: 64-bit kernel code (G,L,P,DPL0)
    .quad 0x00CF92000000FFFF  // 0x10: 32/64-bit kernel data (G,B,P,DPL0)
pvh_gdt_end:

pvh_gdt_descriptor:
    .word pvh_gdt_end - pvh_gdt - 1
    .long pvh_gdt

// ─── Minimal page tables for first 4GB ──────────────────────────────────────
// PML4[0]   → PDPT  (identity map 0–4GB; required by 32-bit trampoline
//                    and by early Rust code that touches phys addresses)
// PML4[256] → PDPT  (same PDPT, aliased into the higher half so that
//                    KERNEL_BASE + phys resolves to phys after the
//                    higher-half far-jump)
// PDPT[0..3] → 4 × 1GB huge pages covering 0–4GB
//
// Index 256 corresponds to virtual address 0xFFFF_8000_0000_0000 — the
// canonical higher-half start used as KERNEL_BASE.
.global pvh_pml4
.global pvh_pdpt
.align 4096
pvh_pml4:
    .quad pvh_pdpt + 3      // [0]   identity (present + writable)
    .fill 255, 8, 0
    .quad pvh_pdpt + 3      // [256] higher-half alias (same PDPT)
    .fill 255, 8, 0

.align 4096
pvh_pdpt:
    .quad 0x000000 + 0x83   // 0GB–1GB: present + writable + huge (PS)
    .quad 0x40000000 + 0x83 // 1GB–2GB
    .quad 0x80000000 + 0x83 // 2GB–3GB
    .quad 0xC0000000 + 0x83 // 3GB–4GB
    .fill 508, 8, 0

// ─── Boot stack (8KB) ────────────────────────────────────────────────────────
.align 16
pvh_boot_stack:
    .fill 8192, 1, 0
pvh_boot_stack_top:
