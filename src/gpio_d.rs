use super::include::CONFIG;
use super::include::{RCC_PTR, GPIOA_PTR, GPIOB_PTR, GPIOC_PTR};

pub enum Mode {
  Input,
  Output,
  AlterateFunction(u32),
  // false: ADC, true: DAC
  Analog(bool)
}

pub enum Speed {
  Low,
  Medium,
  Fast,
  High
}

pub enum Bias {
  None,
  Pullup,
  Pulldown
}

pub fn pin_mode(pin: (u8, char), mode: Mode) {  
  if pin.0 > 15 {panic!("P{}{} is not an available GPIO Pin", pin.1.to_uppercase(), pin.0);}
  
  unsafe {
    if CONFIG.pin.contains(&pin) == true {panic!("P{}{} is already configured!", pin.1.to_uppercase(), pin.0);}

    match pin.1 {
      'a' => {
        (*RCC_PTR).ahb1enr.modify(|_, w| w.gpioaen().enabled());

        match mode {
          Mode::Input => (*GPIOA_PTR).moder.modify(|r, w| w.bits(r.bits() & !(3 << (2 * pin.0)))),

          Mode::Output => (*GPIOA_PTR).moder.modify(|r, w| w.bits(r.bits() & !(3 << (2 * pin.0)) | (1 << (2 * pin.0)))),

          Mode::AlterateFunction(func) => {
            (*GPIOA_PTR).moder.modify(|r, w| w.bits(r.bits() & !(3 << (2 * pin.0)) | (2 << (2 * pin.0))));
            if pin.0 < 8 {
              (*GPIOA_PTR).afrl.modify(|r, w| w.bits(r.bits() & !(0xF << (4 * pin.0)) | (func << (4 * pin.0))));
            } else {
              (*GPIOA_PTR).afrh.modify(|r, w| w.bits(r.bits() & !(0xF << (4 * pin.0)) | (func << (4 * pin.0))));
            }
          },

          Mode::Analog(_) => (*GPIOA_PTR).moder.modify(|r, w| w.bits(r.bits() | (3 << (2 * pin.0))))
        }
      },
      'b' => {
        (*RCC_PTR).ahb1enr.modify(|_, w| w.gpioben().enabled());

        match mode {
          Mode::Input => (*GPIOB_PTR).moder.modify(|r, w| w.bits(r.bits() & !(3 << (2 * pin.0)))),

          Mode::Output => (*GPIOB_PTR).moder.modify(|r, w| w.bits(r.bits() & !(3 << (2 * pin.0)) | (1 << (2 * pin.0)))),

          Mode::AlterateFunction(func) => {
            (*GPIOB_PTR).moder.modify(|r, w| w.bits(r.bits() & !(3 << (2 * pin.0)) | (2 << (2 * pin.0))));
            if pin.0 < 8 {
              (*GPIOB_PTR).afrl.modify(|r, w| w.bits(r.bits() & !(0xF << (4 * pin.0)) | (func << (4 * pin.0))));
            } else {
              (*GPIOB_PTR).afrh.modify(|r, w| w.bits(r.bits() & !(0xF << (4 * pin.0)) | (func << (4 * pin.0))));
            }
          },

          Mode::Analog(_) => (*GPIOB_PTR).moder.modify(|r, w| w.bits(r.bits() | (3 << (2 * pin.0))))
        }
      },
      'c' => {
        (*RCC_PTR).ahb1enr.modify(|_, w| w.gpiocen().enabled());

        match mode {
          Mode::Input => (*GPIOC_PTR).moder.modify(|r, w| w.bits(r.bits() & !(3 << (2 * pin.0)))),

          Mode::Output => (*GPIOC_PTR).moder.modify(|r, w| w.bits(r.bits() & !(3 << (2 * pin.0)) | (1 << (2 * pin.0)))),

          Mode::AlterateFunction(func) => {
            (*GPIOC_PTR).moder.modify(|r, w| w.bits(r.bits() & !(3 << (2 * pin.0)) | (2 << (2 * pin.0))));
            if pin.0 < 8 {
              (*GPIOC_PTR).afrl.modify(|r, w| w.bits(r.bits() & !(0xF << (4 * pin.0)) | (func << (4 * pin.0))));
            } else {
              (*GPIOC_PTR).afrh.modify(|r, w| w.bits(r.bits() & !(0xF << (4 * pin.0)) | (func << (4 * pin.0))));
            }
          },

          Mode::Analog(_) => (*GPIOC_PTR).moder.modify(|r, w| w.bits(r.bits() | (3 << (2 * pin.0))))
        }
      },
      _   => panic!("P{}{} is not an available GPIO Pin", pin.1.to_uppercase(), pin.0)
    };

    CONFIG.pin.push(pin).expect("Could not store pin configuration!");
    match mode {
      Mode::Input => {
        CONFIG.config.push(0).expect("Could not store pin configuration!");
        CONFIG.alternate.push(16).expect("Could not store pin configuration!");
        CONFIG.analog.push(0).expect("Could not store pin configuration!");
      },
      Mode::Output => {
        CONFIG.config.push(1).expect("Could not store pin configuration!");
        CONFIG.alternate.push(16).expect("Could not store pin configuration!");
        CONFIG.analog.push(0).expect("Could not store pin configuration!");
      },
      Mode::AlterateFunction(f) => {
        CONFIG.config.push(2).expect("Could not store pin configuration!");
        CONFIG.alternate.push(f).expect("Could not store pin configuration!");
        CONFIG.analog.push(0).expect("Could not store pin configuration!");
      },
      Mode::Analog(f) => {
        CONFIG.config.push(3).expect("Could not store pin configuration!");
        CONFIG.alternate.push(16).expect("Could not store pin configuration!");
        if f == false {CONFIG.analog.push(1).expect("Could not store pin configuration!");}
        else {CONFIG.analog.push(2).expect("Could not store pin configuration!");}
      }
    };
  }
}

