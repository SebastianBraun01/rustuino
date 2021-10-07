//! This module contains everything that is used for UART communication.

use crate::include::{stm_peripherals, SerialError, ProgError, UART_MAP, pins::PIN_CONF};
use crate::gpio::pinmode_alternate_function;
use stm32f4::stm32f446::{NVIC, Interrupt};
use rtt_target::rprintln;

// 8 = bits, 4 = stops, 2,1 = parity
pub const UART_8N1: u8 = 0;
pub const UART_8N2: u8 = 4;
pub const UART_8E1: u8 = 1;
pub const UART_8E2: u8 = 5;
pub const UART_8O1: u8 = 2;
pub const UART_8O2: u8 = 6;
pub const UART_9N1: u8 = 8;
pub const UART_9N2: u8 = 12;
pub const UART_9E1: u8 = 9;
pub const UART_9E2: u8 = 13;
pub const UART_9O1: u8 = 10;
pub const UART_9O2: u8 = 14;

pub struct UART {
  core: u8
}

impl UART {
  pub fn new(core: u8, tx_pin: (char, u8), rx_pin: (char, u8), baud: u32, conf: u8) -> Result<Self, ProgError> {
    let peripheral_ptr = stm_peripherals();
    let rcc = &peripheral_ptr.RCC;

    let af = if core == 1 || core == 2 || core == 3 {7}
    else {8};
    
    if UART_MAP.tx_pins.iter().zip(UART_MAP.rx_pins.iter())
    .zip(UART_MAP.cores.iter()).any(|i| i == ((&tx_pin, &rx_pin), &core)) == false {
      rprintln!("These pins are not available for UART communication! | UART::new()");
      return Err(ProgError::InvalidConfiguration);
    }

    unsafe {
      if PIN_CONF.contains(&tx_pin) || PIN_CONF.contains(&rx_pin) {
        rprintln!("These pins are already configured for another function! | UART::new()");
        return Err(ProgError::InvalidConfiguration);
      }
      else {
        PIN_CONF.push(tx_pin).expect("Could not store pin number! | UART::new()");
        PIN_CONF.push(rx_pin).expect("Could not store pin number! | UART::new()");
      }
    }

    if let Err(_) = pinmode_alternate_function(tx_pin, af) {return Err(ProgError::Internal);}
    if let Err(_) = pinmode_alternate_function(rx_pin, af) {return Err(ProgError::Internal);}
    
    match core {
      1 => {
        let uart1 = &peripheral_ptr.USART1;
        if rcc.apb2enr.read().usart1en().is_enabled() == true {
          rprintln!("U(S)ART{} is already configured! | UART::new()", core);
          return Err(ProgError::InvalidConfiguration);
        }
        rcc.apb2enr.modify(|_, w| w.usart1en().enabled());
        set_baud(core, baud);
        if conf & 8 > 0 {uart1.cr1.modify(|_, w| w.m().m9());}
        if conf & 4 > 0 {uart1.cr2.modify(|_, w| w.stop().stop2());}
        if conf & 3 == 1 {uart1.cr1.modify(|_, w| w.pce().enabled());}
        else if conf & 3 == 2 {uart1.cr1.modify(|_, w| {w.ps().odd(); w.pce().enabled()});}
        uart1.cr1.modify(|_, w| {
          w.te().enabled();
          w.re().enabled();
          w.ue().enabled()
        });
      },
      2 => {
        let uart2 = &peripheral_ptr.USART2;
        if rcc.apb1enr.read().usart2en().is_enabled() == true {
          rprintln!("U(S)ART{} is already configured! | UART::new()", core);
          return Err(ProgError::InvalidConfiguration);
        }
        rcc.apb1enr.modify(|_, w| w.usart2en().enabled());
        set_baud(core, baud);
        if conf & 8 > 0 {uart2.cr1.modify(|_, w| w.m().m9());}
        if conf & 4 > 0 {uart2.cr2.modify(|_, w| w.stop().stop2());}
        if conf & 3 == 1 {uart2.cr1.modify(|_, w| w.pce().enabled());}
        else if conf & 3 == 2 {uart2.cr1.modify(|_, w| {w.ps().odd(); w.pce().enabled()});}
        uart2.cr1.modify(|_, w| {
          w.te().enabled();
          w.re().enabled();
          w.ue().enabled()
        });
      },
      3 => {
        let uart3 = &peripheral_ptr.USART3;
        if rcc.apb1enr.read().usart3en().is_enabled() == true {
          rprintln!("U(S)ART{} is already configured! | UART::new()", core);
          return Err(ProgError::InvalidConfiguration);
        }
        rcc.apb1enr.modify(|_, w| w.usart3en().enabled());
        set_baud(core, baud);
        if conf & 8 > 0 {uart3.cr1.modify(|_, w| w.m().m9());}
        if conf & 4 > 0 {uart3.cr2.modify(|_, w| w.stop().stop2());}
        if conf & 3 == 1 {uart3.cr1.modify(|_, w| w.pce().enabled());}
        else if conf & 3 == 2 {uart3.cr1.modify(|_, w| {w.ps().odd(); w.pce().enabled()});}
        uart3.cr1.modify(|_, w| {
          w.te().enabled();
          w.re().enabled();
          w.ue().enabled()
        });
      },
      4 => {
        let uart4 = &peripheral_ptr.UART4;
        if rcc.apb1enr.read().uart4en().is_enabled() == true {
          rprintln!("U(S)ART{} is already configured! | UART::new()", core);
          return Err(ProgError::InvalidConfiguration);
        }
        rcc.apb1enr.modify(|_, w| w.uart4en().enabled());
        set_baud(core, baud);
        if conf & 8 > 0 {uart4.cr1.modify(|_, w| w.m().m9());}
        if conf & 4 > 0 {uart4.cr2.modify(|_, w| w.stop().stop2());}
        if conf & 3 == 1 {uart4.cr1.modify(|_, w| w.pce().enabled());}
        else if conf & 3 == 2 {uart4.cr1.modify(|_, w| {w.ps().odd(); w.pce().enabled()});}
        uart4.cr1.modify(|_, w| {
          w.te().enabled();
          w.re().enabled();
          w.ue().enabled()
        });
      },
      5 => {
        let uart5 = &peripheral_ptr.UART5;
        if rcc.apb1enr.read().uart5en().is_enabled() == true {
          rprintln!("U(S)ART{} is already configured! | UART::new()", core);
          return Err(ProgError::InvalidConfiguration);
        }
        rcc.apb1enr.modify(|_, w| w.uart5en().enabled());
        set_baud(core, baud);
        if conf & 8 > 0 {uart5.cr1.modify(|_, w| w.m().m9());}
        if conf & 4 > 0 {uart5.cr2.modify(|_, w| w.stop().stop2());}
        if conf & 3 == 1 {uart5.cr1.modify(|_, w| w.pce().enabled());}
        else if conf & 3 == 2 {uart5.cr1.modify(|_, w| {w.ps().odd(); w.pce().enabled()});}
        uart5.cr1.modify(|_, w| {
          w.te().enabled();
          w.re().enabled();
          w.ue().enabled()
        });
      },
      6 => {
        let uart6 = &peripheral_ptr.USART6;
        if rcc.apb2enr.read().usart6en().is_enabled() == true {
          rprintln!("U(S)ART{} is already configured! | UART::new()", core);
          return Err(ProgError::InvalidConfiguration);
        }
        rcc.apb2enr.modify(|_, w| w.usart6en().enabled());
        set_baud(core, baud);
        if conf & 8 > 0 {uart6.cr1.modify(|_, w| w.m().m9());}
        if conf & 4 > 0 {uart6.cr2.modify(|_, w| w.stop().stop2());}
        if conf & 3 == 1 {uart6.cr1.modify(|_, w| w.pce().enabled());}
        else if conf & 3 == 2 {uart6.cr1.modify(|_, w| {w.ps().odd(); w.pce().enabled()});}
        uart6.cr1.modify(|_, w| {
          w.te().enabled();
          w.re().enabled();
          w.ue().enabled()
        });
      },
      _ => {
        rprintln!("U(S)ART{} is not a valid U(S)ART peripheral! | UART::new()", core);
        return Err(ProgError::InvalidConfiguration);
      }
    };

    return Ok(Self {
      core
    });
  }

