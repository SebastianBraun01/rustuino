#![allow(dead_code)]

pub use cortex_m_rt::entry;
pub use panic_semihosting as _;
pub use heapless::{Vec, String};

pub enum Mode {
  Input,
  Output,
  AlterateFunction(u32),
  Analog
}

pub mod include;
pub mod gpio_d;

pub mod gpio_a {
  use super::include::{RCC_PTR, ADCC_PTR, ADC1_PTR, ADC2_PTR, ADC3_PTR};

  pub fn adc_init(num: u8, channel: u8) {
    unsafe {
      (*ADCC_PTR).ccr.modify(|_, w| w.adcpre().div2());
  
      match num {
        1 => {
          (*RCC_PTR).apb2enr.modify(|_, w| w.adc1en().enabled());
          if channel < 10 {(*ADC1_PTR).smpr2.modify(|r, w| w.bits(r.bits() | (0x7 << (channel * 3))));}
          else {(*ADC1_PTR).smpr1.modify(|r, w| w.bits(r.bits() | (0x7 << ((channel - 10) * 3))));}
          (*ADC1_PTR).sqr3.modify(|_, w| w.sq1().bits(channel));
          (*ADC1_PTR).cr2.modify(|_, w| {
            w.cont().continuous();
            w.adon().enabled()
          });
          (*ADC1_PTR).cr2.modify(|_, w| w.swstart().start());
        },
        2 => {
          (*RCC_PTR).apb2enr.modify(|_, w| w.adc2en().enabled());
          if channel < 10 {(*ADC2_PTR).smpr2.modify(|r, w| w.bits(r.bits() | (0x7 << (channel * 3))));}
          else {(*ADC2_PTR).smpr1.modify(|r, w| w.bits(r.bits() | (0x7 << ((channel - 10) * 3))));}
          (*ADC2_PTR).sqr3.modify(|_, w| w.sq1().bits(channel));
          (*ADC2_PTR).cr2.modify(|_, w| {
            w.cont().continuous();
            w.adon().enabled()
          });
          (*ADC2_PTR).cr2.modify(|_, w| w.swstart().start());
        },
        3 => {
          (*RCC_PTR).apb2enr.modify(|_, w| w.adc3en().enabled());
          if channel < 10 {(*ADC3_PTR).smpr2.modify(|r, w| w.bits(r.bits() | (0x7 << (channel * 3))));}
          else {(*ADC3_PTR).smpr1.modify(|r, w| w.bits(r.bits() | (0x7 << ((channel - 10) * 3))));}
          (*ADC3_PTR).sqr3.modify(|_, w| w.sq1().bits(channel));
          (*ADC3_PTR).cr2.modify(|_, w| {
            w.cont().continuous();
            w.adon().enabled()
          });
          (*ADC3_PTR).cr2.modify(|_, w| w.swstart().start());
        },
        _ => panic!("{} is not a valid adc!", num),
      }    
    }
  }
  
  pub fn analog_read(num: u8) -> u16 {
    let buffer: u16;
  
    unsafe {
      buffer = match num {
        1 => (*ADC1_PTR).dr.read().data().bits(),
        2 => (*ADC2_PTR).dr.read().data().bits(),
        3 => (*ADC3_PTR).dr.read().data().bits(),
        _ => panic!("{} is not a valid adc!", num),
      }
    }
  
    return buffer;
  }
}

pub mod time {
  use super::include::SYSTICK_PTR;

  pub fn delay(ms: u32) {
    // 2MHz mit 2000 PSC -> 1kHz
    let systick_psc = 2000000 / 1000;
  
    unsafe {
      (*SYSTICK_PTR).ctrl.modify(|_, w| w.enable().clear_bit());
      (*SYSTICK_PTR).load.write(|w| w.reload().bits(systick_psc * ms));
      (*SYSTICK_PTR).val.write(|w| w.current().bits(0));
      (*SYSTICK_PTR).ctrl.modify(|_, w| w.enable().set_bit());
  
      while !(*SYSTICK_PTR).ctrl.read().countflag().bit_is_set() {}
      (*SYSTICK_PTR).ctrl.modify(|_, w| w.countflag().clear_bit());
      (*SYSTICK_PTR).ctrl.modify(|_, w| w.enable().clear_bit());
    }
  }
}

