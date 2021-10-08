use crate::include::{I2cError, ProgError, I2C_MAP, PIN_CONF};
use crate::gpio::{pinmode_alternate_function, open_drain, set_bias, GpioBias::Pullup, Pin, AlternateFunction};
use heapless::Vec;
use rtt_target::rprintln;

const BUS_FREQ: u32 = 16000000;
const I2C_FREQ: u32 = 100000;

pub struct I2C<const N: usize> {
  core: u8,
  _scl_pin: Pin<AlternateFunction>,
  _sda_pin: Pin<AlternateFunction>,
  tx_buffer: Vec<u8, N>,
  rx_buffer: Vec<u8, N>,
  tx_addr: u8,
  transmitting: bool
}

impl<const N: usize> I2C<N> {
  pub fn new(core: u8, scl_pin: (char, u8), sda_pin: (char, u8), pullup: bool, addr: u8) -> Result<Self, ProgError> {
    let peripheral_ptr;
    unsafe {peripheral_ptr = stm32f4::stm32f446::Peripherals::steal();}
    let rcc = &peripheral_ptr.RCC;
  
    if I2C_MAP.scl_pins.iter().zip(I2C_MAP.sda_pins.iter()).zip(I2C_MAP.cores.iter()).any(|i| i == ((&scl_pin, &sda_pin), &core)) == false {
      rprintln!("These pins are not available for I2C communication! | I2C::new()");
      return Err(ProgError::InvalidConfiguration);
    }

    unsafe {
      if PIN_CONF.contains(&scl_pin) || PIN_CONF.contains(&sda_pin) {
        rprintln!("These pins are already configured for another function! | I2C::new()");
        return Err(ProgError::InvalidConfiguration);
      }
      else {
        PIN_CONF.push(scl_pin).expect("Could not store pin number! | I2C::new()");
        PIN_CONF.push(sda_pin).expect("Could not store pin number! | I2C::new()");
      }
    }

    let scl = match pinmode_alternate_function(scl_pin, 4) {
      Ok(value) => {
        open_drain(&value, true);
        value
      },
      Err(_) => return Err(ProgError::Internal)
    };

    let sda = match pinmode_alternate_function(sda_pin, 4) {
      Ok(value) => {
        open_drain(&value, true);
        value
      },
      Err(_) => return Err(ProgError::Internal)
    };

    if pullup == true {
      set_bias(&scl, Pullup);
      set_bias(&sda, Pullup);
    }

    let (ccr_t, rise_t) = calc_i2c_freq(I2C_FREQ);
    
    match core {
      1 => {
        let i2c1 = &peripheral_ptr.I2C1;
        if rcc.apb1enr.read().i2c1en().is_enabled() == true {
          rprintln!("I2C{} is already configured! | I2C::new()", core);
          return Err(ProgError::AlreadyConfigured);
        }
        rcc.apb1enr.modify(|_, w| w.i2c1en().enabled());
        i2c1.cr2.modify(|_, w| unsafe {w.freq().bits(BUS_FREQ as u8)});
        i2c1.ccr.modify(|_, w| unsafe {w.ccr().bits(ccr_t as u16)});
        i2c1.trise.write(|w| w.trise().bits(rise_t as u8));
        if addr > 0 {if addr > 0 {i2c1.oar1.modify(|_, w| w.add().bits((addr << 1).into()));}}
        i2c1.cr1.modify(|_, w| {
          w.ack().set_bit();
          w.pe().enabled()
        });
      },
      2 => {
        let i2c2 = &peripheral_ptr.I2C2;
        if rcc.apb1enr.read().i2c2en().is_enabled() == true {
          rprintln!("I2C{} is already configured! | I2C::new()", core);
          return Err(ProgError::AlreadyConfigured);
        }
        rcc.apb1enr.modify(|_, w| w.i2c2en().enabled());
        i2c2.cr2.modify(|_, w| unsafe {w.freq().bits(BUS_FREQ as u8)});
        i2c2.ccr.modify(|_, w| unsafe {w.ccr().bits(ccr_t as u16)});
        i2c2.trise.write(|w| w.trise().bits(rise_t as u8));
        if addr > 0 {i2c2.oar1.modify(|_, w| w.add().bits((addr << 1).into()));}
        i2c2.cr1.modify(|_, w| {
          w.ack().set_bit();
          w.pe().enabled()
        });
      },
      3 => {
        let i2c3 = &peripheral_ptr.I2C3;
        if rcc.apb1enr.read().i2c3en().is_enabled() == true {
          rprintln!("I2C{} is already configured! | I2C::new()", core);
          return Err(ProgError::AlreadyConfigured);
        }
        rcc.apb1enr.modify(|_, w| w.i2c3en().enabled());
        i2c3.cr2.modify(|_, w| unsafe {w.freq().bits(BUS_FREQ as u8)});
        i2c3.ccr.modify(|_, w| unsafe {w.ccr().bits(ccr_t as u16)});
        i2c3.trise.write(|w| w.trise().bits(rise_t as u8));
        if addr > 0 {i2c3.oar1.modify(|_, w| w.add().bits((addr << 1).into()));}
        i2c3.cr1.modify(|_, w| {
          w.ack().set_bit();
          w.pe().enabled()
        });
      },
      _ => panic!("I2C{} is not a valid core! | I2C::new()", core)
    };

    return Ok(Self {
      core,
      _scl_pin: scl,
      _sda_pin: sda,
      tx_buffer: Vec::new(),
      rx_buffer: Vec::new(),
      tx_addr: 0,
      transmitting: false
    });
  }

