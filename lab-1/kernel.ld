OUTPUT_ARCH(riscv)
ENTRY(_entry)

SECTIONS
{
    . = 0x80000000;

    .text : {
        *(.text.entry)
        *(.text .text.*)
    }

    . = ALIGN(4096);
    .rodata : { *(.rodata .rodata.*) }

    . = ALIGN(4096);
    .data : { *(.data .data.*) }

    . = ALIGN(16);
    .bss : {
        *(.bss.stack)
        *(.bss .bss.* COMMON)
    }

    /DISCARD/ : {
        *(.eh_frame .eh_frame_hdr)
        *(.note .note.*)
    }
}
