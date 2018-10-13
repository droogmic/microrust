#![no_std]
#![no_main]

extern crate panic_semihosting;
extern crate cortex_m_rt as rt;
extern crate cortex_m_semihosting as sh;
extern crate heapless;
extern crate microbit;

use core::fmt::Write;
use rt::entry;
use sh::hio;
use heapless::{consts, Vec, String};

use microbit::hal::prelude::*;
use microbit::hal::delay::Delay;
use microbit::hal::serial;
use microbit::hal::serial::BAUD115200;

#[entry]
fn main() -> ! {
    let mut stdout = hio::hstdout().unwrap();
    writeln!(stdout, "Start").unwrap();
    if let Some(p) = microbit::Peripherals::take() {
        // Split GPIO
        let mut gpio = p.GPIO.split();
        // Create delay provider
        let mut delay = Delay::new(p.TIMER0);
        // Configure RX and TX pins accordingly
        let tx = gpio.pin24.into_push_pull_output().downgrade();
        let rx = gpio.pin25.into_floating_input().downgrade();
        // Configure serial communication
        let (mut tx, mut rx) = serial::Serial::uart0(p.UART0, tx, rx, BAUD115200).split();
        writeln!(tx, "Start");
        loop {
            // A buffer with 32 bytes of capacity
            let mut buffer: Vec<u8, consts::U32> = Vec::new();
            loop {
                // Read
                let byte = block!(rx.read()).unwrap();
                // Echo
                block!(tx.write(byte));
                // Carriage return
                if byte == b'\r' {
                    break;
                }
                // Push to buffer
                if buffer.push(byte).is_err() {
                    // Buffer full
                    writeln!(tx, "\r\nWarning: buffer full, dumping buffer");
                    break;
                }
            }
            // Buffer to string
            let buf_str = String::from_utf8(buffer).unwrap();
            writeln!(tx, "");
            match buf_str.parse() {
                // Transmit countdown
                Ok(buf_int) => {
                    for i in (1..buf_int).rev() {
                        delay.delay_ms(1000_u32);
                        writeln!(tx, "{}", i);
                    }
                    // Add post countdown effects here
                },
                // Transmit parse error
                Err(e) => writeln!(tx, "{:?}", e).unwrap(),
            }
        }
    }
    panic!("End");
}