  pub fn end(self) {
    let peripheral_ptr;
    unsafe {peripheral_ptr = stm32f4::stm32f446::Peripherals::steal();}
    let rcc = &peripheral_ptr.RCC;

    match self.core {
      1 => {
        let i2c1 = &peripheral_ptr.I2C1;
        rcc.apb1enr.modify(|_, w| w.i2c1en().disabled());
        i2c1.cr1.reset();
        i2c1.cr2.reset();
        i2c1.ccr.reset();
        i2c1.trise.reset();
        i2c1.oar1.reset(); 
      },
      2 => {
        let i2c2 = &peripheral_ptr.I2C2;
        rcc.apb1enr.modify(|_, w| w.i2c2en().disabled());
        i2c2.cr1.reset();
        i2c2.cr2.reset();
        i2c2.ccr.reset();
        i2c2.trise.reset();
        i2c2.oar1.reset(); 
      },
      3 => {
        let i2c3 = &peripheral_ptr.I2C3;
        rcc.apb1enr.modify(|_, w| w.i2c3en().disabled());
        i2c3.cr1.reset();
        i2c3.cr2.reset();
        i2c3.ccr.reset();
        i2c3.trise.reset();
        i2c3.oar1.reset(); 
      },
      _ => panic!("I2C{} is not a valid core! | I2C::new()", self.core)
    };
  }

  pub fn begin_transmission(&mut self, addr: u8) {
    self.transmitting = true;
    self.tx_addr = addr << 1;
    self.tx_buffer.clear();
  }

  pub fn write(&mut self, data: u8) -> Result<(), ()> {
    if self.transmitting == false {return Err(());}

    if self.tx_buffer.push(data).is_err() {return Err(());}
    else {return Ok(());}
  }

