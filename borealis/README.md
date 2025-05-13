# `borealis`

> ISA simulation and development toolchain

[![CI](https://github.com/avisi-group/borealis/actions/workflows/ci.yml/badge.svg)](https://github.com/avisi-group/borealis/actions/workflows/ci.yml)


## Usage

$ cargo r --bin borealis -- --dump-ir ./target ../arm-v9.4-a_d43f3f4c.rkyv ../aarch64


## Testing

`earthly +test` to run all tests.

## Docker/Earthly

The included `Earthfile` builds, tests, and generates documentation for `borealis`. It is used by the GitHub Actions workflow for CI. For that reason, `borealis` should always be `x86_64` compatible, while `aarch64` (and non-Linux) support will be provided on a best-effort basis.

### Usage

To build:

```
$ earthly +build
```

To run both unit and the end-2-end test:

```
$ earthly +test
```

## Workspace Packages

### `borealis`

Sail frontend to GenSim. Depends on `sail` crate to parse Sail definition, which is then compiled to GenC.

### `sail`

Rust interface for the the [Sail compiler and AST](https://www.cl.cam.ac.uk/~pes20/sail/) written in OCaml.

### `common`

Rust does not support cyclic dependencies among crates so this crate contains types and logic shared by multiple other crates in the workspace.

## Common Issues

### Linker Errors

Typically Rust programs benefit from a high degree of portability, unfortunately by depending on the Sail library `borealis` has a more complex linking situation. The `sail` crate contains an OCaml library called `wrapper`. This is built with the `build.rs` script in `sail`. `wrapper` depends on the Opam `sail` package, which depends on `conf-gmp` (a virtual Opam package relying on the system GMP library installation).

On several systems we have experienced linker errors relating to GMP as `conf-gmp` does not add the correct paths to the shared libary search paths. Building GMP statically and manually exporting the path to the object file does work, however we elected to add the `gmp` Opam package as a dependency to the `wrapper` library, as even though `sail` Opam package does not use it in any way, it sets the correct paths used by OCaml allowing `conf-gmp` to build successfully.

Additionally, on macOS, `-lgmp` must be supplied as well as the path to the Homebrew `/lib` folder, which is done in the `build.rs` inside conditional compilation flags.

### Wrong OCaml Version

Currently only version 4.14.0 is supported (or whichever version is in the Dockerfile). Until the `ocaml-rs` crate is updated to depend on the latest `ocaml-boxroot-sys`, building with OCaml 5 will result in several warnings and an error `vendor/boxroot/boxroot.c:821:7: error: too few arguments to function ‘action’` from `ocaml-boxroot-sys`.

### Missing Opam/Dune Binaries

If the `opam` or `dune` binary is not in `PATH` then you may experience the following errors in `ocaml-rs` or `sail` respectively:

```
error: failed to run custom build command for `ocaml-boxroot-sys v0.2.0`

Caused by:
  process didn't exit successfully: `/Users/ferdiamckeogh/.cargo/target/release/build/ocaml-boxroot-sys-88e415fa3d2fbfcd/build-script-build` (exit status: 101)
  --- stdout
  cargo:rerun-if-changed=vendor/boxroot/boxroot.c
  cargo:rerun-if-changed=vendor/boxroot/boxroot.h
  cargo:rerun-if-env-changed=OCAMLOPT
  cargo:rerun-if-env-changed=OCAML_WHERE_PATH

  --- stderr
  thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 2, kind: NotFound, message: "No such file or directory" }', /Users/ferdiamckeogh/.cargo/registry/src/github.com-1ecc6299db9ec823/ocaml-boxroot-sys-0.2.0/build.rs:108:26
  note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
warning: build failed, waiting for other jobs to finish...
error: failed to run custom build command for `ocaml-sys v0.22.3`

Caused by:
  process didn't exit successfully: `/Users/ferdiamckeogh/.cargo/target/release/build/ocaml-sys-de095a00ef3bb896/build-script-build` (exit status: 101)
  --- stdout
  cargo:rerun-if-env-changed=OCAMLOPT
  cargo:rerun-if-env-changed=OCAML_VERSION
  cargo:rerun-if-env-changed=OCAML_WHERE_PATH

  --- stderr
  thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 2, kind: NotFound, message: "No such file or directory" }', /Users/ferdiamckeogh/.cargo/registry/src/github.com-1ecc6299db9ec823/ocaml-sys-0.22.3/build.rs:143:11
  note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

```
error: failed to run custom build command for `sail v0.1.1 (/Users/ferdiamckeogh/Downloads/borealis/sail)`

Caused by:
  process didn't exit successfully: `/Users/ferdiamckeogh/.cargo/target/release/build/sail-c7644b8fa718363b/build-script-build` (exit status: 101)
  --- stderr
  The application panicked (crashed).
  Message:  Failed to execute dune, is it installed and in the PATH?
  Location: sail/build.rs:77

  Backtrace omitted. Run with RUST_BACKTRACE=1 environment variable to display it.
  Run with RUST_BACKTRACE=full to include source snippets.
```

To fix, ensure the `opam` and `dune` binaries are installed and made available to the environment in which you are compiling. For example, when using `rust-analyzer` in VSCode, you may want to launch VSCode from the command line prefixed with `eval (opam env)`, as opening it as an application may not result in the `PATH` being updated.