pub fn pin_config(pin: (u8, char), open_drain: bool, speed: Speed, bias: Bias) {
  if pin.0 > 15 {panic!("P{}{} is not an available GPIO Pin", pin.1.to_uppercase(), pin.0);}

  unsafe {
    match pin.1 {
      'a' => {
        if open_drain == true {(*GPIOA_PTR).otyper.modify(|r, w| w.bits(r.bits() | (1 << pin.0)));}
        else {(*GPIOA_PTR).otyper.modify(|r, w| w.bits(r.bits() & !(1 << pin.0)));}

        match speed {
          Speed::Low => {(*GPIOA_PTR).ospeedr.modify(|r, w| w.bits(r.bits() & !(3 << (2 * pin.0))));},
          Speed::Medium => {(*GPIOA_PTR).ospeedr.modify(|r, w| w.bits(r.bits() & !(3 << (2 * pin.0)) | (1 << (2 * pin.0))));},
          Speed::Fast => {(*GPIOA_PTR).ospeedr.modify(|r, w| w.bits(r.bits() & !(3 << (2 * pin.0)) | (2 << (2 * pin.0))));},
          Speed::High => {(*GPIOA_PTR).ospeedr.modify(|r, w| w.bits(r.bits() | (3 << (2 * pin.0))));}
        };

        match bias {
          Bias::None => {(*GPIOA_PTR).pupdr.modify(|r, w| w.bits(r.bits() & !(3 << (2 * pin.0))));},
          Bias::Pullup => {(*GPIOA_PTR).pupdr.modify(|r, w| w.bits(r.bits() & !(3 << (2 * pin.0)) | (1 << (2 * pin.0))));},
          Bias::Pulldown => {(*GPIOA_PTR).pupdr.modify(|r, w| w.bits(r.bits() & !(3 << (2 * pin.0)) | (2 << (2 * pin.0))));}
        };
      },
      'b' => {
        if open_drain == true {(*GPIOB_PTR).otyper.modify(|r, w| w.bits(r.bits() | (1 << pin.0)));}
        else {(*GPIOB_PTR).otyper.modify(|r, w| w.bits(r.bits() & !(1 << pin.0)));}

        match speed {
          Speed::Low => {(*GPIOB_PTR).ospeedr.modify(|r, w| w.bits(r.bits() & !(3 << (2 * pin.0))));},
          Speed::Medium => {(*GPIOB_PTR).ospeedr.modify(|r, w| w.bits(r.bits() & !(3 << (2 * pin.0)) | (1 << (2 * pin.0))));},
          Speed::Fast => {(*GPIOB_PTR).ospeedr.modify(|r, w| w.bits(r.bits() & !(3 << (2 * pin.0)) | (2 << (2 * pin.0))));},
          Speed::High => {(*GPIOB_PTR).ospeedr.modify(|r, w| w.bits(r.bits() | (3 << (2 * pin.0))));}
        };

        match bias {
          Bias::None => {(*GPIOB_PTR).pupdr.modify(|r, w| w.bits(r.bits() & !(3 << (2 * pin.0))));},
          Bias::Pullup => {(*GPIOB_PTR).pupdr.modify(|r, w| w.bits(r.bits() & !(3 << (2 * pin.0)) | (1 << (2 * pin.0))));},
          Bias::Pulldown => {(*GPIOB_PTR).pupdr.modify(|r, w| w.bits(r.bits() & !(3 << (2 * pin.0)) | (2 << (2 * pin.0))));}
        };
      },
      'c' => {
        if open_drain == true {(*GPIOC_PTR).otyper.modify(|r, w| w.bits(r.bits() | (1 << pin.0)));}
        else {(*GPIOC_PTR).otyper.modify(|r, w| w.bits(r.bits() & !(1 << pin.0)));}

        match speed {
          Speed::Low => {(*GPIOC_PTR).ospeedr.modify(|r, w| w.bits(r.bits() & !(3 << (2 * pin.0))));},
          Speed::Medium => {(*GPIOC_PTR).ospeedr.modify(|r, w| w.bits(r.bits() & !(3 << (2 * pin.0)) | (1 << (2 * pin.0))));},
          Speed::Fast => {(*GPIOC_PTR).ospeedr.modify(|r, w| w.bits(r.bits() & !(3 << (2 * pin.0)) | (2 << (2 * pin.0))));},
          Speed::High => {(*GPIOC_PTR).ospeedr.modify(|r, w| w.bits(r.bits() | (3 << (2 * pin.0))));}
        };

        match bias {
          Bias::None => {(*GPIOC_PTR).pupdr.modify(|r, w| w.bits(r.bits() & !(3 << (2 * pin.0))));},
          Bias::Pullup => {(*GPIOC_PTR).pupdr.modify(|r, w| w.bits(r.bits() & !(3 << (2 * pin.0)) | (1 << (2 * pin.0))));},
          Bias::Pulldown => {(*GPIOC_PTR).pupdr.modify(|r, w| w.bits(r.bits() & !(3 << (2 * pin.0)) | (2 << (2 * pin.0))));}
        };
      },
      _   => panic!("P{}{} is not an available GPIO Pin", pin.1.to_uppercase(), pin.0)
    };
  }
}

