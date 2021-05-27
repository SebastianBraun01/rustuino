use heapless::{String, Vec};
use libm::*;
use super::Mode;
use super::include::{*, UART_MAP, UART_USB};
use super::gpio_d::pin_mode;
use super::include::RCC_PTR;
use super::include::{USART1_PTR, USART2_PTR, USART3_PTR, UART4_PTR, USART6_PTR};

pub fn uart_usb_init(baud: u32, rxint: bool, txint: bool) {
  // (Mantisse, Fractal)
  let usartdiv: (f64, f64) = modf(16000000.0 / (16.0 * baud as f64));

  unsafe {
    if UART_USB == true {panic!("UART USB channel is already configured!");}

    (*RCC_PTR).apb1enr.modify(|_, w| w.usart2en().enabled());

    (*USART2_PTR).brr.write(|w| {
      w.div_mantissa().bits(usartdiv.0 as u16);
      w.div_fraction().bits(usartdiv.1 as u8)
    });

    if rxint == true {(*USART2_PTR).cr1.modify(|_, w| w.rxneie().enabled());}
    if txint == true {(*USART2_PTR).cr1.modify(|_, w| w.tcie().enabled());}

    (*USART2_PTR).cr1.modify(|_, w| {
      w.re().enabled();
      w.te().enabled()
    });

    UART_USB = true;
  }

  pin_mode(PA2, Mode::AlterateFunction(7));
  pin_mode(PA3, Mode::AlterateFunction(7));
}

pub fn send_char_usb(c: char) {
  if c.is_ascii() == true {
    unsafe {
      if UART_USB == false {panic!("UART USB channel ist not configured!");}

      while (*USART2_PTR).sr.read().txe().bit_is_set() == true {}
      (*USART2_PTR).dr.write(|w| w.dr().bits(c as u16));
      while (*USART2_PTR).sr.read().txe().bit_is_set() == true {}
    }
  }
  else {panic!("{} is not an ASCII character!", c);}
}

pub fn recieve_char_usb() -> char {
  let buffer: u8;

  unsafe {
    if UART_USB == false {panic!("UART USB channel ist not configured!");}

    while (*USART2_PTR).sr.read().rxne().bit_is_clear() == true {}
    buffer = (*USART2_PTR).dr.read().dr().bits() as u8;
  }

  return buffer as char;
}

