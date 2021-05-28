#![no_std]
#![no_main]
#![allow(unused_imports)]

use rustuino::*;

#[entry]
fn main() -> ! {
  pin_mode(PB0, Mode::Input);
	pin_mode(PB1, Mode::Output);

  uart_usb_init(115200, false, false);
  sprintln!("UART gestartet!");

  loop {
    sprintln!("UART gestartet!");
    time::delay(500);
  }
}
