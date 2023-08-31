.globl _start

_start:
    mov x0, #0  // x
    mov x1, #1  // y
    mov x2, #0  // z
    mov x3, #0  // i
    mov x4, #10  // num

loop:
    cmp x3, x4
    bge done

    add x2, x0, x1  // z = x + y
    mov x0, x1      // x = y
    mov x1, x2      // y = z

    add x3, x3, #1
    b loop

done:
    mov x0, x2

    mov x1, 0x0
    add x1, x1, 0x401000
    ldr x0, [x1]
    add x0, x0, #3141
    str x0, [x1]

    mov x0, #0
    mov w8, #93
    svc 0
