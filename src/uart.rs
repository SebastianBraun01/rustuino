use libm::*;
use heapless::Vec;
use super::common::*;
use super::include::PERIPHERAL_PTR;


// Converter implementations ======================================================================
impl<const B: char, const P: u8> ToUart for GpioPin<B, P, 8> {
  fn uart(self) -> UartPin<Self> {
    
  }
}


// Helper functions ===============================================================================
pub fn uart_init(baud: u32, rxint: bool, txint: bool) {
  let rcc = &PERIPHERAL_PTR.RCC;
  let usart1 = &PERIPHERAL_PTR.USART1;
  let usart3 = &PERIPHERAL_PTR.USART3;
  let uart4 = &PERIPHERAL_PTR.UART4;
  let usart6 = &PERIPHERAL_PTR.USART6;
  
  // (Mantisse, Fractal)
  let usartdiv: (f64, f64) = modf(16000000.0 / (16.0 * baud as f64));
  let mut channels: Vec<(u8, bool), 5> = Vec::new();
  
  unsafe {
    for i in 0..PINCONFIG.pin.len() {
      if PINCONFIG.alternate[i] == 7 || PINCONFIG.alternate[i] == 8 {
        if UART_MAP.rx_pin.contains(&PINCONFIG.pin[i]) {
          channels.push((UART_MAP.channel[UART_MAP.rx_pin.iter().position(|&r| r == PINCONFIG.pin[i]).unwrap()], false))
          .expect("Could not PINCONFIGure UART channel!");
          UART_MAP.active[UART_MAP.rx_pin.iter().position(|&r| r == PINCONFIG.pin[i]).unwrap()] = true;
        }else if UART_MAP.tx_pin.contains(&PINCONFIG.pin[i]) {
          channels.push((UART_MAP.channel[UART_MAP.tx_pin.iter().position(|&r| r == PINCONFIG.pin[i]).unwrap()], true))
          .expect("Could not PINCONFIGure UART channel!");
          UART_MAP.active[UART_MAP.rx_pin.iter().position(|&r| r == PINCONFIG.pin[i]).unwrap()] = true;
        }
        else {panic!("P{}{} is not available for UART connection!", PINCONFIG.pin[i].1.to_uppercase(), PINCONFIG.pin[i].0);}
      }
    }
  }
  
  for i in 0..channels.len() {
    if channels[i].1 == false {
      match channels[i].0 {
        1 => {
          rcc.apb2enr.modify(|_, w| w.usart1en().enabled());
          
          usart1.brr.modify(|_, w| {
            w.div_mantissa().bits(usartdiv.0 as u16);
            w.div_fraction().bits(usartdiv.1 as u8)
          });
          
          if rxint == true {usart1.cr1.modify(|_, w| w.rxneie().enabled());}
          if txint == true {usart1.cr1.modify(|_, w| w.tcie().enabled());}
          
          usart1.cr1.modify(|_, w| w.re().enabled());
        },
        3 => {
          rcc.apb1enr.modify(|_, w| w.usart3en().enabled());
          
          usart3.brr.modify(|_, w| {
            w.div_mantissa().bits(usartdiv.0 as u16);
            w.div_fraction().bits(usartdiv.1 as u8)
          });
          
          if rxint == true {usart3.cr1.modify(|_, w| w.rxneie().enabled());}
          if txint == true {usart3.cr1.modify(|_, w| w.tcie().enabled());}
          
          usart3.cr1.modify(|_, w| w.re().enabled());
        },
        4 => {
          rcc.apb1enr.modify(|_, w| w.uart4en().enabled());
          
          uart4.brr.modify(|_, w| {
            w.div_mantissa().bits(usartdiv.0 as u16);
            w.div_fraction().bits(usartdiv.1 as u8)
          });
          
          if rxint == true {uart4.cr1.modify(|_, w| w.rxneie().enabled());}
          if txint == true {uart4.cr1.modify(|_, w| w.tcie().enabled());}
          
          uart4.cr1.modify(|_, w| w.re().enabled());
        },
        6 => {
          rcc.apb2enr.modify(|_, w| w.usart6en().enabled());
          
          usart6.brr.modify(|_, w| {
            w.div_mantissa().bits(usartdiv.0 as u16);
            w.div_fraction().bits(usartdiv.1 as u8)
          });
          
          if rxint == true {usart6.cr1.modify(|_, w| w.rxneie().enabled());}
          if txint == true {usart6.cr1.modify(|_, w| w.tcie().enabled());}
          
          usart6.cr1.modify(|_, w| w.re().enabled());
        },
        _ => panic!("{} is not a valid UART peripheral!", channels[i].0)
      };
    }
    else {
      match channels[i].0 {
        1 => {
          rcc.apb2enr.modify(|_, w| w.usart1en().enabled());
          
          usart1.brr.modify(|_, w| {
            w.div_mantissa().bits(usartdiv.0 as u16);
            w.div_fraction().bits(usartdiv.1 as u8)
          });
          
          if rxint == true {usart1.cr1.modify(|_, w| w.rxneie().enabled());}
          if txint == true {usart1.cr1.modify(|_, w| w.tcie().enabled());}
          
          usart1.cr1.modify(|_, w| w.te().enabled());
        },
        3 => {
          rcc.apb1enr.modify(|_, w| w.usart3en().enabled());
          
          usart3.brr.modify(|_, w| {
            w.div_mantissa().bits(usartdiv.0 as u16);
            w.div_fraction().bits(usartdiv.1 as u8)
          });
          
          if rxint == true {usart3.cr1.modify(|_, w| w.rxneie().enabled());}
          if txint == true {usart3.cr1.modify(|_, w| w.tcie().enabled());}
          
          usart3.cr1.modify(|_, w| w.te().enabled());
        },
        4 => {
          rcc.apb1enr.modify(|_, w| w.uart4en().enabled());
          
          uart4.brr.modify(|_, w| {
            w.div_mantissa().bits(usartdiv.0 as u16);
            w.div_fraction().bits(usartdiv.1 as u8)
          });
          
          if rxint == true {uart4.cr1.modify(|_, w| w.rxneie().enabled());}
          if txint == true {uart4.cr1.modify(|_, w| w.tcie().enabled());}
          
          uart4.cr1.modify(|_, w| w.te().enabled());
        },
        6 => {
          rcc.apb2enr.modify(|_, w| w.usart6en().enabled());
          
          usart6.brr.modify(|_, w| {
            w.div_mantissa().bits(usartdiv.0 as u16);
            w.div_fraction().bits(usartdiv.1 as u8)
          });
          
          if rxint == true {usart6.cr1.modify(|_, w| w.rxneie().enabled());}
          if txint == true {usart6.cr1.modify(|_, w| w.tcie().enabled());}
          
          usart6.cr1.modify(|_, w| w.te().enabled());
        },
        _ => panic!("{} is not a valid UART peripheral!", channels[i].0)
      };
    }
  }
}

