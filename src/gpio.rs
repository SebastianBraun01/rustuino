//! This module contains everything that is related to the digital IO functionality.
//! 
//! # Examples
//! 
//! ```no_run
//! #![no_std]
//! #![no_main]
//! 
//! use rustuino::*;
//! 
//! #[entry]
//! fn main() -> ! {
//!   // Configure pins and get the pin-structs
//!   let in_pin = pinmode_input(PA0).unwrap();
//!   let out_pin = pinmode_output(PA1).unwrap();
//!   let af_pin = pinmode_alternate_function(PA2, 7).unwrap();
//!   let analog_pin = pinmode_analog(PA4).unwrap();
//! 
//!   loop {
//!     // Read from the input pin
//!     let value = digital_read(&in_pin);
//!     
//!     // Set the output pin
//!     digital_write(&out_pin, true);
//!   }
//! }
//! ```

use crate::analog::enable_channel;
use crate::time::setup_pwm;
use crate::include::{ProgError, PIN_CONF};
use rtt_target::rprintln;

/// Represents a configured pin. Is returned from pinmode-functions.
pub struct Pin<T> {
  #[doc(hidden)]
  pub block: char,
  #[doc(hidden)]
  pub number: u8,
  #[doc(hidden)]
  pub inner: T
}

#[doc(hidden)]
pub struct Input;
#[doc(hidden)]
pub struct Output;
#[doc(hidden)]
pub struct AlternateFunction(u32);
#[doc(hidden)]
pub struct Analog {
  #[doc(hidden)]
  pub core: u8,
  #[doc(hidden)]
  pub channel: u8
}
#[doc(hidden)]
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

/// Represents the options for pullup-/pulldown-resistors.
pub enum GpioBias {
  None, Pullup, Pulldown
}


// Public Functions ===============================================================================
/// Configures a pin to be a digital input.
/// 
/// Takes pin identifier [A0, C5, etc.](crate::include::pins) as an argument and returns a [pin-struct](crate::gpio::Pin)
/// for other functions.
/// Panics if pin identifier is not a valid pin.
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

/// Configures a pin to be a digital input. Disregard if pin is already configured.
/// 
/// Takes pin identifier [A0, C5, etc.](crate::include::pins) as an argument and returns a [pin-struct](crate::gpio::Pin)
/// for other functions.
/// Panics if pin identifier is not a valid pin.
/// 
/// # Safety
/// 
/// This function can be used to get more than one pin-structs of a configured pin. Keep in mind that the registers of
/// the pin will still be configured. This can easily break other functions for the pin.
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

/// Configures a pin to be a digital output.
/// 
/// Takes pin identifier [A0, C5, etc.](crate::include::pins) as an argument and returns a [pin-struct](crate::gpio::Pin)
/// for other functions.
/// Panics if pin identifier is not a valid pin.
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

/// Configures a pin to be a digital output. Disregard if pin is already configured.
/// 
/// Takes pin identifier [A0, C5, etc.](crate::include::pins) as an argument and returns a [pin-struct](crate::gpio::Pin)
/// for other functions.
/// Panics if pin identifier is not a valid pin.
/// 
/// # Safety
/// 
/// This function can be used to get more than one pin-structs of a configured pin. Keep in mind that the registers of
/// the pin will still be configured. This can easily break other functions for the pin.
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

/// Configures an alternate function for a pin.
/// 
/// Takes pin identifier [A0, C5, etc.](crate::include::pins) and a AF number as arguments and returns a [pin-struct](crate::gpio::Pin)
/// for other functions.
/// Panics if either pin identifier is not a valid pin or the AF value is not valid.
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

/// Configures an alternate function for a pin.
/// 
/// Takes pin identifier [A0, C5, etc.](crate::include::pins) and a AF number as arguments and returns a [pin-struct](crate::gpio::Pin)
/// for other functions.
/// Panics if either pin identifier is not a valid pin or the AF value is not valid.
/// 
/// # Safety
/// 
/// This function can be used to get more than one pin-structs of a configured pin. Keep in mind that the registers of
/// the pin will still be configured. This can easily break other functions for the pin.
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

