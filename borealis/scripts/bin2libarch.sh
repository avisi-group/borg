#!/usr/bin/env bash
 set -e

 ### Helper script for running the borealis toolchain from Sail AST parsing onwards (parsing the ARM v8.5 spec is slow).

 # export genc from bincode
 cargo r -- --log info sail2genc data/arm-v8.5-a.bincode.lz4 target/genc/

 # execute gensim on genc
 cd target/genc
#  docker pull ghcr.io/fmckeogh/gensim:latest
#  docker run --rm -t -v $(pwd):/workdir ghcr.io/fmckeogh/gensim:latest --verbose -a main.genc -t output -s captive_decoder,captive_cpu,captive_jitv2,captive_disasm -o captive_decoder.GenerateDotGraph=1,captive_decoder.OptimisationEnabled=1,captive_decoder.OptimisationMinPrefixLength=8,captive_decoder.OptimisationMinPrefixMembers=4,captive_decoder.InlineHints=1
#  sudo chown $USER -R output/*
#  sudo chmod 777 output/*
 ../../../gensim/build/dist/bin/gensim --verbose -a main.genc -t output -s captive_decoder,captive_cpu,captive_jitv2,captive_disasm -o captive_decoder.GenerateDotGraph=1,captive_decoder.OptimisationEnabled=1,captive_decoder.OptimisationMinPrefixLength=8,captive_decoder.OptimisationMinPrefixMembers=4,captive_decoder.InlineHints=1
 cd ../..

 # copy libarch source
 cp target/genc/output/arm64-decode.cpp libarch-sys/include/
 cp target/genc/output/arm64-decode.h libarch-sys/include/
 cp target/genc/output/arm64-disasm.cpp libarch-sys/include/
 cp target/genc/output/arm64-disasm.h libarch-sys/include/

 # run libarch on sample
 cd libarch-sys
 cargo t
 cd ..
