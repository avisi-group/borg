# brig

> DBT

## Run

```bash
$ cargo r
```

will build the kernel, place it inside a bootable UEFI image, then start QEMU with that image.


## Issues

### Panic Abort Errors

Make sure you are in the root dir not `kernel` when running `cargo r`
