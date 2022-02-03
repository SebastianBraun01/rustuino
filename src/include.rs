//! Contains all pinmaps for pinmode and init functions and error enums.

use pins::*;

#[doc(hidden)]
pub static mut PIN_CONF: heapless::Vec<(char, u8), 50> = heapless::Vec::new();

/// Pin aliases for function parameters. Use with pinmode- and init functions.
pub mod pins {
  macro_rules! generate_pins {
    ($([$block:literal, $pin:literal]),+) => {
      use paste::paste;

      paste!{
        $(
          pub const [<$block:upper $pin>]: (char, u8) = ($block, $pin);
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

    ['d', 2],

    ['h', 0],
    ['h', 1]
  ];
}


#[doc(hidden)]
pub struct ADCMap {
  pub pins: [(char, u8); 16],
  pub adcs: [u8; 16],
  pub channels: [u8; 16]
}

/// Pinmap of the available analog inputs and outputs.
/// 
/// These pins are available for [pinmode_analog()](crate::gpio::pinmode_analog) as an analog input over the internal ADC.
/// 
/// ## WARNING:
/// 
/// Whitch ADC and channel the pin uses is not important for normal use, but if you intent to use one of these ADCs manually while the pin is configured, normal function of the pin is not guaranteed!
/// 
/// | Pin | Channel | ADCs      |
/// | --- | ------- | --------- |
/// | PA0 | 0       | 1, 2, 3   |
/// | PA1 | 1       | 1, 2, 3   |
/// | PA2 | 2       | 1, 2, 3   |
/// | PA3 | 3       | 1, 2, 3   |
/// | PA4 | 1       | 1, 2, DAC |
/// | PA5 | 2       | 1, 2, DAC |
/// | PA6 | 6       | 1, 2      |
/// | PA7 | 7       | 1, 2      |
/// | PB0 | 8       | 1, 2      |
/// | PB1 | 9       | 1, 2      |
/// | PC0 | 10      | 1, 2, 3   |
/// | PC1 | 11      | 1, 2, 3   |
/// | PC2 | 12      | 1, 2, 3   |
/// | PC3 | 13      | 1, 2, 3   |
/// | PC4 | 14      | 1, 2      |
/// | PC5 | 15      | 1, 2      |
pub const ADC_MAP: ADCMap = ADCMap {
  pins:     [A0, A1, A2, A3, A4, A5, A6, A7, B0, B1, C0, C1, C2, C3, C4, C5],
  adcs:     [1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1],
  channels: [0,  1,  2,  3,  4,  5,  6,  7,  8,  9,  10, 11, 12, 13, 14, 15]
};

#[doc(hidden)]
pub struct PWMMap {
  pub pins: [(char, u8); 27],
  pub timers: [u8; 27],
  pub ccchs: [u8; 27]
}

/// Pinmap of the available PWM outputs.
/// 
/// In [`pinmode_pwm()`](crate::gpio::pinmode_pwm) these pins are available for PWM output. You see that some channels of timer 2 and 3 are used for multiple pins. If you configure both pin of the same channel, the output on both pin will be the same, regardless of whitch pin you modify.
/// 
/// ## WARNING:
/// 
/// Whitch timer and channel the pin uses is not important for normal use, but if you intent to use one of these timers for other purposes, be aware that modifying the values of the timer could lead to a broken PWM signal.
/// 
/// | Timer | Pin                          | Channel          |
/// | ----- | ---------------------------- | ---------------- |
/// | 1     | PA8, PA9, PA10, PA11         | 1, 2, 3, 4       |
/// | 2     | PA0, PA1, PA2, PA3           | 1, 2, 3, 4       |
/// | 2     | PA15, PB2, PB3, PB10, PB11   | 1, 1, 4, 2, 3, 4 |
/// | 3     | PA6, PA7, PB0, PB1           | 1, 2, 3, 4       |
/// | 3     | PB4, PB5, PC6, PC7, PC8, PC9 | 1, 2, 1, 2, 3, 4 |
/// | 4     | PB6, PB7, PB8, PB9           | 1, 2, 3, 4       |
pub const PWM_MAP: PWMMap = PWMMap {
  pins:   [A8, A9, A10, A11, A0, A1, A2, A3, A15, B2, B3, B10, B11, A6, A7, B0, B1, B4, B5, C6, C7, C8, C9, B6, B7, B8, B9],
  timers: [1,  1,  1,   1,   2,  2,  2,  2,  2,   2,  2,  2,   2,   3,  3,  3,  3,  3,  3,  3,  3,  3,  3,  4,  4,  4,  4],
  ccchs:  [1,  2,  3,   4,   1,  2,  3,  4,  1,   4,  2,  3,   4,   1,  2,  3,  4,  1,  2,  1,  2,  3,  4,  1,  2,  3,  4]
};

#[doc(hidden)]
pub struct UARTMap {
  pub tx_pins: [(char, u8); 16],
  pub rx_pins: [(char, u8); 16],
  pub cores: [u8; 16]
}

/// Pinmap for the UART peripheral.
/// 
/// In [`UART::new()`](crate::uart::UART::new) choose the desired peripheral and a combination of available pins for it. Here are the available pins for each core:
/// 
/// | UART Core      | TX Pins    | RX Pins         |
/// | -------------- | ---------- | --------------- |
/// | 1              | PA9, PB6   | PA10, PB7       |
/// | 2 (USB Serial) | PA2        | PA3             |
/// | 3              | PB10, PC10 | PB11, PC5, PC11 |
/// | 4              | PA0, PC10  | PA1, PC11       |
/// | 5              | PC12       | PD2             |
/// | 6              | PC6        | PC7             |
pub const UART_MAP: UARTMap = UARTMap {
  tx_pins: [A9,  A9, B6,  B6, B10, B10, B10, C10, C10, C10, A0, A0,  C10, C10, C12, C6],
  rx_pins: [A10, B7, A10, B7, B11, C5,  C11, B11, C5,  C11, A1, C11, A1,  C11, D2,  C7],
  cores:   [1,   1,  1,   1,  3,   3,   3,   3,   3,   3,   4,  4,   4,   4,   5,   6]
};

#[doc(hidden)]
pub struct I2CMap {
  pub scl_pins: [(char, u8); 9],
  pub sda_pins: [(char, u8); 9],
  pub cores: [u8; 9]
}

/// Pinmap for the I2C peripheral.
/// 
/// In [`I2C::new()`](crate::i2c::I2C::new) choose the desired peripheral and a combination of available pins for it. Here are the available pins for each core:
/// 
/// | I2C Core | SCL Pins | SDA Pins        |
/// | -------- | -------- | --------------- |
/// | I2C1     | PB6, PB8 | PB7, PB9        |
/// | I2C2     | PB10     | PB3, PB11, PC12 |
/// | I2C3     | PA8      | PB4, PC3        |
pub const I2C_MAP: I2CMap = I2CMap {
  scl_pins: [B6, B6, B8, B8, B10, B10, B10, A8, A8],
  sda_pins: [B7, B9, B7, B9, B3,  B11, C12, B4, C3],
  cores:    [1,  1,  1,  1,  2,   2,   2,   3,  3]
};

#[doc(hidden)]
pub struct SPIData {
  pub s1_sck: [(char, u8); 2],
  pub s2_sck: [(char, u8); 4],
  pub s3_sck: [(char, u8); 2],
  pub s1_miso: [(char, u8); 2],
  pub s2_miso: [(char, u8); 2],
  pub s3_miso: [(char, u8); 2],
  pub s1_mosi: [(char, u8); 2],
  pub s2_mosi: [(char, u8); 2],
  pub s3_mosi: [(char, u8); 2],
}

/// Pinmap for the SPI peripheral.
/// 
/// In SPI::new() choose the desired peripheral and a combination of available pins for it. Here are the available pins for each core:
/// 
/// | Core | SCK Pins             | MISO Pins | MOSI Pins |
/// | ---- | -------------------- | --------- | --------- |
/// | 1    | PA5, PB3             | PA6, PB4  | PA7, PB5  |
/// | 2    | PA9, PB10, PB13, PC7 | PB14, PC2 | PB15, PC3 |
/// | 3    | PB3, PC10            | PB4, PC11 | PB5, PC12 |
pub const SPI_DATA: SPIData = SPIData {
  s1_sck: [A5, B3],
  s2_sck: [A9, B10, B13, C7],
  s3_sck: [B3,C10],
  s1_miso: [A6, B4],
  s2_miso: [B14, C2],
  s3_miso: [B4, C11],
  s1_mosi: [A7, B5],
  s2_mosi: [B15, C3],
  s3_mosi: [B5, C12]
};


/// A universal implementation specific error.
///
/// These error kinds can be used to signal implementation specific errors unrelated to the
/// specific peripheral. This will be used for all sorts of connectivity and configuraton problems.
/// 
/// All of the enums in this crate are marked as `#[non_exhaustive]` to allow for additions of new
/// error kinds without requiring a breaking change and version bump.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum ProgError {
  /// Unspecified internal driver error
  Internal,
  /// Ran out of memory while trying to allocate required buffers
  OutOfMemory,
  /// Operation timed out, please retry
  TimedOut,
  /// The peripheral cannot work with the specified settings
  InvalidConfiguration,
  /// Tried to use peripheral without configuring it properly
  NotConfigured,
  /// Tried to setup a peripheral that is already configured
  AlreadyConfigured,
  /// Invalid action
  PermissionDenied
}

/// A GPIO (General input/output) specific error.
///
/// This error type contains errors specific to GPIO peripherals. Also it has an "Prog" kind to
/// pass through implementation specific errors occuring while trying to use a GPIO peripheral.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum GpioError {
  /// The peripheral is in the wrong operational mode for the intended operation
  WrongMode,
  /// Implementation specific error (shared across all peripheral specific error kinds)
  Prog(ProgError)
}

/// A Serial specific error.
///
/// This error type contains errors specific to Serial peripherals. Also it has an "Prog" kind to pass
/// through implementation specific errors occurring while trying to use a Serial peripheral.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum SerialError {
  /// The peripheral receive buffer was overrun.
  Overrun,
  /// Received data does not conform to the peripheral configuration.
  /// Can be caused by a misconfigured device on either end of the serial line.
  FrameFormat,
  /// Parity check failed.
  Parity,
  /// Serial line is too noisy to read valid data.
  Noise,
  /// Implementation specific error (shared across all peripheral specific error kinds).
  Prog(ProgError)
}

/// An I2C specific error.
///
/// This error type contains errors specific to I2C peripherals. Also it has an "Prog" kind to pass
/// through implementation specific errors occurring while trying to use an I2C peripheral.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum I2cError {
  /// An unspecific bus error occured
  Bus,
  /// The arbitration was lost, e.g. electrical problems with the clock signal
  ArbitrationLoss,
  /// A bus operation received a NACK, e.g. due to the addressed device not being available on
  /// the bus or device not being ready to process any requests at the moment
  NACK,
  /// The peripheral receive buffer was overrun or ran out of data
  OverrunUnderrun,
  /// Parity check failed.
  Parity,
  /// Implementation specific error (shared across all peripheral specific error kinds)
  Prog(ProgError)
}

/// A SPI specific error.
///
/// This error type contains errors specific to SPI peripherals. Also it has an "Prog" kind to pass
/// through implementation specific errors occuring while trying to use a SPI peripheral.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum SpiError {
  /// The peripheral receive buffer was overrun
  Overrun,
  /// Multiple devices on the SPI bus are trying across each other, e.g. in a multi-master setup
  ModeFault,
  /// CRC does not match the received data
  CRCError,
  /// Implementation specific error (shared across all peripheral specific error kinds)
  Prog(ProgError)
}
