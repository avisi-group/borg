VERSION 0.8

base-image:
    FROM rust:alpine
    WORKDIR /workdir

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
    RUN eval `opam env` && opam install --assume-depexts -y sail=0.17.1 gmp

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

build-sailrs:
    FROM +chef-cook

    # copy full source
    COPY --dir sailrs common .

    # build borealis
    RUN eval `opam env` && cargo build --target $RUST_TARGET

    SAVE ARTIFACT target/$RUST_TARGET/debug/sailrs sailrs

build-borealis:
    FROM +chef-cook

    # copy full source
    COPY --dir sailrs common borealis .

    # build borealis
    RUN eval `opam env` && cargo build --target $RUST_TARGET

    SAVE ARTIFACT target/$RUST_TARGET/debug/borealis borealis

test:
    BUILD +unit-test
    BUILD +e2e-test-brig

test-chef-cook:
    FROM +base-image
    COPY (+chef-prepare/recipe.json) .
    RUN eval `opam env` && cargo chef cook --recipe-path recipe.json --target $RUST_TARGET --tests

unit-test:
    FROM +test-chef-cook

    # copy full source
    COPY . .

    # check formatting
    RUN cargo fmt --all -- --check

    # build and run tests
    RUN eval `opam env` && cargo test --target $RUST_TARGET --no-fail-fast

### BRIG ###
e2e-test-sailrs:
    FROM +base-image

    ENV SAIL_DIR=/root/.opam/4.14.1+options/share/sail

    COPY data data
    COPY (+build-sailrs/sailrs) sailrs

    RUN find $SAIL_DIR/lib -type f -name '*.sail' | xargs sed -i '/lem_extern_type\|coq_extern_type/d'

    RUN ./sailrs data/arm-v9.4-a.json arm-v9.4-a.rkyv
    SAVE ARTIFACT arm-v9.4-a.rkyv arm-v9.4-a.rkyv

e2e-test-brig:
    FROM rust:alpine

    COPY (+e2e-test-sailrs/arm-v9.4-a.rkyv) arm-v9.4-a.rkyv
    COPY (+build-borealis/borealis) borealis

    RUN ./borealis --standalone arm-v9.4-a.rkyv aarch64.rs
    RUN rustc --edition 2021 aarch64.rs
 #   RUN ./aarch64 91500421
