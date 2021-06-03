#![no_std]

use rustuino::*;

extern int dir;

pub fn InitUSART2() {
  // Initialisierung der USART2-Schnittstelle
  (*RCC_PTR).ahb1enr.modify(|_, w| w.gpioaen().enabled());
  (*GPIOA_PTR).moder.modify(|r, w| w.bits(r.bits() & 0xFFFFFF0F | 0xA0));
  (*GPIOA_PTR).afrl.modify(|r, w| w.bits(r.bits() & 0xFFFF00FF | 0x7700));
  (*GPIOA_PTR).otyper.modify(|r, w| w.bits(r.bits() & 0xFFFB));
	
  (*RCC_PTR).ahb1enr.modify(|_, w| w.usart2en().enabled());

  (*USART2_PTR).brr.modify(|_, w| w.bits(0x682));
  (*USART2_PTR).cr1.modify(|_, w| {
    w.re().enabled();
    w.te().enabled();
    w.ue.enabled()
  });

  NVIC::unmask(Interrupt::USART2);
}

pub fn WriteChar(c: char) {
  // schreibt Zeichen c an USART2
  (*USART2_PTR).dr.write(|w| w.bits(c as u16));
  while (*USART2_PTR).sr.read().txe().bit_not_set() == true {}	
}

pub fn WriteString(string: &str) {
// schreibt String str an USART2
  loop {
    while(*str != '\0'){
		  WriteChar(*str);
		  str += 1;
	  }
  }
	
}
	
pub fn ReadChar() -> char {
  while (*USART2_PTR).sr.read().rxne().bit_not_set() == true {}
  return ((*USART2_PTR).dr.read().bits() & 0xFF) as char;
}

#[interrupt]
fn USART2() {
	let buffer: char;
	static string_buffer[4]: char;
	static count: u8 = 0;
	
	buffer = (*USART2_PTR).dr.read().bits();
	WriteChar(buffer);
		
	if buffer == '.' {
		// Befehl ausführen
		let opcode: char;
		let value;
			
		sscanf(string_buffer, "%c%u", &opcode, &value);

		match opcode {
			'w' => {
				(*SYSTICK_PTR).load.write(|w| w.bits((16000/8) * value));
				WriteString("\r\n");
			},
			'r' => {
				if value == 0 || value == 1 {
					dir = value;
					WriteString("\r\n");
				}
				else {WriteString("\n\rFehler: Richtung nicht definiert!\r\n");}
			},
			_   => WriteString("\n\rFehler: Befehl unbekannt!\r\n")
		};
		
		count = 0;
	}
	else{
		// Prüfen ob Befehl zu lang ist und Zeichen in Puffer laden
		if count > 3 {WriteString("\n\rFehler:Befehl Puffer voll!\r\n");}
		else{
			string_buffer[count] = buffer;
			count += 1;
		}
	}	
	// char buffer = USART2->DR;
	// WriteChar(buffer);
}
