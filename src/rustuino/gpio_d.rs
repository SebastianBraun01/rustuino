use super::Mode;
use super::include::{RCC_PTR, GPIOA_PTR, GPIOB_PTR, GPIOC_PTR};

pub fn pin_mode(pin: (u8, char), mode: Mode) {  
  if pin.0 > 15 {
    panic!("P{}{} is not an available GPIO Pin", pin.1.to_uppercase(), pin.0);
  }
  
  unsafe {
    match pin.1 {
      'a' => {
        (*RCC_PTR).ahb1enr.modify(|_, w| w.gpioaen().enabled());

        match mode {
          Mode::Input => (*GPIOA_PTR).moder.modify(|r, w| w.bits(r.bits() & (!(3 << (pin.0 * 2))))),

          Mode::Output => (*GPIOA_PTR).moder.modify(|r, w| w.bits(r.bits() & (!(3 << (pin.0 * 2))) | (1 << (pin.0 * 2)))),

          Mode::AlterateFunction(func) => {
            (*GPIOA_PTR).moder.modify(|r, w| w.bits(r.bits() & (!(3 << (pin.0 * 2))) | (2 << (pin.0 * 2))));
            if pin.0 < 8 {
              (*GPIOA_PTR).afrl.modify(|r, w| w.bits(r.bits() & (!(0xF << (pin.0 * 4))) | (func << (pin.0 * 4))));
            } else {
              (*GPIOA_PTR).afrh.modify(|r, w| w.bits(r.bits() & (!(0xF << (pin.0 * 4))) | (func << (pin.0 * 4))));
            }
          },

          Mode::Analog => (*GPIOA_PTR).moder.modify(|r, w| w.bits(r.bits() & (!(3 << (pin.0 * 2))) | (3 << (pin.0 * 2))))
        }
      },
      'b' => {
        (*RCC_PTR).ahb1enr.modify(|_, w| w.gpioben().enabled());

        match mode {
          Mode::Input => (*GPIOB_PTR).moder.modify(|r, w| w.bits(r.bits() & (!(3 << (pin.0 * 2))))),

          Mode::Output => (*GPIOB_PTR).moder.modify(|r, w| w.bits(r.bits() & (!(3 << (pin.0 * 2))) | (1 << (pin.0 * 2)))),

          Mode::AlterateFunction(func) => {
            (*GPIOB_PTR).moder.modify(|r, w| w.bits(r.bits() & (!(3 << (pin.0 * 2))) | (2 << (pin.0 * 2))));
            if pin.0 < 8 {
              (*GPIOB_PTR).afrl.modify(|r, w| w.bits(r.bits() & (!(0xF << (pin.0 * 4))) | (func << (pin.0 * 4))));
            } else {
              (*GPIOB_PTR).afrh.modify(|r, w| w.bits(r.bits() & (!(0xF << (pin.0 * 4))) | (func << (pin.0 * 4))));
            }
          },
          
          Mode::Analog => (*GPIOB_PTR).moder.modify(|r, w| w.bits(r.bits() & (!(3 << (pin.0 * 2))) | (3 << (pin.0 * 2))))
        }
      },
      'c' => {
        (*RCC_PTR).ahb1enr.modify(|_, w| w.gpiocen().enabled());

        match mode {
          Mode::Input => (*GPIOC_PTR).moder.modify(|r, w| w.bits(r.bits() & (!(3 << (pin.0 * 2))))),

          Mode::Output => (*GPIOC_PTR).moder.modify(|r, w| w.bits(r.bits() & (!(3 << (pin.0 * 2))) | (1 << (pin.0 * 2)))),

          Mode::AlterateFunction(func) => {
            (*GPIOC_PTR).moder.modify(|r, w| w.bits(r.bits() & (!(3 << (pin.0 * 2))) | (2 << (pin.0 * 2))));
            if pin.0 < 8 {
              (*GPIOC_PTR).afrl.modify(|r, w| w.bits(r.bits() & (!(0xF << (pin.0 * 4))) | (func << (pin.0 * 4))));
            } else {
              (*GPIOC_PTR).afrh.modify(|r, w| w.bits(r.bits() & (!(0xF << (pin.0 * 4))) | (func << (pin.0 * 4))));
            }
          },
          
          Mode::Analog => (*GPIOC_PTR).moder.modify(|r, w| w.bits(r.bits() & (!(3 << (pin.0 * 2))) | (3 << (pin.0 * 2))))
        }
      },
      _   => panic!("P{}{} is not an available GPIO Pin", pin.1.to_uppercase(), pin.0)
    };
  }
}

pub fn pin_write(pin: (u8, char), write: bool) {  
  if pin.0 > 15 {
    panic!("P{}{} is not an available GPIO Pin", pin.1.to_uppercase(), pin.0);
  }

  unsafe {
    match pin.1 {
      'a' => {
        if write == true {(*GPIOA_PTR).bsrr.write(|w| w.bits(1 << pin.0));}
        else {(*GPIOA_PTR).bsrr.write(|w| w.bits(1 << (pin.0 + 16)));}
      },
      'b' => {
        if write == true {(*GPIOB_PTR).bsrr.write(|w| w.bits(1 << pin.0));}
        else {(*GPIOB_PTR).bsrr.write(|w| w.bits(1 << (pin.0 + 16)));}
      },
      'c' => {
        if write == true {(*GPIOC_PTR).bsrr.write(|w| w.bits(1 << pin.0));}
        else {(*GPIOC_PTR).bsrr.write(|w| w.bits(1 << (pin.0 + 16)));}
      },
      _   => panic!("P{}{} is not an available GPIO Pin", pin.1.to_uppercase(), pin.0)
    };
  }
}

pub fn pin_read(pin: (u8, char)) -> bool {  
  if pin.0 > 15 {
    panic!("P{}{} is not an available GPIO Pin", pin.1.to_uppercase(), pin.0);
  }

  unsafe {
    if (*GPIOA_PTR).moder.read().bits() & (3 << (2 * pin.0)) == 0 {
      let bits = match pin.1 {
        'a' => (*GPIOA_PTR).idr.read().bits(),
        'b' => (*GPIOB_PTR).idr.read().bits(),
        'c' => (*GPIOC_PTR).idr.read().bits(),
        _   => panic!("P{}{} is not an available GPIO Pin", pin.1.to_uppercase(), pin.0)
      };

      if bits & (1 << pin.0) == (1 << pin.0) {return true;}
      else {return false;}
    }
    else if (*GPIOA_PTR).moder.read().bits() & (3 << (2 * pin.0)) == (1 << (2 * pin.0)) {
      let bits = match pin.1 {
        'a' => (*GPIOA_PTR).odr.read().bits(),
        'b' => (*GPIOB_PTR).odr.read().bits(),
        'c' => (*GPIOC_PTR).odr.read().bits(),
        _   => panic!("P{}{} is not an available GPIO Pin", pin.1.to_uppercase(), pin.0)
      };

      if bits & (1 << pin.0) == (1 << pin.0) {return true;}
      else {return false;}
    }
    else {panic!("Cannot read GPIO Pin in this Configuration!");}
  }
}
