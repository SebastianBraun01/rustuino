#![no_std]
#![no_main]
#![feature(alloc_error_handler)]
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
    init_heap();

    gpio_d::pin_mode("a", 2, Mode::AlterateFunction(7));
    gpio_d::pin_mode("a", 2, Mode::AlterateFunction(7));
    gpio_d::pin_mode("b", 0, Mode::AlterateFunction(2));

    uart::uart_init(2, 115200);
    sprintln!("UART gestartet!");
    pwm::pwm_init(3, 3);

    loop {
        for i in 0..255 {
            pwm::pin_write_pwm(3, 3, i);
            time::delay(10);
        }
    }
}
