# brig

> Hypervisor DBT

## Run

```bash
$ cargo r -r
```

will build the kernel, place it inside a bootable UEFI image, then start QEMU with that image.

## Memory Layout

```
0x0000_0000_0000┌────────────────────────────────────────────────┐
                │                  Kernel code                   │
                ├────────────────────────────────────────────────┤
                │         Architecture + device plugins          │
                ├────────────────────────────────────────────────┤
                │                                                │
                │                                                │
                │                                                │
                │                                                │
                │                                                │
                │                                                │
                │                                                │
                │                                                │
                │                                                │
                │                                                │
                │                                                │
                │                                                │
                │                                                │
                │                                                │
                │                                                │
                │                                                │
                │                                                │
0x7ffc_0000_0000├────────────────────────────────────────────────┤
                │                                                │
                │                Kernel heap (8GiB)              │
                │                                                │
0x7ffe_0000_0000├────────────────────────────────────────────────┤
                │                                                │
                │             Translation cache (8GiB)           │
                │                                                │
0x7fff_ffff_ffff├────────────────────────────────────────────────┤
                │                                                │
                │                                                │
                │                                                │
                │                                                │
                │                                                │
                │                                                │
                │                                                │
                │                                                │
                │                                                │
                │                                                │
                │                                                │
                │                                                │
                │                                                │
                │                                                │
                │         Virtual machine address space          │
                │                                                │
                │                                                │
                │                                                │
                │                                                │
                │                                                │
                │                                                │
                │                                                │
                │                                                │
                │                                                │
                │                                                │
                │                                                │
                │                                                │
                │                                                │
0xffff_ffff_ffff└────────────────────────────────────────────────┘
```
