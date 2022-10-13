# Included for reference and testing that targeting ARM64 is not entirely broken, as it is not tested in CI

FROM rust AS builder
WORKDIR /tmp/build

# install OCaml, Opam, dune, and sail
RUN apt update && apt install -y opam
RUN opam init --disable-sandboxing --enable-shell-hook -a -y
RUN opam init --shell-setup

ENV PATH="/root/.opam/default/bin:$PATH"

RUN opam install -y dune sail

# build rust dependencies
RUN cargo init --lib .
RUN mkdir examples && touch examples/cli.rs
COPY Cargo.toml .
RUN cargo build

# run tests
COPY . .
RUN touch src/lib.rs
RUN . ~/.opam/opam-init/init.sh && cargo test --all-features --all-targets --no-fail-fast
