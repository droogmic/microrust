#![deny(unsafe_code)]
#![no_std]

extern crate aux5;

use aux5::prelude::*;
use aux5::{Delay, Leds};

fn main() {
    let (mut delay, mut leds): (Delay, Leds) = aux5::init();

    let period = 50_u16;
    let mut step: usize = 0;
    loop {
        match step % 2 {
            0 => leds[step/2].on(),
            1 => {
                let wrap_step = ((step + 16 - 3) / 2) % 8;
                leds[wrap_step].off();
            },
            _ => unreachable!(),
        }
        delay.delay_ms(period);
        match step {
            15 => step = 0,
            _ => step += 1,
        }
    }
}
