/// Pin aliases for function parameters.
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


// Register structs ===============================================================================
pub fn core_peripherals() -> cortex_m::Peripherals {
  unsafe {return cortex_m::Peripherals::steal();}
}

pub fn stm_peripherals() -> stm32f4::stm32f446::Peripherals {
  unsafe {return stm32f4::stm32f446::Peripherals::steal();}
}


// Pin maps =======================================================================================
use pins::*;

pub struct ADCMap {
  pub pins: [(char, u8); 16],
  pub adcs: [u8; 16],
  pub channels: [u8; 16]
}

pub const ADC_MAP: ADCMap = ADCMap {
  pins:     [A0, A1, A2, A3, A4, A5, A6, A7, B0, B1, C0, C1, C2, C3, C4, C5],
  adcs:     [1,  1,  1,  1,  0,  0,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1],
  channels: [0,  1,  2,  3,  1,  2,  6,  7,  8,  9,  10, 11, 12, 13, 14, 15]
};

pub struct PWMMap {
  pub pins: [(char, u8); 27],
  pub timers: [u8; 27],
  pub ccchs: [u8; 27]
}

pub const PWM_MAP: PWMMap = PWMMap {
  pins:   [A8, A9, A10, A11, A0, A1, A2, A3, A15, B2, B3, B10, B11, A6, A7, B0, B1, B4, B5, C6, C7, C8, C9, B6, B7, B8, B9],
  timers: [1,  1,  1,   1,   2,  2,  2,  2,  2,   2,  2,  2,   2,   3,  3,  3,  3,  3,  3,  3,  3,  3,  3,  4,  4,  4,  4],
  ccchs:  [1,  2,  3,   4,   1,  2,  3,  4,  1,   4,  2,  3,   4,   1,  2,  3,  4,  1,  2,  1,  2,  3,  4,  1,  2,  3,  4]
};

pub struct USARTMap {
  pub tx_pins: [(char, u8); 16],
  pub rx_pins: [(char, u8); 16],
  pub cores: [u8; 16]
}

pub const USART_MAP: USARTMap = USARTMap {
  tx_pins: [A9,  A9, B6,  B6, B10, B10, B10, C10, C10, C10, A0, A0,  C10, C10, C12, C6],
  rx_pins: [A10, B7, A10, B7, B11, C5,  C11, B11, C5,  C11, A1, C11, A1,  C11, D2,  C7],
  cores:   [1,   1,  1,   1,  3,   3,   3,   3,   3,   3,   4,  4,   4,   4,   5,   6]
};

pub struct I2CMap {
  pub scl_pins: [(char, u8); 9],
  pub sda_pins: [(char, u8); 9],
  pub cores: [u8; 9]
}

pub const I2C_MAP: I2CMap = I2CMap {
  scl_pins: [B6, B6, B8, B8, B10, B10, B10, A8, A8],
  sda_pins: [B7, B9, B7, B9, B3,  B11, C12, B4, C3],
  cores:    [1,  1,  1,  1,  2,   2,   2,   3,  3]
};

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


// Embedded Errors ================================================================================
/// A universal implementation specific error.
///
/// These error kinds can be used to signal implementation specific errors unrelated to the
/// specific peripheral. This will be used for all sorts of connectivity problems, e.g. if an
/// adapter to the peripheral is used or the target peripheral is connected to indirectly (like bus
/// expanders) or an operating system is controlling the access and denying access.
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

/// This crate contains a variety of universal error types which can be used to universally model
/// conditions which can typically arise for certain peripherals.
///
/// When used by HAL implementations, they allow drivers and applications alike to generically
/// handle those situations without the error handling being specific to the hardware it is
/// supposed to run on (which is usually not possible to implement in drivers).
///
/// All of the enums in this crate are marked as `#[non_exhaustive]` to allow for additions of new
/// error kinds without requiring a breaking change and version bump.

/// A GPIO (General input/output) specific error.
///
/// This error type contains errors specific to GPIO peripherals. Also it has an `Impl` kind to
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
/// This error type contains errors specific to Serial peripherals. Also it has an `Impl` kind to pass
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
/// This error type contains errors specific to I2C peripherals. Also it has an `Impl` kind to pass
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
/// This error type contains errors specific to SPI peripherals. Also it has an `Impl` kind to pass
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
