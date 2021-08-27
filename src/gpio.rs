use crate::include::pins::*;

/// This struct holds the configuration of a pin that has been configured as an input.
/// 
/// To configure a pin as an input, call the input function on the appropriate pin label. The function returns the pin struct with the settings of the pin.
/// # Example
/// ```rust,no_run
/// #![no_std]
/// #![no_main]
///
/// use rustuino::*;
/// 
/// #[entry]
/// fn main() -> ! {
///   let pin1 = PA0::input();
///   let pin2 = PA1::output();
/// 
///   loop {
///     if pin1.read() == true {
///       pin2.write(true);
///     }
///     else {
///       pin2.write(false);
///     }
///     delay(1000);
///   }
/// }
/// ```
pub struct InputPin {
  block: char,
  pin: u8,
  bias: Bias
}

/// Struct that holds the configuration of a pin that has been configured as an output.
///
/// To configure a pin as an output, call the output function on the appropriate pin label. The function returns the pin struct with the settings of the pin.
/// # Example
/// ```rust,no_run
/// #![no_std]
/// #![no_main]
///
/// use rustuino::*;
/// 
/// #[entry]
/// fn main() -> ! {
///   let pin1 = PA0::input();
///   let pin2 = PA1::output();
/// 
///   loop {
///     if pin1.read() == true {
///       pin2.write(true);
///     }
///     else {
///       pin2.write(false);
///     }
///     delay(1000);
///   }
/// }
/// ```
pub struct OutputPin {
  block: char,
  pin: u8,
  bias: Bias,
  speed: Speed,
  open_drain: bool
}

/// Represents the options to configure the GPIO speed of a pin.
pub enum Speed {
  Low, Medium, Fast, High
}

/// Represents the options to configure the bias of a pin.
pub enum Bias {
  None, Pullup, Pulldown
}

/// This trait is implemented on all pin structs that are able to be used as a digital IO pin (all of them).
pub trait ToDigital: Sized {
  const BLOCK: char;
  const PIN: u8;