  pub fn end(self) {
    let peripheral_ptr = stm_peripherals();
    let rcc = &peripheral_ptr.RCC;

    match self.core {
      1 => {
        let uart1 = &peripheral_ptr.USART1;
        rcc.apb2enr.modify(|_, w| w.usart1en().disabled());
        uart1.cr1.reset();
        uart1.cr2.reset();
        NVIC::mask(Interrupt::USART1);
      },
      2 => {
        let uart2 = &peripheral_ptr.USART2;
        rcc.apb1enr.modify(|_, w| w.usart2en().disabled());
        uart2.cr1.reset();
        uart2.cr2.reset();
        NVIC::mask(Interrupt::USART2);
      },
      3 => {
        let uart3 = &peripheral_ptr.USART3;
        rcc.apb1enr.modify(|_, w| w.usart3en().disabled());
        uart3.cr1.reset();
        uart3.cr2.reset();
        NVIC::mask(Interrupt::USART3);
      },
      4 => {
        let uart4 = &peripheral_ptr.UART4;
        rcc.apb1enr.modify(|_, w| w.uart4en().disabled());
        uart4.cr1.reset();
        uart4.cr2.reset();
        NVIC::mask(Interrupt::UART4);
      },
      5 => {
        let uart5 = &peripheral_ptr.UART5;
        rcc.apb1enr.modify(|_, w| w.uart5en().disabled());
        uart5.cr1.reset();
        uart5.cr2.reset();
        NVIC::mask(Interrupt::UART5);
      },
      6 => {
        let uart6 = &peripheral_ptr.USART6;
        rcc.apb2enr.modify(|_, w| w.usart6en().disabled());
        uart6.cr1.reset();
        uart6.cr2.reset();
        NVIC::mask(Interrupt::USART6);
      },
      _ => unreachable!()
    };
  }

