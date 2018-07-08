# Verify the installation

Let's verify that all the tools were installed correctly.

## Linux only

### Verify permissions

Connect the micro:bit to your laptop using an USB cable.

The micro:bit should now appear as a USB device (file) in `/dev/bus/usb`.
Let's find out how it got enumerated:

``` console
$ lsusb | grep -i NXP
Bus 002 Device 033: ID 0d28:0204 NXP ARM mbed
    ^^^        ^^^
```

In my case, the micro:bit got connected to the bus #2 and got enumerated as the device #33.
This means the file `/dev/bus/usb/002/033` *is* the Fmicro:bit3.
Let's check its permissions:

``` console
$ ls -l /dev/bus/usb/002/033
crw-rw---- 1 root uucp 189, 160 Jul  8 14:06 /dev/bus/usb/002/033
                  ^^^^
```

The group should be `uucp`.
If it's not ... then check your [udev rules] and try re-loading them with:

[udev rules]: 03-setup/linux.html#udev%20rules

``` console
$ sudo udevadm control --reload-rules
```

## All

### First OpenOCD connection

First, connect the micro:bit to your computer using the micro-USB cable.
The *yellow* LED next to the USB input should turn on right after connecting the USB cable to the board.

Next, run this command:

``` console
$ # *nix
$ openocd-f interface/cmsis-dap.cfg -f target/nrf51.cfg

$ # Windows
$ # NOTE cygwin users have reported problems with the -s flag. If you run into
$ # that you can call openocd from the `C:\OpenOCD\share\scripts` directory
$ openocd -s C:\OpenOCD\share\scripts -f interface/cmsis-dap.cfg -f target/nrf51.cfg
```

> **NOTE** Windows users: `C:\OpenOCD` is the directory where you installed OpenOCD to.

You should see output like this:

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

(If you don't ... then check the [general troubleshooting] instructions.)

[general troubleshooting]: appendix/1-general-troubleshooting/README.html

`openocd` will block the terminal. That's fine.

Also, the `yellow` LED should start blinking very fast.
It may seem concerning, but it is a good sign.

That's it! It works. You can now close/kill `openocd`.
