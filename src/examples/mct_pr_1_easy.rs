#![no_std]
#![no_main]
#![allow(unused_imports)]

use rustuino::*;

#[entry]
fn main() -> ! {
  pin_mode(PB0, Mode::Output);
	pin_mode(PB1, Mode::Output);
  pin_mode(PB2, Mode::Output);
  pin_mode(PB3, Mode::Output);
  pin_mode(PB4, Mode::Output);
  pin_mode(PB5, Mode::Output);
  pin_mode(PB6, Mode::Output);
  pin_mode(PB7, Mode::Output);
  
  pin_mode(PC0, Mode::Input);
  pin_mode(PC1, Mode::Input);
  pin_mode(PC2, Mode::Input);
  pin_mode(PC3, Mode::Input);
  pin_mode(PC4, Mode::Input);
  pin_mode(PC5, Mode::Input);
  pin_mode(PC6, Mode::Input);
  pin_mode(PC7, Mode::Input);

  uart_usb_init(115200, false, false);
  sprintln!("UART gestartet!");

  loop {
    sprintln!("UART gestartet!");
    delay(500);
  }
}
