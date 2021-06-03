#![no_std]
#![no_main]

use rustuino::*;

#[entry]
fn main() -> ! {
  let output = 0x100;							// Leuchtmuster LED
  let mut dir = 0;								// Laufrichtung Lauflicht
  
  // char buffer = 0;
  
  (*RCC_PTR).ahb1enr.modify(|_, w| w.gpioben().enabled());
  (*GPIOB_PTR).moder.modify(|_, w| w.bits(0x5555));         // PB0...7 als Ausgang
  (*GPIOB_PTR).odr.write(|w| w.bits(output));               // LED einschalten
  
  (*SYSTICK_PTR).load.write(|w| w.bits((16000/8) * 100));   // Startwartezeit Lauflicht 100ms
  (*SYSTICK_PTR).ctrl.modify(|_, w| w.bits(0x03));          // Systick starten, Interrupt, Systemtakt/8								
    
  InitUSART2();
    
  WriteString("\n\r\n\rV2.3 Nucleo\n\r");
  
  loop {
    // buffer = ReadChar();
    // WriteChar(buffer + 1);
  }
}


#[exception]
fn SysTick() {
  if dir == 1 {
    output = output >> 1;		                    // nach rechts schieben
    if (output < 0x0001) {output = 0x0080;}			// Überlauf
  }
  else {
    output = output <<1;		                    // nach links schieben
    if (output > 0x0080) {output = 0x0001;}			// Überlauf
  }

  (*GPIOB_PTR).odr.write(|w| w.bits(output));   // LED ausgeben
}
