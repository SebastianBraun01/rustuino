use crate::common::*;
use crate::include::{I2C_MAP, I2C_CONF};
use heapless::Vec;


// Initialisation function ========================================================================
pub fn i2c_init(scl_pin: (char, u8), sda_pin: (char, u8), pullup: bool) -> Option<I2cCore> {
  let peripheral_ptr;
  unsafe {peripheral_ptr = stm32f4::stm32f446::Peripherals::steal();}
  let rcc = &peripheral_ptr.RCC;

  let core: u8;

  if I2C_MAP.scl_pins.contains(&scl_pin) && I2C_MAP.sda_pins.contains(&sda_pin) {
    let index = I2C_MAP.scl_pins.iter().zip(I2C_MAP.sda_pins.iter()).position(|i| i == (&scl_pin, &sda_pin)).unwrap();
    core = I2C_MAP.core[index];
    unsafe {I2C_CONF[core as usize - 1] = true;}
  }
  else {
    rtt_target::rprintln!("These pins are not available for I2C communication! | ::i2c_init(...)");
    return None;
  }

  // Setup Variablen
  let bus_freq = 16;
  let i2c_freq = 100;

  let i2c_t: u32 = (bus_freq * 1000000) / (2 * i2c_freq * 1000);
  let rise_t: u32 = (1 / 1000000) / (1 / bus_freq * 1000000) + 1;

  i2c_setup_gpio(scl_pin.0, scl_pin.1, pullup);
  i2c_setup_gpio(sda_pin.0, sda_pin.1, pullup);
  
  match core {
    1 => {
      let i2c1 = &peripheral_ptr.I2C1;
      rcc.apb1enr.modify(|_, w| w.i2c1en().enabled());
      i2c1.cr2.modify(|_, w| unsafe {w.freq().bits(bus_freq as u8)});
      i2c1.ccr.modify(|_, w| unsafe {w.ccr().bits(i2c_t as u16)});
      i2c1.trise.write(|w| w.trise().bits(rise_t as u8));
      i2c1.cr1.modify(|_, w| w.ack().set_bit());
      // i2c1.cr2.modify(|_, w| {
      //   w.itbufen().enabled();
      //   w.itevten().enabled()
      // });
      i2c1.cr1.modify(|_, w| w.pe().enabled());
    },
    2 => {
      let i2c2 = &peripheral_ptr.I2C2;
      rcc.apb1enr.modify(|_, w| w.i2c2en().enabled());
      i2c2.cr2.modify(|_, w| unsafe {w.freq().bits(bus_freq as u8)});
      i2c2.ccr.modify(|_, w| unsafe {w.ccr().bits(i2c_t as u16)});
      i2c2.trise.write(|w| w.trise().bits(rise_t as u8));
      i2c2.cr1.modify(|_, w| w.ack().set_bit());
      // i2c2.cr2.modify(|_, w| {
      //   w.itbufen().enabled();
      //   w.itevten().enabled()
      // });
      i2c2.cr1.modify(|_, w| w.pe().enabled());
    },
    3 => {
      let i2c3 = &peripheral_ptr.I2C3;
      rcc.apb1enr.modify(|_, w| w.i2c3en().enabled());
      i2c3.cr2.modify(|_, w| unsafe {w.freq().bits(bus_freq as u8)});
      i2c3.ccr.modify(|_, w| unsafe {w.ccr().bits(i2c_t as u16)});
      i2c3.trise.write(|w| w.trise().bits(rise_t as u8));
      i2c3.cr1.modify(|_, w| w.ack().set_bit());
      // i2c3.cr2.modify(|_, w| {
      //   w.itbufen().enabled();
      //   w.itevten().enabled()
      // });
      i2c3.cr1.modify(|_, w| w.pe().enabled());
    },
    _ => panic!("I2C{} is not a valid core! | ::i2c_init(...)", core)
  };

  return Some(I2cCore {
    scl: scl_pin,
    sda: sda_pin,
    core,
    pullup
  });
}


