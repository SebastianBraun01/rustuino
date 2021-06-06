use super::include::{GPIOB_PTR, RCC_PTR};
use super::{Bias, Speed};

/// Defines a common interface for all pin types.
pub trait Pin: Sized {
    /// Get pin struct.
    /// # Safety
    /// This is unsafe because this type does not enforce any guarantees to prevent misuse.
    /// This function also does not configure the pin!
    /// Only use this function to write save wrappers around this API.
    unsafe fn get_as_pin_raw() -> Self;
}

/// Defines a common interface for all input pins.
/// This should only be used to create safe wrappers around this interface.
pub trait GpioInput: Sized {
    /// Configure pin as input.
    /// # Safety
    /// This function just flips a bunch of registers and doesn't care what configuration was used before.
    /// Make sure to reset the pin if needed before calling this function.
    unsafe fn configure_as_input(&self);

    fn set_bias(&self, bias: Bias);

    fn read_value(&self) -> bool;
}

pub trait GetAsInput: GpioInput {
    fn get_as_input() -> InputPin<Self>;
}

pub trait GetAsOutput: GpioOutput {
    fn get_as_output() -> OutputPin<Self>;
}


/// Defines a common interface for all output pins.
/// This should only be used to create safe wrappers around this interface.
pub trait GpioOutput: Sized {
    /// Configure pin as input.
    /// # Safety
    /// This function just flips a bunch of registers and doesn't care what configuration was used before.
    /// Make sure to reset the pin if needed before calling this function.
    unsafe fn configure_as_output(&self);

    fn set_speed(&self, speed: Speed);

    fn set_value(&self, value: bool);
}

pub struct InputPin<T: GpioInput> {
    inner: T,
}

pub struct OutputPin<T: GpioOutput> {
    inner: T,
}

impl<T: Pin + GpioInput> GetAsInput for T {
    fn get_as_input() -> InputPin<Self> {
        InputPin {
            inner: unsafe {
                let pin = Self::get_as_pin_raw();
                pin.configure_as_input();
                pin
            },
        }
    }
}

impl<T: Pin + GpioOutput> GetAsOutput for T {
    fn get_as_output() -> OutputPin<Self> {
        OutputPin {
            inner: unsafe {
                let pin = Self::get_as_pin_raw();
                pin.configure_as_output();
                pin
            },
        }
    }
}

impl<T: GpioInput> InputPin<T> {
    pub fn read_value(&self) -> bool {
        self.inner.read_value()
    }

    pub fn set_bias(&mut self, bias: Bias) {
        self.inner.set_bias(bias);
    }
}

impl<T: GpioInput + GpioOutput> InputPin<T> {
    pub fn into_output(self) -> OutputPin<T> {
        unsafe {
            self.inner.configure_as_output();
        }
        OutputPin { inner: self.inner }
    }
}

impl<T: GpioOutput> OutputPin<T> {
    pub fn write(&mut self, value: bool) {
        self.inner.set_value(value);
    }

    pub fn set_speed(&mut self, speed: Speed) {
        self.inner.set_speed(speed);
    }

    pub fn set_value(&mut self, value: bool) {
        self.inner.set_value(value);
    }
}

impl<T: GpioInput + GpioOutput> OutputPin<T> {
    pub fn into_input(self) -> InputPin<T> {
        unsafe {
            self.inner.configure_as_input();
        }
        InputPin { inner: self.inner }
    }
}


pub struct PB0;

impl Pin for PB0 {
    unsafe fn get_as_pin_raw() -> Self {
        PB0
    }
}

unsafe fn configure_pin_as_input(
    register: *const stm32f4::stm32f446::gpiob::RegisterBlock,
    pin_number: u8,
) {
    (*register)
        .moder
        .modify(|r, w| w.bits(r.bits() & !(3 << (2 * pin_number))));
}

impl GpioInput for PB0 {
    unsafe fn configure_as_input(&self) {
        (*RCC_PTR).ahb1enr.modify(|_, w| w.gpioben().enabled());
        configure_pin_as_input(GPIOB_PTR, 0);
    }

    fn read_value(&self) -> bool {
        let bits = unsafe { (*GPIOB_PTR).idr.read().bits() };
        bits & (1 << 0) == (1 << 0)
    }

    fn set_bias(&self, bias: Bias) {
        unsafe {
            match bias {
                Bias::None => {
                    (*GPIOB_PTR)
                        .pupdr
                        .modify(|r, w| w.bits(r.bits() & !(3 << (2 * 0))));
                }
                Bias::Pullup => {
                    (*GPIOB_PTR)
                        .pupdr
                        .modify(|r, w| w.bits(r.bits() & !(3 << (2 * 0)) | (1 << (2 * 0))));
                }
                Bias::Pulldown => {
                    (*GPIOB_PTR)
                        .pupdr
                        .modify(|r, w| w.bits(r.bits() & !(3 << (2 * 0)) | (2 << (2 * 0))));
                }
            }
        }
    }
}

unsafe fn configure_pin_as_output(
    register: *const stm32f4::stm32f446::gpiob::RegisterBlock,
    pin_number: u8,
) {
    (*register)
        .moder
        .modify(|r, w| w.bits(r.bits() & !(3 << (2 * pin_number)) | (1 << (2 * pin_number))));
}

impl GpioOutput for PB0 {
    unsafe fn configure_as_output(&self) {
        (*RCC_PTR).ahb1enr.modify(|_, w| w.gpioben().enabled());
        configure_pin_as_output(GPIOB_PTR, 0);
    }

    fn set_speed(&self, speed: Speed) {
        unsafe {
            match speed {
                Speed::Low => {
                    (*GPIOB_PTR)
                        .ospeedr
                        .modify(|r, w| w.bits(r.bits() & !(3 << (2 * 0))));
                }
                Speed::Medium => {
                    (*GPIOB_PTR)
                        .ospeedr
                        .modify(|r, w| w.bits(r.bits() & !(3 << (2 * 0)) | (1 << (2 * 0))));
                }
                Speed::Fast => {
                    (*GPIOB_PTR)
                        .ospeedr
                        .modify(|r, w| w.bits(r.bits() & !(3 << (2 * 0)) | (2 << (2 * 0))));
                }
                Speed::High => {
                    (*GPIOB_PTR)
                        .ospeedr
                        .modify(|r, w| w.bits(r.bits() | (3 << (2 * 0))));
                }
            }
        }
    }

    fn set_value(&self, value: bool) {
        let start_bit = if value {
            1
        } else {
            0x10000 // start at bit 16
        };
        unsafe {
            (*GPIOB_PTR).bsrr.write(|w| w.bits(start_bit << 0));
        }
    }
}


