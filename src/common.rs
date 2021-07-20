use super::gpio::{Bias, Speed};


// Structs ========================================================================================
macro_rules! generate_pins {
  ($([$block:literal, $pin:literal]),+) => {
    use paste::paste;

    paste!{
      $(
      pub struct [<P $block:upper $pin>];
      )+
    }
  };
}

generate_pins![
  ['a', 0],
  ['a', 1],
  ['a', 2],
  ['a', 3],
  ['a', 4],
  ['a', 5],
  ['a', 6],
  ['a', 7],
  ['a', 8],
  ['a', 9],
  ['a', 10],
  ['a', 11],
  ['a', 12],
  ['a', 13],
  ['a', 14],
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
  ['b', 12],
  ['b', 13],
  ['b', 14],
  ['b', 15],

  ['c', 0],
  ['c', 1],
  ['c', 2],
  ['c', 3],
  ['c', 4],
  ['c', 5],
  ['c', 6],
  ['c', 7],
  ['c', 8],
  ['c', 9],
  ['c', 10],
  ['c', 11],
  ['c', 12],
  ['c', 13],
  ['c', 14],
  ['c', 15],

  ['d', 0],
  ['d', 1],
  ['d', 2],
  ['d', 3],
  ['d', 4],
  ['d', 5],
  ['d', 6],
  ['d', 7],
  ['d', 8],
  ['d', 9],
  ['d', 10],
  ['d', 11],
  ['d', 12],
  ['d', 13],
  ['d', 14],
  ['d', 15],

  ['e', 0],
  ['e', 1],
  ['e', 2],
  ['e', 3],
  ['e', 4],
  ['e', 5],
  ['e', 6],
  ['e', 7],
  ['e', 8],
  ['e', 9],
  ['e', 10],
  ['e', 11],
  ['e', 12],
  ['e', 13],
  ['e', 14],
  ['e', 15],

  ['f', 0],
  ['f', 1],
  ['f', 2],
  ['f', 3],
  ['f', 4],
  ['f', 5],
  ['f', 6],
  ['f', 7],
  ['f', 8],
  ['f', 9],
  ['f', 10],
  ['f', 11],
  ['f', 12],
  ['f', 13],
  ['f', 14],
  ['f', 15],

  ['g', 0],
  ['g', 1],
  ['g', 2],
  ['g', 3],
  ['g', 4],
  ['g', 5],
  ['g', 6],
  ['g', 7],
  ['g', 8],
  ['g', 9],
  ['g', 10],
  ['g', 11],
  ['g', 12],
  ['g', 13],
  ['g', 14],
  ['g', 15],

  ['h', 0],
  ['h', 1]
];


pub struct InputPin {
  pub block: char,
  pub pin: u8
}

pub struct OutputPin {
  pub block: char,
  pub pin: u8
}

pub struct AnalogPin {
  pub block: char,
  pub pin: u8
}

pub struct PwmPin {
  pub block: char,
  pub pin: u8
}

pub struct UartPin {
  pub block: char,
  pub pin: u8
}

pub struct I2cPin {
  pub block: char,
  pub pin: u8
}


// Traits =========================================================================================
pub trait ToInOut: Sized {
  // fn pin_mode(self, mode: Mode) -> Self;
  fn input() -> InputPin;
  fn output() -> OutputPin;
}

pub trait ToAnalog: Sized {
  fn analog(resolution: u8, eocint: bool) -> AnalogPin;
}

pub trait ToPwm: Sized {
  fn pwm() -> PwmPin;
}

pub trait ToUart: Sized {
  fn uart(baud: u32, rxint: bool, txint: bool) -> UartPin;
}

pub trait ToI2c: Sized {
  fn i2c() -> I2cPin;
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

pub trait I2C: Sized {
  fn send_byte(&self, byte: u8);
  fn recieve_byte(&self) -> u8;
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
