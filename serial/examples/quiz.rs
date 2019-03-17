#![no_std]
#![no_main]

extern crate panic_semihosting;
extern crate cortex_m_rt as rt;
extern crate cortex_m_semihosting as sh;
extern crate microbit;

use core::fmt::Write;
use rt::entry;
use sh::hio;

use microbit::hal::delay::Delay;
use microbit::hal::prelude::*;
use microbit::hal::serial;
use microbit::hal::serial::BAUD115200;
use microbit::led;

const WINNING_SCORE: u8 = 6;
const QUESTION_COUNT: u8 = 2*WINNING_SCORE - 1;

const LETTER_A: [[u8; 5]; 5] = [
    [0, 0, 1, 0, 0],
    [0, 1, 0, 1, 0],
    [0, 1, 1, 1, 0],
    [0, 1, 0, 1, 0],
    [0, 1, 0, 1, 0],
];

const LETTER_B: [[u8; 5]; 5] = [
    [0, 1, 1, 0, 0],
    [0, 1, 0, 1, 0],
    [0, 1, 1, 0, 0],
    [0, 1, 0, 1, 0],
    [0, 1, 1, 0, 0],
];

#[entry]
fn main() -> ! {
    let mut stdout = hio::hstdout().unwrap();
    writeln!(stdout, "Start").unwrap();
    if let Some(p) = microbit::Peripherals::take() {
        // Split GPIO
        let mut gpio = p.GPIO.split();
        // Configure delay
        let mut delay = Delay::new(p.TIMER0);
        // Configure RX and TX pins accordingly
        let tx = gpio.pin24.into_push_pull_output().downgrade();
        let rx = gpio.pin25.into_floating_input().downgrade();
        // Configure serial communication
        let (mut tx, mut rx) = serial::Serial::uart0(p.UART0, tx, rx, BAUD115200).split();
        // Configure display pins
        let row1 = gpio.pin13.into_push_pull_output().downgrade();
        let row2 = gpio.pin14.into_push_pull_output().downgrade();
        let row3 = gpio.pin15.into_push_pull_output().downgrade();
        let col1 = gpio.pin4.into_push_pull_output().downgrade();
        let col2 = gpio.pin5.into_push_pull_output().downgrade();
        let col3 = gpio.pin6.into_push_pull_output().downgrade();
        let col4 = gpio.pin7.into_push_pull_output().downgrade();
        let col5 = gpio.pin8.into_push_pull_output().downgrade();
        let col6 = gpio.pin9.into_push_pull_output().downgrade();
        let col7 = gpio.pin10.into_push_pull_output().downgrade();
        let col8 = gpio.pin11.into_push_pull_output().downgrade();
        let col9 = gpio.pin12.into_push_pull_output().downgrade();
        // Configure display
        let mut leds = led::Display::new(
            row1, row2, row3, col1, col2, col3, col4, col5, col6, col7, col8, col9,
        );
        // Configure button GPIOs as inputs
        let button_a = gpio.pin17.into_floating_input();
        let button_b = gpio.pin26.into_floating_input();
        writeln!(tx, "Start");
        loop {
            let mut score_a: u8 = 0;
            let mut score_b: u8 = 0;
            for n in 0..QUESTION_COUNT {
                writeln!(tx, "Question {} - Score {}:{}", n, score_a, score_b);
                let mut button_a_low;
                let mut button_b_low;
                loop {
                    // Get button states
                    button_a_low = button_a.is_low();
                    button_b_low = button_b.is_low();
                    if button_a_low || button_b_low {
                        break;
                    }
                }
                let letter = match (button_a_low, button_b_low) {
                    (true, false) => {
                        writeln!(tx, "A");
                        LETTER_A
                    },
                    (false, true) => {
                        writeln!(tx, "B");
                        LETTER_B
                    },
                    (true, true) => {
                        writeln!(tx, "Tie! Next question.");
                        continue;
                    },
                    _ => unreachable!(),
                };
                leds.display(&mut delay, letter, 1000);
                loop {
                    // Keep asking until y or n is received
                    write!(tx, "Answer correct? [y/n] ");
                    let byte = block!(rx.read()).unwrap();
                    block!(tx.write(byte));
                    writeln!(tx);
                    match byte {
                        b'y' => {
                            if button_a_low {
                                score_a += 1;
                            }
                            if button_b_low {
                                score_b += 1;
                            }
                            break
                        },
                        b'n' => break,
                        _ => (),
                    }
                }
                if score_a >= WINNING_SCORE || score_b >= WINNING_SCORE {
                    break;
                }
            }
            writeln!(tx, "Final Score {}:{}", score_a, score_b);
        }
    }
    panic!("End");
}
