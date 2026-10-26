OUTPUT_ARCH(riscv)
ENTRY(_entry)

SECTIONS
{
    . = 0x80000000;

    stext = .;
    .text : {
        *(.text.entry)
        *(.text .text.*)
    }
    . = ALIGN(4096);
    etext = .;

    srodata = .;
    .rodata : { *(.rodata .rodata.*) }
    . = ALIGN(4096);
    erodata = .;

    sdata = .;
    .data : { *(.data .data.*) }
    . = ALIGN(4096);
    edata = .;

    sbss = .;
    .bss : {
        *(.bss.stack)
        *(.bss .bss.* COMMON)
    }
    . = ALIGN(4096);
    ebss = .;
    ekernel = .;

    /DISCARD/ : {
        *(.eh_frame .eh_frame_hdr)
        *(.note .note.*)
    }
}
