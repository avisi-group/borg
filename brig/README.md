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
$ cargo r -r
```

in the root directory will build the kernel, place it inside a bootable UEFI image, then start QEMU with that image.

## Issues

### Panic Abort Errors

Make sure you are in the root dir not `kernel` when running `cargo r`.
