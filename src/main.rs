#![no_std]
#![no_main]

use rustuino::{Mode, entry, pin_mode};
use rustuino::gpio::{GetAsOutput, OutputPin, PB0};

#[entry]
fn main() -> ! {
    pin_mode(PA2, Mode::Input);

    let peri = stm32f4::stm32f446::Peripherals::take().unwrap();

    let mut pin = PB0::get_as_output();

    pin.set_value(true);
    pin.set_speed(rustuino::Speed::Low);

    let pin = pin.into_input();

    let value = pin.read_value();

    // Cannot write to input pin!
    // pin.set_value(true);

    let mut pin = pin.into_output();

    loop {
        pin.set_value(false);
        pin.set_value(true);
    }
}
