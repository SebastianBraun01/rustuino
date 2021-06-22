use super::include::pins::*;
use super::include::data_maps::{PINCONFIG, ADC_MAP, DAC_MAP};
use super::include::PERIPHERAL_PTR;

pub fn adc_init(resolution: u8, eocint: bool) {
  let rcc = &PERIPHERAL_PTR.RCC;
  let adcc = &PERIPHERAL_PTR.ADC_COMMON;
  let adc1 = &PERIPHERAL_PTR.ADC1;

  unsafe {
    if ADC_MAP.active.contains(&true) {panic!("ADC is already PINCONFIGured!");}
    for i in 0..PINCONFIG.analog.len() {
      if PINCONFIG.analog[i] == 1 {
        // Check if pin is available for adc connection
        if ADC_MAP.pin.contains(&PINCONFIG.pin[i]) {
          ADC_MAP.active[ADC_MAP.pin.iter().position(|&r| r == PINCONFIG.pin[i]).unwrap()] = true;
        }
        else {
          panic!("P{}{} is not available for analog conversion!", PINCONFIG.pin[i].1.to_uppercase(), PINCONFIG.pin[i].0);
        }
      }
    }
  }

  rcc.apb2enr.modify(|_, w| w.adc1en().enabled());
  adcc.ccr.modify(|_, w| w.adcpre().div2());
  adc1.smpr2.modify(|_, w| w.smp0().cycles144());

  if eocint == true {adc1.cr1.modify(|_, w| w.eocie().enabled());}

  match resolution {
    12 => adc1.cr1.modify(|_, w| w.res().twelve_bit()),
    10 => adc1.cr1.modify(|_, w| w.res().ten_bit()),
    8  => adc1.cr1.modify(|_, w| w.res().eight_bit()),
    6  => adc1.cr1.modify(|_, w| w.res().six_bit()),
    _  => panic!("{} is not a valid ADC resolution!", resolution)
  };

  adc1.cr2.modify(|_, w| w.adon().enabled());
}

pub fn analog_read(pin: (u8, char)) -> u16 {
  let adc1 = &PERIPHERAL_PTR.ADC1;
  let buffer: u16;

  unsafe {
    if ADC_MAP.pin.contains(&pin) {
      if ADC_MAP.active[ADC_MAP.pin.iter().position(|&i| i == pin).unwrap()] == false {
        let channel = ADC_MAP.channel[ADC_MAP.pin.iter().position(|&i| i == pin).unwrap()];
        adc1.sqr3.modify(|_, w| w.sq1().bits(channel));

        adc1.cr2.write(|w| w.swstart().start());
        while adc1.sr.read().eoc().is_not_complete() == true {}
        buffer = adc1.dr.read().data().bits();
      }
      else {
        panic!("P{}{} was not initialized for conversion!", pin.1.to_uppercase(), pin.0);
      }
    }
    else {panic!("P{}{} is not available for analog conversion!", pin.1.to_uppercase(), pin.0);}
  }

  return buffer;
}

pub fn dac_init() {
  let rcc = &PERIPHERAL_PTR.RCC;
  let dac = &PERIPHERAL_PTR.DAC;

  unsafe {
    for i in 0..PINCONFIG.analog.len() {
      if PINCONFIG.analog[i] == 2 {
        // Check if pin is available for adc connection
        if PINCONFIG.pin[i] == PA4 {DAC_MAP.0 = true;}
        else if PINCONFIG.pin[i] == PA5 {DAC_MAP.1 = true;}
        else {panic!("P{}{} is not available for analog conversion!", PINCONFIG.pin[i].1.to_uppercase(), PINCONFIG.pin[i].0);}
      }
    }

    rcc.apb1enr.modify(|_, w| w.dacen().enabled());

    if DAC_MAP.0 == true {
      dac.cr.modify(|_, w| {
        w.boff1().enabled();
        w.ten1().enabled();
        w.tsel1().software();
        w.en1().enabled()
      });
    }

    if DAC_MAP.1 == true {
      dac.cr.modify(|_, w| {
        w.boff2().enabled();
        w.ten2().enabled();
        w.tsel2().software();
        w.en2().enabled()
      });
    }
  }
}

pub fn analog_write(pin: (u8, char), value: u16) {
  let dac = &PERIPHERAL_PTR.DAC;

  if value > 4095 {panic!("DAC value outside of bounds!");}

  if pin == PA4 {
    dac.dhr12r1.write(|w| w.dacc1dhr().bits(value));
    dac.swtrigr.write(|w| w.swtrig1().enabled());
  }
  else if pin == PA5 {
    dac.dhr12r2.write(|w| w.dacc2dhr().bits(value));
    dac.swtrigr.write(|w| w.swtrig2().enabled());
  }
  else {panic!("P{}{} is not available for analog conversion!", pin.1.to_uppercase(), pin.0);}
}

impl<const B: char, const P: u8, const M: u8> Analog for AnalogPin<GpioPin<B, P, M>> {
  fn analog_read(&self) -> u16 {
    let block = B;
    let pin = P;

    let buffer = if block == 'f' {
      let adc3 = &PERIPHERAL_PTR.ADC3;
      let channel = ADC3_MAP.channel[ADC3_MAP.pin.iter().position(|&i| i == pin).unwrap()];
      adc3.sqr3.modify(|_, w| unsafe {w.sq1().bits(channel)});

      adc3.cr2.write(|w| w.swstart().start());
      while adc3.sr.read().eoc().is_not_complete() == true {}
      adc3.dr.read().data().bits() as u16
    }
    else {
      let adc1 = &PERIPHERAL_PTR.ADC1;
      let channel = ADC1_MAP.channel[ADC1_MAP.pin.iter().position(|&i| i == (block, pin)).unwrap()];
      adc1.sqr3.modify(|_, w| unsafe {w.sq1().bits(channel)});
      adc1.cr2.write(|w| w.swstart().start());
      while adc1.sr.read().eoc().is_not_complete() == true {}
      adc1.dr.read().data().bits() as u16
    };

    return buffer;
  }
}

impl<const B: char, const P: u8> AnalogConfig for GpioPin<B, P, 3> {
  fn set_as_analog(self) -> AnalogPin<Self> {
    unimplemented!();
  }
}
