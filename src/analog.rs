use crate::include::pins::*;
use crate::include::{ADC_MAP, ADC_CONF, DAC_MAP, DAC_CONF};

/// This struct holds the configuration of a pin that has been configured as an analog pin.
///
/// To configure a pin as an analog pin, call either the input or output functions on the appropriate pin label. The function returns the pin struct with the settings of the pin.
/// # Example
/// ```rust,no_run
/// #![no_std]
/// #![no_main]
///
/// use rustuino::*;
///
/// #[entry]
/// fn main() -> ! {
///   let pin1 = PA0::analog(10, false);
///   let pin2 = PA1::output();
///
///   let mut adc_value: u16;
///
///   loop {
///     adc_value = pin1.analog_read();
///     if adc_value > 4095 / 2 {
///       pin2.write(true);
///     }
///     else {
///       pin2.write(false);
///     }
///     delay(1000);
///   }
/// }
/// ```
pub struct AnalogPin {
  block: char,
  pin: u8,
  res: u8,
  channel: bool,
  dir: bool
}

/// This trait is implemented on all pin structs that are able to be used with the internal ADC or DAC
pub trait ToAnalog: Sized {
  const BLOCK: char;
  const PIN: u8;

  /// Configures a pin as an analog input and gives back the associated pin struct.
  /// # Examples
  /// ```rust,no_run
  /// use rustuino::*;
  ///
  /// // Configure pin as an analog pin
  /// let mut pin = PA0::analog_input();
  ///
  /// // Read the analog value
  /// let value = pin.analog_read();
  /// ```
  fn analog_input() -> AnalogPin {
    let block = Self::BLOCK;
    let pin = Self::PIN;

    if ADC_MAP.pin.contains(&(block, pin)) == true {
      let peripheral_ptr;
      unsafe {peripheral_ptr = stm32f4::stm32f446::Peripherals::steal();}
      let rcc = &peripheral_ptr.RCC;
      let adcc = &peripheral_ptr.ADC_COMMON;
      let adc1 = &peripheral_ptr.ADC1;
      let adc3 = &peripheral_ptr.ADC3;

      match block {
        'a' => {
          let gpioa = &peripheral_ptr.GPIOA;
          rcc.ahb1enr.modify(|_, w| w.gpioaen().enabled());
          gpioa.moder.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin)) | (3 << (2 * pin)))});
        },
        'b' => {
          let gpiob = &peripheral_ptr.GPIOB;
          rcc.ahb1enr.modify(|_, w| w.gpioben().enabled());
          gpiob.moder.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin)) | (3 << (2 * pin)))});
        },
        'c' => {
          let gpioc = &peripheral_ptr.GPIOC;
          rcc.ahb1enr.modify(|_, w| w.gpiocen().enabled());
          gpioc.moder.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin)) | (3 << (2 * pin)))});
        },
        'f' => {
          let gpiof = &peripheral_ptr.GPIOF;
          rcc.ahb1enr.modify(|_, w| w.gpiofen().enabled());
          gpiof.moder.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin)) | (3 << (2 * pin)))});
        },
        _   => panic!("P{}{} not available for ADC conversion! | ::analog_input(...)", block.to_uppercase(), pin)
      };

      unsafe {
        if block == 'f' && ADC_CONF[1] == false {ADC_CONF[1] = true;}
        else if ADC_CONF[0] == false {ADC_CONF[0] = true;}
        else if block == 'f' {
          return AnalogPin {
            block,
            pin,
            res: adc3.cr1.read().res().bits(),
            channel: true,
            dir: false
          };
        }
        else {
          return AnalogPin {
            block,
            pin,
            res: adc1.cr1.read().res().bits(),
            channel: false,
            dir: false
          };
        }
      }

      if block == 'f' {
        rcc.apb2enr.modify(|_, w| w.adc3en().enabled());
        adcc.ccr.modify(|_, w| w.adcpre().div2());
        adc3.smpr2.modify(|_, w| w.smp0().cycles144());
        adc3.cr1.modify(|_, w| w.res().ten_bit());
        adc3.cr2.modify(|_, w| w.adon().enabled());

        return AnalogPin {
          block,
          pin,
          res: 10,
          channel: true,
          dir: false
        };
      }
      else {
        rcc.apb2enr.modify(|_, w| w.adc1en().enabled());
        adcc.ccr.modify(|_, w| w.adcpre().div2());
        adc1.smpr2.modify(|_, w| w.smp0().cycles144());
        adc1.cr1.modify(|_, w| w.res().ten_bit());
        adc1.cr2.modify(|_, w| w.adon().enabled());

        return AnalogPin {
          block,
          pin,
          res: 10,
          channel: false,
          dir: false
        };
      }
    }
    else {panic!("P{}{} not available for ADC conversion! | ::analog_input(...)", block.to_uppercase(), pin);}
  }

  /// Configures a pin as an analog output and gives back the associated pin struct.
  /// # Examples
  /// ```rust,no_run
  /// use rustuino::*;
  ///
  /// // Configure pin as an analog pin
  /// let pin = PA4::analog_output();
  ///
  /// // Write the analog value
  /// pin.analog_write(0);
  /// pin.analog_write(4095);
  /// ```
  fn analog_output() -> AnalogPin {
    let block = Self::BLOCK;
    let pin = Self::PIN;

    if DAC_MAP.pin.contains(&(block, pin)) == true {
      let peripheral_ptr;
      unsafe {peripheral_ptr = stm32f4::stm32f446::Peripherals::steal();}
      let rcc = &peripheral_ptr.RCC;
      let gpioa = &peripheral_ptr.GPIOA;
      let dac = &peripheral_ptr.DAC;

      rcc.ahb1enr.modify(|_, w| w.gpiofen().enabled());
      gpioa.moder.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin)) | (3 << (2 * pin)))});

      if DAC_MAP.channel[DAC_MAP.pin.iter().position(|&i| i == (block, pin)).unwrap()] == 1 {
        unsafe {
          if DAC_CONF[0] == false {DAC_CONF[1] = true;}
          else {
            return AnalogPin {
              block,
              pin,
              res: 12,
              channel: false,
              dir: true
            };
          }
        }

        rcc.apb1enr.modify(|_, w| w.dacen().enabled());
        dac.cr.modify(|_, w| {
          w.boff1().enabled();
          w.ten1().enabled();
          w.tsel1().software();
          w.en1().enabled()
        });

        return AnalogPin {
          block,
          pin,
          res: 12,
          channel: false,
          dir: true
        };
      }
      else {
        unsafe {
          if DAC_CONF[1] == false {DAC_CONF[1] = true;}
          else {
            return AnalogPin {
              block,
              pin,
              res: 12,
              channel: true,
              dir: true
            };
          }
        }

        rcc.apb1enr.modify(|_, w| w.dacen().enabled());
        dac.cr.modify(|_, w| {
          w.boff2().enabled();
          w.ten2().enabled();
          w.tsel2().software();
          w.en2().enabled()
        });

        return AnalogPin {
          block,
          pin,
          res: 12,
          channel: true,
          dir: true
        };
      }
    }
    else {panic!("P{}{} not available for DAC conversion! | ::analog_output(...)", block.to_uppercase(), pin);}
  }
}

