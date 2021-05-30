use super::include::pins::*;
use super::include::data_maps::{PINCONFIG, ADC_MAP, DAC_MAP};
use super::include::register::{RCC_PTR, DAC_PTR, ADCC_PTR, ADC1_PTR};

pub fn adc_init(resolution: u8, eocint: bool) {
  unsafe {
    if ADC_MAP.active.contains(&true) {panic!("ADC is already PINCONFIGured!");}

    for i in 0..PINPINCONFIG.analog.len() {
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
    
    (*RCC_PTR).apb2enr.modify(|_, w| w.adc1en().enabled());
    (*ADCC_PTR).ccr.modify(|_, w| w.adcpre().div2());

    if eocint == true {(*ADC1_PTR).cr1.modify(|_, w| w.eocie().enabled());}

    (*ADC1_PTR).smpr2.modify(|_, w| w.smp0().cycles144());

    match resolution {
      12 => (*ADC1_PTR).cr1.modify(|_, w| w.res().twelve_bit()),
      10 => (*ADC1_PTR).cr1.modify(|_, w| w.res().ten_bit()),
      8  => (*ADC1_PTR).cr1.modify(|_, w| w.res().eight_bit()),
      6  => (*ADC1_PTR).cr1.modify(|_, w| w.res().six_bit()),
      _  => panic!("{} is not a valid ADC resolution!", resolution)
    };

    (*ADC1_PTR).cr2.modify(|_, w| w.adon().enabled());
  }
}

pub fn analog_read(pin: (u8, char)) -> u16 {
  let buffer: u16;

  unsafe {
    if ADC_MAP.pin.contains(&pin) {
      if ADC_MAP.active[ADC_MAP.pin.iter().position(|&i| i == pin).unwrap()] == false {
        let channel = ADC_MAP.channel[ADC_MAP.pin.iter().position(|&i| i == pin).unwrap()];
        (*ADC1_PTR).sqr3.modify(|_, w| w.sq1().bits(channel));

        (*ADC1_PTR).cr2.write(|w| w.swstart().start());
        while (*ADC1_PTR).sr.read().eoc().is_not_complete() == true {}
        buffer = (*ADC1_PTR).dr.read().data().bits();
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
  unsafe {
    for i in 0..PINCONFIG.analog.len() {
      if PINCONFIG.analog[i] == 2 {
        // Check if pin is available for adc connection
        if PINCONFIG.pin[i] == PA4 {DAC_MAP.0 = true;}
        else if PINCONFIG.pin[i] == PA5 {DAC_MAP.1 = true;}
        else {panic!("P{}{} is not available for analog conversion!", PINCONFIG.pin[i].1.to_uppercase(), PINCONFIG.pin[i].0);}
      }
    }

    if DAC_MAP.0 == true {
      (*DAC_PTR).cr.modify(|_, w| {
        w.boff1().enabled();
        w.ten1().enabled();
        w.tsel1().software();
        w.en1().enabled()
      });
    }

    if DAC_MAP.1 == true {
      (*DAC_PTR).cr.modify(|_, w| {
        w.boff2().enabled();
        w.ten2().enabled();
        w.tsel2().software();
        w.en2().enabled()
      });
    }
  }
}

pub fn analog_write(pin: (u8, char), value: u16) {
  if value > 4095 {panic!("DAC value outside of bounds!");}

  if pin == PA4 {
    unsafe {
      (*DAC_PTR).dhr12r1.write(|w| w.dacc1dhr().bits(value));
      (*DAC_PTR).swtrigr.write(|w| w.swtrig1().enabled());
    }
  }
  else if pin == PA5 {
    unsafe {
      (*DAC_PTR).dhr12r2.write(|w| w.dacc2dhr().bits(value));
      (*DAC_PTR).swtrigr.write(|w| w.swtrig2().enabled());
    }
  }
  else {panic!("P{}{} is not available for analog conversion!", pin.1.to_uppercase(), pin.0);}
}
