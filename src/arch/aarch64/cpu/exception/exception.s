.macro HANDLE_WITH_CONTEXT handler
    sub sp, sp,   #16 * 17
    stp x0, x1,   [sp, #16 * 0]
    stp x2, x3,   [sp, #16 * 1]
    stp x4, x5,   [sp, #16 * 2]
    stp x6, x7,   [sp, #16 * 3]
    stp x8, x9,   [sp, #16 * 4]
    stp x10, x11, [sp, #16 * 5]
    stp x12, x13, [sp, #16 * 6]
    stp x14, x15, [sp, #16 * 7]
    stp x16, x17, [sp, #16 * 8]
    stp x18, x19, [sp, #16 * 9]
    stp x20, x21, [sp, #16 * 10]
    stp x22, x23, [sp, #16 * 11]
    stp x24, x25, [sp, #16 * 12]
    stp x26, x27, [sp, #16 * 13]
    stp x28, x29, [sp, #16 * 14]

    mrs x1, elr_el1
    mrs x2, spsr_el1

    stp lr, x1, [sp, #16 * 15]
    str x2, [sp, #16 * 16]

    mov x0, sp

    bl \handler

    b __ex_restore
.endm

.macro FIQ_SUSPEND
    1:  wfe
    b 1b
.endm

.align 11

__exv_start:
.org 0x000
    HANDLE_WITH_CONTEXT current_el0_sync
.org 0x080
    HANDLE_WITH_CONTEXT current_el0_irq
.org 0x100
    FIQ_SUSPEND
.org 0x180
    HANDLE_WITH_CONTEXT current_el0_serror

.org 0x200
    HANDLE_WITH_CONTEXT current_elx_sync
.org 0x280
    HANDLE_WITH_CONTEXT current_elx_irq
.org 0x300
    FIQ_SUSPEND
.org 0x380
    HANDLE_WITH_CONTEXT current_elx_serror

.org 0x400
    HANDLE_WITH_CONTEXT lower_aarch64_sync
.org 0x480
    HANDLE_WITH_CONTEXT lower_aarch64_irq
.org 0x500
    FIQ_SUSPEND
.org 0x580
    HANDLE_WITH_CONTEXT lower_aarch64_serror

.org 0x600
    HANDLE_WITH_CONTEXT lower_aarch32_sync
.org 0x680
    HANDLE_WITH_CONTEXT lower_aarch32_irq
.org 0x700
    FIQ_SUSPEND
.org 0x780
    HANDLE_WITH_CONTEXT lower_aarch32_serror

.org 0x800

__ex_restore:
    ldr	w19,      [sp, #16 * 16]
    ldp	lr,  x20, [sp, #16 * 15]

    msr	SPSR_EL1, x19
    msr	ELR_EL1,  x20

    ldp	x0,  x1,  [sp, #16 * 0]
    ldp	x2,  x3,  [sp, #16 * 1]
    ldp	x4,  x5,  [sp, #16 * 2]
    ldp	x6,  x7,  [sp, #16 * 3]
    ldp	x8,  x9,  [sp, #16 * 4]
    ldp	x10, x11, [sp, #16 * 5]
    ldp	x12, x13, [sp, #16 * 6]
    ldp	x14, x15, [sp, #16 * 7]
    ldp	x16, x17, [sp, #16 * 8]
    ldp	x18, x19, [sp, #16 * 9]
    ldp	x20, x21, [sp, #16 * 10]
    ldp	x22, x23, [sp, #16 * 11]
    ldp	x24, x25, [sp, #16 * 12]
    ldp	x26, x27, [sp, #16 * 13]
    ldp	x28, x29, [sp, #16 * 14]

    add	sp,  sp,  #16 * 17

    eret

.size __ex_restore, . - __ex_restore
.type __ex_restore, function
