VERSION 0.7

base-image:
    FROM rust:alpine
    WORKDIR /workdir

    ENV RUSTFLAGS="-D warnings"

    ARG TARGETARCH

    IF [ "$TARGETARCH" = "amd64" ]
        ENV RUST_TARGET=x86_64-unknown-linux-musl
    ELSE IF [ "$TARGETARCH" = "arm64" ]
        ENV RUST_TARGET=aarch64-unknown-linux-musl
    ELSE
        ENV RUST_TARGET=invalid
    END

    RUN rustup component add rustfmt
    RUN apk update && apk add opam alpine-sdk zlib-dev xz m4 z3 gmp-dev clang mold

    # setup OCaml
    RUN opam init --disable-sandboxing --bare -y
    RUN opam switch create 4.14.1+options --packages ocaml-variants.4.14.1+options,ocaml-option-static,ocaml-option-musl,ocaml-option-flambda

    # install sail
    RUN eval `opam env` && opam install --assume-depexts -y sail=0.15 gmp

    RUN cargo install cargo-chef --locked

    SAVE IMAGE --cache-hint --push ghcr.io/avisi-group/borealis/baseimage

chef-prepare:
    FROM +base-image
    COPY . .
    RUN cargo chef prepare --recipe-path recipe.json
    SAVE ARTIFACT recipe.json recipe.json

chef-cook:
    FROM +base-image
    COPY (+chef-prepare/recipe.json) .
    RUN eval `opam env` && cargo chef cook --recipe-path recipe.json --target $RUST_TARGET

build:
    FROM +chef-cook

    # copy full source
    COPY --dir sail common borealis libarch-sys .

    # build borealis
    RUN eval `opam env` && mold -run cargo build --target $RUST_TARGET

    SAVE ARTIFACT target/$RUST_TARGET/debug/borealis borealis

borealis-docs:
    FROM +build
    RUN eval `opam env` && cargo doc --target $RUST_TARGET
    RUN echo '<!DOCTYPE html><html><head><meta http-equiv="refresh" content="0; URL=/borealis/borealis" /></head></html>' > target/$RUST_TARGET/doc/index.html
    SAVE ARTIFACT target/$RUST_TARGET/doc docs

docs:
    FROM scratch
    COPY (+borealis-docs/docs) docs
#    COPY (+libarch-docs/docs) docs/libarch_sys
    CMD [""]
    SAVE IMAGE --push ghcr.io/avisi-group/borealis/docs

docker:
    BUILD +build
    FROM alpine

    RUN apk update && apk add z3

    COPY (+build/borealis) borealis
    ENTRYPOINT ["./borealis"]
    SAVE IMAGE --push ghcr.io/avisi-group/borealis

test:
    BUILD +unit-test
    BUILD --platform=linux/amd64 +e2e-test-archsim

test-chef-cook:
    FROM +base-image
    COPY (+chef-prepare/recipe.json) .
    RUN eval `opam env` && cargo chef cook --recipe-path recipe.json --target $RUST_TARGET --tests

unit-test:
    BUILD +build
    FROM +test-chef-cook

    # copy full source
    COPY . .

    # check formatting
    RUN cargo fmt --all -- --check

    # build and run tests
    RUN eval `opam env` && cargo test --target $RUST_TARGET --no-fail-fast

e2e-test-borealis-genc:
    # must use alpine instead of +docker due to https://github.com/earthly/earthly/issues/2618. reading this comment on 2023-08-23 i do not understand it.
    FROM +docker
    COPY data/arm-v8.5-a.bincode.lz4 arm-v8.5-a.bincode.lz4
    RUN ./borealis sail2genc arm-v8.5-a.bincode.lz4 genc
    SAVE ARTIFACT genc genc

e2e-test-gensim:
    FROM ghcr.io/fmckeogh/gensim:latest
    COPY (+e2e-test-borealis-genc/genc) .
    RUN ./dist/bin/gensim --verbose -a main.genc -t output -s module,arch,decode,disasm,ee_interp,ee_blockjit,jumpinfo,function,makefile -o decode.GenerateDotGraph=1,makefile.libtrace_path=/gensim/support/libtrace/inc,makefile.archsim_path=/gensim/archsim/inc,makefile.Optimise=3,makefile.Debug=1
    RUN cd output && make -j$(nproc)
    SAVE ARTIFACT output/arm64.dll arm64.dll

e2e-test-archsim:
    FROM ghcr.io/fmckeogh/gensim:latest

    RUN apt-get install -yy gcc-aarch64-linux-gnu
    COPY data/fib.S .
    COPY data/fib.trace .
    RUN aarch64-linux-gnu-gcc -o fib -nostdlib -static fib.S

    RUN mkdir modules
    COPY (+e2e-test-gensim/arm64.dll) modules

    RUN ./dist/bin/archsim -m aarch64-user -l contiguous -s arm64 --modules ./modules -e ./fib -t -U trace.out.fib --mode Interpreter
    RUN bash -c 'diff -u fib.trace <(./dist/bin/TraceCat trace.out.fib0)'
