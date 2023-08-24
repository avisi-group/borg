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

    SAVE IMAGE --cache-hint --push ghcr.io/avisi-group/borealis/baseimage

prebuild:
    BUILD +base-image
    FROM +base-image
    WORKDIR /tmp/build

    # build and document rust dependencies by creating empty crates
    RUN cargo init --lib borealis && \
        cargo init --lib sail && \
        cargo init --lib common && \
        cargo init --lib libarch-sys

    COPY Cargo.lock .
    COPY Cargo.toml .
    COPY borealis/Cargo.toml borealis/
    COPY sail/Cargo.toml sail/
    COPY common/Cargo.toml common/
    COPY libarch-sys/Cargo.toml libarch-sys/

    RUN eval `opam env` && \
        cargo build --target $RUST_TARGET --release && \
        cargo test --target $RUST_TARGET --no-run

    SAVE IMAGE --cache-hint --push ghcr.io/avisi-group/borealis/prebuild

build:
    BUILD +prebuild
    FROM +prebuild

    # copy full source
    COPY --keep-ts --dir borealis common sail data .
    RUN touch borealis/src/lib.rs sail/src/lib.rs common/src/lib.rs

    # build borealis
    RUN eval `opam env` && mold -run cargo build --target $RUST_TARGET --release

    SAVE ARTIFACT target/$RUST_TARGET/release/borealis borealis
    # we save the workspace as a copy of all the code with no build artefacts as the e2e-test uses a different target none of that is reusable
    SAVE ARTIFACT . workspace

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

unit-test:
    BUILD +build
    FROM +build

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
    RUN cd output && make -j8
    SAVE ARTIFACT output/arm64.dll arm64.dll

e2e-test-archsim:
    FROM ghcr.io/fmckeogh/gensim:latest

    RUN apt-get install -yy gcc-aarch64-linux-gnu
    COPY data/fib.S .
    RUN aarch64-linux-gnu-gcc -o fib -nostdlib -static fib.S

    RUN mkdir modules
    COPY (+e2e-test-gensim/arm64.dll) modules

    RUN ./dist/bin/archsim -m aarch64-user -l contiguous -s arm64 --modules ./modules -e ./fib -t -U trace.out --mode Interpreter
    RUN ./dist/bin/TraceCat trace.out0