  /// Configures a pin as a digital input and gives back the associated pin struct.
  /// # Examples
  /// ```rust,no_run
  /// use rustuino::*;
  ///
  /// // Configure pin as input
  /// let pin = PA0::input();
  /// 
  /// // Read the digital value from the pin
  /// let state: bool = pin.read();
  /// ```
  fn input() -> InputPin {
    let peripheral_ptr;
    unsafe {peripheral_ptr = stm32f4::stm32f446::Peripherals::steal();}
    let rcc = &peripheral_ptr.RCC;

    let block = Self::BLOCK;
    let pin = Self::PIN;

    match block {
      'a' => {
        let gpioa = &peripheral_ptr.GPIOA;
        rcc.ahb1enr.modify(|_, w| w.gpioaen().enabled());
        gpioa.moder.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin)))});
      },
      'b' => {
        let gpiob = &peripheral_ptr.GPIOB;
        rcc.ahb1enr.modify(|_, w| w.gpioben().enabled());
        gpiob.moder.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin)))});
      },
      'c' => {
        let gpioc = &peripheral_ptr.GPIOC;
        rcc.ahb1enr.modify(|_, w| w.gpiocen().enabled());
        gpioc.moder.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin)))});
      },
      'd' => {
        let gpiod = &peripheral_ptr.GPIOD;
        rcc.ahb1enr.modify(|_, w| w.gpioden().enabled());
        gpiod.moder.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin)))});
      },
      'e' => {
        let gpioe = &peripheral_ptr.GPIOE;
        rcc.ahb1enr.modify(|_, w| w.gpioeen().enabled());
        gpioe.moder.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin)))});
      },
      'f' => {
        let gpiof = &peripheral_ptr.GPIOF;
        rcc.ahb1enr.modify(|_, w| w.gpiofen().enabled());
        gpiof.moder.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin)))});
      },
      'g' => {
        let gpiog = &peripheral_ptr.GPIOG;
        rcc.ahb1enr.modify(|_, w| w.gpiogen().enabled());
        gpiog.moder.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin)))});
      },
      'h' => {
        let gpioh = &peripheral_ptr.GPIOH;
        rcc.ahb1enr.modify(|_, w| w.gpiohen().enabled());
        gpioh.moder.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin)))});
      },
      _   => panic!("P{}{} is not an available GPIO Pin | ::input()", block.to_uppercase(), pin)
    };

    return InputPin {
      block,
      pin,
      bias: Bias::None
    };
  }
  
  /// Configures a pin as a digital output and gives back the associated pin struct.
  /// # Examples
  /// ```rust,no_run
  /// use rustuino::*;
  ///
  /// // Configure pin as output
  /// let pin = PA0::output();
  /// 
  /// // Set the value of the pin
  /// pin.write(true);
  /// pin.write(false);
  /// ```
  fn output() -> OutputPin {
    let peripheral_ptr;
    unsafe {peripheral_ptr = stm32f4::stm32f446::Peripherals::steal();}
    let rcc = &peripheral_ptr.RCC;

    let block = Self::BLOCK;
    let pin = Self::PIN;

    match block {
      'a' => {
        let gpioa = &peripheral_ptr.GPIOA;
        rcc.ahb1enr.modify(|_, w| w.gpioaen().enabled());
        gpioa.moder.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin)) | (1 << (2 * pin)))});
      },
      'b' => {
        let gpiob = &peripheral_ptr.GPIOB;
        rcc.ahb1enr.modify(|_, w| w.gpioben().enabled());
        gpiob.moder.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin)) | (1 << (2 * pin)))});
      },
      'c' => {
        let gpioc = &peripheral_ptr.GPIOC;
        rcc.ahb1enr.modify(|_, w| w.gpiocen().enabled());
        gpioc.moder.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin)) | (1 << (2 * pin)))});
      },
      'd' => {
        let gpiod = &peripheral_ptr.GPIOD;
        rcc.ahb1enr.modify(|_, w| w.gpioden().enabled());
        gpiod.moder.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin)) | (1 << (2 * pin)))});
      },
      'e' => {
        let gpioe = &peripheral_ptr.GPIOE;
        rcc.ahb1enr.modify(|_, w| w.gpioeen().enabled());
        gpioe.moder.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin)) | (1 << (2 * pin)))});
      },
      'f' => {
        let gpiof = &peripheral_ptr.GPIOF;
        rcc.ahb1enr.modify(|_, w| w.gpiofen().enabled());
        gpiof.moder.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin)) | (1 << (2 * pin)))});
      },
      'g' => {
        let gpiog = &peripheral_ptr.GPIOG;
        rcc.ahb1enr.modify(|_, w| w.gpiogen().enabled());
        gpiog.moder.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin)) | (1 << (2 * pin)))});
      },
      'h' => {
        let gpioh = &peripheral_ptr.GPIOH;
        rcc.ahb1enr.modify(|_, w| w.gpiohen().enabled());
        gpioh.moder.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin)) | (1 << (2 * pin)))});
      },
      _   => panic!("P{}{} is not an available GPIO Pin | ::output()", block.to_uppercase(), pin)
    };

    return OutputPin {
      block,
      pin,
      bias: Bias::None,
      speed: Speed::Low,
      open_drain: false
    };
  }
}

macro_rules! generate_ToInOut {
  ($([$letter:literal, $number: literal]),+) => {
    use paste::paste;

    paste!{
      $(
        impl ToDigital for [<P $letter:upper $number>] {        
          const BLOCK: char = $letter;
          const PIN: u8 = $number;
        }
      )+
    }
  };
}