macro_rules! generate_ToAnalog {
  ($([$letter:literal, $number:literal]),+) => {
    use paste::paste;

    paste!{
      $(
        impl ToAnalog for [<P $letter:upper $number>] {
          const BLOCK: char = $letter;
          const PIN: u8 = $number;
        }
      )+
    }
  };
}

generate_ToAnalog![
  ['a', 0], ['a', 1], ['a', 2], ['a', 3], ['a', 4], ['a', 5], ['a', 6], ['a', 7],
  ['b', 0], ['b', 1],
  ['c', 0], ['c', 1], ['c', 2], ['c', 3], ['c', 4], ['c', 5],
  ['f', 3], ['f', 4], ['f', 5], ['f', 6], ['f', 7], ['f', 8], ['f', 9]
];


// Functions implementations ======================================================================
impl AnalogPin {
  /// Change the Change the ADC/DAC resolution of the pin. The default for input pins is 10 bit and for output pins 12 bit.
  /// # Examples
  /// ```rust,no_run
  /// use rustuino::*;
  ///
  /// // Configure pin as an analog pin
  /// let mut pin = PA0::analog_input();
  ///
  /// // Change the ADC/DAC resolution of the pin
  /// pin.analog_resolution(6);
  /// pin.analog_resolution(8);
  /// pin.analog_resolution(10); // Input default
  /// pin.analog_resolution(12); // Output default
  /// ```
  pub fn analog_resolution(&mut self, res: u8) {
    let peripheral_ptr;
    unsafe {peripheral_ptr = stm32f4::stm32f446::Peripherals::steal();}

    if self.dir == false {
      if self.channel == false {
        let adc1 = &peripheral_ptr.ADC1;
        match res {
          6  => adc1.cr1.modify(|_, w| w.res().six_bit()),
          8  => adc1.cr1.modify(|_, w| w.res().eight_bit()),
          10 => adc1.cr1.modify(|_, w| w.res().ten_bit()),
          12 => adc1.cr1.modify(|_, w| w.res().twelve_bit()),
          _  => panic!("{} Bit is not a possible ADC resolution! | .analog_resolution(...)", res)
        };
        self.res = res;
      }
      else {
        let adc3 = &peripheral_ptr.ADC3;
        match res {
          6  => adc3.cr1.modify(|_, w| w.res().six_bit()),
          8  => adc3.cr1.modify(|_, w| w.res().eight_bit()),
          10 => adc3.cr1.modify(|_, w| w.res().ten_bit()),
          12 => adc3.cr1.modify(|_, w| w.res().twelve_bit()),
          _  => panic!("{} Bit is not a possible ADC resolution! | .analog_resolution(...)", res)
        };
        self.res = res;
      }
    }
    else {
      todo!("Implement DAC resolution change!");
    }
  }

