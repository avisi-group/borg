#!/usr/bin/env bash
set -e

# export genc from bincode
cargo r -- --log info sail2genc data/arm-v8.5-a.bincode.lz4 target/genc/

# run gensim
cd target/genc
/t1/Sync/gensim/build/dist/bin/gensim --verbose -a main.genc -t output -s module,arch,decode,disasm,ee_interp,ee_blockjit,jumpinfo,function,makefile -o decode.GenerateDotGraph=1,makefile.libtrace_path=/t1/Sync/gensim/support/libtrace/inc,makefile.archsim_path=/t1/Sync/gensim/archsim/inc,makefile.Optimise=3,makefile.Debug=1

cd output
mkdir -p modules
make -j$(nproc) && cp arm64.dll modules/

cd /t1/Sync/gensim/build/dist/bin/
./archsim -m aarch64-user -l contiguous -s arm64 --modules /t1/Sync/borealis/target/genc/output/modules -e ./fib -t -U trace.out --mode Interpreter
./TraceCat trace.out0
