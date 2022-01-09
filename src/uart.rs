//! This module contains everything that is used for UART communication.
//! 
//! For information on whitch pins have UART capabilities, check [`UART_MAP`](crate::include::UART_MAP)
//! 
//! # Examples
//! 
//! ```no_run
//! #![no_std]
//! #![no_main]
//! 
//! use rustuino::*;
//! 
//! #[entry]
//! fn main() -> ! {
//!   // Configure the serial connection
//!   let uart = UART::new(2, PA2, PA3, 115200).unwrap();
//! 
//!   loop {
//!     // Send Message
//!     uart.println("Hello World!");
//!     delay(1000);   
//!   }
//! }
//! ```

use crate::include::{SerialError, ProgError, UART_MAP, PIN_CONF};
use crate::gpio::{pinmode_alternate_function, Pin, AlternateFunction};
use stm32f4::stm32f446::{NVIC, Interrupt};
use rtt_target::rprintln;

/// This struct represents a configured UART peripheral.
pub struct UART {
  #[doc(hidden)]
  core: u8,
  #[doc(hidden)]
  _tx_pin: Pin<AlternateFunction>,
  #[doc(hidden)]
  _rx_pin: Pin<AlternateFunction>
}

impl UART {
  /// Configure a serial connection with one of the internal UART peripherals.
  /// 
  /// This Method expects the used UART core, two [pin identifiers](crate::include::pins) for the tx and rx-pins
  /// and a baudrate as parameters and returns the [UART Struct](crate::uart::UART). Panics if the core or pins
  /// are already used or invalid.
  pub fn new(core: u8, tx_pin: (char, u8), rx_pin: (char, u8), baud: u32) -> Result<Self, ProgError> {
    let peripheral_ptr;
    unsafe {peripheral_ptr = stm32f4::stm32f446::Peripherals::steal();}
    let rcc = &peripheral_ptr.RCC;

    let af = if core == 1 || core == 2 || core == 3 {7}
    else {8};
    
    if !UART_MAP.tx_pins.iter().zip(UART_MAP.rx_pins.iter())
    .zip(UART_MAP.cores.iter()).any(|i| i == ((&tx_pin, &rx_pin), &core)) {
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

    let tx = match pinmode_alternate_function(tx_pin, af) {
      Ok(value) => value,
      Err(_) => return Err(ProgError::Internal)
    };

    let rx = match pinmode_alternate_function(rx_pin, af) {
      Ok(value) => value,
      Err(_) => return Err(ProgError::Internal)
    };
    
    match core {
      1 => {
        let uart1 = &peripheral_ptr.USART1;
        if rcc.apb2enr.read().usart1en().is_enabled() {
          rprintln!("U(S)ART{} is already configured! | UART::new()", core);
          return Err(ProgError::InvalidConfiguration);
        }
        rcc.apb2enr.modify(|_, w| w.usart1en().enabled());
        set_baud(core, baud);
        uart1.cr1.modify(|_, w| {
          w.te().enabled();
          w.re().enabled();
          w.ue().enabled()
        });
      },
      2 => {
        let uart2 = &peripheral_ptr.USART2;
        if rcc.apb1enr.read().usart2en().is_enabled() {
          rprintln!("U(S)ART{} is already configured! | UART::new()", core);
          return Err(ProgError::InvalidConfiguration);
        }
        rcc.apb1enr.modify(|_, w| w.usart2en().enabled());
        set_baud(core, baud);
        uart2.cr1.modify(|_, w| {
          w.te().enabled();
          w.re().enabled();
          w.ue().enabled()
        });
      },
      3 => {
        let uart3 = &peripheral_ptr.USART3;
        if rcc.apb1enr.read().usart3en().is_enabled() {
          rprintln!("U(S)ART{} is already configured! | UART::new()", core);
          return Err(ProgError::InvalidConfiguration);
        }
        rcc.apb1enr.modify(|_, w| w.usart3en().enabled());
        set_baud(core, baud);
        uart3.cr1.modify(|_, w| {
          w.te().enabled();
          w.re().enabled();
          w.ue().enabled()
        });
      },
      4 => {
        let uart4 = &peripheral_ptr.UART4;
        if rcc.apb1enr.read().uart4en().is_enabled() {
          rprintln!("U(S)ART{} is already configured! | UART::new()", core);
          return Err(ProgError::InvalidConfiguration);
        }
        rcc.apb1enr.modify(|_, w| w.uart4en().enabled());
        set_baud(core, baud);
        uart4.cr1.modify(|_, w| {
          w.te().enabled();
          w.re().enabled();
          w.ue().enabled()
        });
      },
      5 => {
        let uart5 = &peripheral_ptr.UART5;
        if rcc.apb1enr.read().uart5en().is_enabled() {
          rprintln!("U(S)ART{} is already configured! | UART::new()", core);
          return Err(ProgError::InvalidConfiguration);
        }
        rcc.apb1enr.modify(|_, w| w.uart5en().enabled());
        set_baud(core, baud);
        uart5.cr1.modify(|_, w| {
          w.te().enabled();
          w.re().enabled();
          w.ue().enabled()
        });
      },
      6 => {
        let uart6 = &peripheral_ptr.USART6;
        if rcc.apb2enr.read().usart6en().is_enabled() {
          rprintln!("U(S)ART{} is already configured! | UART::new()", core);
          return Err(ProgError::InvalidConfiguration);
        }
        rcc.apb2enr.modify(|_, w| w.usart6en().enabled());
        set_baud(core, baud);
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
      core,
      _tx_pin: tx,
      _rx_pin: rx
    });
  }

  /// Deacitivates the UART connection and destroys the struct, freeing the core and pins.
  pub fn end(self) {
    let peripheral_ptr;
    unsafe {peripheral_ptr = stm32f4::stm32f446::Peripherals::steal();}
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

    drop(self);
  }

  /// Sends an ASCII string over the serial connection. Returns an error-enum if problems with the connection are detected.
  pub fn print(&self, data: &str) -> Result<(), SerialError> {
    let peripheral_ptr;
    unsafe {peripheral_ptr = stm32f4::stm32f446::Peripherals::steal();}
    
    let bytes = data.as_bytes();
    
    match self.core {
      1 => {
        let uart1 = &peripheral_ptr.USART1;
        for byte in bytes {
          while uart1.sr.read().txe().bit_is_clear() {
            if let Err(error) = check_uart_errors(uart1.sr.read().bits()) {return Err(error);}
          }
          uart1.dr.write(|w| w.dr().bits((*byte).into()));
        }
      },
      2 => {
        let uart2 = &peripheral_ptr.USART2;
        for byte in bytes {
          while uart2.sr.read().txe().bit_is_clear() {
            if let Err(error) = check_uart_errors(uart2.sr.read().bits()) {return Err(error);}
          }
          uart2.dr.write(|w| w.dr().bits((*byte).into()));
        }
      },
      3 => {
        let uart3 = &peripheral_ptr.USART3;
        for byte in bytes {
          while uart3.sr.read().txe().bit_is_clear() {
            if let Err(error) = check_uart_errors(uart3.sr.read().bits()) {return Err(error);}
          }
          uart3.dr.write(|w| w.dr().bits((*byte).into()));
        }
      },
      4 => {
        let uart4 = &peripheral_ptr.UART4;
        for byte in bytes {
          while uart4.sr.read().txe().bit_is_clear() {
            if let Err(error) = check_uart_errors(uart4.sr.read().bits()) {return Err(error);}
          }
          uart4.dr.write(|w| w.dr().bits((*byte).into()));
        }
      },
      5 => {
        let uart5 = &peripheral_ptr.UART5;
        for byte in bytes {
          while uart5.sr.read().txe().bit_is_clear() {
            if let Err(error) = check_uart_errors(uart5.sr.read().bits()) {return Err(error);}
          }
          uart5.dr.write(|w| w.dr().bits((*byte).into()));
        }
      },
      6 => {
        let uart6 = &peripheral_ptr.USART6;
        for byte in bytes {
          while uart6.sr.read().txe().bit_is_clear() {
            if let Err(error) = check_uart_errors(uart6.sr.read().bits()) {return Err(error);}
          }
          uart6.dr.write(|w| w.dr().bits((*byte).into()));
        }
      },
      _ => unreachable!()
    };

    return Ok(());
  }

  /// Acts like [print](crate::uart::UART::print) except it prints a newline at the end of the string.
  pub fn println(&self, data: &str) -> Result<(), SerialError> {
    if let Err(error) = self.print(data) {return Err(error);}
    if let Err(error) = self.print("\r\n") {return Err(error);}

    return Ok(());
  }

  /// Sends a raw byte over the serial connection. Returns an error-enum if problems with the connection are detected.
  pub fn write(&self, data: u8) -> Result<(), SerialError> {
    let peripheral_ptr;
    unsafe {peripheral_ptr = stm32f4::stm32f446::Peripherals::steal();}

    match self.core {
      1 => {
        let uart1 = &peripheral_ptr.USART1;
        while uart1.sr.read().txe().bit_is_clear() {
          if let Err(error) = check_uart_errors(uart1.sr.read().bits()) {return Err(error);}
        }
        uart1.dr.write(|w| w.dr().bits(data.into()));
      },
      2 => {
        let uart2 = &peripheral_ptr.USART2;
        while uart2.sr.read().txe().bit_is_clear() {
          if let Err(error) = check_uart_errors(uart2.sr.read().bits()) {return Err(error);}
        }
        uart2.dr.write(|w| w.dr().bits(data.into()));
      },
      3 => {
        let uart3 = &peripheral_ptr.USART3;
        while uart3.sr.read().txe().bit_is_clear() {
          if let Err(error) = check_uart_errors(uart3.sr.read().bits()) {return Err(error);}
        }
        uart3.dr.write(|w| w.dr().bits(data.into()));
      },
      4 => {
        let uart4 = &peripheral_ptr.UART4;
        while uart4.sr.read().txe().bit_is_clear() {
          if let Err(error) = check_uart_errors(uart4.sr.read().bits()) {return Err(error);}
        }
        uart4.dr.write(|w| w.dr().bits(data.into()));
      },
      5 => {
        let uart5 = &peripheral_ptr.UART5;
        while uart5.sr.read().txe().bit_is_clear() {
          if let Err(error) = check_uart_errors(uart5.sr.read().bits()) {return Err(error);}
        }
        uart5.dr.write(|w| w.dr().bits(data.into()));
      },
      6 => {
        let uart6 = &peripheral_ptr.USART6;
        while uart6.sr.read().txe().bit_is_clear() {
          if let Err(error) = check_uart_errors(uart6.sr.read().bits()) {return Err(error);}
        }
        uart6.dr.write(|w| w.dr().bits(data.into()));
      },
      _ => unreachable!()
    };

    return Ok(());
  }

  /// Waits until it recieves an ASCII char. Returns an error-enum if problems with the connection are detected.
  pub fn read_char(&self) -> Option<char> {
    let peripheral_ptr;
    unsafe {peripheral_ptr = stm32f4::stm32f446::Peripherals::steal();}

    let buffer: u8;
    
    match self.core {
      1 => {
        let uart1 = &peripheral_ptr.USART1;
        while uart1.sr.read().rxne().bit_is_clear() {
          if check_uart_errors(uart1.sr.read().bits()).is_err() {return None;}
        }
        buffer = uart1.dr.read().dr().bits() as u8;
      },
      2 => {
        let uart2 = &peripheral_ptr.USART2;
        while uart2.sr.read().rxne().bit_is_clear() {
          if check_uart_errors(uart2.sr.read().bits()).is_err() {return None;}
        }
        buffer = uart2.dr.read().dr().bits() as u8;
      },
      3 => {
        let uart3 = &peripheral_ptr.USART3;
        while uart3.sr.read().rxne().bit_is_clear() {
          if check_uart_errors(uart3.sr.read().bits()).is_err() {return None;}
        }
        buffer = uart3.dr.read().dr().bits() as u8;
      },
      4 => {
        let uart4 = &peripheral_ptr.UART4;
        while uart4.sr.read().rxne().bit_is_clear() {
          if check_uart_errors(uart4.sr.read().bits()).is_err() {return None;}
        }
        buffer = uart4.dr.read().dr().bits() as u8;
      },
      5 => {
        let uart5 = &peripheral_ptr.UART5;
        while uart5.sr.read().rxne().bit_is_clear() {
          if check_uart_errors(uart5.sr.read().bits()).is_err() {return None;}
        }
        buffer = uart5.dr.read().dr().bits() as u8;
      },
      6 => {
        let uart6 = &peripheral_ptr.USART6;
        while uart6.sr.read().rxne().bit_is_clear() {
          if check_uart_errors(uart6.sr.read().bits()).is_err() {return None;}
        }
        buffer = uart6.dr.read().dr().bits() as u8;
      },
      _ => unreachable!()
    };

    return Some(buffer as char);
  }

  /// Waits until it recieves a byte. Returns an error-enum if problems with the connection are detected.
  pub fn read_byte(&self) -> Option<u8> {
    let peripheral_ptr;
    unsafe {peripheral_ptr = stm32f4::stm32f446::Peripherals::steal();}

    let buffer: u8;
    
    match self.core {
      1 => {
        let uart1 = &peripheral_ptr.USART1;
        while uart1.sr.read().rxne().bit_is_clear() {
          if check_uart_errors(uart1.sr.read().bits()).is_err() {return None;}
        }
        buffer = uart1.dr.read().dr().bits() as u8;
      },
      2 => {
        let uart2 = &peripheral_ptr.USART2;
        while uart2.sr.read().rxne().bit_is_clear() {
          if check_uart_errors(uart2.sr.read().bits()).is_err() {return None;}
        }
        buffer = uart2.dr.read().dr().bits() as u8;
      },
      3 => {
        let uart3 = &peripheral_ptr.USART3;
        while uart3.sr.read().rxne().bit_is_clear() {
          if check_uart_errors(uart3.sr.read().bits()).is_err() {return None;}
        }
        buffer = uart3.dr.read().dr().bits() as u8;
      },
      4 => {
        let uart4 = &peripheral_ptr.UART4;
        while uart4.sr.read().rxne().bit_is_clear() {
          if check_uart_errors(uart4.sr.read().bits()).is_err() {return None;}
        }
        buffer = uart4.dr.read().dr().bits() as u8;
      },
      5 => {
        let uart5 = &peripheral_ptr.UART5;
        while uart5.sr.read().rxne().bit_is_clear() {
          if check_uart_errors(uart5.sr.read().bits()).is_err() {return None;}
        }
        buffer = uart5.dr.read().dr().bits() as u8;
      },
      6 => {
        let uart6 = &peripheral_ptr.USART6;
        while uart6.sr.read().rxne().bit_is_clear() {
          if check_uart_errors(uart6.sr.read().bits()).is_err() {return None;}
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

  if bits & 0x8 == 8 {return Err(SerialError::Overrun);}
  else if bits & 0x4 == 4 {return Err(SerialError::Noise);}
  else if bits & 0x2 == 2 {return Err(SerialError::FrameFormat);}
  else if bits & 0x1 == 1 {return Err(SerialError::Parity);}

  return Ok(());
}

fn set_baud(core: u8, baud: u32) {
  let peripheral_ptr;
  unsafe {peripheral_ptr = stm32f4::stm32f446::Peripherals::steal();}
  
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


#[doc(hidden)]
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
