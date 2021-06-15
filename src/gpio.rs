use super::include::{GPIOB_PTR, RCC_PTR};
use super::{Bias, Mode, Speed};

impl Mode {
  const fn bit_value(&self) -> u32 {
    match self {
      Mode::Input => 0,
      Mode::Output => 1,
      Mode::AlterateFunction(_) => 2,
      Mode::Analog(_) => 3,
    }
  }
}

impl Speed {
  const fn bit_value(&self) -> u32 {
    match self {
      Speed::Low => 0,
      Speed::Medium => 1,
      Speed::High => 2,
      Speed::Fast => 3,
    }
  }
}

impl Bias {
  const fn bit_value(&self) -> u32 {
    match self {
      Bias::None => 0,
      Bias::Pullup => 1,
      Bias::Pulldown => 2,
    }
  }
}

/// Common interface for all GPIO registers
pub trait GpioRegister {
  unsafe fn initialize();
  unsafe fn set_pin_mode(pin_number: u8, mode: Mode);
  unsafe fn set_speed(pin_number: u8, speed: Speed);
  unsafe fn set_bias(pin_number: u8, bias: Bias);
  unsafe fn write_output(pin_number: u8, value: bool);
  unsafe fn read_pin(pin_number: u8) -> bool;
}

pub struct GpioB {}

impl GpioRegister for GpioB {
  unsafe fn initialize() {
    (*RCC_PTR).ahb1enr.modify(|_, w| w.gpioben().enabled());
  }

  unsafe fn set_pin_mode(pin_number: u8, mode: Mode) {
    (*GPIOB_PTR).moder.modify(|r, w| {
      w.bits(r.bits() & !(3 << (2 * pin_number)) | (mode.bit_value() << (2 * pin_number)))
    })
  }

  unsafe fn set_speed(pin_number: u8, speed: Speed) {
    (*GPIOB_PTR).ospeedr.modify(|r, w| {
      w.bits(r.bits() & !(3 << (2 * pin_number)) | (speed.bit_value() << (2 * pin_number)))
    });
  }

  unsafe fn set_bias(pin_number: u8, bias: Bias) {
    (*GPIOB_PTR).pupdr.modify(|r, w| {
      w.bits(r.bits() & !(3 << (2 * pin_number)) | (bias.bit_value() << (2 * pin_number)))
    });
  }

  unsafe fn write_output(pin_number: u8, value: bool) {
    let start_bit = if value {
      1
    } else {
      0x10000 // start at bit 16
    };
    (*GPIOB_PTR).bsrr.write(|w| w.bits(start_bit << pin_number));
  }

  unsafe fn read_pin(pin_number: u8) -> bool {
    let bits = (*GPIOB_PTR).idr.read().bits();
    bits & (1 << pin_number) == (1 << pin_number)
  }
}

/// Gerneric type used for automatically implementing GPIO pins
pub struct GpioPin<R: GpioRegister, const N: u8> {
  register: core::marker::PhantomData<R>,
}

impl<R: GpioRegister, const N: u8> Pin for GpioPin<R, N> {
  unsafe fn initialize() -> Self {
    R::initialize();
    GpioPin {
      register: core::marker::PhantomData,
    }
  }
}

impl<R: GpioRegister, const N: u8> GpioOutput for GpioPin<R, N> {
  unsafe fn configure_as_output(&self) {
    R::set_pin_mode(N, Mode::Output);
  }

  unsafe fn set_speed(&self, speed: Speed) {
    R::set_speed(N, speed);
  }

  unsafe fn set_value(&self, value: bool) {
    R::write_output(N, value);
  }
}

impl<R: GpioRegister, const N: u8> GpioInput for GpioPin<R, N> {
  unsafe fn configure_as_input(&self) {
    R::set_pin_mode(N, Mode::Input);
  }

  unsafe fn set_bias(&self, bias: Bias) {
    R::set_bias(N, bias);
  }

  unsafe fn read_value(&self) -> bool {
    R::read_pin(N)
  }
}

/// Defines a common interface for all pin types.
pub trait Pin: Sized {
  /// Intitialize the pin and get pin struct.
  /// # Safety
  /// This is unsafe because this type does not enforce any guarantees to prevent misuse.
  /// Only use this function to write save wrappers around this API.
  unsafe fn initialize() -> Self;
}

/// Defines an unsafe common interface for all input pins.
///
/// This should only be used to create safe wrappers around this interface.
pub trait GpioInput: Sized + Pin {
  /// Configure pin as input.
  /// # Safety
  /// This function just flips a bunch of registers and doesn't care what configuration was used before.
  /// Make sure to reset the pin if needed before calling this function.
  unsafe fn configure_as_input(&self);

  /// # Safety
  /// This function is only safe to be called on a pin configured as input.
  unsafe fn set_bias(&self, bias: Bias);

