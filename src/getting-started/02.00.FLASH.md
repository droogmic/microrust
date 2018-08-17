# Flashing

Flashing is the process of moving our program into the microcontroller's (persistent) memory. Once flashed, the microcontroller will execute the flashed program every time it is powered on.

In this case, our `rustled` program will be the only program in the microcontroller memory. By this I mean that there's nothing else running on the microcontroller: no OS, no daemon, nothing. `rustled` has full control over the device. This is what is meant by *bare-metal* programming.

<dl>
  <dt>OS</dt>
  <dd>operating system</dd>
  <dt>Daemon</dt>
  <dd>program running in the background</dd>
</dl>

Connect the micro:bit to your computer and run the following commands on a new terminal.

We need to give OCD the name of the interfaces we are using:

``` console
$ # All
$ # Windows: remember that you need an extra `-s %PATH_TO_OPENOCD%\<version>\scripts`
$ openocd -f interface/cmsis-dap.cfg -f target/nrf51.cfg
```

The program will block; leave that terminal open.

Now it's a good time to explain what this command is actually doing.

I mentioned that the micro:bit actually has two microcontrollers.
One of them is used as a USB interface and programmer/debugger.
This microcontroller is connected to the target microcontroller using a Serial Wire Debug (SWD) interface 
(this interface is an ARM standard so you'll run into it when dealing with other Cortex-M based microcontrollers).
This SWD interface can be used to flash and debug a microcontroller.
It uses the CMSIS-DAP protocol for host debugging of application programs.
It will appear as a USB device when you connect the micro:bit to your laptop.

As for OpenOCD,
it's software that provides some services like a *GDB server* 
on top of USB devices that expose a debugging protocol like SWD or JTAG.

> GDB: The **G**NU **d**e**b**ugger will allow us to debug our software 
> by controlling the execution of our program.
> We will learn more about this a little bit later.

Onto the actual command: those `.cfg` files we are using instruct OpenOCD to look for
- a CMSIS-DAP USB interface device (`interface/cmsis-dap.cfg`)
- a nRF51XXX microcontroller target (`target/nrf51.cfg`) to be connected to the USB interface.

The OpenOCD output looks like this:

``` console
Open On-Chip Debugger 0.10.0
Licensed under GNU GPL v2
For bug reports, read
	http://openocd.org/doc/doxygen/bugs.html
Info : auto-selecting first available session transport "swd". To override use 'transport select <transport>'.
cortex_m reset_config sysresetreq
adapter speed: 1000 kHz
Info : CMSIS-DAP: SWD  Supported
Info : CMSIS-DAP: Interface Initialised (SWD)
Info : CMSIS-DAP: FW Version = 1.0
Info : SWCLK/TCK = 1 SWDIO/TMS = 1 TDI = 0 TDO = 0 nTRST = 0 nRESET = 1
Info : CMSIS-DAP: Interface ready
Info : clock speed 1000 kHz
Info : SWD DPIDR 0x0bb11477
Info : nrf51.cpu: hardware has 4 breakpoints, 2 watchpoints
```

The "4 breakpoints, 2 watchpoints" part indicates the debugging features the processor has
available.

I mentioned that OpenOCD provides a GDB server so let's connect to that right now:

``` console
$ arm-none-eabi-gdb -q target/thumbv6m-none-eabi/debug/rustled
Reading symbols from target/thumbv6m-none-eabi/debug/rustled...done.
(gdb)
```

This only opens a GDB shell. To actually connect to the OpenOCD GDB server, use the following
command within the GDB shell:

``` gdb
(gdb) target remote :3333
Remote debugging using :3333
0x00000000 in ?? ()
```

By default OpenOCD's GDB server listens on TCP port 3333 (localhost). This command is connecting to
that port.

After entering this command, you'll see new output in the OpenOCD terminal:

``` diff
 Info : stm32f3x.cpu: hardware has 4 breakpoints, 2 watchpoints
+Info : accepting 'gdb' connection on tcp/3333
+Info : nRF51822-QFAA(build code: H0) 256kB Flash
```

Almost there. To flash the device, we'll use the `load` command inside the GDB shell:

``` gdb
(gdb) load
Loading section .vector_table, size 0x188 lma 0x8000000
Loading section .text, size 0x38a lma 0x8000188
Loading section .rodata, size 0x8 lma 0x8000514
Start address 0x8000188, load size 1306
Transfer rate: 6 KB/sec, 435 bytes/write.
```

And that's it. You'll also see new output in the OpenOCD terminal.

``` diff
 Info : flash size = 256kbytes
+Info : Unable to match requested speed 1000 kHz, using 950 kHz
+Info : Unable to match requested speed 1000 kHz, using 950 kHz
+adapter speed: 950 kHz
+target state: halted
+target halted due to debug-request, current mode: Thread
+xPSR: 0x01000000 pc: 0x08000194 msp: 0x2000a000
+Info : Unable to match requested speed 8000 kHz, using 4000 kHz
+Info : Unable to match requested speed 8000 kHz, using 4000 kHz
+adapter speed: 4000 kHz
+target state: halted
+target halted due to breakpoint, current mode: Thread
+xPSR: 0x61000000 pc: 0x2000003a msp: 0x2000a000
+Info : Unable to match requested speed 1000 kHz, using 950 kHz
+Info : Unable to match requested speed 1000 kHz, using 950 kHz
+adapter speed: 950 kHz
+target state: halted
+target halted due to debug-request, current mode: Thread
+xPSR: 0x01000000 pc: 0x08000194 msp: 0x2000a000
```

Our program is loaded, we can now run it!

``` gdb
(gdb) continue
Continuing.
```

Continue runs the program until the next breakpoint.
This time it blocks, nothing happens.
This is because all we have in our code is a loop!

## `.gdbinit`

Before we move on though, we are going to add one more file to our project.
This will automate the last few steps so we don't need to repeatedly do the same actions in gdb:

`.gdbinit`

``` gdbinit
# Connects GDB to OpenOCD server port
target remote :3333
# (optional) Unmangle function names when debugging
set print asm-demangle on
# Load your program, breaks at entry
load
# (optional) Add breakpoint at function
break rustled::main
# Continue with execution
continue
```

Now we can learn how to debug code on the micro:bit.