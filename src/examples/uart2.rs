#![no_std]

use rustuino::*;


#define milliseconds 16000/8		// LOAD-Wert für eine Millisekunde

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
	let inputChar: char;							// Eingelesenes Zeichen
	static bufferPos: u32 = 0;		    // Pufferposition
	
	// Empfangenes Zeichen speichern
  inputChar = (*USART2_PTR).dr.read().bits();
	WriteChar(inputChar);				      // Echo zurückschicken

	if inputChar == 0x7F {		      	// Backspace
		bufferPos = bufferPos - 1;
		if bufferPos < 0 {bufferPos=0;}
	} else {inputBuffer[bufferPos++] = inputChar;}
			
	
	// Prüfen ob Punkt oder Entertaste eingelesen wurde
	if inputChar == '\r' || inputChar == '.' {
    // falls ja, ist die Zeileneingabe beendet
		inputBuffer[bufferPos] = 0;     // String abschließen
		bufferPos = 0;							    // Einlesen beginnt von vorne
    cmdflag = 1;							      // Kommandoausführung im Hauptprogramm
		WriteString("\n\r");				    // neue Zeile
	}
}
