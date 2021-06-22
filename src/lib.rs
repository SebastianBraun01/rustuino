#![no_std]

// Library includes ===============================================================================
pub use cortex_m_rt::{entry, exception};
pub use stm32f4::stm32f446::{NVIC, Interrupt, interrupt};
pub use panic_semihosting as _;

// Für benutzer
pub use libm::*;
pub use heapless::{Vec, String, FnvIndexMap};
pub use {include::*,  gpio::*, analog::*, time::*, uart::*, common::*};


// Submodule includes =============================================================================
pub mod include;
pub mod common;
pub mod gpio;
pub mod analog;
pub mod time;
pub mod uart;

pub mod pwm {
  use super::include::PERIPHERAL_PTR;

  pub fn pwm_init(num: u8, channel: u8) {
    let rcc = &PERIPHERAL_PTR.RCC;
    let tim2 = &PERIPHERAL_PTR.TIM2;
    let tim3 = &PERIPHERAL_PTR.TIM3;
    let tim4 = &PERIPHERAL_PTR.TIM4;
    let tim5 = &PERIPHERAL_PTR.TIM5;

      match num {
        2 => {
          rcc.apb1enr.modify(|_, w| w.tim2en().enabled());
          tim2.cr1.modify(|_, w| w.arpe().enabled());
          tim2.egr.write(|w| w.ug().set_bit());
          tim2.psc.write(|w| w.psc().bits(1000));
          tim2.arr.write_with_zero(|w| w.arr().bits(255));
          match channel {
            1 => {
              tim2.ccmr1_output_mut().modify(|_, w| {
                w.oc1m().pwm_mode1();
                w.oc1pe().enabled()
              });
              tim2.ccer.modify(|_, w| w.cc1e().set_bit());
            },
            2 => {
              tim2.ccmr1_output_mut().modify(|_, w| {
                w.oc2m().pwm_mode1();
                w.oc2pe().enabled()
              });
              tim2.ccer.modify(|_, w| w.cc2e().set_bit());
            },
            3 => {
              tim2.ccmr2_output_mut().modify(|_, w| {
                w.oc3m().pwm_mode1();
                w.oc3pe().enabled()
              });
              tim2.ccer.modify(|_, w| w.cc3e().set_bit());
            },
            4 => {
              tim2.ccmr2_output_mut().modify(|_, w| {
                w.oc4m().pwm_mode1();
                w.oc4pe().enabled()
              });
              tim2.ccer.modify(|_, w| w.cc4e().set_bit());
            },
            _ => panic!("{} is not a valid channel!", channel),
          }
        },
        3 => {
          rcc.apb1enr.modify(|_, w| w.tim3en().enabled());
          tim3.cr1.modify(|_, w| w.arpe().enabled());
          tim3.egr.write(|w| w.ug().set_bit());
          tim3.psc.write(|w| w.psc().bits(1000));
          tim3.arr.write_with_zero(|w| w.arr().bits(255));
          match channel {
            1 => {
              tim3.ccmr1_output_mut().modify(|_, w| {
                w.oc1m().pwm_mode1();
                w.oc1pe().enabled()
              });
              tim3.ccer.modify(|_, w| w.cc1e().set_bit());
            },
            2 => {
              tim3.ccmr1_output_mut().modify(|_, w| {
                w.oc2m().pwm_mode1();
                w.oc2pe().enabled()
              });
              tim3.ccer.modify(|_, w| w.cc2e().set_bit());
            },
            3 => {
              tim3.ccmr2_output_mut().modify(|_, w| {
                w.oc3m().pwm_mode1();
                w.oc3pe().enabled()
              });
              tim3.ccer.modify(|_, w| w.cc3e().set_bit());
            },
            4 => {
              tim3.ccmr2_output_mut().modify(|_, w| {
                w.oc4m().pwm_mode1();
                w.oc4pe().enabled()
              });
              tim3.ccer.modify(|_, w| w.cc4e().set_bit());
            },
            _ => panic!("{} is not a valid channel!", channel),
          }
        },
        4 => {
          rcc.apb1enr.modify(|_, w| w.tim4en().enabled());
          tim4.cr1.modify(|_, w| w.arpe().enabled());
          tim4.egr.write(|w| w.ug().set_bit());
          tim4.psc.write(|w| w.psc().bits(1000));
          tim4.arr.write_with_zero(|w| w.arr().bits(255));
          match channel {
            1 => {
              tim4.ccmr1_output_mut().modify(|_, w| {
                w.oc1m().pwm_mode1();
                w.oc1pe().enabled()
              });
              tim4.ccer.modify(|_, w| w.cc1e().set_bit());
            },
            2 => {
              tim4.ccmr1_output_mut().modify(|_, w| {
                w.oc2m().pwm_mode1();
                w.oc2pe().enabled()
              });
              tim4.ccer.modify(|_, w| w.cc2e().set_bit());
            },
            3 => {
              tim4.ccmr2_output_mut().modify(|_, w| {
                w.oc3m().pwm_mode1();
                w.oc3pe().enabled()
              });
              tim4.ccer.modify(|_, w| w.cc3e().set_bit());
            },
            4 => {
              tim4.ccmr2_output_mut().modify(|_, w| {
                w.oc4m().pwm_mode1();
                w.oc4pe().enabled()
              });
              tim4.ccer.modify(|_, w| w.cc4e().set_bit());
            },
            _ => panic!("{} is not a valid channel!", channel),
          }
        },
        5 => {
          rcc.apb1enr.modify(|_, w| w.tim5en().enabled());
          tim5.cr1.modify(|_, w| w.arpe().enabled());
          tim5.egr.write(|w| w.ug().set_bit());
          tim5.psc.write(|w| w.psc().bits(1000));
          tim5.arr.write_with_zero(|w| w.arr().bits(255));
          match channel {
            1 => {
              tim5.ccmr1_output_mut().modify(|_, w| {
                w.oc1m().pwm_mode1();
                w.oc1pe().enabled()
              });
              tim5.ccer.modify(|_, w| w.cc1e().set_bit());
            },
            2 => {
              tim5.ccmr1_output_mut().modify(|_, w| {
                w.oc2m().pwm_mode1();
                w.oc2pe().enabled()
              });
              tim5.ccer.modify(|_, w| w.cc2e().set_bit());
            },
            3 => {
              tim5.ccmr2_output_mut().modify(|_, w| {
                w.oc3m().pwm_mode1();
                w.oc3pe().enabled()
              });
              tim5.ccer.modify(|_, w| w.cc3e().set_bit());
            },
            4 => {
              tim5.ccmr2_output_mut().modify(|_, w| {
                w.oc4m().pwm_mode1();
                w.oc4pe().enabled()
              });
              tim5.ccer.modify(|_, w| w.cc4e().set_bit());
            },
            _ => panic!("{} is not a valid channel!", channel),
          }
        },
        _ => panic!("{} is not a valid timer!", num),
      }
  }
  
