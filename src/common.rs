use heapless::String;
use super::gpio::{Bias, Speed};


// struct Number<const N: u8> {}

// trait Test {fn test(self) -> bool;}

// impl<const N: u8> Number<N> {
//   fn get() -> Number<N> {
//     return Number {};
//   }
// }

// impl Test for Number<2> {
//   fn test(self) -> bool {return false;}
// }

// type Number1 = Number<1>;
// type Number2 = Number<2>;

// pub fn call() {
//   let one = Number1::get();
//   let two = Number2::get();

//   // let get_one = one.test();
//   let get_two = two.test();
// }


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
  ($([$block:ident; $block_char:literal; $pin:literal; $func:literal]),+) => {
    use paste;

    paste!{
      $(
        pub type [<P $block:upper $pin>] = GpioPin<$block_char, $pin, $func>;
      )+
    }
  };
}

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
  fn get_string(&self, stopper: char) -> String<30>;
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
