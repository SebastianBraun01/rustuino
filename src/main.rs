#![no_std]
#![no_main]
#![feature(alloc_error_handler)]
#![allow(unused_imports)]

mod rustuino;
use rustuino::*;

#[entry]
fn main() -> ! {
    init_heap();

    pin_mode("a", 2, Mode::AlterateFunction(7));
    pin_mode("a", 2, Mode::AlterateFunction(7));

    uart_init(2, 115200);
    sprintln!("UART gestartet!");
    analog_read_init();
    sprintln!("ADC gestartet!");
    let mut buffer: u16;

    loop {
        buffer = analog_get();
        sprintln!(buffer);
        delay(250);
    }
}
