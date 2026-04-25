/* Elminux Kernel Linker Script
 *
 * Dual-address layout:
 *   .boot.text                     — runs in 32-bit protected mode, must
 *                                    be reachable at its physical load
 *                                    address (identity == VMA == LMA).
 *   .text/.rodata/.data/.bss       — linked at higher-half virtual base
 *                                    KERNEL_VBASE + KERNEL_PBASE; physically
 *                                    loaded at KERNEL_PBASE via AT().
 *
 * The PVH trampoline installs a higher-half mapping (PML4[256] aliasing
 * the same PDPT as PML4[0]) before the long-mode far-jump into Rust, so
 * the higher-half VAs resolve to the correct physical pages.
 */

KERNEL_VBASE = 0xFFFF800000000000;
KERNEL_PBASE = 0x100000;

ENTRY(pvh_start)

SECTIONS {
    . = KERNEL_PBASE;

    /* PVH ELF note must come first for QEMU to detect it */
    .note.pvh : {
        *(.note.pvh)
    }

    .note : {
        *(.note .note.*)
    }

    /* PVH 32-bit boot trampoline — VMA == LMA == low phys.  Must stay
     * within the first 4 GiB and be reachable without higher-half. */
    .boot.text : {
        *(.boot.text)
    }

    /* Switch to higher-half virtual addresses for the rest of the
     * kernel.  Each section's LMA is the physical address; VMA is
     * KERNEL_VBASE + LMA. */
    . = KERNEL_VBASE + .;

    .text : AT(ADDR(.text) - KERNEL_VBASE) {
        *(.text .text.*)
        *(.ltext .ltext.*)
    }

    .rodata : AT(ADDR(.rodata) - KERNEL_VBASE) {
        *(.rodata .rodata.*)
    }

    .data : AT(ADDR(.data) - KERNEL_VBASE) {
        *(.data .data.*)
        *(.ldata .ldata.*)
    }

    .bss : AT(ADDR(.bss) - KERNEL_VBASE) {
        __bss_start = .;
        *(.bss .bss.*)
        *(.lbss .lbss.*)
        *(COMMON)
        __bss_end = .;
    }

    __kernel_phys_start = KERNEL_PBASE;
    __kernel_phys_end   = ADDR(.bss) - KERNEL_VBASE + SIZEOF(.bss);

    /* Discard unused sections */
    /DISCARD/ : {
        *(.comment)
        *(.note.gnu.build-id)
    }
}
