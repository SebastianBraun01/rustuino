//! Contains everything that is related to the analog IO functionality.

use crate::include::{GpioError, ProgError, ADC_MAP};
use crate::gpio::{Pin, AnalogIn, AnalogOut};
use rtt_target::rprintln;


#[doc(hidden)]
pub fn enable_channel(pin: (char, u8), dma: bool) -> Result<(u8, u8), ProgError> {
  let peripheral_ptr;
  unsafe {peripheral_ptr = stm32f4::stm32f446::Peripherals::steal();}
  let rcc = &peripheral_ptr.RCC;
  let adcc = &peripheral_ptr.ADC_COMMON;

  let (core, channel) = match check_channel(pin, !dma, dma) {
    Ok(values) => values,
    Err(error) => {
      rprintln!("P{}{} is not available for analog functions! | enable_channel()", pin.0.to_uppercase(), pin.1);
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
      return Ok((core, channel));
    },
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

pub fn adc_resolution(res: u8) {
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
      rprintln!("{} is not a available ADC resolution! Keep default (10) | adc_resolution()", res);
      1
    }
  };

  if rcc.apb2enr.read().adc1en().is_enabled() {adc1.cr1.modify(|_, w| w.res().bits(enc_res));}
  if rcc.apb2enr.read().adc2en().is_enabled() {adc2.cr1.modify(|_, w| w.res().bits(enc_res));}
  if rcc.apb2enr.read().adc3en().is_enabled() {adc3.cr1.modify(|_, w| w.res().bits(enc_res));}
}

pub fn analog_read(pin: &Pin<AnalogIn>) -> u16 {
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

pub fn analog_write(pin: &Pin<AnalogOut>, value: u16) -> Result<(), GpioError> {
  let peripheral_ptr;
  unsafe {peripheral_ptr = stm32f4::stm32f446::Peripherals::steal();}
  let dac = &peripheral_ptr.DAC;

  if pin.inner.core != 0 {
    rprintln!("Analog write not available for inputs! | analog_write()");
    return Err(GpioError::WrongMode);
  }

  let val = if value > 4095 {
    rprintln!("Analog value outside of bounds! | analog_write()");
    4095
  }
  else {value};

  if pin.inner.channel == 1 {
    if !dac.cr.read().wave1().is_disabled() {
      dac.cr.modify(|_, w| {
      w.tsel1().software();
      w.wave1().disabled()
      });
    }
    dac.dhr12r1.write(|w| w.dacc1dhr().bits(val));
    dac.swtrigr.write(|w| w.swtrig1().enabled());
  }
  else {
    if !dac.cr.read().wave2().is_disabled() {
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

pub fn analog_write_noise(pin: &Pin<AnalogOut>, level: u8) -> Result<(), GpioError> {
  let peripheral_ptr;
  unsafe {peripheral_ptr = stm32f4::stm32f446::Peripherals::steal();}
  let dac = &peripheral_ptr.DAC;

  if pin.inner.core != 0 {
    rprintln!("Analog write not available for inputs! | analog_write()");
    return Err(GpioError::WrongMode);
  }


  let lvl = if level > 15 {
    rprintln!("DAC level value outside of bounds! | analog_write_noise()");
    15
  }
  else {level};
  
  if pin.inner.channel == 1 {
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

pub fn analog_write_triangle(pin: &Pin<AnalogOut>, level: u8) -> Result<(), GpioError> {
  let peripheral_ptr;
  unsafe {peripheral_ptr = stm32f4::stm32f446::Peripherals::steal();}
  let dac = &peripheral_ptr.DAC;

  if pin.inner.core != 0 {
    rprintln!("Analog write not available for inputs! | analog_write()");
    return Err(GpioError::WrongMode);
  }

  let lvl = if level > 15 {
    rprintln!("DAC level value outside of bounds! | analog_write_triangle()");
    15
  }
  else {level};

  if pin.inner.channel == 1 {
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
    rprintln!("Outside limits of internal clock! | analog_wave_freq()");
    1
  }
  else {16000000 / freq};

  tim5.arr.write(|w| w.arr().bits(val));
}


// Private Functions ==============================================================================
fn check_channel(pin: (char, u8), adc: bool, dac: bool) -> Result<(u8, u8), ProgError> {
  if !ADC_MAP.pins.contains(&pin) {return Err(ProgError::InvalidConfiguration);}
  else {
    let core = ADC_MAP.adcs[ADC_MAP.pins.iter().position(|&i| i == pin).unwrap()];
    let channel = ADC_MAP.channels[ADC_MAP.pins.iter().position(|&i| i == pin).unwrap()];

    if (!dac && core == 0) || (!adc && core != 0) {return Err(ProgError::InvalidConfiguration);}
    else {return Ok((core, channel));}
  }
}

fn start_dac_timer() {
  let peripheral_ptr;
  unsafe {peripheral_ptr = stm32f4::stm32f446::Peripherals::steal();}
  let rcc = &peripheral_ptr.RCC;
  let tim5 = &peripheral_ptr.TIM5;

  if rcc.apb1enr.read().tim5en().is_enabled() {return;}
  
  rcc.apb1enr.modify(|_, w| w.tim5en().enabled());
  tim5.cr1.modify(|_, w| w.arpe().enabled());
  tim5.psc.write(|w| w.psc().bits(1));
  tim5.arr.write(|w| w.arr().bits(16000000 / 1000));
  tim5.egr.write(|w| w.ug().update());
  tim5.cr2.modify(|_, w| w.mms().update());
  tim5.cr1.modify(|_, w| w.cen().enabled());
}
