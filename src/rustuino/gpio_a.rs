use heapless::Vec;
use super::include::{CONFIG, ADC_MAP};
use super::include::RCC_PTR;
use super::include::{ADCC_PTR, ADC1_PTR};

pub fn adc_init() {
  let mut channels: Vec<u8, 16> = Vec::new();

  unsafe {
    for i in 0..CONFIG.analog.len() {
      if CONFIG.analog[i] == 1 {
        // Check if pin is available for adc connection
        if ADC_MAP.pin.contains(&CONFIG.pin[i]) {
          channels.push(ADC_MAP.channel[ADC_MAP.pin.iter().position(|&r| r == CONFIG.pin[i]).unwrap()])
          .expect("Could not configure ADC for channel");
          ADC_MAP.active[ADC_MAP.pin.iter().position(|&r| r == CONFIG.pin[i]).unwrap()] = true;
        }
        else {
          panic!("P{}{} is not available for analog conversion!", CONFIG.pin[i].1, CONFIG.pin[i].0);
        }
      }
    }
    
    (*ADCC_PTR).ccr.modify(|_, w| w.adcpre().div2());
    (*RCC_PTR).apb2enr.modify(|_, w| w.adc1en().enabled());

    for i in 0..channels.len() {
      if channels[i] < 10 {(*ADC1_PTR).smpr2.modify(|r, w| w.bits(r.bits() | (7 << (channels[i] * 3))));}
      else {(*ADC1_PTR).smpr1.modify(|r, w| w.bits(r.bits() | (7 << ((channels[i] - 10) * 3))));}

      if i < 6 {(*ADC1_PTR).sqr3.modify(|r, w| w.bits(r.bits() | ((channels[i] as u32) << (i * 5))));}
      else if i >= 6 && i < 12 {(*ADC1_PTR).sqr2.modify(|r, w| w.bits(r.bits() | ((channels[i] as u32) << ((i - 6) * 5))));}
      else {(*ADC1_PTR).sqr1.modify(|r, w| w.bits(r.bits() | ((channels[i] as u32) << ((i - 12) * 5))));}
    }

    (*ADC1_PTR).cr2.modify(|_, w| {
      w.cont().continuous();
      w.adon().enabled()
    });
    (*ADC1_PTR).cr2.modify(|_, w| w.swstart().start());
  }
}

pub fn analog_read(pin: (u8, char)) -> u16 {
  let buffer: u16;

  unsafe {
    if ADC_MAP.pin.contains(&pin) {
      if ADC_MAP.active[ADC_MAP.pin.iter().position(|&i| i == pin).unwrap()] == true {
        buffer = (*ADC1_PTR).dr.read().data().bits();
      }
      else {
        panic!("P{}{} was not initialized for conversion!", pin.1, pin.0);
      }
    }
    else {panic!("P{}{} is not available for analog conversion!", pin.1, pin.0);}
  }

  return buffer;
}
