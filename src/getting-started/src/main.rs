#![no_std]
#![no_main]

extern crate panic_abort;
extern crate cortex_m_rt as rt;

#[macro_use(entry, exception)]
extern crate microbit;

use rt::ExceptionFrame;

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
    let _y;
    let x = 42;
    _y = x;
    loop {}
}