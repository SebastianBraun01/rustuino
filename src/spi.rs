use crate::include::{SPI_MAP, SPI_CONF};
use stm32f4::stm32f446::{NVIC, Interrupt};

pub struct SpiCore {
  pub sck: (char, u8),
  pub miso: (char, u8),
  pub mosi: (char, u8),
  pub core: u8,
  pub rx_int: bool,
  pub tx_int: bool
}


// Initialisation function ======================================================================
pub fn spi_init(sck: (char, u8), miso: (char, u8), mosi: (char, u8)) -> Option<SpiCore> {
  let peripheral_ptr;
  unsafe {peripheral_ptr = stm32f4::stm32f446::Peripherals::steal();}
  let rcc = &peripheral_ptr.RCC;

  let index = SPI_MAP.sck_pins.iter().position(|&i| i == sck).unwrap();
  let af = SPI_MAP.af.iter().nth(index).unwrap().clone();
  let core = SPI_MAP.core.iter().nth(index).unwrap().clone();
  if SPI_MAP.miso_pins.iter().position(|&i| i == miso).unwrap() != index || SPI_MAP.mosi_pins.iter().position(|&i| i == mosi).unwrap() != index {
    rtt_target::rprintln!("These pins are not available for SPI communication! | ::spi_init(...)");
    return None;
  }

  unsafe {SPI_CONF[core as usize] = true;}

  spi_gpio_init(sck.0, sck.1, af);
  spi_gpio_init(miso.0, miso.1, af);
  spi_gpio_init(mosi.0, mosi.1, af);

  match core {
    1 => {
      let spi1 = &peripheral_ptr.SPI1;
      rcc.apb2enr.modify(|_, w| w.spi1en().enabled());
      spi1.cr1.modify(|_, w| {
        w.ssm().enabled();
        w.br().div4();
        w.mstr().master()
      });
      spi1.cr2.modify(|_, w| w.ssoe().enabled());
      spi1.cr1.modify(|_, w| w.spe().enabled());
    },
    2 => {
      let spi2 = &peripheral_ptr.SPI2;
      rcc.apb1enr.modify(|_, w| w.spi2en().enabled());
      spi2.cr1.modify(|_, w| {
        w.ssm().enabled();
        w.br().div4();
        w.mstr().master()
      });
      spi2.cr2.modify(|_, w| w.ssoe().enabled());
      spi2.cr1.modify(|_, w| w.spe().enabled());
    },
    3 => {
      let spi3 = &peripheral_ptr.SPI3;
      rcc.apb1enr.modify(|_, w| w.spi3en().enabled());
      spi3.cr1.modify(|_, w| {
        w.ssm().enabled();
        w.br().div4();
        w.mstr().master()
      });
      spi3.cr2.modify(|_, w| w.ssoe().enabled());
      spi3.cr1.modify(|_, w| w.spe().enabled());
    },
    4 => {
      let spi4 = &peripheral_ptr.SPI4;
      rcc.apb2enr.modify(|_, w| w.spi4en().enabled());
      spi4.cr1.modify(|_, w| {
        w.ssm().enabled();
        w.br().div4();
        w.mstr().master()
      });
      spi4.cr2.modify(|_, w| w.ssoe().enabled());
      spi4.cr1.modify(|_, w| w.spe().enabled());
    },
    _ => panic!("SPI{} is not a valid core! | ::spi_init(...)", core)
  };

  return Some(SpiCore {
    sck,
    miso,
    mosi,
    core,
    rx_int: false,
    tx_int: false
  });
}


// Communication functions ======================================================================
impl SpiCore {
  pub fn rxint_enable(&mut self) {
    let peripheral_ptr;
    unsafe {peripheral_ptr = stm32f4::stm32f446::Peripherals::steal();}

    match self.core {
      1 => {
        let spi1 = &peripheral_ptr.SPI1;
        unsafe {NVIC::unmask(Interrupt::SPI1);}
        spi1.cr2.modify(|_, w| w.rxneie().set_bit());
      },
      2 => {
        let spi2 = &peripheral_ptr.SPI2;
        unsafe {NVIC::unmask(Interrupt::SPI2);}
        spi2.cr2.modify(|_, w| w.rxneie().set_bit());
      },
      3 => {
        let spi3 = &peripheral_ptr.SPI3;
        unsafe {NVIC::unmask(Interrupt::SPI3);}
        spi3.cr2.modify(|_, w| w.rxneie().set_bit());
      },
      4 => {
        let spi4 = &peripheral_ptr.SPI4;
        unsafe {NVIC::unmask(Interrupt::SPI4);}
        spi4.cr2.modify(|_, w| w.rxneie().set_bit());
      },
      _ => panic!("SPI{} is not a valid core! | ::spi_init(...)", self.core)
    };

    self.rx_int = true;
  }

