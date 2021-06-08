#[derive(PartialEq, Eq)]
pub enum Config {
  None, Input, Output, Analog,
  PWM, UART, I2C, SPI
}

pub struct Pin(u8, char, u8, Config);

impl Pin {
  fn has_mode(&self, mode: Config) -> bool {
    let result = match mode {
      Config::None => true,
      Config::Input => true,
      Config::Output => true,
      Config::Analog => {
        if self.2 & 0x01 != 0 {true}
        else {false}
      },
      Config::PWM => {
        if self.2 & 0x02 != 0 {true}
        else {false}
      },
      Config::UART => {
        if self.2 & 0x04 != 0 {true}
        else {false}
      },
      Config::I2C => {
        if self.2 & 0x08 != 0 {true}
        else {false}
      },
      Config::SPI => {
        if self.2 & 0x10 != 0 {true}
        else {false}
      }
    };

    return result;
  }

  fn is_set_as(&self) -> Config {
    return self.3;
  }

  fn can_be_set_as(&self, mode: Config) -> bool {
    if self.3 == mode {true}
    else {false}
  }
}


// Pin Definitions ================================================================================
pub const PA0: Pin = Pin(0, 'a', 0, Config::None);
pub const PA1: Pin = Pin(1, 'a', 0, Config::None);
pub const PA2: Pin = Pin(2, 'a', 0, Config::None);
pub const PA3: Pin = Pin(3, 'a', 0, Config::None);
pub const PA4: Pin = Pin(4, 'a', 0, Config::None);
pub const PA5: Pin = Pin(5, 'a', 0, Config::None);
pub const PA6: Pin = Pin(6, 'a', 0, Config::None);
pub const PA7: Pin = Pin(7, 'a', 0, Config::None);
pub const PA8: Pin = Pin(8, 'a', 0, Config::None);
pub const PA9: Pin = Pin(9, 'a', 0, Config::None);
pub const PA10: Pin = Pin(10, 'a', 0, Config::None);
pub const PA11: Pin = Pin(11, 'a', 0, Config::None);
pub const PA12: Pin = Pin(12, 'a', 0, Config::None);
pub const PA13: Pin = Pin(13, 'a', 0, Config::None);
pub const PA14: Pin = Pin(14, 'a', 0, Config::None);
pub const PA15: Pin = Pin(15, 'a', 0, Config::None);

pub const PB0: Pin = Pin(0, 'b', 0, Config::None);
pub const PB1: Pin = Pin(1, 'b', 0, Config::None);
pub const PB2: Pin = Pin(2, 'b', 0, Config::None);
pub const PB3: Pin = Pin(3, 'b', 0, Config::None);
pub const PB4: Pin = Pin(4, 'b', 0, Config::None);
pub const PB5: Pin = Pin(5, 'b', 0, Config::None);
pub const PB6: Pin = Pin(6, 'b', 0, Config::None);
pub const PB7: Pin = Pin(7, 'b', 0, Config::None);
pub const PB8: Pin = Pin(8, 'b', 0, Config::None);
pub const PB9: Pin = Pin(9, 'b', 0, Config::None);
pub const PB10: Pin = Pin(10, 'b', 0, Config::None);
pub const PB11: Pin = Pin(11, 'b', 0, Config::None);
pub const PB12: Pin = Pin(12, 'b', 0, Config::None);
pub const PB13: Pin = Pin(13, 'b', 0, Config::None);
pub const PB14: Pin = Pin(14, 'b', 0, Config::None);
pub const PB15: Pin = Pin(15, 'b', 0, Config::None);

pub const PC0: Pin = Pin(0, 'c', 0, Config::None);
pub const PC1: Pin = Pin(1, 'c', 0, Config::None);
pub const PC2: Pin = Pin(2, 'c', 0, Config::None);
pub const PC3: Pin = Pin(3, 'c', 0, Config::None);
pub const PC4: Pin = Pin(4, 'c', 0, Config::None);
pub const PC5: Pin = Pin(5, 'c', 0, Config::None);
pub const PC6: Pin = Pin(6, 'c', 0, Config::None);
pub const PC7: Pin = Pin(7, 'c', 0, Config::None);
pub const PC8: Pin = Pin(8, 'c', 0, Config::None);
pub const PC9: Pin = Pin(9, 'c', 0, Config::None);
pub const PC10: Pin = Pin(10, 'c', 0, Config::None);
pub const PC11: Pin = Pin(11, 'c', 0, Config::None);
pub const PC12: Pin = Pin(12, 'c', 0, Config::None);
pub const PC13: Pin = Pin(13, 'c', 0, Config::None);
pub const PC14: Pin = Pin(14, 'c', 0, Config::None);
pub const PC15: Pin = Pin(15, 'c', 0, Config::None);

