#![no_std]
#![no_main]

extern crate panic_abort;
extern crate cortex_m_rt as rt;
extern crate microbit;

use core::fmt::Write;
use rt::entry;

use microbit::hal::prelude::*;
use microbit::hal::serial;
use microbit::hal::serial::BAUD115200;

#[entry]
fn main() -> ! {
    if let Some(p) = microbit::Peripherals::take() {
        // Split GPIO
        let mut gpio = p.GPIO.split();
        // Configure RX and TX pins accordingly
        let tx = gpio.pin24.into_push_pull_output().downgrade();
        let rx = gpio.pin25.into_floating_input().downgrade();
        // Configure serial communication
        let (mut tx, _) = serial::Serial::uart0(p.UART0, tx, rx, BAUD115200).split();
        // Configure button GPIOs as inputs
        let button_a = gpio.pin17.into_floating_input();
        let button_b = gpio.pin26.into_floating_input();
        // loop variables
        let mut state_a_low = false;
        let mut state_b_low = false;
        loop {
            // Get button states
            let button_a_low = button_a.is_low();
            let button_b_low = button_b.is_low();
            if button_a_low && !state_a_low {
                writeln!(tx, "Button A down").unwrap();
            }
            if button_b_low && !state_b_low {
                writeln!(tx, "Button B down").unwrap();
            }
            if !button_a_low && state_a_low {
                writeln!(tx, "Button A up").unwrap();
            }
            if !button_b_low && state_b_low {
                writeln!(tx, "Button B up").unwrap();
            }
            // Store buttons states
            // This should not read the GPIO pins again, as the state
            // may have changed and the change will not be recorded
            state_a_low = button_a_low;
            state_b_low = button_b_low;
        }
    }
    panic!("End");
}
