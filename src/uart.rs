use super::common::*;
use super::include::{UART_MAP, UART_CONF};
use cortex_m_semihosting::hprintln;
use heapless::String;


// Initialisation function ========================================================================
pub fn uart_init(rx_pin: (char, u8), tx_pin: (char, u8), baud: u32, rxint: bool, txint: bool) -> Result<UartCore, String<20>> {
  let peripheral_ptr = stm32f4::stm32f446::Peripherals::take().unwrap();
  let rcc = &peripheral_ptr.RCC;

  let channel: u8;
        
  if UART_MAP.rx_pins.contains(&rx_pin) && UART_MAP.tx_pins.contains(&tx_pin) {
    let index = UART_MAP.rx_pins.iter().zip(UART_MAP.tx_pins.iter()).position(|i| i == (&rx_pin, &tx_pin)).unwrap();
    channel = UART_MAP.channel[index];
    unsafe {UART_CONF[channel as usize - 1] = true;}
  }
  else {return Err(String::from("These pins are not available for UART communication!"));}

  uart_setup_gpio(rx_pin.0, rx_pin.1, channel);
  uart_setup_gpio(tx_pin.0, tx_pin.1, channel);
  
  match channel {
    1 => {
      let usart1 = &peripheral_ptr.USART1;
      rcc.apb2enr.modify(|_, w| w.usart1en().enabled());
      set_baud(channel, baud);
      usart1.cr1.modify(|_, w| {
        w.te().enabled();
        w.re().enabled();
        w.ue().enabled()
      });
    },
    3 => {
      let usart3 = &peripheral_ptr.USART3;
      rcc.apb1enr.modify(|_, w| w.usart3en().enabled()); 
      set_baud(channel, baud);
      usart3.cr1.modify(|_, w| {
        w.te().enabled();
        w.re().enabled();
        w.ue().enabled()
      });
    },
    4 => {
      let uart4 = &peripheral_ptr.UART4;
      rcc.apb1enr.modify(|_, w| w.uart4en().enabled());
      set_baud(channel, baud);
      uart4.cr1.modify(|_, w| {
        w.te().enabled();
        w.re().enabled();
        w.ue().enabled()
      });
    },
    5 => {
      let uart5 = &peripheral_ptr.UART5;
      rcc.apb1enr.modify(|_, w| w.uart5en().enabled());
      set_baud(channel, baud);
      uart5.cr1.modify(|_, w| {
        w.te().enabled();
        w.re().enabled();
        w.ue().enabled()
      });
    },
    6 => {
      let usart6 = &peripheral_ptr.USART6;
      rcc.apb2enr.modify(|_, w| w.usart6en().enabled());
      set_baud(channel, baud);
      usart6.cr1.modify(|_, w| {
        w.te().enabled();
        w.re().enabled();
        w.ue().enabled()
      });
    },
    _ => panic!("{} is not a valid UART peripheral!", channel)
  };

  if rxint == true {rx_interrupt(channel, true);}
  if txint == true {tx_interrupt(channel, true);}

  return Ok(UartCore {
    rx: rx_pin,
    tx: tx_pin,
    channel,
    rx_int: rxint,
    tx_int: txint
  });
}


// Function implementations =======================================================================
impl UART for UartCore {
  fn rxint_enable(&self) {
    rx_interrupt(self.channel, true);
  }

  fn rxint_disable(&self) {
    rx_interrupt(self.channel, false);
  }

  fn txint_enable(&self) {
    tx_interrupt(self.channel, true);
  }

  fn txint_disable(&self) {
    tx_interrupt(self.channel, false);
  }

  fn change_baud(&self, baud: u32) {
    set_baud(self.channel, baud);
  }

