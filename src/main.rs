#![no_std]
#![no_main]
#![allow(unused_imports)]

// use stm32f4::stm32f446::{self, Peripherals, interrupt};
// unsafe {cortex_m::peripheral::NVIC::unmask(stm32f446::Interrupt::TIM2);}
// #[interrupt]
// fn TIM2() {
// }

mod rustuino;
use rustuino::*;

#[entry]
fn main() -> ! {
    // adc_init_test([PA0, PA1]);

    pin_mode(PA2, Mode::AlterateFunction(7));
    pin_mode(PA3, Mode::AlterateFunction(7));
    pin_mode(PB0, Mode::AlterateFunction(2));

    uart_init(2, 115200);
    sprintln!("UART gestartet!");
    pwm_init(3, 3);

    loop {
        for i in 0..255 {
            pin_write_pwm(3, 3, i);
            time::delay(10);
        }
    }
}
