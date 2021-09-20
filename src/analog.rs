//! This module contains everything that is related to the analog IO functionality.

use crate::include::pins::*;
use crate::gpio::{GpioMode, return_pinmode};
use crate::include::{GpioError, ProgError};


// Public Functions ===============================================================================
pub fn enable_channel(pin: (char, u8)) -> Result<(), GpioError> {
  let peripheral_ptr;
  unsafe {peripheral_ptr = stm32f4::stm32f446::Peripherals::steal();}
  let rcc = &peripheral_ptr.RCC;
  let adcc = &peripheral_ptr.ADC_COMMON;

  let (core, channel) = match check_channel(pin, true, true) {
    Ok(values) => values,
    Err(error) => {
      rtt_target::rprintln!("P{}{} is not available for analog functions! | enable_channel()", pin.0.to_uppercase(), pin.1);
      return Err(error);
    }
  };

  match core {
    0 => {
      let dac = &peripheral_ptr.DAC;
      rcc.apb1enr.modify(|_, w| w.dacen().enabled());
      if channel == 1 {
        dac.cr.modify(|_, w| {
          w.boff1().enabled();
          w.ten1().enabled();
          w.tsel1().software();
          w.en1().enabled()
        });
      }
      else {
        dac.cr.modify(|_, w| {
          w.boff2().enabled();
          w.ten2().enabled();
          w.tsel2().software();
          w.en2().enabled()
        });
      }

      start_dac_timer();
    },
    1 => {
      let adc1 = &peripheral_ptr.ADC1;
      if rcc.apb2enr.read().adc1en().is_disabled() == true {
        rcc.apb2enr.modify(|_, w| w.adc1en().enabled());
        adcc.ccr.modify(|_, w| w.adcpre().div2());
        adc1.smpr2.modify(|_, w| w.smp0().cycles144());
        adc1.cr1.modify(|_, w| w.res().ten_bit());
        adc1.cr2.modify(|_, w| w.adon().enabled());
      }
    },
    2 => {
      let adc2 = &peripheral_ptr.ADC2;
      if rcc.apb2enr.read().adc2en().is_disabled() == true {
        rcc.apb2enr.modify(|_, w| w.adc2en().enabled());
        adcc.ccr.modify(|_, w| w.adcpre().div2());
        adc2.smpr2.modify(|_, w| w.smp0().cycles144());
        adc2.cr1.modify(|_, w| w.res().ten_bit());
        adc2.cr2.modify(|_, w| w.adon().enabled());
      } 
    },
    3 => {
      let adc3 = &peripheral_ptr.ADC3;
      if rcc.apb2enr.read().adc3en().is_disabled() == true {
        rcc.apb2enr.modify(|_, w| w.adc3en().enabled());
        adcc.ccr.modify(|_, w| w.adcpre().div2());
        adc3.smpr2.modify(|_, w| w.smp0().cycles144());
        adc3.cr1.modify(|_, w| w.res().ten_bit());
        adc3.cr2.modify(|_, w| w.adon().enabled());
      }
    },
    _ => {
      rtt_target::rprintln!("ADC{} is not a valid ADC peripheral! | enable_channel()", core);
      return Err(GpioError::ConfigurationError);
    }
  };

  return Ok(());
}

pub fn adc_resolution(pin: (char, u8), res: u8) -> Result<(), GpioError> {
  let peripheral_ptr;
  unsafe {peripheral_ptr = stm32f4::stm32f446::Peripherals::steal();}

  let enc_res = match res {
    6  => 3,
    8  => 2,
    10 => 1,
    12 => 0,
    _ => {
      rtt_target::rprintln!("{} is not a available ADC resolution! | adc_resolution()", res);
      return Err(GpioError::Prog(ProgError::InvalidArguments));
    }
  };

  match check_channel(pin, true, false) {
    Ok(target) => {
      match target.0 {
        1 => {
          let adc1 = &peripheral_ptr.ADC1;
          adc1.cr1.modify(|_, w| w.res().bits(enc_res));
        },
        2 => {
          let adc2 = &peripheral_ptr.ADC2;
          adc2.cr1.modify(|_, w| w.res().bits(enc_res));
        },
        3 => {
          let adc3 = &peripheral_ptr.ADC3;
          adc3.cr1.modify(|_, w| w.res().bits(enc_res));
        },
        _ => {
          rtt_target::rprintln!("ADC{} is not a valid ADC peripheral! | adc_resolution()", target.0);
          return Err(GpioError::ConfigurationError);
        }
      };
    },
    Err(error) => return Err(error)
  };

  return Ok(());
}

