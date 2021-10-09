//! This module contains everything that is related to the digital IO functionality.

use crate::analog::enable_channel;
use crate::time::setup_pwm;
use crate::include::{ProgError, PIN_CONF};
use rtt_target::rprintln;

pub struct Pin<T> {
  pub block: char,
  pub number: u8,
  #[doc(hidden)]
  pub inner: T
}

pub struct Input;
pub struct Output;
pub struct AlternateFunction(u32);
pub struct AnalogIn {
  #[doc(hidden)]
  pub core: u8,
  #[doc(hidden)]
  pub channel: u8
}
pub struct AnalogOut {
  #[doc(hidden)]
  pub core: u8,
  #[doc(hidden)]
  pub channel: u8
}
pub struct PWM {
  #[doc(hidden)]
  pub timer: u8,
  #[doc(hidden)]
  pub ccch: u8
}

/// Represents the options to configure the GPIO speed of a pin.
///
/// Keep in mind, that the internal clock of the MC needs to be faster than the IO speed to be used fully.
///
/// | Speed  | Max. IO Frequency |
/// | ------ | ----------------- |
/// | Low    | 4MHz              |
/// | Medium | 25MHz             |
/// | Fast   | 50MHz             |
/// | High   | 100MHz            |
pub enum GpioSpeed {
  Low, Medium, Fast, High
}

// Represents the options for pullup-/pulldown-resistors.
pub enum GpioBias {
  None, Pullup, Pulldown
}


// Public Functions ===============================================================================
pub fn pinmode_input(pin: (char, u8)) -> Result<Pin<Input>, ProgError> {
  let peripheral_ptr;
  unsafe {peripheral_ptr = stm32f4::stm32f446::Peripherals::steal();} 
  let rcc = &peripheral_ptr.RCC;

  if let Err(error) = check_pin(pin) {return Err(error);}

  unsafe {
    if !PIN_CONF.contains(&pin) {PIN_CONF.push(pin).unwrap();}
    else {
      rprintln!("P{}{} is already configured! | pin_mode()", pin.0.to_uppercase(), pin.1);
      return Err(ProgError::AlreadyConfigured);
    }
  }

  match pin.0 {
    'a' => {
      let gpioa = &peripheral_ptr.GPIOA;
      rcc.ahb1enr.modify(|_, w| w.gpioaen().enabled());
      gpioa.moder.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin.1)))});
    },
    'b' => {
      let gpiob = &peripheral_ptr.GPIOB;
      rcc.ahb1enr.modify(|_, w| w.gpioben().enabled());
      gpiob.moder.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin.1)))});
    },
    'c' => {
      let gpioc = &peripheral_ptr.GPIOC;
      rcc.ahb1enr.modify(|_, w| w.gpiocen().enabled());
      gpioc.moder.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin.1)))});
    },
    'd' => {
      let gpiod = &peripheral_ptr.GPIOD;
      rcc.ahb1enr.modify(|_, w| w.gpioden().enabled());
      gpiod.moder.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin.1)))});
    },
    'h' => {
      let gpioh = &peripheral_ptr.GPIOH;
      rcc.ahb1enr.modify(|_, w| w.gpiohen().enabled());
      gpioh.moder.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin.1)))});
    },
    _   => unreachable!()
  };

  return Ok(Pin {
    block: pin.0,
    number: pin.1,
    inner: Input
  });
}

pub unsafe fn pinmode_input_force(pin: (char, u8)) -> Result<Pin<Input>, ProgError> {
  let peripheral_ptr = stm32f4::stm32f446::Peripherals::steal();
  let rcc = &peripheral_ptr.RCC;

  if let Err(error) = check_pin(pin) {return Err(error);}

  match pin.0 {
    'a' => {
      let gpioa = &peripheral_ptr.GPIOA;
      rcc.ahb1enr.modify(|_, w| w.gpioaen().enabled());
      gpioa.moder.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin.1)))});
    },
    'b' => {
      let gpiob = &peripheral_ptr.GPIOB;
      rcc.ahb1enr.modify(|_, w| w.gpioben().enabled());
      gpiob.moder.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin.1)))});
    },
    'c' => {
      let gpioc = &peripheral_ptr.GPIOC;
      rcc.ahb1enr.modify(|_, w| w.gpiocen().enabled());
      gpioc.moder.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin.1)))});
    },
    'd' => {
      let gpiod = &peripheral_ptr.GPIOD;
      rcc.ahb1enr.modify(|_, w| w.gpioden().enabled());
      gpiod.moder.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin.1)))});
    },
    'h' => {
      let gpioh = &peripheral_ptr.GPIOH;
      rcc.ahb1enr.modify(|_, w| w.gpiohen().enabled());
      gpioh.moder.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin.1)))});
    },
    _   => unreachable!()
  };

  return Ok(Pin {
    block: pin.0,
    number: pin.1,
    inner: Input
  });
}

