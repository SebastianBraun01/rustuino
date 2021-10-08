use crate::include::{stm_peripherals, SpiError, ProgError, SPI_DATA, PIN_CONF};
use crate::gpio::{pinmode_output, pinmode_alternate_function, digital_write, Pin, Output, AlternateFunction};
use heapless::FnvIndexMap;
use rtt_target::rprintln;

#[allow(non_camel_case_types)]
pub enum ClockMode {
  MODE_0,
  MODE_1,
  MODE_2,
  MODE_3
}

#[allow(non_camel_case_types)]
pub enum SpiMode {
  FULL_DUPLEX,
  HALF_DUPLEX,
  SIMPLEX_INPUT,
  SIMPLEX_OUTPUT
}

pub enum FrameFormat {
  MSBFIRST,
  LSBFIRST
}

#[allow(non_camel_case_types)]
pub enum SpiBr {
  DIV_2,
  DIV_4,
  DIV_8,
  DIV_16,
  DIV_32,
  DIV_64,
  DIV_128,
  DIV_256
}

pub struct SPI {
  core: u8,
  _sck_pin: Pin<AlternateFunction>,
  _miso_pin: Pin<AlternateFunction>,
  _mosi_pin: Pin<AlternateFunction>,
  mode: SpiMode,
  nss: FnvIndexMap<u8, Pin<Output>, 5>,
  active: bool,
  id_active: u8
}

impl SPI {
  pub fn new(core: u8, sck: (char, u8), miso: (char, u8), mosi: (char, u8)) -> Result<Self, ProgError> {
    let peripheral_ptr = stm_peripherals();
    let rcc = &peripheral_ptr.RCC;

    let af = match check_spi(core, sck, miso, mosi) {
      Ok(value) => value,
      Err(error) => return Err(error)
    };

    unsafe {
      if PIN_CONF.contains(&sck) || PIN_CONF.contains(&miso) || PIN_CONF.contains(&mosi) {
        rprintln!("These pins are already configured for another function! | SPI::new()");
        return Err(ProgError::InvalidConfiguration);
      }
      else {
        PIN_CONF.push(sck).expect("Could not store pin number! | SPI::new()");
        PIN_CONF.push(miso).expect("Could not store pin number! | SPI::new()");
        PIN_CONF.push(mosi).expect("Could not store pin number! | SPI::new()");
      }
    }

    let sck_pin = match pinmode_alternate_function(sck, af.into()) {
      Ok(value) => value,
      Err(_) => return Err(ProgError::Internal)
    };

    let miso_pin = match pinmode_alternate_function(miso, af.into()) {
      Ok(value) => value,
      Err(_) => return Err(ProgError::Internal)
    };

    let mosi_pin = match pinmode_alternate_function(mosi, af.into()) {
      Ok(value) => value,
      Err(_) => return Err(ProgError::Internal)
    };

    match core {
      1 => {
        let spi1 = &peripheral_ptr.SPI1;
        if rcc.apb2enr.read().spi1en().is_enabled() {
          rprintln!("SPI{} is already configured! | SPI::new()", core);
          return Err(ProgError::AlreadyConfigured);
        }
        rcc.apb2enr.modify(|_, w| w.spi1en().enabled());
        spi1.cr2.modify(|_, w| w.ssoe().enabled());
        spi1.cr1.modify(|_, w| {
          w.ssm().enabled();
          w.br().div4();
          w.mstr().master();
          w.spe().enabled()
        });
      },
      2 => {
        let spi2 = &peripheral_ptr.SPI2;
        if rcc.apb1enr.read().spi2en().is_enabled() == true {
          rprintln!("SPI{} is already configured! | SPI::new()", core);
          return Err(ProgError::AlreadyConfigured);
        }
        rcc.apb1enr.modify(|_, w| w.spi2en().enabled());
        spi2.cr2.modify(|_, w| w.ssoe().enabled());
        spi2.cr1.modify(|_, w| {
          w.ssm().enabled();
          w.br().div4();
          w.mstr().master();
          w.spe().enabled()
        });
      },
      3 => {
        let spi3 = &peripheral_ptr.SPI3;
        if rcc.apb1enr.read().spi3en().is_enabled() == true {
          rprintln!("SPI{} is already configured! | SPI::new()", core);
          return Err(ProgError::AlreadyConfigured);
        }
        rcc.apb1enr.modify(|_, w| w.spi3en().enabled());
        spi3.cr2.modify(|_, w| w.ssoe().enabled());
        spi3.cr1.modify(|_, w| {
          w.ssm().enabled();
          w.br().div4();
          w.mstr().master();
          w.spe().enabled()
        });
      },
      _ => unreachable!()
    };

    return Ok(Self {
      core,
      _sck_pin: sck_pin,
      _miso_pin: miso_pin,
      _mosi_pin: mosi_pin,
      mode: SpiMode::FULL_DUPLEX,
      nss: FnvIndexMap::new(),
      active: false,
      id_active: 0
    });
  }

