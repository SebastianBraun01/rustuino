use cortex_m::peripheral::NVIC;
use stm32f4::stm32f446::{Interrupt, interrupt};
use super::include::variables::TIME_COUNTER;

pub fn delay(ms: u32) {
  let peripheral_ptr = stm32f4::stm32f446::Peripherals::take().unwrap();
  let systick = &peripheral_ptr.STK;

  // 2MHz mit 2000 PSC -> 1kHz
  let systick_psc = 2000000 / 1000;

  if ms * systick_psc > (2^24) - 1 {panic!("Delay value too large for Timer!");}

  systick.ctrl.modify(|_, w| w.enable().clear_bit());
  systick.load.write(|w| unsafe {w.reload().bits(systick_psc * ms)});
  systick.val.write(|w| unsafe {w.current().bits(0)});
  systick.ctrl.modify(|_, w| w.enable().set_bit());

  while !systick.ctrl.read().countflag().bit_is_set() {}
  systick.ctrl.modify(|_, w| w.countflag().clear_bit());
  systick.ctrl.modify(|_, w| w.enable().clear_bit());
}

pub fn start_time() {
  let peripheral_ptr = stm32f4::stm32f446::Peripherals::take().unwrap();
  let rcc = &peripheral_ptr.RCC;
  let tim9 = &peripheral_ptr.TIM9;

  rcc.apb2enr.modify(|_, w| w.tim9en().enabled());
  tim9.dier.modify(|_, w| w.uie().enabled());

  unsafe {NVIC::unmask(Interrupt::TIM8_UP_TIM13);}
 
  tim9.arr.modify(|_, w| unsafe {w.arr().bits(8000)});
  tim9.egr.write(|w| w.ug().update());
  tim9.cr1.modify(|_, w| w.cen().enabled());
}

pub fn millis() -> usize {
  let peripheral_ptr = stm32f4::stm32f446::Peripherals::take().unwrap();
  let tim9 = &peripheral_ptr.TIM9;
  let buffer: usize;

  tim9.cr1.modify(|_, w| w.cen().disabled());
  while tim9.sr.read().uif().bit_is_clear() == false {}
  unsafe {buffer = TIME_COUNTER;}
  tim9.cr1.modify(|_, w| w.cen().enabled());

  return buffer;
}

// TODO: timer_init function!!!!!
pub fn timer_init(timer: u8, time: usize) {

}

#[allow(non_snake_case)]
#[interrupt]
fn TIM8_UP_TIM13() {
  unsafe {TIME_COUNTER += 1;}
}
