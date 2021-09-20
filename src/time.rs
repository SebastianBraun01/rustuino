//! This module contains everything that is related to timer based functions.

use crate::gpio::pins::*;
use crate::gpio::{GpioError, pin_mode, GpioMode, return_pinmode};
use stm32f4::stm32f446::{NVIC, Interrupt, interrupt};


// Public PWM Functions ===========================================================================
pub fn setup_pwm(pin: (char, u8)) -> Result<(), GpioError>{
  let peripheral_ptr;
  unsafe {peripheral_ptr = stm32f4::stm32f446::Peripherals::steal();}
  let rcc = &peripheral_ptr.RCC;

  let (timer, ccch, af) = match check_pwm(pin) {
    Ok(target) => target,
    Err(error) => return Err(error)
  };

  if let Err(error) = pin_mode(pin, GpioMode::AlternateFunction(af.into())) {return Err(error);}

  match timer {
    1 => {
      let tim1 = &peripheral_ptr.TIM1;
      rcc.apb2enr.modify(|_, w| w.tim1en().enabled());
      tim1.cr1.modify(|_, w| w.arpe().enabled());
      tim1.psc.write(|w| w.psc().bits(1000));
      tim1.arr.write(|w| w.arr().bits(255));
      tim1.egr.write(|w| w.ug().set_bit());
      match ccch {
        1 => tim1.ccmr1_output_mut().modify(|_, w| { w.oc1pe().enabled(); w.oc1m().pwm_mode1()}),
        2 => tim1.ccmr1_output_mut().modify(|_, w| { w.oc2pe().enabled(); w.oc2m().pwm_mode1()}),
        3 => tim1.ccmr2_output_mut().modify(|_, w| { w.oc3pe().enabled(); w.oc3m().pwm_mode1()}),
        4 => tim1.ccmr2_output_mut().modify(|_, w| { w.oc4pe().enabled(); w.oc4m().pwm_mode1()}),
        _ => {
          rtt_target::rprintln!("Channel{} is not a valid CC channel! | setup_pwm()", ccch);
          return Err(GpioError::ConfigurationError);
        }
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
        1 => tim2.ccmr1_output_mut().modify(|_, w| { w.oc1pe().enabled(); w.oc1m().pwm_mode1()}),
        2 => tim2.ccmr1_output_mut().modify(|_, w| { w.oc2pe().enabled(); w.oc2m().pwm_mode1()}),
        3 => tim2.ccmr2_output_mut().modify(|_, w| { w.oc3pe().enabled(); w.oc3m().pwm_mode1()}),
        4 => tim2.ccmr2_output_mut().modify(|_, w| { w.oc4pe().enabled(); w.oc4m().pwm_mode1()}),
        _ => {
          rtt_target::rprintln!("Channel{} is not a valid CC channel! | setup_pwm()", ccch);
          return Err(GpioError::ConfigurationError);
        }
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
        1 => tim3.ccmr1_output_mut().modify(|_, w| { w.oc1pe().enabled(); w.oc1m().pwm_mode1()}),
        2 => tim3.ccmr1_output_mut().modify(|_, w| { w.oc2pe().enabled(); w.oc2m().pwm_mode1()}),
        3 => tim3.ccmr2_output_mut().modify(|_, w| { w.oc3pe().enabled(); w.oc3m().pwm_mode1()}),
        4 => tim3.ccmr2_output_mut().modify(|_, w| { w.oc4pe().enabled(); w.oc4m().pwm_mode1()}),
        _ => {
          rtt_target::rprintln!("Channel{} is not a valid CC channel! | setup_pwm()", ccch);
          return Err(GpioError::ConfigurationError);
        }
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
        1 => tim4.ccmr1_output_mut().modify(|_, w| { w.oc1pe().enabled(); w.oc1m().pwm_mode1()}),
        2 => tim4.ccmr1_output_mut().modify(|_, w| { w.oc2pe().enabled(); w.oc2m().pwm_mode1()}),
        3 => tim4.ccmr2_output_mut().modify(|_, w| { w.oc3pe().enabled(); w.oc3m().pwm_mode1()}),
        4 => tim4.ccmr2_output_mut().modify(|_, w| { w.oc4pe().enabled(); w.oc4m().pwm_mode1()}),
        _ => {
          rtt_target::rprintln!("Channel{} is not a valid CC channel! | setup_pwm()", ccch);
          return Err(GpioError::ConfigurationError);
        }
      };
      tim4.ccer.modify(|r, w| unsafe {w.bits(r.bits() | (1 << (4 * (ccch - 1))))});
      tim4.cr1.modify(|_, w| w.cen().enabled());
    },
    _  => {
      rtt_target::rprintln!("Timer{} is not a valid timer! | setup_pwm()", timer);
      return Err(GpioError::ConfigurationError);
    }
  };

  return Ok(());
}

