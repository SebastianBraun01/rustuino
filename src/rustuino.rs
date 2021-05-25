#![allow(dead_code)]

pub use cortex_m_rt::entry;
pub use panic_semihosting as _;

// pub extern crate alloc;
// pub use core::alloc::Layout;
// pub use alloc_cortex_m::CortexMHeap;
// pub use alloc::string::String;
pub use heapless::{Vec, String};

// #[global_allocator]
// static ALLOCATOR: CortexMHeap = CortexMHeap::empty();

// pub fn init_heap() {
//   // Initialize the allocator BEFORE you use it
//   unsafe { ALLOCATOR.init(cortex_m_rt::heap_start() as usize, 1024); }
// }

// #[alloc_error_handler]
// fn oom(_: Layout) -> ! {
//   loop {}
// }

pub enum Mode {
  Input,
  Output,
  AlterateFunction(u32),
  Analog,
}

pub mod pins {
  pub const PA0: (u8, char) = (0, 'a');
  pub const PA1: (u8, char) = (1, 'a');
  pub const PA2: (u8, char) = (2, 'a');
  pub const PA3: (u8, char) = (3, 'a');
  pub const PA4: (u8, char) = (4, 'a');
  pub const PA5: (u8, char) = (5, 'a');
  pub const PA6: (u8, char) = (6, 'a');
  pub const PA7: (u8, char) = (7, 'a');
  pub const PA8: (u8, char) = (8, 'a');
  pub const PA9: (u8, char) = (9, 'a');
  pub const PA10: (u8, char) = (10, 'a');
  pub const PA11: (u8, char) = (11, 'a');
  pub const PA12: (u8, char) = (12, 'a');
  pub const PA13: (u8, char) = (13, 'a');
  pub const PA14: (u8, char) = (14, 'a');
  pub const PA15: (u8, char) = (15, 'a');

  pub const PB0: (u8, char) = (0, 'b');
  pub const PB1: (u8, char) = (1, 'b');
  pub const PB2: (u8, char) = (2, 'b');
  pub const PB3: (u8, char) = (3, 'b');
  pub const PB4: (u8, char) = (4, 'b');
  pub const PB5: (u8, char) = (5, 'b');
  pub const PB6: (u8, char) = (6, 'b');
  pub const PB7: (u8, char) = (7, 'b');
  pub const PB8: (u8, char) = (8, 'b');
  pub const PB9: (u8, char) = (9, 'b');
  pub const PB10: (u8, char) = (10, 'b');
  pub const PB11: (u8, char) = (11, 'b');
  pub const PB12: (u8, char) = (12, 'b');
  pub const PB13: (u8, char) = (13, 'b');
  pub const PB14: (u8, char) = (14, 'b');
  pub const PB15: (u8, char) = (15, 'b');

  pub const PC0: (u8, char) = (0, 'c');
  pub const PC1: (u8, char) = (1, 'c');
  pub const PC2: (u8, char) = (2, 'c');
  pub const PC3: (u8, char) = (3, 'c');
  pub const PC4: (u8, char) = (4, 'c');
  pub const PC5: (u8, char) = (5, 'c');
  pub const PC6: (u8, char) = (6, 'c');
  pub const PC7: (u8, char) = (7, 'c');
  pub const PC8: (u8, char) = (8, 'c');
  pub const PC9: (u8, char) = (9, 'c');
  pub const PC10: (u8, char) = (10, 'c');
  pub const PC11: (u8, char) = (11, 'c');
  pub const PC12: (u8, char) = (12, 'c');
  pub const PC13: (u8, char) = (13, 'c');
  pub const PC14: (u8, char) = (14, 'c');
  pub const PC15: (u8, char) = (15, 'c');

  pub const PD0: (u8, char) = (0, 'd');
  pub const PD1: (u8, char) = (1, 'd');
  pub const PD2: (u8, char) = (2, 'd');
  pub const PD3: (u8, char) = (3, 'd');
  pub const PD5: (u8, char) = (5, 'd');
  pub const PD6: (u8, char) = (6, 'd');
  pub const PD7: (u8, char) = (7, 'd');
  pub const PD8: (u8, char) = (8, 'd');
  pub const PD9: (u8, char) = (9, 'd');
  pub const PD10: (u8, char) = (10, 'd');
  pub const PD11: (u8, char) = (11, 'd');
  pub const PD12: (u8, char) = (12, 'd');
  pub const PD13: (u8, char) = (13, 'd');
  pub const PD14: (u8, char) = (14, 'd');
  pub const PD15: (u8, char) = (15, 'd');

