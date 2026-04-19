/* Elminux Kernel Linker Script */

ENTRY(pvh_start)

SECTIONS {
    /* Kernel starts at 1MB (standard for x86 kernels) */
    . = 0x100000;

    /* PVH ELF note must come first for QEMU to detect it */
    .note.pvh : {
        *(.note.pvh)
    }

    .note : {
        *(.note .note.*)
    }

    /* PVH 32-bit boot trampoline must be within first 4GB */
    .boot.text : {
        *(.boot.text)
    }

    .text : {
        *(.text .text.*)
        *(.ltext .ltext.*)
    }

    .rodata : {
        *(.rodata .rodata.*)
    }

    .data : {
        *(.data .data.*)
        *(.ldata .ldata.*)
    }

    .bss : {
        __bss_start = .;
        *(.bss .bss.*)
        *(.lbss .lbss.*)
        *(COMMON)
        __bss_end = .;
    }

    /* Discard unused sections */
    /DISCARD/ : {
        *(.comment)
        *(.note.gnu.build-id)
    }
}
