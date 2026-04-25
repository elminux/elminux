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

    // Set up a boot stack
    lea (pvh_boot_stack_top), %rsp

    // Zero BSS (Rust requires it)
    lea (__bss_start), %rdi
    lea (__bss_end), %rcx
    sub %rdi, %rcx
    xor %al, %al
    rep stosb

    // Jump to 64-bit Rust kernel entry
    // Pass hvm_start_info in %rdi (first SysV arg) — zero-extend from %ebp
    movl %ebp, %edi         // zero-extend hvm_start_info to 64-bit in %rdi
    xorl %ebp, %ebp         // clear %rbp to mark outermost stack frame
    call _start

    // Should never return
1:  hlt
    jmp 1b

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

// ─── Minimal identity-mapped page tables for first 4GB ──────────────────────
// PML4[0] → PDPT
// PDPT[0..3] → 4 × 1GB huge pages covering 0–4GB identity mapped
.global pvh_pml4
.global pvh_pdpt
.align 4096
pvh_pml4:
    .quad pvh_pdpt + 3      // present + writable
    .fill 511, 8, 0

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
