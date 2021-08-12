#![allow(non_snake_case)]

use super::common::*;
use super::include:: {TIMER_MAP, TIMER_CONF, TIME_COUNTER, DELAY_COUNTER};
use cortex_m_semihosting::hprintln;
use cortex_m::peripheral::NVIC;
use cortex_m_rt::exception;
use stm32f4::stm32f446::{Interrupt, interrupt};
use heapless::String;


// Converter implementations ======================================================================
macro_rules! generate_ToPwm {
  ($([$letter:literal, $number:literal]),+) => {
    use paste::paste;
    
    paste!{
      $(
        impl ToPwm for [<P $letter:upper $number>] {
          fn pwm() -> Result<PwmPin, String<20>>{
            let block = $letter;
            let pin = $number;
            let timer: usize;
            let channel: usize;
            
            if TIMER_MAP.pin.contains(&(block, pin)) {
              timer = TIMER_MAP.timer[TIMER_MAP.pin.iter().position(|&i| i == (block, pin)).unwrap()] as usize;
              channel = TIMER_MAP.ccch[TIMER_MAP.pin.iter().position(|&i| i == (block, pin)).unwrap()] as usize;
              
              unsafe {
                if TIMER_CONF[(timer * 4) - channel] == false {TIMER_CONF[(timer * 4) - channel] = true;}
                else {
                  let mut str_buffer: String<20> = String::new();
                  core::fmt::write(&mut str_buffer, format_args!("Timer {} channel {} already in use!", timer, channel)).expect("Could not construct error message!");
                  return Err(str_buffer);
                }
              }
            }
            else {
              let mut str_buffer: String<20> = String::new();
              core::fmt::write(&mut str_buffer, format_args!("P{}{} is not available for pwm output!", block.to_uppercase(), pin)).expect("Could not construct error message!");
              return Err(str_buffer);
            }
            
            pwm_init(timer, channel, block, pin);
            
            return Ok(PwmPin {
              block,
              pin
            });
          }
        }
      )+
    }
  };
}

generate_ToPwm![
['a', 0],
['a', 1],
['a', 2],
['a', 3],
['a', 5],
['a', 6],
['a', 7],
['a', 8],
['a', 9],
['a', 10],
['a', 11],
['a', 15],

['b', 0],
['b', 1],
['b', 2],
['b', 3],
['b', 4],
['b', 5],
['b', 6],
['b', 7],
['b', 8],
['b', 9],
['b', 10],
['b', 11],
['b', 14],
['b', 15],

['c', 6],
['c', 7],
['c', 8],
['c', 9],

['d', 12],
['d', 13],
['d', 14],
['d', 15],

['e', 5],
['e', 6],
['e', 9],
['e', 11],
['e', 13],
['e', 14],

['f', 6],
['f', 7],
['f', 8],
['f', 9]
];


// Function implementations =======================================================================
impl PWM for PwmPin {
  fn pwm_write(&self, value: u8) {
    let block = self.block;
    let pin = self.pin;
    let timer: usize;
    let channel: usize;
    
    if TIMER_MAP.pin.contains(&(block, pin)) {
      timer = TIMER_MAP.timer[TIMER_MAP.pin.iter().position(|&i| i == (block, pin)).unwrap()] as usize;
      channel = TIMER_MAP.ccch[TIMER_MAP.pin.iter().position(|&i| i == (block, pin)).unwrap()] as usize;
      
      unsafe {
        if TIMER_CONF[(timer * 4) - channel] == false {
          hprintln!("Timer {} channel {} not configured!", timer, channel).expect("Could not send semihosting message!");
          return;
        }
      }
    }
    else {panic!("P{}{} is not available for pwm output!", block.to_uppercase(), pin);}
    
    pwm_set_duty(timer, channel, value);
  }
}


