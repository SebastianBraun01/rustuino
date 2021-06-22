use super::include::PERIPHERAL_PTR;
use super::common::*;

pub enum Speed {
  Low, Medium, Fast, High
}

pub enum Bias {
  None, Pullup, Pulldown
}


// Converter implementations ======================================================================
macro_rules! generate_ToInOut {
  ($($number: literal),+) => {
    $(
      impl<const B: char, const P: u8> ToInOut for GpioPin<B, P, $number> {
        fn input(self) -> InputPin<Self> {
          self.block = B;
          self.pin = P;
      
          set_input(self.block, self.pin);
      
          return InputPin {
            inner: self
          };
        }
      
        fn output(self) -> OutputPin<Self> {
          self.block = B;
          self.pin = P;
      
          set_output(self.block, self.pin);
      
          return OutputPin {
            inner: self
          };
        }
      }
    )+
  };
}

generate_ToInOut!(1, 3, 5, 7);


// Function implementations =======================================================================
impl<const B: char, const P: u8, const M: u8> Input for InputPin<GpioPin<B, P, M>> {
  fn bias(&self, bias: Bias) {
    let block = B;
    let pin = P;

    set_bias(block, pin, bias);
  }

  fn read(&self) -> bool {
    let block = B;
    let pin = P;

    return digital_read(block, pin);
  }
}

impl<const B: char, const P: u8, const M: u8> Output for OutputPin<GpioPin<B, P, M>> {
  fn speed(&self, speed: Speed) {
    let block = B;
    let pin = P;

    set_speed(block, pin, speed);
  }

  fn bias(&self, bias: Bias) {
    let block = B;
    let pin = P;

    set_bias(block, pin, bias);
  }

  fn open_drain(&self) {
    let block = B;
    let pin = P;

    set_open_drain(block, pin);
  }

  fn write(&self, value: bool) {
    let block = B;
    let pin = P;

    digital_write(block, pin, value);
  }
}


// Helper functions ===============================================================================
fn set_input(block: char, pin: u8) {
  let rcc = &PERIPHERAL_PTR.RCC;

  match block {
    'a' => {
      let gpioa = &PERIPHERAL_PTR.GPIOA;
      rcc.ahb1enr.modify(|_, w| w.gpioaen().enabled());
      gpioa.moder.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin)))});
    },
    'b' => {
      let gpiob = &PERIPHERAL_PTR.GPIOB;
      rcc.ahb1enr.modify(|_, w| w.gpioben().enabled());
      gpiob.moder.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin)))});
    },
    'c' => {
      let gpioc = &PERIPHERAL_PTR.GPIOC;
      rcc.ahb1enr.modify(|_, w| w.gpiocen().enabled());
      gpioc.moder.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin)))});
    },
    'd' => {
      let gpiod = &PERIPHERAL_PTR.GPIOD;
      rcc.ahb1enr.modify(|_, w| w.gpioden().enabled());
      gpiod.moder.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin)))});
    },
    'e' => {
      let gpioe = &PERIPHERAL_PTR.GPIOE;
      rcc.ahb1enr.modify(|_, w| w.gpioeen().enabled());
      gpioe.moder.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin)))});
    },
    'f' => {
      let gpiof = &PERIPHERAL_PTR.GPIOF;
      rcc.ahb1enr.modify(|_, w| w.gpiofen().enabled());
      gpiof.moder.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin)))});
    },
    'g' => {
      let gpiog = &PERIPHERAL_PTR.GPIOG;
      rcc.ahb1enr.modify(|_, w| w.gpiogen().enabled());
      gpiog.moder.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin)))});
    },
    'h' => {
      let gpioh = &PERIPHERAL_PTR.GPIOH;
      rcc.ahb1enr.modify(|_, w| w.gpiohen().enabled());
      gpioh.moder.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin)))});
    },
    _   => panic!("P{}{} is not an available GPIO Pin", block.to_uppercase(), pin)
  };
}