  pub fn end_transmission(&mut self, stop: bool) -> Result<(), I2cError> {
    let peripheral_ptr;
    unsafe {peripheral_ptr = stm32f4::stm32f446::Peripherals::steal();}
    let _sr: u32;
  
    match self.core {
      1 => {
        let i2c1 = &peripheral_ptr.I2C1;
        i2c1.cr1.write(|w| w.start().set_bit());
        while i2c1.sr1.read().sb().is_no_start() == true {}
        i2c1.dr.write(|w| w.dr().bits(self.tx_addr));
        while i2c1.sr1.read().addr().is_not_match() == true {
          if let Err(error) = scan_i2c_error(i2c1.sr1.read().bits() as u16) {return Err(error);}
        }
        _sr = i2c1.sr2.read().bits();
        for byte in self.tx_buffer.iter() {
          i2c1.dr.write(|w| w.dr().bits(byte.clone()));
          while i2c1.sr1.read().tx_e().is_not_empty() == true {
            if let Err(error) = scan_i2c_error(i2c1.sr1.read().bits() as u16) {return Err(error);}
          }
        }
        if stop == true {i2c1.cr1.write(|w| w.stop().set_bit());}
        else {i2c1.cr1.write(|w| w.start().set_bit());}
      },
      2 => {
        let i2c2 = &peripheral_ptr.I2C2;
        i2c2.cr1.write(|w| w.start().set_bit());
        while i2c2.sr1.read().sb().is_no_start() == true {}
        i2c2.dr.write(|w| w.dr().bits(self.tx_addr));
        while i2c2.sr1.read().addr().is_not_match() == true {
          if let Err(error) = scan_i2c_error(i2c2.sr1.read().bits() as u16) {return Err(error);}
        }
        _sr= i2c2.sr2.read().bits();
        for byte in self.tx_buffer.iter() {
          i2c2.dr.write(|w| w.dr().bits(byte.clone()));
          while i2c2.sr1.read().tx_e().is_not_empty() == true {
            if let Err(error) = scan_i2c_error(i2c2.sr1.read().bits() as u16) {return Err(error);}
          }
        }
        if stop == true {i2c2.cr1.write(|w| w.stop().set_bit());}
        else {i2c2.cr1.write(|w| w.start().set_bit());}
      },
      3 => {
        let i2c3 = &peripheral_ptr.I2C3;
        i2c3.cr1.write(|w| w.start().set_bit());
        while i2c3.sr1.read().sb().is_no_start() == true {}
        i2c3.dr.write(|w| w.dr().bits(self.tx_addr));
        while i2c3.sr1.read().addr().is_not_match() == true {
          if let Err(error) = scan_i2c_error(i2c3.sr1.read().bits() as u16) {return Err(error);}
        }
        _sr = i2c3.sr2.read().bits();
        for byte in self.tx_buffer.iter() {
          i2c3.dr.write(|w| w.dr().bits(byte.clone()));
          while i2c3.sr1.read().tx_e().is_not_empty() == true {
            if let Err(error) = scan_i2c_error(i2c3.sr1.read().bits() as u16) {return Err(error);}
          }
        }
        if stop == true {i2c3.cr1.write(|w| w.stop().set_bit());}
        else {i2c3.cr1.write(|w| w.start().set_bit());}
      },
      _ => panic!("I2C{} is not a valid core! | .send_bytes(...)", self.core)
    };

    return Ok(());
  }

