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

You need a recent rust `nightly` compiler : probably `> 1.41.0`. If you don't have nightly, it should be automatically installed during first compilation.

Install QEMU following the [instructions for your OS](https://www.qemu.org/download/). For ubuntu :

```sh
apt install qemu
```

Then install `cargo-make` :

```sh
cargo install cargo-make
```

### Running

Run `cargo-make`, depending on your willing :

- to build the kernel :

    ```sh
    cargo m
    ```

- to run it :

    ```sh
    cargo m run
    ```

- to pass the tests, usefull to check for breaking changes :

    ```sh
    cargo m build_test
    ```

Actually, `cargo m` is just a sugar for `cargo make --makefile make.toml`.

That should be all, tell me if I forgot anything!

### Without `cargo-make`

`cargo-make` will automatically install required dependencies, and run QEMU.

To use this kernel without `cargo-make`, you need to install yourself required dependencies :

```sh
cargo install cargo-xbuild cargo-make bootimage
```

Then, you can build the kernel :

```sh
cargo xbuild
```

Or run it :

```sh
cargo xrun
```

Or pass the tests :

```sh
cargo xtest
```

Compiling by hand seems easier than using `cargo-make`. However, this tool will maybe be used a lot more soon to automatize builds : that's why it is the recommended method.

## Contributing

Every contribution is welcome, wether you are fluent in os dev or not!

You can start by reading [TODO](./TODO.md) and [ROADMAP](./ROADMAP.md). You can contact me if you have any question, and feel free to send pull request / open an issue whenever you want.
