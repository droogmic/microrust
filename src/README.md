# MicroRust

> Discover the world of microcontrollers through [Rust] on the [BBC micro:bit][microbit]!

[Rust]: https://www.rust-lang.org/
[microbit]: https://microbit.org/

This book is an introductory course on microcontroller-based embedded systems that uses Rust as the
teaching language (rather than the usual C/C++), and the micro:bit as target system.

## Scope

The following topics will be covered:

- How to write, build, flash and debug an embedded program.

- Functionality ("peripherals") commonly found in microcontrollers:
  - Digital input and output, including buttons and LEDs

<!-- - Functionality ("peripherals") commonly found in microcontrollers: Digital input and output, Pulse
  Width Modulation (PWM), Analog to Digital Converters (ADC), common communication protocols like
  Serial, I2C and SPI, etc. -->

<!-- - Multitasking concepts: cooperative vs preemptive multitasking, interrupts, schedulers, etc. -->

<!-- - Control systems concepts: sensors, calibration, digital filters, actuators, open loop control,
  closed loop control, etc. -->

## Approach

- Beginner friendly.
  No previous experience with microcontrollers or embedded systems is required.

- Hands on.
  *You* will be doing most of the work here.
  When possible, pages will end on a problem for you to solve, with the solution on the next page.
  Plenty of exercises to put the theory into practice.
  
- Standard. We'll make plenty use of standard tooling and processes to ease development.
  Fixing compiler errors, debugging with GDB, and logging will be introduced early on.
  Using LEDs as a debugging mechanism has no place here.

## Non-goals

What's out of scope for this book:

- Teaching Rust.
  There's plenty of material on that topic already.
  We'll focus on microcontrollers and embedded systems.

- Teaching electric circuit theory or electronics.
  We'll cover the minimum required to understand how some devices work along the way.

- Covering Rustic, low level details.
  We won't be talking about linker scripts, the boot process,
  or how to glue those two into a minimally working Rust program.

## Reporting problems

The source of this book is in [this repository].
If you encounter any typo or problem with the code report it on the [issue tracker],
or even submit a pull request.

[this repository]: https://github.com/droogmic/microrust
[issue tracker]: https://github.com/droogmic/microrust/issues
