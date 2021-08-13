#![no_std]

// Library includes ===============================================================================
pub use cortex_m_rt::{entry, exception};
pub use stm32f4::stm32f446::{NVIC, Interrupt, interrupt};
pub use panic_halt as _;

// Für benutzer
pub use libm::*;
pub use heapless::{Vec, String};
pub use {common::*, gpio::*, analog::*, time::*, uart::*};


// Submodule includes =============================================================================
pub mod include;
pub mod common;
pub mod gpio;
pub mod analog;
pub mod time;
pub mod uart;
pub mod i2c;
