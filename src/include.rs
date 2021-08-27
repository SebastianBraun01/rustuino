/// Pin structs for the digital, analog, and pwm functions and aliases for the paremeters for the uart, i2c and spi init fuctions
pub mod pins {
  macro_rules! generate_pins {
    ($([$block:literal, $pin:literal]),+) => {
      use paste::paste;
  
      paste!{
        $(
        pub struct [<P $block:upper $pin>];
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
}


use pins::*;


// Analog configs =================================================================================
/// Struct for the ADC channel 1 pin map
pub struct ADCMap {
  /// Pin identifiers
  pub pin: [(char, u8); 23],
  /// ADC1 channel for the pin
  pub channel: [u8; 23]
}

/// Struct for DAC channel pin map
pub struct DACMap {
  /// Pin identifiers
  pub pin: [(char, u8); 2],
  /// DAC channel for the pin
  pub channel: [u8; 2]
}

/// ADC channel pin map
pub const ADC_MAP: ADCMap = ADCMap{
  pin:     [A0, A1, A2, A3, A4, A5, A6, A7, B0, B1, C0, C1, C2, C3, C4, C5, F3, F4,  F5,  F6, F7, F8, F9],
  channel: [0,  1,  2,  3,  4,  5,  6,  7,  8,  9,  10, 11, 12, 13, 14, 15, 9,  14,  15,  4,  5,  6,  7]
};

/// DAC pin map
pub const DAC_MAP: DACMap = DACMap {
  pin:     [A4, A5],
  channel: [1,  2]
};

/// ADC config memory
pub static mut ADC_CONF: [bool; 2] = [false, false];

/// DAC config memory
pub static mut DAC_CONF: [bool; 2] = [false, false];


// UART configs ===================================================================================
/// Struct for UART pin map
pub struct UARTMap {
  /// TX pin identifiers
  pub tx_pins: [(char, u8); 11],
  /// RX pin identifiers
  pub rx_pins: [(char, u8); 11],
  /// UART peripheral for the selected pins
  pub core: [u8; 11]
}

/// UART pin map
pub const UART_MAP: UARTMap = UARTMap{
  tx_pins: [A9,  B6, B10, C10, D8, A0, C6, C10, C12, E8, G14],
  rx_pins: [A10, B7, B11, C11, D9, A1, C7, C11, D2,  E7, G9],
  core:    [1,   1,  3,   3,   3,  4,  6,  4,   5,   5,  6]
};

/// UART config memory
pub static mut UART_CONF: [bool; 6] = [false, false, false, false, false, false];


// PWM configs ====================================================================================
/// Struct for PWM CC channel pin map
pub struct TIMERMap {
  /// PWM CC pin identifier
  pub pin: [(char, u8); 74],
  /// Timer peripheral for pin
  pub timer: [u8; 74],
  /// CC channel for pin
  pub ccch: [u8; 74]
}

/// PWM CC channel pin map
pub const TIMER_MAP: TIMERMap = TIMERMap{
  pin: [('a', 0), ('a', 1), ('a', 2), ('a', 3), ('a', 5), ('a', 8), ('a', 9), ('a', 10), ('a', 11), ('a', 15), ('b', 0), ('b', 1), ('b', 2), ('b', 3), ('b', 8), ('b', 9), ('b', 10), ('b', 11), ('b', 13), ('b', 14), ('b', 15), ('e', 8), ('e', 9), ('e', 10), ('e', 11), ('e', 12), ('e', 13), ('e', 14), ('a', 0), ('a', 1), ('a', 2), ('a', 3), ('a', 6), ('a', 7), ('b', 0), ('b', 1), ('b', 4), ('b', 5), ('b', 6), ('b', 7), ('b', 8), ('b', 9), ('c', 6), ('c', 7), ('c', 8), ('c', 9), ('d', 12), ('d', 13), ('d', 14), ('d', 15), ('a', 2), ('a', 3), ('a', 5), ('a', 7), ('b', 0), ('b', 1), ('b', 8), ('b', 9), ('b', 14), ('b', 15), ('c', 6), ('c', 7), ('c', 8), ('c', 9), ('e', 5), ('e', 6), ('f', 6), ('f', 7), ('a', 6), ('a', 7), ('b', 14), ('b', 15), ('f', 8), ('f', 9)],
    
  timer: [2, 2, 2, 2, 2, 1, 1, 1, 1, 2, 1, 1, 2, 2, 2, 2, 2, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 5, 5, 5, 5, 3, 3, 3, 3, 3, 3, 4, 4, 4, 4, 3, 3, 3, 3, 4, 4, 4, 4, 9, 9, 8, 8, 8, 8, 10, 11, 8, 8, 8, 8, 8, 8, 9, 9, 10, 11, 13, 14, 12, 12, 13, 14],
    
  ccch: [1, 2, 3, 4, 1, 1, 2, 3, 4, 1, 2, 3, 4, 2, 1, 2, 3, 4, 1, 2, 3, 1, 1, 2, 2 ,3, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 1, 1, 2, 3, 1, 1, 2, 3, 1, 2, 3, 4, 1, 2, 1, 1, 1, 1, 1, 2, 1, 1]
};

/// PWM config memory
pub static mut TIMER_CONF: [bool; 56] = [
  false, false, false, false, // 1: 4
  false, false, false, false, // 2: 4
  false, false, false, false, // 3: 4
  false, false, false, false, // 4: 4
  false, false, false, false, // 5: 4
  false, false, false, false, // 6: 0
  false, false, false, false, // 7: 0
  false, false, false, false, // 8: 4
  false, false, false, false, // 9: 2
  false, false, false, false, // 10: 1
  false, false, false, false, // 11: 1
  false, false, false, false, // 12: 2
  false, false, false, false, // 13: 1
  false, false, false, false, // 14: 1
];


// Timer Variables ================================================================================
/// Counter for the delay function
pub static mut DELAY_COUNTER: (bool, u32) = (false, 0);

/// Counter for the millis function
pub static mut TIME_COUNTER: usize = 0;


// I2C configs ====================================================================================
/// Struct for I2C pin map
pub struct I2CMap {
  /// SCL pin identifier
  pub scl_pins: [(char, u8); 9],
  /// SDA pin identifier
  pub sda_pins: [(char, u8); 9],
  /// I2C peripheral for the selected pins
  pub core: [u8; 9]
}

/// I2C pin map
pub const I2C_MAP: I2CMap = I2CMap{
  scl_pins: [B6, B6, B8, B8, B10, B10, F1, F1,  A8],
  sda_pins: [B7, B9, B7, B9, B11, F0,  F0, B11, B4],
  core:  [1,  1,  1,  1,  2,   2,   2,  2,   3]
};

/// I2C config memory
pub static mut I2C_CONF: [bool; 3] = [false, false, false];


// SPI configs ====================================================================================
/// Struct for SPI pin map
pub struct SPIMap {
  /// SCK pin identifier
  pub sck_pins: [(char, u8); 7],
  /// MISO pin identifier
  pub miso_pins: [(char, u8); 7],
  /// MOSI pin identifier
  pub mosi_pins: [(char, u8); 7],
  /// SPI peripheral for the selected pins
  pub core: [u8; 7],
  /// Alternate function number for the selected pins
  pub af: [u32; 7]
}

/// SPI pin map
pub const SPI_MAP: SPIMap = SPIMap {
  sck_pins:  [A5, B3, B3, B13, C10, E12, G11],
  miso_pins: [A6, B4, B4, B14, C11, E13, G12],
  mosi_pins: [A7, B5, B5, B15, C12, E14, G13],
  core:      [1,  1,  3,  2,   3,   4,   4],
  af:        [5,  5,  6,  5,   6,   5,   6]
};

/// SPI config memory
pub static mut SPI_CONF: [bool; 4] = [false, false, false, false];
