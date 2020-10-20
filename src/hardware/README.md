# Meet your hardware

Let's get familiar with the hardware we'll be working with.

## BBC micro:bit (the "microbit")

<p align="center">
<img title="microbit" src="http://tech.microbit.org/docs/hardware/assets/microbit-overview.png">
</p>

What does this board contain? For full details see the [microbit hardware page][microbit].

[microbit]: http://tech.microbit.org/hardware

- A Nordic nRF51822 microcontroller. This microcontroller has

  - A single core ARM Cortex-M0 processor with a maximum clock frequency of 16 MHz.

  - 256 KB of flash memory. (1 KB = 10**24** bytes)

  - 16 KB of static RAM.

  - many "peripherals": timers, GPIO, I2C, SPI, UART, etc.

  - This microcontroller operates at (around) 3.3V.

- 2 user buttons on the front and 1 reset button on the back.

- A 5x5 array of user LEDs.

- A configurable 23-pin edge connector

- A 2.4GHz radio transceiver with support for [bluetooth low energy][ble] (BLE).

[ble]: https://en.wikipedia.org/wiki/Bluetooth_Low_Energy

- An on-core nRF51 temperature sensor.

- An NXP/Freescale MMA8652 3-axis [accelerometer].

[accelerometer]: https://en.wikipedia.org/wiki/Accelerometer

- An NXP/Freescale MAG3110 3-axis [magnetometer].

[magnetometer]: https://en.wikipedia.org/wiki/Magnetometer

- A second microcontroller: NXP/Freescale KL26Z. This microcontroller handles the USB interface,
  communication between your computer and the main microcontroller,
  and converting the USB's input voltage from 5V to 3.3V.

## Versions

At the time of writing there are (soon to be) 3 released versions.
The Microbit foundation goes to herculean lengths to make this detail unimportant to you.
Here documents what you might want to know or how to find out.

- 1.3B [schematics][schematics1.3], The initial release
- 1.5 [schematics][schematics1.5], Combined the compass and accelerometer chips. [All Changes][1.3to1.5]
- 2 [new][2news], Announced new hex format, nRF52833, speaker, microphone.
- For full details see the [microbit hardware page][microbit]

If you're wondering where the other numbers are they'll be developer iterations you won't find in the wild.

[hwgithub]: https://github.com/bbcmicrobit/hardware
[schematics1.3]: https://github.com/bbcmicrobit/hardware/blob/08876867dc77a7e3af026d6db3c021f05116e10f/V1.3B/SCH_BBC-Microbit_V1.3B.pdf
[schematics1.5]: https://github.com/bbcmicrobit/hardware/blob/08876867dc77a7e3af026d6db3c021f05116e10f/V1.5/SCH_BBC-Microbit_V1.5.PDF
[1.3to1.5]: https://support.microbit.org/support/solutions/articles/19000087020-micro-bit-motion-sensor-hardware-change
[2news]: https://tech.microbit.org/latest-revision/announcement/

## Micro-USB Cable

This comes with your microbit but can be any generic cable,
and is used to connect the microbit to your computer.

## External battery pack

The external battery pack that comes with the microbit will not be used explicitly as part of this guide,
but feel free to use it to test your software without being tethered to a computer.

## Plugging it in

You can use the micro-USB cable to power the micro:bit, and to transfer data.
When you power up a new micro:bit you will see the display light up as the factory-installed program is executed.
Otherwise, the last program will automatically be executed.
The black reset button next to the USB input will restart the program being run.
