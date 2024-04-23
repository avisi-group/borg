set trace-commands on
tui enable
layout split

add-symbol-file /home/fm208/.cargo/target/x86_64-unknown-none/debug/brig 0xffff800000000000+0xf4070

target remote :1234