  pub fn pin_write_pwm(num: u8, channel: u8, write: u8) {
    let tim2 = &PERIPHERAL_PTR.TIM2;
    let tim3 = &PERIPHERAL_PTR.TIM3;
    let tim4 = &PERIPHERAL_PTR.TIM4;
    let tim5 = &PERIPHERAL_PTR.TIM5;

      match num {
        2 => {
          match channel {
            1 => {
              tim2.cr1.modify(|_, w| w.cen().disabled());
              tim2.ccr1.write_with_zero(|w| w.ccr().bits(write as u32));
              tim2.cr1.modify(|_, w| w.cen().enabled());
            },
            2 => {
              tim2.cr1.modify(|_, w| w.cen().disabled());
              tim2.ccr2.write_with_zero(|w| w.ccr().bits(write as u32));
              tim2.cr1.modify(|_, w| w.cen().enabled());
            },
            3 => {
              tim2.cr1.modify(|_, w| w.cen().disabled());
              tim2.ccr3.write_with_zero(|w| w.ccr().bits(write as u32));
              tim2.cr1.modify(|_, w| w.cen().enabled());
            },
            4 => {
              tim2.cr1.modify(|_, w| w.cen().disabled());
              tim2.ccr4.write_with_zero(|w| w.ccr().bits(write as u32));
              tim2.cr1.modify(|_, w| w.cen().enabled());
            },
            _ => panic!("{} is not a valid channel!", channel),
          }
        },
        3 => {
          match channel {
            1 => {
              tim3.cr1.modify(|_, w| w.cen().disabled());
              tim3.ccr1.write_with_zero(|w| w.ccr().bits(write as u16));
              tim3.cr1.modify(|_, w| w.cen().enabled());
            },
            2 => {
              tim3.cr1.modify(|_, w| w.cen().disabled());
              tim3.ccr2.write_with_zero(|w| w.ccr().bits(write as u16));
              tim3.cr1.modify(|_, w| w.cen().enabled());
            },
            3 => {
              tim3.cr1.modify(|_, w| w.cen().disabled());
              tim3.ccr3.write_with_zero(|w| w.ccr().bits(write as u16));
              tim3.cr1.modify(|_, w| w.cen().enabled());
            },
            4 => {
              tim3.cr1.modify(|_, w| w.cen().disabled());
              tim3.ccr4.write_with_zero(|w| w.ccr().bits(write as u16));
              tim3.cr1.modify(|_, w| w.cen().enabled());
            },
            _ => panic!("{} is not a valid channel!", channel),
          }
        },
        4 => {
          match channel {
            1 => {
              tim4.cr1.modify(|_, w| w.cen().disabled());
              tim4.ccr1.write_with_zero(|w| w.ccr().bits(write as u16));
              tim4.cr1.modify(|_, w| w.cen().enabled());
            },
            2 => {
              tim4.cr1.modify(|_, w| w.cen().disabled());
              tim4.ccr2.write_with_zero(|w| w.ccr().bits(write as u16));
              tim4.cr1.modify(|_, w| w.cen().enabled());
            },
            3 => {
              tim4.cr1.modify(|_, w| w.cen().disabled());
              tim4.ccr3.write_with_zero(|w| w.ccr().bits(write as u16));
              tim4.cr1.modify(|_, w| w.cen().enabled());
            },
            4 => {
              tim4.cr1.modify(|_, w| w.cen().disabled());
              tim4.ccr4.write_with_zero(|w| w.ccr().bits(write as u16));
              tim4.cr1.modify(|_, w| w.cen().enabled());
            },
            _ => panic!("{} is not a valid channel!", channel),
          }
        },
        5 => {
          match channel {
            1 => {
              tim5.cr1.modify(|_, w| w.cen().disabled());
              tim5.ccr1.write_with_zero(|w| w.ccr().bits(write as u32));
              tim5.cr1.modify(|_, w| w.cen().enabled());
            },
            2 => {
              tim5.cr1.modify(|_, w| w.cen().disabled());
              tim5.ccr2.write_with_zero(|w| w.ccr().bits(write as u32));
              tim5.cr1.modify(|_, w| w.cen().enabled());
            },
            3 => {
              tim5.cr1.modify(|_, w| w.cen().disabled());
              tim5.ccr3.write_with_zero(|w| w.ccr().bits(write as u32));
              tim5.cr1.modify(|_, w| w.cen().enabled());
            },
            4 => {
              tim5.cr1.modify(|_, w| w.cen().disabled());
              tim5.ccr4.write_with_zero(|w| w.ccr().bits(write as u32));
              tim5.cr1.modify(|_, w| w.cen().enabled());
            },
            _ => panic!("{} is not a valid channel!", channel),
          }
        },
        _ => panic!("{} is not a valid timer!", num),
      }
  }
}


// Macro declerations =============================================================================
#[macro_export]
macro_rules! sprint {
  ($param:expr) => {
    use core::fmt;

    let mut txt_buff: String<50> = String::new();
    fmt::write(&mut txt_buff, format_args!($param));

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

    let txt_buff: String<50> = String::new();
    fmt::write(&mut txt_buff, format_args!($param));

    for char in text_buffer.chars() {
      if char.is_ascii() == true {send_char_usb(char);}
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
}

#[macro_export]
macro_rules! sreads {
  ($stop:expr) => {{
    let mut string: String<50> = String::new();
    let mut c: char;
    loop {
      c = recieve_char_usb();
      if c == $stop as char {break;}
      if c.is_ascii() == true {string.push(c).expect("String buffer full!");}
      else {string.push('?').expect("String buffer full!");}
    }
    string
  }};
}