fn set_output(block: char, pin: u8) {
  let rcc = &PERIPHERAL_PTR.RCC;

  match block {
    'a' => {
      let gpioa = &PERIPHERAL_PTR.GPIOA;
      rcc.ahb1enr.modify(|_, w| w.gpioaen().enabled());
      gpioa.moder.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin)) | (1 << (2 * pin)))});
    },
    'b' => {
      let gpiob = &PERIPHERAL_PTR.GPIOB;
      rcc.ahb1enr.modify(|_, w| w.gpioben().enabled());
      gpiob.moder.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin)) | (1 << (2 * pin)))});
    },
    'c' => {
      let gpioc = &PERIPHERAL_PTR.GPIOC;
      rcc.ahb1enr.modify(|_, w| w.gpiocen().enabled());
      gpioc.moder.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin)) | (1 << (2 * pin)))});
    },
    'd' => {
      let gpiod = &PERIPHERAL_PTR.GPIOD;
      rcc.ahb1enr.modify(|_, w| w.gpioden().enabled());
      gpiod.moder.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin)) | (1 << (2 * pin)))});
    },
    'e' => {
      let gpioe = &PERIPHERAL_PTR.GPIOE;
      rcc.ahb1enr.modify(|_, w| w.gpioeen().enabled());
      gpioe.moder.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin)) | (1 << (2 * pin)))});
    },
    'f' => {
      let gpiof = &PERIPHERAL_PTR.GPIOF;
      rcc.ahb1enr.modify(|_, w| w.gpiofen().enabled());
      gpiof.moder.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin)) | (1 << (2 * pin)))});
    },
    'g' => {
      let gpiog = &PERIPHERAL_PTR.GPIOG;
      rcc.ahb1enr.modify(|_, w| w.gpiogen().enabled());
      gpiog.moder.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin)) | (1 << (2 * pin)))});
    },
    'h' => {
      let gpioh = &PERIPHERAL_PTR.GPIOH;
      rcc.ahb1enr.modify(|_, w| w.gpiohen().enabled());
      gpioh.moder.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin)) | (1 << (2 * pin)))});
    },
    _   => panic!("P{}{} is not an available GPIO Pin", block.to_uppercase(), pin)
  };
}

fn set_bias(block: char, pin: u8, bias: Bias) {
  match block {
    'a' => {
      let gpioa = &PERIPHERAL_PTR.GPIOA;
      match bias {
        Bias::None => gpioa.pupdr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin)))}),
        Bias::Pullup => gpioa.pupdr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin)) | (1 << (2 * pin)))}),
        Bias::Pulldown => gpioa.pupdr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin)) | (2 << (2 * pin)))})
      };
    },
    'b' => {
      let gpiob = &PERIPHERAL_PTR.GPIOB;
      match bias {
        Bias::None => gpiob.pupdr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin)))}),
        Bias::Pullup => gpiob.pupdr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin)) | (1 << (2 * pin)))}),
        Bias::Pulldown => gpiob.pupdr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin)) | (2 << (2 * pin)))})
      };
    },
    'c' => {
      let gpioc = &PERIPHERAL_PTR.GPIOC;
      match bias {
        Bias::None => gpioc.pupdr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin)))}),
        Bias::Pullup => gpioc.pupdr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin)) | (1 << (2 * pin)))}),
        Bias::Pulldown => gpioc.pupdr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin)) | (2 << (2 * pin)))})
      };
    },
    'd' => {
      let gpiod = &PERIPHERAL_PTR.GPIOD;
      match bias {
        Bias::None => gpiod.pupdr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin)))}),
        Bias::Pullup => gpiod.pupdr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin)) | (1 << (2 * pin)))}),
        Bias::Pulldown => gpiod.pupdr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin)) | (2 << (2 * pin)))})
      };
    },
    'e' => {
      let gpioe = &PERIPHERAL_PTR.GPIOE;
      match bias {
        Bias::None => gpioe.pupdr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin)))}),
        Bias::Pullup => gpioe.pupdr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin)) | (1 << (2 * pin)))}),
        Bias::Pulldown => gpioe.pupdr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin)) | (2 << (2 * pin)))})
      };
    },
    'f' => {
      let gpiof = &PERIPHERAL_PTR.GPIOF;
      match bias {
        Bias::None => gpiof.pupdr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin)))}),
        Bias::Pullup => gpiof.pupdr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin)) | (1 << (2 * pin)))}),
        Bias::Pulldown => gpiof.pupdr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin)) | (2 << (2 * pin)))})
      };
    },
    'g' => {
      let gpiog = &PERIPHERAL_PTR.GPIOG;
      match bias {
        Bias::None => gpiog.pupdr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin)))}),
        Bias::Pullup => gpiog.pupdr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin)) | (1 << (2 * pin)))}),
        Bias::Pulldown => gpiog.pupdr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin)) | (2 << (2 * pin)))})
      };
    },
    'h' => {
      let gpioh = &PERIPHERAL_PTR.GPIOH;
      match bias {
        Bias::None => gpioh.pupdr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin)))}),
        Bias::Pullup => gpioh.pupdr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin)) | (1 << (2 * pin)))}),
        Bias::Pulldown => gpioh.pupdr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin)) | (2 << (2 * pin)))})
      };
    },
    _   => panic!("P{}{} is not an available GPIO Pin", block.to_uppercase(), pin)
  };
}