pub fn pinmode_output(pin: (char, u8)) -> Result<Pin<Output>, ProgError> {
  let peripheral_ptr;
  unsafe {peripheral_ptr = stm32f4::stm32f446::Peripherals::steal();} 
  let rcc = &peripheral_ptr.RCC;

  if let Err(error) = check_pin(pin) {return Err(error);}

  unsafe {
    if !PIN_CONF.contains(&pin) {PIN_CONF.push(pin).unwrap();}
    else {
      rprintln!("P{}{} is already configured! | pin_mode()", pin.0.to_uppercase(), pin.1);
      return Err(ProgError::AlreadyConfigured);
    }
  }

  match pin.0 {
    'a' => {
      let gpioa = &peripheral_ptr.GPIOA;
      rcc.ahb1enr.modify(|_, w| w.gpioaen().enabled());
      gpioa.moder.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin.1)) | (1 << (2 * pin.1)))});
    },
    'b' => {
      let gpiob = &peripheral_ptr.GPIOB;
      rcc.ahb1enr.modify(|_, w| w.gpioben().enabled());
      gpiob.moder.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin.1)) | (1 << (2 * pin.1)))});
    },
    'c' => {
      let gpioc = &peripheral_ptr.GPIOC;
      rcc.ahb1enr.modify(|_, w| w.gpiocen().enabled());
      gpioc.moder.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin.1)) | (1 << (2 * pin.1)))});
    },
    'd' => {
      let gpiod = &peripheral_ptr.GPIOD;
      rcc.ahb1enr.modify(|_, w| w.gpioden().enabled());
      gpiod.moder.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin.1)) | (1 << (2 * pin.1)))});
    },
    'h' => {
      let gpioh = &peripheral_ptr.GPIOH;
      rcc.ahb1enr.modify(|_, w| w.gpiohen().enabled());
      gpioh.moder.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin.1)) | (1 << (2 * pin.1)))});
    },
    _   => unreachable!()
  };

  return Ok(Pin {
    block: pin.0,
    number: pin.1,
    inner: Output
  });
}

pub unsafe fn pinmode_output_force(pin: (char, u8)) -> Result<Pin<Output>, ProgError> {
  let peripheral_ptr = stm32f4::stm32f446::Peripherals::steal();
  let rcc = &peripheral_ptr.RCC;

  if let Err(error) = check_pin(pin) {return Err(error);}

  match pin.0 {
    'a' => {
      let gpioa = &peripheral_ptr.GPIOA;
      rcc.ahb1enr.modify(|_, w| w.gpioaen().enabled());
      gpioa.moder.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin.1)) | (1 << (2 * pin.1)))});
    },
    'b' => {
      let gpiob = &peripheral_ptr.GPIOB;
      rcc.ahb1enr.modify(|_, w| w.gpioben().enabled());
      gpiob.moder.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin.1)) | (1 << (2 * pin.1)))});
    },
    'c' => {
      let gpioc = &peripheral_ptr.GPIOC;
      rcc.ahb1enr.modify(|_, w| w.gpiocen().enabled());
      gpioc.moder.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin.1)) | (1 << (2 * pin.1)))});
    },
    'd' => {
      let gpiod = &peripheral_ptr.GPIOD;
      rcc.ahb1enr.modify(|_, w| w.gpioden().enabled());
      gpiod.moder.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin.1)) | (1 << (2 * pin.1)))});
    },
    'h' => {
      let gpioh = &peripheral_ptr.GPIOH;
      rcc.ahb1enr.modify(|_, w| w.gpiohen().enabled());
      gpioh.moder.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin.1)) | (1 << (2 * pin.1)))});
    },
    _   => unreachable!()
  };

  return Ok(Pin {
    block: pin.0,
    number: pin.1,
    inner: Output
  });
}

pub fn pinmode_alternate_function(pin: (char, u8), af: u32) -> Result<Pin<AlternateFunction>, ProgError> {
  let peripheral_ptr;
  unsafe {peripheral_ptr = stm32f4::stm32f446::Peripherals::steal();} 
  let rcc = &peripheral_ptr.RCC;

  if let Err(error) = check_pin(pin) {return Err(error);}
  if af > 15 {
    rprintln!("Only alternate funtion values between 0 and 15 are valid! | pin_mode()");
    return Err(ProgError::InvalidConfiguration);
  }

  unsafe {
    if !PIN_CONF.contains(&pin) {PIN_CONF.push(pin).unwrap();}
    else {
      rprintln!("P{}{} is already configured! | pin_mode()", pin.0.to_uppercase(), pin.1);
      return Err(ProgError::AlreadyConfigured);
    }
  }

  match pin.0 {
    'a' => {
      let gpioa = &peripheral_ptr.GPIOA;
      rcc.ahb1enr.modify(|_, w| w.gpioaen().enabled());
      gpioa.moder.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin.1)) | (2 << (2 * pin.1)))});
      if pin.1 > 7 {gpioa.afrh.modify(|r, w| unsafe {w.bits(r.bits() | (af << (4 * (pin.1 - 8))))});}
      else {gpioa.afrl.modify(|r, w| unsafe {w.bits(r.bits() | (af << (4 * pin.1)))});}
    },
    'b' => {
      let gpiob = &peripheral_ptr.GPIOB;
      rcc.ahb1enr.modify(|_, w| w.gpioben().enabled());
      gpiob.moder.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin.1)) | (2 << (2 * pin.1)))});
      if pin.1 > 7 {gpiob.afrh.modify(|r, w| unsafe {w.bits(r.bits() | (af << (4 * (pin.1 - 8))))});}
      else {gpiob.afrl.modify(|r, w| unsafe {w.bits(r.bits() | (af << (4 * pin.1)))});}
    },
    'c' => {
      let gpioc = &peripheral_ptr.GPIOC;
      rcc.ahb1enr.modify(|_, w| w.gpiocen().enabled());
      gpioc.moder.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin.1)) | (2 << (2 * pin.1)))});
      if pin.1 > 7 {gpioc.afrh.modify(|r, w| unsafe {w.bits(r.bits() | (af << (4 * (pin.1 - 8))))});}
      else {gpioc.afrl.modify(|r, w| unsafe {w.bits(r.bits() | (af << (4 * pin.1)))});}
    },
    'd' => {
      let gpiod = &peripheral_ptr.GPIOD;
      rcc.ahb1enr.modify(|_, w| w.gpioden().enabled());
      gpiod.moder.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin.1)) | (2 << (2 * pin.1)))});
      if pin.1 > 7 {gpiod.afrh.modify(|r, w| unsafe {w.bits(r.bits() | (af << (4 * (pin.1 - 8))))});}
      else {gpiod.afrl.modify(|r, w| unsafe {w.bits(r.bits() | (af << (4 * pin.1)))});}
    },
    'h' => {
      let gpioh = &peripheral_ptr.GPIOH;
      rcc.ahb1enr.modify(|_, w| w.gpiohen().enabled());
      gpioh.moder.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin.1)) | (2 << (2 * pin.1)))});
      if pin.1 > 7 {gpioh.afrh.modify(|r, w| unsafe {w.bits(r.bits() | (af << (4 * (pin.1 - 8))))});}
      else {gpioh.afrl.modify(|r, w| unsafe {w.bits(r.bits() | (af << (4 * pin.1)))});}
    },
    _   => unreachable!()
  };

  return Ok(Pin {
    block: pin.0,
    number: pin.1,
    inner: AlternateFunction(af)
  });
}