  pub const PE0: (u8, char) = (0, 'e');
  pub const PE1: (u8, char) = (1, 'e');
  pub const PE2: (u8, char) = (2, 'e');
  pub const PE3: (u8, char) = (3, 'e');
  pub const PE4: (u8, char) = (4, 'e');
  pub const PE5: (u8, char) = (5, 'e');
  pub const PE6: (u8, char) = (6, 'e');
  pub const PE7: (u8, char) = (7, 'e');
  pub const PE8: (u8, char) = (8, 'e');
  pub const PE9: (u8, char) = (9, 'e');
  pub const PE10: (u8, char) = (10, 'e');
  pub const PE11: (u8, char) = (11, 'e');
  pub const PE12: (u8, char) = (12, 'e');
  pub const PE13: (u8, char) = (13, 'e');
  pub const PE14: (u8, char) = (14, 'e');
  pub const PE15: (u8, char) = (15, 'e');

  pub const PF0: (u8, char) = (0, 'f');
  pub const PF1: (u8, char) = (1, 'f');
  pub const PF2: (u8, char) = (2, 'f');
  pub const PF3: (u8, char) = (3, 'f');
  pub const PF4: (u8, char) = (4, 'f');
  pub const PF5: (u8, char) = (5, 'f');
  pub const PF6: (u8, char) = (6, 'f');
  pub const PF7: (u8, char) = (7, 'f');
  pub const PF8: (u8, char) = (8, 'f');
  pub const PF9: (u8, char) = (9, 'f');
  pub const PF10: (u8, char) = (10, 'f');
  pub const PF11: (u8, char) = (11, 'f');
  pub const PF12: (u8, char) = (12, 'f');
  pub const PF13: (u8, char) = (13, 'f');
  pub const PF14: (u8, char) = (14, 'f');
  pub const PF15: (u8, char) = (15, 'f');

  pub const PG0: (u8, char) = (0, 'g');
  pub const PG1: (u8, char) = (1, 'g');
  pub const PG2: (u8, char) = (2, 'g');
  pub const PG3: (u8, char) = (3, 'g');
  pub const PG4: (u8, char) = (4, 'g');
  pub const PG5: (u8, char) = (5, 'g');
  pub const PG6: (u8, char) = (6, 'g');
  pub const PG7: (u8, char) = (7, 'g');
  pub const PG8: (u8, char) = (8, 'g');
  pub const PG9: (u8, char) = (9, 'g');
  pub const PG10: (u8, char) = (10, 'g');
  pub const PG11: (u8, char) = (11, 'g');
  pub const PG12: (u8, char) = (12, 'g');
  pub const PG13: (u8, char) = (13, 'g');
  pub const PG14: (u8, char) = (14, 'g');
  pub const PG15: (u8, char) = (15, 'g');

  pub const PH0: (u8, char) = (0, 'h');
  pub const PH1: (u8, char) = (1, 'h');
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

  pub fn pin_mode(pin: (u8, char), mode: Mode) {  
    if pin.0 > 15 {
      panic!("P{}{} is not an available GPIO Pin", pin.1.to_uppercase(), pin.0);
    }
    
    unsafe {
      match pin.1 {
        'a' => {
          (*RCC_PTR).ahb1enr.modify(|_, w| w.gpioaen().enabled());

          match mode {
            Mode::Input => (*GPIOA_PTR).moder.modify(|r, w| w.bits(r.bits() & (!(3 << (pin.0 * 2))))),

            Mode::Output => (*GPIOA_PTR).moder.modify(|r, w| w.bits(r.bits() & (!(3 << (pin.0 * 2))) | (1 << (pin.0 * 2)))),

            Mode::AlterateFunction(func) => {
              (*GPIOA_PTR).moder.modify(|r, w| w.bits(r.bits() & (!(3 << (pin.0 * 2))) | (2 << (pin.0 * 2))));
              if pin.0 < 8 {
                (*GPIOA_PTR).afrl.modify(|r, w| w.bits(r.bits() & (!(0xF << (pin.0 * 4))) | (func << (pin.0 * 4))));
              } else {
                (*GPIOA_PTR).afrh.modify(|r, w| w.bits(r.bits() & (!(0xF << (pin.0 * 4))) | (func << (pin.0 * 4))));
              }
            },

            Mode::Analog => (*GPIOA_PTR).moder.modify(|r, w| w.bits(r.bits() & (!(3 << (pin.0 * 2))) | (3 << (pin.0 * 2))))
          }
        },
        'b' => {
          (*RCC_PTR).ahb1enr.modify(|_, w| w.gpioben().enabled());

          match mode {
            Mode::Input => (*GPIOB_PTR).moder.modify(|r, w| w.bits(r.bits() & (!(3 << (pin.0 * 2))))),

            Mode::Output => (*GPIOB_PTR).moder.modify(|r, w| w.bits(r.bits() & (!(3 << (pin.0 * 2))) | (1 << (pin.0 * 2)))),

            Mode::AlterateFunction(func) => {
              (*GPIOB_PTR).moder.modify(|r, w| w.bits(r.bits() & (!(3 << (pin.0 * 2))) | (2 << (pin.0 * 2))));
              if pin.0 < 8 {
                (*GPIOB_PTR).afrl.modify(|r, w| w.bits(r.bits() & (!(0xF << (pin.0 * 4))) | (func << (pin.0 * 4))));
              } else {
                (*GPIOB_PTR).afrh.modify(|r, w| w.bits(r.bits() & (!(0xF << (pin.0 * 4))) | (func << (pin.0 * 4))));
              }
            },
            
            Mode::Analog => (*GPIOB_PTR).moder.modify(|r, w| w.bits(r.bits() & (!(3 << (pin.0 * 2))) | (3 << (pin.0 * 2))))
          }
        },
        'c' => {
          (*RCC_PTR).ahb1enr.modify(|_, w| w.gpiocen().enabled());

          match mode {
            Mode::Input => (*GPIOC_PTR).moder.modify(|r, w| w.bits(r.bits() & (!(3 << (pin.0 * 2))))),

            Mode::Output => (*GPIOC_PTR).moder.modify(|r, w| w.bits(r.bits() & (!(3 << (pin.0 * 2))) | (1 << (pin.0 * 2)))),

            Mode::AlterateFunction(func) => {
              (*GPIOC_PTR).moder.modify(|r, w| w.bits(r.bits() & (!(3 << (pin.0 * 2))) | (2 << (pin.0 * 2))));
              if pin.0 < 8 {
                (*GPIOC_PTR).afrl.modify(|r, w| w.bits(r.bits() & (!(0xF << (pin.0 * 4))) | (func << (pin.0 * 4))));
              } else {
                (*GPIOC_PTR).afrh.modify(|r, w| w.bits(r.bits() & (!(0xF << (pin.0 * 4))) | (func << (pin.0 * 4))));
              }
            },
            
            Mode::Analog => (*GPIOC_PTR).moder.modify(|r, w| w.bits(r.bits() & (!(3 << (pin.0 * 2))) | (3 << (pin.0 * 2))))
          }
        },
        _   => panic!("P{}{} is not an available GPIO Pin", pin.1.to_uppercase(), pin.0)
      };
    }
  }
  