  pub fn end(self) {
    let peripheral_ptr = stm_peripherals();
    let rcc = &peripheral_ptr.RCC;

    match self.core {
      1 => {
        let spi1 = &peripheral_ptr.SPI1;
        rcc.apb2enr.modify(|_, w| w.spi1en().disabled());
        spi1.cr1.reset();
        spi1.cr2.reset();
      },
      2 => {
        let spi2 = &peripheral_ptr.SPI2;
        rcc.apb2enr.modify(|_, w| w.spi1en().disabled());
        spi2.cr1.reset();
        spi2.cr2.reset();
      },
      3 => {
        let spi3 = &peripheral_ptr.SPI3;
        rcc.apb2enr.modify(|_, w| w.spi1en().disabled());
        spi3.cr1.reset();
        spi3.cr2.reset();
      },
      _ => unreachable!()
    };
  }

  pub fn set_mode(&mut self, mode: SpiMode) -> Result<(), SpiError> {
    let peripheral_ptr = stm_peripherals();

    if self.active == true {
      rprintln!("Cannot configure SPI core while active! | .set_mode()");
      return Err(SpiError::Prog(ProgError::PermissionDenied));
    }

    match self.core {
      1 => {
        let spi1 = &peripheral_ptr.SPI1;
        spi1.cr1.modify(|_, w| w.spe().disabled());
        match mode {
          SpiMode::FULL_DUPLEX => spi1.cr1.modify(|_, w| w.bidimode().clear_bit()),
          SpiMode::HALF_DUPLEX => spi1.cr1.modify(|_, w| w.bidimode().set_bit()),
          SpiMode::SIMPLEX_INPUT => {
            spi1.cr1.modify(|_, w| w.bidimode().set_bit());
            spi1.cr1.modify(|_, w| w.bidioe().clear_bit());
          },
          SpiMode::SIMPLEX_OUTPUT => {
            spi1.cr1.modify(|_, w| w.bidimode().set_bit());
            spi1.cr1.modify(|_, w| w.bidioe().set_bit());
          }
        };
        spi1.cr1.modify(|_, w| w.spe().enabled());
      },
      2 => {
        let spi2 = &peripheral_ptr.SPI2;
        spi2.cr1.modify(|_, w| w.spe().disabled());
        match mode {
          SpiMode::FULL_DUPLEX => spi2.cr1.modify(|_, w| w.bidimode().clear_bit()),
          SpiMode::HALF_DUPLEX => spi2.cr1.modify(|_, w| w.bidimode().set_bit()),
          SpiMode::SIMPLEX_INPUT => {
            spi2.cr1.modify(|_, w| w.bidimode().set_bit());
            spi2.cr1.modify(|_, w| w.bidioe().clear_bit());
          },
          SpiMode::SIMPLEX_OUTPUT => {
            spi2.cr1.modify(|_, w| w.bidimode().set_bit());
            spi2.cr1.modify(|_, w| w.bidioe().set_bit());
          }
        };
        spi2.cr1.modify(|_, w| w.spe().enabled());
      },
      3 => {
        let spi3 = &peripheral_ptr.SPI3;
        spi3.cr1.modify(|_, w| w.spe().disabled());
        match mode {
          SpiMode::FULL_DUPLEX => spi3.cr1.modify(|_, w| w.bidimode().clear_bit()),
          SpiMode::HALF_DUPLEX => spi3.cr1.modify(|_, w| w.bidimode().set_bit()),
          SpiMode::SIMPLEX_INPUT => {
            spi3.cr1.modify(|_, w| w.bidimode().set_bit());
            spi3.cr1.modify(|_, w| w.bidioe().clear_bit());
          },
          SpiMode::SIMPLEX_OUTPUT => {
            spi3.cr1.modify(|_, w| w.bidimode().set_bit());
            spi3.cr1.modify(|_, w| w.bidioe().set_bit());
          }
        };
        spi3.cr1.modify(|_, w| w.spe().enabled());
      },
      _ => unreachable!()
    };

    self.mode = mode;
    return Ok(());
  }