  pub fn rxint_disable(&mut self) {
    let peripheral_ptr;
    unsafe {peripheral_ptr = stm32f4::stm32f446::Peripherals::steal();}

    match self.core {
      1 => {
        let spi1 = &peripheral_ptr.SPI1;
        NVIC::mask(Interrupt::SPI1);
        spi1.cr2.modify(|_, w| w.rxneie().clear_bit());
      },
      2 => {
        let spi2 = &peripheral_ptr.SPI2;
        NVIC::mask(Interrupt::SPI2);
        spi2.cr2.modify(|_, w| w.rxneie().clear_bit());
      },
      3 => {
        let spi3 = &peripheral_ptr.SPI3;
        NVIC::mask(Interrupt::SPI3);
        spi3.cr2.modify(|_, w| w.rxneie().clear_bit());
      },
      4 => {
        let spi4 = &peripheral_ptr.SPI4;
        NVIC::mask(Interrupt::SPI4);
        spi4.cr2.modify(|_, w| w.rxneie().clear_bit());
      },
      _ => panic!("SPI{} is not a valid core! | ::spi_init(...)", self.core)
    };

    self.rx_int = false;
  }

  pub fn txint_enable(&mut self) {
    let peripheral_ptr;
    unsafe {peripheral_ptr = stm32f4::stm32f446::Peripherals::steal();}

    match self.core {
      1 => {
        let spi1 = &peripheral_ptr.SPI1;
        unsafe {NVIC::unmask(Interrupt::SPI1);}
        spi1.cr2.modify(|_, w| w.txeie().set_bit());
      },
      2 => {
        let spi2 = &peripheral_ptr.SPI2;
        unsafe {NVIC::unmask(Interrupt::SPI2);}
        spi2.cr2.modify(|_, w| w.txeie().set_bit());
      },
      3 => {
        let spi3 = &peripheral_ptr.SPI3;
        unsafe {NVIC::unmask(Interrupt::SPI3);}
        spi3.cr2.modify(|_, w| w.txeie().set_bit());
      },
      4 => {
        let spi4 = &peripheral_ptr.SPI4;
        unsafe {NVIC::unmask(Interrupt::SPI4);}
        spi4.cr2.modify(|_, w| w.txeie().set_bit());
      },
      _ => panic!("SPI{} is not a valid core! | ::spi_init(...)", self.core)
    };

    self.tx_int = true;
  }

  pub fn txint_disable(&mut self) {
    let peripheral_ptr;
    unsafe {peripheral_ptr = stm32f4::stm32f446::Peripherals::steal();}

    match self.core {
      1 => {
        let spi1 = &peripheral_ptr.SPI1;
        NVIC::mask(Interrupt::SPI1);
        spi1.cr2.modify(|_, w| w.txeie().clear_bit());
      },
      2 => {
        let spi2 = &peripheral_ptr.SPI2;
        NVIC::mask(Interrupt::SPI2);
        spi2.cr2.modify(|_, w| w.txeie().clear_bit());
      },
      3 => {
        let spi3 = &peripheral_ptr.SPI3;
        NVIC::mask(Interrupt::SPI3);
        spi3.cr2.modify(|_, w| w.txeie().clear_bit());
      },
      4 => {
        let spi4 = &peripheral_ptr.SPI4;
        NVIC::mask(Interrupt::SPI4);
        spi4.cr2.modify(|_, w| w.txeie().clear_bit());
      },
      _ => panic!("SPI{} is not a valid core! | ::spi_init(...)", self.core)
    };

    self.tx_int = false;
  }

  pub fn send_bytes(&self, data: u8) {
    let peripheral_ptr;
    unsafe {peripheral_ptr = stm32f4::stm32f446::Peripherals::steal();}

    match self.core {
      1 => {
        let spi1 = &peripheral_ptr.SPI1;
        spi1.dr.write(|w| w.dr().bits(data as u16));
        while spi1.sr.read().txe().bit_is_clear() == true {}
      },
      2 => {
        let spi2 = &peripheral_ptr.SPI2;
        spi2.dr.write(|w| w.dr().bits(data as u16));
        while spi2.sr.read().txe().bit_is_clear() == true {}
      },
      3 => {
        let spi3 = &peripheral_ptr.SPI3;
        spi3.dr.write(|w| w.dr().bits(data as u16));
        while spi3.sr.read().txe().bit_is_clear() == true {}
      },
      4 => {
        let spi4 = &peripheral_ptr.SPI4;
        spi4.dr.write(|w| w.dr().bits(data as u16));
        while spi4.sr.read().txe().bit_is_clear() == true {}
      },
      _ => panic!("SPI{} is not a valid core! | ::spi_init(...)", self.core)
    };
  }

