#![allow(dead_code)]

pub use cortex_m_rt::entry;
pub use panic_semihosting as _;

pub extern crate alloc;
pub use core::alloc::Layout;
pub use alloc_cortex_m::CortexMHeap;
pub use alloc::string::String;
pub use heapless::String as HLS;

#[global_allocator]
static ALLOCATOR: CortexMHeap = CortexMHeap::empty();

pub enum Mode {
  Input,
  Output,
  AlterateFunction(u32),
  Analog,
}

pub mod reg {
  use stm32f4::stm32f446;

  pub const RCC_PTR: *const stm32f446::rcc::RegisterBlock = stm32f446::RCC::ptr();
  pub const SYSTICK_PTR: *const stm32f446::stk::RegisterBlock = stm32f446::STK::ptr();
  pub const GPIOA_PTR: *const stm32f446::gpioa::RegisterBlock = stm32f446::GPIOA::ptr();
  pub const GPIOB_PTR: *const stm32f446::gpiob::RegisterBlock = stm32f446::GPIOB::ptr();
  pub const GPIOC_PTR: *const stm32f446::gpioh::RegisterBlock = stm32f446::GPIOC::ptr();
  pub const ADCC_PTR: *const stm32f446::adc_common::RegisterBlock = stm32f446::ADC_COMMON::ptr();
  pub const ADC1_PTR: *const stm32f446::adc1::RegisterBlock = stm32f446::ADC1::ptr();
  pub const ADC2_PTR: *const stm32f446::adc1::RegisterBlock = stm32f446::ADC2::ptr();
  pub const ADC3_PTR: *const stm32f446::adc1::RegisterBlock = stm32f446::ADC3::ptr();
  pub const USART1_PTR: *const stm32f446::usart1::RegisterBlock = stm32f446::USART1::ptr();
  pub const USART2_PTR: *const stm32f446::usart1::RegisterBlock = stm32f446::USART2::ptr();
  pub const USART3_PTR: *const stm32f446::usart1::RegisterBlock = stm32f446::USART3::ptr();
  pub const UART4_PTR: *const stm32f446::uart4::RegisterBlock = stm32f446::UART4::ptr();
  pub const UART5_PTR: *const stm32f446::uart4::RegisterBlock = stm32f446::UART5::ptr();
  pub const USART6_PTR: *const stm32f446::usart1::RegisterBlock = stm32f446::USART6::ptr();
  pub const TIM2_PTR: *const stm32f446::tim2::RegisterBlock = stm32f446::TIM2::ptr();
  pub const TIM3_PTR: *const stm32f446::tim3::RegisterBlock = stm32f446::TIM3::ptr();
  pub const TIM4_PTR: *const stm32f446::tim3::RegisterBlock = stm32f446::TIM4::ptr();
  pub const TIM5_PTR: *const stm32f446::tim5::RegisterBlock = stm32f446::TIM5::ptr();
}

pub mod gpio_d {
  use super::Mode;
  use super::reg::{RCC_PTR, GPIOA_PTR, GPIOB_PTR, GPIOC_PTR};