pub unsafe fn pinmode_alternate_function_force(pin: (char, u8), af: u32) -> Result<Pin<AlternateFunction>, ProgError> {
  let peripheral_ptr = stm32f4::stm32f446::Peripherals::steal();
  let rcc = &peripheral_ptr.RCC;

  if let Err(error) = check_pin(pin) {return Err(error);}
  if af > 15 {
    rprintln!("Only alternate funtion values between 0 and 15 are valid! | pin_mode()");
    return Err(ProgError::InvalidConfiguration);
  }

  match pin.0 {
    'a' => {
      let gpioa = &peripheral_ptr.GPIOA;
      rcc.ahb1enr.modify(|_, w| w.gpioaen().enabled());
      gpioa.moder.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin.1)) | (2 << (2 * pin.1)))});
      if pin.1 > 7 {gpioa.afrh.modify(|r, w| unsafe {w.bits(r.bits() | (af << (4 * (pin.1 - 8))))});}
      else {gpioa.afrl.modify(|r, w| unsafe {w.bits(r.bits() | (af << (4 * pin.1)))});}
    },
    'b' => {
      let gpiob = &peripheral_ptr.GPIOB;
      rcc.ahb1enr.modify(|_, w| w.gpioben().enabled());
      gpiob.moder.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin.1)) | (2 << (2 * pin.1)))});
      if pin.1 > 7 {gpiob.afrh.modify(|r, w| unsafe {w.bits(r.bits() | (af << (4 * (pin.1 - 8))))});}
      else {gpiob.afrl.modify(|r, w| unsafe {w.bits(r.bits() | (af << (4 * pin.1)))});}
    },
    'c' => {
      let gpioc = &peripheral_ptr.GPIOC;
      rcc.ahb1enr.modify(|_, w| w.gpiocen().enabled());
      gpioc.moder.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin.1)) | (2 << (2 * pin.1)))});
      if pin.1 > 7 {gpioc.afrh.modify(|r, w| unsafe {w.bits(r.bits() | (af << (4 * (pin.1 - 8))))});}
      else {gpioc.afrl.modify(|r, w| unsafe {w.bits(r.bits() | (af << (4 * pin.1)))});}
    },
    'd' => {
      let gpiod = &peripheral_ptr.GPIOD;
      rcc.ahb1enr.modify(|_, w| w.gpioden().enabled());
      gpiod.moder.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin.1)) | (2 << (2 * pin.1)))});
      if pin.1 > 7 {gpiod.afrh.modify(|r, w| unsafe {w.bits(r.bits() | (af << (4 * (pin.1 - 8))))});}
      else {gpiod.afrl.modify(|r, w| unsafe {w.bits(r.bits() | (af << (4 * pin.1)))});}
    },
    'h' => {
      let gpioh = &peripheral_ptr.GPIOH;
      rcc.ahb1enr.modify(|_, w| w.gpiohen().enabled());
      gpioh.moder.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin.1)) | (2 << (2 * pin.1)))});
      if pin.1 > 7 {gpioh.afrh.modify(|r, w| unsafe {w.bits(r.bits() | (af << (4 * (pin.1 - 8))))});}
      else {gpioh.afrl.modify(|r, w| unsafe {w.bits(r.bits() | (af << (4 * pin.1)))});}
    },
    _   => unreachable!()
  };

  return Ok(Pin {
    block: pin.0,
    number: pin.1,
    inner: AlternateFunction(af)
  });
}

