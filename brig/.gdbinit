set trace-commands on
tui enable
layout split

# offset to kernel load bias + start of .text section
add-symbol-file /home/fm208/.cargo/target/x86_64-unknown-brig/debug/kernel 0xffff800000000000+0x12a9dc

target remote :1234
