.section ".text._start"

.global _start

_start:
    ldr     r1, =_start
    mov     sp, r1
    bl      __start_os