// Helper functions ===============================================================================
fn pwm_init(timer: usize, channel: usize, block: char, pin: u8) {
  let peripheral_ptr = stm32f4::stm32f446::Peripherals::take().unwrap();
  let rcc = &peripheral_ptr.RCC;
  
  match block {
    'a' => {
      let gpioa = &peripheral_ptr.GPIOA;
      rcc.ahb1enr.modify(|_, w| w.gpioaen().enabled());
      gpioa.moder.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin)) | (2 << (2 * pin)))});
      if timer == 1 || timer == 2 {
        if pin > 7 {gpioa.afrh.modify(|r, w| unsafe {w.bits(r.bits() | (1 << (4 * (pin - 8))))});}
        else {gpioa.afrl.modify(|r, w| unsafe {w.bits(r.bits() | (1 << (4 * pin)))});}
      }
      else if timer == 3 || timer == 4 || timer == 5 {
        if timer == 1 || timer == 2 {
          if pin > 7 {gpioa.afrh.modify(|r, w| unsafe {w.bits(r.bits() | (2 << (4 * (pin - 8))))});}
          else {gpioa.afrl.modify(|r, w| unsafe {w.bits(r.bits() | (2 << (4 * pin)))});}
        }
      }
      else if timer == 8 || timer == 9 || timer == 10 || timer == 11 {
        if timer == 1 || timer == 2 {
          if pin > 7 {gpioa.afrh.modify(|r, w| unsafe {w.bits(r.bits() | (3 << (4 * (pin - 8))))});}
          else {gpioa.afrl.modify(|r, w| unsafe {w.bits(r.bits() | (3 << (4 * pin)))});}
        }
      }
      else {
        if timer == 1 || timer == 2 {
          if pin > 7 {gpioa.afrh.modify(|r, w| unsafe {w.bits(r.bits() | (9 << (4 * (pin - 8))))});}
          else {gpioa.afrl.modify(|r, w| unsafe {w.bits(r.bits() | (9 << (4 * pin)))});}
        }
      }
    },
    'b' => {
      let gpiob = &peripheral_ptr.GPIOB;
      rcc.ahb1enr.modify(|_, w| w.gpioben().enabled());
      gpiob.moder.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin)) | (2 << (2 * pin)))});
      if timer == 1 || timer == 2 {
        if pin > 7 {gpiob.afrh.modify(|r, w| unsafe {w.bits(r.bits() | (1 << (4 * (pin - 8))))});}
        else {gpiob.afrl.modify(|r, w| unsafe {w.bits(r.bits() | (1 << (4 * pin)))});}
      }
      else if timer == 3 || timer == 4 || timer == 5 {
        if timer == 1 || timer == 2 {
          if pin > 7 {gpiob.afrh.modify(|r, w| unsafe {w.bits(r.bits() | (2 << (4 * (pin - 8))))});}
          else {gpiob.afrl.modify(|r, w| unsafe {w.bits(r.bits() | (2 << (4 * pin)))});}
        }
      }
      else if timer == 8 || timer == 9 || timer == 10 || timer == 11 {
        if timer == 1 || timer == 2 {
          if pin > 7 {gpiob.afrh.modify(|r, w| unsafe {w.bits(r.bits() | (3 << (4 * (pin - 8))))});}
          else {gpiob.afrl.modify(|r, w| unsafe {w.bits(r.bits() | (3 << (4 * pin)))});}
        }
      }
      else {
        if timer == 1 || timer == 2 {
          if pin > 7 {gpiob.afrh.modify(|r, w| unsafe {w.bits(r.bits() | (9 << (4 * (pin - 8))))});}
          else {gpiob.afrl.modify(|r, w| unsafe {w.bits(r.bits() | (9 << (4 * pin)))});}
        }
      }
    },
    'c' => {
      let gpioc = &peripheral_ptr.GPIOC;
      rcc.ahb1enr.modify(|_, w| w.gpiocen().enabled());
      gpioc.moder.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin)) | (2 << (2 * pin)))});
      if timer == 1 || timer == 2 {
        if pin > 7 {gpioc.afrh.modify(|r, w| unsafe {w.bits(r.bits() | (1 << (4 * (pin - 8))))});}
        else {gpioc.afrl.modify(|r, w| unsafe {w.bits(r.bits() | (1 << (4 * pin)))});}
      }
      else if timer == 3 || timer == 4 || timer == 5 {
        if timer == 1 || timer == 2 {
          if pin > 7 {gpioc.afrh.modify(|r, w| unsafe {w.bits(r.bits() | (2 << (4 * (pin - 8))))});}
          else {gpioc.afrl.modify(|r, w| unsafe {w.bits(r.bits() | (2 << (4 * pin)))});}
        }
      }
      else if timer == 8 || timer == 9 || timer == 10 || timer == 11 {
        if timer == 1 || timer == 2 {
          if pin > 7 {gpioc.afrh.modify(|r, w| unsafe {w.bits(r.bits() | (3 << (4 * (pin - 8))))});}
          else {gpioc.afrl.modify(|r, w| unsafe {w.bits(r.bits() | (3 << (4 * pin)))});}
        }
      }
      else {
        if timer == 1 || timer == 2 {
          if pin > 7 {gpioc.afrh.modify(|r, w| unsafe {w.bits(r.bits() | (9 << (4 * (pin - 8))))});}
          else {gpioc.afrl.modify(|r, w| unsafe {w.bits(r.bits() | (9 << (4 * pin)))});}
        }
      }
    },
    'd' => {
      let gpiod = &peripheral_ptr.GPIOD;
      rcc.ahb1enr.modify(|_, w| w.gpioden().enabled());
      gpiod.moder.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin)) | (2 << (2 * pin)))});
      if timer == 1 || timer == 2 {
        if pin > 7 {gpiod.afrh.modify(|r, w| unsafe {w.bits(r.bits() | (1 << (4 * (pin - 8))))});}
        else {gpiod.afrl.modify(|r, w| unsafe {w.bits(r.bits() | (1 << (4 * pin)))});}
      }
      else if timer == 3 || timer == 4 || timer == 5 {
        if timer == 1 || timer == 2 {
          if pin > 7 {gpiod.afrh.modify(|r, w| unsafe {w.bits(r.bits() | (2 << (4 * (pin - 8))))});}
          else {gpiod.afrl.modify(|r, w| unsafe {w.bits(r.bits() | (2 << (4 * pin)))});}
        }
      }
      else if timer == 8 || timer == 9 || timer == 10 || timer == 11 {
        if timer == 1 || timer == 2 {
          if pin > 7 {gpiod.afrh.modify(|r, w| unsafe {w.bits(r.bits() | (3 << (4 * (pin - 8))))});}
          else {gpiod.afrl.modify(|r, w| unsafe {w.bits(r.bits() | (3 << (4 * pin)))});}
        }
      }
      else {
        if timer == 1 || timer == 2 {
          if pin > 7 {gpiod.afrh.modify(|r, w| unsafe {w.bits(r.bits() | (9 << (4 * (pin - 8))))});}
          else {gpiod.afrl.modify(|r, w| unsafe {w.bits(r.bits() | (9 << (4 * pin)))});}
        }
      }
    },
    'e' => {
      let gpioe = &peripheral_ptr.GPIOE;
      rcc.ahb1enr.modify(|_, w| w.gpioeen().enabled());
      gpioe.moder.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin)) | (2 << (2 * pin)))});
      if timer == 1 || timer == 2 {
        if pin > 7 {gpioe.afrh.modify(|r, w| unsafe {w.bits(r.bits() | (1 << (4 * (pin - 8))))});}
        else {gpioe.afrl.modify(|r, w| unsafe {w.bits(r.bits() | (1 << (4 * pin)))});}
      }
      else if timer == 3 || timer == 4 || timer == 5 {
        if timer == 1 || timer == 2 {
          if pin > 7 {gpioe.afrh.modify(|r, w| unsafe {w.bits(r.bits() | (2 << (4 * (pin - 8))))});}
          else {gpioe.afrl.modify(|r, w| unsafe {w.bits(r.bits() | (2 << (4 * pin)))});}
        }
      }
      else if timer == 8 || timer == 9 || timer == 10 || timer == 11 {
        if timer == 1 || timer == 2 {
          if pin > 7 {gpioe.afrh.modify(|r, w| unsafe {w.bits(r.bits() | (3 << (4 * (pin - 8))))});}
          else {gpioe.afrl.modify(|r, w| unsafe {w.bits(r.bits() | (3 << (4 * pin)))});}
        }
      }
      else {
        if timer == 1 || timer == 2 {
          if pin > 7 {gpioe.afrh.modify(|r, w| unsafe {w.bits(r.bits() | (9 << (4 * (pin - 8))))});}
          else {gpioe.afrl.modify(|r, w| unsafe {w.bits(r.bits() | (9 << (4 * pin)))});}
        }
      }
    },
    'f' => {
      let gpiof = &peripheral_ptr.GPIOF;
      rcc.ahb1enr.modify(|_, w| w.gpiofen().enabled());
      gpiof.moder.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin)) | (2 << (2 * pin)))});
      if timer == 1 || timer == 2 {
        if pin > 7 {gpiof.afrh.modify(|r, w| unsafe {w.bits(r.bits() | (1 << (4 * (pin - 8))))});}
        else {gpiof.afrl.modify(|r, w| unsafe {w.bits(r.bits() | (1 << (4 * pin)))});}
      }
      else if timer == 3 || timer == 4 || timer == 5 {
        if timer == 1 || timer == 2 {
          if pin > 7 {gpiof.afrh.modify(|r, w| unsafe {w.bits(r.bits() | (2 << (4 * (pin - 8))))});}
          else {gpiof.afrl.modify(|r, w| unsafe {w.bits(r.bits() | (2 << (4 * pin)))});}
        }
      }
      else if timer == 8 || timer == 9 || timer == 10 || timer == 11 {
        if timer == 1 || timer == 2 {
          if pin > 7 {gpiof.afrh.modify(|r, w| unsafe {w.bits(r.bits() | (3 << (4 * (pin - 8))))});}
          else {gpiof.afrl.modify(|r, w| unsafe {w.bits(r.bits() | (3 << (4 * pin)))});}
        }
      }
      else {
        if timer == 1 || timer == 2 {
          if pin > 7 {gpiof.afrh.modify(|r, w| unsafe {w.bits(r.bits() | (9 << (4 * (pin - 8))))});}
          else {gpiof.afrl.modify(|r, w| unsafe {w.bits(r.bits() | (9 << (4 * pin)))});}
        }
      }
    },
    _   => panic!("P{}{} is not available for PWM output!", block.to_uppercase(), pin)
  };
  
  match timer {
    1 => {
      let tim1 = &peripheral_ptr.TIM1;
      
      rcc.apb2enr.modify(|_, w| w.tim1en().enabled());
      tim1.cr1.modify(|_, w| w.arpe().enabled());
      tim1.psc.write(|w| w.psc().bits(1000));
      tim1.arr.write_with_zero(|w| w.arr().bits(255));
      tim1.egr.write(|w| w.ug().set_bit());
      
      match channel {
        1 => tim1.ccmr1_output_mut().modify(|r, w| unsafe {w.bits(r.bits() | (0xD << 3))}),
        2 => tim1.ccmr1_output_mut().modify(|r, w| unsafe {w.bits(r.bits() | (0xD << 11))}),
        3 => tim1.ccmr2_output_mut().modify(|r, w| unsafe {w.bits(r.bits() | (0xD << 3))}),
        4 => tim1.ccmr2_output_mut().modify(|r, w| unsafe {w.bits(r.bits() | (0xD << 11))}),
        _ => panic!("Channel {} is not a valid CC channel!", channel)
      };
    },
    2 => {
      let tim2 = &peripheral_ptr.TIM2;
      
      rcc.apb2enr.modify(|_, w| w.tim1en().enabled());
      tim2.cr1.modify(|_, w| w.arpe().enabled());
      tim2.psc.write(|w| w.psc().bits(1000));
      tim2.arr.write_with_zero(|w| w.arr().bits(255));
      tim2.egr.write(|w| w.ug().set_bit());
      
      match channel {
        1 => tim2.ccmr1_output_mut().modify(|r, w| unsafe {w.bits(r.bits() | (0xD << 3))}),
        2 => tim2.ccmr1_output_mut().modify(|r, w| unsafe {w.bits(r.bits() | (0xD << 11))}),
        3 => tim2.ccmr2_output_mut().modify(|r, w| unsafe {w.bits(r.bits() | (0xD << 3))}),
        4 => tim2.ccmr2_output_mut().modify(|r, w| unsafe {w.bits(r.bits() | (0xD << 11))}),
        _ => panic!("Channel {} is not a valid CC channel!", channel)
      };
    },
    3 => {
      let tim3 = &peripheral_ptr.TIM3;
      
      rcc.apb2enr.modify(|_, w| w.tim1en().enabled());
      tim3.cr1.modify(|_, w| w.arpe().enabled());
      tim3.psc.write(|w| w.psc().bits(1000));
      tim3.arr.write_with_zero(|w| w.arr().bits(255));
      tim3.egr.write(|w| w.ug().set_bit());
      
      match channel {
        1 => tim3.ccmr1_output_mut().modify(|r, w| unsafe {w.bits(r.bits() | (0xD << 3))}),
        2 => tim3.ccmr1_output_mut().modify(|r, w| unsafe {w.bits(r.bits() | (0xD << 11))}),
        3 => tim3.ccmr2_output_mut().modify(|r, w| unsafe {w.bits(r.bits() | (0xD << 3))}),
        4 => tim3.ccmr2_output_mut().modify(|r, w| unsafe {w.bits(r.bits() | (0xD << 11))}),
        _ => panic!("Channel {} is not a valid CC channel!", channel)
      };
    },
    4 => {
      let tim4 = &peripheral_ptr.TIM4;
      
      rcc.apb2enr.modify(|_, w| w.tim1en().enabled());
      tim4.cr1.modify(|_, w| w.arpe().enabled());
      tim4.psc.write(|w| w.psc().bits(1000));
      tim4.arr.write_with_zero(|w| w.arr().bits(255));
      tim4.egr.write(|w| w.ug().set_bit());
      
      match channel {
        1 => tim4.ccmr1_output_mut().modify(|r, w| unsafe {w.bits(r.bits() | (0xD << 3))}),
        2 => tim4.ccmr1_output_mut().modify(|r, w| unsafe {w.bits(r.bits() | (0xD << 11))}),
        3 => tim4.ccmr2_output_mut().modify(|r, w| unsafe {w.bits(r.bits() | (0xD << 3))}),
        4 => tim4.ccmr2_output_mut().modify(|r, w| unsafe {w.bits(r.bits() | (0xD << 11))}),
        _ => panic!("Channel {} is not a valid CC channel!", channel)
      };
    },
    5 => {
      let tim5 = &peripheral_ptr.TIM5;
      
      rcc.apb2enr.modify(|_, w| w.tim1en().enabled());
      tim5.cr1.modify(|_, w| w.arpe().enabled());
      tim5.psc.write(|w| w.psc().bits(1000));
      tim5.arr.write_with_zero(|w| w.arr().bits(255));
      tim5.egr.write(|w| w.ug().set_bit());
      
      match channel {
        1 => tim5.ccmr1_output_mut().modify(|r, w| unsafe {w.bits(r.bits() | (0xD << 3))}),
        2 => tim5.ccmr1_output_mut().modify(|r, w| unsafe {w.bits(r.bits() | (0xD << 11))}),
        3 => tim5.ccmr2_output_mut().modify(|r, w| unsafe {w.bits(r.bits() | (0xD << 3))}),
        4 => tim5.ccmr2_output_mut().modify(|r, w| unsafe {w.bits(r.bits() | (0xD << 11))}),
        _ => panic!("Channel {} is not a valid CC channel!", channel)
      };
    },
    8 => {
      let tim8 = &peripheral_ptr.TIM8;
      
      rcc.apb2enr.modify(|_, w| w.tim1en().enabled());
      tim8.cr1.modify(|_, w| w.arpe().enabled());
      tim8.psc.write(|w| w.psc().bits(1000));
      tim8.arr.write_with_zero(|w| w.arr().bits(255));
      tim8.egr.write(|w| w.ug().set_bit());
      
      match channel {
        1 => tim8.ccmr1_output_mut().modify(|r, w| unsafe {w.bits(r.bits() | (0xD << 3))}),
        2 => tim8.ccmr1_output_mut().modify(|r, w| unsafe {w.bits(r.bits() | (0xD << 11))}),
        3 => tim8.ccmr2_output_mut().modify(|r, w| unsafe {w.bits(r.bits() | (0xD << 3))}),
        4 => tim8.ccmr2_output_mut().modify(|r, w| unsafe {w.bits(r.bits() | (0xD << 11))}),
        _ => panic!("Channel {} is not a valid CC channel!", channel)
      };
    },
    9 => {
      let tim9 = &peripheral_ptr.TIM9;
      
      rcc.apb2enr.modify(|_, w| w.tim1en().enabled());
      tim9.cr1.modify(|_, w| w.arpe().enabled());
      tim9.psc.write(|w| w.psc().bits(1000));
      tim9.arr.write_with_zero(|w| unsafe {w.arr().bits(255)});
      tim9.egr.write(|w| w.ug().set_bit());
      
      match channel {
        1 => tim9.ccmr1_output_mut().modify(|r, w| unsafe {w.bits(r.bits() | (0xD << 3))}),
        2 => tim9.ccmr1_output_mut().modify(|r, w| unsafe {w.bits(r.bits() | (0xD << 11))}),
        _ => panic!("Channel {} is not a valid CC channel!", channel)
      };
    },
    10 => {
      let tim10 = &peripheral_ptr.TIM10;
      
      rcc.apb2enr.modify(|_, w| w.tim1en().enabled());
      tim10.cr1.modify(|_, w| w.arpe().enabled());
      tim10.psc.write(|w| w.psc().bits(1000));
      tim10.arr.write_with_zero(|w| unsafe {w.arr().bits(255)});
      tim10.egr.write(|w| w.ug().set_bit());
      
      match channel {
        1 => tim10.ccmr1_output_mut().modify(|r, w| unsafe {w.bits(r.bits() | (0xD << 3))}),
        2 => tim10.ccmr1_output_mut().modify(|r, w| unsafe {w.bits(r.bits() | (0xD << 11))}),
        _ => panic!("Channel {} is not a valid CC channel!", channel)
      };
    },
    11 => {
      let tim11 = &peripheral_ptr.TIM11;
      
      rcc.apb2enr.modify(|_, w| w.tim1en().enabled());
      tim11.cr1.modify(|_, w| w.arpe().enabled());
      tim11.psc.write(|w| w.psc().bits(1000));
      tim11.arr.write_with_zero(|w| unsafe {w.arr().bits(255)});
      tim11.egr.write(|w| w.ug().set_bit());
      
      match channel {
        1 => tim11.ccmr1_output_mut().modify(|r, w| unsafe {w.bits(r.bits() | (0xD << 3))}),
        2 => tim11.ccmr1_output_mut().modify(|r, w| unsafe {w.bits(r.bits() | (0xD << 11))}),
        _ => panic!("Channel {} is not a valid CC channel!", channel)
      };
    },
    12 => {
      let tim12 = &peripheral_ptr.TIM12;
      
      rcc.apb2enr.modify(|_, w| w.tim1en().enabled());
      tim12.cr1.modify(|_, w| w.arpe().enabled());
      tim12.psc.write(|w| w.psc().bits(1000));
      tim12.arr.write_with_zero(|w| unsafe {w.arr().bits(255)});
      tim12.egr.write(|w| w.ug().set_bit());
      
      match channel {
        1 => tim12.ccmr1_output_mut().modify(|r, w| unsafe {w.bits(r.bits() | (0xD << 3))}),
        2 => tim12.ccmr1_output_mut().modify(|r, w| unsafe {w.bits(r.bits() | (0xD << 11))}),
        _ => panic!("Channel {} is not a valid CC channel!", channel)
      };
    },
    13 => {
      let tim13 = &peripheral_ptr.TIM13;
      
      rcc.apb2enr.modify(|_, w| w.tim1en().enabled());
      tim13.cr1.modify(|_, w| w.arpe().enabled());
      tim13.psc.write(|w| w.psc().bits(1000));
      tim13.arr.write_with_zero(|w| unsafe {w.arr().bits(255)});
      tim13.egr.write(|w| w.ug().set_bit());
      
      match channel {
        1 => tim13.ccmr1_output_mut().modify(|r, w| unsafe {w.bits(r.bits() | (0xD << 3))}),
        2 => tim13.ccmr1_output_mut().modify(|r, w| unsafe {w.bits(r.bits() | (0xD << 11))}),
        _ => panic!("Channel {} is not a valid CC channel!", channel)
      };
    },
    14 => {
      let tim14 = &peripheral_ptr.TIM14;
      
      rcc.apb2enr.modify(|_, w| w.tim1en().enabled());
      tim14.cr1.modify(|_, w| w.arpe().enabled());
      tim14.psc.write(|w| w.psc().bits(1000));
      tim14.arr.write(|w| unsafe {w.arr().bits(255)});
      tim14.egr.write(|w| w.ug().set_bit());
      
      match channel {
        1 => tim14.ccmr1_output_mut().modify(|r, w| unsafe {w.bits(r.bits() | (0xD << 3))}),
        2 => tim14.ccmr1_output_mut().modify(|r, w| unsafe {w.bits(r.bits() | (0xD << 11))}),
        _ => panic!("Channel {} is not a valid CC channel!", channel)
      };
    },
    _  => panic!("Timer {} is not a valid timer!", timer)
  };
}