pub fn analog_read(pin: (char, u8)) -> Result<u16, GpioError> {
  let peripheral_ptr;
  unsafe {peripheral_ptr = stm32f4::stm32f446::Peripherals::steal();}

  let target = match check_channel(pin, true, false) {
    Ok(p) => p,
    Err(error) => return Err(error)
  };

  if return_pinmode(pin) != Ok(GpioMode::Analog) {
    rtt_target::rprintln!("P{}{} is not configured as analog! | analog_read()", pin.0.to_uppercase(), pin.1);
    return Err(GpioError::WrongMode);
  }

  let buffer = match target.0 {
    1 => {
      let adc1 = &peripheral_ptr.ADC1;
      if adc1.cr2.read().adon().is_disabled() == true {
        rtt_target::rprintln!("ADC{} not configured! | analog_read()", target.0);
        return Err(GpioError::Prog(ProgError::NotConfigured));
      }
      adc1.sqr3.modify(|_, w| unsafe {w.sq1().bits(target.1)});
      adc1.cr2.write(|w| w.swstart().start());
      while adc1.sr.read().eoc().is_not_complete() == true {}
      adc1.dr.read().data().bits()
    },
    2 => {
      let adc2 = &peripheral_ptr.ADC2;
      if adc2.cr2.read().adon().is_disabled() == true {
        rtt_target::rprintln!("ADC{} not configured! | analog_read()", target.0);
        return Err(GpioError::Prog(ProgError::NotConfigured));
      }      adc2.sqr3.modify(|_, w| unsafe {w.sq1().bits(target.1)});
      adc2.cr2.write(|w| w.swstart().start());
      while adc2.sr.read().eoc().is_not_complete() == true {}
      adc2.dr.read().data().bits()
    },
    3 => {
      let adc3 = &peripheral_ptr.ADC3;
      if adc3.cr2.read().adon().is_disabled() == true {
        rtt_target::rprintln!("ADC{} not configured! | analog_read()", target.0);
        return Err(GpioError::Prog(ProgError::NotConfigured));
      }      adc3.sqr3.modify(|_, w| unsafe {w.sq1().bits(target.1)});
      adc3.cr2.write(|w| w.swstart().start());
      while adc3.sr.read().eoc().is_not_complete() == true {}
      adc3.dr.read().data().bits()
    },
    _ => {
      rtt_target::rprintln!("ADC{} is not a valid ADC peripheral! | analog_read()", target.0);
      return Err(GpioError::ConfigurationError);
    }
  };

  return Ok(buffer);
}

pub fn analog_write(pin: (char, u8), value: u16) -> Result<(), GpioError> {
  let peripheral_ptr;
  unsafe {peripheral_ptr = stm32f4::stm32f446::Peripherals::steal();}
  let dac = &peripheral_ptr.DAC;

  let val = if value > 4095 {
    rtt_target::rprintln!("Analog value outside of bounds! | analog_write()");
    4095
  }
  else {value};

  let target = match check_channel(pin, false, true) {
    Ok(p) => p,
    Err(error) => return Err(error)
  };

  if return_pinmode(pin) != Ok(GpioMode::Analog) {
    rtt_target::rprintln!("P{}{} is not configured as analog! | analog_write()", pin.0.to_uppercase(), pin.1);
    return Err(GpioError::WrongMode);
  }

  if target.1 == 1 {
    if dac.cr.read().wave1().is_disabled() == false {
      dac.cr.modify(|_, w| {
      w.tsel1().software();
      w.wave1().disabled()
      });
    }
    dac.dhr12r1.write(|w| w.dacc1dhr().bits(val));
    dac.swtrigr.write(|w| w.swtrig1().enabled());
  }
  else {
    if dac.cr.read().wave2().is_disabled() == false {
      dac.cr.modify(|_, w| {
      w.tsel2().software();
      w.wave2().disabled()
      });
    }
    dac.dhr12r2.write(|w| w.dacc2dhr().bits(val));
    dac.swtrigr.write(|w| w.swtrig2().enabled());
  }

  return Ok(());
}

