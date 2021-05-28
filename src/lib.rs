#![no_std]
#![allow(dead_code)]

// Library includes ===============================================================================
pub use cortex_m_rt::entry;
pub use panic_semihosting as _;
pub use cortex_m::peripheral::NVIC;
pub use stm32f4::stm32f446::{Interrupt, interrupt};
pub use libm::*;
pub use heapless::{Vec, String, FnvIndexMap, FnvIndexSet};
pub use {include::*, gpio_d::*, uart::*, pwm::*, time::*};


// Struct and Enum declerations ===================================================================
pub enum Mode {
  Input,
  Output,
  AlterateFunction(u32),
  // false: ADC, true: DAC
  Analog(bool)
}

pub enum Speed {
  Low,
  Medium,
  Fast,
  High
}

pub enum Bias {
  None,
  Pullup,
  Pulldown
}

pub struct Config {
  pin: Vec<(u8, char), 25>,
  // config: {0: input, 1: output, 2: alternate, 3: analog}
  config: Vec<u8, 25>,
  // alternate: {<16: func_number, 16: none}
  alternate: Vec<u32, 25>,
  // analog: {0: none, 1: adc, 2: dac}
  analog: Vec<u8, 25>
}

pub struct ADCMap {
  pin: [(u8, char); 16],
  channel: [u8; 16],
  active: [bool; 16]
}

pub struct UARTMap {
  tx_pin: [(u8, char); 12],
  rx_pin: [(u8, char); 12],
  channel: [u8; 12],
  active: [bool; 12]
}

pub struct TIMERMap {
  pin: [(u8, char); 74],
  timer: [u8; 74],
  ccch: [u8; 74],
  active: [bool; 74]
}


// Submodule includes =============================================================================
pub mod include;
pub mod gpio_d;
pub mod gpio_a;
pub mod time;
pub mod uart;

pub mod pwm {
  use super::include::{RCC_PTR, TIM2_PTR, TIM3_PTR, TIM4_PTR, TIM5_PTR};