  pub fn pin_write(pin: (u8, char), write: bool) {  
    if pin.0 > 15 {
      panic!("P{}{} is not an available GPIO Pin", pin.1.to_uppercase(), pin.0);
    }

    unsafe {
      match pin.1 {
        'a' => {
          if write == true {(*GPIOA_PTR).bsrr.write(|w| w.bits(1 << pin.0));}
          else {(*GPIOA_PTR).bsrr.write(|w| w.bits(1 << (pin.0 + 16)));}
        },
        'b' => {
          if write == true {(*GPIOB_PTR).bsrr.write(|w| w.bits(1 << pin.0));}
          else {(*GPIOB_PTR).bsrr.write(|w| w.bits(1 << (pin.0 + 16)));}
        },
        'c' => {
          if write == true {(*GPIOC_PTR).bsrr.write(|w| w.bits(1 << pin.0));}
          else {(*GPIOC_PTR).bsrr.write(|w| w.bits(1 << (pin.0 + 16)));}
        },
        _   => panic!("P{}{} is not an available GPIO Pin", pin.1.to_uppercase(), pin.0)
      };
    }
  }
  
  pub fn pin_read(pin: (u8, char)) -> bool {  
    if pin.0 > 15 {
      panic!("P{}{} is not an available GPIO Pin", pin.1.to_uppercase(), pin.0);
    }
  
    unsafe {
      if (*GPIOA_PTR).moder.read().bits() & (3 << (2 * pin.0)) == 0 {
        let bits = match pin.1 {
          'a' => (*GPIOA_PTR).idr.read().bits(),
          'b' => (*GPIOB_PTR).idr.read().bits(),
          'c' => (*GPIOC_PTR).idr.read().bits(),
          _   => panic!("P{}{} is not an available GPIO Pin", pin.1.to_uppercase(), pin.0)
        };

        if bits & (1 << pin.0) == (1 << pin.0) {return true;}
        else {return false;}
      }
      else if (*GPIOA_PTR).moder.read().bits() & (3 << (2 * pin.0)) == (1 << (2 * pin.0)) {
        let bits = match pin.1 {
          'a' => (*GPIOA_PTR).odr.read().bits(),
          'b' => (*GPIOB_PTR).odr.read().bits(),
          'c' => (*GPIOC_PTR).odr.read().bits(),
          _   => panic!("P{}{} is not an available GPIO Pin", pin.1.to_uppercase(), pin.0)
        };

        if bits & (1 << pin.0) == (1 << pin.0) {return true;}
        else {return false;}
      }
      else {panic!("Cannot read GPIO Pin in this Configuration!");}
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
  // use super::alloc::string::String;
  use heapless::String;
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