/// Configures a pin to be an analog input.
/// 
/// Takes pin identifier [A0, C5, etc.](crate::include::pins) as an argument and returns a [pin-struct](crate::gpio::Pin)
/// for other functions.
/// Panics if pin identifier is not a pin that can be used the the internal ADCs. To see witch pins are available for
/// analog functionality see the docs of [ADC_MAP](crate::include::ADC_MAP).
pub fn pinmode_analog(pin: (char, u8)) -> Result<Pin<Analog>, ProgError> {
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
      channel_data = match enable_channel(pin) {
        Ok(value) => value,
        Err(error) => return Err(error)
      };
    },
    'b' => {
      let gpiob = &peripheral_ptr.GPIOB;
      rcc.ahb1enr.modify(|_, w| w.gpioben().enabled());
      gpiob.moder.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin.1)) | (3 << (2 * pin.1)))});
      channel_data = match enable_channel(pin) {
        Ok(value) => value,
        Err(error) => return Err(error)
      };
    },
    'c' => {
      let gpioc = &peripheral_ptr.GPIOC;
      rcc.ahb1enr.modify(|_, w| w.gpiocen().enabled());
      gpioc.moder.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin.1)) | (3 << (2 * pin.1)))});
      channel_data = match enable_channel(pin) {
        Ok(value) => value,
        Err(error) => return Err(error)
      };
    },
    'd' => {
      let gpiod = &peripheral_ptr.GPIOD;
      rcc.ahb1enr.modify(|_, w| w.gpioden().enabled());
      gpiod.moder.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin.1)) | (3 << (2 * pin.1)))});
      channel_data = match enable_channel(pin) {
        Ok(value) => value,
        Err(error) => return Err(error)
      };
    },
    'h' => {
      let gpioh = &peripheral_ptr.GPIOH;
      rcc.ahb1enr.modify(|_, w| w.gpiohen().enabled());
      gpioh.moder.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin.1)) | (3 << (2 * pin.1)))});
      channel_data = match enable_channel(pin) {
        Ok(value) => value,
        Err(error) => return Err(error)
      };
    },
    _   => unreachable!()
  };

  return Ok(Pin {
    block: pin.0,
    number: pin.1,
    inner: Analog {
      core: channel_data.0,
      channel: channel_data.1
    }
  });
}

