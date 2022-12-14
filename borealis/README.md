# `borealis`

> ISA simulation and development toolchain

[![CI](https://github.com/avisi-group/borealis/actions/workflows/ci.yml/badge.svg)](https://github.com/avisi-group/borealis/actions/workflows/ci.yml)
[![borealis docs](https://img.shields.io/badge/docs-borealis-blue)](https://avisi.org.uk/borealis/borealis/)
[![sail docs](https://img.shields.io/badge/docs-sail-blue)](https://avisi.org.uk/borealis/sail/)
[![Lines of code](https://tokei.rs/b1/github/avisi-group/borealis)](https://github.com/avisi-group/borealis)

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

The included `Dockerfile` builds, tests, and generates documentation for `borealis`. It is used by the GitHub Actions workflow for CI. For that reason, the Docker image should always be compatible with `x86_64` while `aarch64` (and non-Linux) support will be provided on a best-effort basis.

## Workspace Packages

### `borealis`

Sail frontend to GenSim. Depends on `sail` crate to parse Sail definition, which is then compiled to GenC.

### `sail`

Rust interface for the the [Sail compiler and AST](https://www.cl.cam.ac.uk/~pes20/sail/) written in OCaml.

### `common`

Rust does not support cyclic dependencies among crates so this crate contains types and logic shared by multiple other crates in the workspace.

### `borealis_macro`

Procedural macros must be defined in a separate crate, this is such a crate for the workspace and will contain all procedural macros required by other crates.