  pub fn pwm_init(num: u8, channel: u8) {  
    unsafe {
      match num {
        2 => {
          (*RCC_PTR).apb1enr.modify(|_, w| w.tim2en().enabled());
          (*TIM2_PTR).cr1.modify(|_, w| w.arpe().enabled());
          (*TIM2_PTR).egr.write(|w| w.ug().set_bit());
          (*TIM2_PTR).psc.write(|w| w.psc().bits(1000));
          (*TIM2_PTR).arr.write_with_zero(|w| w.arr().bits(255));
          match channel {
            1 => {
              (*TIM2_PTR).ccmr1_output_mut().modify(|_, w| {
                w.oc1m().pwm_mode1();
                w.oc1pe().enabled()
              });
              (*TIM2_PTR).ccer.modify(|_, w| w.cc1e().set_bit());
            },
            2 => {
              (*TIM2_PTR).ccmr1_output_mut().modify(|_, w| {
                w.oc2m().pwm_mode1();
                w.oc2pe().enabled()
              });
              (*TIM2_PTR).ccer.modify(|_, w| w.cc2e().set_bit());
            },
            3 => {
              (*TIM2_PTR).ccmr2_output_mut().modify(|_, w| {
                w.oc3m().pwm_mode1();
                w.oc3pe().enabled()
              });
              (*TIM2_PTR).ccer.modify(|_, w| w.cc3e().set_bit());
            },
            4 => {
              (*TIM2_PTR).ccmr2_output_mut().modify(|_, w| {
                w.oc4m().pwm_mode1();
                w.oc4pe().enabled()
              });
              (*TIM2_PTR).ccer.modify(|_, w| w.cc4e().set_bit());
            },
            _ => panic!("{} is not a valid channel!", channel),
          }
        },
        3 => {
          (*RCC_PTR).apb1enr.modify(|_, w| w.tim3en().enabled());
          (*TIM3_PTR).cr1.modify(|_, w| w.arpe().enabled());
          (*TIM3_PTR).egr.write(|w| w.ug().set_bit());
          (*TIM3_PTR).psc.write(|w| w.psc().bits(1000));
          (*TIM3_PTR).arr.write_with_zero(|w| w.arr().bits(255));
          match channel {
            1 => {
              (*TIM3_PTR).ccmr1_output_mut().modify(|_, w| {
                w.oc1m().pwm_mode1();
                w.oc1pe().enabled()
              });
              (*TIM3_PTR).ccer.modify(|_, w| w.cc1e().set_bit());
            },
            2 => {
              (*TIM3_PTR).ccmr1_output_mut().modify(|_, w| {
                w.oc2m().pwm_mode1();
                w.oc2pe().enabled()
              });
              (*TIM3_PTR).ccer.modify(|_, w| w.cc2e().set_bit());
            },
            3 => {
              (*TIM3_PTR).ccmr2_output_mut().modify(|_, w| {
                w.oc3m().pwm_mode1();
                w.oc3pe().enabled()
              });
              (*TIM3_PTR).ccer.modify(|_, w| w.cc3e().set_bit());
            },
            4 => {
              (*TIM3_PTR).ccmr2_output_mut().modify(|_, w| {
                w.oc4m().pwm_mode1();
                w.oc4pe().enabled()
              });
              (*TIM3_PTR).ccer.modify(|_, w| w.cc4e().set_bit());
            },
            _ => panic!("{} is not a valid channel!", channel),
          }
        },
        4 => {
          (*RCC_PTR).apb1enr.modify(|_, w| w.tim4en().enabled());
          (*TIM4_PTR).cr1.modify(|_, w| w.arpe().enabled());
          (*TIM4_PTR).egr.write(|w| w.ug().set_bit());
          (*TIM4_PTR).psc.write(|w| w.psc().bits(1000));
          (*TIM4_PTR).arr.write_with_zero(|w| w.arr().bits(255));
          match channel {
            1 => {
              (*TIM4_PTR).ccmr1_output_mut().modify(|_, w| {
                w.oc1m().pwm_mode1();
                w.oc1pe().enabled()
              });
              (*TIM4_PTR).ccer.modify(|_, w| w.cc1e().set_bit());
            },
            2 => {
              (*TIM4_PTR).ccmr1_output_mut().modify(|_, w| {
                w.oc2m().pwm_mode1();
                w.oc2pe().enabled()
              });
              (*TIM4_PTR).ccer.modify(|_, w| w.cc2e().set_bit());
            },
            3 => {
              (*TIM4_PTR).ccmr2_output_mut().modify(|_, w| {
                w.oc3m().pwm_mode1();
                w.oc3pe().enabled()
              });
              (*TIM4_PTR).ccer.modify(|_, w| w.cc3e().set_bit());
            },
            4 => {
              (*TIM4_PTR).ccmr2_output_mut().modify(|_, w| {
                w.oc4m().pwm_mode1();
                w.oc4pe().enabled()
              });
              (*TIM4_PTR).ccer.modify(|_, w| w.cc4e().set_bit());
            },
            _ => panic!("{} is not a valid channel!", channel),
          }
        },
        5 => {
          (*RCC_PTR).apb1enr.modify(|_, w| w.tim5en().enabled());
          (*TIM5_PTR).cr1.modify(|_, w| w.arpe().enabled());
          (*TIM5_PTR).egr.write(|w| w.ug().set_bit());
          (*TIM5_PTR).psc.write(|w| w.psc().bits(1000));
          (*TIM5_PTR).arr.write_with_zero(|w| w.arr().bits(255));
          match channel {
            1 => {
              (*TIM5_PTR).ccmr1_output_mut().modify(|_, w| {
                w.oc1m().pwm_mode1();
                w.oc1pe().enabled()
              });
              (*TIM5_PTR).ccer.modify(|_, w| w.cc1e().set_bit());
            },
            2 => {
              (*TIM5_PTR).ccmr1_output_mut().modify(|_, w| {
                w.oc2m().pwm_mode1();
                w.oc2pe().enabled()
              });
              (*TIM5_PTR).ccer.modify(|_, w| w.cc2e().set_bit());
            },
            3 => {
              (*TIM5_PTR).ccmr2_output_mut().modify(|_, w| {
                w.oc3m().pwm_mode1();
                w.oc3pe().enabled()
              });
              (*TIM5_PTR).ccer.modify(|_, w| w.cc3e().set_bit());
            },
            4 => {
              (*TIM5_PTR).ccmr2_output_mut().modify(|_, w| {
                w.oc4m().pwm_mode1();
                w.oc4pe().enabled()
              });
              (*TIM5_PTR).ccer.modify(|_, w| w.cc4e().set_bit());
            },
            _ => panic!("{} is not a valid channel!", channel),
          }
        },
        _ => panic!("{} is not a valid timer!", num),
      }
    }
  }
  