pub const PD0: Pin = Pin(0, 'd', 0, Config::None);
pub const PD1: Pin = Pin(1, 'd', 0, Config::None);
pub const PD2: Pin = Pin(2, 'd', 0, Config::None);
pub const PD3: Pin = Pin(3, 'd', 0, Config::None);
pub const PD5: Pin = Pin(5, 'd', 0, Config::None);
pub const PD6: Pin = Pin(6, 'd', 0, Config::None);
pub const PD7: Pin = Pin(7, 'd', 0, Config::None);
pub const PD8: Pin = Pin(8, 'd', 0, Config::None);
pub const PD9: Pin = Pin(9, 'd', 0, Config::None);
pub const PD10: Pin = Pin(10, 'd', 0, Config::None);
pub const PD11: Pin = Pin(11, 'd', 0, Config::None);
pub const PD12: Pin = Pin(12, 'd', 0, Config::None);
pub const PD13: Pin = Pin(13, 'd', 0, Config::None);
pub const PD14: Pin = Pin(14, 'd', 0, Config::None);
pub const PD15: Pin = Pin(15, 'd', 0, Config::None);

pub const PE0: Pin = Pin(0, 'e', 0, Config::None);
pub const PE1: Pin = Pin(1, 'e', 0, Config::None);
pub const PE2: Pin = Pin(2, 'e', 0, Config::None);
pub const PE3: Pin = Pin(3, 'e', 0, Config::None);
pub const PE4: Pin = Pin(4, 'e', 0, Config::None);
pub const PE5: Pin = Pin(5, 'e', 0, Config::None);
pub const PE6: Pin = Pin(6, 'e', 0, Config::None);
pub const PE7: Pin = Pin(7, 'e', 0, Config::None);
pub const PE8: Pin = Pin(8, 'e', 0, Config::None);
pub const PE9: Pin = Pin(9, 'e', 0, Config::None);
pub const PE10: Pin = Pin(10, 'e', 0, Config::None);
pub const PE11: Pin = Pin(11, 'e', 0, Config::None);
pub const PE12: Pin = Pin(12, 'e', 0, Config::None);
pub const PE13: Pin = Pin(13, 'e', 0, Config::None);
pub const PE14: Pin = Pin(14, 'e', 0, Config::None);
pub const PE15: Pin = Pin(15, 'e', 0, Config::None);

pub const PF0: Pin = Pin(0, 'f', 0, Config::None);
pub const PF1: Pin = Pin(1, 'f', 0, Config::None);
pub const PF2: Pin = Pin(2, 'f', 0, Config::None);
pub const PF3: Pin = Pin(3, 'f', 0, Config::None);
pub const PF4: Pin = Pin(4, 'f', 0, Config::None);
pub const PF5: Pin = Pin(5, 'f', 0, Config::None);
pub const PF6: Pin = Pin(6, 'f', 0, Config::None);
pub const PF7: Pin = Pin(7, 'f', 0, Config::None);
pub const PF8: Pin = Pin(8, 'f', 0, Config::None);
pub const PF9: Pin = Pin(9, 'f', 0, Config::None);
pub const PF10: Pin = Pin(10, 'f', 0, Config::None);
pub const PF11: Pin = Pin(11, 'f', 0, Config::None);
pub const PF12: Pin = Pin(12, 'f', 0, Config::None);
pub const PF13: Pin = Pin(13, 'f', 0, Config::None);
pub const PF14: Pin = Pin(14, 'f', 0, Config::None);
pub const PF15: Pin = Pin(15, 'f', 0, Config::None);

pub const PG0: Pin = Pin(0, 'g', 0, Config::None);
pub const PG1: Pin = Pin(1, 'g', 0, Config::None);
pub const PG2: Pin = Pin(2, 'g', 0, Config::None);
pub const PG3: Pin = Pin(3, 'g', 0, Config::None);
pub const PG4: Pin = Pin(4, 'g', 0, Config::None);
pub const PG5: Pin = Pin(5, 'g', 0, Config::None);
pub const PG6: Pin = Pin(6, 'g', 0, Config::None);
pub const PG7: Pin = Pin(7, 'g', 0, Config::None);
pub const PG8: Pin = Pin(8, 'g', 0, Config::None);
pub const PG9: Pin = Pin(9, 'g', 0, Config::None);
pub const PG10: Pin = Pin(10, 'g', 0, Config::None);
pub const PG11: Pin = Pin(11, 'g', 0, Config::None);
pub const PG12: Pin = Pin(12, 'g', 0, Config::None);
pub const PG13: Pin = Pin(13, 'g', 0, Config::None);
pub const PG14: Pin = Pin(14, 'g', 0, Config::None);
pub const PG15: Pin = Pin(15, 'g', 0, Config::None);

pub const PH0: Pin = Pin(0, 'h', 0, Config::None);
pub const PH1: Pin = Pin(1, 'h', 0, Config::None);
