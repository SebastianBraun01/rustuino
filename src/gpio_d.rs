use crate::PERIPHERAL_PTR;
use super::include::data_maps::PINCONFIG;

#[derive(Debug)]
pub enum Mode {
  Input, Output, AlterateFunction(Fn), Analog(Dir)  
}

#[derive(Debug)]
pub enum Fn {
  None, PWM, UART, Timer
}

#[derive(Debug)]
pub enum Dir {
  None, Input, Output
}

pub enum Speed {
  Low, Medium, Fast, High
}

pub enum Bias {
  None, Pullup, Pulldown
}

pub fn pin_mode(pin: (u8, char), mode: Mode) {
  let rcc = &PERIPHERAL_PTR.RCC;
  let gpioa = &PERIPHERAL_PTR.GPIOA;
  let gpiob = &PERIPHERAL_PTR.GPIOB;
  let gpioc = &PERIPHERAL_PTR.GPIOC;

  if pin.0 > 15 {panic!("P{}{} is not an available GPIO Pin", pin.1.to_uppercase(), pin.0);}

  unsafe {
    if PINCONFIG.pin.contains(&pin) == true {panic!("P{}{} is already configured!", pin.1.to_uppercase(), pin.0);}
    PINCONFIG.pin.push(pin).expect("Could not store pin configuration!");
    PINCONFIG.mode.push(mode).expect("Could not store pin configuration!");
  }
  
  match pin.1 {
    'a' => {
      rcc.ahb1enr.modify(|_, w| w.gpioaen().enabled());

      match mode {
        Mode::Input => gpioa.moder.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin.0)))}),
        Mode::Output => gpioa.moder.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin.0)) | (1 << (2 * pin.0)))}),
        Mode::AlterateFunction(func) => {
          gpioa.moder.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin.0)) | (2 << (2 * pin.0)))});
          if pin.0 < 8 {gpioa.afrl.modify(|r, w| unsafe {w.bits(r.bits() & !(0xF << (4 * pin.0)) | (func << (4 * pin.0)))});}
          else {gpioa.afrh.modify(|r, w| unsafe {w.bits(r.bits() & !(0xF << (4 * pin.0)) | (func << (4 * pin.0)))});}
        },
        Mode::Analog(_) => gpioa.moder.modify(|r, w| unsafe {w.bits(r.bits() | (3 << (2 * pin.0)))})
      };
    },
    'b' => {
      rcc.ahb1enr.modify(|_, w| w.gpioben().enabled());

      match mode {
        Mode::Input => gpiob.moder.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin.0)))}),
        Mode::Output => gpiob.moder.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin.0)) | (1 << (2 * pin.0)))}),
        Mode::AlterateFunction(func) => {
          gpiob.moder.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin.0)) | (2 << (2 * pin.0)))});
          if pin.0 < 8 {gpiob.afrl.modify(|r, w| unsafe {w.bits(r.bits() & !(0xF << (4 * pin.0)) | (func << (4 * pin.0)))});}
          else {gpiob.afrh.modify(|r, w| unsafe {w.bits(r.bits() & !(0xF << (4 * pin.0)) | (func << (4 * pin.0)))});}
        },
        Mode::Analog(_) => gpiob.moder.modify(|r, w| unsafe {w.bits(r.bits() | (3 << (2 * pin.0)))})
      }
    },
    'c' => {
      rcc.ahb1enr.modify(|_, w| w.gpiocen().enabled());

      match mode {
        Mode::Input => gpioc.moder.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin.0)))}),
        Mode::Output => gpioc.moder.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin.0)) | (1 << (2 * pin.0)))}),
        Mode::AlterateFunction(func) => {
          gpioc.moder.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin.0)) | (2 << (2 * pin.0)))});
          if pin.0 < 8 {gpioc.afrl.modify(|r, w| unsafe {w.bits(r.bits() & !(0xF << (4 * pin.0)) | (func << (4 * pin.0)))});}
          else {gpioc.afrh.modify(|r, w| unsafe {w.bits(r.bits() & !(0xF << (4 * pin.0)) | (func << (4 * pin.0)))});}
        },
        Mode::Analog(_) => gpioc.moder.modify(|r, w| unsafe {w.bits(r.bits() | (3 << (2 * pin.0)))})
      }
    },
    _   => panic!("P{}{} is not an available GPIO Pin", pin.1.to_uppercase(), pin.0)
  };
}