  pub fn request_bytes(&mut self, addr: u8, nbytes: u8, stop: bool) -> Result<usize, I2cError> {
    let peripheral_ptr;
    unsafe {peripheral_ptr = stm32f4::stm32f446::Peripherals::steal();}
    let _sr: u32;

    if nbytes == 0 || nbytes as usize > N {
      rprintln!("Cannot store number of bytes! ({}) | .request_bytes()", nbytes);
      return Err(I2cError::Prog(ProgError::InvalidConfiguration));
    }

    self.rx_buffer.clear();
  
    match self.core {
      1 => {
        let i2c1 = &peripheral_ptr.I2C1;
        i2c1.cr1.write(|w| w.start().set_bit());
        while i2c1.sr1.read().sb().is_no_start() == true {}
        i2c1.dr.write(|w| w.dr().bits((addr << 1) + 1));
        if nbytes == 1 {i2c1.cr1.modify(|_, w| w.ack().clear_bit());}
        while i2c1.sr1.read().addr().is_not_match() == true {
          if let Err(error) = scan_i2c_error(i2c1.sr1.read().bits() as u16) {return Err(error);}
        }

        if nbytes == 1 {
          _sr = i2c1.sr2.read().bits();
          if stop == true {i2c1.cr1.modify(|_, w| w.stop().set_bit());}
          else {i2c1.cr1.modify(|_, w| w.start().set_bit());}
          while i2c1.sr1.read().rx_ne().is_empty() == true {
            if let Err(error) = scan_i2c_error(i2c1.sr1.read().bits() as u16) {return Err(error);}
          }
          self.rx_buffer.push(i2c1.dr.read().dr().bits()).unwrap();
        }
        else if nbytes == 2 {
          i2c1.cr1.modify(|_, w| {
            w.ack().clear_bit();
            w.pos().set_bit()
          });
          _sr = i2c1.sr2.read().bits();
          while i2c1.sr1.read().btf().is_not_finished() == true {
            if let Err(error) = scan_i2c_error(i2c1.sr1.read().bits() as u16) {return Err(error);}
          }
          if stop == true {i2c1.cr1.modify(|_, w| w.stop().set_bit());}
          else {i2c1.cr1.modify(|_, w| w.start().set_bit());}
          while i2c1.sr1.read().rx_ne().is_empty() == true {}
          self.rx_buffer.push(i2c1.dr.read().dr().bits()).unwrap();
          while i2c1.sr1.read().rx_ne().is_empty() == true {}
          self.rx_buffer.push(i2c1.dr.read().dr().bits()).unwrap();
        }
        else {
          _sr = i2c1.sr2.read().bits();
          if nbytes > 3 {
            for _ in 0..(nbytes - 3) {
              while i2c1.sr1.read().btf().is_not_finished() == true {
                if let Err(error) = scan_i2c_error(i2c1.sr1.read().bits() as u16) {return Err(error);}
              }
              self.rx_buffer.push(i2c1.dr.read().dr().bits()).unwrap();
            }
          }
          while i2c1.sr1.read().btf().is_not_finished() == true {
            if let Err(error) = scan_i2c_error(i2c1.sr1.read().bits() as u16) {return Err(error);}
          }
          i2c1.cr1.modify(|_, w| w.ack().clear_bit());
          self.rx_buffer.push(i2c1.dr.read().dr().bits()).unwrap();
          while i2c1.sr1.read().btf().is_not_finished() == true {
            if let Err(error) = scan_i2c_error(i2c1.sr1.read().bits() as u16) {return Err(error);}
          }
          if stop == true {i2c1.cr1.modify(|_, w| w.stop().set_bit());}
          else {i2c1.cr1.modify(|_, w| w.start().set_bit());}
          while i2c1.sr1.read().rx_ne().is_empty() == true {}
          self.rx_buffer.push(i2c1.dr.read().dr().bits()).unwrap();
          while i2c1.sr1.read().rx_ne().is_empty() == true {}
          self.rx_buffer.push(i2c1.dr.read().dr().bits()).unwrap();
        }

        i2c1.cr1.modify(|_, w| w.ack().set_bit());
      },
      2 => {
        let i2c2 = &peripheral_ptr.I2C2;
        i2c2.cr1.write(|w| w.start().set_bit());
        while i2c2.sr1.read().sb().is_no_start() == true {}
        i2c2.dr.write(|w| w.dr().bits((addr << 1) + 1));
        if nbytes == 1 {i2c2.cr1.modify(|_, w| w.ack().clear_bit());}
        while i2c2.sr1.read().addr().is_not_match() == true {
          if let Err(error) = scan_i2c_error(i2c2.sr1.read().bits() as u16) {return Err(error);}
        }

        if nbytes == 1 {
          _sr = i2c2.sr2.read().bits();
          if stop == true {i2c2.cr1.modify(|_, w| w.stop().set_bit());}
          else {i2c2.cr1.modify(|_, w| w.start().set_bit());}
          while i2c2.sr1.read().rx_ne().is_empty() == true {
            if let Err(error) = scan_i2c_error(i2c2.sr1.read().bits() as u16) {return Err(error);}
          }
          self.rx_buffer.push(i2c2.dr.read().dr().bits()).unwrap();
        }
        else if nbytes == 2 {
          i2c2.cr1.modify(|_, w| {
            w.ack().clear_bit();
            w.pos().set_bit()
          });
          _sr = i2c2.sr2.read().bits();
          while i2c2.sr1.read().btf().is_not_finished() == true {
            if let Err(error) = scan_i2c_error(i2c2.sr1.read().bits() as u16) {return Err(error);}
          }
          if stop == true {i2c2.cr1.modify(|_, w| w.stop().set_bit());}
          else {i2c2.cr1.modify(|_, w| w.start().set_bit());}
          while i2c2.sr1.read().rx_ne().is_empty() == true {}
          self.rx_buffer.push(i2c2.dr.read().dr().bits()).unwrap();
          while i2c2.sr1.read().rx_ne().is_empty() == true {}
          self.rx_buffer.push(i2c2.dr.read().dr().bits()).unwrap();
        }
        else {
          _sr = i2c2.sr2.read().bits();
          if nbytes > 3 {
            for _ in 0..(nbytes - 3) {
              while i2c2.sr1.read().btf().is_not_finished() == true {
                if let Err(error) = scan_i2c_error(i2c2.sr1.read().bits() as u16) {return Err(error);}
              }
              self.rx_buffer.push(i2c2.dr.read().dr().bits()).unwrap();
            }
          }
          while i2c2.sr1.read().btf().is_not_finished() == true {
            if let Err(error) = scan_i2c_error(i2c2.sr1.read().bits() as u16) {return Err(error);}
          }
          i2c2.cr1.modify(|_, w| w.ack().clear_bit());
          self.rx_buffer.push(i2c2.dr.read().dr().bits()).unwrap();
          while i2c2.sr1.read().btf().is_not_finished() == true {
            if let Err(error) = scan_i2c_error(i2c2.sr1.read().bits() as u16) {return Err(error);}
          }
          if stop == true {i2c2.cr1.modify(|_, w| w.stop().set_bit());}
          else {i2c2.cr1.modify(|_, w| w.start().set_bit());}
          while i2c2.sr1.read().rx_ne().is_empty() == true {}
          self.rx_buffer.push(i2c2.dr.read().dr().bits()).unwrap();
          while i2c2.sr1.read().rx_ne().is_empty() == true {}
          self.rx_buffer.push(i2c2.dr.read().dr().bits()).unwrap();
        }

        i2c2.cr1.modify(|_, w| w.ack().set_bit());
      },
      3 => {
        let i2c3 = &peripheral_ptr.I2C3;
        i2c3.cr1.write(|w| w.start().set_bit());
        while i2c3.sr1.read().sb().is_no_start() == true {}
        i2c3.dr.write(|w| w.dr().bits((addr << 1) + 1));
        if nbytes == 1 {i2c3.cr1.modify(|_, w| w.ack().clear_bit());}
        while i2c3.sr1.read().addr().is_not_match() == true {
          if let Err(error) = scan_i2c_error(i2c3.sr1.read().bits() as u16) {return Err(error);}
        }

        if nbytes == 1 {
          _sr = i2c3.sr2.read().bits();
          if stop == true {i2c3.cr1.modify(|_, w| w.stop().set_bit());}
          else {i2c3.cr1.modify(|_, w| w.start().set_bit());}
          while i2c3.sr1.read().rx_ne().is_empty() == true {
            if let Err(error) = scan_i2c_error(i2c3.sr1.read().bits() as u16) {return Err(error);}
          }
          self.rx_buffer.push(i2c3.dr.read().dr().bits()).unwrap();
        }
        else if nbytes == 2 {
          i2c3.cr1.modify(|_, w| {
            w.ack().clear_bit();
            w.pos().set_bit()
          });
          _sr = i2c3.sr2.read().bits();
          while i2c3.sr1.read().btf().is_not_finished() == true {
            if let Err(error) = scan_i2c_error(i2c3.sr1.read().bits() as u16) {return Err(error);}
          }
          if stop == true {i2c3.cr1.modify(|_, w| w.stop().set_bit());}
          else {i2c3.cr1.modify(|_, w| w.start().set_bit());}
          while i2c3.sr1.read().rx_ne().is_empty() == true {}
          self.rx_buffer.push(i2c3.dr.read().dr().bits()).unwrap();
          while i2c3.sr1.read().rx_ne().is_empty() == true {}
          self.rx_buffer.push(i2c3.dr.read().dr().bits()).unwrap();
        }
        else {
          _sr = i2c3.sr2.read().bits();
          if nbytes > 3 {
            for _ in 0..(nbytes - 3) {
              while i2c3.sr1.read().btf().is_not_finished() == true {
                if let Err(error) = scan_i2c_error(i2c3.sr1.read().bits() as u16) {return Err(error);}
              }
              self.rx_buffer.push(i2c3.dr.read().dr().bits()).unwrap();
            }
          }
          while i2c3.sr1.read().btf().is_not_finished() == true {
            if let Err(error) = scan_i2c_error(i2c3.sr1.read().bits() as u16) {return Err(error);}
          }
          i2c3.cr1.modify(|_, w| w.ack().clear_bit());
          self.rx_buffer.push(i2c3.dr.read().dr().bits()).unwrap();
          while i2c3.sr1.read().btf().is_not_finished() == true {
            if let Err(error) = scan_i2c_error(i2c3.sr1.read().bits() as u16) {return Err(error);}
          }
          if stop == true {i2c3.cr1.modify(|_, w| w.stop().set_bit());}
          else {i2c3.cr1.modify(|_, w| w.start().set_bit());}
          while i2c3.sr1.read().rx_ne().is_empty() == true {}
          self.rx_buffer.push(i2c3.dr.read().dr().bits()).unwrap();
          while i2c3.sr1.read().rx_ne().is_empty() == true {}
          self.rx_buffer.push(i2c3.dr.read().dr().bits()).unwrap();
        }

        i2c3.cr1.modify(|_, w| w.ack().set_bit());
      },
      _ => panic!("I2C{} is not a valid core! | .recieve_bytes(...)", self.core)
    };

    return Ok(self.rx_buffer.len());
  }

