# brig

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

## Usage

Running

```bash
$ cargo r
```

in the `brig-cli` directory will build the kernel and plugins, place them inside a bootable UEFI image and guest tarfile, then start QEMU with that image.

The `standalone` directory can be built to run the generated ISA model outside of `brig` as a normal binary.

## Issues

### Panic Abort Errors

Make sure you are in the `brig-cli` directory not `brig` when running `cargo r`.