  fn send_char(&self, c: char) {
    let peripheral_ptr = stm32f4::stm32f446::Peripherals::take().unwrap();
  
    match self.channel {
      1 => {
        let usart1 = &peripheral_ptr.USART1;
        if c.is_ascii() == true {
          while usart1.sr.read().txe().bit_is_set() == true {}
          usart1.dr.write(|w| w.dr().bits(c as u16));
          while usart1.sr.read().txe().bit_is_set() == true {}
        }
        else {
          hprintln!("{} is not an ASCII character!", c).expect("Could not send semihosting message!");
      
          while usart1.sr.read().txe().bit_is_set() == true {}
          usart1.dr.write(|w| w.dr().bits('?' as u16));
          while usart1.sr.read().txe().bit_is_set() == true {}
        }
      },
      2 => {
        let usart2 = &peripheral_ptr.USART2;
        if c.is_ascii() == true {
          while usart2.sr.read().txe().bit_is_set() == true {}
          usart2.dr.write(|w| w.dr().bits(c as u16));
          while usart2.sr.read().txe().bit_is_set() == true {}
        }
        else {
          hprintln!("{} is not an ASCII character!", c).expect("Could not send semihosting message!");
      
          while usart2.sr.read().txe().bit_is_set() == true {}
          usart2.dr.write(|w| w.dr().bits('?' as u16));
          while usart2.sr.read().txe().bit_is_set() == true {}
        }
      },
      3 => {
        let usart3 = &peripheral_ptr.USART3;
        if c.is_ascii() == true {
          while usart3.sr.read().txe().bit_is_set() == true {}
          usart3.dr.write(|w| w.dr().bits(c as u16));
          while usart3.sr.read().txe().bit_is_set() == true {}
        }
        else {
          hprintln!("{} is not an ASCII character!", c).expect("Could not send semihosting message!");
      
          while usart3.sr.read().txe().bit_is_set() == true {}
          usart3.dr.write(|w| w.dr().bits('?' as u16));
          while usart3.sr.read().txe().bit_is_set() == true {}
        }
      },
      4 => {let uart4 = &peripheral_ptr.UART4;
        if c.is_ascii() == true {
          while uart4.sr.read().txe().bit_is_set() == true {}
          uart4.dr.write(|w| w.dr().bits(c as u16));
          while uart4.sr.read().txe().bit_is_set() == true {}
        }
        else {
          hprintln!("{} is not an ASCII character!", c).expect("Could not send semihosting message!");
      
          while uart4.sr.read().txe().bit_is_set() == true {}
          uart4.dr.write(|w| w.dr().bits('?' as u16));
          while uart4.sr.read().txe().bit_is_set() == true {}
        }},
      5 => {
        let uart5 = &peripheral_ptr.UART5;
        if c.is_ascii() == true {
          while uart5.sr.read().txe().bit_is_set() == true {}
          uart5.dr.write(|w| w.dr().bits(c as u16));
          while uart5.sr.read().txe().bit_is_set() == true {}
        }
        else {
          hprintln!("{} is not an ASCII character!", c).expect("Could not send semihosting message!");
      
          while uart5.sr.read().txe().bit_is_set() == true {}
          uart5.dr.write(|w| w.dr().bits('?' as u16));
          while uart5.sr.read().txe().bit_is_set() == true {}
        }
      },
      6 => {
        let usart6 = &peripheral_ptr.USART6;
        if c.is_ascii() == true {
          while usart6.sr.read().txe().bit_is_set() == true {}
          usart6.dr.write(|w| w.dr().bits(c as u16));
          while usart6.sr.read().txe().bit_is_set() == true {}
        }
        else {
          hprintln!("{} is not an ASCII character!", c).expect("Could not send semihosting message!");
      
          while usart6.sr.read().txe().bit_is_set() == true {}
          usart6.dr.write(|w| w.dr().bits('?' as u16));
          while usart6.sr.read().txe().bit_is_set() == true {}
        }
      },
      _ => panic!("{} is not a valid UART peripheral!", self.channel)
    };
  }

