/* Elminux Kernel Linker Script */
/* Higher-half kernel at 0xffffffff80000000 */

ENTRY(_start)

PHDRS
{
    text    PT_LOAD FLAGS(5);  /* RX */
    rodata  PT_LOAD FLAGS(4);  /* R  */
    data    PT_LOAD FLAGS(6);  /* RW */
}

SECTIONS
{
    /* Higher half base address */
    . = 0xffffffff80000000;

    .text : {
        *(.text.start)
        *(.text .text.*)
    } :text

    /* Align to page boundary */
    . = ALIGN(0x1000);

    .rodata : {
        *(.rodata .rodata.*)
    } :rodata

    . = ALIGN(0x1000);

    .data : {
        *(.data .data.*)
    } :data

    .bss : {
        __bss_start = .;
        *(.bss .bss.*)
        *(COMMON)
        __bss_end = .;
    } :data

    /* Discard unwinding info - kernel doesn't use unwinding */
    /DISCARD/ : {
        *(.eh_frame)
        *(.eh_frame_hdr)
    }
}
