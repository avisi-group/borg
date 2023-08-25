.globl _start

_start:
    mov x0, #0  // x
    mov x1, #1  // y
    mov x2, #0  // z
    mov x3, #0  // i
    mov x4, #100  // num

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
    udf #0
