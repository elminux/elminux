/* Elminux Kernel Linker Script */

ENTRY(_start)

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

    .text : {
        *(.text .text.*)
    }

    .rodata : {
        *(.rodata .rodata.*)
    }

    .data : {
        *(.data .data.*)
    }

    .bss : {
        *(.bss .bss.*)
        *(COMMON)
    }

    /* Discard unused sections */
    /DISCARD/ : {
        *(.comment)
        *(.note.gnu.build-id)
    }
}