generate_ToInOut![
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


// Function implementations =======================================================================
impl InputPin {
  /// Sets the internal pullup/pulldown resistor of the pin.
  /// # Examples
  /// ```rust,no_run
  /// use rustuino::*;
  ///
  /// // Configure pin as input
  /// let mut pin = PA0::input();
  /// 
  /// // Set the bias of the pin
  /// pin.bias(Bias::None);
  /// pin.bias(Bias::Pullup);
  /// pin.bias(Bias::Pulldown);
  /// ```
  pub fn bias(&mut self, bias: Bias) {
    let peripheral_ptr;
    unsafe {peripheral_ptr = stm32f4::stm32f446::Peripherals::steal();}
    let pin = self.pin;

    match self.block {
      'a' => {
        let gpioa = &peripheral_ptr.GPIOA;
        match bias {
          Bias::None => gpioa.pupdr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin)))}),
          Bias::Pullup => gpioa.pupdr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin)) | (1 << (2 * pin)))}),
          Bias::Pulldown => gpioa.pupdr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin)) | (2 << (2 * pin)))})
        };
      },
      'b' => {
        let gpiob = &peripheral_ptr.GPIOB;
        match bias {
          Bias::None => gpiob.pupdr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin)))}),
          Bias::Pullup => gpiob.pupdr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin)) | (1 << (2 * pin)))}),
          Bias::Pulldown => gpiob.pupdr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin)) | (2 << (2 * pin)))})
        };
      },
      'c' => {
        let gpioc = &peripheral_ptr.GPIOC;
        match bias {
          Bias::None => gpioc.pupdr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin)))}),
          Bias::Pullup => gpioc.pupdr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin)) | (1 << (2 * pin)))}),
          Bias::Pulldown => gpioc.pupdr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin)) | (2 << (2 * pin)))})
        };
      },
      'd' => {
        let gpiod = &peripheral_ptr.GPIOD;
        match bias {
          Bias::None => gpiod.pupdr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin)))}),
          Bias::Pullup => gpiod.pupdr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin)) | (1 << (2 * pin)))}),
          Bias::Pulldown => gpiod.pupdr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin)) | (2 << (2 * pin)))})
        };
      },
      'e' => {
        let gpioe = &peripheral_ptr.GPIOE;
        match bias {
          Bias::None => gpioe.pupdr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin)))}),
          Bias::Pullup => gpioe.pupdr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin)) | (1 << (2 * pin)))}),
          Bias::Pulldown => gpioe.pupdr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin)) | (2 << (2 * pin)))})
        };
      },
      'f' => {
        let gpiof = &peripheral_ptr.GPIOF;
        match bias {
          Bias::None => gpiof.pupdr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin)))}),
          Bias::Pullup => gpiof.pupdr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin)) | (1 << (2 * pin)))}),
          Bias::Pulldown => gpiof.pupdr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin)) | (2 << (2 * pin)))})
        };
      },
      'g' => {
        let gpiog = &peripheral_ptr.GPIOG;
        match bias {
          Bias::None => gpiog.pupdr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin)))}),
          Bias::Pullup => gpiog.pupdr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin)) | (1 << (2 * pin)))}),
          Bias::Pulldown => gpiog.pupdr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin)) | (2 << (2 * pin)))})
        };
      },
      'h' => {
        let gpioh = &peripheral_ptr.GPIOH;
        match bias {
          Bias::None => gpioh.pupdr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin)))}),
          Bias::Pullup => gpioh.pupdr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin)) | (1 << (2 * pin)))}),
          Bias::Pulldown => gpioh.pupdr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin)) | (2 << (2 * pin)))})
        };
      },
      _   => panic!("P{}{} is not an available GPIO Pin | .bias(...)", self.block.to_uppercase(), pin)
    };

    self.bias = bias;
  }

  /// Reads the input value of the pin.
  /// # Examples
  /// ```rust,no_run
  /// use rustuino::*;
  ///
  /// // Configure pin as input
  /// let pin = PA0::input();
  /// 
  /// // Read the digital value from the pin
  /// let state: bool = pin.read();
  /// ```
  pub fn read(&self) -> bool {
    let peripheral_ptr;
    unsafe {peripheral_ptr = stm32f4::stm32f446::Peripherals::steal();}

    let bits = match self.block {
      'a' => {
        let gpioa = &peripheral_ptr.GPIOA;
        gpioa.idr.read().bits()
      },
      'b' => {
        let gpiob = &peripheral_ptr.GPIOB;
        gpiob.idr.read().bits()
      },
      'c' => {
        let gpioc = &peripheral_ptr.GPIOC;
        gpioc.idr.read().bits()
      },
      'd' => {
        let gpiod = &peripheral_ptr.GPIOD;
        gpiod.idr.read().bits()
      },
      'e' => {
        let gpioe = &peripheral_ptr.GPIOE;
        gpioe.idr.read().bits()
      },
      'f' => {
        let gpiof = &peripheral_ptr.GPIOF;
        gpiof.idr.read().bits()
      },
      'g' => {
        let gpiog = &peripheral_ptr.GPIOG;
        gpiog.idr.read().bits()
      },
      'h' => {
        let gpioh = &peripheral_ptr.GPIOH;
        gpioh.idr.read().bits()
      },
      _   => panic!("P{}{} is not an available GPIO Pin | .read()", self.block.to_uppercase(), self.pin)
    };
  
    if bits & (1 << self.pin) == (1 << self.pin) {return true;}
    else {return false;}
  }
}

