#![no_std]

// Library includes ===============================================================================
pub use cortex_m_rt::{entry, exception};
pub use stm32f4::stm32f446::{NVIC, Interrupt, interrupt};

// Für benutzer
pub use libm;
pub use heapless::{Vec, String};
pub use {analog::*, time::*, uart::serial};
pub use gpio::{*, pins::*};
pub use rtt_target::{rtt_init_print, rprint, rprintln};


// Submodule includes =============================================================================
pub mod include;
pub mod gpio;
pub mod analog;
pub mod time;
pub mod uart;
pub mod i2c;
pub mod spi;


// Panic handler ==================================================================================
use core::panic::PanicInfo;

#[inline(never)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
  rtt_target::rprintln!("{}", info);
  loop {core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);}
}
