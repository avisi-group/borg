#!/bin/sh
set -e
echo ".word 0x$1" > /tmp/disasm.S
aarch64-linux-gnu-gcc -c -o /tmp/disasm.o /tmp/disasm.S
aarch64-linux-gnu-strip /tmp/disasm.o
aarch64-linux-gnu-objdump -d /tmp/disasm.o
