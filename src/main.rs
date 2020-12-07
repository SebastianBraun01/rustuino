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
    pin_mode("b", 0, Mode::AlterateFunction(2));

    uart_init(2, 115200);
    sprintln!("UART gestartet!");
    pwm_init(3, 3);

    loop {
        for i in 0..255 {
            pin_write_pwm(3, 3, i);
            delay(10);
        }
    }
}
