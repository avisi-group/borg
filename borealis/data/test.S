.globl _start

_start:
    add x0, x1, #0x20
    add x0, x0, #0x30
    subs x0, x0, #0x10
    mov x2, x0
