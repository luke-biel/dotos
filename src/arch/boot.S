.macro ADR_REL register, symbol
    adrp   \register, \symbol
    add    \register, \register, #:lo12:\symbol
.endm

.equ _core_id_mask, 0b11

.section .text._start

_start:
    mrs     x1, MPIDR_EL1
    and     x1, x1, _core_id_mask
    ldr     x2, BOOT_CORE_ID
    cmp     x1, x2
    b.ne    parking_loop

    ADR_REL x0, __bss_start
    ADR_REL x1, __bss_end_exclusive

bss_init_loop:
    cmp     x0, x1
    b.eq    prepare_rust
    stp     xzr, xzr, [x0], #16
    b       bss_init_loop

prepare_rust:
    ADR_REL x0, __boot_core_stack_end_exclusive
    mov     sp, x0

    b       _start_rust

parking_loop:
    wfe
    b       parking_loop

.size       _start, . - _start
.type       _start, function
.global     _start
