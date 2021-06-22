use super::include::data_maps::{ADC1_MAP, ADC3_MAP};
use super::common::*;
use super::include::PERIPHERAL_PTR;
use cortex_m_semihosting::hprintln;


// Converter implementations ======================================================================
macro_rules! generate_ToAnalog {
  ($($number:literal),+) => {
    $(
      impl<const B: char, const P: u8> ToAnalog for GpioPin<B, P, $number> {
        fn analog(self, resolution: u8, eocint: bool) -> AnalogPin<Self> {
          self.block = B;
          self.pin = P;
          
          if self.block == 'a' && self.pin == 4 {
            dac_init(1);
          }
          else if self.block == 'a' && self.pin == 5 {
            dac_init(2);
          }
          else if self.block == 'f' {adc_init(3, resolution, eocint);}
          else {adc_init(1, resolution, eocint);}
        }
      }
    )+
  };
}

generate_ToAnalog!(3, 7);


// Functions implementations ======================================================================
impl<const B: char, const P: u8> Analog for AnalogPin<GpioPin<B, P, 3>> {
  fn analog_read(&self) -> u16 {
    if B == 'a' && P == 4 {panic!("P{}{} is reserved for DAC channel", B.to_uppercase(), P);}
    if B == 'a' && P == 5 {panic!("P{}{} is reserved for DAC channel", B.to_uppercase(), P);}
    
    return adc_read(B, P);
  }

  fn analog_write(&self, value: u16) {
    if B == 'a' && P == 4 {dac_write(1, value);}
    else if B == 'a' && P == 5 {dac_write(2, value);}
    else {panic!("P{}{} is not a DAC channel", B.to_uppercase(), P);}
  }
}


// Helper functions ===============================================================================
pub fn adc_init(adc: u8, resolution: u8, eocint: bool) {
  let rcc = &PERIPHERAL_PTR.RCC;
  let adcc = &PERIPHERAL_PTR.ADC_COMMON;
  
  match adc {
    1 => {
      let adc1 = &PERIPHERAL_PTR.ADC1;
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
    },
    3 => {
      let adc3 = &PERIPHERAL_PTR.ADC3;
      rcc.apb2enr.modify(|_, w| w.adc3en().enabled());
      adcc.ccr.modify(|_, w| w.adcpre().div2());
      adc3.smpr2.modify(|_, w| w.smp0().cycles144());
      
      if eocint == true {adc3.cr1.modify(|_, w| w.eocie().enabled());}
      
      match resolution {
        12 => adc3.cr1.modify(|_, w| w.res().twelve_bit()),
        10 => adc3.cr1.modify(|_, w| w.res().ten_bit()),
        8  => adc3.cr1.modify(|_, w| w.res().eight_bit()),
        6  => adc3.cr1.modify(|_, w| w.res().six_bit()),
        _  => panic!("{} is not a valid ADC resolution!", resolution)
      };
      
      adc3.cr2.modify(|_, w| w.adon().enabled());
    },
    _ => panic!("{} is not a valid ADC!", adc)
  };
}

pub fn dac_init(dac: u8) {
  let rcc = &PERIPHERAL_PTR.RCC;
  let dac = &PERIPHERAL_PTR.DAC;
  
  rcc.apb1enr.modify(|_, w| w.dacen().enabled());
  
  if dac == 1 {
    dac.cr.modify(|_, w| {
      w.boff1().enabled();
      w.ten1().enabled();
      w.tsel1().software();
      w.en1().enabled()
    });
  }
  else if dac == 2 {
    dac.cr.modify(|_, w| {
      w.boff2().enabled();
      w.ten2().enabled();
      w.tsel2().software();
      w.en2().enabled()
    });
  }
}

pub fn adc_read(block: char, pin: u8) -> u16 {
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

pub fn dac_write(dac: u8, value: u16) {
  let dac = &PERIPHERAL_PTR.DAC;
  
  if value > 4095 {
    hprintln!("Value outside of bounds!");
    value = 4095;
  }
  
  if dac == 1 {
    dac.dhr12r1.write(|w| w.dacc1dhr().bits(value));
    dac.swtrigr.write(|w| w.swtrig1().enabled());
  }
  else {
    dac.dhr12r2.write(|w| w.dacc2dhr().bits(value));
    dac.swtrigr.write(|w| w.swtrig2().enabled());
  }
}
