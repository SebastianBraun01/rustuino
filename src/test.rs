// Dieser entwurf ist jetzt ein Kompromiss von den Ideen, die wir besprochen haben. Ich bin der Meinung, dass wir mit Traits
// nur definieren sollten, welche pins für die Funktion verfügbar sind, unabhängig davon ob sie schon als was anderes
// konfiguriert sind. Die traits enthalten funtionen, die die Funktion eines Pins setzten oder überprüfen. Sollte ein pin
// falsch oder nicht konpiguriert sein, fängt das ein panic auf. Pins werden nie innerhalb eines Programms umgestellt,
// sondern nur am anfang konfiguriert. Ich finde das ist ein guter Kompromiss.

#[derive(Debug, PartialEq, Eq)]
pub enum Config {
  None, Input, Output, Analog,
  PWM, UART, I2C, SPI
}

pub trait GPIO {
  fn set_gpio(&self, config: Config);
  fn check_gpio(&self, config: Config);
}

pub trait Analog {
  fn set_analog(&self);
  fn check_analog(&self);
}

pub trait PWM {
  fn set_pwm(&self);
  fn check_pwm(&self);
}

pub trait UART {
  fn set_uart(&self);
  fn check_uart(&self);
}

pub trait I2C {
  fn set_i2c(&self);
  fn check_i2c(&self);
}

pub trait SPI {
  fn set_spi(&self);
  fn check_spi(&self);
}


macro_rules! gen_impls {
  ($([$pin:ident; $($config:ident),*]),+) => {
    use paste::paste;
    
    paste!{
      $(
        #[derive(Debug)]
        struct [<$pin Struct>](Config);
        
        impl GPIO for [<$pin Struct>] {
          fn set_gpio(&self, config: Config) {
            if self.0 == config {panic!("Pin {:?} is already configured as {:?}", self, config);}
            else {self.0 = config;}
          }
          fn check_gpio(&self, config: Config) {
            if self.0 != config {panic!("Pin {:?} is not configured as {:?}", self, config);}
          }
        }
        
        $(
          impl $config for [<$pin Struct>] {
            fn [<set_ $config:lower>](&self) {
              if self.0 == Config::$config {panic!("Pin {:?} is already configured as {:?}", self, Config::$config);}
              else {self.0 = Config::$config;}
            }
            fn [<check_ $config:lower>](&self) {
              if self.0 != Config::$config {panic!("Pin {:?} is not configured as {:?}", self, Config::$config);}
            }
          }
        )*
        
        pub static mut $pin: [<$pin Struct>] = [<$pin Struct>](Config::None);
      )+
    }
  };
}

// Format [$pin; $funktion, $funktion, ...],
gen_impls![
[PA0; Analog, PWM],
[PA1; Analog, PWM]
];