pub fn pwm_write(pin: (char, u8), value: u8) -> Result<(), GpioError> {
  let peripheral_ptr;
  unsafe {peripheral_ptr = stm32f4::stm32f446::Peripherals::steal();}

  let (timer, ccch, af) = match check_pwm(pin) {
    Ok(target) => target,
    Err(error) => return Err(error)
  };

  if return_pinmode(pin) != GpioMode::AlternateFunction(af.into()) {
    rtt_target::rprintln!("P{}{} is not configured for pwm output! | pwm_write()", pin.0.to_uppercase(), pin.1);
    return Err(GpioError::WrongMode);
  }

  match timer {
    1 => {
      let tim1 = &peripheral_ptr.TIM1;
      match ccch {
        1 => tim1.ccr1.write(|w| w.ccr().bits(value.into())),
        2 => tim1.ccr2.write(|w| w.ccr().bits(value.into())),
        3 => tim1.ccr3.write(|w| w.ccr().bits(value.into())),
        4 => tim1.ccr4.write(|w| w.ccr().bits(value.into())),
        _ => {
          rtt_target::rprintln!("Channel{} is not a valid CC channel! | pwm_write()", ccch);
          return Err(GpioError::ConfigurationError);
        }
      };
    },
    2 => {
      let tim2 = &peripheral_ptr.TIM2;
      match ccch {
        1 => tim2.ccr1.write(|w| w.ccr().bits(value.into())),
        2 => tim2.ccr2.write(|w| w.ccr().bits(value.into())),
        3 => tim2.ccr3.write(|w| w.ccr().bits(value.into())),
        4 => tim2.ccr4.write(|w| w.ccr().bits(value.into())),
        _ => {
          rtt_target::rprintln!("Channel{} is not a valid CC channel! | pwm_write()", ccch);
          return Err(GpioError::ConfigurationError);
        }
      };
    },
    3 => {
      let tim3 = &peripheral_ptr.TIM3;
      match ccch {
        1 => tim3.ccr1.write(|w| w.ccr().bits(value.into())),
        2 => tim3.ccr2.write(|w| w.ccr().bits(value.into())),
        3 => tim3.ccr3.write(|w| w.ccr().bits(value.into())),
        4 => tim3.ccr4.write(|w| w.ccr().bits(value.into())),
        _ => {
          rtt_target::rprintln!("Channel{} is not a valid CC channel! | pwm_write()", ccch);
          return Err(GpioError::ConfigurationError);
        }
      };
    },
    4 => {
      let tim4 = &peripheral_ptr.TIM4;
      match ccch {
        1 => tim4.ccr1.write(|w| w.ccr().bits(value.into())),
        2 => tim4.ccr2.write(|w| w.ccr().bits(value.into())),
        3 => tim4.ccr3.write(|w| w.ccr().bits(value.into())),
        4 => tim4.ccr4.write(|w| w.ccr().bits(value.into())),
        _ => {
          rtt_target::rprintln!("Channel{} is not a valid CC channel! | pwm_write()", ccch);
          return Err(GpioError::ConfigurationError);
        }
      };
    },
    _ => {
      rtt_target::rprintln!("Timer{} is not a valid timer! | pwm_write()", timer);
      return Err(GpioError::ConfigurationError);
    }
  };

  return Ok(());
}