pub fn pinmode_analog_in(pin: (char, u8)) -> Result<Pin<AnalogIn>, ProgError> {
  let peripheral_ptr;
  unsafe {peripheral_ptr = stm32f4::stm32f446::Peripherals::steal();} 
  let rcc = &peripheral_ptr.RCC;

  let channel_data: (u8, u8);

  if let Err(error) = check_pin(pin) {return Err(error);}

  unsafe {
    if !PIN_CONF.contains(&pin) {PIN_CONF.push(pin).unwrap();}
    else {
      rprintln!("P{}{} is already configured! | pin_mode()", pin.0.to_uppercase(), pin.1);
      return Err(ProgError::AlreadyConfigured);
    }
  }

  match pin.0 {
    'a' => {
      let gpioa = &peripheral_ptr.GPIOA;
      rcc.ahb1enr.modify(|_, w| w.gpioaen().enabled());
      gpioa.moder.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin.1)) | (3 << (2 * pin.1)))});
      channel_data = match enable_channel(pin, false) {
        Ok(value) => value,
        Err(error) => return Err(error)
      };
    },
    'b' => {
      let gpiob = &peripheral_ptr.GPIOB;
      rcc.ahb1enr.modify(|_, w| w.gpioben().enabled());
      gpiob.moder.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin.1)) | (3 << (2 * pin.1)))});
      channel_data = match enable_channel(pin, false) {
        Ok(value) => value,
        Err(error) => return Err(error)
      };
    },
    'c' => {
      let gpioc = &peripheral_ptr.GPIOC;
      rcc.ahb1enr.modify(|_, w| w.gpiocen().enabled());
      gpioc.moder.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin.1)) | (3 << (2 * pin.1)))});
      channel_data = match enable_channel(pin, false) {
        Ok(value) => value,
        Err(error) => return Err(error)
      };
    },
    'd' => {
      let gpiod = &peripheral_ptr.GPIOD;
      rcc.ahb1enr.modify(|_, w| w.gpioden().enabled());
      gpiod.moder.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin.1)) | (3 << (2 * pin.1)))});
      channel_data = match enable_channel(pin, false) {
        Ok(value) => value,
        Err(error) => return Err(error)
      };
    },
    'h' => {
      let gpioh = &peripheral_ptr.GPIOH;
      rcc.ahb1enr.modify(|_, w| w.gpiohen().enabled());
      gpioh.moder.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin.1)) | (3 << (2 * pin.1)))});
      channel_data = match enable_channel(pin, false) {
        Ok(value) => value,
        Err(error) => return Err(error)
      };
    },
    _   => unreachable!()
  };

  return Ok(Pin {
    block: pin.0,
    number: pin.1,
    inner: AnalogIn {
      core: channel_data.0,
      channel: channel_data.1
    }
  });
}

pub unsafe fn pinmode_analog_force(pin: (char, u8)) -> Result<Pin<AnalogIn>, ProgError> {
  let peripheral_ptr = stm32f4::stm32f446::Peripherals::steal();
  let rcc = &peripheral_ptr.RCC;

  let channel_data: (u8, u8);

  if let Err(error) = check_pin(pin) {return Err(error);}

  match pin.0 {
    'a' => {
      let gpioa = &peripheral_ptr.GPIOA;
      rcc.ahb1enr.modify(|_, w| w.gpioaen().enabled());
      gpioa.moder.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin.1)) | (3 << (2 * pin.1)))});
      channel_data = match enable_channel(pin, false) {
        Ok(value) => value,
        Err(error) => return Err(error)
      };
    },
    'b' => {
      let gpiob = &peripheral_ptr.GPIOB;
      rcc.ahb1enr.modify(|_, w| w.gpioben().enabled());
      gpiob.moder.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin.1)) | (3 << (2 * pin.1)))});
      channel_data = match enable_channel(pin, false) {
        Ok(value) => value,
        Err(error) => return Err(error)
      };
    },
    'c' => {
      let gpioc = &peripheral_ptr.GPIOC;
      rcc.ahb1enr.modify(|_, w| w.gpiocen().enabled());
      gpioc.moder.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin.1)) | (3 << (2 * pin.1)))});
      channel_data = match enable_channel(pin, false) {
        Ok(value) => value,
        Err(error) => return Err(error)
      };
    },
    'd' => {
      let gpiod = &peripheral_ptr.GPIOD;
      rcc.ahb1enr.modify(|_, w| w.gpioden().enabled());
      gpiod.moder.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin.1)) | (3 << (2 * pin.1)))});
      channel_data = match enable_channel(pin, false) {
        Ok(value) => value,
        Err(error) => return Err(error)
      };
    },
    'h' => {
      let gpioh = &peripheral_ptr.GPIOH;
      rcc.ahb1enr.modify(|_, w| w.gpiohen().enabled());
      gpioh.moder.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin.1)) | (3 << (2 * pin.1)))});
      channel_data = match enable_channel(pin, false) {
        Ok(value) => value,
        Err(error) => return Err(error)
      };
    },
    _   => unreachable!()
  };

  return Ok(Pin {
    block: pin.0,
    number: pin.1,
    inner: AnalogIn {
      core: channel_data.0,
      channel: channel_data.1
    }
  });
}