  pub fn set_clk(&self, clk: ClockMode, br: SpiBr) -> Result<(), SpiError> {
    let peripheral_ptr = stm_peripherals();

    if self.active == true {
      rprintln!("Cannot configure SPI core while active! | .set_clk()");
      return Err(SpiError::Prog(ProgError::PermissionDenied));
    }

    match self.core {
      1 => {
        let spi1 = &peripheral_ptr.SPI1;
        spi1.cr1.modify(|_, w| w.spe().disabled());
        match clk {
          ClockMode::MODE_0 => spi1.cr1.modify(|_, w| {w.cpol().clear_bit(); w.cpha().clear_bit()}),
          ClockMode::MODE_1 => spi1.cr1.modify(|_, w| {w.cpol().clear_bit(); w.cpha().set_bit()}),
          ClockMode::MODE_2 => spi1.cr1.modify(|_, w| {w.cpol().set_bit(); w.cpha().set_bit()}),
          ClockMode::MODE_3 => spi1.cr1.modify(|_, w| {w.cpol().set_bit(); w.cpha().clear_bit()}),
        };
        match br {
          SpiBr::DIV_2   => spi1.cr1.modify(|_, w| w.br().div2()),
          SpiBr::DIV_4   => spi1.cr1.modify(|_, w| w.br().div4()),
          SpiBr::DIV_8   => spi1.cr1.modify(|_, w| w.br().div8()),
          SpiBr::DIV_16  => spi1.cr1.modify(|_, w| w.br().div16()),
          SpiBr::DIV_32  => spi1.cr1.modify(|_, w| w.br().div32()),
          SpiBr::DIV_64  => spi1.cr1.modify(|_, w| w.br().div64()),
          SpiBr::DIV_128 => spi1.cr1.modify(|_, w| w.br().div128()),
          SpiBr::DIV_256 => spi1.cr1.modify(|_, w| w.br().div256()),
        };
        spi1.cr1.modify(|_, w| w.spe().enabled());
      },
      2 => {
        let spi2 = &peripheral_ptr.SPI2;
        spi2.cr1.modify(|_, w| w.spe().disabled());
        match clk {
          ClockMode::MODE_0 => spi2.cr1.modify(|_, w| {w.cpol().clear_bit(); w.cpha().clear_bit()}),
          ClockMode::MODE_1 => spi2.cr1.modify(|_, w| {w.cpol().clear_bit(); w.cpha().set_bit()}),
          ClockMode::MODE_2 => spi2.cr1.modify(|_, w| {w.cpol().set_bit(); w.cpha().set_bit()}),
          ClockMode::MODE_3 => spi2.cr1.modify(|_, w| {w.cpol().set_bit(); w.cpha().clear_bit()}),
        };
        match br {
          SpiBr::DIV_2   => spi2.cr1.modify(|_, w| w.br().div2()),
          SpiBr::DIV_4   => spi2.cr1.modify(|_, w| w.br().div4()),
          SpiBr::DIV_8   => spi2.cr1.modify(|_, w| w.br().div8()),
          SpiBr::DIV_16  => spi2.cr1.modify(|_, w| w.br().div16()),
          SpiBr::DIV_32  => spi2.cr1.modify(|_, w| w.br().div32()),
          SpiBr::DIV_64  => spi2.cr1.modify(|_, w| w.br().div64()),
          SpiBr::DIV_128 => spi2.cr1.modify(|_, w| w.br().div128()),
          SpiBr::DIV_256 => spi2.cr1.modify(|_, w| w.br().div256()),
        };
        spi2.cr1.modify(|_, w| w.spe().enabled());
      },
      3 => {
        let spi3 = &peripheral_ptr.SPI3;
        spi3.cr1.modify(|_, w| w.spe().disabled());
        match clk {
          ClockMode::MODE_0 => spi3.cr1.modify(|_, w| {w.cpol().clear_bit(); w.cpha().clear_bit()}),
          ClockMode::MODE_1 => spi3.cr1.modify(|_, w| {w.cpol().clear_bit(); w.cpha().set_bit()}),
          ClockMode::MODE_2 => spi3.cr1.modify(|_, w| {w.cpol().set_bit(); w.cpha().set_bit()}),
          ClockMode::MODE_3 => spi3.cr1.modify(|_, w| {w.cpol().set_bit(); w.cpha().clear_bit()}),
        };
        match br {
          SpiBr::DIV_2   => spi3.cr1.modify(|_, w| w.br().div2()),
          SpiBr::DIV_4   => spi3.cr1.modify(|_, w| w.br().div4()),
          SpiBr::DIV_8   => spi3.cr1.modify(|_, w| w.br().div8()),
          SpiBr::DIV_16  => spi3.cr1.modify(|_, w| w.br().div16()),
          SpiBr::DIV_32  => spi3.cr1.modify(|_, w| w.br().div32()),
          SpiBr::DIV_64  => spi3.cr1.modify(|_, w| w.br().div64()),
          SpiBr::DIV_128 => spi3.cr1.modify(|_, w| w.br().div128()),
          SpiBr::DIV_256 => spi3.cr1.modify(|_, w| w.br().div256()),
        };
        spi3.cr1.modify(|_, w| w.spe().enabled());
      },
      _ => unreachable!()

    };

    return Ok(());
  }

