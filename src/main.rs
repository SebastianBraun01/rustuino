#![no_std]
#![no_main]
#![feature(alloc_error_handler)]
#![allow(unused_imports)]



mod rustuino;
use rustuino::*;

#[entry]
fn main() -> ! {
    init_heap();

    uart_init(115200);
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