  /// Performs an ADC conversion on the pin.
  /// # Examples
  /// ```rust,no_run
  /// use rustuino::*;
  ///
  /// // Configure pin as an analog pin
  /// let pin = PA0::analog_input();
  ///
  /// // Read the analog value
  /// let value: u16 = pin.analog_read();
  /// ```
  pub fn analog_read(&self) -> u16 {
    let peripheral_ptr;
    unsafe {peripheral_ptr = stm32f4::stm32f446::Peripherals::steal();}

    let buffer = if self.block == 'f' {
      let adc3 = &peripheral_ptr.ADC3;
      let channel = ADC_MAP.channel[ADC_MAP.pin.iter().position(|&i| i == ('f', self.pin)).unwrap()];
      adc3.sqr3.modify(|_, w| unsafe {w.sq1().bits(channel)});

      adc3.cr2.write(|w| w.swstart().start());
      while adc3.sr.read().eoc().is_not_complete() == true {}
      adc3.dr.read().data().bits() as u16
    }
    else {
      let adc1 = &peripheral_ptr.ADC1;
      let channel = ADC_MAP.channel[ADC_MAP.pin.iter().position(|&i| i == (self.block, self.pin)).unwrap()];
      adc1.sqr3.modify(|_, w| unsafe {w.sq1().bits(channel)});
      adc1.cr2.write(|w| w.swstart().start());
      while adc1.sr.read().eoc().is_not_complete() == true {}
      adc1.dr.read().data().bits() as u16
    };

    return buffer;
  }

  /// Performs an DAC conversion on the pin.
  /// # Examples
  /// ```rust,no_run
  /// use rustuino::*;
  ///
  /// // Configure pin as an analog pin
  /// let pin = PA4::analog_output();
  ///
  /// // Write the analog value
  /// pin.analog_write(0);
  /// pin.analog_write(4095);
  /// ```
  pub fn analog_write(&self, value: u16) {
    let peripheral_ptr;
    unsafe {peripheral_ptr = stm32f4::stm32f446::Peripherals::steal();}
    let dac = &peripheral_ptr.DAC;
    let val: u16;

    if value > 4095 {
      rtt_target::rprintln!("Analog value outside of bounds! | .analog_write(...)");
      val = 4095;
    }
    else {val = value;}

    if self.pin == 4  {
      dac.dhr12r1.write(|w| w.dacc1dhr().bits(val));
      dac.swtrigr.write(|w| w.swtrig1().enabled());
    }
    else {
      dac.dhr12r2.write(|w| w.dacc2dhr().bits(val));
      dac.swtrigr.write(|w| w.swtrig2().enabled());
    }
  }
}
