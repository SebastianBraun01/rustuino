use crate::include::{stm_peripherals, SpiError, ProgError, SPI_DATA};
use crate::gpio::{pin_mode, digital_write, GpioMode::AlternateFunction, GpioMode::Output};
use heapless::FnvIndexMap;

#[allow(non_camel_case_types)]
#[derive(PartialEq, Eq)]
pub enum ClockMode {
  MODE_0,
  MODE_1,
  MODE_2,
  MODE_3
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, Eq)]
pub enum SpiMode {
  FULL_DUPLEX,
  HALF_DUPLEX,
  SIMPLEX_INPUT,
  SIMPLEX_OUTPUT
}

#[derive(PartialEq, Eq)]
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
  mode: SpiMode,
  com_pins: [(char, u8); 3],
  nss: FnvIndexMap<u8, (char, u8), 5>,
  active: bool,
  id_active: u8
}

impl SPI {
  pub fn new(core: u8, sck: (char, u8), miso: (char, u8), mosi: (char, u8)) -> Result<SPI, SpiError> {
    let peripheral_ptr = stm_peripherals();
    let rcc = &peripheral_ptr.RCC;

    let af = match check_spi(core, sck, miso, mosi) {
      Ok(value) => value,
      Err(error) => return Err(error)
    };

    if let Err(_) = pin_mode(sck, AlternateFunction(af.into())) {return Err(SpiError::Prog(ProgError::Internal));}
    if let Err(_) = pin_mode(miso, AlternateFunction(af.into())) {return Err(SpiError::Prog(ProgError::Internal));}
    if let Err(_) = pin_mode(mosi, AlternateFunction(af.into())) {return Err(SpiError::Prog(ProgError::Internal));}

    match core {
      1 => {
        let spi1 = &peripheral_ptr.SPI1;
        if rcc.apb2enr.read().spi1en().is_enabled() {
          rtt_target::rprintln!("SPI{} is already configured! | SPI::new()", core);
          return Err(SpiError::Prog(ProgError::PermissionDenied));
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
          rtt_target::rprintln!("SPI{} is already configured! | SPI::new()", core);
          return Err(SpiError::Prog(ProgError::PermissionDenied));
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
          rtt_target::rprintln!("SPI{} is already configured! | SPI::new()", core);
          return Err(SpiError::Prog(ProgError::PermissionDenied));
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
      _ => panic!("SPI{} is not a valid core! | SPI::new()", core)
    };

    return Ok(SPI {
      core,
      mode: SpiMode::FULL_DUPLEX,
      com_pins: [sck, miso, mosi],
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
      _ => panic!("SPI{} is not a valid core! | .end()", self.core)
    };
  }

  pub fn configure(&mut self, mode: SpiMode, clk: ClockMode, frame: FrameFormat, br: SpiBr) -> Result<(), ()> {
    let peripheral_ptr = stm_peripherals();

    if self.active == true {return Err(());}

    match self.core {
      1 => {
        let spi1 = &peripheral_ptr.SPI1;
        spi1.cr1.modify(|_, w| w.spe().disabled());
        if mode != SpiMode::FULL_DUPLEX {spi1.cr1.modify(|_, w| w.bidimode().set_bit());}
        else {spi1.cr1.modify(|_, w| w.bidimode().clear_bit());}
        if mode == SpiMode::SIMPLEX_INPUT {spi1.cr1.modify(|_, w| w.bidioe().clear_bit());}
        else if mode == SpiMode::SIMPLEX_OUTPUT {spi1.cr1.modify(|_, w| w.bidioe().set_bit());}
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

        if frame == FrameFormat::MSBFIRST {spi1.cr1.modify(|_, w| w.lsbfirst().clear_bit());}
        else {spi1.cr1.modify(|_, w| w.lsbfirst().set_bit());}
        spi1.cr1.modify(|_, w| w.spe().enabled());
      },
      2 => {
        let spi2 = &peripheral_ptr.SPI2;
        spi2.cr1.modify(|_, w| w.spe().disabled());
        if mode != SpiMode::FULL_DUPLEX {spi2.cr1.modify(|_, w| w.bidimode().set_bit());}
        else {spi2.cr1.modify(|_, w| w.bidimode().clear_bit());}
        if mode == SpiMode::SIMPLEX_INPUT {spi2.cr1.modify(|_, w| w.bidioe().clear_bit());}
        else if mode == SpiMode::SIMPLEX_OUTPUT {spi2.cr1.modify(|_, w| w.bidioe().set_bit());}
        match clk {
          ClockMode::MODE_0 => spi2.cr1.modify(|_, w| {w.cpol().clear_bit(); w.cpha().clear_bit()}),
          ClockMode::MODE_1 => spi2.cr1.modify(|_, w| {w.cpol().clear_bit(); w.cpha().set_bit()}),
          ClockMode::MODE_2 => spi2.cr1.modify(|_, w| {w.cpol().set_bit(); w.cpha().set_bit()}),
          ClockMode::MODE_3 => spi2.cr1.modify(|_, w| {w.cpol().set_bit(); w.cpha().clear_bit()}),
        };
        if frame == FrameFormat::MSBFIRST {spi2.cr1.modify(|_, w| w.lsbfirst().clear_bit());}
        else {spi2.cr1.modify(|_, w| w.lsbfirst().set_bit());}
        spi2.cr1.modify(|_, w| w.spe().enabled());
      },
      3 => {
        let spi3 = &peripheral_ptr.SPI3;
        spi3.cr1.modify(|_, w| w.spe().disabled());
        if mode != SpiMode::FULL_DUPLEX {spi3.cr1.modify(|_, w| w.bidimode().set_bit());}
        else {spi3.cr1.modify(|_, w| w.bidimode().clear_bit());}
        if mode == SpiMode::SIMPLEX_INPUT {spi3.cr1.modify(|_, w| w.bidioe().clear_bit());}
        else if mode == SpiMode::SIMPLEX_OUTPUT {spi3.cr1.modify(|_, w| w.bidioe().set_bit());}
        match clk {
          ClockMode::MODE_0 => spi3.cr1.modify(|_, w| {w.cpol().clear_bit(); w.cpha().clear_bit()}),
          ClockMode::MODE_1 => spi3.cr1.modify(|_, w| {w.cpol().clear_bit(); w.cpha().set_bit()}),
          ClockMode::MODE_2 => spi3.cr1.modify(|_, w| {w.cpol().set_bit(); w.cpha().set_bit()}),
          ClockMode::MODE_3 => spi3.cr1.modify(|_, w| {w.cpol().set_bit(); w.cpha().clear_bit()}),
        };
        if frame == FrameFormat::MSBFIRST {spi3.cr1.modify(|_, w| w.lsbfirst().clear_bit());}
        else {spi3.cr1.modify(|_, w| w.lsbfirst().set_bit());}
        spi3.cr1.modify(|_, w| w.spe().enabled());
      },
      _ => panic!("SPI{} is not a valid core! | .configure()", self.core)
    };

    self.mode = mode;
    return Ok(());
  }

  pub fn add_slave(&mut self, pin: (char, u8), id: u8) -> Result<(), ()> {
    if self.com_pins.contains(&pin) == true {
      rtt_target::rprintln!("P{}{} is not available as an NSS pin! | .add_slave()", pin.0.to_uppercase(), pin.1);
      return Err(());
    }
    else if self.nss.contains_key(&id) == true {
      rtt_target::rprintln!("ID {} already registered for an NSS pin! | .add_slave()", id);
      return Err(());
    }
    else if self.nss.values().any(|&i| i == pin) == true {
      rtt_target::rprintln!("P{}{} already registered as an NSS pin! | .add_slave()", pin.0.to_uppercase(), pin.1);
      return Err(());
    }

    if self.nss.insert(id, pin).is_err() {
      rtt_target::rprintln!("Cannot register more than 5 NSS pins! | .add_slave()");
      return Err(());
    }

    pin_mode(pin, Output).expect("Could not configre pin as an NSS pin! | .add_slave()");
    digital_write(pin, true).expect("Could not set pin value! | .add_slave()");

    return Ok(());
  }

  pub fn begin_transaction(&mut self, id: u8) -> Result<(), ()> {
    if self.nss.contains_key(&id) == false {
      rtt_target::rprintln!("ID {} not registered! | .begin_transaction()", id);
      return Err(());
    }

    if self.active == true {
      rtt_target::rprintln!("SPI already active! | .begin_transaction()");
      return Err(());
    }

    digital_write(self.nss.get(&id).unwrap().clone(), false).expect("Could not set pin value! | .begin_transaction()");
    self.active = true;
    self.id_active = id;

    return Ok(());
  }

  pub fn write(&self, data: u8) -> Result<(), SpiError> {
    let peripheral_ptr = stm_peripherals();

    if self.mode == SpiMode::SIMPLEX_INPUT {return Err(SpiError::Prog(ProgError::PermissionDenied));}

    match self.core {
      1 => {
        let spi1 = &peripheral_ptr.SPI1;
        if self.mode == SpiMode::HALF_DUPLEX {spi1.cr1.modify(|_, w| w.bidioe().set_bit());}
        while spi1.sr.read().txe().bit_is_clear() == true {
          if let Err(error) = scan_spi_error(spi1.sr.read().bits() as u16) {return Err(error);}
        }
        spi1.dr.write(|w| w.dr().bits(data.into()));
      },
      2 => {
        let spi2 = &peripheral_ptr.SPI2;
        if self.mode == SpiMode::HALF_DUPLEX {spi2.cr1.modify(|_, w| w.bidioe().set_bit());}
        while spi2.sr.read().txe().bit_is_clear() == true {
          if let Err(error) = scan_spi_error(spi2.sr.read().bits() as u16) {return Err(error);}
        }
        spi2.dr.write(|w| w.dr().bits(data.into()));
      },
      3 => {
        let spi3 = &peripheral_ptr.SPI3;
        if self.mode == SpiMode::HALF_DUPLEX {spi3.cr1.modify(|_, w| w.bidioe().set_bit());}
        while spi3.sr.read().txe().bit_is_clear() == true {
          if let Err(error) = scan_spi_error(spi3.sr.read().bits() as u16) {return Err(error);}
        }
        spi3.dr.write(|w| w.dr().bits(data.into()));
      },
      _ => panic!("SPI{} is not a valid core! | .write()", self.core)
    };

    return Ok(());
  }

  pub fn read(&self) -> Result<u8, SpiError> {
    let peripheral_ptr = stm_peripherals();
    let buffer: u8;

    if self.mode == SpiMode::SIMPLEX_OUTPUT {return Err(SpiError::Prog(ProgError::PermissionDenied));}

    match self.core {
      1 => {
        let spi1 = &peripheral_ptr.SPI1;
        if self.mode == SpiMode::HALF_DUPLEX {spi1.cr1.modify(|_, w| w.bidioe().set_bit());}
        while spi1.sr.read().txe().bit_is_clear() == true {
          if let Err(error) = scan_spi_error(spi1.sr.read().bits() as u16) {return Err(error);}
        }
        spi1.dr.write(|w| w.dr().bits(0xFF));
        while spi1.sr.read().rxne().bit_is_clear() == true {
          if let Err(error) = scan_spi_error(spi1.sr.read().bits() as u16) {return Err(error);}
        }
        if self.mode == SpiMode::HALF_DUPLEX {spi1.cr1.modify(|_, w| w.bidioe().clear_bit());}
        buffer = spi1.dr.read().dr().bits() as u8;
      },
      2 => {
        let spi2 = &peripheral_ptr.SPI2;
        if self.mode == SpiMode::HALF_DUPLEX {spi2.cr1.modify(|_, w| w.bidioe().set_bit());}
        while spi2.sr.read().txe().bit_is_clear() == true {
          if let Err(error) = scan_spi_error(spi2.sr.read().bits() as u16) {return Err(error);}
        }
        spi2.dr.write(|w| w.dr().bits(0xFF));
        while spi2.sr.read().rxne().bit_is_clear() == true {
          if let Err(error) = scan_spi_error(spi2.sr.read().bits() as u16) {return Err(error);}
        }
        if self.mode == SpiMode::HALF_DUPLEX {spi2.cr1.modify(|_, w| w.bidioe().clear_bit());}
        buffer = spi2.dr.read().dr().bits() as u8;
      },
      3 => {
        let spi3 = &peripheral_ptr.SPI3;
        if self.mode == SpiMode::HALF_DUPLEX {spi3.cr1.modify(|_, w| w.bidioe().set_bit());}
        while spi3.sr.read().txe().bit_is_clear() == true {
          if let Err(error) = scan_spi_error(spi3.sr.read().bits() as u16) {return Err(error);}
        }
        spi3.dr.write(|w| w.dr().bits(0xFF));
        while spi3.sr.read().rxne().bit_is_clear() == true {
          if let Err(error) = scan_spi_error(spi3.sr.read().bits() as u16) {return Err(error);}
        }
        if self.mode == SpiMode::HALF_DUPLEX {spi3.cr1.modify(|_, w| w.bidioe().clear_bit());}
        buffer = spi3.dr.read().dr().bits() as u8;
      },
      _ => panic!("SPI{} is not a valid core! | .read()", self.core)
    }

    return Ok(buffer);
  }

  pub fn end_transaction(&mut self) {
    digital_write(self.nss.get(&self.id_active).unwrap().clone(), true)
    .expect("Could not set pin value! | .begin_transaction()");

    self.active = false;
  }
}


// Private Functions ==============================================================================
fn check_spi(core: u8, sck: (char, u8), miso: (char, u8), mosi: (char, u8)) -> Result<u8, SpiError> {
  // SPI1 -> AF5
  // SPI2 -> AF5
  // SPI3 -> AF6

  match core {
    1 => {
      if SPI_DATA.s1_sck.contains(&sck) == false {return Err(SpiError::Prog(ProgError::InvalidConfiguration));}
      else if SPI_DATA.s1_miso.contains(&miso) == false {return Err(SpiError::Prog(ProgError::InvalidConfiguration));}
      else if SPI_DATA.s1_mosi.contains(&mosi) == false {return Err(SpiError::Prog(ProgError::InvalidConfiguration));}
      else {return Ok(5);}
    },
    2 => {
      if SPI_DATA.s2_sck.contains(&sck) == false {return Err(SpiError::Prog(ProgError::InvalidConfiguration));}
      else if SPI_DATA.s2_miso.contains(&miso) == false {return Err(SpiError::Prog(ProgError::InvalidConfiguration));}
      else if SPI_DATA.s2_mosi.contains(&mosi) == false {return Err(SpiError::Prog(ProgError::InvalidConfiguration));}
      else {return Ok(5);}
    },
    3 => {
      if SPI_DATA.s3_sck.contains(&sck) == false {return Err(SpiError::Prog(ProgError::InvalidConfiguration));}
      else if SPI_DATA.s3_miso.contains(&miso) == false {return Err(SpiError::Prog(ProgError::InvalidConfiguration));}
      else if SPI_DATA.s3_mosi.contains(&mosi) == false {return Err(SpiError::Prog(ProgError::InvalidConfiguration));}
      else {return Ok(6);}
    },
    _ => panic!("SPI{} is not a valid core! | check_spi()", core)
  };
}

fn scan_spi_error(sr: u16) -> Result<(), SpiError> {
  let status = sr & 0b0000000101110000;

  if status &       0b0000000100000000 > 0 {return Err(SpiError::FrameFormat);}
  else if status &  0b0000000001000000 > 0 {return Err(SpiError::Overrun);}
  else if status &  0b0000000000100000 > 0 {return Err(SpiError::ModeFault);}
  else if status &  0b0000000000010000 > 0 {return Err(SpiError::CRCError);}
  else {return Ok(());}
}
