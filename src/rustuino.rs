#![allow(dead_code)]

pub use cortex_m_rt::entry;
pub use panic_halt as _;

pub fn delay(ms: u32) {
  let rcc_ptr = stm32f4::stm32f446::RCC::ptr();
  let tim2_ptr = stm32f4::stm32f446::TIM2::ptr();

  unsafe {
    (*rcc_ptr).apb1enr.modify(|_, w| w.tim6en().enabled());

    (*tim2_ptr).cr1.modify(|_, w| {
      w.opm().enabled();
      w.cen().disabled()
    });
    
  
    (*tim2_ptr).psc.write(|w| w.psc().bits(7999));
    (*tim2_ptr).arr.write(|w| w.arr().bits(ms));
    (*tim2_ptr).cr1.modify(|_, w| w.cen().enabled());
  
    while !(*tim2_ptr).sr.read().uif().bit_is_set() {}
  
    (*tim2_ptr).sr.modify(|_, w| (w.uif().clear_bit()));
  }
}

pub fn pin_mode(block: &str, pin: u8, mode: bool) {
  let rcc_ptr = stm32f4::stm32f446::RCC::ptr();
  let gpioa_ptr = stm32f4::stm32f446::GPIOA::ptr();
  let gpiob_ptr = stm32f4::stm32f446::GPIOB::ptr();
  let gpioc_ptr = stm32f4::stm32f446::GPIOC::ptr();

  unsafe {
    match block {
      "a" => (*rcc_ptr).ahb1enr.modify(|_, w| w.gpioaen().enabled()),
      "b" => (*rcc_ptr).ahb1enr.modify(|_, w| w.gpioben().enabled()),
      "c" => (*rcc_ptr).ahb1enr.modify(|_, w| w.gpiocen().enabled()),
      _   => return,
    };
  
    if mode == true {
      match block {
        "a" => (*gpioa_ptr).moder.modify(|r, w| w.bits(r.bits() & (!(3 << (pin * 2))) | (1 << (pin * 2)))),
        "b" => (*gpiob_ptr).moder.modify(|r, w| w.bits(r.bits() & (!(3 << (pin * 2))) | (1 << (pin * 2)))),
        "c" => (*gpioc_ptr).moder.modify(|r, w| w.bits(r.bits() & (!(3 << (pin * 2))) | (1 << (pin * 2)))),
        _   => return,
      }
    } else {
      match block {
        "a" => (*gpioa_ptr).moder.modify(|r, w| w.bits(r.bits() & (!(3 << (pin * 2))))),
        "b" => (*gpiob_ptr).moder.modify(|r, w| w.bits(r.bits() & (!(3 << (pin * 2))))),
        "c" => (*gpioc_ptr).moder.modify(|r, w| w.bits(r.bits() & (!(3 << (pin * 2))))),
        _   => return,
      }
    }
  }
}

pub fn pin_write(block: &str, pin: u8, write: bool) {
  let gpioa_ptr = stm32f4::stm32f446::GPIOA::ptr();
  let gpiob_ptr = stm32f4::stm32f446::GPIOB::ptr();
  let gpioc_ptr = stm32f4::stm32f446::GPIOC::ptr();

  unsafe {
    if write == true {
      match block {
        "a" => (*gpioa_ptr).bsrr.write(|w| w.bits(1 << pin)),
        "b" => (*gpiob_ptr).bsrr.write(|w| w.bits(1 << pin)),
        "c" => (*gpioc_ptr).bsrr.write(|w| w.bits(1 << pin)),
        _   => return,
      }
    } else {
      match block {
        "a" => (*gpioa_ptr).bsrr.write(|w| w.bits(1 << pin + 16)),
        "b" => (*gpiob_ptr).bsrr.write(|w| w.bits(1 << pin + 16)),
        "c" => (*gpioc_ptr).bsrr.write(|w| w.bits(1 << pin + 16)),
        _   => return,
      }
    }
  }
}

pub fn pin_read(block: &str, pin: u8) -> bool {
  let gpioa_ptr = stm32f4::stm32f446::GPIOA::ptr();
  let gpiob_ptr = stm32f4::stm32f446::GPIOB::ptr();
  let gpioc_ptr = stm32f4::stm32f446::GPIOC::ptr();
  let state: bool;

  unsafe {
    let bits = match block {
      "a" => (*gpioa_ptr).idr.read().bits(),
      "b" => (*gpiob_ptr).idr.read().bits(),
      "c" => (*gpioc_ptr).idr.read().bits(),
      _   => return false,
    };
  
    if bits & (1 << pin) == (2 << pin) {
      state = true;
    } else {
      state = false;
    }
  
    return state;
  }
}

pub fn uart_init(baud: u32) {
  let rcc_ptr = stm32f4::stm32f446::RCC::ptr();
  let usart2_ptr = stm32f4::stm32f446::USART2::ptr();
  let gpioa_ptr = stm32f4::stm32f446::GPIOA::ptr();

  let psc = match baud {
    9600 => (104, 3),
    115200 => (8, 11),
    _ => (8, 11)
  };

  unsafe {
    (*rcc_ptr).apb1enr.modify(|_, w| w.usart2en().enabled());
    (*rcc_ptr).ahb1enr.modify(|_, w| w.gpioaen().enabled());
  
    (*gpioa_ptr).moder.modify(|_, w| {
      w.moder2().alternate();
      w.moder3().alternate()
    });
    (*gpioa_ptr).afrl.modify(|_, w| {
      w.afrl2().bits(7);
      w.afrl3().bits(7)
    });
  
    (*usart2_ptr).cr1.modify(|_, w| {
      w.ue().enabled();
      w.te().enabled();
      w.re().enabled()
    });
  
    (*usart2_ptr).brr.modify(|_, w| {
      w.div_mantissa().bits(psc.0);
      w.div_fraction().bits(psc.1)
    });
  }
}

pub fn sprint(data: &str){
  let usart2_ptr = stm32f4::stm32f446::USART2::ptr();

  unsafe {
    for c in data.chars() {
      (*usart2_ptr).dr.write(|w| w.dr().bits(c as u16));
      while (*usart2_ptr).sr.read().tc().bit_is_clear() {}
    }
  }
}

pub fn sprintln(data: &str){
  let usart2_ptr = stm32f4::stm32f446::USART2::ptr();

  unsafe {
    for c in data.chars() {
      (*usart2_ptr).dr.write(|w| w.dr().bits(c as u16));
      while (*usart2_ptr).sr.read().tc().bit_is_clear() {}
    }
    
    (*usart2_ptr).dr.write(|w| w.dr().bits('\r' as u16));
    while (*usart2_ptr).sr.read().tc().bit_is_clear() {}
    (*usart2_ptr).dr.write(|w| w.dr().bits('\n' as u16));
    while (*usart2_ptr).sr.read().tc().bit_is_clear() {}
  }
}
