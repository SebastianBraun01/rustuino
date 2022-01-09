//! This module contains everything that is related to timer based functions.
//! 
//! For information on whitch pins have PWM capabilities, check [`PWM_MAP`](crate::include::PWM_MAP)
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
//!   // Configure A8 for PWM use
//!   let pwm_pin = pinmode_pwm(PA8).unwrap();
//! 
//!   loop {
//!     for i in 0..256 {
//!       pwm_write(&pwm_pin, i).unwrap();
//!       delay(100);
//!     }    
//!   }
//! }
//! ```

use crate::include::{GpioError, ProgError, PWM_MAP};
use crate::gpio::{Pin, PWM};
use stm32f4::stm32f446::{NVIC, Interrupt, interrupt};
use cortex_m::interrupt::{Mutex, free};
use core::cell::RefCell;
use rtt_target::rprintln;

static TIME_COUNTER: Mutex<RefCell<usize>> = Mutex::new(RefCell::new(0));


// Public PWM Functions ===========================================================================
#[doc(hidden)]
pub fn setup_pwm(pin: (char, u8)) -> Result<(u8, u8, u8), ProgError>{
  let peripheral_ptr;
  unsafe {peripheral_ptr = stm32f4::stm32f446::Peripherals::steal();}
  let rcc = &peripheral_ptr.RCC;

  let (timer, ccch, af) = match check_pwm(pin) {
    Ok(target) => target,
    Err(error) => return Err(error)
  };

  match timer {
    1 => {
      let tim1 = &peripheral_ptr.TIM1;
      rcc.apb2enr.modify(|_, w| w.tim1en().enabled());
      tim1.cr1.modify(|_, w| w.arpe().enabled());
      tim1.psc.write(|w| w.psc().bits(1000));
      tim1.arr.write(|w| w.arr().bits(255));
      tim1.egr.write(|w| w.ug().set_bit());
      match ccch {
        1 => tim1.ccmr1_output().modify(|_, w| { w.oc1pe().enabled(); w.oc1m().pwm_mode1()}),
        2 => tim1.ccmr1_output().modify(|_, w| { w.oc2pe().enabled(); w.oc2m().pwm_mode1()}),
        3 => tim1.ccmr2_output().modify(|_, w| { w.oc3pe().enabled(); w.oc3m().pwm_mode1()}),
        4 => tim1.ccmr2_output().modify(|_, w| { w.oc4pe().enabled(); w.oc4m().pwm_mode1()}),
        _ => unreachable!()
      };
      tim1.ccer.modify(|r, w| unsafe {w.bits(r.bits() | (1 << (4 * (ccch - 1))))});
      tim1.cr1.modify(|_, w| w.cen().enabled());
    },
    2 => {
      let tim2 = &peripheral_ptr.TIM2;
      rcc.apb2enr.modify(|_, w| w.tim1en().enabled());
      tim2.cr1.modify(|_, w| w.arpe().enabled());
      tim2.psc.write(|w| w.psc().bits(1000));
      tim2.arr.write(|w| w.arr().bits(255));
      tim2.egr.write(|w| w.ug().set_bit());
      match ccch {
        1 => tim2.ccmr1_output().modify(|_, w| { w.oc1pe().enabled(); w.oc1m().pwm_mode1()}),
        2 => tim2.ccmr1_output().modify(|_, w| { w.oc2pe().enabled(); w.oc2m().pwm_mode1()}),
        3 => tim2.ccmr2_output().modify(|_, w| { w.oc3pe().enabled(); w.oc3m().pwm_mode1()}),
        4 => tim2.ccmr2_output().modify(|_, w| { w.oc4pe().enabled(); w.oc4m().pwm_mode1()}),
        _ => unreachable!()
      };
      tim2.ccer.modify(|r, w| unsafe {w.bits(r.bits() | (1 << (4 * (ccch - 1))))});
      tim2.cr1.modify(|_, w| w.cen().enabled());
    },
    3 => {
      let tim3 = &peripheral_ptr.TIM3;
      rcc.apb2enr.modify(|_, w| w.tim1en().enabled());
      tim3.cr1.modify(|_, w| w.arpe().enabled());
      tim3.psc.write(|w| w.psc().bits(1000));
      tim3.arr.write(|w| w.arr().bits(255));
      tim3.egr.write(|w| w.ug().set_bit());
      match ccch {
        1 => tim3.ccmr1_output().modify(|_, w| { w.oc1pe().enabled(); w.oc1m().pwm_mode1()}),
        2 => tim3.ccmr1_output().modify(|_, w| { w.oc2pe().enabled(); w.oc2m().pwm_mode1()}),
        3 => tim3.ccmr2_output().modify(|_, w| { w.oc3pe().enabled(); w.oc3m().pwm_mode1()}),
        4 => tim3.ccmr2_output().modify(|_, w| { w.oc4pe().enabled(); w.oc4m().pwm_mode1()}),
        _ => unreachable!()
      };
      tim3.ccer.modify(|r, w| unsafe {w.bits(r.bits() | (1 << (4 * (ccch - 1))))});
      tim3.cr1.modify(|_, w| w.cen().enabled());
    },
    4 => {
      let tim4 = &peripheral_ptr.TIM4;
      rcc.apb2enr.modify(|_, w| w.tim1en().enabled());
      tim4.cr1.modify(|_, w| w.arpe().enabled());
      tim4.psc.write(|w| w.psc().bits(1000));
      tim4.arr.write(|w| w.arr().bits(255));
      tim4.egr.write(|w| w.ug().set_bit());
      match ccch {
        1 => tim4.ccmr1_output().modify(|_, w| { w.oc1pe().enabled(); w.oc1m().pwm_mode1()}),
        2 => tim4.ccmr1_output().modify(|_, w| { w.oc2pe().enabled(); w.oc2m().pwm_mode1()}),
        3 => tim4.ccmr2_output().modify(|_, w| { w.oc3pe().enabled(); w.oc3m().pwm_mode1()}),
        4 => tim4.ccmr2_output().modify(|_, w| { w.oc4pe().enabled(); w.oc4m().pwm_mode1()}),
        _ => unreachable!()
      };
      tim4.ccer.modify(|r, w| unsafe {w.bits(r.bits() | (1 << (4 * (ccch - 1))))});
      tim4.cr1.modify(|_, w| w.cen().enabled());
    },
    _  => unreachable!()
  };

  return Ok((timer, ccch, af));
}