impl OutputPin {
  /// Sets the max speed of the GPIO pin.
  /// # Examples
  /// ```rust,no_run
  /// use rustuino::*;
  ///
  /// // Configure pin as output
  /// let mut pin = PA0::output();
  /// 
  /// // Set the speed
  /// pin.speed(Speed::Low);
  /// pin.speed(Speed::Medium);
  /// pin.speed(Speed::Fast);
  /// pin.speed(Speed::High);
  /// ```
  pub fn speed(&mut self, speed: Speed) {
    let peripheral_ptr;
    unsafe {peripheral_ptr = stm32f4::stm32f446::Peripherals::steal();}
    let pin = self.pin;

    match self.block {
      'a' => {
        let gpioa = &peripheral_ptr.GPIOA;
        match speed {
          Speed::Low => gpioa.ospeedr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin)))}),
          Speed::Medium => gpioa.ospeedr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin)) | (1 << (2 * pin)))}),
          Speed::Fast => gpioa.ospeedr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin)) | (2 << (2 * pin)))}),
          Speed::High => gpioa.ospeedr.modify(|r, w| unsafe {w.bits(r.bits() | (3 << (2 * pin)))})
        };
      },
      'b' => {
        let gpiob = &peripheral_ptr.GPIOB;
        match speed {
          Speed::Low => gpiob.ospeedr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin)))}),
          Speed::Medium => gpiob.ospeedr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin)) | (1 << (2 * pin)))}),
          Speed::Fast => gpiob.ospeedr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin)) | (2 << (2 * pin)))}),
          Speed::High => gpiob.ospeedr.modify(|r, w| unsafe {w.bits(r.bits() | (3 << (2 * pin)))})
        };
      },
      'c' => {
        let gpioc = &peripheral_ptr.GPIOC;
        match speed {
          Speed::Low => gpioc.ospeedr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin)))}),
          Speed::Medium => gpioc.ospeedr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin)) | (1 << (2 * pin)))}),
          Speed::Fast => gpioc.ospeedr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin)) | (2 << (2 * pin)))}),
          Speed::High => gpioc.ospeedr.modify(|r, w| unsafe {w.bits(r.bits() | (3 << (2 * pin)))})
        };
      },
      'd' => {
        let gpiod = &peripheral_ptr.GPIOD;
        match speed {
          Speed::Low => gpiod.ospeedr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin)))}),
          Speed::Medium => gpiod.ospeedr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin)) | (1 << (2 * pin)))}),
          Speed::Fast => gpiod.ospeedr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin)) | (2 << (2 * pin)))}),
          Speed::High => gpiod.ospeedr.modify(|r, w| unsafe {w.bits(r.bits() | (3 << (2 * pin)))})
        };
      },
      'e' => {
        let gpioe = &peripheral_ptr.GPIOE;
        match speed {
          Speed::Low => gpioe.ospeedr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin)))}),
          Speed::Medium => gpioe.ospeedr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin)) | (1 << (2 * pin)))}),
          Speed::Fast => gpioe.ospeedr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin)) | (2 << (2 * pin)))}),
          Speed::High => gpioe.ospeedr.modify(|r, w| unsafe {w.bits(r.bits() | (3 << (2 * pin)))})
        };
      },
      'f' => {
        let gpiof = &peripheral_ptr.GPIOF;
        match speed {
          Speed::Low => gpiof.ospeedr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin)))}),
          Speed::Medium => gpiof.ospeedr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin)) | (1 << (2 * pin)))}),
          Speed::Fast => gpiof.ospeedr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin)) | (2 << (2 * pin)))}),
          Speed::High => gpiof.ospeedr.modify(|r, w| unsafe {w.bits(r.bits() | (3 << (2 * pin)))})
        };
      },
      'g' => {
        let gpiog = &peripheral_ptr.GPIOG;
        match speed {
          Speed::Low => gpiog.ospeedr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin)))}),
          Speed::Medium => gpiog.ospeedr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin)) | (1 << (2 * pin)))}),
          Speed::Fast => gpiog.ospeedr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin)) | (2 << (2 * pin)))}),
          Speed::High => gpiog.ospeedr.modify(|r, w| unsafe {w.bits(r.bits() | (3 << (2 * pin)))})
        };
      },
      'h' => {
        let gpioh = &peripheral_ptr.GPIOH;
        match speed {
          Speed::Low => gpioh.ospeedr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin)))}),
          Speed::Medium => gpioh.ospeedr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin)) | (1 << (2 * pin)))}),
          Speed::Fast => gpioh.ospeedr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin)) | (2 << (2 * pin)))}),
          Speed::High => gpioh.ospeedr.modify(|r, w| unsafe {w.bits(r.bits() | (3 << (2 * pin)))})
        };
      },
      _   => panic!("P{}{} is not an available GPIO Pin | .speed(...)", self.block.to_uppercase(), pin)
    };

    self.speed = speed;
  }

  /// Sets the internal pullup/pulldown resistor of the pin.
  /// # Examples
  /// ```rust,no_run
  /// use rustuino::*;
  ///
  /// // Configure pin as output
  /// let mut pin = PA0::output();
  /// 
  /// // Set the bias of the pin
  /// pin.bias(Bias::None);
  /// pin.bias(Bias::Pullup);
  /// pin.bias(Bias::Pulldown);
  /// ```
  pub fn bias(&mut self, bias: Bias) {
    let peripheral_ptr;
    unsafe {peripheral_ptr = stm32f4::stm32f446::Peripherals::steal();}
    let pin = self.pin;

    match self.block {
      'a' => {
        let gpioa = &peripheral_ptr.GPIOA;
        match bias {
          Bias::None => gpioa.pupdr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin)))}),
          Bias::Pullup => gpioa.pupdr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin)) | (1 << (2 * pin)))}),
          Bias::Pulldown => gpioa.pupdr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin)) | (2 << (2 * pin)))})
        };
      },
      'b' => {
        let gpiob = &peripheral_ptr.GPIOB;
        match bias {
          Bias::None => gpiob.pupdr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin)))}),
          Bias::Pullup => gpiob.pupdr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin)) | (1 << (2 * pin)))}),
          Bias::Pulldown => gpiob.pupdr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin)) | (2 << (2 * pin)))})
        };
      },
      'c' => {
        let gpioc = &peripheral_ptr.GPIOC;
        match bias {
          Bias::None => gpioc.pupdr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin)))}),
          Bias::Pullup => gpioc.pupdr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin)) | (1 << (2 * pin)))}),
          Bias::Pulldown => gpioc.pupdr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin)) | (2 << (2 * pin)))})
        };
      },
      'd' => {
        let gpiod = &peripheral_ptr.GPIOD;
        match bias {
          Bias::None => gpiod.pupdr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin)))}),
          Bias::Pullup => gpiod.pupdr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin)) | (1 << (2 * pin)))}),
          Bias::Pulldown => gpiod.pupdr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin)) | (2 << (2 * pin)))})
        };
      },
      'e' => {
        let gpioe = &peripheral_ptr.GPIOE;
        match bias {
          Bias::None => gpioe.pupdr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin)))}),
          Bias::Pullup => gpioe.pupdr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin)) | (1 << (2 * pin)))}),
          Bias::Pulldown => gpioe.pupdr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin)) | (2 << (2 * pin)))})
        };
      },
      'f' => {
        let gpiof = &peripheral_ptr.GPIOF;
        match bias {
          Bias::None => gpiof.pupdr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin)))}),
          Bias::Pullup => gpiof.pupdr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin)) | (1 << (2 * pin)))}),
          Bias::Pulldown => gpiof.pupdr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin)) | (2 << (2 * pin)))})
        };
      },
      'g' => {
        let gpiog = &peripheral_ptr.GPIOG;
        match bias {
          Bias::None => gpiog.pupdr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin)))}),
          Bias::Pullup => gpiog.pupdr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin)) | (1 << (2 * pin)))}),
          Bias::Pulldown => gpiog.pupdr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin)) | (2 << (2 * pin)))})
        };
      },
      'h' => {
        let gpioh = &peripheral_ptr.GPIOH;
        match bias {
          Bias::None => gpioh.pupdr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin)))}),
          Bias::Pullup => gpioh.pupdr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin)) | (1 << (2 * pin)))}),
          Bias::Pulldown => gpioh.pupdr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin)) | (2 << (2 * pin)))})
        };
      },
      _   => panic!("P{}{} is not an available GPIO Pin | .bias(...)", self.block.to_uppercase(), pin)
    };

    self.bias = bias;
  }

  /// Set the driving of the pin from push-pull to open-drain.
  /// # Examples
  /// ```rust,no_run
  /// use rustuino::*;
  ///
  /// // Configure pin as output
  /// let mut pin = PA0::output();
  /// 
  /// // Set the pin to open-drain, the default behaviour is push-pull
  /// pin.open_drain();
  /// ```
  pub fn open_drain(&mut self) {
    let peripheral_ptr;
    unsafe {peripheral_ptr = stm32f4::stm32f446::Peripherals::steal();}
    let pin = self.pin;

    match self.block {
      'a' => {
        let gpioa = &peripheral_ptr.GPIOA;
        gpioa.otyper.modify(|r, w| unsafe {w.bits(r.bits() | (1 << pin))});
      },
      'b' => {
        let gpiob = &peripheral_ptr.GPIOB;
        gpiob.otyper.modify(|r, w| unsafe {w.bits(r.bits() | (1 << pin))});
      },
      'c' => {
        let gpioc = &peripheral_ptr.GPIOC;
        gpioc.otyper.modify(|r, w| unsafe {w.bits(r.bits() | (1 << pin))});
      },
      'd' => {
        let gpiod = &peripheral_ptr.GPIOD;
        gpiod.otyper.modify(|r, w| unsafe {w.bits(r.bits() | (1 << pin))});
      },
      'e' => {
        let gpioe = &peripheral_ptr.GPIOE;
        gpioe.otyper.modify(|r, w| unsafe {w.bits(r.bits() | (1 << pin))});
      },
      'f' => {
        let gpiof = &peripheral_ptr.GPIOF;
        gpiof.otyper.modify(|r, w| unsafe {w.bits(r.bits() | (1 << pin))});
      },
      'g' => {
        let gpiog = &peripheral_ptr.GPIOG;
        gpiog.otyper.modify(|r, w| unsafe {w.bits(r.bits() | (1 << pin))});
      },
      'h' => {
        let gpioh = &peripheral_ptr.GPIOH;
        gpioh.otyper.modify(|r, w| unsafe {w.bits(r.bits() | (1 << pin))});
      },
      _   => panic!("P{}{} is not an available GPIO Pin | .open_drain()", self.block.to_uppercase(), pin)
    };

    self.open_drain = true;
  }

  /// Writes the digital value to the output pin.
  /// # Examples
  /// ```rust,no_run
  /// use rustuino::*;
  ///
  /// // Configure pin as output
  /// let pin = PA0::output();
  /// 
  /// // Set the value of the pin
  /// pin.write(true);
  /// pin.write(false);
  /// ```
  pub fn write(&self, value: bool) {
    let peripheral_ptr;
    unsafe {peripheral_ptr = stm32f4::stm32f446::Peripherals::steal();}
    let pin = self.pin;

    match self.block {
      'a' => {
        let gpioa = &peripheral_ptr.GPIOA;
        if value == true {gpioa.bsrr.write(|w| unsafe {w.bits(1 << pin)});}
        else {gpioa.bsrr.write(|w| unsafe {w.bits(1 << (pin + 16))});}
      },
      'b' => {
        let gpiob = &peripheral_ptr.GPIOB;
        if value == true {gpiob.bsrr.write(|w| unsafe {w.bits(1 << pin)});}
        else {gpiob.bsrr.write(|w| unsafe {w.bits(1 << (pin + 16))});}
      },
      'c' => {
        let gpioc = &peripheral_ptr.GPIOC;
        if value == true {gpioc.bsrr.write(|w| unsafe {w.bits(1 << pin)});}
        else {gpioc.bsrr.write(|w| unsafe {w.bits(1 << (pin + 16))});}
      },
      'd' => {
        let gpiod = &peripheral_ptr.GPIOD;
        if value == true {gpiod.bsrr.write(|w| unsafe {w.bits(1 << pin)});}
        else {gpiod.bsrr.write(|w| unsafe {w.bits(1 << (pin + 16))});}
      },
      'e' => {
        let gpioe = &peripheral_ptr.GPIOE;
        if value == true {gpioe.bsrr.write(|w| unsafe {w.bits(1 << pin)});}
        else {gpioe.bsrr.write(|w| unsafe {w.bits(1 << (pin + 16))});}
      },
      'f' => {
        let gpiof = &peripheral_ptr.GPIOF;
        if value == true {gpiof.bsrr.write(|w| unsafe {w.bits(1 << pin)});}
        else {gpiof.bsrr.write(|w| unsafe {w.bits(1 << (pin + 16))});}
      },
      'g' => {
        let gpiog = &peripheral_ptr.GPIOG;
        if value == true {gpiog.bsrr.write(|w| unsafe {w.bits(1 << pin)});}
        else {gpiog.bsrr.write(|w| unsafe {w.bits(1 << (pin + 16))});}
      },
      'h' => {
        let gpioh = &peripheral_ptr.GPIOH;
        if value == true {gpioh.bsrr.write(|w| unsafe {w.bits(1 << pin)});}
        else {gpioh.bsrr.write(|w| unsafe {w.bits(1 << (pin + 16))});}
      },
      _   => panic!("P{}{} is not an available GPIO Pin | .write(...)", self.block.to_uppercase(), pin)
    };
  }
}
