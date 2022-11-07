# `borealis`

> ISA simulation and development toolchain

[![CI](https://github.com/avisi-group/sail-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/avisi-group/sail-rs/actions/workflows/ci.yml)
[![borealis docs](https://img.shields.io/badge/docs-borealis-blue)](https://avisi.org.uk/sail-rs/borealis/)
[![sail docs](https://img.shields.io/badge/docs-sail-blue)](https://avisi.org.uk/sail-rs/sail/)

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

`Dockerfile` included for reference on building `borealis` but Github Actions workflow is more likely to be up-to-date and correct.

## Workspace Contents

### `borealis`

Sail frontend to GenSim. Depends on `sail` crate to parse Sail definition, which is then compiled to GenC.

### `sail`

Rust interface for the the [Sail compiler and AST](https://www.cl.cam.ac.uk/~pes20/sail/) written in OCaml.