/// Sets the duty cycle of a PWM pin.
/// 
/// Takes pin identifier [A0, C5, etc.](crate::include::pins) and an 8bit value as arguments and sets the duty cycle
/// of the pin.
/// Panics if provided value is out of bounds (<0 or >255).
/// 
/// # Examples
/// 
/// ```no_run
/// // Configure pin as an PWM output
/// let pin = pinmode_pwm(PA8).unwrap();
/// 
/// // Set the duty cycle of the pin
/// let mut value: u16 = pwm_write(&pin, 128).unwrap();
/// ```
pub fn pwm_write(pin: &Pin<PWM>, value: u8) -> Result<(), GpioError> {
  let peripheral_ptr;
  unsafe {peripheral_ptr = stm32f4::stm32f446::Peripherals::steal();}

  match pin.inner.timer {
    1 => {
      let tim1 = &peripheral_ptr.TIM1;
      match pin.inner.ccch {
        1 => tim1.ccr1.write(|w| w.ccr().bits(value.into())),
        2 => tim1.ccr2.write(|w| w.ccr().bits(value.into())),
        3 => tim1.ccr3.write(|w| w.ccr().bits(value.into())),
        4 => tim1.ccr4.write(|w| w.ccr().bits(value.into())),
        _ => unreachable!()
      };
    },
    2 => {
      let tim2 = &peripheral_ptr.TIM2;
      match pin.inner.ccch {
        1 => tim2.ccr1.write(|w| w.ccr().bits(value.into())),
        2 => tim2.ccr2.write(|w| w.ccr().bits(value.into())),
        3 => tim2.ccr3.write(|w| w.ccr().bits(value.into())),
        4 => tim2.ccr4.write(|w| w.ccr().bits(value.into())),
        _ => unreachable!()
      };
    },
    3 => {
      let tim3 = &peripheral_ptr.TIM3;
      match pin.inner.ccch {
        1 => tim3.ccr1.write(|w| w.ccr().bits(value.into())),
        2 => tim3.ccr2.write(|w| w.ccr().bits(value.into())),
        3 => tim3.ccr3.write(|w| w.ccr().bits(value.into())),
        4 => tim3.ccr4.write(|w| w.ccr().bits(value.into())),
        _ => unreachable!()
      };
    },
    4 => {
      let tim4 = &peripheral_ptr.TIM4;
      match pin.inner.ccch {
        1 => tim4.ccr1.write(|w| w.ccr().bits(value.into())),
        2 => tim4.ccr2.write(|w| w.ccr().bits(value.into())),
        3 => tim4.ccr3.write(|w| w.ccr().bits(value.into())),
        4 => tim4.ccr4.write(|w| w.ccr().bits(value.into())),
        _ => unreachable!()
      };
    },
    _ => unreachable!()
  };

  return Ok(());
}