  pub fn pin_write_pwm(num: u8, channel: u8, write: u8) {
    unsafe {
      match num {
        2 => {
          match channel {
            1 => {
              (*TIM2_PTR).cr1.modify(|_, w| w.cen().disabled());
              (*TIM2_PTR).ccr1.write_with_zero(|w| w.ccr().bits(write as u32));
              (*TIM2_PTR).cr1.modify(|_, w| w.cen().enabled());
            },
            2 => {
              (*TIM2_PTR).cr1.modify(|_, w| w.cen().disabled());
              (*TIM2_PTR).ccr2.write_with_zero(|w| w.ccr().bits(write as u32));
              (*TIM2_PTR).cr1.modify(|_, w| w.cen().enabled());
            },
            3 => {
              (*TIM2_PTR).cr1.modify(|_, w| w.cen().disabled());
              (*TIM2_PTR).ccr3.write_with_zero(|w| w.ccr().bits(write as u32));
              (*TIM2_PTR).cr1.modify(|_, w| w.cen().enabled());
            },
            4 => {
              (*TIM2_PTR).cr1.modify(|_, w| w.cen().disabled());
              (*TIM2_PTR).ccr4.write_with_zero(|w| w.ccr().bits(write as u32));
              (*TIM2_PTR).cr1.modify(|_, w| w.cen().enabled());
            },
            _ => panic!("{} is not a valid channel!", channel),
          }
        },
        3 => {
          match channel {
            1 => {
              (*TIM3_PTR).cr1.modify(|_, w| w.cen().disabled());
              (*TIM3_PTR).ccr1.write_with_zero(|w| w.ccr().bits(write as u16));
              (*TIM3_PTR).cr1.modify(|_, w| w.cen().enabled());
            },
            2 => {
              (*TIM3_PTR).cr1.modify(|_, w| w.cen().disabled());
              (*TIM3_PTR).ccr2.write_with_zero(|w| w.ccr().bits(write as u16));
              (*TIM3_PTR).cr1.modify(|_, w| w.cen().enabled());
            },
            3 => {
              (*TIM3_PTR).cr1.modify(|_, w| w.cen().disabled());
              (*TIM3_PTR).ccr3.write_with_zero(|w| w.ccr().bits(write as u16));
              (*TIM3_PTR).cr1.modify(|_, w| w.cen().enabled());
            },
            4 => {
              (*TIM3_PTR).cr1.modify(|_, w| w.cen().disabled());
              (*TIM3_PTR).ccr4.write_with_zero(|w| w.ccr().bits(write as u16));
              (*TIM3_PTR).cr1.modify(|_, w| w.cen().enabled());
            },
            _ => panic!("{} is not a valid channel!", channel),
          }
        },
        4 => {
          match channel {
            1 => {
              (*TIM4_PTR).cr1.modify(|_, w| w.cen().disabled());
              (*TIM4_PTR).ccr1.write_with_zero(|w| w.ccr().bits(write as u16));
              (*TIM4_PTR).cr1.modify(|_, w| w.cen().enabled());
            },
            2 => {
              (*TIM4_PTR).cr1.modify(|_, w| w.cen().disabled());
              (*TIM4_PTR).ccr2.write_with_zero(|w| w.ccr().bits(write as u16));
              (*TIM4_PTR).cr1.modify(|_, w| w.cen().enabled());
            },
            3 => {
              (*TIM4_PTR).cr1.modify(|_, w| w.cen().disabled());
              (*TIM4_PTR).ccr3.write_with_zero(|w| w.ccr().bits(write as u16));
              (*TIM4_PTR).cr1.modify(|_, w| w.cen().enabled());
            },
            4 => {
              (*TIM4_PTR).cr1.modify(|_, w| w.cen().disabled());
              (*TIM4_PTR).ccr4.write_with_zero(|w| w.ccr().bits(write as u16));
              (*TIM4_PTR).cr1.modify(|_, w| w.cen().enabled());
            },
            _ => panic!("{} is not a valid channel!", channel),
          }
        },
        5 => {
          match channel {
            1 => {
              (*TIM5_PTR).cr1.modify(|_, w| w.cen().disabled());
              (*TIM5_PTR).ccr1.write_with_zero(|w| w.ccr().bits(write as u32));
              (*TIM5_PTR).cr1.modify(|_, w| w.cen().enabled());
            },
            2 => {
              (*TIM5_PTR).cr1.modify(|_, w| w.cen().disabled());
              (*TIM5_PTR).ccr2.write_with_zero(|w| w.ccr().bits(write as u32));
              (*TIM5_PTR).cr1.modify(|_, w| w.cen().enabled());
            },
            3 => {
              (*TIM5_PTR).cr1.modify(|_, w| w.cen().disabled());
              (*TIM5_PTR).ccr3.write_with_zero(|w| w.ccr().bits(write as u32));
              (*TIM5_PTR).cr1.modify(|_, w| w.cen().enabled());
            },
            4 => {
              (*TIM5_PTR).cr1.modify(|_, w| w.cen().disabled());
              (*TIM5_PTR).ccr4.write_with_zero(|w| w.ccr().bits(write as u32));
              (*TIM5_PTR).cr1.modify(|_, w| w.cen().enabled());
            },
            _ => panic!("{} is not a valid channel!", channel),
          }
        },
        _ => panic!("{} is not a valid timer!", num),
      }
    }
  }
}


// Macro declerations =============================================================================
#[macro_export]
macro_rules! sprint {
  ($param:expr) => {
    let text_buffer: String<50> = String::from($param);
    for char in text_buffer.chars() {
      if char.is_ascii() == true {
        send_char_usb(char);
      }
      else {panic!("{} is not an ASCII character!", char)}
    }
  };
}

#[macro_export]
macro_rules! sprintln {
  ($param:expr) => {
    let text_buffer: String<50> = String::from($param);
    for char in text_buffer.chars() {
      if char.is_ascii() == true {
        send_char_usb(char);
      }
      else {panic!("{} is not an ASCII character!", char)}
    }
    send_char_usb('\r');
    send_char_usb('\n');
  };
}

#[macro_export]
macro_rules! sread {
  () => {{
    let text_buffer: char = recieve_char_usb();  
    text_buffer
  }};
}

#[macro_export]
macro_rules! sreads {
  ($stop:expr) => {{
    let mut string: String<50> = String::new();
    let mut buffer: char;
    loop {
      buffer = recieve_char_usb();
      if buffer == $stop as char {break;}
      string.push(buffer).expect("String buffer full!");
    }
    string
  }};
}