pub fn pin_config(pin: (u8, char), open_drain: bool, speed: Speed, bias: Bias) {
  let gpioa = &PERIPHERAL_PTR.GPIOA;
  let gpiob = &PERIPHERAL_PTR.GPIOB;
  let gpioc = &PERIPHERAL_PTR.GPIOC;

  if pin.0 > 15 {panic!("P{}{} is not an available GPIO Pin", pin.1.to_uppercase(), pin.0);}

  match pin.1 {
    'a' => { 
      if open_drain == true {gpioa.otyper.modify(|r, w| unsafe {w.bits(r.bits() | (1 << pin.0))});}
      else {gpioa.otyper.modify(|r, w| unsafe {w.bits(r.bits() & !(1 << pin.0))});}

      match speed {
        Speed::Low => gpioa.ospeedr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin.0)))}),
        Speed::Medium => gpioa.ospeedr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin.0)) | (1 << (2 * pin.0)))}),
        Speed::Fast => gpioa.ospeedr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin.0)) | (2 << (2 * pin.0)))}),
        Speed::High => gpioa.ospeedr.modify(|r, w| unsafe {w.bits(r.bits() | (3 << (2 * pin.0)))})
      };

      match bias {
        Bias::None => gpioa.pupdr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin.0)))}),
        Bias::Pullup => gpioa.pupdr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin.0)) | (1 << (2 * pin.0)))}),
        Bias::Pulldown => gpioa.pupdr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin.0)) | (2 << (2 * pin.0)))})
      };
    },
    'b' => {   
      if open_drain == true {gpiob.otyper.modify(|r, w| unsafe {w.bits(r.bits() | (1 << pin.0))});}
      else {gpiob.otyper.modify(|r, w| unsafe {w.bits(r.bits() & !(1 << pin.0))});}

      match speed {
        Speed::Low => gpiob.ospeedr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin.0)))}),
        Speed::Medium => gpiob.ospeedr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin.0)) | (1 << (2 * pin.0)))}),
        Speed::Fast => gpiob.ospeedr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin.0)) | (2 << (2 * pin.0)))}),
        Speed::High => gpiob.ospeedr.modify(|r, w| unsafe {w.bits(r.bits() | (3 << (2 * pin.0)))})
      };

      match bias {
        Bias::None => gpiob.pupdr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin.0)))}),
        Bias::Pullup => gpiob.pupdr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin.0)) | (1 << (2 * pin.0)))}),
        Bias::Pulldown => gpiob.pupdr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin.0)) | (2 << (2 * pin.0)))})
      };
    },
    'c' => {  
      if open_drain == true {gpioc.otyper.modify(|r, w| unsafe {w.bits(r.bits() | (1 << pin.0))});}
      else {gpioc.otyper.modify(|r, w| unsafe {w.bits(r.bits() & !(1 << pin.0))});}

      match speed {
        Speed::Low => gpioc.ospeedr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin.0)))}),
        Speed::Medium => gpioc.ospeedr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin.0)) | (1 << (2 * pin.0)))}),
        Speed::Fast => gpioc.ospeedr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin.0)) | (2 << (2 * pin.0)))}),
        Speed::High => gpioc.ospeedr.modify(|r, w| unsafe {w.bits(r.bits() | (3 << (2 * pin.0)))})
      };

      match bias {
        Bias::None => gpioc.pupdr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin.0)))}),
        Bias::Pullup => gpioc.pupdr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin.0)) | (1 << (2 * pin.0)))}),
        Bias::Pulldown => gpioc.pupdr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin.0)) | (2 << (2 * pin.0)))})
      };
    },
    _   => panic!("P{}{} is not an available GPIO Pin", pin.1.to_uppercase(), pin.0)
  };
}

pub fn pin_write(pin: (u8, char), write: bool) {
  let gpioa = &PERIPHERAL_PTR.GPIOA;
  let gpiob = &PERIPHERAL_PTR.GPIOB;
  let gpioc = &PERIPHERAL_PTR.GPIOC;

  if pin.0 > 15 {panic!("P{}{} is not an available GPIO Pin", pin.1.to_uppercase(), pin.0);}

  match pin.1 {
    'a' => {       
      if write == true {gpioa.bsrr.write(|w| unsafe {w.bits(1 << pin.0)});}
      else {gpioa.bsrr.write(|w| unsafe {w.bits(1 << (pin.0 + 16))});}
    },
    'b' => {
      if write == true {gpiob.bsrr.write(|w| unsafe {w.bits(1 << pin.0)});}
      else {gpiob.bsrr.write(|w| unsafe {w.bits(1 << (pin.0 + 16))});}
    },
    'c' => {
      if write == true {gpioc.bsrr.write(|w| unsafe {w.bits(1 << pin.0)});}
      else {gpioc.bsrr.write(|w| unsafe {w.bits(1 << (pin.0 + 16))});}
    },
      _   => panic!("P{}{} is not an available GPIO Pin", pin.1.to_uppercase(), pin.0)
  };
}

pub fn pin_read(pin: (u8, char)) -> bool {
  let gpioa = &PERIPHERAL_PTR.GPIOA;
  let gpiob = &PERIPHERAL_PTR.GPIOB;
  let gpioc = &PERIPHERAL_PTR.GPIOC;
  let bits: u32;

  if pin.0 > 15 {panic!("P{}{} is not an available GPIO Pin", pin.1.to_uppercase(), pin.0);}

  unsafe {
    if PINCONFIG.pin.contains(&pin) {
      if PINCONFIG.config[PINCONFIG.pin.iter().position(|&i| i == pin).unwrap()] == 0 {
        bits = match pin.1 {
          'a' => gpioa.idr.read().bits(),
          'b' => gpiob.idr.read().bits(),
          'c' => gpioc.idr.read().bits(),
          _   => panic!("P{}{} is not an available GPIO Pin", pin.1.to_uppercase(), pin.0)
        };
      }
      else if PINCONFIG.config[PINCONFIG.pin.iter().position(|&i| i == pin).unwrap()] == 1 {
        bits = match pin.1 {
          'a' => gpioa.odr.read().bits(),
          'b' => gpiob.odr.read().bits(),
          'c' => gpioc.odr.read().bits(),
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