// Private PWM Functions ==========================================================================
fn check_pwm(pin: (char, u8)) -> Result<(u8, u8, u8), GpioError> {
  const PINS: [(char, u8); 31] = [A0, A1, A2, A3, A5, A8, A9, A10, A11, A15, B0, B1, B2, B3, B8, B9, B10, B11, B13, B14, B15, A6, A7, B4, B5, B6, B7, C6, C7, C8, C9];
  const TIMERS: [u8; 31] = [2, 2, 2, 2, 2, 1, 1, 1, 1, 2, 1, 1, 2, 2, 2, 2, 2, 2, 1, 1, 1, 3, 3, 3, 3, 4, 4, 3, 3, 3, 3];
  const CCCHS: [u8; 31] = [1, 2, 3, 4, 1, 1, 2, 3, 4, 1, 2, 3, 4, 2, 1, 2, 3, 4, 1, 2, 3, 1, 2, 1, 2, 1, 2, 1, 2, 3, 4];

  if PINS.contains(&pin) == false {return Err(GpioError::NoPinForFunction);}
  else {
    let timer = TIMERS[PINS.iter().position(|&i| i == pin).unwrap()];
    let ccch = CCCHS[PINS.iter().position(|&i| i == pin).unwrap()];
    let af = match timer {
      1 => 1,
      2 => 1,
      3 => 2,
      4 => 2,
      _  => {
        rtt_target::rprintln!("Timer{} is not a valid timer! | check_pwm()", timer);
        return Err(GpioError::ConfigurationError);
      }
    };

    return Ok((timer, ccch, af));
  }
}


// Standalone time functions ======================================================================
static mut DELAY_COUNTER: (bool, u32) = (false, 0);
static mut TIME_COUNTER: usize = 0;

/// Lets the microcontroller wait for the specified time in milliseconds. In this time no other instructions can be run.
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
pub fn delay(ms: u32) {
  let peripheral_ptr;
  unsafe {peripheral_ptr = stm32f4::stm32f446::Peripherals::steal();}
  let rcc = &peripheral_ptr.RCC;
  let tim6 = &peripheral_ptr.TIM6;

  if rcc.apb1enr.read().tim6en().is_disabled() == true {
    rcc.apb1enr.modify(|_, w| w.tim6en().enabled());
    tim6.cr1.modify(|_, w| w.arpe().enabled());

    tim6.dier.modify(|_, w| w.uie().enabled());
    unsafe {NVIC::unmask(Interrupt::TIM6_DAC);}

    // 16MHz -> 1MHz : 1000 = 1kHz -> 1ms
    tim6.psc.write(|w| w.psc().bits(16));
    tim6.arr.write(|w| w.arr().bits(1000));
    tim6.egr.write(|w| w.ug().update());
    tim6.cr1.modify(|_, w| w.cen().enabled());
  }
  else {tim6.cr1.modify(|_, w| w.cen().enabled());}

  unsafe {
    DELAY_COUNTER.1 = 0;
    DELAY_COUNTER.0 = true;
    while DELAY_COUNTER.1 < ms {}
    DELAY_COUNTER.0 = false;
  }

  tim6.cr1.modify(|_, w| w.cen().disabled());
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

  if rcc.apb1enr.read().tim7en().is_enabled() == true {
    rtt_target::rprintln!("Millis Timer already configured! | start_time()");
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
  unsafe {buffer = TIME_COUNTER;}
  tim7.cr1.modify(|_, w| w.cen().enabled());

  return buffer;
}


// Interrupts =====================================================================================
#[allow(non_snake_case)]
#[interrupt]
fn TIM6_DAC() {
  unsafe {
    if DELAY_COUNTER.0 == true {DELAY_COUNTER.1 += 1;}
    rtt_target::rprintln!("Delay-counter: {}", DELAY_COUNTER.1);
  }
}

#[allow(non_snake_case)]
#[interrupt]
fn TIM7() {
  unsafe {TIME_COUNTER += 1;}
}