  fn send_string(&self, s: &str) {
    let peripheral_ptr = stm32f4::stm32f446::Peripherals::take().unwrap();

    for c in s.chars() {
      match self.channel {
        1 => {
          let usart1 = &peripheral_ptr.USART1;
          if c.is_ascii() == true {
            while usart1.sr.read().txe().bit_is_set() == true {}
            usart1.dr.write(|w| w.dr().bits(c as u16));
            while usart1.sr.read().txe().bit_is_set() == true {}
          }
          else {
            hprintln!("{} is not an ASCII character!", c).expect("Could not send semihosting message!");
        
            while usart1.sr.read().txe().bit_is_set() == true {}
            usart1.dr.write(|w| w.dr().bits('?' as u16));
            while usart1.sr.read().txe().bit_is_set() == true {}
          }
        },
        2 => {
          let usart2 = &peripheral_ptr.USART2;
          if c.is_ascii() == true {
            while usart2.sr.read().txe().bit_is_set() == true {}
            usart2.dr.write(|w| w.dr().bits(c as u16));
            while usart2.sr.read().txe().bit_is_set() == true {}
          }
          else {
            hprintln!("{} is not an ASCII character!", c).expect("Could not send semihosting message!");
        
            while usart2.sr.read().txe().bit_is_set() == true {}
            usart2.dr.write(|w| w.dr().bits('?' as u16));
            while usart2.sr.read().txe().bit_is_set() == true {}
          }
        },
        3 => {
          let usart3 = &peripheral_ptr.USART3;
          if c.is_ascii() == true {
            while usart3.sr.read().txe().bit_is_set() == true {}
            usart3.dr.write(|w| w.dr().bits(c as u16));
            while usart3.sr.read().txe().bit_is_set() == true {}
          }
          else {
            hprintln!("{} is not an ASCII character!", c).expect("Could not send semihosting message!");
        
            while usart3.sr.read().txe().bit_is_set() == true {}
            usart3.dr.write(|w| w.dr().bits('?' as u16));
            while usart3.sr.read().txe().bit_is_set() == true {}
          }
        },
        4 => {let uart4 = &peripheral_ptr.UART4;
          if c.is_ascii() == true {
            while uart4.sr.read().txe().bit_is_set() == true {}
            uart4.dr.write(|w| w.dr().bits(c as u16));
            while uart4.sr.read().txe().bit_is_set() == true {}
          }
          else {
            hprintln!("{} is not an ASCII character!", c).expect("Could not send semihosting message!");
        
            while uart4.sr.read().txe().bit_is_set() == true {}
            uart4.dr.write(|w| w.dr().bits('?' as u16));
            while uart4.sr.read().txe().bit_is_set() == true {}
          }},
        5 => {
          let uart5 = &peripheral_ptr.UART5;
          if c.is_ascii() == true {
            while uart5.sr.read().txe().bit_is_set() == true {}
            uart5.dr.write(|w| w.dr().bits(c as u16));
            while uart5.sr.read().txe().bit_is_set() == true {}
          }
          else {
            hprintln!("{} is not an ASCII character!", c).expect("Could not send semihosting message!");
        
            while uart5.sr.read().txe().bit_is_set() == true {}
            uart5.dr.write(|w| w.dr().bits('?' as u16));
            while uart5.sr.read().txe().bit_is_set() == true {}
          }
        },
        6 => {
          let usart6 = &peripheral_ptr.USART6;
          if c.is_ascii() == true {
            while usart6.sr.read().txe().bit_is_set() == true {}
            usart6.dr.write(|w| w.dr().bits(c as u16));
            while usart6.sr.read().txe().bit_is_set() == true {}
          }
          else {
            hprintln!("{} is not an ASCII character!", c).expect("Could not send semihosting message!");
        
            while usart6.sr.read().txe().bit_is_set() == true {}
            usart6.dr.write(|w| w.dr().bits('?' as u16));
            while usart6.sr.read().txe().bit_is_set() == true {}
          }
        },
        _ => panic!("{} is not a valid UART peripheral!", self.channel)
      };
    }
  }

