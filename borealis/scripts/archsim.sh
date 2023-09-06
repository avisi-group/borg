#!/usr/bin/env bash
set -e

GENSIM_PATH="/home/fm208/Sync/gensim"

# export genc from bincode
#cargo r -- --log info sail2genc data/arm-v8.5-a.bincode.lz4 target/genc/

# run gensim
cd target/genc
$GENSIM_PATH/build/dist/bin/gensim --verbose -a main.genc -t output -s module,arch,decode,disasm,ee_interp,ee_blockjit,jumpinfo,function,makefile -o decode.GenerateDotGraph=1,makefile.libtrace_path=$GENSIM_PATH/support/libtrace/inc,makefile.archsim_path=$GENSIM_PATH/archsim/inc,makefile.Optimise=3,makefile.Debug=1

cd output
mkdir -p modules
make -j$(nproc) && cp arm64.dll modules/
cp ../../../data/mcf/mcf_r_base.aarch64-static-64 $GENSIM_PATH/build/dist/bin/

cd $GENSIM_PATH/build/dist/bin/

rm trace0 || true
./archsim -m aarch64-user -l contiguous -s arm64 --modules /home/fm208/Sync/borealis/target/genc/output/modules -e ./mcf_r_base.aarch64-static-64 -d -t -U trace --mode Interpreter
./TraceCat trace0