  pub fn recieve_bytes(&self) {
    todo!("Soon to be implemented! | .recieve_bytes(...)");
  }
}


// Helper functions =============================================================================
fn spi_gpio_init(block: char, pin: u8, af: u32) {
  let peripheral_ptr;
  unsafe {peripheral_ptr = stm32f4::stm32f446::Peripherals::steal();}
  let rcc = &peripheral_ptr.RCC;

  match block {
    'a' => {
      let gpioa = &peripheral_ptr.GPIOA;
      rcc.ahb1enr.modify(|_, w| w.gpioaen().enabled());
      gpioa.moder.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin)) | (2 << (2 * pin)))});
      if pin > 7 {gpioa.afrh.modify(|r, w| unsafe {w.bits(r.bits() | (af << (4 * (pin - 8))))});}
      else {gpioa.afrl.modify(|r, w| unsafe {w.bits(r.bits() | (af << (4 * pin)))});}
    },
    'b' => {
      let gpiob = &peripheral_ptr.GPIOB;
      rcc.ahb1enr.modify(|_, w| w.gpioben().enabled());
      gpiob.moder.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin)) | (2 << (2 * pin)))});
      if pin > 7 {gpiob.afrh.modify(|r, w| unsafe {w.bits(r.bits() | (af << (4 * (pin - 8))))});}
      else {gpiob.afrl.modify(|r, w| unsafe {w.bits(r.bits() | (af << (4 * pin)))});}
    },
    'c' => {
      let gpioc = &peripheral_ptr.GPIOC;
      rcc.ahb1enr.modify(|_, w| w.gpiocen().enabled());
      gpioc.moder.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin)) | (2 << (2 * pin)))});
      if pin > 7 {gpioc.afrh.modify(|r, w| unsafe {w.bits(r.bits() | (af << (4 * (pin - 8))))});}
      else {gpioc.afrl.modify(|r, w| unsafe {w.bits(r.bits() | (af << (4 * pin)))});}
    },
    'd' => {
      let gpiod = &peripheral_ptr.GPIOD;
      rcc.ahb1enr.modify(|_, w| w.gpioden().enabled());
      gpiod.moder.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin)) | (2 << (2 * pin)))});
      if pin > 7 {gpiod.afrh.modify(|r, w| unsafe {w.bits(r.bits() | (af << (4 * (pin - 8))))});}
      else {gpiod.afrl.modify(|r, w| unsafe {w.bits(r.bits() | (af << (4 * pin)))});}
    },
    'e' => {
      let gpioe = &peripheral_ptr.GPIOE;
      rcc.ahb1enr.modify(|_, w| w.gpioeen().enabled());
      gpioe.moder.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin)) | (2 << (2 * pin)))});
      if pin > 7 {gpioe.afrh.modify(|r, w| unsafe {w.bits(r.bits() | (af << (4 * (pin - 8))))});}
      else {gpioe.afrl.modify(|r, w| unsafe {w.bits(r.bits() | (af << (4 * pin)))});}
    },
    'f' => {
      let gpiof = &peripheral_ptr.GPIOF;
      rcc.ahb1enr.modify(|_, w| w.gpiofen().enabled());
      gpiof.moder.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin)) | (2 << (2 * pin)))});
      if pin > 7 {gpiof.afrh.modify(|r, w| unsafe {w.bits(r.bits() | (af << (4 * (pin - 8))))});}
      else {gpiof.afrl.modify(|r, w| unsafe {w.bits(r.bits() | (af << (4 * pin)))});}
    },
    'g' => {
      let gpiog = &peripheral_ptr.GPIOG;
      rcc.ahb1enr.modify(|_, w| w.gpiogen().enabled());
      gpiog.moder.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin)) | (2 << (2 * pin)))});
      if pin > 7 {gpiog.afrh.modify(|r, w| unsafe {w.bits(r.bits() | (af << (4 * (pin - 8))))});}
      else {gpiog.afrl.modify(|r, w| unsafe {w.bits(r.bits() | (af << (4 * pin)))});}
    },
    _   => panic!("P{}{} is not available for spi transmission! | spi_setup_gpio(...)", block.to_uppercase(), pin)
  };
}