pub fn pinmode_analog_out(pin: (char, u8)) -> Result<Pin<AnalogOut>, ProgError> {
  let peripheral_ptr;
  unsafe {peripheral_ptr = stm32f4::stm32f446::Peripherals::steal();} 
  let rcc = &peripheral_ptr.RCC;

  let channel_data: (u8, u8);

  if pin != ('a', 4) && pin != ('a', 5) {return Err(ProgError::InvalidConfiguration);}

  unsafe {
    if !PIN_CONF.contains(&pin) {PIN_CONF.push(pin).unwrap();}
    else {
      rprintln!("P{}{} is already configured! | pin_mode()", pin.0.to_uppercase(), pin.1);
      return Err(ProgError::AlreadyConfigured);
    }
  }

  match pin.0 {
    'a' => {
      let gpioa = &peripheral_ptr.GPIOA;
      rcc.ahb1enr.modify(|_, w| w.gpioaen().enabled());
      gpioa.moder.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin.1)) | (3 << (2 * pin.1)))});
      channel_data = match enable_channel(pin, true) {
        Ok(value) => value,
        Err(error) => return Err(error)
      };
    },
    _   => unreachable!()
  };

  return Ok(Pin {
    block: pin.0,
    number: pin.1,
    inner: AnalogOut {
      core: channel_data.0,
      channel: channel_data.1
    }
  });
}

pub unsafe fn pinmode_analog_out_force(pin: (char, u8)) -> Result<Pin<AnalogOut>, ProgError> {
  let peripheral_ptr = stm32f4::stm32f446::Peripherals::steal();
  let rcc = &peripheral_ptr.RCC;

  let channel_data: (u8, u8);

  if pin != ('a', 4) && pin != ('a', 5) {return Err(ProgError::InvalidConfiguration);}

  match pin.0 {
    'a' => {
      let gpioa = &peripheral_ptr.GPIOA;
      rcc.ahb1enr.modify(|_, w| w.gpioaen().enabled());
      gpioa.moder.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin.1)) | (3 << (2 * pin.1)))});
      channel_data = match enable_channel(pin, true) {
        Ok(value) => value,
        Err(error) => return Err(error)
      };
    },
    _   => unreachable!()
  };

  return Ok(Pin {
    block: pin.0,
    number: pin.1,
    inner: AnalogOut {
      core: channel_data.0,
      channel: channel_data.1
    }
  });
}

pub fn pinmode_pwm(pin: (char, u8)) -> Result<Pin<PWM>, ProgError> {
  let peripheral_ptr;
  unsafe {peripheral_ptr = stm32f4::stm32f446::Peripherals::steal();} 
  let rcc = &peripheral_ptr.RCC;

  if let Err(error) = check_pin(pin) {return Err(error);}

  unsafe {
    if !PIN_CONF.contains(&pin) {PIN_CONF.push(pin).unwrap();}
    else {
      rprintln!("P{}{} is already configured! | pin_mode()", pin.0.to_uppercase(), pin.1);
      return Err(ProgError::AlreadyConfigured);
    }
  }

  let returns = match setup_pwm(pin) {
    Ok(values) => values,
    Err(error) => return Err(error)
  };

  match pin.0 {
    'a' => {
      let gpioa = &peripheral_ptr.GPIOA;
      rcc.ahb1enr.modify(|_, w| w.gpioaen().enabled());
      gpioa.moder.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin.1)) | (2 << (2 * pin.1)))});
      if pin.1 > 7 {gpioa.afrh.modify(|r, w| unsafe {w.bits(r.bits() | ((returns.2 as u32) << (4 * (pin.1 - 8))))});}
      else {gpioa.afrl.modify(|r, w| unsafe {w.bits(r.bits() | ((returns.2 as u32) << (4 * pin.1)))});}
    },
    'b' => {
      let gpiob = &peripheral_ptr.GPIOB;
      rcc.ahb1enr.modify(|_, w| w.gpioben().enabled());
      gpiob.moder.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin.1)) | (2 << (2 * pin.1)))});
      if pin.1 > 7 {gpiob.afrh.modify(|r, w| unsafe {w.bits(r.bits() | ((returns.2 as u32) << (4 * (pin.1 - 8))))});}
      else {gpiob.afrl.modify(|r, w| unsafe {w.bits(r.bits() | ((returns.2 as u32) << (4 * pin.1)))});}
    },
    'c' => {
      let gpioc = &peripheral_ptr.GPIOC;
      rcc.ahb1enr.modify(|_, w| w.gpiocen().enabled());
      gpioc.moder.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin.1)) | (2 << (2 * pin.1)))});
      if pin.1 > 7 {gpioc.afrh.modify(|r, w| unsafe {w.bits(r.bits() | ((returns.2 as u32) << (4 * (pin.1 - 8))))});}
      else {gpioc.afrl.modify(|r, w| unsafe {w.bits(r.bits() | ((returns.2 as u32) << (4 * pin.1)))});}
    },
    'd' => {
      let gpiod = &peripheral_ptr.GPIOD;
      rcc.ahb1enr.modify(|_, w| w.gpioden().enabled());
      gpiod.moder.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin.1)) | (2 << (2 * pin.1)))});
      if pin.1 > 7 {gpiod.afrh.modify(|r, w| unsafe {w.bits(r.bits() | ((returns.2 as u32) << (4 * (pin.1 - 8))))});}
      else {gpiod.afrl.modify(|r, w| unsafe {w.bits(r.bits() | ((returns.2 as u32) << (4 * pin.1)))});}
    },
    'h' => {
      let gpioh = &peripheral_ptr.GPIOH;
      rcc.ahb1enr.modify(|_, w| w.gpiohen().enabled());
      gpioh.moder.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin.1)) | (2 << (2 * pin.1)))});
      if pin.1 > 7 {gpioh.afrh.modify(|r, w| unsafe {w.bits(r.bits() | ((returns.2 as u32) << (4 * (pin.1 - 8))))});}
      else {gpioh.afrl.modify(|r, w| unsafe {w.bits(r.bits() | ((returns.2 as u32) << (4 * pin.1)))});}
    },
    _   => unreachable!()
  };

  return Ok(Pin {
    block: pin.0,
    number: pin.1,
    inner: PWM {
      timer: returns.0,
      ccch: returns.1
    }
  });
}

