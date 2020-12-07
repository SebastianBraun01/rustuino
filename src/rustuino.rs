#![allow(dead_code)]

pub use cortex_m_rt::entry;
pub use panic_semihosting as _;

pub extern crate alloc;
pub use core::alloc::Layout;
pub use alloc_cortex_m::CortexMHeap;
pub use alloc::string::String;

#[global_allocator]
static ALLOCATOR: CortexMHeap = CortexMHeap::empty();

pub enum Mode {
  Input,
  Output,
  AlterateFunction(u32),
  Analog,
}

pub fn pin_mode(block: &str, pin: u8, mode: Mode) {
  let rcc_ptr = stm32f4::stm32f446::RCC::ptr();
  let gpioa_ptr = stm32f4::stm32f446::GPIOA::ptr();
  let gpiob_ptr = stm32f4::stm32f446::GPIOB::ptr();
  let gpioc_ptr = stm32f4::stm32f446::GPIOC::ptr();

  if pin > 15 {
    panic!("{} is not an available GPIO Pin", pin)
  }

  unsafe {
    match block {
      "a" => (*rcc_ptr).ahb1enr.modify(|_, w| w.gpioaen().enabled()),
      "b" => (*rcc_ptr).ahb1enr.modify(|_, w| w.gpioben().enabled()),
      "c" => (*rcc_ptr).ahb1enr.modify(|_, w| w.gpiocen().enabled()),
      _   => panic!("{} is not an available GPIO Block!", block),
    };

    match mode {
      Mode::Input => {
        match block {
          "a" => (*gpioa_ptr).moder.modify(|r, w| w.bits(r.bits() & (!(3 << (pin * 2))))),
          "b" => (*gpiob_ptr).moder.modify(|r, w| w.bits(r.bits() & (!(3 << (pin * 2))))),
          "c" => (*gpioc_ptr).moder.modify(|r, w| w.bits(r.bits() & (!(3 << (pin * 2))))),
          _   => return,
        }
      },
      Mode::Output => {
        match block {
          "a" => (*gpioa_ptr).moder.modify(|r, w| w.bits(r.bits() & (!(3 << (pin * 2))) | (1 << (pin * 2)))),
          "b" => (*gpiob_ptr).moder.modify(|r, w| w.bits(r.bits() & (!(3 << (pin * 2))) | (1 << (pin * 2)))),
          "c" => (*gpioc_ptr).moder.modify(|r, w| w.bits(r.bits() & (!(3 << (pin * 2))) | (1 << (pin * 2)))),
          _   => return,
        }
      },
      Mode::AlterateFunction(func) => {
        match block {
          "a" => {
            (*gpioa_ptr).moder.modify(|r, w| w.bits(r.bits() & (!(3 << (pin * 2))) | (2 << (pin * 2))));
            if pin < 8 {
              (*gpioa_ptr).afrl.modify(|r, w| w.bits(r.bits() & (!(0xF << (pin * 4))) | (func << (pin * 4))));
            } else {
              (*gpioa_ptr).afrh.modify(|r, w| w.bits(r.bits() & (!(0xF << (pin * 4))) | (func << (pin * 4))));
            }
          },
          "b" => {
            (*gpiob_ptr).moder.modify(|r, w| w.bits(r.bits() & (!(3 << (pin * 2))) | (2 << (pin * 2))));
            if pin < 8 {
              (*gpiob_ptr).afrl.modify(|r, w| w.bits(r.bits() & (!(0xF << (pin * 4))) | (func << (pin * 4))));
            } else {
              (*gpiob_ptr).afrh.modify(|r, w| w.bits(r.bits() & (!(0xF << (pin * 4))) | (func << (pin * 4))));
            }
          },
          "c" => {
            (*gpioc_ptr).moder.modify(|r, w| w.bits(r.bits() & (!(3 << (pin * 2))) | (2 << (pin * 2))));
            if pin < 8 {
              (*gpioc_ptr).afrl.modify(|r, w| w.bits(r.bits() & (!(0xF << (pin * 4))) | (func << (pin * 4))));
            } else {
              (*gpioc_ptr).afrh.modify(|r, w| w.bits(r.bits() & (!(0xF << (pin * 4))) | (func << (pin * 4))));
            }
          },
          _   => return,
        };
      }
      Mode::Analog => {
        match block {
          "a" => (*gpioa_ptr).moder.modify(|r, w| w.bits(r.bits() & (!(3 << (pin * 2))) | (3 << (pin * 2)))),
          "b" => (*gpiob_ptr).moder.modify(|r, w| w.bits(r.bits() & (!(3 << (pin * 2))) | (3 << (pin * 2)))),
          "c" => (*gpioc_ptr).moder.modify(|r, w| w.bits(r.bits() & (!(3 << (pin * 2))) | (3 << (pin * 2)))),
          _   => return,
        }
      }
    };
  }
}

