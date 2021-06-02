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
    SysTick_Init();
  }

  loop {
    // ---
  }
}

fn SysTick_Init() {
  unsafe {
    (*SYSTICK_PTR).load.write(|w| w.bits(16000000));
    (*SYSTICK_PTR).ctrl.modify(|_, w| {
      w.tickint().set_bit();
      w.enable().set_bit()
    });
  }
}

#[exception]
fn SysTick() {
  unsafe {
    let mut buffer = (*GPIOB_PTR).odr.read().bits();

    if buffer < 127 {buffer = buffer << 1;}
    else {buffer = 1;}

    (*GPIOB_PTR).odr.write(|w| w.bits(buffer));
  }
  
}