pub fn pin_write(pin: (u8, char), write: bool) {  
  if pin.0 > 15 {panic!("P{}{} is not an available GPIO Pin", pin.1.to_uppercase(), pin.0);}

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
  if pin.0 > 15 {panic!("P{}{} is not an available GPIO Pin", pin.1.to_uppercase(), pin.0);}

  let bits: u32;

  unsafe {
    if CONFIG.pin.contains(&pin) {
      if CONFIG.config[CONFIG.pin.iter().position(|&i| i == pin).unwrap()] == 0 {
        bits = match pin.1 {
          'a' => (*GPIOA_PTR).idr.read().bits(),
          'b' => (*GPIOB_PTR).idr.read().bits(),
          'c' => (*GPIOC_PTR).idr.read().bits(),
          _   => panic!("P{}{} is not an available GPIO Pin", pin.1.to_uppercase(), pin.0)
        };
      }
      else if CONFIG.config[CONFIG.pin.iter().position(|&i| i == pin).unwrap()] == 1 {
        bits = match pin.1 {
          'a' => (*GPIOA_PTR).odr.read().bits(),
          'b' => (*GPIOB_PTR).odr.read().bits(),
          'c' => (*GPIOC_PTR).odr.read().bits(),
          _   => panic!("P{}{} is not an available GPIO Pin", pin.1.to_uppercase(), pin.0)
        };
      }
      else {panic!("P{}{} is no readable in the configuration!", pin.1.to_uppercase(), pin.0);}
    }
    else {panic!("P{}{} is not configured!", pin.1.to_uppercase(), pin.0);}
  }

  if bits & (1 << pin.0) == (1 << pin.0) {return true;}
  else {return false;}
}
