use super::common::*;
use super::include::{ADC1_MAP, ADC3_MAP, ADC_CONF};
use cortex_m_semihosting::hprintln;


// Converter implementations ======================================================================
macro_rules! generate_ToAnalog {
  ($([$letter:literal, $number:literal]),+) => {
    use paste::paste;

    paste!{
      $(
        impl ToAnalog for [<P $letter:upper $number>] {
          fn analog(resolution: u8, eocint: bool) -> AnalogPin {
            let block = $letter;
            let pin = $number;
            
            if block == 'a' && pin == 4 {
              dac_init(1);
            }
            else if block == 'a' && pin == 5 {
              dac_init(2);
            }
            else if block == 'f' {
              unsafe {
                if ADC_CONF[1] == false {
                  adc_init(3, resolution, eocint);
                  ADC_CONF[1] = true;
                }
              }
            }
            else {
              unsafe {
                if ADC_CONF[0] == false {
                  adc_init(1, resolution, eocint);
                  ADC_CONF[0] = true;
                }
              }
            }
  
            return AnalogPin {
              block,
              pin
            };
          }
        }
      )+
    }
  };
}

// 2⁰ == 1 && 2¹ == 1 | restliche stellen egal
// generate_ToAnalog!(3, 7, 11, 15, 19, 23, 27, 31, 35, 39, 43, 47, 51, 55, 59, 63);

generate_ToAnalog![
  ['a', 0], ['a', 1], ['a', 2], ['a', 3], ['a', 4], ['a', 5], ['a', 6], ['a', 7],
  ['b', 0], ['b', 1],
  ['c', 0], ['c', 1], ['c', 2], ['c', 3], ['c', 4], ['c', 5],
  ['f', 3], ['f', 4], ['f', 5], ['f', 6], ['f', 7], ['f', 8], ['f', 9]
];


// Functions implementations ======================================================================
impl Analog for AnalogPin {
  fn analog_read(&self) -> u16 {
    let block = self.block;
    let pin = self.pin;

    if block == 'a' && pin == 4 {panic!("P{}{} is reserved for DAC channel", block.to_uppercase(), pin);}
    if block == 'a' && pin == 5 {panic!("P{}{} is reserved for DAC channel", block.to_uppercase(), pin);}
    
    return adc_read(block, pin);
  }

  fn analog_write(&self, value: u16) {
    let block = self.block;
    let pin = self.pin;

    if block == 'a' && pin == 4 {dac_write(1, value);}
    else if block == 'a' && pin == 5 {dac_write(2, value);}
    else {panic!("P{}{} is not a DAC channel", block.to_uppercase(), pin);}
  }
}


// Helper functions ===============================================================================
fn adc_init(adc: u8, resolution: u8, eocint: bool) {
  let peripheral_ptr = stm32f4::stm32f446::Peripherals::take().unwrap();
  let rcc = &peripheral_ptr.RCC;
  let adcc = &peripheral_ptr.ADC_COMMON;
  
  match adc {
    1 => {
      let adc1 = &peripheral_ptr.ADC1;
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
      let adc3 = &peripheral_ptr.ADC3;
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

fn dac_init(channel: u8) {
  let peripheral_ptr = stm32f4::stm32f446::Peripherals::take().unwrap();
  let rcc = &peripheral_ptr.RCC;
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
  else if channel == 2 {
    dac.cr.modify(|_, w| {
      w.boff2().enabled();
      w.ten2().enabled();
      w.tsel2().software();
      w.en2().enabled()
    });
  }
}

fn adc_read(block: char, pin: u8) -> u16 {
  let peripheral_ptr = stm32f4::stm32f446::Peripherals::take().unwrap();

  let buffer = if block == 'f' {
    let adc3 = &peripheral_ptr.ADC3;
    let channel = ADC3_MAP.channel[ADC3_MAP.pin.iter().position(|&i| i == pin).unwrap()];
    adc3.sqr3.modify(|_, w| unsafe {w.sq1().bits(channel)});

    adc3.cr2.write(|w| w.swstart().start());
    while adc3.sr.read().eoc().is_not_complete() == true {}
    adc3.dr.read().data().bits() as u16
  }
  else {
    let adc1 = &peripheral_ptr.ADC1;
    let channel = ADC1_MAP.channel[ADC1_MAP.pin.iter().position(|&i| i == (block, pin)).unwrap()];
    adc1.sqr3.modify(|_, w| unsafe {w.sq1().bits(channel)});
    adc1.cr2.write(|w| w.swstart().start());
    while adc1.sr.read().eoc().is_not_complete() == true {}
    adc1.dr.read().data().bits() as u16
  };

  return buffer;
}

fn dac_write(channel: u8, value: u16) {
  let peripheral_ptr = stm32f4::stm32f446::Peripherals::take().unwrap();
  let dac = &peripheral_ptr.DAC;
  let val: u16;
  
  if value > 4095 {
    hprintln!("Value outside of bounds!").expect("Could not send semihosting message!");
    val = 4095;
  }
  else {val = value;}
  
  if channel == 1 {
    dac.dhr12r1.write(|w| w.dacc1dhr().bits(val));
    dac.swtrigr.write(|w| w.swtrig1().enabled());
  }
  else {
    dac.dhr12r2.write(|w| w.dacc2dhr().bits(val));
    dac.swtrigr.write(|w| w.swtrig2().enabled());
  }
}
