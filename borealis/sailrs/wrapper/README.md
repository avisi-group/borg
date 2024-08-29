# Sail Wrapper

OCaml side of the Rust-OCaml interface to the Sail compiler.

`sail_plugin_isla.ml` is taken from `isla-sail` in [`https://github.com/rems-project/isla`](https://github.com/rems-project/isla), git rev #b572667.

The OCaml ecosystem is frustrating:

* I cannot add `isla-sail` as an opam package because `opam pin` does not support subdirectories
* I cannot use a git submodule as dune `vendored_dirs` does not support subdirectories
* I cannot even vendor the `isla-sail` directory itself because dune does not appear to support using an executable as a library

I hate this, this is so stupid, why can't it work like Cargo?
