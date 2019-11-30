#![no_std]
#![no_main]

extern crate panic_semihosting;
extern crate cortex_m_rt as rt;
extern crate cortex_m_semihosting as sh;
extern crate microbit;

use core::fmt::Write;
use rt::entry;
use sh::hio;

use microbit::hal::prelude::*;
use microbit::hal::serial;
use microbit::hal::serial::BAUD115200;
use microbit::hal::nb::block;

#[entry]
fn main() -> ! {
    let mut stdout = hio::hstdout().unwrap();
    writeln!(stdout, "Start").unwrap();
    if let Some(p) = microbit::Peripherals::take() {
        // Split GPIO
        let gpio = p.GPIO.split();
        // Configure RX and TX pins accordingly
        let tx = gpio.pin24.into_push_pull_output().downgrade();
        let rx = gpio.pin25.into_floating_input().downgrade();
        // Configure serial communication
        let (mut tx, mut rx) = serial::Serial::uart0(p.UART0, tx, rx, BAUD115200).split();
        writeln!(tx, "Start");
        loop {
            let val = block!(rx.read()).unwrap();
            block!(tx.write(val));
        }
    }
    panic!("End");
}