pub unsafe fn pinmode_pwm_force(pin: (char, u8)) -> Result<Pin<PWM>, ProgError> {
  let peripheral_ptr = stm32f4::stm32f446::Peripherals::steal();
  let rcc = &peripheral_ptr.RCC;

  if let Err(error) = check_pin(pin) {return Err(error);}

  if !PIN_CONF.contains(&pin) {PIN_CONF.push(pin).unwrap();}
  else {
    rprintln!("P{}{} is already configured! | pin_mode()", pin.0.to_uppercase(), pin.1);
    return Err(ProgError::AlreadyConfigured);
  }

  let returns = match setup_pwm(pin) {
    Ok(values) => values,
    Err(error) => return Err(error)
  };

  match pin.0 {
    'a' => {
      let gpioa = &peripheral_ptr.GPIOA;
      rcc.ahb1enr.modify(|_, w| w.gpioaen().enabled());
      gpioa.moder.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin.1)) | (2 << (2 * pin.1)))});
      if pin.1 > 7 {gpioa.afrh.modify(|r, w| unsafe {w.bits(r.bits() | ((returns.2 as u32) << (4 * (pin.1 - 8))))});}
      else {gpioa.afrl.modify(|r, w| unsafe {w.bits(r.bits() | ((returns.2 as u32) << (4 * pin.1)))});}
    },
    'b' => {
      let gpiob = &peripheral_ptr.GPIOB;
      rcc.ahb1enr.modify(|_, w| w.gpioben().enabled());
      gpiob.moder.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin.1)) | (2 << (2 * pin.1)))});
      if pin.1 > 7 {gpiob.afrh.modify(|r, w| unsafe {w.bits(r.bits() | ((returns.2 as u32) << (4 * (pin.1 - 8))))});}
      else {gpiob.afrl.modify(|r, w| unsafe {w.bits(r.bits() | ((returns.2 as u32) << (4 * pin.1)))});}
    },
    'c' => {
      let gpioc = &peripheral_ptr.GPIOC;
      rcc.ahb1enr.modify(|_, w| w.gpiocen().enabled());
      gpioc.moder.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin.1)) | (2 << (2 * pin.1)))});
      if pin.1 > 7 {gpioc.afrh.modify(|r, w| unsafe {w.bits(r.bits() | ((returns.2 as u32) << (4 * (pin.1 - 8))))});}
      else {gpioc.afrl.modify(|r, w| unsafe {w.bits(r.bits() | ((returns.2 as u32) << (4 * pin.1)))});}
    },
    'd' => {
      let gpiod = &peripheral_ptr.GPIOD;
      rcc.ahb1enr.modify(|_, w| w.gpioden().enabled());
      gpiod.moder.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin.1)) | (2 << (2 * pin.1)))});
      if pin.1 > 7 {gpiod.afrh.modify(|r, w| unsafe {w.bits(r.bits() | ((returns.2 as u32) << (4 * (pin.1 - 8))))});}
      else {gpiod.afrl.modify(|r, w| unsafe {w.bits(r.bits() | ((returns.2 as u32) << (4 * pin.1)))});}
    },
    'h' => {
      let gpioh = &peripheral_ptr.GPIOH;
      rcc.ahb1enr.modify(|_, w| w.gpiohen().enabled());
      gpioh.moder.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin.1)) | (2 << (2 * pin.1)))});
      if pin.1 > 7 {gpioh.afrh.modify(|r, w| unsafe {w.bits(r.bits() | ((returns.2 as u32) << (4 * (pin.1 - 8))))});}
      else {gpioh.afrl.modify(|r, w| unsafe {w.bits(r.bits() | ((returns.2 as u32) << (4 * pin.1)))});}
    },
    _   => unreachable!()
  };

  return Ok(Pin {
    block: pin.0,
    number: pin.1,
    inner: PWM {
      timer: returns.0,
      ccch: returns.1
    }
  });
}

