# Development environment setup

Dealing with microcontrollers involves several tools,
as we'll be dealing with an architecture different than your laptop's,
and we'll have to run and debug programs on a "remote" device.

## Documentation

Without documentation it is pretty much impossible to work with microcontrollers.

We'll be referring to the [micro:bit hardware page][microbit] and the links found within.

[microbit]: http://tech.microbit.org/hardware

*HEADS UP* Some of the links point to large PDF files several MBs in size.

## Tools

We'll use all the tools listed below. Where a minimum version is not specified,
any recent version should work but we have listed the version we have tested.

- Cargo & `rustc`.

- OpenOCD. version >=0.8

- `arm-none-eabi` toolchain. Tested version: gcc 8.1.0, binutils 2.30.

- `arm-none-eabi-gdb`.

- `minicom` on Linux and macOS. Tested version: 2.7.
  Readers report that `picocom` also works but we'll use `minicom` in this book.

- `PuTTY` on Windows.

Next, follow OS-agnostic installation instructions for a few of the tools:

### `rustc` & Cargo

Install rustup by following the instructions at [https://rustup.rs](https://rustup.rs).

Then, install or switch to the nightly channel.

``` shell
$ rustup default nightly
```

**NOTE** Make sure you have a nightly newer than `nightly-2018-10-12`.
`rustc -V` should return a date newer than the one shown below:

``` shell
$ rustc -V
rustc 1.31.0-nightly (2c2e2c57d 2018-10-12)
```

### OS specific instructions

Now follow the instructions specific to the OS you are using:

- [Linux](../setup/LINUX.html)
- [Windows](../setup/WINDOWS.html)
- [macOS](../setup/MACOS.html)