pub fn send_char(c: char) {
  let usart2 = &PERIPHERAL_PTR.USART2;
  
  if c.is_ascii() == true {
    unsafe {if UART_USB == false {panic!("UART USB channel ist not PINCONFIGured!");}}
    
    while usart2.sr.read().txe().bit_is_set() == true {}
    usart2.dr.write(|w| w.dr().bits(c as u16));
    while usart2.sr.read().txe().bit_is_set() == true {}
  }
  else {panic!("{} is not an ASCII character!", c);}
}

pub fn recieve_char() -> char {
  // ----------
  return 'a';
}


// UART Serial connection =========================================================================
pub mod Serial {
  use libm::*;
  use cortex_m_semihosting::hprintln;
  use super::super::include::PERIPHERAL_PTR;
  use super::super::include::data_maps::UART_USB;

  pub fn init(baud: u32, rxint: bool, txint: bool) {
    let rcc = &PERIPHERAL_PTR.RCC;
    let usart2 = &PERIPHERAL_PTR.USART2;
    let gpioa = &PERIPHERAL_PTR.GPIOA;
    
    // (Mantisse, Fractal)
    let usartdiv: (f64, f64) = modf(16000000.0 / (16.0 * baud as f64));

    unsafe {
      if UART_USB == true {
        hprintln!("Serial connection already configured!");
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
    
    unsafe {UART_USB = true;}
    
    rcc.ahb1enr.modify(|_, w| w.gpioaen().enabled());
    gpioa.moder.modify(|_, w| w.moder2().alternate());
    gpioa.moder.modify(|_, w| w.moder3().alternate());
    gpioa.afrl.modify(|_, w| w.afrl2().af7());
    gpioa.afrl.modify(|_, w| w.afrl3().af7());
  }

  pub fn send_char_usb(c: char) {
    let usart2 = &PERIPHERAL_PTR.USART2;
    
    unsafe {if UART_USB == false {panic!("UART USB channel ist not PINCONFIGured!");}}

    if c.is_ascii() == true {
      while usart2.sr.read().txe().bit_is_set() == true {}
      usart2.dr.write(|w| w.dr().bits(c as u16));
      while usart2.sr.read().txe().bit_is_set() == true {}
    }
    else {
      c = '?';
      hprintln!("Cant send non-ascii characters!");

      while usart2.sr.read().txe().bit_is_set() == true {}
      usart2.dr.write(|w| w.dr().bits(c as u16));
      while usart2.sr.read().txe().bit_is_set() == true {}
    }
  }
  
  pub fn recieve_char_usb() -> char {
    let usart2 = &PERIPHERAL_PTR.USART2;
    let buffer: u8;
    
    unsafe {if UART_USB == false {panic!("UART USB channel ist not PINCONFIGured!");}}
    
    while usart2.sr.read().rxne().bit_is_clear() == true {}
    buffer = usart2.dr.read().dr().bits() as u8;
    
    return buffer as char;
  }


  // Macro declerations ===========================================================================
  #[macro_export]
  macro_rules! sprint {
    ($param:expr) => {
      use core::fmt;

      let mut txt_buff: String<50> = String::new();
      if fmt::write(&mut txt_buff, format_args!($param)).is_err() {txt_buff = String::from("~\r\n")};
    
      for c in txt_buff.chars() {
        if c.is_ascii() == true {send_char_usb(c);}
        else {send_char_usb('?');}
      }
    };
  }

  #[macro_export]
  macro_rules! sprintln {
    ($param:expr) => {
      use core::fmt;

      let mut txt_buff: String<50> = String::new();
      if fmt::write(&mut txt_buff, format_args!(" ")).is_err() {txt_buff = String::from("~\r\n")};
    
      for c in txt_buff.chars() {
        if c.is_ascii() == true {send_char_usb(c);}
        else {send_char_usb('?');}
      }

      send_char_usb('\r');
      send_char_usb('\n');
    };
  }

  #[macro_export]
  macro_rules! sread {
    () => {{
      let c_buff: char = recieve_char_usb();  
      c_buff
    }};

    ($c:expr) => {{
      let found: bool;

      if recieve_char_usb() == $c {found = true;}
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
        buff = recieve_char_usb();
        if buff == $stop as char {break;}
        str.push(buff).expect("String buffer full!");
      }
      str
    }};
  }
}
