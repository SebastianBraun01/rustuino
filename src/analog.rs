//! Contains everything that is related to the analog IO functionality.
//! 
//! For information on whitch pins have analog IO capabilities, check [`ADC_MAP`](crate::include::ADC_MAP)
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
//!   // Configure an analog input, analog output and a a digital output
//!   let in_pin = pinmode_analog(PA0).unwrap();
//!   let led_pin = pinmode_output(PA1).unwrap();
//! 
//!   // Variable to store the analog value
//!   let mut value = 0;
//! 
//!   // Change the ADC resolution to 12 bits
//!   analog_resolution(12);
//! 
//!   loop {
//!     // Read from the analog input
//!     value = analog_read(&in_pin);
//!     
//!     // If voltage is above 5V turn on led, otherwise turn it off
//!     if value > 2046 {
//!       digital_write(&led_pin, true);
//!     }
//!     else {
//!       digital_write(&led_pin, false);
//!     }
//!   }
//! }
//! ```

use crate::include::{ProgError, ADC_MAP};
use crate::gpio::{Pin, Analog};
use rtt_target::rprintln;


#[doc(hidden)]
pub fn enable_channel(pin: (char, u8)) -> Result<(u8, u8), ProgError> {
  let peripheral_ptr;
  unsafe {peripheral_ptr = stm32f4::stm32f446::Peripherals::steal();}
  let rcc = &peripheral_ptr.RCC;
  let adcc = &peripheral_ptr.ADC_COMMON;

  let (core, channel) = match return_channel(pin) {
    Ok(values) => values,
    Err(error) => {
      rprintln!("P{}{} is not available for analog functions! | enable_channel()", pin.0.to_uppercase(), pin.1);
      return Err(error);
    }
  };

  match core {
    1 => {
      let adc1 = &peripheral_ptr.ADC1;
      if rcc.apb2enr.read().adc1en().is_disabled() {
        rcc.apb2enr.modify(|_, w| w.adc1en().enabled());
        adcc.ccr.modify(|_, w| w.adcpre().div2());
        adc1.smpr2.modify(|_, w| w.smp0().cycles144());
        adc1.cr1.modify(|_, w| w.res().ten_bit());
        adc1.cr2.modify(|_, w| w.adon().enabled());
      }
    },
    2 => {
      let adc2 = &peripheral_ptr.ADC2;
      if rcc.apb2enr.read().adc2en().is_disabled() {
        rcc.apb2enr.modify(|_, w| w.adc2en().enabled());
        adcc.ccr.modify(|_, w| w.adcpre().div2());
        adc2.smpr2.modify(|_, w| w.smp0().cycles144());
        adc2.cr1.modify(|_, w| w.res().ten_bit());
        adc2.cr2.modify(|_, w| w.adon().enabled());
      } 
    },
    3 => {
      let adc3 = &peripheral_ptr.ADC3;
      if rcc.apb2enr.read().adc3en().is_disabled() {
        rcc.apb2enr.modify(|_, w| w.adc3en().enabled());
        adcc.ccr.modify(|_, w| w.adcpre().div2());
        adc3.smpr2.modify(|_, w| w.smp0().cycles144());
        adc3.cr1.modify(|_, w| w.res().ten_bit());
        adc3.cr2.modify(|_, w| w.adon().enabled());
      }
    },
    _ => unreachable!()
  };

  return Ok((core, channel));
}

/// Changes the resolution for all analog pins
/// 
/// Possible values are 6, 8, 10 and 12 bit resolutions. 10 bits are the default.
/// Panics if resolution value is invalid.
pub fn adc_resolution(res: u8) -> Result<(), ProgError> {
  let peripheral_ptr;
  unsafe {peripheral_ptr = stm32f4::stm32f446::Peripherals::steal();}
  let rcc = &peripheral_ptr.RCC;
  let adc1 = &peripheral_ptr.ADC1;
  let adc2 = &peripheral_ptr.ADC2;
  let adc3 = &peripheral_ptr.ADC3;

  let enc_res = match res {
    6  => 3,
    8  => 2,
    10 => 1,
    12 => 0,
    _ => {
      rprintln!("{} is not a available ADC resolution! | adc_resolution()", res);
      return Err(ProgError::InvalidConfiguration);
    }
  };

  if rcc.apb2enr.read().adc1en().is_enabled() {adc1.cr1.modify(|_, w| w.res().bits(enc_res));}
  if rcc.apb2enr.read().adc2en().is_enabled() {adc2.cr1.modify(|_, w| w.res().bits(enc_res));}
  if rcc.apb2enr.read().adc3en().is_enabled() {adc3.cr1.modify(|_, w| w.res().bits(enc_res));}

  return Ok(());
}

/// Performs an ADC conversion on the pin and gives back the analog value
/// 
/// # Examples
/// 
/// ```no_run
/// // Configure pin as an analog input
/// let pin = pinmode_analog(PA0).unwrap();
/// 
/// // Read the analog value on the pin
/// let mut value: u16 = analog_read(&pin);
/// ```
pub fn analog_read(pin: &Pin<Analog>) -> u16 {
  let peripheral_ptr;
  unsafe {peripheral_ptr = stm32f4::stm32f446::Peripherals::steal();}

  let buffer = match pin.inner.core {
    1 => {
      let adc1 = &peripheral_ptr.ADC1;
      adc1.sqr3.modify(|_, w| unsafe {w.sq1().bits(pin.number)});
      adc1.cr2.write(|w| w.swstart().start());
      while adc1.sr.read().eoc().is_not_complete() {}
      adc1.dr.read().data().bits()
    },
    2 => {
      let adc2 = &peripheral_ptr.ADC2;
      adc2.sqr3.modify(|_, w| unsafe {w.sq1().bits(pin.number)});
      adc2.cr2.write(|w| w.swstart().start());
      while adc2.sr.read().eoc().is_not_complete() {}
      adc2.dr.read().data().bits()
    },
    3 => {
      let adc3 = &peripheral_ptr.ADC3;
      adc3.sqr3.modify(|_, w| unsafe {w.sq1().bits(pin.number)});
      adc3.cr2.write(|w| w.swstart().start());
      while adc3.sr.read().eoc().is_not_complete() {}
      adc3.dr.read().data().bits()
    },
    _ => unreachable!()
  };

  return buffer;
}


// Private Functions ==============================================================================
fn return_channel(pin: (char, u8)) -> Result<(u8, u8), ProgError> {
  if !ADC_MAP.pins.contains(&pin) {return Err(ProgError::InvalidConfiguration);}
  else {
    let core = ADC_MAP.adcs[ADC_MAP.pins.iter().position(|&i| i == pin).unwrap()];
    let channel = ADC_MAP.channels[ADC_MAP.pins.iter().position(|&i| i == pin).unwrap()];

    return Ok((core, channel));
  }
}