  pub fn set_frame_format(&self, frame: FrameFormat) -> Result<(), SpiError> {
    let peripheral_ptr = stm_peripherals();

    if self.active == true {
      rprintln!("Cannot configure SPI core while active! | .set_frame_format()");
      return Err(SpiError::Prog(ProgError::PermissionDenied));
    }

    match self.core {
      1 => {
        let spi1 = &peripheral_ptr.SPI1;
        spi1.cr1.modify(|_, w| w.spe().disabled());
        if let FrameFormat::MSBFIRST = frame {spi1.cr1.modify(|_, w| w.lsbfirst().clear_bit());}
        else {spi1.cr1.modify(|_, w| w.lsbfirst().set_bit());}
        spi1.cr1.modify(|_, w| w.spe().enabled());
      },
      2 => {
        let spi2 = &peripheral_ptr.SPI2;
        spi2.cr1.modify(|_, w| w.spe().disabled());
        if let FrameFormat::MSBFIRST = frame {spi2.cr1.modify(|_, w| w.lsbfirst().clear_bit());}
        else {spi2.cr1.modify(|_, w| w.lsbfirst().set_bit());}
        spi2.cr1.modify(|_, w| w.spe().enabled());
      },
      3 => {
        let spi3 = &peripheral_ptr.SPI3;
        spi3.cr1.modify(|_, w| w.spe().disabled());
        if let FrameFormat::MSBFIRST = frame {spi3.cr1.modify(|_, w| w.lsbfirst().clear_bit());}
        else {spi3.cr1.modify(|_, w| w.lsbfirst().set_bit());}
        spi3.cr1.modify(|_, w| w.spe().enabled());
      },
      _ => unreachable!()
    };

    return Ok(());
  }

