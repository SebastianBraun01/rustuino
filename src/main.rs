#![no_std]
#![no_main]

use rustuino::entry;
use rustuino::gpio::*;

#[entry]
fn main() -> ! {
  let mut pin = PB1::get_as_output();

  set_value(&mut pin, true);
  set_speed(&mut pin, rustuino::Speed::Low);

  let pin = into_input(pin);

  let _value = read_value(&pin);

  // Cannot write to input pin!
  // set_value(&mut pin, true);

  let mut pin = into_output(pin);

  loop {
    set_value(&mut pin, false);
    set_value(&mut pin, true);
  }
}