fn pwm_set_duty(timer: usize, channel: usize, value: u8) {
  let peripheral_ptr = stm32f4::stm32f446::Peripherals::take().unwrap();
  
  match timer {
    1 => {
      let tim1 = &peripheral_ptr.TIM1;
      match channel {
        1 => tim1.ccr1.write_with_zero(|w| w.ccr().bits(value as u16)),
        2 => tim1.ccr2.write_with_zero(|w| w.ccr().bits(value as u16)),
        3 => tim1.ccr3.write_with_zero(|w| w.ccr().bits(value as u16)),
        4 => tim1.ccr4.write_with_zero(|w| w.ccr().bits(value as u16)),
        _ => panic!("Channel {} is not a valid CC channel!", channel)
      };
    },
    2 => {
      let tim2 = &peripheral_ptr.TIM2;
      match channel {
        1 => tim2.ccr1.write_with_zero(|w| w.ccr().bits(value as u32)),
        2 => tim2.ccr2.write_with_zero(|w| w.ccr().bits(value as u32)),
        3 => tim2.ccr3.write_with_zero(|w| w.ccr().bits(value as u32)),
        4 => tim2.ccr4.write_with_zero(|w| w.ccr().bits(value as u32)),
        _ => panic!("Channel {} is not a valid CC channel!", channel)
      };
    },
    3 => {
      let tim3 = &peripheral_ptr.TIM3;
      match channel {
        1 => tim3.ccr1.write_with_zero(|w| w.ccr().bits(value as u16)),
        2 => tim3.ccr2.write_with_zero(|w| w.ccr().bits(value as u16)),
        3 => tim3.ccr3.write_with_zero(|w| w.ccr().bits(value as u16)),
        4 => tim3.ccr4.write_with_zero(|w| w.ccr().bits(value as u16)),
        _ => panic!("Channel {} is not a valid CC channel!", channel)
      };
    },
    4 => {
      let tim4 = &peripheral_ptr.TIM4;
      match channel {
        1 => tim4.ccr1.write_with_zero(|w| w.ccr().bits(value as u16)),
        2 => tim4.ccr2.write_with_zero(|w| w.ccr().bits(value as u16)),
        3 => tim4.ccr3.write_with_zero(|w| w.ccr().bits(value as u16)),
        4 => tim4.ccr4.write_with_zero(|w| w.ccr().bits(value as u16)),
        _ => panic!("Channel {} is not a valid CC channel!", channel)
      };
    },
    5 => {
      let tim5 = &peripheral_ptr.TIM5;
      match channel {
        1 => tim5.ccr1.write_with_zero(|w| w.ccr().bits(value as u32)),
        2 => tim5.ccr2.write_with_zero(|w| w.ccr().bits(value as u32)),
        3 => tim5.ccr3.write_with_zero(|w| w.ccr().bits(value as u32)),
        4 => tim5.ccr4.write_with_zero(|w| w.ccr().bits(value as u32)),
        _ => panic!("Channel {} is not a valid CC channel!", channel)
      };
    },
    8 => {
      let tim8 = &peripheral_ptr.TIM8;
      match channel {
        1 => tim8.ccr1.write_with_zero(|w| w.ccr().bits(value as u16)),
        2 => tim8.ccr2.write_with_zero(|w| w.ccr().bits(value as u16)),
        3 => tim8.ccr3.write_with_zero(|w| w.ccr().bits(value as u16)),
        4 => tim8.ccr4.write_with_zero(|w| w.ccr().bits(value as u16)),
        _ => panic!("Channel {} is not a valid CC channel!", channel)
      };
    },
    9 => {
      let tim9 = &peripheral_ptr.TIM9;
      match channel {
        1 => tim9.ccr1.write_with_zero(|w| unsafe {w.ccr().bits(value as u16)}),
        2 => tim9.ccr2.write_with_zero(|w| unsafe {w.ccr().bits(value as u16)}),
        _ => panic!("Channel {} is not a valid CC channel!", channel)
      };
    },
    10 => {
      let tim10 = &peripheral_ptr.TIM10;
      match channel {
        1 => tim10.ccr1.write_with_zero(|w| unsafe {w.ccr().bits(value as u16)}),
        _ => panic!("Channel {} is not a valid CC channel!", channel)
      };
    },
    11 => {
      let tim11 = &peripheral_ptr.TIM11;
      match channel {
        1 => tim11.ccr1.write_with_zero(|w| unsafe {w.ccr().bits(value as u16)}),
        _ => panic!("Channel {} is not a valid CC channel!", channel)
      };
    },
    12 => {
      let tim12 = &peripheral_ptr.TIM12;
      match channel {
        1 => tim12.ccr1.write_with_zero(|w| unsafe {w.ccr().bits(value as u16)}),
        2 => tim12.ccr2.write_with_zero(|w| unsafe {w.ccr().bits(value as u16)}),
        _ => panic!("Channel {} is not a valid CC channel!", channel)
      };
    },
    13 => {
      let tim13 = &peripheral_ptr.TIM13;
      match channel {
        1 => tim13.ccr1.write_with_zero(|w| unsafe {w.ccr().bits(value as u16)}),
        _ => panic!("Channel {} is not a valid CC channel!", channel)
      };
    },
    14 => {
      let tim14 = &peripheral_ptr.TIM14;
      match channel {
        1 => tim14.ccr1.write_with_zero(|w| unsafe {w.ccr().bits(value as u16)}),
        _ => panic!("Channel {} is not a valid CC channel!", channel)
      };
    },
    _ => panic!("Timer {} is not a valid timer!", timer)
  };
}


