# Borealis

> ISA simulation and development.

[![CI](https://github.com/avisi-group/borealis/actions/workflows/ci.yml/badge.svg)](https://github.com/avisi-group/borealis/actions/workflows/ci.yml)
[![Docs](https://img.shields.io/badge/docs-borealis-blue)](https://avisi.org.uk/borealis/borealis/)

## Build Requirements

* [Rust toolchain](https://rustup.rs)
* [OCaml toolchain](https://ocaml.org)
* [Z3](https://github.com/Z3Prover/z3)
* [GMP](https://gmplib.org)
* [opam](https://opam.ocaml.org)
* [Dune](https://dune.build)
* [`sail` opam package](https://opam.ocaml.org/packages/sail/)

## Usage

$ cargo r --bin borealis -- --dump-ir ./target ../arm-v9.4-a_d43f3f4c.rkyv ~/Documents/Sync/brig-aarch64
