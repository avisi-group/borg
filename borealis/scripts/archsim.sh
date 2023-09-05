#!/usr/bin/env bash
set -e

# export genc from bincode
#cargo r -- --log info sail2genc data/arm-v8.5-a.bincode.lz4 target/genc/

# run gensim
cd target/genc
/home/fm208/Sync/gensim/build/dist/bin/gensim --verbose -a main.genc -t output -s module,arch,decode,disasm,ee_interp,ee_blockjit,jumpinfo,function,makefile -o decode.GenerateDotGraph=1,makefile.libtrace_path=/home/fm208/Sync/gensim/support/libtrace/inc,makefile.archsim_path=/home/fm208/Sync/gensim/archsim/inc,makefile.Optimise=3,makefile.Debug=1

cd output
mkdir -p modules
make -j$(nproc) && cp arm64.dll modules/

cd /home/fm208/Sync/gensim/build/dist/bin/

cp /home/fm208/Sync/borealis/data/mem.S .
aarch64-linux-gnu-gcc -o mem -nostdlib -static mem.S

./archsim -m aarch64-user -l contiguous -s arm64 --modules /home/fm208/Sync/borealis/target/genc/output/modules -e ./mem -d --mode Interpreter