// TODO: finish not usb uart functions
pub fn uart_init(baud: u32, rxint: bool, txint: bool) {  
  // (Mantisse, Fractal)
  let usartdiv: (f64, f64) = modf(16000000.0 / (16.0 * baud as f64));
  let mut channels: Vec<(u8, bool), 5> = Vec::new();

  unsafe {
    for i in 0..CONFIG.pin.len() {
      if CONFIG.alternate[i] == 7 || CONFIG.alternate[i] == 8 {
        if UART_MAP.rx_pin.contains(&CONFIG.pin[i]) {
          channels.push((UART_MAP.channel[UART_MAP.rx_pin.iter().position(|&r| r == CONFIG.pin[i]).unwrap()], false))
          .expect("Could not configure UART channel!");
          UART_MAP.active[UART_MAP.rx_pin.iter().position(|&r| r == CONFIG.pin[i]).unwrap()] = true;
        }else if UART_MAP.tx_pin.contains(&CONFIG.pin[i]) {
          channels.push((UART_MAP.channel[UART_MAP.tx_pin.iter().position(|&r| r == CONFIG.pin[i]).unwrap()], true))
          .expect("Could not configure UART channel!");
          UART_MAP.active[UART_MAP.rx_pin.iter().position(|&r| r == CONFIG.pin[i]).unwrap()] = true;
        }
        else {panic!("P{}{} is not available for UART connection!", CONFIG.pin[i].1.to_uppercase(), CONFIG.pin[i].0);}
      }
    }

    for i in 0..channels.len() {
      if channels[i].1 == false {
        match channels[i].0 {
          1 => {
            (*RCC_PTR).apb2enr.modify(|_, w| w.usart1en().enabled());

            (*USART1_PTR).brr.modify(|_, w| {
              w.div_mantissa().bits(usartdiv.0 as u16);
              w.div_fraction().bits(usartdiv.1 as u8)
            });

            if rxint == true {(*USART1_PTR).cr1.modify(|_, w| w.rxneie().enabled());}
            if txint == true {(*USART1_PTR).cr1.modify(|_, w| w.tcie().enabled());}

            (*USART1_PTR).cr1.modify(|_, w| w.re().enabled());
          },
          3 => {
            (*RCC_PTR).apb1enr.modify(|_, w| w.usart3en().enabled());

            (*USART3_PTR).brr.modify(|_, w| {
              w.div_mantissa().bits(usartdiv.0 as u16);
              w.div_fraction().bits(usartdiv.1 as u8)
            });

            if rxint == true {(*USART3_PTR).cr1.modify(|_, w| w.rxneie().enabled());}
            if txint == true {(*USART3_PTR).cr1.modify(|_, w| w.tcie().enabled());}

            (*USART3_PTR).cr1.modify(|_, w| w.re().enabled());
          },
          4 => {
            (*RCC_PTR).apb1enr.modify(|_, w| w.uart4en().enabled());

            (*UART4_PTR).brr.modify(|_, w| {
              w.div_mantissa().bits(usartdiv.0 as u16);
              w.div_fraction().bits(usartdiv.1 as u8)
            });

            if rxint == true {(*UART4_PTR).cr1.modify(|_, w| w.rxneie().enabled());}
            if txint == true {(*UART4_PTR).cr1.modify(|_, w| w.tcie().enabled());}

            (*UART4_PTR).cr1.modify(|_, w| w.re().enabled());
          },
          6 => {
            (*RCC_PTR).apb2enr.modify(|_, w| w.usart6en().enabled());

            (*USART6_PTR).brr.modify(|_, w| {
              w.div_mantissa().bits(usartdiv.0 as u16);
              w.div_fraction().bits(usartdiv.1 as u8)
            });

            if rxint == true {(*USART6_PTR).cr1.modify(|_, w| w.rxneie().enabled());}
            if txint == true {(*USART6_PTR).cr1.modify(|_, w| w.tcie().enabled());}

            (*USART6_PTR).cr1.modify(|_, w| w.re().enabled());
          },
          _ => panic!("{} is not a valid UART peripheral!", channels[i].0)
        };
      }
      else {
        match channels[i].0 {
          1 => {
            (*RCC_PTR).apb2enr.modify(|_, w| w.usart1en().enabled());

            (*USART1_PTR).brr.modify(|_, w| {
              w.div_mantissa().bits(usartdiv.0 as u16);
              w.div_fraction().bits(usartdiv.1 as u8)
            });

            if rxint == true {(*USART1_PTR).cr1.modify(|_, w| w.rxneie().enabled());}
            if txint == true {(*USART1_PTR).cr1.modify(|_, w| w.tcie().enabled());}

            (*USART1_PTR).cr1.modify(|_, w| w.te().enabled());
          },
          3 => {
            (*RCC_PTR).apb1enr.modify(|_, w| w.usart3en().enabled());

            (*USART3_PTR).brr.modify(|_, w| {
              w.div_mantissa().bits(usartdiv.0 as u16);
              w.div_fraction().bits(usartdiv.1 as u8)
            });

            if rxint == true {(*USART3_PTR).cr1.modify(|_, w| w.rxneie().enabled());}
            if txint == true {(*USART3_PTR).cr1.modify(|_, w| w.tcie().enabled());}

            (*USART3_PTR).cr1.modify(|_, w| w.te().enabled());
          },
          4 => {
            (*RCC_PTR).apb1enr.modify(|_, w| w.uart4en().enabled());

            (*UART4_PTR).brr.modify(|_, w| {
              w.div_mantissa().bits(usartdiv.0 as u16);
              w.div_fraction().bits(usartdiv.1 as u8)
            });

            if rxint == true {(*UART4_PTR).cr1.modify(|_, w| w.rxneie().enabled());}
            if txint == true {(*UART4_PTR).cr1.modify(|_, w| w.tcie().enabled());}

            (*UART4_PTR).cr1.modify(|_, w| w.te().enabled());
          },
          6 => {
            (*RCC_PTR).apb2enr.modify(|_, w| w.usart6en().enabled());

            (*USART6_PTR).brr.modify(|_, w| {
              w.div_mantissa().bits(usartdiv.0 as u16);
              w.div_fraction().bits(usartdiv.1 as u8)
            });

            if rxint == true {(*USART6_PTR).cr1.modify(|_, w| w.rxneie().enabled());}
            if txint == true {(*USART6_PTR).cr1.modify(|_, w| w.tcie().enabled());}

            (*USART6_PTR).cr1.modify(|_, w| w.te().enabled());
          },
          _ => panic!("{} is not a valid UART peripheral!", channels[i].0)
        };
      }
    }
  }
}

pub fn send_char(c: char) {
  if c.is_ascii() == true {
    unsafe {
      if UART_USB == false {
        panic!("UART USB channel ist not configured!");
      }

      while (*USART2_PTR).sr.read().txe().bit_is_set() == true {}
      (*USART2_PTR).dr.write(|w| w.dr().bits(c as u16));
      while (*USART2_PTR).sr.read().txe().bit_is_set() == true {}
    }
  }
  else {panic!("{} is not an ASCII character!", c);}
}

pub fn recieve_char() -> char {
  // ----------
  return 'a';
}
