use super::include::TIME_COUNTER;
use super::include::SYSTICK_PTR;
use super::include::RCC_PTR;
use super::include::TIM9_PTR;

pub fn delay(ms: u32) {
  // 2MHz mit 2000 PSC -> 1kHz
  let systick_psc = 2000000 / 1000;

  if ms * systick_psc > (2^24) - 1 {panic!("Delay value too large for Timer!");}

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

// TODO: setup timer interrupt
pub fn start_time() {
  unsafe {
    (*RCC_PTR).apb2enr.modify(|_, w| w.tim9en().enabled());
    (*TIM9_PTR).dier.modify(|_, w| w.uie().enabled());
    (*TIM9_PTR).arr.modify(|_, w| w.arr().bits(8000));
    (*TIM9_PTR).egr.write(|w| w.ug().update());
    (*TIM9_PTR).cr1.modify(|_, w| w.cen().enabled());
  }
}

pub fn millis() -> usize {
  let buffer: usize;

  unsafe {
    (*TIM9_PTR).cr1.modify(|_, w| w.cen().disabled());
    while (*TIM9_PTR).sr.read().uif().bit_is_clear() == false {}
    buffer = TIME_COUNTER;
    (*TIM9_PTR).cr1.modify(|_, w| w.cen().enabled());
  }

  return buffer;
}
