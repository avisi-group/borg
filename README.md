# borg

## Borealis

> ISA simulation and development.

[![CI](https://github.com/avisi-group/borealis/actions/workflows/ci.yml/badge.svg)](https://github.com/avisi-group/borealis/actions/workflows/ci.yml)
[![Docs](https://img.shields.io/badge/docs-borealis-blue)](https://avisi.org.uk/borealis/borealis/)

### Build Requirements

* [Rust toolchain](https://rustup.rs)
* [OCaml toolchain](https://ocaml.org)
* [Z3](https://github.com/Z3Prover/z3)
* [GMP](https://gmplib.org)
* [opam](https://opam.ocaml.org)
* [Dune](https://dune.build)
* [`sail` opam package](https://opam.ocaml.org/packages/sail/)

### Usage

```bash
$ cd borealis && cargo r --bin borealis -- ../arm-v9.4-a_d43f3f4c.rkyv ../aarch64
```

## brig

> Unikernel dynamic binary translator

```
 __
/\ \             __
\ \ \____  _ __ /\_\     __             __4___
 \ \  __ \/\  __\/\ \  / _  \        _  \ \ \ \
  \ \ \L\ \ \ \/ \ \ \/\ \L\ \      <'\ /_/_/_/
   \ \____/\ \_\  \ \_\ \____ \      ((____!___/)
    \/___/  \/_/   \/_/\/___L\ \      \0\0\0\0\/
                         /\____/   ~~~~~~~~~~~~~~~~
                         \_/__/
```

### Usage

Running

```bash
$ cd brig-cli && cargo r
```

in the `brig-cli` directory will build the kernel and plugins, place them inside a bootable UEFI image and guest tarfile, then start QEMU with that image.

### Issues

#### Panic Abort Errors

Make sure you are in the `brig-cli` directory not `brig` when running `cargo r`.