pub fn digital_write(pin: &Pin<Output>, value: bool) {
  let peripheral_ptr;
  unsafe {peripheral_ptr = stm32f4::stm32f446::Peripherals::steal();} 

  match pin.block {
    'a' => {
      let gpioa = &peripheral_ptr.GPIOA;
      if value {gpioa.bsrr.write(|w| unsafe {w.bits(1 << pin.number)});}
      else {gpioa.bsrr.write(|w| unsafe {w.bits(1 << (pin.number + 16))});}
    },
    'b' => {
      let gpiob = &peripheral_ptr.GPIOB;
      if value {gpiob.bsrr.write(|w| unsafe {w.bits(1 << pin.number)});}
      else {gpiob.bsrr.write(|w| unsafe {w.bits(1 << (pin.number + 16))});}
    },
    'c' => {
      let gpioc = &peripheral_ptr.GPIOC;
      if value {gpioc.bsrr.write(|w| unsafe {w.bits(1 << pin.number)});}
      else {gpioc.bsrr.write(|w| unsafe {w.bits(1 << (pin.number + 16))});}
    },
    'd' => {
      let gpiod = &peripheral_ptr.GPIOD;
      if value {gpiod.bsrr.write(|w| unsafe {w.bits(1 << pin.number)});}
      else {gpiod.bsrr.write(|w| unsafe {w.bits(1 << (pin.number + 16))});}
    },
    'h' => {
      let gpioh = &peripheral_ptr.GPIOH;
      if value {gpioh.bsrr.write(|w| unsafe {w.bits(1 << pin.number)});}
      else {gpioh.bsrr.write(|w| unsafe {w.bits(1 << (pin.number + 16))});}
    },
    _   => unreachable!()
  };
}

pub fn digital_read(pin: &Pin<Input>) -> bool {
  let peripheral_ptr;
  unsafe {peripheral_ptr = stm32f4::stm32f446::Peripherals::steal();} 

  let bits = match pin.block {
    'a' => {
      let gpioa = &peripheral_ptr.GPIOA;
      gpioa.idr.read().bits()
    },
    'b' => {
      let gpiob = &peripheral_ptr.GPIOB;
      gpiob.idr.read().bits()
    },
    'c' => {
      let gpioc = &peripheral_ptr.GPIOC;
      gpioc.idr.read().bits()
    },
    'd' => {
      let gpiod = &peripheral_ptr.GPIOD;
      gpiod.idr.read().bits()
    },
    'h' => {
      let gpioh = &peripheral_ptr.GPIOH;
      gpioh.idr.read().bits()
    },
    _   => unreachable!()
  };

  return bits & (1 << pin.number) == (1 << pin.number);
}

pub fn digital_state(pin: &Pin<Output>) -> bool {
  let peripheral_ptr;
  unsafe {peripheral_ptr = stm32f4::stm32f446::Peripherals::steal();} 

  let bits = match pin.block {
    'a' => {
      let gpioa = &peripheral_ptr.GPIOA;
      gpioa.odr.read().bits()
    },
    'b' => {
      let gpiob = &peripheral_ptr.GPIOB;
      gpiob.odr.read().bits()
    },
    'c' => {
      let gpioc = &peripheral_ptr.GPIOC;
      gpioc.odr.read().bits()
    },
    'd' => {
      let gpiod = &peripheral_ptr.GPIOD;
      gpiod.odr.read().bits()
    },
    'h' => {
      let gpioh = &peripheral_ptr.GPIOH;
      gpioh.odr.read().bits()
    },
    _   => unreachable!()
  };

  return bits & (1 << pin.number) == (1 << pin.number);
}

pub fn set_bias<T>(pin: &Pin<T>, bias: GpioBias) {
  let peripheral_ptr;
  unsafe {peripheral_ptr = stm32f4::stm32f446::Peripherals::steal();} 

  let num = pin.number;

  match pin.block {
    'a' => {
      let gpioa = &peripheral_ptr.GPIOA;
      match bias {
        GpioBias::None => gpioa.pupdr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * num)))}),
        GpioBias::Pullup => gpioa.pupdr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * num)) | (1 << (2 * num)))}),
        GpioBias::Pulldown => gpioa.pupdr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * num)) | (2 << (2 * num)))})
      };
    },
    'b' => {
      let gpiob = &peripheral_ptr.GPIOB;
      match bias {
        GpioBias::None => gpiob.pupdr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * num)))}),
        GpioBias::Pullup => gpiob.pupdr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * num)) | (1 << (2 * num)))}),
        GpioBias::Pulldown => gpiob.pupdr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * num)) | (2 << (2 * num)))})
      };
    },
    'c' => {
      let gpioc = &peripheral_ptr.GPIOC;
      match bias {
        GpioBias::None => gpioc.pupdr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * num)))}),
        GpioBias::Pullup => gpioc.pupdr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * num)) | (1 << (2 * num)))}),
        GpioBias::Pulldown => gpioc.pupdr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * num)) | (2 << (2 * num)))})
      };
    },
    'd' => {
      let gpiod = &peripheral_ptr.GPIOD;
      match bias {
        GpioBias::None => gpiod.pupdr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * num)))}),
        GpioBias::Pullup => gpiod.pupdr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * num)) | (1 << (2 * num)))}),
        GpioBias::Pulldown => gpiod.pupdr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * num)) | (2 << (2 * num)))})
      };
    },
    'h' => {
      let gpioh = &peripheral_ptr.GPIOH;
      match bias {
        GpioBias::None => gpioh.pupdr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * num)))}),
        GpioBias::Pullup => gpioh.pupdr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * num)) | (1 << (2 * num)))}),
        GpioBias::Pulldown => gpioh.pupdr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * num)) | (2 << (2 * num)))})
      };
    },
    _   => unreachable!()
  };
}