  pub fn print(&self, data: &str) -> Result<(), SerialError> {
    let peripheral_ptr = stm_peripherals();
    
    let bytes = data.as_bytes();
    
    match self.core {
      1 => {
        let uart1 = &peripheral_ptr.USART1;
        for byte in bytes {
          while uart1.sr.read().txe().bit_is_clear() == true {
            if let Err(error) = check_uart_errors(uart1.sr.read().bits()) {return Err(error);}
          }
          uart1.dr.write(|w| w.dr().bits(byte.clone().into()));
        }
      },
      2 => {
        let uart2 = &peripheral_ptr.USART2;
        for byte in bytes {
          while uart2.sr.read().txe().bit_is_clear() == true {
            if let Err(error) = check_uart_errors(uart2.sr.read().bits()) {return Err(error);}
          }
          uart2.dr.write(|w| w.dr().bits(byte.clone().into()));
        }
      },
      3 => {
        let uart3 = &peripheral_ptr.USART3;
        for byte in bytes {
          while uart3.sr.read().txe().bit_is_clear() == true {
            if let Err(error) = check_uart_errors(uart3.sr.read().bits()) {return Err(error);}
          }
          uart3.dr.write(|w| w.dr().bits(byte.clone().into()));
        }
      },
      4 => {
        let uart4 = &peripheral_ptr.UART4;
        for byte in bytes {
          while uart4.sr.read().txe().bit_is_clear() == true {
            if let Err(error) = check_uart_errors(uart4.sr.read().bits()) {return Err(error);}
          }
          uart4.dr.write(|w| w.dr().bits(byte.clone().into()));
        }
      },
      5 => {
        let uart5 = &peripheral_ptr.UART5;
        for byte in bytes {
          while uart5.sr.read().txe().bit_is_clear() == true {
            if let Err(error) = check_uart_errors(uart5.sr.read().bits()) {return Err(error);}
          }
          uart5.dr.write(|w| w.dr().bits(byte.clone().into()));
        }
      },
      6 => {
        let uart6 = &peripheral_ptr.USART6;
        for byte in bytes {
          while uart6.sr.read().txe().bit_is_clear() == true {
            if let Err(error) = check_uart_errors(uart6.sr.read().bits()) {return Err(error);}
          }
          uart6.dr.write(|w| w.dr().bits(byte.clone().into()));
        }
      },
      _ => unreachable!()
    };

    return Ok(());
  }

  pub fn println(&self, data: &str) -> Result<(), SerialError> {
    if let Err(error) = self.print(data) {return Err(error);}
    if let Err(error) = self.print("\r\n") {return Err(error);}

    return Ok(());
  }

