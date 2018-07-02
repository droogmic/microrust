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

- A configureable 23-pin edge connector

- A 2.4GHz radio transciever with support for [bluetooth low energy][ble] (BLE).

[ble]: https://en.wikipedia.org/wiki/Bluetooth_Low_Energy

- An on-core nRF51 temperature sensor.

- An NXP/Freescale MMA8652 3-axis [accelerometer].

[accelerometer]: https://en.wikipedia.org/wiki/Accelerometer

- An NXP/Freescale MAG3110 3-axis [magnetometer].

[magnetometer]: https://en.wikipedia.org/wiki/Magnetometer

- A second microcontroller: NXP/Freescale KL26Z. This microcontroller handles the USB interface,
  communication between your computer and the main microcontroller, 
  and changing the USB's input voltage from 5V to 3.3V.

## Micro-USB Cable

This can be any generic cable, and is used to connect the microbit to your computer.

## External battery pack

The external battery pack that comes with the microbit will not be used explicitly as part of this guide, 
but feel free to use it to test your software without being tethered to a computer.
