#![no_std]
#![no_main]

extern crate panic_semihosting;
extern crate cortex_m_rt as rt;
extern crate cortex_m_semihosting as sh;

#[macro_use(entry, exception)]
extern crate microbit;

use core::fmt::Write;
use rt::ExceptionFrame;
use sh::hio;

use microbit::hal::delay::Delay;
use microbit::hal::gpio::gpio::PIN;
use microbit::hal::gpio::gpio::{PIN4, PIN5, PIN6, PIN7, PIN8, PIN9, PIN10, PIN11, PIN12, PIN13, PIN14, PIN15};
use microbit::hal::gpio::{Output, PushPull};
use microbit::hal::serial;
use microbit::hal::serial::BAUD115200;
use microbit::hal::prelude::*;

exception!(HardFault, hard_fault);

fn hard_fault(ef: &ExceptionFrame) -> ! {
    panic!("{:#?}", ef);
}

exception!(*, default_handler);

fn default_handler(irqn: i16) {
    panic!("Unhandled exception (IRQn = {})", irqn);
}

type LED = PIN<Output<PushPull>>;

const DEFAULT_DELAY_MS: u32 = 2;
const LED_LAYOUT: [[(usize, usize); 5]; 5] = [
    [(0, 0), (1, 3), (0, 1), (1, 4), (0, 2)],
    [(2, 3), (2, 4), (2, 5), (2, 6), (2, 7)],
    [(1, 1), (0, 8), (1, 2), (2, 8), (1, 0)],
    [(0, 7), (0, 6), (0, 5), (0, 4), (0, 3)],
    [(2, 2), (1, 6), (2, 0), (1, 5), (2, 1)],
];

/// Array of all the LEDs in the 5x5 display on the board
pub struct Display {
    delay_ms: u32,
    rows: [LED; 3],
    cols: [LED; 9],
}

impl Display {
    /// Initializes all the user LEDs
    pub fn new(
        col1: PIN4<Output<PushPull>>,
        col2: PIN5<Output<PushPull>>,
        col3: PIN6<Output<PushPull>>,
        col4: PIN7<Output<PushPull>>,
        col5: PIN8<Output<PushPull>>,
        col6: PIN9<Output<PushPull>>,
        col7: PIN10<Output<PushPull>>,
        col8: PIN11<Output<PushPull>>,
        col9: PIN12<Output<PushPull>>,
        row1: PIN13<Output<PushPull>>,
        row2: PIN14<Output<PushPull>>,
        row3: PIN15<Output<PushPull>>,
    ) -> Self {
        let mut retval = Display {
            delay_ms: DEFAULT_DELAY_MS,
            rows: [row1.downgrade(), row2.downgrade(), row3.downgrade()],
            cols: [
                col1.downgrade(), col2.downgrade(), col3.downgrade(),
                col4.downgrade(), col5.downgrade(), col6.downgrade(),
                col7.downgrade(), col8.downgrade(), col9.downgrade()
            ],
        };
        // This is needed to reduce flickering on reset
        retval.clear();
        retval
    }

    /// Clear display
    pub fn clear(&mut self) {
        for row in &mut self.rows {
            row.set_low();
        }
        for col in &mut self.cols {
            col.set_high();
        }
    }

    /// Convert 5x5 display image to 3x9 matrix image
    pub fn display2matrix(led_display: [[u8; 5]; 5]) -> [[u8; 9]; 3] {
        let mut led_matrix: [[u8; 9]; 3] = [[0; 9]; 3];
        for (led_display_row, layout_row) in led_display.iter().zip(LED_LAYOUT.iter()) {
            for (led_display_val, layout_loc) in led_display_row.iter().zip(layout_row) {
                led_matrix[layout_loc.0][layout_loc.1] = *led_display_val;
            }
        }
        led_matrix
    }