  pub fn available(&self) -> usize {
    return self.rx_buffer.len();
  }

  pub fn read(&mut self) -> Option<u8> {
    return self.rx_buffer.pop();
  }

  pub fn set_clock(&self, clk: u32) -> Result<(), I2cError> {
    let peripheral_ptr;
    unsafe {peripheral_ptr = stm32f4::stm32f446::Peripherals::steal();}

    if clk < 10000 || clk > 400000 {
      rtt_target::rprint!("Clock speed is not compatible with this device! | .set_clock()");
      return Err(I2cError::Prog(ProgError::InvalidConfiguration));
    }

    let (ccr_t, rise_t) = calc_i2c_freq(clk);

    match self.core {
      1 => {
        let i2c1 = &peripheral_ptr.I2C1;
        i2c1.cr1.modify(|_, w| w.pe().disabled());
        i2c1.ccr.modify(|_, w| unsafe {w.ccr().bits(ccr_t as u16)});
        i2c1.trise.write(|w| w.trise().bits(rise_t as u8));
        i2c1.cr1.modify(|_, w| w.pe().enabled());
      },
      2 => {
        let i2c2 = &peripheral_ptr.I2C2;
        i2c2.cr1.modify(|_, w| w.pe().disabled());
        i2c2.ccr.modify(|_, w| unsafe {w.ccr().bits(ccr_t as u16)});
        i2c2.trise.write(|w| w.trise().bits(rise_t as u8));
        i2c2.cr1.modify(|_, w| w.pe().enabled());
      },
      3 => {
        let i2c3 = &peripheral_ptr.I2C3;
        i2c3.cr1.modify(|_, w| w.pe().disabled());
        i2c3.ccr.modify(|_, w| unsafe {w.ccr().bits(ccr_t as u16)});
        i2c3.trise.write(|w| w.trise().bits(rise_t as u8));
        i2c3.cr1.modify(|_, w| w.pe().enabled());
      },
      _ => panic!("I2C{} is not a valid core! | .set_clock()", self.core)
    };

    return Ok(());
  }
}


// Private Functions ==============================================================================
fn calc_i2c_freq(freq: u32) -> (u32, u32) {
  // (I2C_T / 2) / BUS_T ->  BUS_FREQ / (I2C_FREQ * 2)
  let ccr_t = BUS_FREQ / (2 * freq);

  // (1000ns / BUS_T) + 1 -> (BUS_FREQ / 1000000) + 1
  let rise_t = (BUS_FREQ / 1000000) + 1;

  return (ccr_t, rise_t);
}

fn scan_i2c_error(sr: u16) -> Result<(), I2cError> {
  let status = sr & 0b0000111100000000;

  if status & 0b0000100000000000 > 0 {return Err(I2cError::OverrunUnderrun);}
  else if status & 0b0000010000000000 > 0 {return Err(I2cError::NACK);}
  else if status & 0b0000001000000000 > 0 {return Err(I2cError::ArbitrationLoss);}
  else if status & 0b0000000100000000 > 0 {return Err(I2cError::Bus);}
  else {return Ok(());}
}