// Private PWM Functions ==========================================================================
fn check_pwm(pin: (char, u8)) -> Result<(u8, u8, u8), ProgError> {
  if !PWM_MAP.pins.contains(&pin) {return Err(ProgError::InvalidConfiguration);}
  else {
    let timer = PWM_MAP.timers[PWM_MAP.pins.iter().position(|&i| i == pin).unwrap()];
    let ccch = PWM_MAP.ccchs[PWM_MAP.pins.iter().position(|&i| i == pin).unwrap()];
    let af = match timer {
      1 => 1,
      2 => 1,
      3 => 2,
      4 => 2,
      _  => unreachable!()
    };

    return Ok((timer, ccch, af));
  }
}


// Public Time Functions ==========================================================================
/// Lets the microcontroller wait for the specified time in milliseconds. In this time no other instructions
/// other than interrupts can be run.
///
/// # Example
///
/// ```rust,no_run
/// use rustuino::*;
///
/// let pin = PA0::output();
///
/// loop {
///   pin.write(true);
///   delay(1000);
///   pin.write(false);
///   delay(1000);
/// }
/// ```
pub fn delay(ms: u16) {
  let peripheral_ptr;
  unsafe {peripheral_ptr = stm32f4::stm32f446::Peripherals::steal();}
  let rcc = &peripheral_ptr.RCC;
  let tim6 = &peripheral_ptr.TIM6;

  if rcc.apb1enr.read().tim6en().is_disabled() {
    rcc.apb1enr.modify(|_, w| w.tim6en().enabled());
    tim6.cr1.modify(|_, w| {
      w.arpe().enabled();
      w.opm().set_bit()
    });

    // 16MHz -> 1MHz : 1000 = 1kHz -> 1ms
    tim6.psc.write(|w| w.psc().bits(16000));
  }

  tim6.arr.write(|w| w.arr().bits(ms));
  tim6.egr.write(|w| w.ug().update());
  tim6.cr1.modify(|_, w| w.cen().enabled());
  while tim6.cr1.read().cen().bit_is_set() {}
}

/// Starts a timer that will continuously count the time in milliseconds.
///
/// This is used for non-blocking delays like [millis] and other time related applications.
///
/// # Example
///
/// ```rust,no_run
/// use rustuino::*;
///
/// let mut counter: usize = 0;
/// let delay: usize = 1000;
/// start_time();
///
/// loop {
///   if counter + delay >= millis() {
///     // Do something
///     counter = millis();
///   }
/// }
/// ```
pub fn start_time() {
  let peripheral_ptr;
  unsafe {peripheral_ptr = stm32f4::stm32f446::Peripherals::steal();}
  let rcc = &peripheral_ptr.RCC;
  let tim7 = &peripheral_ptr.TIM7;

  if rcc.apb1enr.read().tim7en().is_enabled() {
    rprintln!("Millis Timer already configured! | start_time()");
    return;
  }

  rcc.apb1enr.modify(|_, w| w.tim7en().enabled());
  tim7.cr1.modify(|_, w| w.arpe().enabled());

  tim7.dier.modify(|_, w| w.uie().enabled());
  unsafe {NVIC::unmask(Interrupt::TIM7);}

  // 16MHz -> 1MHz : 1000 = 1kHz -> 1ms
  tim7.psc.write(|w| w.psc().bits(16));
  tim7.arr.write(|w| w.arr().bits(1000));
  tim7.egr.write(|w| w.ug().update());
  tim7.cr1.modify(|_, w| w.cen().enabled());
}

/// Non-blocking delay function. Gives back the time in milliseconds since [start_time] was invoked.
///
/// # Example
///
/// ```rust,no_run
/// use rustuino::*;
///
/// let mut counter: usize = 0;
/// let delay: usize = 1000;
/// start_time();
///
/// loop {
///   if counter + delay >= millis() {
///     // Do something
///     counter = millis();
///   }
/// }
/// ```
pub fn millis() -> usize {
  let peripheral_ptr;
  unsafe {peripheral_ptr = stm32f4::stm32f446::Peripherals::steal();}
  let tim7 = &peripheral_ptr.TIM6;

  let buffer: usize;

  tim7.cr1.modify(|_, w| w.cen().disabled());
  buffer = free(|cs| *TIME_COUNTER.borrow(cs).borrow());
  tim7.cr1.modify(|_, w| w.cen().enabled());

  return buffer;
}


// Interrupts =====================================================================================
#[allow(non_snake_case)]
#[interrupt]
fn TIM7() {
  free(|cs| TIME_COUNTER.borrow(cs).replace_with(|&mut i| i + 1));
}