  pub fn add_slave(&mut self, pin: (char, u8), id: u8) -> Result<(), ProgError> {
    unsafe {
      if PIN_CONF.contains(&pin) {
        rprintln!("P{}{} is already configured! | .add_slave()", pin.0.to_uppercase(), pin.1);
        return Err(ProgError::InvalidConfiguration);
      }
    }

    if self.nss.contains_key(&id) == true {
      rprintln!("ID {} already registered for an NSS pin! | .add_slave()", id);
      return Err(ProgError::InvalidConfiguration);
    }

    if self.nss.insert(id, pinmode_output(pin).unwrap()).is_err() {
      rprintln!("Cannot register more than 5 NSS pins! | .add_slave()");
      return Err(ProgError::InvalidConfiguration);
    }

    digital_write(self.nss.get(&id).unwrap(), true);

    return Ok(());
  }

  pub fn begin_transaction(&mut self, id: u8) -> Result<(), SpiError> {
    if self.nss.contains_key(&id) == false {
      rprintln!("ID {} not registered! | .begin_transaction()", id);
      return Err(SpiError::Prog(ProgError::InvalidConfiguration));
    }

    if self.active == true {
      rprintln!("SPI already active! | .begin_transaction()");
      return Err(SpiError::Prog(ProgError::InvalidConfiguration));
    }

    digital_write(self.nss.get(&id).unwrap(), false);
    self.active = true;
    self.id_active = id;

    return Ok(());
  }

  pub fn write(&self, data: u8) -> Result<(), SpiError> {
    let peripheral_ptr = stm_peripherals();

    if let SpiMode::SIMPLEX_INPUT = self.mode {
      rprintln!("Cannot send data in SIMPLEX_INPUT configuration! | .write()");
      return Err(SpiError::Prog(ProgError::PermissionDenied));
    }

    match self.core {
      1 => {
        let spi1 = &peripheral_ptr.SPI1;
        if let SpiMode::HALF_DUPLEX = self.mode {spi1.cr1.modify(|_, w| w.bidioe().set_bit());}
        while spi1.sr.read().txe().bit_is_clear() == true {
          if let Err(error) = scan_spi_error(spi1.sr.read().bits() as u16) {return Err(error);}
        }
        spi1.dr.write(|w| w.dr().bits(data.into()));
      },
      2 => {
        let spi2 = &peripheral_ptr.SPI2;
        if let SpiMode::HALF_DUPLEX = self.mode {spi2.cr1.modify(|_, w| w.bidioe().set_bit());}
        while spi2.sr.read().txe().bit_is_clear() == true {
          if let Err(error) = scan_spi_error(spi2.sr.read().bits() as u16) {return Err(error);}
        }
        spi2.dr.write(|w| w.dr().bits(data.into()));
      },
      3 => {
        let spi3 = &peripheral_ptr.SPI3;
        if let SpiMode::HALF_DUPLEX = self.mode {spi3.cr1.modify(|_, w| w.bidioe().set_bit());}
        while spi3.sr.read().txe().bit_is_clear() == true {
          if let Err(error) = scan_spi_error(spi3.sr.read().bits() as u16) {return Err(error);}
        }
        spi3.dr.write(|w| w.dr().bits(data.into()));
      },
      _ => unreachable!()
    };

    return Ok(());
  }