// Communication functions ========================================================================
impl I2C for I2cCore {
  fn send_bytes<const N: usize>(&self, addr: u8, data: &Vec<u8, N>) {
    let peripheral_ptr;
    unsafe {peripheral_ptr = stm32f4::stm32f446::Peripherals::steal();}
    let _sr: u32;
  
    match self.core {
      1 => {
        let i2c1 = &peripheral_ptr.I2C1;
        i2c1.cr1.write(|w| w.start().set_bit());
        while i2c1.sr1.read().sb().is_no_start() == true {}
        i2c1.dr.write(|w| w.dr().bits(addr));
        while i2c1.sr1.read().addr().is_not_match() == true {}
        _sr = i2c1.sr2.read().bits();
        for byte in data.iter() {
          i2c1.dr.write(|w| w.dr().bits(byte.clone()));
          while i2c1.sr1.read().tx_e().is_not_empty() == true {}
        }
        i2c1.cr1.write(|w| w.stop().set_bit());
      },
      2 => {
        let i2c2 = &peripheral_ptr.I2C2;
        i2c2.cr1.write(|w| w.start().set_bit());
        while i2c2.sr1.read().sb().is_no_start() == true {}
        i2c2.dr.write(|w| w.dr().bits(addr));
        while i2c2.sr1.read().addr().is_not_match() == true {}
        _sr= i2c2.sr2.read().bits();
        for byte in data.iter() {
          i2c2.dr.write(|w| w.dr().bits(byte.clone()));
          while i2c2.sr1.read().tx_e().is_not_empty() == true {}
        }
        i2c2.cr1.write(|w| w.stop().set_bit());
      },
      3 => {
        let i2c3 = &peripheral_ptr.I2C3;
        i2c3.cr1.write(|w| w.start().set_bit());
        while i2c3.sr1.read().sb().is_no_start() == true {}
        i2c3.dr.write(|w| w.dr().bits(addr));
        while i2c3.sr1.read().addr().is_not_match() == true {}
        _sr = i2c3.sr2.read().bits();
        for byte in data.iter() {
          i2c3.dr.write(|w| w.dr().bits(byte.clone()));
          while i2c3.sr1.read().tx_e().is_not_empty() == true {}
        }
        i2c3.cr1.write(|w| w.stop().set_bit());
      },
      _ => panic!("I2C{} is not a valid core! | .send_bytes(...)", self.core)
    };
  }

  fn recieve_bytes<const N: usize>(&self, addr: u8, vec: &mut Vec<u8, N>, nbytes: u8) {
    let peripheral_ptr;
    unsafe {peripheral_ptr = stm32f4::stm32f446::Peripherals::steal();}
    let _sr: u32;
  
    match self.core {
      1 => {
        let i2c1 = &peripheral_ptr.I2C1;
        i2c1.cr1.write(|w| w.start().set_bit());
        while i2c1.sr1.read().sb().is_no_start() == true {}
        i2c1.dr.write(|w| w.dr().bits(addr + 1));
        while i2c1.sr1.read().addr().is_not_match() == true {}
        if nbytes > 2 {
          _sr = i2c1.sr2.read().bits();
          for i in 0..nbytes {
            if i >= (nbytes - 2) {
              while i2c1.sr1.read().btf().is_not_finished() == true {}
              i2c1.cr1.write(|w| w.stop().set_bit());
              vec.push(i2c1.dr.read().dr().bits()).unwrap();
              while i2c1.sr1.read().rx_ne().is_empty() == true {}
              vec.push(i2c1.dr.read().dr().bits()).unwrap();
              break;
            }
            while i2c1.sr1.read().btf().is_not_finished() == true {}
            vec.push(i2c1.dr.read().dr().bits()).unwrap();
          }
        }
        else {
          i2c1.cr1.modify(|_, w| w.pos().set_bit());
          _sr = i2c1.sr2.read().bits();
          while i2c1.sr1.read().btf().is_not_finished() == true {}
          i2c1.cr1.write(|w| w.stop().set_bit());
          vec.push(i2c1.dr.read().dr().bits()).unwrap();
          while i2c1.sr1.read().rx_ne().is_empty() == true {}
          vec.push(i2c1.dr.read().dr().bits()).unwrap();
        }
      },
      2 => {
        let i2c2 = &peripheral_ptr.I2C2;
        i2c2.cr1.write(|w| w.start().set_bit());
        while i2c2.sr1.read().sb().is_no_start() == true {}
        i2c2.dr.write(|w| w.dr().bits(addr + 1));
        while i2c2.sr1.read().addr().is_not_match() == true {}
        if nbytes > 2 {
          _sr = i2c2.sr2.read().bits();
          for i in 0..nbytes {
            if i >= (nbytes - 2) {
              while i2c2.sr1.read().btf().is_not_finished() == true {}
              i2c2.cr1.write(|w| w.stop().set_bit());
              vec.push(i2c2.dr.read().dr().bits()).unwrap();
              while i2c2.sr1.read().rx_ne().is_empty() == true {}
              vec.push(i2c2.dr.read().dr().bits()).unwrap();
              break;
            }
            while i2c2.sr1.read().btf().is_not_finished() == true {}
            vec.push(i2c2.dr.read().dr().bits()).unwrap();
          }
        }
        else {
          i2c2.cr1.modify(|_, w| w.pos().set_bit());
          _sr = i2c2.sr2.read().bits();
          while i2c2.sr1.read().btf().is_not_finished() == true {}
          i2c2.cr1.write(|w| w.stop().set_bit());
          vec.push(i2c2.dr.read().dr().bits()).unwrap();
          while i2c2.sr1.read().rx_ne().is_empty() == true {}
          vec.push(i2c2.dr.read().dr().bits()).unwrap();
        }
      },
      3 => {
        let i2c3 = &peripheral_ptr.I2C3;
        i2c3.cr1.write(|w| w.start().set_bit());
        while i2c3.sr1.read().sb().is_no_start() == true {}
        i2c3.dr.write(|w| w.dr().bits(addr + 1));
        while i2c3.sr1.read().addr().is_not_match() == true {}
        if nbytes > 2 {
          _sr = i2c3.sr2.read().bits();
          for i in 0..nbytes {
            if i >= (nbytes - 2) {
              while i2c3.sr1.read().btf().is_not_finished() == true {}
              i2c3.cr1.write(|w| w.stop().set_bit());
              vec.push(i2c3.dr.read().dr().bits()).unwrap();
              while i2c3.sr1.read().rx_ne().is_empty() == true {}
              vec.push(i2c3.dr.read().dr().bits()).unwrap();
              break;
            }
            while i2c3.sr1.read().btf().is_not_finished() == true {}
            vec.push(i2c3.dr.read().dr().bits()).unwrap();
          }
        }
        else {
          i2c3.cr1.modify(|_, w| w.pos().set_bit());
          _sr = i2c3.sr2.read().bits();
          while i2c3.sr1.read().btf().is_not_finished() == true {}
          i2c3.cr1.write(|w| w.stop().set_bit());
          vec.push(i2c3.dr.read().dr().bits()).unwrap();
          while i2c3.sr1.read().rx_ne().is_empty() == true {}
          vec.push(i2c3.dr.read().dr().bits()).unwrap();
        }
      },
      _ => panic!("I2C{} is not a valid core! | .recieve_bytes(...)", self.core)
    };
  }
}


