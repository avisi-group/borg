FROM rust:latest

RUN apt-get update
RUN apt-get install -y build-essential device-tree-compiler opam mold gcc-aarch64-linux-gnu gcc-arm-none-eabi z3 qemu-system qemu-user

RUN opam init --disable-sandboxing --bare -y
RUN opam switch create default --packages ocaml-variants.5.2.1+options,ocaml-option-flambda

RUN opam install -y sail=0.18 gmp dune ocaml-lsp-server ocamlformat
