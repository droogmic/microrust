#![no_std]
#![no_main]

extern crate panic_abort;
extern crate cortex_m_rt as rt;

#[macro_use(entry, exception)]
extern crate microbit;

use core::fmt::Write;
use rt::ExceptionFrame;

use microbit::hal::prelude::*;
use microbit::hal::delay::Delay;
use microbit::hal::serial;
use microbit::hal::serial::BAUD115200;

exception!(HardFault, hard_fault);

fn hard_fault(ef: &ExceptionFrame) -> ! {
    panic!("{:#?}", ef);
}

exception!(*, default_handler);

fn default_handler(irqn: i16) {
    panic!("Unhandled exception (IRQn = {})", irqn);
}

entry!(main);
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
        // delay
        let mut delay = Delay::new(p.TIMER0);
        loop {
            writeln!(
                tx, "Button A(down:{}, up:{}) B(down:{}, up:{})", 
                button_a.is_low(),
                button_a.is_high(),
                button_b.is_low(),
                button_b.is_high(),
            ).unwrap();
            delay.delay_ms(100_u8);
        }
    }
    panic!("End");
}
