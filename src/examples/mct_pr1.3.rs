#![no_std]
#![no_main]

use rustuino::*;

#[entry]
fn main() -> ! {
  unsafe {
    (*RCC_PTR).ahb1enr.modify(|_, w| {
      w.gpioben().enabled();
      w.gpiocen().enabled()
    });

    (*GPIOB_PTR).moder.modify(|_, w| w.bits(0x5555));
    (*GPIOC_PTR).moder.modify(|_, w| w.bits(0x3));
    
    (*GPIOB_PTR).ord.write(|w| w.bits(0x1));
  }

  let mut input;
  let mut output;

  loop {
    output = 0x1;

    for i in 0..8 {
      output = (*GPIOB_PTR).odr.read().bits() << 1;

      input = (*GPIOC_PTR).idr.read().bits();
      input = ~input & 0x3;

      wait(200 * (input + 1));
    }
  }
}

fn wait(wait: u32) {
  // Wartefunktion, wartet entsprechend des wait-Werts
	for i in 0..wait {
		for j in 0..1800 {}
  }
}