pub fn analog_write_noise(pin: (char, u8), level: u8) -> Result<(), GpioError> {
  let peripheral_ptr;
  unsafe {peripheral_ptr = stm32f4::stm32f446::Peripherals::steal();}
  let dac = &peripheral_ptr.DAC;

  let lvl = if level > 15 {
    rtt_target::rprintln!("DAC level value outside of bounds! | analog_write_noise()");
    15
  }
  else {level};

  let target = match check_channel(pin, false, true) {
    Ok(p) => p,
    Err(error) => return Err(error)
  };

  if return_pinmode(pin) != Ok(GpioMode::Analog) {
    rtt_target::rprintln!("P{}{} is not configured as analog! | analog_write_noise()", pin.0.to_uppercase(), pin.1);
    return Err(GpioError::WrongMode);
  }

  if target.1 == 1 {
    dac.cr.modify(|_, w| {
      w.ten1().disabled();
      w.wave1().noise();
      unsafe {w.tsel1().bits(0x011);}
      w.mamp1().bits(lvl);
      w.ten1().enabled()
    });
  }
  else {
    dac.cr.modify(|_, w| {
      w.ten2().disabled();
      w.wave2().noise();
      w.tsel2().bits(0x011);
      w.mamp2().bits(lvl);
      w.ten2().enabled()
    });
  }

  return Ok(());
}

pub fn analog_write_triangle(pin: (char, u8), level: u8) -> Result<(), GpioError> {
  let peripheral_ptr;
  unsafe {peripheral_ptr = stm32f4::stm32f446::Peripherals::steal();}
  let dac = &peripheral_ptr.DAC;

  let lvl = if level > 15 {
    rtt_target::rprintln!("DAC level value outside of bounds! | analog_write_triangle()");
    15
  }
  else {level};

  let target = match check_channel(pin, false, true) {
    Ok(p) => p,
    Err(error) => return Err(error)
  };

  if return_pinmode(pin) != Ok(GpioMode::Analog) {
    rtt_target::rprintln!("P{}{} is not configured as analog! | analog_write_triangle()", pin.0.to_uppercase(), pin.1);
    return Err(GpioError::WrongMode);
  }

  if target.1 == 1 {
    dac.cr.modify(|_, w| {
      w.ten1().disabled();
      w.wave1().triangle();
      unsafe {w.tsel1().bits(0x011);}
      w.mamp1().bits(lvl);
      w.ten1().enabled()
    });
  }
  else {
    dac.cr.modify(|_, w| {
      w.ten2().disabled();
      w.wave2().triangle();
      w.tsel2().bits(0x011);
      w.mamp2().bits(lvl);
      w.ten2().enabled()
    });
  }

  return Ok(());
}

pub fn analog_wave_freq(freq: u32) {
  let peripheral_ptr;
  unsafe {peripheral_ptr = stm32f4::stm32f446::Peripherals::steal();}
  let tim5 = &peripheral_ptr.TIM5;

  // Max. 16MHz -> arr = 16000000 / freq
  let val = if freq > 16000000 {
    rtt_target::rprintln!("Outside limits of internal clock! | analog_wave_freq()");
    1
  }
  else {16000000 / freq};

  tim5.arr.write(|w| w.arr().bits(val.into()));
}


// Private Functions ==============================================================================
fn check_channel(pin: (char, u8), adc: bool, dac: bool) -> Result<(u8, u8), GpioError> {
  const PINS: [(char, u8); 16] = [A0, A1, A2, A3, A4, A5, A6, A7, B0, B1, C0, C1, C2, C3, C4, C5];
  const CORES: [u8; 16]        = [1,  1,  1,  1,  0,  0,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1];
  const CHANNELS: [u8; 16]     = [0,  1,  2,  3,  1,  2,  6,  7,  8,  9,  10, 11, 12, 13, 14, 15];

  if PINS.contains(&pin) == false {return Err(GpioError::Prog(ProgError::InvalidArguments));}
  else {
    let core = CORES[PINS.iter().position(|&i| i == pin).unwrap()];
    let channel = CHANNELS[PINS.iter().position(|&i| i == pin).unwrap()];

    if dac == false && core == 0 {return Err(GpioError::Prog(ProgError::InvalidArguments));}
    else if adc == false && core != 0 {return Err(GpioError::Prog(ProgError::InvalidArguments));}
    else {return Ok((core, channel));}
  }
}

fn start_dac_timer() {
  let peripheral_ptr;
  unsafe {peripheral_ptr = stm32f4::stm32f446::Peripherals::steal();}
  let rcc = &peripheral_ptr.RCC;
  let tim5 = &peripheral_ptr.TIM5;

  if rcc.apb1enr.read().tim5en().is_enabled() == true {return;}
  
  rcc.apb1enr.modify(|_, w| w.tim5en().enabled());
  tim5.cr1.modify(|_, w| w.arpe().enabled());
  tim5.psc.write(|w| w.psc().bits(1));
  tim5.arr.write(|w| w.arr().bits(16000000 / 1000));
  tim5.egr.write(|w| w.ug().update());
  tim5.cr2.modify(|_, w| w.mms().update());
  tim5.cr1.modify(|_, w| w.cen().enabled());
}