  fn get_char(&self) -> char {
    let peripheral_ptr = stm32f4::stm32f446::Peripherals::take().unwrap();

    let buffer = match self.channel {
      1 => {
        let usart1 = &peripheral_ptr.USART1;
        while usart1.sr.read().rxne().bit_is_clear() == true {}
        usart1.dr.read().dr().bits() as u8
      },
      2 => {
        let usart2 = &peripheral_ptr.USART2;
        while usart2.sr.read().rxne().bit_is_clear() == true {}
        usart2.dr.read().dr().bits() as u8
      },
      3 => {
        let usart3 = &peripheral_ptr.USART3;
        while usart3.sr.read().rxne().bit_is_clear() == true {}
        usart3.dr.read().dr().bits() as u8
      },
      4 => {
        let uart4 = &peripheral_ptr.UART4;
        while uart4.sr.read().rxne().bit_is_clear() == true {}
        uart4.dr.read().dr().bits() as u8
      },
      5 => {
        let uart5 = &peripheral_ptr.UART5;
        while uart5.sr.read().rxne().bit_is_clear() == true {}
        uart5.dr.read().dr().bits() as u8
      },
      6 => {
        let usart6 = &peripheral_ptr.USART6;
        while usart6.sr.read().rxne().bit_is_clear() == true {}
        usart6.dr.read().dr().bits() as u8
      },
      _ => panic!("{} is not a valid UART peripheral!", self.channel)
    };
  
    return buffer as char;
  }

  fn get_string(&self, stopper: char) -> Result<String<30>, String<20>> {
    let peripheral_ptr = stm32f4::stm32f446::Peripherals::take().unwrap();
    let mut buffer: u8;
    let mut string_buffer: String<30> = String::new();

    todo!("variable vec size!");

    if stopper.is_ascii() == false {return Err(String::from("Stop char is not an ASCII character!"));}

    loop {
      buffer = match self.channel {
        1 => {
          let usart1 = &peripheral_ptr.USART1;
          while usart1.sr.read().rxne().bit_is_clear() == true {}
          usart1.dr.read().dr().bits() as u8
        },
        2 => {
          let usart2 = &peripheral_ptr.USART2;
          while usart2.sr.read().rxne().bit_is_clear() == true {}
          usart2.dr.read().dr().bits() as u8
        },
        3 => {
          let usart3 = &peripheral_ptr.USART3;
          while usart3.sr.read().rxne().bit_is_clear() == true {}
          usart3.dr.read().dr().bits() as u8
        },
        4 => {
          let uart4 = &peripheral_ptr.UART4;
          while uart4.sr.read().rxne().bit_is_clear() == true {}
          uart4.dr.read().dr().bits() as u8
        },
        5 => {
          let uart5 = &peripheral_ptr.UART5;
          while uart5.sr.read().rxne().bit_is_clear() == true {}
          uart5.dr.read().dr().bits() as u8
        },
        6 => {
          let usart6 = &peripheral_ptr.USART6;
          while usart6.sr.read().rxne().bit_is_clear() == true {}
          usart6.dr.read().dr().bits() as u8
        },
        _ => panic!("{} is not a valid UART peripheral!", self.channel)
      };

      if buffer == stopper as u8 {return Ok(string_buffer);}
      string_buffer.push(buffer as char).expect("String buffer overflow!");  
    }
  }
}


