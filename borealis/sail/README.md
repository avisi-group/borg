# `sail`

[![Docs](https://img.shields.io/badge/docs-sail-blue)](https://avisi.org.uk/borealis/sail/)

Rust interface for the the [Sail compiler and AST](https://www.cl.cam.ac.uk/~pes20/sail/) written in OCaml.

## Build Requirements

* [Rust toolchain](https://rustup.rs)
* [OCaml toolchain](https://ocaml.org)
* [Z3](https://github.com/Z3Prover/z3)
* [GMP](https://gmplib.org)
* [opam](https://opam.ocaml.org)
* [Dune](https://dune.build)
* [`sail` opam package](https://opam.ocaml.org/packages/sail/)

## Testing

`cargo test --feature redact` to run all tests.

`redact` feature must be enabled for snapshot tests to pass as some AST nodes are non-deterministic.

## Docker

`Dockerfile` included for reference on building `sail` but Github Actions workflow is more likely to be up-to-date and correct.
