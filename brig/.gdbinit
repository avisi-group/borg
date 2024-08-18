set trace-commands on
tui enable
layout split

# offset to kernel load bias + start of .text section
add-symbol-file /t1/cargo_global_target_dir/x86_64-unknown-none/debug/brig 0xffff800000000000+0x17750

target remote :1234