pub mod uart {
  // use super::alloc::string::String;
  use heapless::String;
  use super::include::{RCC_PTR, USART1_PTR, USART2_PTR, USART3_PTR, UART4_PTR, UART5_PTR, USART6_PTR};

  pub fn uart_init(num: u8, baud: u32) {
    // UART1: PA9_TX|PA10_RX,   PB6_TX|PB7_RX
    // UART2: PA2_TX|PA3_RX,    PD5_TX|PD6_RX
    // UART3: PB10_TX|PB11_RX,  PC10_TX|PC11_RX,  PD8_TX|PD9_RX
    // UART4: PA0_TX|PA1_RX,    PC10_TX|PC11_RX
    // UART5: PE8_TX|PE7_RX
    // UART6: PC6_TX|PC7_RX,    PG14_TX|PG9_RX
  
    let psc = match baud {
      9600 => (104, 2),
      115200 => (8, 7),
      _ => (8, 7)
    };
  
    unsafe {
      match num {
        1 => {
          (*RCC_PTR).apb2enr.modify(|_, w| w.usart1en().enabled());
          (*USART1_PTR).cr1.modify(|_, w| {
            w.ue().enabled();
            w.te().enabled();
            w.re().enabled()
          });
          (*USART1_PTR).brr.modify(|_, w| {
            w.div_mantissa().bits(psc.0);
            w.div_fraction().bits(psc.1)
          });
        },
        2 => {
          (*RCC_PTR).apb1enr.modify(|_, w| w.usart2en().enabled());
          (*USART2_PTR).cr1.modify(|_, w| {
            w.ue().enabled();
            w.te().enabled();
            w.re().enabled()
          });
          (*USART2_PTR).brr.modify(|_, w| {
            w.div_mantissa().bits(psc.0);
            w.div_fraction().bits(psc.1)
          });
        },
        3 => {
          (*RCC_PTR).apb1enr.modify(|_, w| w.usart3en().enabled());
          (*USART3_PTR).cr1.modify(|_, w| {
            w.ue().enabled();
            w.te().enabled();
            w.re().enabled()
          });
          (*USART3_PTR).brr.modify(|_, w| {
            w.div_mantissa().bits(psc.0);
            w.div_fraction().bits(psc.1)
          });
        },
        4 => {
          (*RCC_PTR).apb1enr.modify(|_, w| w.uart4en().enabled());
          (*UART4_PTR).cr1.modify(|_, w| {
            w.ue().enabled();
            w.te().enabled();
            w.re().enabled()
          });
          (*UART4_PTR).brr.modify(|_, w| {
            w.div_mantissa().bits(psc.0);
            w.div_fraction().bits(psc.1)
          });
        },
        5 => {
          (*RCC_PTR).apb1enr.modify(|_, w| w.uart5en().enabled());
          (*UART5_PTR).cr1.modify(|_, w| {
            w.ue().enabled();
            w.te().enabled();
            w.re().enabled()
          });
          (*UART5_PTR).brr.modify(|_, w| {
            w.div_mantissa().bits(psc.0);
            w.div_fraction().bits(psc.1)
          });
        },
        6 => {
          (*RCC_PTR).apb2enr.modify(|_, w| w.usart6en().enabled());
          (*USART6_PTR).cr1.modify(|_, w| {
            w.ue().enabled();
            w.te().enabled();
            w.re().enabled()
          });
          (*USART6_PTR).brr.modify(|_, w| {
            w.div_mantissa().bits(psc.0);
            w.div_fraction().bits(psc.1)
          });
        },
        _ => panic!("{} is not a valid UART peripheral!", num)
      };
    }
  }
  
  pub fn serial_print(data: String<20>) {
    unsafe {
      for c in data.chars() {
        (*USART2_PTR).dr.write(|w| w.dr().bits(c as u16));
        while (*USART2_PTR).sr.read().tc().bit_is_clear() {}
      }
    }
  }
}

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

#[macro_export]
macro_rules! sprint {
  ($param:expr) => {
    let text_buffer = String::from(alloc::format!("{}", format_args!("{}", $param)));
    serial_print(text_buffer);
  };
}

#[macro_export]
macro_rules! sprintln {
  ($param:expr) => {
    let mut text_buffer = String::from($param);
    text_buffer.push('\r');
    text_buffer.push('\n');
    uart::serial_print(text_buffer);
  };
}