fn digital_read(block: char, pin: u8) -> bool {
  let bits = match block {
    'a' => {
      let gpioa = &PERIPHERAL_PTR.GPIOA;
      gpioa.idr.read().bits()
    },
    'b' => {
      let gpiob = &PERIPHERAL_PTR.GPIOB;
      gpiob.idr.read().bits()
    },
    'c' => {
      let gpioc = &PERIPHERAL_PTR.GPIOC;
      gpioc.idr.read().bits()
    },
    'd' => {
      let gpiod = &PERIPHERAL_PTR.GPIOD;
      gpiod.idr.read().bits()
    },
    'e' => {
      let gpioe = &PERIPHERAL_PTR.GPIOE;
      gpioe.idr.read().bits()
    },
    'f' => {
      let gpiof = &PERIPHERAL_PTR.GPIOF;
      gpiof.idr.read().bits()
    },
    'g' => {
      let gpiog = &PERIPHERAL_PTR.GPIOG;
      gpiog.idr.read().bits()
    },
    'h' => {
      let gpioh = &PERIPHERAL_PTR.GPIOH;
      gpioh.idr.read().bits()
    },
    _   => panic!("P{}{} is not an available GPIO Pin", block.to_uppercase(), pin)
  };

  if bits & (1 << pin) == (1 << pin) {return true;}
  else {return false;}
}

