use super::{Config, ADCMap, UARTMap};
use stm32f4::stm32f446;
use heapless::Vec;

// Pin Identifiers ================================================================================
pub const PA0: (u8, char) = (0, 'a');
pub const PA1: (u8, char) = (1, 'a');
pub const PA2: (u8, char) = (2, 'a');
pub const PA3: (u8, char) = (3, 'a');
pub const PA4: (u8, char) = (4, 'a');
pub const PA5: (u8, char) = (5, 'a');
pub const PA6: (u8, char) = (6, 'a');
pub const PA7: (u8, char) = (7, 'a');
pub const PA8: (u8, char) = (8, 'a');
pub const PA9: (u8, char) = (9, 'a');
pub const PA10: (u8, char) = (10, 'a');
pub const PA11: (u8, char) = (11, 'a');
pub const PA12: (u8, char) = (12, 'a');
pub const PA13: (u8, char) = (13, 'a');
pub const PA14: (u8, char) = (14, 'a');
pub const PA15: (u8, char) = (15, 'a');

pub const PB0: (u8, char) = (0, 'b');
pub const PB1: (u8, char) = (1, 'b');
pub const PB2: (u8, char) = (2, 'b');
pub const PB3: (u8, char) = (3, 'b');
pub const PB4: (u8, char) = (4, 'b');
pub const PB5: (u8, char) = (5, 'b');
pub const PB6: (u8, char) = (6, 'b');
pub const PB7: (u8, char) = (7, 'b');
pub const PB8: (u8, char) = (8, 'b');
pub const PB9: (u8, char) = (9, 'b');
pub const PB10: (u8, char) = (10, 'b');
pub const PB11: (u8, char) = (11, 'b');
pub const PB12: (u8, char) = (12, 'b');
pub const PB13: (u8, char) = (13, 'b');
pub const PB14: (u8, char) = (14, 'b');
pub const PB15: (u8, char) = (15, 'b');

pub const PC0: (u8, char) = (0, 'c');
pub const PC1: (u8, char) = (1, 'c');
pub const PC2: (u8, char) = (2, 'c');
pub const PC3: (u8, char) = (3, 'c');
pub const PC4: (u8, char) = (4, 'c');
pub const PC5: (u8, char) = (5, 'c');
pub const PC6: (u8, char) = (6, 'c');
pub const PC7: (u8, char) = (7, 'c');
pub const PC8: (u8, char) = (8, 'c');
pub const PC9: (u8, char) = (9, 'c');
pub const PC10: (u8, char) = (10, 'c');
pub const PC11: (u8, char) = (11, 'c');
pub const PC12: (u8, char) = (12, 'c');
pub const PC13: (u8, char) = (13, 'c');
pub const PC14: (u8, char) = (14, 'c');
pub const PC15: (u8, char) = (15, 'c');

pub const PD0: (u8, char) = (0, 'd');
pub const PD1: (u8, char) = (1, 'd');
pub const PD2: (u8, char) = (2, 'd');
pub const PD3: (u8, char) = (3, 'd');
pub const PD5: (u8, char) = (5, 'd');
pub const PD6: (u8, char) = (6, 'd');
pub const PD7: (u8, char) = (7, 'd');
pub const PD8: (u8, char) = (8, 'd');
pub const PD9: (u8, char) = (9, 'd');
pub const PD10: (u8, char) = (10, 'd');
pub const PD11: (u8, char) = (11, 'd');
pub const PD12: (u8, char) = (12, 'd');
pub const PD13: (u8, char) = (13, 'd');
pub const PD14: (u8, char) = (14, 'd');
pub const PD15: (u8, char) = (15, 'd');

pub const PE0: (u8, char) = (0, 'e');
pub const PE1: (u8, char) = (1, 'e');
pub const PE2: (u8, char) = (2, 'e');
pub const PE3: (u8, char) = (3, 'e');
pub const PE4: (u8, char) = (4, 'e');
pub const PE5: (u8, char) = (5, 'e');
pub const PE6: (u8, char) = (6, 'e');
pub const PE7: (u8, char) = (7, 'e');
pub const PE8: (u8, char) = (8, 'e');
pub const PE9: (u8, char) = (9, 'e');
pub const PE10: (u8, char) = (10, 'e');
pub const PE11: (u8, char) = (11, 'e');
pub const PE12: (u8, char) = (12, 'e');
pub const PE13: (u8, char) = (13, 'e');
pub const PE14: (u8, char) = (14, 'e');
pub const PE15: (u8, char) = (15, 'e');

pub const PF0: (u8, char) = (0, 'f');
pub const PF1: (u8, char) = (1, 'f');
pub const PF2: (u8, char) = (2, 'f');
pub const PF3: (u8, char) = (3, 'f');
pub const PF4: (u8, char) = (4, 'f');
pub const PF5: (u8, char) = (5, 'f');
pub const PF6: (u8, char) = (6, 'f');
pub const PF7: (u8, char) = (7, 'f');
pub const PF8: (u8, char) = (8, 'f');
pub const PF9: (u8, char) = (9, 'f');
pub const PF10: (u8, char) = (10, 'f');
pub const PF11: (u8, char) = (11, 'f');
pub const PF12: (u8, char) = (12, 'f');
pub const PF13: (u8, char) = (13, 'f');
pub const PF14: (u8, char) = (14, 'f');
pub const PF15: (u8, char) = (15, 'f');

