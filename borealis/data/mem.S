.globl _start

_start:
    mov x0, 0xAAFF

    mov x1, 0x0
    add x1, x1, 0x401000
    str x0, [x1]
    ldr x0, [x1]