  /// # Safety
  /// This function is only safe to be called on a pin configured as input.
  unsafe fn read_value(&self) -> bool;

  /// Receive input pin inside a wrapper stuct that guarantees save usage.
  fn get_as_input() -> InputPin<Self> {
    InputPin {
      inner: unsafe {
        let pin = Self::initialize();
        pin.configure_as_input();
        pin
      },
    }
  }
}

/// Defines a common interface for all output pins.
/// This should only be used to create safe wrappers around this interface.
pub trait GpioOutput: Sized + Pin {
  /// Configure pin as input.
  /// # Safety
  /// This function just flips a bunch of registers and doesn't care what configuration was used before.
  /// Make sure to reset the pin if needed before calling this function.
  unsafe fn configure_as_output(&self);

  /// # Safety
  /// This function is only safe to be called on a pin configured as output.
  unsafe fn set_speed(&self, speed: Speed);

  /// # Safety
  /// This function is only safe to be called on a pin configured as output.
  unsafe fn set_value(&self, value: bool);

  /// Receive output pin inside a wrapper stuct that guarantees save usage.
  fn get_as_output() -> OutputPin<Self> {
    OutputPin {
      inner: unsafe {
        let pin = Self::initialize();
        pin.configure_as_output();
        pin
      },
    }
  }
}

// State machine implemeting a safe interface for GPIO pins
pub struct InputPin<T: GpioInput> {
  inner: T,
}

pub struct OutputPin<T: GpioOutput> {
  inner: T,
}

impl<T: GpioInput> InputPin<T> {
  pub fn read_value(&self) -> bool {
    unsafe { self.inner.read_value() }
  }

  pub fn set_bias(&mut self, bias: Bias) {
    unsafe {
      self.inner.set_bias(bias);
    }
  }
}

pub fn read_value<T: GpioInput>(pin: &InputPin<T>) -> bool {
  pin.read_value()
}

pub fn set_bias<T: GpioInput>(pin: &mut InputPin<T>, bias: Bias) {
  pin.set_bias(bias);
}

impl<T: GpioInput + GpioOutput> InputPin<T> {
  pub fn into_output(self) -> OutputPin<T> {
    unsafe {
      self.inner.configure_as_output();
    }
    OutputPin { inner: self.inner }
  }
}

pub fn into_output<T: GpioInput + GpioOutput>(pin: InputPin<T>) -> OutputPin<T> {
  pin.into_output()
}

impl<T: GpioOutput> OutputPin<T> {
  pub fn set_speed(&mut self, speed: Speed) {
    unsafe {
      self.inner.set_speed(speed);
    }
  }

  pub fn set_value(&mut self, value: bool) {
    unsafe {
      self.inner.set_value(value);
    }
  }
}

pub fn set_value<T: GpioOutput>(pin: &mut OutputPin<T>, value: bool) {
  pin.set_value(value);
}

pub fn set_speed<T: GpioOutput>(pin: &mut OutputPin<T>, speed: Speed) {
  pin.set_speed(speed);
}

impl<T: GpioInput + GpioOutput> OutputPin<T> {
  pub fn into_input(self) -> InputPin<T> {
    unsafe {
      self.inner.configure_as_input();
    }
    InputPin { inner: self.inner }
  }
}

pub fn into_input<T: GpioInput + GpioOutput>(pin: OutputPin<T>) -> InputPin<T> {
  pin.into_input()
}

// Define pin types
pub type PB0 = GpioPin<GpioB, 0>;
pub type PB1 = GpioPin<GpioB, 1>;
pub type PB2 = GpioPin<GpioB, 2>;
pub type PB3 = GpioPin<GpioB, 3>;
pub type PB4 = GpioPin<GpioB, 4>;
pub type PB5 = GpioPin<GpioB, 5>;
pub type PB6 = GpioPin<GpioB, 6>;
pub type PB7 = GpioPin<GpioB, 7>;
pub type PB8 = GpioPin<GpioB, 8>;

/*pub struct PB0;

impl Pin for PB0 {
  unsafe fn initialize() -> Self {
    (*RCC_PTR).ahb1enr.modify(|_, w| w.gpioben().enabled());
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
    configure_pin_as_input(GPIOB_PTR, 0);
  }

  unsafe fn read_value(&self) -> bool {
    let bits = (*GPIOB_PTR).idr.read().bits();
    bits & (1 << 0) == (1 << 0)
  }

  unsafe fn set_bias(&self, bias: Bias) {
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
    configure_pin_as_output(GPIOB_PTR, 0);
  }

  unsafe fn set_speed(&self, speed: Speed) {
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

  unsafe fn set_value(&self, value: bool) {
    let start_bit = if value {
      1
    } else {
      0x10000 // start at bit 16
    };
    (*GPIOB_PTR).bsrr.write(|w| w.bits(start_bit << 0));
  }
}*/
