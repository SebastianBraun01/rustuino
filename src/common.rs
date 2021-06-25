use super::gpio::{Bias, Speed};


// Structs ========================================================================================
pub struct GpioPin<const B: char, const P: u8, const M: u8> {}

// M: 2⁵ -> SPI     +
//    2⁴ -> I2C     +
//    2³ -> UART    +
//    2² -> PWM     +
//    2¹ -> Analog  +
//    2⁰ -> GPIO    +

// pub type PB0 = GpioPin<'a', 0, 1>;
macro_rules! generate_pins {
  ($([$block:literal, $pin:literal, $func:literal]),+) => {
    use paste::paste;

    paste!{
      $(
      pub type [<P $block:upper $pin>] = GpioPin<$block, $pin, $func>;
      )+
    }
  };
}

generate_pins![
  ['a', 0, 15],
  ['a', 1, 15],
  ['a', 2, 15],
  ['a', 3, 15],
  ['a', 4, 3],
  ['a', 5, 39],
  ['a', 6, 39],
  ['a', 7, 39],
  ['a', 8, 21],
  ['a', 9, 29],
  ['a', 10, 13],
  ['a', 11, 5],
  ['a', 12, 1],
  ['a', 13, 1],
  ['a', 14, 1],
  ['a', 15, 5],

  ['b', 0, 39],
  ['b', 1, 35],
  ['b', 2, 37],
  ['b', 3, 53],
  ['b', 4, 53],
  ['b', 5, 37],
  ['b', 6, 21],
  ['b', 7, 21],
  ['b', 8, 21],
  ['b', 9, 21],
  ['b', 10, 61],
  ['b', 11, 29],
  ['b', 12, 1],
  ['b', 13, 33],
  ['b', 14, 37],
  ['b', 15, 37],

  ['c', 0, 2],
  ['c', 1, 35],
  ['c', 2, 35],
  ['c', 3, 35],
  ['c', 4, 3],
  ['c', 5, 11],
  ['c', 6, 13],
  ['c', 7, 46],
  ['c', 8, 5],
  ['c', 9, 17],
  ['c', 10, 42],
  ['c', 11, 42],
  ['c', 12, 58],
  ['c', 13, 1],
  ['c', 14, 1],
  ['c', 15, 1],

  ['d', 0, 33],
  ['d', 1, 1],
  ['d', 2, 9],
  ['d', 3, 33],
  ['d', 4, 1],
  ['d', 5, 9],
  ['d', 6, 41],
  ['d', 7, 1],
  ['d', 8, 9],
  ['d', 9, 9],
  ['d', 10, 1],
  ['d', 11, 1],
  ['d', 12, 33],
  ['d', 13, 33],
  ['d', 14, 33],
  ['d', 15, 33],

  ['e', 0, 1],
  ['e', 1, 1],
  ['e', 2, 33],
  ['e', 3, 1],
  ['e', 4, 1],
  ['e', 5, 33],
  ['e', 6, 33],
  ['e', 7, 9],
  ['e', 8, 9],
  ['e', 9, 5],
  ['e', 10, 1],
  ['e', 11, 5],
  ['e', 12, 33],
  ['e', 13, 37],
  ['e', 14, 37],
  ['e', 15, 1],

  ['f', 0, 17],
  ['f', 1, 17],
  ['f', 2, 1],
  ['f', 3, 3],
  ['f', 4, 3],
  ['f', 5, 3],
  ['f', 6, 7],
  ['f', 7, 7],
  ['f', 8, 7],
  ['f', 9, 7],
  ['f', 10, 1],
  ['f', 11, 1],
  ['f', 12, 1],
  ['f', 13, 1],
  ['f', 14, 1],
  ['f', 15, 1],

  ['g', 0, 1],
  ['g', 1, 1],
  ['g', 2, 1],
  ['g', 3, 1],
  ['g', 4, 1],
  ['g', 5, 1],
  ['g', 6, 1],
  ['g', 7, 1],
  ['g', 8, 1],
  ['g', 9, 9],
  ['g', 10, 1],
  ['g', 11, 33],
  ['g', 12, 33],
  ['g', 13, 33],
  ['g', 14, 9],
  ['g', 15, 1],

  ['h', 0, 1],
  ['h', 1, 1]
];

pub struct InputPin<T> {
  pub inner: T
}

pub struct OutputPin<T> {
  pub inner: T
}

pub struct AnalogPin<T> {
  pub inner: T
}

pub struct PwmPin<T> {
  pub inner: T
}

pub struct UartPin<T> {
  pub inner: T
}


// Traits =========================================================================================
pub trait ToInOut: Sized {
  // fn pin_mode(self, mode: Mode) -> Self;
  fn input(self) -> InputPin<Self>;
  fn output(self) -> OutputPin<Self>;
}

pub trait ToAnalog: Sized {
  fn analog(self, resolution: u8, eocint: bool) -> AnalogPin<Self>;
}

pub trait ToPwm: Sized {
  fn pwm(self) -> PwmPin<Self>;
}

pub trait ToUart: Sized {
  fn uart(self, baud: u32, rxint: bool, txint: bool) -> UartPin<Self>;
}

pub trait Input: Sized {
  fn bias(&self, bias: Bias);
  fn read(&self) -> bool;
}

pub trait Output: Sized {
  fn speed(&self, speed: Speed);
  fn bias(&self, bias: Bias);
  fn open_drain(&self);
  fn write(&self, value: bool);
}

pub trait Analog: Sized {
  fn analog_read(&self) -> u16;
  fn analog_write(&self, value: u16);
}

pub trait PWM: Sized {
  fn pwm_write(&self, value: u8);
}

pub trait UART: Sized {
  fn rxint_enable(&self);
  fn rxint_disable(&self);
  fn txint_enable(&self);
  fn txint_disable(&self);
  fn change_baud(&self, baud: u32);
  fn send_char(&self, c: char);
  fn send_string(&self, s: &str);
  fn get_char(&self) -> char;
  fn get_string(&self, stopper: char) -> heapless::String<30>;
}


// Functions ======================================================================================
// pub fn read_value<T: Input>(pin: &InputPin<T>) -> bool {
//   pin.read_value()
// }

// pub fn set_bias<T: Input>(pin: &mut InputPin<T>, bias: Bias) {
//   pin.set_bias(bias);
// }

// pub fn into_output<T: Input + Output>(pin: InputPin<T>) -> OutputPin<T> {
//   pin.into_output()
// }

// pub fn set_value<T: Output>(pin: &mut OutputPin<T>, value: bool) {
//   pin.set_value(value);
// }

// pub fn set_speed<T: Output>(pin: &mut OutputPin<T>, speed: Speed) {
//   pin.set_speed(speed);
// }

// pub fn into_input<T: Input + Output>(pin: OutputPin<T>) -> InputPin<T> {
//   pin.into_input()
// }
