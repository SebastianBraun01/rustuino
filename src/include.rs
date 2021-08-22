use super::common::*;


// Analog pin config map ========================================================================
pub struct ADC1Map {
  pub pin: [(char, u8); 16],
  pub channel: [u8; 16]
}

pub struct ADC3Map {
  pub pin: [u8; 7],
  pub channel: [u8; 7]
}

pub const ADC1_MAP: ADC1Map = ADC1Map{
  pin:     [A0, A1, A2, A3, A4, A5, A6, A7, B0, B1, C0, C1, C2, C3, C4, C5],
  channel: [0,  1,  2,  3,  4,  5,  6,  7,  8,  9,  10, 11, 12, 13, 14, 15]
};

pub const ADC3_MAP: ADC3Map = ADC3Map{
  pin:     [3, 4,  5,  6, 7, 8, 9],
  channel: [9, 14, 15, 4, 5, 6, 7]
};

pub static mut ADC_CONF: [bool; 2] = [false, false];


// UART pin config map ==========================================================================
pub struct UARTMap {
  pub tx_pins: [(char, u8); 11],
  pub rx_pins: [(char, u8); 11],
  pub core: [u8; 11]
}

// Noch entscheiden ob USART2 in die Liste soll!
pub const UART_MAP: UARTMap = UARTMap{
  tx_pins: [A9,  B6, B10, C10, D8, A0, C6, C10, C12, E8, G14],
  rx_pins: [A10, B7, B11, C11, D9, A1, C7, C11, D2,  E7, G9],
  core: [1,   1,  3,   3,   3,  4,  6,  4,   5,   5,  6]
};

pub static mut UART_CONF: [bool; 6] = [false, false, false, false, false, false];


// PWM timer map ==================================================================================
pub struct TIMERMap {
  pub pin: [(char, u8); 74],
  pub timer: [u8; 74],
  pub ccch: [u8; 74]
}
  
pub const TIMER_MAP: TIMERMap = TIMERMap{
  pin: [('a', 0), ('a', 1), ('a', 2), ('a', 3), ('a', 5), ('a', 8), ('a', 9), ('a', 10), ('a', 11), ('a', 15), ('b', 0), ('b', 1), ('b', 2), ('b', 3), ('b', 8), ('b', 9), ('b', 10), ('b', 11), ('b', 13), ('b', 14), ('b', 15), ('e', 8), ('e', 9), ('e', 10), ('e', 11), ('e', 12), ('e', 13), ('e', 14), ('a', 0), ('a', 1), ('a', 2), ('a', 3), ('a', 6), ('a', 7), ('b', 0), ('b', 1), ('b', 4), ('b', 5), ('b', 6), ('b', 7), ('b', 8), ('b', 9), ('c', 6), ('c', 7), ('c', 8), ('c', 9), ('d', 12), ('d', 13), ('d', 14), ('d', 15), ('a', 2), ('a', 3), ('a', 5), ('a', 7), ('b', 0), ('b', 1), ('b', 8), ('b', 9), ('b', 14), ('b', 15), ('c', 6), ('c', 7), ('c', 8), ('c', 9), ('e', 5), ('e', 6), ('f', 6), ('f', 7), ('a', 6), ('a', 7), ('b', 14), ('b', 15), ('f', 8), ('f', 9)],
    
  timer: [2, 2, 2, 2, 2, 1, 1, 1, 1, 2, 1, 1, 2, 2, 2, 2, 2, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 5, 5, 5, 5, 3, 3, 3, 3, 3, 3, 4, 4, 4, 4, 3, 3, 3, 3, 4, 4, 4, 4, 9, 9, 8, 8, 8, 8, 10, 11, 8, 8, 8, 8, 8, 8, 9, 9, 10, 11, 13, 14, 12, 12, 13, 14],
    
  ccch: [1, 2, 3, 4, 1, 1, 2, 3, 4, 1, 2, 3, 4, 2, 1, 2, 3, 4, 1, 2, 3, 1, 1, 2, 2 ,3, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 1, 1, 2, 3, 1, 1, 2, 3, 1, 2, 3, 4, 1, 2, 1, 1, 1, 1, 1, 2, 1, 1]
};

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
pub static mut DELAY_COUNTER: (bool, u32) = (false, 0);
pub static mut TIME_COUNTER: usize = 0;


// I2C Channel map ================================================================================
pub struct I2CMap {
  pub scl_pins: [(char, u8); 9],
  pub sda_pins: [(char, u8); 9],
  pub core: [u8; 9]
}

pub const I2C_MAP: I2CMap = I2CMap{
  scl_pins: [B6, B6, B8, B8, B10, B10, F1, F1,  A8],
  sda_pins: [B7, B9, B7, B9, B11, F0,  F0, B11, B4],
  core:  [1,  1,  1,  1,  2,   2,   2,  2,   3]
};

pub static mut I2C_CONF: [bool; 3] = [false, false, false];


// SPI Channel map ================================================================================
pub struct SPIMap {
  pub sck_pins: [(char, u8); 7],
  pub miso_pins: [(char, u8); 7],
  pub mosi_pins: [(char, u8); 7],
  pub core: [u8; 7],
  pub af: [u32; 7]
}

pub const SPI_MAP: SPIMap = SPIMap {
  sck_pins:  [A5, B3, B3, B13, C10, E12, G11],
  miso_pins: [A6, B4, B4, B14, C11, E13, G12],
  mosi_pins: [A7, B5, B5, B15, C12, E14, G13],
  core:      [1,  1,  3,  2,   3,   4,   4],
  af:        [5,  5,  6,  5,   6,   5,   6]
};

pub static mut SPI_CONF: [bool; 4] = [false, false, false, false];