pub const PG0: (u8, char) = (0, 'g');
pub const PG1: (u8, char) = (1, 'g');
pub const PG2: (u8, char) = (2, 'g');
pub const PG3: (u8, char) = (3, 'g');
pub const PG4: (u8, char) = (4, 'g');
pub const PG5: (u8, char) = (5, 'g');
pub const PG6: (u8, char) = (6, 'g');
pub const PG7: (u8, char) = (7, 'g');
pub const PG8: (u8, char) = (8, 'g');
pub const PG9: (u8, char) = (9, 'g');
pub const PG10: (u8, char) = (10, 'g');
pub const PG11: (u8, char) = (11, 'g');
pub const PG12: (u8, char) = (12, 'g');
pub const PG13: (u8, char) = (13, 'g');
pub const PG14: (u8, char) = (14, 'g');
pub const PG15: (u8, char) = (15, 'g');

pub const PH0: (u8, char) = (0, 'h');
pub const PH1: (u8, char) = (1, 'h');


// Register pointers ==============================================================================
pub const RCC_PTR: *const stm32f446::rcc::RegisterBlock = stm32f446::RCC::ptr();
pub const SYSTICK_PTR: *const stm32f446::stk::RegisterBlock = stm32f446::STK::ptr();
pub const GPIOA_PTR: *const stm32f446::gpioa::RegisterBlock = stm32f446::GPIOA::ptr();
pub const GPIOB_PTR: *const stm32f446::gpiob::RegisterBlock = stm32f446::GPIOB::ptr();
pub const GPIOC_PTR: *const stm32f446::gpioh::RegisterBlock = stm32f446::GPIOC::ptr();
pub const ADCC_PTR: *const stm32f446::adc_common::RegisterBlock = stm32f446::ADC_COMMON::ptr();
pub const ADC1_PTR: *const stm32f446::adc1::RegisterBlock = stm32f446::ADC1::ptr();
pub const ADC2_PTR: *const stm32f446::adc1::RegisterBlock = stm32f446::ADC2::ptr();
pub const ADC3_PTR: *const stm32f446::adc1::RegisterBlock = stm32f446::ADC3::ptr();
pub const DAC_PTR: *const stm32f446::dac::RegisterBlock = stm32f446::DAC::ptr();
pub const USART1_PTR: *const stm32f446::usart1::RegisterBlock = stm32f446::USART1::ptr();
pub const USART2_PTR: *const stm32f446::usart1::RegisterBlock = stm32f446::USART2::ptr();
pub const USART3_PTR: *const stm32f446::usart1::RegisterBlock = stm32f446::USART3::ptr();
pub const UART4_PTR: *const stm32f446::uart4::RegisterBlock = stm32f446::UART4::ptr();
pub const UART5_PTR: *const stm32f446::uart4::RegisterBlock = stm32f446::UART5::ptr();
pub const USART6_PTR: *const stm32f446::usart1::RegisterBlock = stm32f446::USART6::ptr();
pub const TIM2_PTR: *const stm32f446::tim2::RegisterBlock = stm32f446::TIM2::ptr();
pub const TIM3_PTR: *const stm32f446::tim3::RegisterBlock = stm32f446::TIM3::ptr();
pub const TIM4_PTR: *const stm32f446::tim3::RegisterBlock = stm32f446::TIM4::ptr();
pub const TIM5_PTR: *const stm32f446::tim5::RegisterBlock = stm32f446::TIM5::ptr();
pub const TIM9_PTR: *const stm32f446::tim9::RegisterBlock = stm32f446::TIM9::ptr();
pub const TIM10_PTR: *const stm32f446::tim10::RegisterBlock = stm32f446::TIM10::ptr();
pub const TIM11_PTR: *const stm32f446::tim11::RegisterBlock = stm32f446::TIM11::ptr();
pub const TIM12_PTR: *const stm32f446::tim9::RegisterBlock = stm32f446::TIM12::ptr();
pub const TIM13_PTR: *const stm32f446::tim10::RegisterBlock = stm32f446::TIM13::ptr();
pub const TIM14_PTR: *const stm32f446::tim10::RegisterBlock = stm32f446::TIM14::ptr();


// Pin config struct ==============================================================================
pub static mut CONFIG: Config = Config{
  pin: Vec::new(),
  config: Vec::new(),
  alternate: Vec::new(),
  analog: Vec::new()
};

// Analog pin config map ==========================================================================
pub static mut ADC_MAP: ADCMap = ADCMap{
  pin: [PA0, PA1, PA2, PA3, PA4, PA5, PA6, PA7, PB0, PB1, PC0, PC1, PC2, PC3, PC4, PC5],
  channel: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15],
  active: [false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false]
};


// DAC pin config map =============================================================================
pub static mut DAC_MAP: (bool, bool) = (false, false);


// Millis counter =================================================================================
pub static mut TIME_COUNTER: usize = 0;


// UART pin config map ============================================================================
pub static mut UART_MAP: UARTMap = UARTMap{
  tx_pin: [PA9, PB6, PA2, PB10, PC10, PA0, PC6],
  rx_pin: [PA10, PB7, PA3, PB11, PC11, PA1, PC7],
  channel: [1, 1, 2, 3, 3, 4, 6],
  active: [false, false, false, false, false, false, false]
};