  pub fn write(&self, data: u8) -> Result<(), SerialError> {
    let peripheral_ptr = stm_peripherals();

    match self.core {
      1 => {
        let uart1 = &peripheral_ptr.USART1;
        while uart1.sr.read().txe().bit_is_clear() == true {
          if let Err(error) = check_uart_errors(uart1.sr.read().bits()) {return Err(error);}
        }
        uart1.dr.write(|w| w.dr().bits(data.into()));
      },
      2 => {
        let uart2 = &peripheral_ptr.USART2;
        while uart2.sr.read().txe().bit_is_clear() == true {
          if let Err(error) = check_uart_errors(uart2.sr.read().bits()) {return Err(error);}
        }
        uart2.dr.write(|w| w.dr().bits(data.into()));
      },
      3 => {
        let uart3 = &peripheral_ptr.USART3;
        while uart3.sr.read().txe().bit_is_clear() == true {
          if let Err(error) = check_uart_errors(uart3.sr.read().bits()) {return Err(error);}
        }
        uart3.dr.write(|w| w.dr().bits(data.into()));
      },
      4 => {
        let uart4 = &peripheral_ptr.UART4;
        while uart4.sr.read().txe().bit_is_clear() == true {
          if let Err(error) = check_uart_errors(uart4.sr.read().bits()) {return Err(error);}
        }
        uart4.dr.write(|w| w.dr().bits(data.into()));
      },
      5 => {
        let uart5 = &peripheral_ptr.UART5;
        while uart5.sr.read().txe().bit_is_clear() == true {
          if let Err(error) = check_uart_errors(uart5.sr.read().bits()) {return Err(error);}
        }
        uart5.dr.write(|w| w.dr().bits(data.into()));
      },
      6 => {
        let uart6 = &peripheral_ptr.USART6;
        while uart6.sr.read().txe().bit_is_clear() == true {
          if let Err(error) = check_uart_errors(uart6.sr.read().bits()) {return Err(error);}
        }
        uart6.dr.write(|w| w.dr().bits(data.into()));
      },
      _ => unreachable!()
    };

    return Ok(());
  }

  pub fn read_char(&self) -> Option<char> {
    let peripheral_ptr = stm_peripherals();

    let buffer: u8;
    
    match self.core {
      1 => {
        let uart1 = &peripheral_ptr.USART1;
        while uart1.sr.read().rxne().bit_is_clear() == true {
          if let Err(_) = check_uart_errors(uart1.sr.read().bits()) {return None;}
        }
        buffer = uart1.dr.read().dr().bits() as u8;
      },
      2 => {
        let uart2 = &peripheral_ptr.USART2;
        while uart2.sr.read().rxne().bit_is_clear() == true {
          if let Err(_) = check_uart_errors(uart2.sr.read().bits()) {return None;}
        }
        buffer = uart2.dr.read().dr().bits() as u8;
      },
      3 => {
        let uart3 = &peripheral_ptr.USART3;
        while uart3.sr.read().rxne().bit_is_clear() == true {
          if let Err(_) = check_uart_errors(uart3.sr.read().bits()) {return None;}
        }
        buffer = uart3.dr.read().dr().bits() as u8;
      },
      4 => {
        let uart4 = &peripheral_ptr.UART4;
        while uart4.sr.read().rxne().bit_is_clear() == true {
          if let Err(_) = check_uart_errors(uart4.sr.read().bits()) {return None;}
        }
        buffer = uart4.dr.read().dr().bits() as u8;
      },
      5 => {
        let uart5 = &peripheral_ptr.UART5;
        while uart5.sr.read().rxne().bit_is_clear() == true {
          if let Err(_) = check_uart_errors(uart5.sr.read().bits()) {return None;}
        }
        buffer = uart5.dr.read().dr().bits() as u8;
      },
      6 => {
        let uart6 = &peripheral_ptr.USART6;
        while uart6.sr.read().rxne().bit_is_clear() == true {
          if let Err(_) = check_uart_errors(uart6.sr.read().bits()) {return None;}
        }
        buffer = uart6.dr.read().dr().bits() as u8;
      },
      _ => unreachable!()
    };

    return Some(buffer as char);
  }

