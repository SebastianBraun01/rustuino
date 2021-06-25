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
  pin: [('a', 0),('a', 1),('a', 2),('a', 3),('a', 4),('a', 5),('a', 6),('a', 7),('b', 0),('b', 1),('c', 0),('c', 1),('c', 2),('c', 3),('c', 4),('c', 5)],
  channel: [0,       1,       2,       3,       4,       5,       6,       7,       8,       9,       10,      11,     12,    13,      14,      15]
};

pub const ADC3_MAP: ADC3Map = ADC3Map{
  pin:     [3, 4,  5,  6, 7, 8, 9],
  channel: [9, 14, 15, 4, 5, 6, 7]
};

pub static mut ADC_CONF: [bool; 2] = [false, false];


// UART pin config map ==========================================================================
pub struct UARTMap {
  pub tx_pin: [(char, u8); 11],
  pub rx_pin: [(char, u8); 11],
  pub channel: [u8; 11]
}

// Noch entscheiden ob USART2 in die Liste soll!
pub const UART_MAP: UARTMap = UARTMap{
  tx_pin:  [('a', 9), ('b', 6),('b', 10),('c', 10),('d', 8),('a', 0),('c', 6),('c', 10),('c', 12),('e', 8),('g', 14)],
  rx_pin:  [('a', 10),('b', 7),('b', 11),('c', 11),('d', 9),('a', 1),('c', 7),('c', 11),('d', 2), ('e', 7),('g', 9)],
  channel: [    1,        1,       3,        3,        3,       4,       6,       4,        5,        5,       6]
};

pub static mut UART_CONF: [bool; 6] = [false, false, false, false, false, false];


// PWM timer map ==================================================================================
pub struct TIMERMap {
    pub pin: [(char, u8); 74],
    pub timer: [u8; 74],
    pub ccch: [u8; 74],
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