  pub fn pin_mode(block: &str, pin: u8, mode: Mode) {  
    if pin > 15 {
      panic!("{} is not an available GPIO Pin", pin)
    }
  
    unsafe {
      match block {
        "a" => (*RCC_PTR).ahb1enr.modify(|_, w| w.gpioaen().enabled()),
        "b" => (*RCC_PTR).ahb1enr.modify(|_, w| w.gpioben().enabled()),
        "c" => (*RCC_PTR).ahb1enr.modify(|_, w| w.gpiocen().enabled()),
        _   => panic!("{} is not an available GPIO Block!", block),
      };
  
      match mode {
        Mode::Input => {
          match block {
            "a" => (*GPIOA_PTR).moder.modify(|r, w| w.bits(r.bits() & (!(3 << (pin * 2))))),
            "b" => (*GPIOB_PTR).moder.modify(|r, w| w.bits(r.bits() & (!(3 << (pin * 2))))),
            "c" => (*GPIOC_PTR).moder.modify(|r, w| w.bits(r.bits() & (!(3 << (pin * 2))))),
            _   => return,
          }
        },
        Mode::Output => {
          match block {
            "a" => (*GPIOA_PTR).moder.modify(|r, w| w.bits(r.bits() & (!(3 << (pin * 2))) | (1 << (pin * 2)))),
            "b" => (*GPIOB_PTR).moder.modify(|r, w| w.bits(r.bits() & (!(3 << (pin * 2))) | (1 << (pin * 2)))),
            "c" => (*GPIOC_PTR).moder.modify(|r, w| w.bits(r.bits() & (!(3 << (pin * 2))) | (1 << (pin * 2)))),
            _   => return,
          }
        },
        Mode::AlterateFunction(func) => {
          match block {
            "a" => {
              (*GPIOA_PTR).moder.modify(|r, w| w.bits(r.bits() & (!(3 << (pin * 2))) | (2 << (pin * 2))));
              if pin < 8 {
                (*GPIOA_PTR).afrl.modify(|r, w| w.bits(r.bits() & (!(0xF << (pin * 4))) | (func << (pin * 4))));
              } else {
                (*GPIOA_PTR).afrh.modify(|r, w| w.bits(r.bits() & (!(0xF << (pin * 4))) | (func << (pin * 4))));
              }
            },
            "b" => {
              (*GPIOB_PTR).moder.modify(|r, w| w.bits(r.bits() & (!(3 << (pin * 2))) | (2 << (pin * 2))));
              if pin < 8 {
                (*GPIOB_PTR).afrl.modify(|r, w| w.bits(r.bits() & (!(0xF << (pin * 4))) | (func << (pin * 4))));
              } else {
                (*GPIOB_PTR).afrh.modify(|r, w| w.bits(r.bits() & (!(0xF << (pin * 4))) | (func << (pin * 4))));
              }
            },
            "c" => {
              (*GPIOC_PTR).moder.modify(|r, w| w.bits(r.bits() & (!(3 << (pin * 2))) | (2 << (pin * 2))));
              if pin < 8 {
                (*GPIOC_PTR).afrl.modify(|r, w| w.bits(r.bits() & (!(0xF << (pin * 4))) | (func << (pin * 4))));
              } else {
                (*GPIOC_PTR).afrh.modify(|r, w| w.bits(r.bits() & (!(0xF << (pin * 4))) | (func << (pin * 4))));
              }
            },
            _   => return,
          };
        }
        Mode::Analog => {
          match block {
            "a" => (*GPIOA_PTR).moder.modify(|r, w| w.bits(r.bits() & (!(3 << (pin * 2))) | (3 << (pin * 2)))),
            "b" => (*GPIOB_PTR).moder.modify(|r, w| w.bits(r.bits() & (!(3 << (pin * 2))) | (3 << (pin * 2)))),
            "c" => (*GPIOC_PTR).moder.modify(|r, w| w.bits(r.bits() & (!(3 << (pin * 2))) | (3 << (pin * 2)))),
            _   => return,
          }
        }
      };
    }
  }
  
  pub fn pin_write(block: &str, pin: u8, write: bool) {  
    if pin > 15 {
      panic!("{} is not an available GPIO Pin", pin)
    }
  
    unsafe {
      if write {
        match block {
          "a" => (*GPIOA_PTR).bsrr.write(|w| w.bits(1 << pin)),
          "b" => (*GPIOB_PTR).bsrr.write(|w| w.bits(1 << pin)),
          "c" => (*GPIOC_PTR).bsrr.write(|w| w.bits(1 << pin)),
          _   => panic!("{} is not an available GPIO Block!", block),
        }
      } else {
        match block {
          "a" => (*GPIOA_PTR).bsrr.write(|w| w.bits(1 << (pin + 16))),
          "b" => (*GPIOB_PTR).bsrr.write(|w| w.bits(1 << (pin + 16))),
          "c" => (*GPIOC_PTR).bsrr.write(|w| w.bits(1 << (pin + 16))),
          _   => panic!("{} is not an available GPIO Block!", block),
        }
      }
    }
  }
  
  pub fn pin_read(block: &str, pin: u8) -> bool {
    let state: bool;
  
    if pin > 15 {
      panic!("{} is not an available GPIO Pin", pin)
    }
  
    unsafe {
      let bits = match block {
        "a" => (*GPIOA_PTR).idr.read().bits(),
        "b" => (*GPIOB_PTR).idr.read().bits(),
        "c" => (*GPIOC_PTR).idr.read().bits(),
        _   => panic!("{} is not an available GPIO Block!", block),
      };
    
      if bits & (1 << pin) == (2 << pin) {
        state = true;
      } else {
        state = false;
      }
    
      return state;
    }
  }
}

pub mod gpio_a {
  use super::reg::{RCC_PTR, ADCC_PTR, ADC1_PTR, ADC2_PTR, ADC3_PTR};

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
  use super::reg::SYSTICK_PTR;

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
  use super::alloc::string::String;
  use super::reg::{RCC_PTR, USART1_PTR, USART2_PTR, USART3_PTR, UART4_PTR, UART5_PTR, USART6_PTR};

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
  
  pub fn serial_print(data: String) {
    unsafe {
      for c in data.chars() {
        (*USART2_PTR).dr.write(|w| w.dr().bits(c as u16));
        while (*USART2_PTR).sr.read().tc().bit_is_clear() {}
      }
    }
  }
}

pub mod pwm {
  use super::reg::{RCC_PTR, TIM2_PTR, TIM3_PTR, TIM4_PTR, TIM5_PTR};

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

pub fn init_heap() {
  // Initialize the allocator BEFORE you use it
  unsafe { ALLOCATOR.init(cortex_m_rt::heap_start() as usize, 1024); }
}

#[alloc_error_handler]
fn oom(_: Layout) -> ! {
    loop {}
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
    let mut text_buffer = String::from(alloc::format!("{}", format_args!("{}", $param)));
    text_buffer.push('\r');
    text_buffer.push('\n');
    uart::serial_print(text_buffer);
  };
}