/// Configures a pin to be an analog input.
/// 
/// Takes pin identifier [A0, C5, etc.](crate::include::pins) as an argument and returns a [pin-struct](crate::gpio::Pin)
/// for other functions.
/// Panics if pin identifier is not a pin that can be used the the internal ADCs.  To see witch pins are available
/// for analog functionality see the docs of [ADC_MAP](crate::include::ADC_MAP).
/// 
/// # Safety
/// 
/// This function can be used to get more than one pin-structs of a configured pin. Keep in mind that the registers of
/// the pin will still be configured. This can easily break other functions for the pin.
pub unsafe fn pinmode_analog_force(pin: (char, u8)) -> Result<Pin<Analog>, ProgError> {
  let peripheral_ptr;
  peripheral_ptr = stm32f4::stm32f446::Peripherals::steal();
  let rcc = &peripheral_ptr.RCC;

  let channel_data: (u8, u8);

  if let Err(error) = check_pin(pin) {return Err(error);}

  match pin.0 {
    'a' => {
      let gpioa = &peripheral_ptr.GPIOA;
      rcc.ahb1enr.modify(|_, w| w.gpioaen().enabled());
      gpioa.moder.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin.1)) | (3 << (2 * pin.1)))});
      channel_data = match enable_channel(pin) {
        Ok(value) => value,
        Err(error) => return Err(error)
      };
    },
    'b' => {
      let gpiob = &peripheral_ptr.GPIOB;
      rcc.ahb1enr.modify(|_, w| w.gpioben().enabled());
      gpiob.moder.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin.1)) | (3 << (2 * pin.1)))});
      channel_data = match enable_channel(pin) {
        Ok(value) => value,
        Err(error) => return Err(error)
      };
    },
    'c' => {
      let gpioc = &peripheral_ptr.GPIOC;
      rcc.ahb1enr.modify(|_, w| w.gpiocen().enabled());
      gpioc.moder.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin.1)) | (3 << (2 * pin.1)))});
      channel_data = match enable_channel(pin) {
        Ok(value) => value,
        Err(error) => return Err(error)
      };
    },
    'd' => {
      let gpiod = &peripheral_ptr.GPIOD;
      rcc.ahb1enr.modify(|_, w| w.gpioden().enabled());
      gpiod.moder.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin.1)) | (3 << (2 * pin.1)))});
      channel_data = match enable_channel(pin) {
        Ok(value) => value,
        Err(error) => return Err(error)
      };
    },
    'h' => {
      let gpioh = &peripheral_ptr.GPIOH;
      rcc.ahb1enr.modify(|_, w| w.gpiohen().enabled());
      gpioh.moder.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin.1)) | (3 << (2 * pin.1)))});
      channel_data = match enable_channel(pin) {
        Ok(value) => value,
        Err(error) => return Err(error)
      };
    },
    _   => unreachable!()
  };

  return Ok(Pin {
    block: pin.0,
    number: pin.1,
    inner: Analog {
      core: channel_data.0,
      channel: channel_data.1
    }
  });
}

/// Configures a pin to be a PWM output.
/// 
/// Takes pin identifier [A0, C5, etc.](crate::include::pins) as an argument and returns a [pin-struct](crate::gpio::Pin)
/// for other functions.
/// Panics if pin identifier is not a pin that can be used as a PWM output with the internal timers. To see witch pins
/// are available for PWM functionality see the docs of [PWM_MAP](crate::include::PWM_MAP).
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

/// Configures a pin to be a PWM output.
/// 
/// Takes pin identifier [A0, C5, etc.](crate::include::pins) as an argument and returns a [pin-struct](crate::gpio::Pin)
/// for other functions.
/// Panics if pin identifier is not a pin that can be used as a PWM output with the internal timers. To see witch pins
/// are available for PWM functionality see the docs of [PWM_MAP](crate::include::PWM_MAP).
/// 
/// # Safety
/// 
/// This function can be used to get more than one pin-structs of a configured pin. Keep in mind that the registers of
/// the pin will still be configured. This can easily break other functions for the pin.
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

/// Sets the state of an output pin.
/// 
/// Takes [pin-struct](crate::gpio::Pin) of an output pin and a boolean value as arguments and sets the pin to that value.
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

/// Reads the state of an input pin.
/// 
/// Takes [pin-struct](crate::gpio::Pin) of an output pin as an argument and returns the boolean value of that pin.
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

/// Reads the set state of an output pin.
/// 
/// Takes [pin-struct](crate::gpio::Pin) of an output pin as an argument and returns the set state of that pin.
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

/// Sets if the pin has a pullup-, pulldown- or no bias-resistor connected internally.
/// 
/// Takes [pin-struct](crate::gpio::Pin) of a pin and the [config](crate::gpio::GpioBias) as arguments and sets the
/// bias of that pin.
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

/// Sets the driving speed of the pin.
/// 
/// Takes [pin-struct](crate::gpio::Pin) of a pin and the [speed](crate::gpio::GpioSpeed) as arguments and sets the
/// speed of that pin.
/// See the documentation of [GpioSpeed](crate::gpio::GpioSpeed) for frequency ratings.
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

/// Sets if the pin is driven in push-pull- or open-drain-configuration.
/// 
/// Takes [pin-struct](crate::gpio::Pin) of a pin and a boolean value as arguments and sets the drive-mode of that pin.
/// If the value is false the config is push-pull, if the value is true the config is open-drain.
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
