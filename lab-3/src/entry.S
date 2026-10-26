    .section .text.entry
    .globl _entry
_entry:
    csrr a0, mhartid
    la sp, boot_stack
    li t0, 16384
    addi t1, a0, 1
    mul t0, t0, t1
    add sp, sp, t0
    call start

1:
    wfi
    j 1b

    .section .bss.stack
    .align 14
    .globl boot_stack
boot_stack:
    .space 16384 * 2
