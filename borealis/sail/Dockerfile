# Included for reference and testing that targeting ARM64 / MUSL is not entirely broken, as it is not tested in CI

FROM rust:alpine AS builder
WORKDIR /tmp/build

# install packages
RUN apk update && apk add opam alpine-sdk findutils zlib-dev gcc m4 z3

# download and build GMP
RUN curl https://gmplib.org/download/gmp/gmp-6.2.1.tar.lz | tar --lzip -x
RUN cd gmp-6.2.1; ./configure --prefix /tmp/gmp-prefix && make && make install

# setup OCaml
RUN opam init --disable-sandboxing --enable-shell-hook -a -y
RUN opam init --shell-setup
RUN opam switch create 4.11.2+musl+static+flambda

# install sail
RUN eval `opam env` && CPPFLAGS=-I/tmp/gmp-prefix/include CFLAGS=-I/tmp/gmp-prefix/include LDFLAGS=-L/tmp/gmp-prefix/lib opam install --assume-depexts -y sail

# build rust dependencies
RUN cargo init --lib .
COPY Cargo.toml .
RUN eval `opam env` && cargo build --release --tests --examples

# run tests
COPY . .
RUN touch src/lib.rs
RUN eval `opam env` && cargo test --release --no-fail-fast