// Helper functions ===============================================================================
fn i2c_setup_gpio(block: char, pin: u8, pullup: bool) {
  let peripheral_ptr;
  unsafe {peripheral_ptr = stm32f4::stm32f446::Peripherals::steal();}
  let rcc = &peripheral_ptr.RCC;

  match block {
    'a' => {
      let gpioa = &peripheral_ptr.GPIOA;
      rcc.ahb1enr.modify(|_, w| w.gpioaen().enabled());
      gpioa.moder.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin)) | (2 << (2 * pin)))});
      gpioa.otyper.modify(|r, w| unsafe {w.bits(r.bits() | (1 << pin))});
      if pullup == true {gpioa.pupdr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin) | (2 << (2 * pin))))});}
      if pin > 7 {gpioa.afrh.modify(|r, w| unsafe {w.bits(r.bits() | (4 << (4 * (pin - 8))))});}
      else {gpioa.afrl.modify(|r, w| unsafe {w.bits(r.bits() | (4 << (4 * pin)))});}
    },
    'b' => {
      let gpiob = &peripheral_ptr.GPIOB;
      rcc.ahb1enr.modify(|_, w| w.gpioben().enabled());
      gpiob.moder.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin)) | (2 << (2 * pin)))});
      gpiob.otyper.modify(|r, w| unsafe {w.bits(r.bits() | (1 << pin))});
      if pullup == true {gpiob.pupdr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin) | (2 << (2 * pin))))});}
      if pin > 7 {gpiob.afrh.modify(|r, w| unsafe {w.bits(r.bits() | (4 << (4 * (pin - 8))))});}
      else {gpiob.afrl.modify(|r, w| unsafe {w.bits(r.bits() | (4 << (4 * pin)))});}
    },
    'c' => {
      let gpioc = &peripheral_ptr.GPIOC;
      rcc.ahb1enr.modify(|_, w| w.gpiocen().enabled());
      gpioc.moder.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin)) | (2 << (2 * pin)))});
      gpioc.otyper.modify(|r, w| unsafe {w.bits(r.bits() | (1 << pin))});
      if pullup == true {gpioc.pupdr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin) | (2 << (2 * pin))))});}
      if pin > 7 {gpioc.afrh.modify(|r, w| unsafe {w.bits(r.bits() | (4 << (4 * (pin - 8))))});}
      else {gpioc.afrl.modify(|r, w| unsafe {w.bits(r.bits() | (4 << (4 * pin)))});}
    },
    'f' => {
      let gpiof = &peripheral_ptr.GPIOF;
      rcc.ahb1enr.modify(|_, w| w.gpiofen().enabled());
      gpiof.moder.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin)) | (2 << (2 * pin)))});
      gpiof.otyper.modify(|r, w| unsafe {w.bits(r.bits() | (1 << pin))});
      if pullup == true {gpiof.pupdr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin) | (2 << (2 * pin))))});}
      if pin > 7 {gpiof.afrh.modify(|r, w| unsafe {w.bits(r.bits() | (4 << (4 * (pin - 8))))});}
      else {gpiof.afrl.modify(|r, w| unsafe {w.bits(r.bits() | (4 << (4 * pin)))});}
    },
    _   => panic!("P{}{} is not available for i2c transmission! | i2c_setup_gpio(...)", block.to_uppercase(), pin)
  };
}