  pub fn read_byte(&self) -> Option<u8> {
    let peripheral_ptr = stm_peripherals();

    let buffer: u8;
    
    match self.core {
      1 => {
        let uart1 = &peripheral_ptr.USART1;
        while uart1.sr.read().rxne().bit_is_clear() == true {
          if let Err(_) = check_uart_errors(uart1.sr.read().bits()) {return None;}
        }
        buffer = uart1.dr.read().dr().bits() as u8;
      },
      2 => {
        let uart2 = &peripheral_ptr.USART2;
        while uart2.sr.read().rxne().bit_is_clear() == true {
          if let Err(_) = check_uart_errors(uart2.sr.read().bits()) {return None;}
        }
        buffer = uart2.dr.read().dr().bits() as u8;
      },
      3 => {
        let uart3 = &peripheral_ptr.USART3;
        while uart3.sr.read().rxne().bit_is_clear() == true {
          if let Err(_) = check_uart_errors(uart3.sr.read().bits()) {return None;}
        }
        buffer = uart3.dr.read().dr().bits() as u8;
      },
      4 => {
        let uart4 = &peripheral_ptr.UART4;
        while uart4.sr.read().rxne().bit_is_clear() == true {
          if let Err(_) = check_uart_errors(uart4.sr.read().bits()) {return None;}
        }
        buffer = uart4.dr.read().dr().bits() as u8;
      },
      5 => {
        let uart5 = &peripheral_ptr.UART5;
        while uart5.sr.read().rxne().bit_is_clear() == true {
          if let Err(_) = check_uart_errors(uart5.sr.read().bits()) {return None;}
        }
        buffer = uart5.dr.read().dr().bits() as u8;
      },
      6 => {
        let uart6 = &peripheral_ptr.USART6;
        while uart6.sr.read().rxne().bit_is_clear() == true {
          if let Err(_) = check_uart_errors(uart6.sr.read().bits()) {return None;}
        }
        buffer = uart6.dr.read().dr().bits() as u8;
      },
      _ => unreachable!()
    };

    return Some(buffer);
  }
}
  
  
// Private Functions ==============================================================================
fn check_uart_errors(sr: u32) -> Result<(), SerialError> {
  let bits = sr & 0xF;

  if bits & 0x8 > 1 {return Err(SerialError::Overrun);}
  else if bits & 0x4 > 1 {return Err(SerialError::Noise);}
  else if bits & 0x2 > 1 {return Err(SerialError::FrameFormat);}
  else if bits & 0x1 > 1 {return Err(SerialError::Parity);}

  return Ok(());
}

fn set_baud(core: u8, baud: u32) {
  let peripheral_ptr = stm_peripherals();
  
  // (Mantisse, Fractal)
  let uartdiv: (f64, f64) = modf(16000000.0 / (16.0 * baud as f64));

  match core {
    1 => {
      let uart1 = &peripheral_ptr.USART1;
      uart1.brr.modify(|_, w| {
        w.div_mantissa().bits(uartdiv.1 as u16);
        w.div_fraction().bits((uartdiv.0 * 16.0) as u8)
      });
    },
    2 => {
      let uart2 = &peripheral_ptr.USART2;
      uart2.brr.modify(|_, w| {
        w.div_mantissa().bits(uartdiv.1 as u16);
        w.div_fraction().bits((uartdiv.0 * 16.0) as u8)
      });
    },
    3 => {
      let uart3 = &peripheral_ptr.USART3;
      uart3.brr.modify(|_, w| {
        w.div_mantissa().bits(uartdiv.1 as u16);
        w.div_fraction().bits((uartdiv.0 * 16.0) as u8)
      });
    },
    4 => {
      let uart4 = &peripheral_ptr.UART4;
      uart4.brr.modify(|_, w| {
        w.div_mantissa().bits(uartdiv.1 as u16);
        w.div_fraction().bits((uartdiv.0 * 16.0) as u8)
      });
    },
    5 => {
      let uart5 = &peripheral_ptr.UART5;
      uart5.brr.modify(|_, w| {
        w.div_mantissa().bits(uartdiv.1 as u16);
        w.div_fraction().bits((uartdiv.0 * 16.0) as u8)
      });
    },
    6 => {
      let uart6 = &peripheral_ptr.USART6;
      uart6.brr.modify(|_, w| {
        w.div_mantissa().bits(uartdiv.1 as u16);
        w.div_fraction().bits((uartdiv.0 * 16.0) as u8)
      });
    },
    _ => unreachable!()
  }
}


pub fn modf(x: f64) -> (f64, f64) {
  let rv2: f64;
  let mut u = x.to_bits();
  let mask: u64;
  let e = ((u >> 52 & 0x7ff) as i32) - 0x3ff;

  // no fractional part
  if e >= 52 {
      rv2 = x;
      if e == 0x400 && (u << 12) != 0 {
          /* nan */
          return (x, rv2);
      }
      u &= 1 << 63;
      return (f64::from_bits(u), rv2);
  }

  // no integral part
  if e < 0 {
      u &= 1 << 63;
      rv2 = f64::from_bits(u);
      return (x, rv2);
  }

  mask = ((!0) >> 12) >> e;
  if (u & mask) == 0 {
      rv2 = x;
      u &= 1 << 63;
      return (f64::from_bits(u), rv2);
  }
  u &= !mask;
  rv2 = f64::from_bits(u);
  return (x - rv2, rv2);
}