// Helper functions ===============================================================================
fn uart_setup_gpio(block: char, pin: u8, channel: u8) {
  let peripheral_ptr = stm32f4::stm32f446::Peripherals::take().unwrap();
  let rcc = &peripheral_ptr.RCC;

  match block {
    'a' => {
      let gpioa = &peripheral_ptr.GPIOA;
      rcc.ahb1enr.modify(|_, w| w.gpioaen().enabled());
      gpioa.moder.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin)) | (2 << (2 * pin)))});
      if channel < 4 {
        if pin > 7 {gpioa.afrh.modify(|r, w| unsafe {w.bits(r.bits() | (7 << (4 * (pin - 8))))});}
        else {gpioa.afrl.modify(|r, w| unsafe {w.bits(r.bits() | (7 << (4 * pin)))});}
      }
      else {
        if pin > 7 {gpioa.afrh.modify(|r, w| unsafe {w.bits(r.bits() | (8 << (4 * (pin - 8))))});}
        else {gpioa.afrl.modify(|r, w| unsafe {w.bits(r.bits() | (8 << (4 * pin)))});}
      }
    },
    'b' => {
      let gpiob = &peripheral_ptr.GPIOB;
      rcc.ahb1enr.modify(|_, w| w.gpioben().enabled());
      gpiob.moder.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin)) | (2 << (2 * pin)))});
      if channel < 4 {
        if pin > 7 {gpiob.afrh.modify(|r, w| unsafe {w.bits(r.bits() | (7 << (4 * (pin - 8))))});}
        else {gpiob.afrl.modify(|r, w| unsafe {w.bits(r.bits() | (7 << (4 * pin)))});}
      }
      else {
        if pin > 7 {gpiob.afrh.modify(|r, w| unsafe {w.bits(r.bits() | (8 << (4 * (pin - 8))))});}
        else {gpiob.afrl.modify(|r, w| unsafe {w.bits(r.bits() | (8 << (4 * pin)))});}
      }
    },
    'c' => {
      let gpioc = &peripheral_ptr.GPIOC;
      rcc.ahb1enr.modify(|_, w| w.gpiocen().enabled());
      gpioc.moder.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin)) | (2 << (2 * pin)))});
      if channel < 4 {
        if pin > 7 {gpioc.afrh.modify(|r, w| unsafe {w.bits(r.bits() | (7 << (4 * (pin - 8))))});}
        else {gpioc.afrl.modify(|r, w| unsafe {w.bits(r.bits() | (7 << (4 * pin)))});}
      }
      else {
        if pin > 7 {gpioc.afrh.modify(|r, w| unsafe {w.bits(r.bits() | (8 << (4 * (pin - 8))))});}
        else {gpioc.afrl.modify(|r, w| unsafe {w.bits(r.bits() | (8 << (4 * pin)))});}
      }
    },
    'd' => {
      let gpiod = &peripheral_ptr.GPIOD;
      rcc.ahb1enr.modify(|_, w| w.gpioden().enabled());
      gpiod.moder.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin)) | (2 << (2 * pin)))});
      if channel < 4 {
        if pin > 7 {gpiod.afrh.modify(|r, w| unsafe {w.bits(r.bits() | (7 << (4 * (pin - 8))))});}
        else {gpiod.afrl.modify(|r, w| unsafe {w.bits(r.bits() | (7 << (4 * pin)))});}
      }
      else {
        if pin > 7 {gpiod.afrh.modify(|r, w| unsafe {w.bits(r.bits() | (8 << (4 * (pin - 8))))});}
        else {gpiod.afrl.modify(|r, w| unsafe {w.bits(r.bits() | (8 << (4 * pin)))});}
      }
    },
    'e' => {
      let gpioe = &peripheral_ptr.GPIOE;
      rcc.ahb1enr.modify(|_, w| w.gpioeen().enabled());
      gpioe.moder.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin)) | (2 << (2 * pin)))});
      if channel < 4 {
        if pin > 7 {gpioe.afrh.modify(|r, w| unsafe {w.bits(r.bits() | (7 << (4 * (pin - 8))))});}
        else {gpioe.afrl.modify(|r, w| unsafe {w.bits(r.bits() | (7 << (4 * pin)))});}
      }
      else {
        if pin > 7 {gpioe.afrh.modify(|r, w| unsafe {w.bits(r.bits() | (8 << (4 * (pin - 8))))});}
        else {gpioe.afrl.modify(|r, w| unsafe {w.bits(r.bits() | (8 << (4 * pin)))});}
      }
    },
    'g' => {
      let gpiog = &peripheral_ptr.GPIOG;
      rcc.ahb1enr.modify(|_, w| w.gpiogen().enabled());
      gpiog.moder.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin)) | (2 << (2 * pin)))});
      if channel < 4 {
        if pin > 7 {gpiog.afrh.modify(|r, w| unsafe {w.bits(r.bits() | (7 << (4 * (pin - 8))))});}
        else {gpiog.afrl.modify(|r, w| unsafe {w.bits(r.bits() | (7 << (4 * pin)))});}
      }
      else {
        if pin > 7 {gpiog.afrh.modify(|r, w| unsafe {w.bits(r.bits() | (8 << (4 * (pin - 8))))});}
        else {gpiog.afrl.modify(|r, w| unsafe {w.bits(r.bits() | (8 << (4 * pin)))});}
      }
    },
    _   => panic!("P{}{} is not available for UART transmissions!", block.to_uppercase(), pin)
  };
}