// Standalone time functions ======================================================================  
pub fn delay(ms: u32) {
  let peripheral_ptr = stm32f4::stm32f446::Peripherals::take().unwrap();
  let systick = &peripheral_ptr.STK;
  
  if systick.ctrl.read().enable().bit_is_clear() {
    // 2MHz mit 2000 PSC -> 1kHz
    systick.load.write(|w| unsafe {w.reload().bits(2000000 / 1000)});
    systick.val.reset();
    systick.ctrl.modify(|_, w| {
      w.tickint().set_bit();
      w.enable().set_bit()
    });
  }
  
  unsafe {
    DELAY_COUNTER.1 = 0;
    DELAY_COUNTER.0 = true;
    while DELAY_COUNTER.1 < ms {}
    DELAY_COUNTER.0 = false;
  }
}

pub fn start_time() {
  let peripheral_ptr = stm32f4::stm32f446::Peripherals::take().unwrap();
  let rcc = &peripheral_ptr.RCC;
  let tim6 = &peripheral_ptr.TIM6;
  
  unsafe {
    if TIMER_CONF[20] == false {TIMER_CONF[20] = true;}
    else {
      hprintln!("Millis Timer already configured!").expect("Could not send semihosting message!");
      return;
    }
  }
  
  rcc.apb1enr.modify(|_, w| w.tim6en().enabled());
  tim6.cr1.modify(|_, w| w.arpe().enabled());
  
  tim6.dier.modify(|_, w| w.uie().enabled());
  unsafe {NVIC::unmask(Interrupt::TIM1_UP_TIM10);}
  
  tim6.psc.write(|w| w.psc().bits(8));
  tim6.arr.write(|w| w.arr().bits(1000));
  tim6.egr.write(|w| w.ug().update());
  tim6.cr1.modify(|_, w| w.cen().enabled());
}

pub fn millis() -> usize {
  let peripheral_ptr = stm32f4::stm32f446::Peripherals::take().unwrap();
  let tim6 = &peripheral_ptr.TIM6;
  let buffer: usize;
  
  tim6.cr1.modify(|_, w| w.cen().disabled());
  unsafe {buffer = TIME_COUNTER;}
  tim6.cr1.modify(|_, w| w.cen().enabled());
  
  return buffer;
}


// Interrupts and Exceptions ====================================================================
#[exception]
fn SysTick() {
  unsafe {
    if DELAY_COUNTER.0 == true {DELAY_COUNTER.1 += 1;}
  }
}

#[interrupt]
fn TIM1_UP_TIM10() {
  unsafe {TIME_COUNTER += 1;}
}
