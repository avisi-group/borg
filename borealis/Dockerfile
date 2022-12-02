# Included for reference and testing that targeting ARM64 / MUSL is not entirely broken, as it is not tested in CI

FROM rust:alpine AS builder
WORKDIR /tmp/build

# install packages
RUN apk update && apk add opam alpine-sdk findutils zlib-dev gcc m4 z3

# download and build GMP
RUN curl https://gmplib.org/download/gmp/gmp-6.2.1.tar.lz | tar --lzip -x
RUN cd gmp-6.2.1; ./configure --prefix /tmp/gmp-prefix && make && make install
ENV CPPFLAGS=-I/tmp/gmp-prefix/include
ENV CFLAGS=-I/tmp/gmp-prefix/include
ENV LDFLAGS=-L/tmp/gmp-prefix/lib

# setup OCaml
RUN opam init --disable-sandboxing --enable-shell-hook -a -y
RUN opam init --shell-setup
RUN opam switch create 4.11.2+musl+static+flambda

# install sail
RUN eval `opam env` && opam install --assume-depexts -y sail

# build and document rust dependencies
RUN cargo init --lib borealis
RUN cargo init --lib sail
RUN cargo init --lib common
COPY Cargo.lock .
COPY Cargo.toml .
COPY borealis/Cargo.toml borealis/
COPY sail/Cargo.toml sail/
COPY common/Cargo.toml common/
RUN eval `opam env` && cargo build --release && cargo test --release --no-run && cargo doc --release

# copy full source
COPY . .
RUN touch borealis/src/lib.rs sail/src/lib.rs common/src/lib.rs

# build and run tests
RUN eval `opam env` && cargo test --release --no-fail-fast

# build borealis
RUN eval `opam env` && cargo build --release --all-targets

# build docs
RUN eval `opam env` && cargo doc --release
RUN echo '<!DOCTYPE html><html><head><meta http-equiv="refresh" content="0; URL=/borealis/borealis" /></head></html>' > target/doc/index.html

FROM scratch
COPY --from=builder /tmp/build/target/doc /doc
COPY --from=builder /tmp/build/target/release/borealis .
ENTRYPOINT [ "./borealis", "-o", ".", "--force" ]