fn rx_interrupt(channel: u8, enable: bool) {
  let peripheral_ptr = stm32f4::stm32f446::Peripherals::take().unwrap();
  
  match channel {
    1 => {
      let usart1 = &peripheral_ptr.USART1;
      if enable == true {usart1.cr1.modify(|_, w| w.rxneie().enabled());}
      else {usart1.cr1.modify(|_, w| w.rxneie().disabled());}
    },
    2 => {
      let usart2 = &peripheral_ptr.USART2;
      if enable == true {usart2.cr1.modify(|_, w| w.rxneie().enabled());}
      else {usart2.cr1.modify(|_, w| w.rxneie().disabled());}
    },
    3 => {
      let usart3 = &peripheral_ptr.USART3;
      if enable == true {usart3.cr1.modify(|_, w| w.rxneie().enabled());}
      else {usart3.cr1.modify(|_, w| w.rxneie().disabled());}
    },
    4 => {
      let uart4 = &peripheral_ptr.UART4;
      if enable == true {uart4.cr1.modify(|_, w| w.rxneie().enabled());}
      else {uart4.cr1.modify(|_, w| w.rxneie().disabled());}
    },
    5 => {
      let uart5 = &peripheral_ptr.UART5;
      if enable == true {uart5.cr1.modify(|_, w| w.rxneie().enabled());}
      else {uart5.cr1.modify(|_, w| w.rxneie().disabled());}
    },
    6 => {
      let usart6 = &peripheral_ptr.USART6;
      if enable == true {usart6.cr1.modify(|_, w| w.rxneie().enabled());}
      else {usart6.cr1.modify(|_, w| w.rxneie().disabled());}
    },
    _ => panic!("{} is not a valid UART peripheral!", channel)
  };
}

fn tx_interrupt(channel: u8, enable: bool) {
  let peripheral_ptr = stm32f4::stm32f446::Peripherals::take().unwrap();

  match channel {
    1 => {
      let usart1 = &peripheral_ptr.USART1;
      if enable == true {usart1.cr1.modify(|_, w| w.tcie().enabled());}
      else {usart1.cr1.modify(|_, w| w.tcie().disabled());}
    },
    2 => {
      let usart2 = &peripheral_ptr.USART2;
      if enable == true {usart2.cr1.modify(|_, w| w.tcie().enabled());}
      else {usart2.cr1.modify(|_, w| w.tcie().disabled());}
    },
    3 => {
      let usart3 = &peripheral_ptr.USART3;
      if enable == true {usart3.cr1.modify(|_, w| w.tcie().enabled());}
      else {usart3.cr1.modify(|_, w| w.tcie().disabled());}
    },
    4 => {
      let uart4 = &peripheral_ptr.UART4;
      if enable == true {uart4.cr1.modify(|_, w| w.tcie().enabled());}
      else {uart4.cr1.modify(|_, w| w.tcie().disabled());}
    },
    5 => {
      let uart5 = &peripheral_ptr.UART5;
      if enable == true {uart5.cr1.modify(|_, w| w.tcie().enabled());}
      else {uart5.cr1.modify(|_, w| w.tcie().disabled());}
    },
    6 => {
      let usart6 = &peripheral_ptr.USART6;
      if enable == true {usart6.cr1.modify(|_, w| w.tcie().enabled());}
      else {usart6.cr1.modify(|_, w| w.tcie().disabled());}
    },
    _ => panic!("{} is not a valid UART peripheral!", channel)
  };
}