pub fn pin_write(block: &str, pin: u8, write: bool) {
  let gpioa_ptr = stm32f4::stm32f446::GPIOA::ptr();
  let gpiob_ptr = stm32f4::stm32f446::GPIOB::ptr();
  let gpioc_ptr = stm32f4::stm32f446::GPIOC::ptr();

  if pin > 15 {
    panic!("{} is not an available GPIO Pin", pin)
  }

  unsafe {
    if write == true {
      match block {
        "a" => (*gpioa_ptr).bsrr.write(|w| w.bits(1 << pin)),
        "b" => (*gpiob_ptr).bsrr.write(|w| w.bits(1 << pin)),
        "c" => (*gpioc_ptr).bsrr.write(|w| w.bits(1 << pin)),
        _   => panic!("{} is not an available GPIO Block!", block),
      }
    } else {
      match block {
        "a" => (*gpioa_ptr).bsrr.write(|w| w.bits(1 << pin + 16)),
        "b" => (*gpiob_ptr).bsrr.write(|w| w.bits(1 << pin + 16)),
        "c" => (*gpioc_ptr).bsrr.write(|w| w.bits(1 << pin + 16)),
        _   => panic!("{} is not an available GPIO Block!", block),
      }
    }
  }
}

pub fn pin_read(block: &str, pin: u8) -> bool {
  let gpioa_ptr = stm32f4::stm32f446::GPIOA::ptr();
  let gpiob_ptr = stm32f4::stm32f446::GPIOB::ptr();
  let gpioc_ptr = stm32f4::stm32f446::GPIOC::ptr();
  let state: bool;

  if pin > 15 {
    panic!("{} is not an available GPIO Pin", pin)
  }

  unsafe {
    let bits = match block {
      "a" => (*gpioa_ptr).idr.read().bits(),
      "b" => (*gpiob_ptr).idr.read().bits(),
      "c" => (*gpioc_ptr).idr.read().bits(),
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

pub fn delay(ms: u32) {
  // 2MHz mit 2000 PSC -> 1kHz
  let systick_psc = 2000000 / 1000;
  let systick_ptr = stm32f4::stm32f446::STK::ptr();

  unsafe {
    (*systick_ptr).ctrl.modify(|_, w| w.enable().clear_bit());
    (*systick_ptr).load.write(|w| w.reload().bits(systick_psc * ms));
    (*systick_ptr).val.write(|w| w.current().bits(0));
    (*systick_ptr).ctrl.modify(|_, w| w.enable().set_bit());

    while !(*systick_ptr).ctrl.read().countflag().bit_is_set() {}
    (*systick_ptr).ctrl.modify(|_, w| w.countflag().clear_bit());
    (*systick_ptr).ctrl.modify(|_, w| w.enable().clear_bit());
  }
}

pub fn uart_init(num: u8, baud: u32) {
  // UART1: PA9_TX|PA10_RX,   PB6_TX|PB7_RX
  // UART2: PA2_TX|PA3_RX,    PD5_TX|PD6_RX
  // UART3: PB10_TX|PB11_RX,  PC10_TX|PC11_RX,  PD8_TX|PD9_RX
  // UART4: PA0_TX|PA1_RX,    PC10_TX|PC11_RX
  // UART5: PE8_TX|PE7_RX
  // UART6: PC6_TX|PC7_RX,    PG14_TX|PG9_RX

  let rcc_ptr = stm32f4::stm32f446::RCC::ptr();
  let uart1_ptr = stm32f4::stm32f446::USART1::ptr();
  let uart2_ptr = stm32f4::stm32f446::USART2::ptr();
  let uart3_ptr = stm32f4::stm32f446::USART3::ptr();
  let uart4_ptr = stm32f4::stm32f446::UART4::ptr();
  let uart5_ptr = stm32f4::stm32f446::UART5::ptr();
  let uart6_ptr = stm32f4::stm32f446::USART6::ptr();

  let psc = match baud {
    9600 => (104, 2),
    115200 => (8, 7),
    _ => (8, 7)
  };

  unsafe {
    match num {
      1 => {
        (*rcc_ptr).apb2enr.modify(|_, w| w.usart1en().enabled());
        (*uart1_ptr).cr1.modify(|_, w| {
          w.ue().enabled();
          w.te().enabled();
          w.re().enabled()
        });
        (*uart1_ptr).brr.modify(|_, w| {
          w.div_mantissa().bits(psc.0);
          w.div_fraction().bits(psc.1)
        });
      },
      2 => {
        (*rcc_ptr).apb1enr.modify(|_, w| w.usart2en().enabled());
        (*uart2_ptr).cr1.modify(|_, w| {
          w.ue().enabled();
          w.te().enabled();
          w.re().enabled()
        });
        (*uart2_ptr).brr.modify(|_, w| {
          w.div_mantissa().bits(psc.0);
          w.div_fraction().bits(psc.1)
        });
      },
      3 => {
        (*rcc_ptr).apb1enr.modify(|_, w| w.usart3en().enabled());
        (*uart3_ptr).cr1.modify(|_, w| {
          w.ue().enabled();
          w.te().enabled();
          w.re().enabled()
        });
        (*uart3_ptr).brr.modify(|_, w| {
          w.div_mantissa().bits(psc.0);
          w.div_fraction().bits(psc.1)
        });
      },
      4 => {
        (*rcc_ptr).apb1enr.modify(|_, w| w.uart4en().enabled());
        (*uart4_ptr).cr1.modify(|_, w| {
          w.ue().enabled();
          w.te().enabled();
          w.re().enabled()
        });
        (*uart4_ptr).brr.modify(|_, w| {
          w.div_mantissa().bits(psc.0);
          w.div_fraction().bits(psc.1)
        });
      },
      5 => {
        (*rcc_ptr).apb1enr.modify(|_, w| w.uart5en().enabled());
        (*uart5_ptr).cr1.modify(|_, w| {
          w.ue().enabled();
          w.te().enabled();
          w.re().enabled()
        });
        (*uart5_ptr).brr.modify(|_, w| {
          w.div_mantissa().bits(psc.0);
          w.div_fraction().bits(psc.1)
        });
      },
      6 => {
        (*rcc_ptr).apb2enr.modify(|_, w| w.usart6en().enabled());
        (*uart6_ptr).cr1.modify(|_, w| {
          w.ue().enabled();
          w.te().enabled();
          w.re().enabled()
        });
        (*uart6_ptr).brr.modify(|_, w| {
          w.div_mantissa().bits(psc.0);
          w.div_fraction().bits(psc.1)
        });
      },
      _ => panic!("{} is not a valid UART peripheral!", num)
    };
  }
}

pub fn serial_print(data: String){
  let usart2_ptr = stm32f4::stm32f446::USART2::ptr();

  unsafe {
    for c in data.chars() {
      (*usart2_ptr).dr.write(|w| w.dr().bits(c as u16));
      while (*usart2_ptr).sr.read().tc().bit_is_clear() {}
    }
  }
}

pub fn adc_init(num: u8, channel: u8) {
  let rcc_ptr = stm32f4::stm32f446::RCC::ptr();
  let adc1_ptr = stm32f4::stm32f446::ADC1::ptr();
  let adc2_ptr = stm32f4::stm32f446::ADC2::ptr();
  let adc3_ptr = stm32f4::stm32f446::ADC3::ptr();
  let adcc_ptr = stm32f4::stm32f446::ADC_COMMON::ptr();

  unsafe {
    (*adcc_ptr).ccr.modify(|_, w| w.adcpre().div2());

    match num {
      1 => {
        (*rcc_ptr).apb2enr.modify(|_, w| w.adc1en().enabled());
        if channel < 10 {(*adc1_ptr).smpr2.modify(|r, w| w.bits(r.bits() | (0x7 << (channel * 3))));}
        else {(*adc1_ptr).smpr1.modify(|r, w| w.bits(r.bits() | (0x7 << ((channel - 10) * 3))));}
        (*adc1_ptr).sqr3.modify(|_, w| w.sq1().bits(channel));
        (*adc1_ptr).cr2.modify(|_, w| {
          w.cont().continuous();
          w.adon().enabled()
        });
        (*adc1_ptr).cr2.modify(|_, w| w.swstart().start());
      },
      2 => {
        (*rcc_ptr).apb2enr.modify(|_, w| w.adc2en().enabled());
        if channel < 10 {(*adc2_ptr).smpr2.modify(|r, w| w.bits(r.bits() | (0x7 << (channel * 3))));}
        else {(*adc2_ptr).smpr1.modify(|r, w| w.bits(r.bits() | (0x7 << ((channel - 10) * 3))));}
        (*adc2_ptr).sqr3.modify(|_, w| w.sq1().bits(channel));
        (*adc2_ptr).cr2.modify(|_, w| {
          w.cont().continuous();
          w.adon().enabled()
        });
        (*adc2_ptr).cr2.modify(|_, w| w.swstart().start());
      },
      3 => {
        (*rcc_ptr).apb2enr.modify(|_, w| w.adc3en().enabled());
        if channel < 10 {(*adc3_ptr).smpr2.modify(|r, w| w.bits(r.bits() | (0x7 << (channel * 3))));}
        else {(*adc3_ptr).smpr1.modify(|r, w| w.bits(r.bits() | (0x7 << ((channel - 10) * 3))));}
        (*adc3_ptr).sqr3.modify(|_, w| w.sq1().bits(channel));
        (*adc3_ptr).cr2.modify(|_, w| {
          w.cont().continuous();
          w.adon().enabled()
        });
        (*adc3_ptr).cr2.modify(|_, w| w.swstart().start());
      },
      _ => panic!("{} is not a valid adc!", num),
    }    
  }
}

pub fn analog_read(num: u8) -> u16 {
  let adc1_ptr = stm32f4::stm32f446::ADC1::ptr();
  let adc2_ptr = stm32f4::stm32f446::ADC2::ptr();
  let adc3_ptr = stm32f4::stm32f446::ADC3::ptr();
  let buffer: u16;

  unsafe {
    buffer = match num {
      1 => (*adc1_ptr).dr.read().data().bits(),
      2 => (*adc2_ptr).dr.read().data().bits(),
      3 => (*adc3_ptr).dr.read().data().bits(),
      _ => panic!("{} is not a valid adc!", num),
    }
  }

  return buffer;
}

pub fn init_heap() {
  // Initialize the allocator BEFORE you use it
  unsafe { ALLOCATOR.init(cortex_m_rt::heap_start() as usize, 1024); }
}

pub fn pwm_init(num: u8, channel: u8) {
  let rcc_ptr = stm32f4::stm32f446::RCC::ptr();
  let tim2_ptr = stm32f4::stm32f446::TIM2::ptr();
  let tim3_ptr = stm32f4::stm32f446::TIM3::ptr();
  let tim4_ptr = stm32f4::stm32f446::TIM4::ptr();
  let tim5_ptr = stm32f4::stm32f446::TIM5::ptr();

  unsafe {
    match num {
      2 => {
        (*rcc_ptr).apb1enr.modify(|_, w| w.tim2en().enabled());
        (*tim2_ptr).cr1.modify(|_, w| w.arpe().enabled());
        (*tim2_ptr).egr.write(|w| w.ug().set_bit());
        (*tim2_ptr).psc.write(|w| w.psc().bits(1000));
        (*tim2_ptr).arr.write_with_zero(|w| w.arr().bits(255));
        match channel {
          1 => {
            (*tim2_ptr).ccmr1_output_mut().modify(|_, w| {
              w.oc1m().pwm_mode1();
              w.oc1pe().enabled()
            });
            (*tim2_ptr).ccer.modify(|_, w| w.cc1e().set_bit());
          },
          2 => {
            (*tim2_ptr).ccmr1_output_mut().modify(|_, w| {
              w.oc2m().pwm_mode1();
              w.oc2pe().enabled()
            });
            (*tim2_ptr).ccer.modify(|_, w| w.cc2e().set_bit());
          },
          3 => {
            (*tim2_ptr).ccmr2_output_mut().modify(|_, w| {
              w.oc3m().pwm_mode1();
              w.oc3pe().enabled()
            });
            (*tim2_ptr).ccer.modify(|_, w| w.cc3e().set_bit());
          },
          4 => {
            (*tim2_ptr).ccmr2_output_mut().modify(|_, w| {
              w.oc4m().pwm_mode1();
              w.oc4pe().enabled()
            });
            (*tim2_ptr).ccer.modify(|_, w| w.cc4e().set_bit());
          },
          _ => panic!("{} is not a valid channel!", channel),
        }
      },
      3 => {
        (*rcc_ptr).apb1enr.modify(|_, w| w.tim3en().enabled());
        (*tim3_ptr).cr1.modify(|_, w| w.arpe().enabled());
        (*tim3_ptr).egr.write(|w| w.ug().set_bit());
        (*tim3_ptr).psc.write(|w| w.psc().bits(1000));
        (*tim3_ptr).arr.write_with_zero(|w| w.arr().bits(255));
        match channel {
          1 => {
            (*tim3_ptr).ccmr1_output_mut().modify(|_, w| {
              w.oc1m().pwm_mode1();
              w.oc1pe().enabled()
            });
            (*tim3_ptr).ccer.modify(|_, w| w.cc1e().set_bit());
          },
          2 => {
            (*tim3_ptr).ccmr1_output_mut().modify(|_, w| {
              w.oc2m().pwm_mode1();
              w.oc2pe().enabled()
            });
            (*tim3_ptr).ccer.modify(|_, w| w.cc2e().set_bit());
          },
          3 => {
            (*tim3_ptr).ccmr2_output_mut().modify(|_, w| {
              w.oc3m().pwm_mode1();
              w.oc3pe().enabled()
            });
            (*tim3_ptr).ccer.modify(|_, w| w.cc3e().set_bit());
          },
          4 => {
            (*tim3_ptr).ccmr2_output_mut().modify(|_, w| {
              w.oc4m().pwm_mode1();
              w.oc4pe().enabled()
            });
            (*tim3_ptr).ccer.modify(|_, w| w.cc4e().set_bit());
          },
          _ => panic!("{} is not a valid channel!", channel),
        }
      },
      4 => {
        (*rcc_ptr).apb1enr.modify(|_, w| w.tim4en().enabled());
        (*tim4_ptr).cr1.modify(|_, w| w.arpe().enabled());
        (*tim4_ptr).egr.write(|w| w.ug().set_bit());
        (*tim4_ptr).psc.write(|w| w.psc().bits(1000));
        (*tim4_ptr).arr.write_with_zero(|w| w.arr().bits(255));
        match channel {
          1 => {
            (*tim4_ptr).ccmr1_output_mut().modify(|_, w| {
              w.oc1m().pwm_mode1();
              w.oc1pe().enabled()
            });
            (*tim4_ptr).ccer.modify(|_, w| w.cc1e().set_bit());
          },
          2 => {
            (*tim4_ptr).ccmr1_output_mut().modify(|_, w| {
              w.oc2m().pwm_mode1();
              w.oc2pe().enabled()
            });
            (*tim4_ptr).ccer.modify(|_, w| w.cc2e().set_bit());
          },
          3 => {
            (*tim4_ptr).ccmr2_output_mut().modify(|_, w| {
              w.oc3m().pwm_mode1();
              w.oc3pe().enabled()
            });
            (*tim4_ptr).ccer.modify(|_, w| w.cc3e().set_bit());
          },
          4 => {
            (*tim4_ptr).ccmr2_output_mut().modify(|_, w| {
              w.oc4m().pwm_mode1();
              w.oc4pe().enabled()
            });
            (*tim4_ptr).ccer.modify(|_, w| w.cc4e().set_bit());
          },
          _ => panic!("{} is not a valid channel!", channel),
        }
      },
      5 => {
        (*rcc_ptr).apb1enr.modify(|_, w| w.tim5en().enabled());
        (*tim5_ptr).cr1.modify(|_, w| w.arpe().enabled());
        (*tim5_ptr).egr.write(|w| w.ug().set_bit());
        (*tim5_ptr).psc.write(|w| w.psc().bits(1000));
        (*tim5_ptr).arr.write_with_zero(|w| w.arr().bits(255));
        match channel {
          1 => {
            (*tim5_ptr).ccmr1_output_mut().modify(|_, w| {
              w.oc1m().pwm_mode1();
              w.oc1pe().enabled()
            });
            (*tim5_ptr).ccer.modify(|_, w| w.cc1e().set_bit());
          },
          2 => {
            (*tim5_ptr).ccmr1_output_mut().modify(|_, w| {
              w.oc2m().pwm_mode1();
              w.oc2pe().enabled()
            });
            (*tim5_ptr).ccer.modify(|_, w| w.cc2e().set_bit());
          },
          3 => {
            (*tim5_ptr).ccmr2_output_mut().modify(|_, w| {
              w.oc3m().pwm_mode1();
              w.oc3pe().enabled()
            });
            (*tim5_ptr).ccer.modify(|_, w| w.cc3e().set_bit());
          },
          4 => {
            (*tim5_ptr).ccmr2_output_mut().modify(|_, w| {
              w.oc4m().pwm_mode1();
              w.oc4pe().enabled()
            });
            (*tim5_ptr).ccer.modify(|_, w| w.cc4e().set_bit());
          },
          _ => panic!("{} is not a valid channel!", channel),
        }
      },
      _ => panic!("{} is not a valid timer!", num),
    }
  }
}

pub fn pin_write_pwm(num: u8, channel: u8, write: u8) {
  let tim2_ptr = stm32f4::stm32f446::TIM2::ptr();
  let tim3_ptr = stm32f4::stm32f446::TIM3::ptr();
  let tim4_ptr = stm32f4::stm32f446::TIM4::ptr();
  let tim5_ptr = stm32f4::stm32f446::TIM5::ptr();

  unsafe {
    match num {
      2 => {
        match channel {
          1 => {
            (*tim2_ptr).cr1.modify(|_, w| w.cen().disabled());
            (*tim2_ptr).ccr1.write_with_zero(|w| w.ccr().bits(write as u32));
            (*tim2_ptr).cr1.modify(|_, w| w.cen().enabled());
          },
          2 => {
            (*tim2_ptr).cr1.modify(|_, w| w.cen().disabled());
            (*tim2_ptr).ccr2.write_with_zero(|w| w.ccr().bits(write as u32));
            (*tim2_ptr).cr1.modify(|_, w| w.cen().enabled());
          },
          3 => {
            (*tim2_ptr).cr1.modify(|_, w| w.cen().disabled());
            (*tim2_ptr).ccr3.write_with_zero(|w| w.ccr().bits(write as u32));
            (*tim2_ptr).cr1.modify(|_, w| w.cen().enabled());
          },
          4 => {
            (*tim2_ptr).cr1.modify(|_, w| w.cen().disabled());
            (*tim2_ptr).ccr4.write_with_zero(|w| w.ccr().bits(write as u32));
            (*tim2_ptr).cr1.modify(|_, w| w.cen().enabled());
          },
          _ => panic!("{} is not a valid channel!", channel),
        }
      },
      3 => {
        match channel {
          1 => {
            (*tim3_ptr).cr1.modify(|_, w| w.cen().disabled());
            (*tim3_ptr).ccr1.write_with_zero(|w| w.ccr().bits(write as u16));
            (*tim3_ptr).cr1.modify(|_, w| w.cen().enabled());
          },
          2 => {
            (*tim3_ptr).cr1.modify(|_, w| w.cen().disabled());
            (*tim3_ptr).ccr2.write_with_zero(|w| w.ccr().bits(write as u16));
            (*tim3_ptr).cr1.modify(|_, w| w.cen().enabled());
          },
          3 => {
            (*tim3_ptr).cr1.modify(|_, w| w.cen().disabled());
            (*tim3_ptr).ccr3.write_with_zero(|w| w.ccr().bits(write as u16));
            (*tim3_ptr).cr1.modify(|_, w| w.cen().enabled());
          },
          4 => {
            (*tim3_ptr).cr1.modify(|_, w| w.cen().disabled());
            (*tim3_ptr).ccr4.write_with_zero(|w| w.ccr().bits(write as u16));
            (*tim3_ptr).cr1.modify(|_, w| w.cen().enabled());
          },
          _ => panic!("{} is not a valid channel!", channel),
        }
      },
      4 => {
        match channel {
          1 => {
            (*tim4_ptr).cr1.modify(|_, w| w.cen().disabled());
            (*tim4_ptr).ccr1.write_with_zero(|w| w.ccr().bits(write as u16));
            (*tim4_ptr).cr1.modify(|_, w| w.cen().enabled());
          },
          2 => {
            (*tim4_ptr).cr1.modify(|_, w| w.cen().disabled());
            (*tim4_ptr).ccr2.write_with_zero(|w| w.ccr().bits(write as u16));
            (*tim4_ptr).cr1.modify(|_, w| w.cen().enabled());
          },
          3 => {
            (*tim4_ptr).cr1.modify(|_, w| w.cen().disabled());
            (*tim4_ptr).ccr3.write_with_zero(|w| w.ccr().bits(write as u16));
            (*tim4_ptr).cr1.modify(|_, w| w.cen().enabled());
          },
          4 => {
            (*tim4_ptr).cr1.modify(|_, w| w.cen().disabled());
            (*tim4_ptr).ccr4.write_with_zero(|w| w.ccr().bits(write as u16));
            (*tim4_ptr).cr1.modify(|_, w| w.cen().enabled());
          },
          _ => panic!("{} is not a valid channel!", channel),
        }
      },
      5 => {
        match channel {
          1 => {
            (*tim5_ptr).cr1.modify(|_, w| w.cen().disabled());
            (*tim5_ptr).ccr1.write_with_zero(|w| w.ccr().bits(write as u32));
            (*tim5_ptr).cr1.modify(|_, w| w.cen().enabled());
          },
          2 => {
            (*tim5_ptr).cr1.modify(|_, w| w.cen().disabled());
            (*tim5_ptr).ccr2.write_with_zero(|w| w.ccr().bits(write as u32));
            (*tim5_ptr).cr1.modify(|_, w| w.cen().enabled());
          },
          3 => {
            (*tim5_ptr).cr1.modify(|_, w| w.cen().disabled());
            (*tim5_ptr).ccr3.write_with_zero(|w| w.ccr().bits(write as u32));
            (*tim5_ptr).cr1.modify(|_, w| w.cen().enabled());
          },
          4 => {
            (*tim5_ptr).cr1.modify(|_, w| w.cen().disabled());
            (*tim5_ptr).ccr4.write_with_zero(|w| w.ccr().bits(write as u32));
            (*tim5_ptr).cr1.modify(|_, w| w.cen().enabled());
          },
          _ => panic!("{} is not a valid channel!", channel),
        }
      },
      _ => panic!("{} is not a valid timer!", num),
    }
  }
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
    serial_print(text_buffer);
  };
}
