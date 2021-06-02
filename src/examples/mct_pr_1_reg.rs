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
    (*SYSTICK_PTR).load.write(|w| w.bits(16000000));
    (*SYSTICK_PTR).ctrl.modify(|_, w| {
      w.tickint().set_bit();
      w.enable().set_bit()
    });
  }

  uart_usb_init(115200, false, false);
  sprintln!("UART gestartet!");

  loop {
    sprintln!("UART gestartet!");
    delay(500);
  }
}

#[exception]
fn SysTick() {
  
}