fn set_baud(channel: u8, baud: u32) {
  let peripheral_ptr = stm32f4::stm32f446::Peripherals::take().unwrap();

  // (Mantisse, Fractal)
  let usartdiv: (f64, f64) = libm::modf(16000000.0 / (16.0 * baud as f64));

  match channel {
    1 => {
      let usart1 = &peripheral_ptr.USART1;
      usart1.brr.modify(|_, w| {
        w.div_mantissa().bits(usartdiv.0 as u16);
        w.div_fraction().bits(usartdiv.1 as u8)
      });
    },
    2 => {
      let usart2 = &peripheral_ptr.USART2;
      usart2.brr.modify(|_, w| {
        w.div_mantissa().bits(usartdiv.0 as u16);
        w.div_fraction().bits(usartdiv.1 as u8)
      });
    },
    3 => {
      let usart3 = &peripheral_ptr.USART3;
      usart3.brr.modify(|_, w| {
        w.div_mantissa().bits(usartdiv.0 as u16);
        w.div_fraction().bits(usartdiv.1 as u8)
      });
    },
    4 => {
      let uart4 = &peripheral_ptr.UART4;
      uart4.brr.modify(|_, w| {
        w.div_mantissa().bits(usartdiv.0 as u16);
        w.div_fraction().bits(usartdiv.1 as u8)
      });
    },
    5 => {
      let uart5 = &peripheral_ptr.UART5;
      uart5.brr.modify(|_, w| {
        w.div_mantissa().bits(usartdiv.0 as u16);
        w.div_fraction().bits(usartdiv.1 as u8)
      });
    },
    6 => {
      let usart6 = &peripheral_ptr.USART6;
      usart6.brr.modify(|_, w| {
        w.div_mantissa().bits(usartdiv.0 as u16);
        w.div_fraction().bits(usartdiv.1 as u8)
      });
    },
    _ => panic!("{} is not a valid UART peripheral!", channel)
  }
}


// UART Serial connection =========================================================================
pub mod serial {
  use libm::*;
  use cortex_m_semihosting::hprintln;
  use super::super::include::UART_CONF;

  pub fn init(baud: u32, rxint: bool, txint: bool) {
    let peripheral_ptr = stm32f4::stm32f446::Peripherals::take().unwrap();
    let rcc = &peripheral_ptr.RCC;
    let usart2 = &peripheral_ptr.USART2;
    let gpioa = &peripheral_ptr.GPIOA;
    
    // (Mantisse, Fractal)
    let usartdiv: (f64, f64) = modf(16000000.0 / (16.0 * baud as f64));

    unsafe {
      if UART_CONF[1] == true {
        hprintln!("Serial connection already configured!").expect("Could not send semihosting message!");
        usart2.cr1.modify(|_, w| w.ue().disabled());
      }
    }
    
    rcc.apb1enr.modify(|_, w| w.usart2en().enabled());
    
    usart2.brr.write(|w| {
      w.div_mantissa().bits(usartdiv.0 as u16);
      w.div_fraction().bits(usartdiv.1 as u8)
    });
    
    if rxint == true {usart2.cr1.modify(|_, w| w.rxneie().enabled());}
    if txint == true {usart2.cr1.modify(|_, w| w.tcie().enabled());}
    
    usart2.cr1.modify(|_, w| {
      w.re().enabled();
      w.te().enabled();
      w.ue().enabled()
    });
    
    unsafe {UART_CONF[1] = true;}
    
    rcc.ahb1enr.modify(|_, w| w.gpioaen().enabled());
    gpioa.moder.modify(|_, w| w.moder2().alternate());
    gpioa.moder.modify(|_, w| w.moder3().alternate());
    gpioa.afrl.modify(|_, w| w.afrl2().af7());
    gpioa.afrl.modify(|_, w| w.afrl3().af7());
  }