  pub fn read(&self) -> Result<u8, SpiError> {
    let peripheral_ptr = stm_peripherals();
    let buffer: u8;

    if let SpiMode::SIMPLEX_OUTPUT = self.mode {
      rprintln!("Cannot read data in SIMPLEX_OUTPUT configuration! | .read()");
      return Err(SpiError::Prog(ProgError::PermissionDenied));
    }

    match self.core {
      1 => {
        let spi1 = &peripheral_ptr.SPI1;
        if let SpiMode::HALF_DUPLEX = self.mode {spi1.cr1.modify(|_, w| w.bidioe().set_bit());}
        while spi1.sr.read().txe().bit_is_clear() == true {
          if let Err(error) = scan_spi_error(spi1.sr.read().bits() as u16) {return Err(error);}
        }
        spi1.dr.write(|w| w.dr().bits(0xFF));
        while spi1.sr.read().rxne().bit_is_clear() == true {
          if let Err(error) = scan_spi_error(spi1.sr.read().bits() as u16) {return Err(error);}
        }
        if let SpiMode::HALF_DUPLEX = self.mode {spi1.cr1.modify(|_, w| w.bidioe().clear_bit());}
        buffer = spi1.dr.read().dr().bits() as u8;
      },
      2 => {
        let spi2 = &peripheral_ptr.SPI2;
        if let SpiMode::HALF_DUPLEX = self.mode {spi2.cr1.modify(|_, w| w.bidioe().set_bit());}
        while spi2.sr.read().txe().bit_is_clear() == true {
          if let Err(error) = scan_spi_error(spi2.sr.read().bits() as u16) {return Err(error);}
        }
        spi2.dr.write(|w| w.dr().bits(0xFF));
        while spi2.sr.read().rxne().bit_is_clear() == true {
          if let Err(error) = scan_spi_error(spi2.sr.read().bits() as u16) {return Err(error);}
        }
        if let SpiMode::HALF_DUPLEX = self.mode {spi2.cr1.modify(|_, w| w.bidioe().clear_bit());}
        buffer = spi2.dr.read().dr().bits() as u8;
      },
      3 => {
        let spi3 = &peripheral_ptr.SPI3;
        if let SpiMode::HALF_DUPLEX = self.mode {spi3.cr1.modify(|_, w| w.bidioe().set_bit());}
        while spi3.sr.read().txe().bit_is_clear() == true {
          if let Err(error) = scan_spi_error(spi3.sr.read().bits() as u16) {return Err(error);}
        }
        spi3.dr.write(|w| w.dr().bits(0xFF));
        while spi3.sr.read().rxne().bit_is_clear() == true {
          if let Err(error) = scan_spi_error(spi3.sr.read().bits() as u16) {return Err(error);}
        }
        if let SpiMode::HALF_DUPLEX = self.mode {spi3.cr1.modify(|_, w| w.bidioe().clear_bit());}
        buffer = spi3.dr.read().dr().bits() as u8;
      },
      _ => unreachable!()
    }

    return Ok(buffer);
  }

  pub fn end_transaction(&mut self) {
    digital_write(self.nss.get(&self.id_active).unwrap(), true);

    self.active = false;
  }
}


// Private Functions ==============================================================================
fn check_spi(core: u8, sck: (char, u8), miso: (char, u8), mosi: (char, u8)) -> Result<u8, ProgError> {
  // SPI1 -> AF5
  // SPI2 -> AF5
  // SPI3 -> AF6

  match core {
    1 => {
      if SPI_DATA.s1_sck.contains(&sck) == false {return Err(ProgError::InvalidConfiguration);}
      else if SPI_DATA.s1_miso.contains(&miso) == false {return Err(ProgError::InvalidConfiguration);}
      else if SPI_DATA.s1_mosi.contains(&mosi) == false {return Err(ProgError::InvalidConfiguration);}
      else {return Ok(5);}
    },
    2 => {
      if SPI_DATA.s2_sck.contains(&sck) == false {return Err(ProgError::InvalidConfiguration);}
      else if SPI_DATA.s2_miso.contains(&miso) == false {return Err(ProgError::InvalidConfiguration);}
      else if SPI_DATA.s2_mosi.contains(&mosi) == false {return Err(ProgError::InvalidConfiguration);}
      else {return Ok(5);}
    },
    3 => {
      if SPI_DATA.s3_sck.contains(&sck) == false {return Err(ProgError::InvalidConfiguration);}
      else if SPI_DATA.s3_miso.contains(&miso) == false {return Err(ProgError::InvalidConfiguration);}
      else if SPI_DATA.s3_mosi.contains(&mosi) == false {return Err(ProgError::InvalidConfiguration);}
      else {return Ok(6);}
    },
    _ => {
      rprintln!("SPI{} is not a valid core! | check_spi()", core);
      return Err(ProgError::InvalidConfiguration);
    }
  };
}

fn scan_spi_error(sr: u16) -> Result<(), SpiError> {
  let status = sr & 0b0000000101110000;

  if status &  0b0000000001000000 > 0 {return Err(SpiError::Overrun);}
  else if status &  0b0000000000100000 > 0 {return Err(SpiError::ModeFault);}
  else if status &  0b0000000000010000 > 0 {return Err(SpiError::CRCError);}
  else {return Ok(());}
}
