// Register pointers ==============================================================================
pub const PERIPHERAL_PTR: stm32f4::stm32f446::Peripherals = stm32f4::stm32f446::Peripherals::take().unwrap();
pub const CORE_PERIPHERAL_PTR: cortex_m::Peripherals = cortex_m::Peripherals::take().unwrap();


// Data maps ======================================================================================
pub mod data_maps {
  // Analog pin config map ========================================================================
  struct ADC1Map {
    pub pin: [(char, u8); 16],
    pub channel: [u8; 16]
  }

  struct ADC3Map {
    pub pin: [u8; 7],
    pub channel: [u8; 7]
  }

  pub const ADC1_MAP: ADC1Map = ADC1Map{
    pin: [('a', 0), ('a', 1), ('a', 2), ('a', 3), ('a', 4), ('a', 5), ('a', 6), ('a', 7), ('b', 0), ('b', 1), ('c', 0), ('c', 1), ('c', 2), ('c', 3), ('c', 4), ('c', 5)],
    channel: [0,   1,   2,   3,   4,   5,   6,   7,   8,   9,   10,  11,  12,  13,  14,  15]
  };

  pub const ADC3_MAP: ADC3Map = ADC3Map{
    pin:     [3, 4,  5,  6, 7, 8, 9],
    channel: [9, 14, 15, 4, 5, 6, 7]
  };  


  // Seral config storage =========================================================================
  pub static mut UART_USB: bool = false;












  // UART pin config map ==========================================================================
  // struct UARTMap {
  //   pub tx_pin: [(u8, char); 12],
  //   pub rx_pin: [(u8, char); 12],
  //   pub channel: [u8; 12],
  //   pub active: [bool; 12]
  // }

  // // TODO: better uart map
  // pub static mut UART_MAP: UARTMap = UARTMap{
  //   tx_pin:  [PA9,   PB6,   PB10,  PC10,  PD5,   PD8,   PA0,   PC6,   PC10,  PC12,  PE8,   PG14],
  //   rx_pin:  [PA10,  PB7,   PB11,  PC11,  PD6,   PD9,   PA1,   PC7,   PC11,  PD2,   PE7,   PG9],
  //   channel: [1,     1,     3,     3,     2,     3,     4,     6,     4,     5,     5,     6],
  //   active:  [false, false, false, false, false, false, false, false, false, false, false, false]
  // };

  


  // PWM pin config map ===========================================================================
  // struct TIMERMap {
  //   pub pin: [(u8, char); 74],
  //   pub timer: [u8; 74],
  //   pub ccch: [u8; 74],
  //   pub active: [bool; 74]
  // }

  // pub static mut TIMER_MAP: TIMERMap = TIMERMap{
  //   pin: [PA0, PA1, PA2, PA3, PA5, PA8, PA9, PA10, PA11, PA15, PB0, PB1, PB2, PB3, PB8, PB9, PB10, PB11, PB13, PB14, PB15, PE8, PE9, PE10, PE11, PE12, PE13, PE14, PA0, PA1, PA2, PA3, PA6, PA7, PB0, PB1, PB4, PB5, PB6, PB7, PB8, PB9, PC6, PC7, PC8, PC9, PD12, PD13, PD14, PD15, PA2, PA3, PA5, PA7, PB0, PB1, PB8, PB9, PB14, PB15, PC6, PC7, PC8, PC9, PE5, PE6, PF6, PF7, PA6, PA7, PB14, PB15, PF8, PF9],

  //   timer: [2, 2, 2, 2, 2, 1, 1, 1, 1, 2, 1, 1, 2, 2, 2, 2, 2, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 5, 5, 5, 5, 3, 3, 3, 3, 3, 3, 4, 4, 4, 4, 3, 3, 3, 3, 4, 4, 4, 4, 9, 9, 8, 8, 8, 8, 10, 11, 8, 8, 8, 8, 8, 8, 9, 9, 10, 11, 13, 14, 12, 12, 13, 14],

  //   ccch: [1, 2, 3, 4, 1, 1, 2, 3, 4, 1, 2, 3, 4, 2, 1, 2, 3, 4, 1, 2, 3, 1, 1, 2, 2 ,3, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 1, 1, 2, 3, 1, 1, 2, 3, 1, 2, 3, 4, 1, 2, 1, 1, 1, 1, 1, 2, 1, 1],

  //   active: [false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false]
  // };
}


// Variables ======================================================================================
pub mod variables {
  pub static mut TIME_COUNTER: usize = 0;
}