  pub fn send_char_usb(c: char) {
    let peripheral_ptr = stm32f4::stm32f446::Peripherals::take().unwrap();
    let usart2 = &peripheral_ptr.USART2;
    
    unsafe {if UART_CONF[1] == false {panic!("UART USB channel ist not PINCONFIGured!");}}

    if c.is_ascii() == true {
      while usart2.sr.read().txe().bit_is_set() == true {}
      usart2.dr.write(|w| w.dr().bits(c as u16));
      while usart2.sr.read().txe().bit_is_set() == true {}
    }
    else {
      hprintln!("{} is not an ASCII character!", c).expect("Could not send semihosting message!");

      while usart2.sr.read().txe().bit_is_set() == true {}
      usart2.dr.write(|w| w.dr().bits('?' as u16));
      while usart2.sr.read().txe().bit_is_set() == true {}
    }
  }
  
  pub fn recieve_char_usb() -> char {
    let peripheral_ptr = stm32f4::stm32f446::Peripherals::take().unwrap();
    let usart2 = &peripheral_ptr.USART2;
    let buffer: u8;
    
    unsafe {if UART_CONF[1] == false {panic!("UART USB channel ist not PINCONFIGured!");}}
    
    while usart2.sr.read().rxne().bit_is_clear() == true {}
    buffer = usart2.dr.read().dr().bits() as u8;
    
    return buffer as char;
  }


  // Macro declerations ===========================================================================
  #[macro_export]
  macro_rules! sprint {
    ($param:expr) => {
      let mut txt_buff: String<50> = String::new();
      if core::fmt::write(&mut txt_buff, format_args!($param)).is_err() {txt_buff = String::from("~\r\n")};
    
      for c in txt_buff.chars() {
        if c.is_ascii() == true {rustuino::serial::send_char_usb(c);}
        else {rustuino::serial::send_char_usb('?');}
      }
    };
  }

  #[macro_export]
  macro_rules! sprintln {
    ($param:expr) => {
      let mut txt_buff: String<50> = String::new();
      if core::fmt::write(&mut txt_buff, format_args!($param)).is_err() {txt_buff = String::from("~\r\n")};
    
      for c in txt_buff.chars() {
        if c.is_ascii() == true {rustuino::serial::send_char_usb(c);}
        else {rustuino::serial::send_char_usb('?');}
      }

      rustuino::serial::send_char_usb('\r');
      rustuino::serial::send_char_usb('\n');
    };
  }

  #[macro_export]
  macro_rules! sread {
    () => {{
      let c_buff: char = rustuino::serial::recieve_char_usb();  
      c_buff
    }};

    ($c:expr) => {{
      let found: bool;

      if rustuino::serial::recieve_char_usb() == $c {found = true;}
      else {found = false;}

      found
    }};
  }

  #[macro_export]
  macro_rules! sreads {
    ($stop:expr) => {{
      let mut str: String<50> = String::new();
      let mut buff: char;
      loop {
        buff = rustuino::serial::recieve_char_usb();
        if buff == $stop as char {break;}
        str.push(buff).expect("String buffer full!");
      }
      str
    }};
  }
}