    /// Display 5x5 display image for a given duration
    pub fn display(&mut self, delay: &mut Delay, led_display: [[u8; 5]; 5], duration_ms: u32) {
        let led_matrix = Display::display2matrix(led_display);
        // Calculates how long to block for
        // e.g. If the duration_ms is 500ms (half a second)
        //      and self.delay_ms is 2ms (about 2ms per scan row),
        //      each refresh takes 3rows×2ms, so we need 500ms / (3×2ms) loops.
        let loops = duration_ms / (self.rows.len() as u32 * self.delay_ms);
        for _ in 0..loops {
            for (row_line, led_matrix_row) in self.rows.iter_mut().zip(led_matrix.iter()) {
                row_line.set_high();
                for (col_line, led_matrix_val) in self.cols.iter_mut().zip(led_matrix_row.iter()) {
                    // We are keeping it simple, and not adding brightness
                    if *led_matrix_val > 0 {
                        col_line.set_low();
                    }
                }
                delay.delay_ms(self.delay_ms);
                for col_line in &mut self.cols {
                    col_line.set_high();
                }
                row_line.set_low();
            }
        }
    }
}

entry!(main);
fn main() -> ! {
    let mut stdout = hio::hstdout().unwrap();
    writeln!(stdout, "Start").unwrap();
    if let Some(p) = microbit::Peripherals::take() {
        // Split GPIO
        let mut gpio = p.GPIO.split();
        
        // Configure RX and TX pins accordingly
        let tx = gpio.pin24.into_push_pull_output().downgrade();
        let rx = gpio.pin25.into_floating_input().downgrade();
        // Configure serial communication
        let (mut tx, _) = serial::Serial::uart0(p.UART0, tx, rx, BAUD115200).split();
        write!(tx, "serial - start\r\n");
        
        // Create delay provider
        let mut delay = Delay::new(p.TIMER0);

        // Display pins
        let row1 = gpio.pin13.into_push_pull_output();
        let row2 = gpio.pin14.into_push_pull_output();
        let row3 = gpio.pin15.into_push_pull_output();
        let col1 = gpio.pin4.into_push_pull_output();
        let col2 = gpio.pin5.into_push_pull_output();
        let col3 = gpio.pin6.into_push_pull_output();
        let col4 = gpio.pin7.into_push_pull_output();
        let col5 = gpio.pin8.into_push_pull_output();
        let col6 = gpio.pin9.into_push_pull_output();
        let col7 = gpio.pin10.into_push_pull_output();
        let col8 = gpio.pin11.into_push_pull_output();
        let col9 = gpio.pin12.into_push_pull_output();
        let mut leds = Display::new(
            col1, col2, col3,
            col4, col5, col6,
            col7, col8, col9,
            row1, row2, row3,
        );

        #[allow(non_snake_case)]
        let letter_I = [
            [0, 1, 1, 1, 0],
            [0, 0, 1, 0, 0],
            [0, 0, 1, 0, 0],
            [0, 0, 1, 0, 0],
            [0, 1, 1, 1, 0],
        ];

        let heart = [
            [0, 1, 0, 1, 0],
            [1, 0, 1, 0, 1],
            [1, 0, 0, 0, 1],
            [0, 1, 0, 1, 0],
            [0, 0, 1, 0, 0],
        ];

        #[allow(non_snake_case)]
        let letter_U = [
            [0, 1, 0, 1, 0],
            [0, 1, 0, 1, 0],
            [0, 1, 0, 1, 0],
            [0, 1, 0, 1, 0],
            [0, 1, 1, 1, 0],
        ];

        let _ = write!(tx, "\n\rStarting!\n\r");

        loop {
            let _ = write!(tx, "I <3 Rust on the micro:bit!\n\r");
            leds.display(&mut delay, letter_I, 1000);
            leds.display(&mut delay, heart, 1000);
            leds.display(&mut delay, letter_U, 1000);
            leds.clear();
            delay.delay_ms(250_u32);
        }

    }
    panic!("End");

}
