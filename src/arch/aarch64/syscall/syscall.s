// Linux convention: x0-x7 store args and x8 stores syscall number

.global _exit
_exit:
    mov w8, #0
    svc #0
    ret

.global _write
_write:
    mov w8, #1
    svc #0
    ret
