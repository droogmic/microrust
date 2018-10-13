# General troubleshooting

## OpenOCD problems

### can't connect to OpenOCD - "Error: open failed"

#### Symptoms

Upon trying to establish a *new connection* with the device you get an error
that looks like this:

```
$ openocd -f (..)
(..)
Error: open failed
in procedure 'init'
in procedure 'ocd_bouncer'
```

#### Cause + Fix

- All: The device is not (properly) connected. Check the USB connection using
  `lsusb` or the Device Manager.
- Linux: You may not have enough permission to open the device. Try again with
  `sudo`. If that works, you can use [these instructions] to make OpenOCD work
  without root privilege.
- Windows: You are probably missing the USB drivers.

[these instructions]: setup/LINUX.html#udev%20rules

### can't connect to OpenOCD - "Polling again in X00ms"

#### Symptoms

Upon trying to establish a *new connection* with the device you get an error
that looks like this:

```
$ openocd -f (..)
(..)
Error: jtag status contains invalid mode value - communication failure
Polling target stm32f3x.cpu failed, trying to reexamine
Examination failed, GDB will be halted. Polling again in 100ms
Info : Previous state query failed, trying to reconnect
Error: jtag status contains invalid mode value - communication failure
Polling target stm32f3x.cpu failed, trying to reexamine
Examination failed, GDB will be halted. Polling again in 300ms
Info : Previous state query failed, trying to reconnect
```

#### Cause

The microcontroller may have get stuck in some tight infinite loop or it may be
continuously raising an exception, e.g. the exception handler is raising an
exception.

#### Fix

- Close OpenOCD, if running
- Press and hold the reset (black) button
- Launch the OpenOCD command
- Now, release the reset button


### OpenOCD connection lost - "Polling again in X00ms"

#### Symptoms

A *running* OpenOCD session suddenly errors with:

```
# openocd -f (..)
Error: jtag status contains invalid mode value - communication failure
Polling target stm32f3x.cpu failed, trying to reexamine
Examination failed, GDB will be halted. Polling again in 100ms
Info : Previous state query failed, trying to reconnect
Error: jtag status contains invalid mode value - communication failure
Polling target stm32f3x.cpu failed, trying to reexamine
Examination failed, GDB will be halted. Polling again in 300ms
Info : Previous state query failed, trying to reconnect
```

#### Cause

The USB connection was lost.

#### Fix

- Close OpenOCD
- Disconnect and re-connect the USB cable.
- Re-launch OpenOCD

## Cargo problems

### "can't find crate for `core`"

#### Symptoms

```
   Compiling volatile-register v0.1.2
   Compiling rlibc v1.0.0
   Compiling r0 v0.1.0
error[E0463]: can't find crate for `core`

error: aborting due to previous error

error[E0463]: can't find crate for `core`

error: aborting due to previous error

error[E0463]: can't find crate for `core`

error: aborting due to previous error

Build failed, waiting for other jobs to finish...
Build failed, waiting for other jobs to finish...
error: Could not compile `r0`.

To learn more, run the command again with --verbose.
```

#### Cause

You are using a toolchain older than `nightly-2018-04-08` and forgot to call `rustup target add
thumbv7m-none-eabi`.

#### Fix

Update your nightly and install the `thumbv7em-none-eabihf` target.

``` console
$ rustup update nightly

$ rustup target add thumbv7em-none-eabihf
```

## Build problems

### `error: language item required, but not found: \`eh_personality\``

#### Cause

The `eh_personality` language item is used to implement stack unwinding in case a panic occurs.

#### Fix

You need to use the correct target
by using `--target thumbv6m-none-eabi`
or modifying `.cargo/config`

### `error: ld: cannot open linker script file memory.x: No such file or directory`

#### Cause

A memory.x file is needed, this specifies memory layout.

#### Fix

Either ask the board support crate maintainer to add memory.x to their crate,
or add a memory.x file to your project.
