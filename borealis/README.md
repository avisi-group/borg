# `sail`

> Rust interface for the the [Sail compiler and AST](https://www.cl.cam.ac.uk/~pes20/sail/) written in OCaml.

[![CI](https://github.com/avisi-group/sail-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/avisi-group/sail-rs/actions/workflows/ci.yml)
[![Docs](https://img.shields.io/badge/docs-sail-blue)](https://avisi.org.uk/sail-rs/sail/)

## Build Requirements

* [Rust toolchain](https://rustup.rs)
* [OCaml toolchain](https://ocaml.org)
* [Z3](https://github.com/Z3Prover/z3)
* [GMP](https://gmplib.org)
* [opam](https://opam.ocaml.org)
* [Dune](https://dune.build)
* [`sail` opam package](https://opam.ocaml.org/packages/sail/)

## Testing

`cargo test` to run all tests.

## Docker

`Dockerfile` included for reference on building `sail` but Github Actions workflow is more likely to be up-to-date and correct.
