#![no_std]
#![no_main]
#![allow(unused_imports)]

mod rustuino;
use rustuino::*;

#[entry]
fn main() -> ! {
    pin_mode("b", 0, true);
    pin_mode("b", 1, true);
    pin_mode("b", 2, true);
    pin_mode("b", 3, true);

    uart_init(115200);
    sprintln("Startup");

    loop {
        pin_write("b", 0, true);
        pin_write("b", 1, true);
        pin_write("b", 2, true);
        pin_write("b", 3, true);
        delay(1000);
        pin_write("b", 0, false);
        pin_write("b", 1, false);
        pin_write("b", 2, false);
        pin_write("b", 3, false);
        delay(1000);
    }
}