pub fn set_speed<T>(pin: &Pin<T>, speed: GpioSpeed) {
  let peripheral_ptr;
  unsafe {peripheral_ptr = stm32f4::stm32f446::Peripherals::steal();} 

  let num = pin.number;

  match pin.block {
    'a' => {
      let gpioa = &peripheral_ptr.GPIOA;
      match speed {
        GpioSpeed::Low => gpioa.ospeedr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * num)))}),
        GpioSpeed::Medium => gpioa.ospeedr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * num)) | (1 << (2 * num)))}),
        GpioSpeed::Fast => gpioa.ospeedr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * num)) | (2 << (2 * num)))}),
        GpioSpeed::High => gpioa.ospeedr.modify(|r, w| unsafe {w.bits(r.bits() | (3 << (2 * num)))})
      };
    },
    'b' => {
      let gpiob = &peripheral_ptr.GPIOB;
      match speed {
        GpioSpeed::Low => gpiob.ospeedr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * num)))}),
        GpioSpeed::Medium => gpiob.ospeedr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * num)) | (1 << (2 * num)))}),
        GpioSpeed::Fast => gpiob.ospeedr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * num)) | (2 << (2 * num)))}),
        GpioSpeed::High => gpiob.ospeedr.modify(|r, w| unsafe {w.bits(r.bits() | (3 << (2 * num)))})
      };
    },
    'c' => {
      let gpioc = &peripheral_ptr.GPIOC;
      match speed {
        GpioSpeed::Low => gpioc.ospeedr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * num)))}),
        GpioSpeed::Medium => gpioc.ospeedr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * num)) | (1 << (2 * num)))}),
        GpioSpeed::Fast => gpioc.ospeedr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * num)) | (2 << (2 * num)))}),
        GpioSpeed::High => gpioc.ospeedr.modify(|r, w| unsafe {w.bits(r.bits() | (3 << (2 * num)))})
      };
    },
    'd' => {
      let gpiod = &peripheral_ptr.GPIOD;
      match speed {
        GpioSpeed::Low => gpiod.ospeedr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * num)))}),
        GpioSpeed::Medium => gpiod.ospeedr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * num)) | (1 << (2 * num)))}),
        GpioSpeed::Fast => gpiod.ospeedr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * num)) | (2 << (2 * num)))}),
        GpioSpeed::High => gpiod.ospeedr.modify(|r, w| unsafe {w.bits(r.bits() | (3 << (2 * num)))})
      };
    },
    'h' => {
      let gpioh = &peripheral_ptr.GPIOH;
      match speed {
        GpioSpeed::Low => gpioh.ospeedr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * num)))}),
        GpioSpeed::Medium => gpioh.ospeedr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * num)) | (1 << (2 * num)))}),
        GpioSpeed::Fast => gpioh.ospeedr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * num)) | (2 << (2 * num)))}),
        GpioSpeed::High => gpioh.ospeedr.modify(|r, w| unsafe {w.bits(r.bits() | (3 << (2 * num)))})
      };
    },
    _   => unreachable!()
  };
}

pub fn open_drain<T>(pin: &Pin<T>, op: bool) {
  let peripheral_ptr;
  unsafe {peripheral_ptr = stm32f4::stm32f446::Peripherals::steal();} 

  match pin.block {
    'a' => {
      let gpioa = &peripheral_ptr.GPIOA;
      if op {gpioa.otyper.modify(|r, w| unsafe {w.bits(r.bits() | (1 << pin.number))});}
      else {gpioa.otyper.modify(|r, w| unsafe {w.bits(r.bits() | (0 << pin.number))});}
    },
    'b' => {
      let gpiob = &peripheral_ptr.GPIOB;
      if op {gpiob.otyper.modify(|r, w| unsafe {w.bits(r.bits() | (1 << pin.number))});}
      else {gpiob.otyper.modify(|r, w| unsafe {w.bits(r.bits() | (0 << pin.number))});}
    },
    'c' => {
      let gpioc = &peripheral_ptr.GPIOC;
      if op {gpioc.otyper.modify(|r, w| unsafe {w.bits(r.bits() | (1 << pin.number))});}
      else {gpioc.otyper.modify(|r, w| unsafe {w.bits(r.bits() | (0 << pin.number))});}
    },
    'd' => {
      let gpiod = &peripheral_ptr.GPIOD;
      if op {gpiod.otyper.modify(|r, w| unsafe {w.bits(r.bits() | (1 << pin.number))});}
      else {gpiod.otyper.modify(|r, w| unsafe {w.bits(r.bits() | (0 << pin.number))});}
    },
    'h' => {
      let gpioh = &peripheral_ptr.GPIOH;
      if op {gpioh.otyper.modify(|r, w| unsafe {w.bits(r.bits() | (1 << pin.number))});}
      else {gpioh.otyper.modify(|r, w| unsafe {w.bits(r.bits() | (0 << pin.number))});}
    },
    _   => unreachable!()
  };
}


// Private Functions ==============================================================================
fn check_pin(pin: (char, u8)) -> Result<(), ProgError> {
  if pin.1 > 15 || pin.0 == 'd' && pin.1 != 2 || pin.0 == 'h' && pin.1 != 0 && pin.1 != 1 {
    rprintln!("P{}{} is not an available GPIO Pin!", pin.0.to_uppercase(), pin.1);
    return Err(ProgError::InvalidConfiguration);
  }
  else {return Ok(());}
}

impl<T> Drop for Pin<T> {
  fn drop(&mut self) {
    unsafe {PIN_CONF.swap_remove(PIN_CONF.iter().position(|&i| i == (self.block, self.number)).unwrap());}
  }
}