fn set_speed(block: char, pin: u8, speed: Speed) {
  match block {
    'a' => {
      let gpioa = &PERIPHERAL_PTR.GPIOA;
      match speed {
        Speed::Low => gpioa.ospeedr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin)))}),
        Speed::Medium => gpioa.ospeedr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin)) | (1 << (2 * pin)))}),
        Speed::Fast => gpioa.ospeedr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin)) | (2 << (2 * pin)))}),
        Speed::High => gpioa.ospeedr.modify(|r, w| unsafe {w.bits(r.bits() | (3 << (2 * pin)))})
      };
    },
    'b' => {
      let gpiob = &PERIPHERAL_PTR.GPIOB;
      match speed {
        Speed::Low => gpiob.ospeedr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin)))}),
        Speed::Medium => gpiob.ospeedr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin)) | (1 << (2 * pin)))}),
        Speed::Fast => gpiob.ospeedr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin)) | (2 << (2 * pin)))}),
        Speed::High => gpiob.ospeedr.modify(|r, w| unsafe {w.bits(r.bits() | (3 << (2 * pin)))})
      };
    },
    'c' => {
      let gpioc = &PERIPHERAL_PTR.GPIOC;
      match speed {
        Speed::Low => gpioc.ospeedr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin)))}),
        Speed::Medium => gpioc.ospeedr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin)) | (1 << (2 * pin)))}),
        Speed::Fast => gpioc.ospeedr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin)) | (2 << (2 * pin)))}),
        Speed::High => gpioc.ospeedr.modify(|r, w| unsafe {w.bits(r.bits() | (3 << (2 * pin)))})
      };
    },
    'd' => {
      let gpiod = &PERIPHERAL_PTR.GPIOD;
      match speed {
        Speed::Low => gpiod.ospeedr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin)))}),
        Speed::Medium => gpiod.ospeedr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin)) | (1 << (2 * pin)))}),
        Speed::Fast => gpiod.ospeedr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin)) | (2 << (2 * pin)))}),
        Speed::High => gpiod.ospeedr.modify(|r, w| unsafe {w.bits(r.bits() | (3 << (2 * pin)))})
      };
    },
    'e' => {
      let gpioe = &PERIPHERAL_PTR.GPIOE;
      match speed {
        Speed::Low => gpioe.ospeedr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin)))}),
        Speed::Medium => gpioe.ospeedr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin)) | (1 << (2 * pin)))}),
        Speed::Fast => gpioe.ospeedr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin)) | (2 << (2 * pin)))}),
        Speed::High => gpioe.ospeedr.modify(|r, w| unsafe {w.bits(r.bits() | (3 << (2 * pin)))})
      };
    },
    'f' => {
      let gpiof = &PERIPHERAL_PTR.GPIOF;
      match speed {
        Speed::Low => gpiof.ospeedr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin)))}),
        Speed::Medium => gpiof.ospeedr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin)) | (1 << (2 * pin)))}),
        Speed::Fast => gpiof.ospeedr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin)) | (2 << (2 * pin)))}),
        Speed::High => gpiof.ospeedr.modify(|r, w| unsafe {w.bits(r.bits() | (3 << (2 * pin)))})
      };
    },
    'g' => {
      let gpiog = &PERIPHERAL_PTR.GPIOG;
      match speed {
        Speed::Low => gpiog.ospeedr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin)))}),
        Speed::Medium => gpiog.ospeedr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin)) | (1 << (2 * pin)))}),
        Speed::Fast => gpiog.ospeedr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin)) | (2 << (2 * pin)))}),
        Speed::High => gpiog.ospeedr.modify(|r, w| unsafe {w.bits(r.bits() | (3 << (2 * pin)))})
      };
    },
    'h' => {
      let gpioh = &PERIPHERAL_PTR.GPIOH;
      match speed {
        Speed::Low => gpioh.ospeedr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin)))}),
        Speed::Medium => gpioh.ospeedr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin)) | (1 << (2 * pin)))}),
        Speed::Fast => gpioh.ospeedr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin)) | (2 << (2 * pin)))}),
        Speed::High => gpioh.ospeedr.modify(|r, w| unsafe {w.bits(r.bits() | (3 << (2 * pin)))})
      };
    },
    _   => panic!("P{}{} is not an available GPIO Pin", block.to_uppercase(), pin)
  };
}

