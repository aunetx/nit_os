/*!
# nit_os

![Make](https://github.com/aunetx/nit_os/workflows/Make/badge.svg)
![Build binary](https://github.com/aunetx/nit_os/workflows/Build%20binary/badge.svg)

`nit_os` is a toy os written in rust, following [phil-opp's blog](https://os.phil-opp.com/).

## Getting started

### Description

This projects tries to create a working kernel for `x86_64` architecture, with a simple architecture yet the most complete possible.

It will probably not be *UNIX-compatible, and may not see userspace toolchain soon.

Its goals are :

- modularity / reusability through a complete library
- complete hardware support (graphics, pci, usb...)
- efficient multitasking / multiprocessing
- support for rust std?
- a complete graphical session
- simplicity of configuration (but config could be done at compile-time)
- easily understandable and well-documented
- easily runnable (inside AND outside QEMU)
- support for BIOS, UEFI and arbitrary bootloader

### Prerequesites

A recent rust `nightly` compiler : probably `> 1.41.0`.

You will need to install `cargo-xbuild`, `cargo-make` and `bootimage` crates :

```sh
cargo install cargo-xbuild cargo-make bootimage
```

To boot into QEMU, you will need a decent version of it installed too.

That should be all, tell me if I forgot anything!

### Running it

Thanks to `cargo-make`, running the kernel is as simple as :

```sh
cargo make --makefile make.toml run
```

To run the tests (usefull to check for breaking changes) :

```sh
cargo make --makefile make.toml build_test
```

## Contributing

Every contribution is welcome, wether you are fluent in os dev or not!

You can start by reading [TODO](./TODO.md) and [ROADMAP](./ROADMAP.md). You can contact me if you have any question, and feel free to send pull request / open an issue whenever you want.

*/

#![no_std]
#![cfg_attr(test, no_main)]
#![test_runner(crate::architecture::testing::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![feature(
    // tests
    custom_test_frameworks,
    // exceptions
    abi_x86_interrupt,
    // allocators
    const_fn,
    alloc_layout_extra,
    alloc_error_handler,
    const_in_array_repeat_expressions
)]

// enable the builtin alloc crate
extern crate alloc;

// submodules exports
pub mod architecture;
pub mod drivers;
pub mod interrupts;
pub mod memory;