fn set_open_drain(block: char, pin: u8) {
  match block {
    'a' => {
      let gpioa = &PERIPHERAL_PTR.GPIOA;
      gpioa.otyper.modify(|r, w| unsafe {w.bits(r.bits() | (1 << pin))});
    },
    'b' => {
      let gpiob = &PERIPHERAL_PTR.GPIOB;
      gpiob.otyper.modify(|r, w| unsafe {w.bits(r.bits() | (1 << pin))});
    },
    'c' => {
      let gpioc = &PERIPHERAL_PTR.GPIOC;
      gpioc.otyper.modify(|r, w| unsafe {w.bits(r.bits() | (1 << pin))});
    },
    'd' => {
      let gpiod = &PERIPHERAL_PTR.GPIOD;
      gpiod.otyper.modify(|r, w| unsafe {w.bits(r.bits() | (1 << pin))});
    },
    'e' => {
      let gpioe = &PERIPHERAL_PTR.GPIOE;
      gpioe.otyper.modify(|r, w| unsafe {w.bits(r.bits() | (1 << pin))});
    },
    'f' => {
      let gpiof = &PERIPHERAL_PTR.GPIOF;
      gpiof.otyper.modify(|r, w| unsafe {w.bits(r.bits() | (1 << pin))});
    },
    'g' => {
      let gpiog = &PERIPHERAL_PTR.GPIOG;
      gpiog.otyper.modify(|r, w| unsafe {w.bits(r.bits() | (1 << pin))});
    },
    'h' => {
      let gpioh = &PERIPHERAL_PTR.GPIOH;
      gpioh.otyper.modify(|r, w| unsafe {w.bits(r.bits() | (1 << pin))});
    },
    _   => panic!("P{}{} is not an available GPIO Pin", block.to_uppercase(), pin)
  };
}

fn digital_write(block: char, pin: u8, value: bool) {
  match block {
    'a' => {
      let gpioa = &PERIPHERAL_PTR.GPIOA;
      if value == true {gpioa.bsrr.write(|w| unsafe {w.bits(1 << pin)});}
      else {gpioa.bsrr.write(|w| unsafe {w.bits(1 << (pin + 16))});}
    },
    'b' => {
      let gpiob = &PERIPHERAL_PTR.GPIOB;
      if value == true {gpiob.bsrr.write(|w| unsafe {w.bits(1 << pin)});}
      else {gpiob.bsrr.write(|w| unsafe {w.bits(1 << (pin + 16))});}
    },
    'c' => {
      let gpioc = &PERIPHERAL_PTR.GPIOC;
      if value == true {gpioc.bsrr.write(|w| unsafe {w.bits(1 << pin)});}
      else {gpioc.bsrr.write(|w| unsafe {w.bits(1 << (pin + 16))});}
    },
    'd' => {
      let gpiod = &PERIPHERAL_PTR.GPIOD;
      if value == true {gpiod.bsrr.write(|w| unsafe {w.bits(1 << pin)});}
      else {gpiod.bsrr.write(|w| unsafe {w.bits(1 << (pin + 16))});}
    },
    'e' => {
      let gpioe = &PERIPHERAL_PTR.GPIOE;
      if value == true {gpioe.bsrr.write(|w| unsafe {w.bits(1 << pin)});}
      else {gpioe.bsrr.write(|w| unsafe {w.bits(1 << (pin + 16))});}
    },
    'f' => {
      let gpiof = &PERIPHERAL_PTR.GPIOF;
      if value == true {gpiof.bsrr.write(|w| unsafe {w.bits(1 << pin)});}
      else {gpiof.bsrr.write(|w| unsafe {w.bits(1 << (pin + 16))});}
    },
    'g' => {
      let gpiog = &PERIPHERAL_PTR.GPIOG;
      if value == true {gpiog.bsrr.write(|w| unsafe {w.bits(1 << pin)});}
      else {gpiog.bsrr.write(|w| unsafe {w.bits(1 << (pin + 16))});}
    },
    'h' => {
      let gpioh = &PERIPHERAL_PTR.GPIOH;
      if value == true {gpioh.bsrr.write(|w| unsafe {w.bits(1 << pin)});}
      else {gpioh.bsrr.write(|w| unsafe {w.bits(1 << (pin + 16))});}
    },
    _   => panic!("P{}{} is not an available GPIO Pin", block.to_uppercase(), pin)
  };
}
