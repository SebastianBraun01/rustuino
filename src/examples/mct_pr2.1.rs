#![no_std]
#![no_main]

use rustuino::*;

#[entry]
fn main() -> ! {
  #define milliseconds 16000/8		// LOAD-Wert für eine Millisekunde

  int output =0x100;							// Leuchtmuster LED
  int dir = 0;										// Laufrichtung Lauflicht
  
  /*----------------------------------------------------------------------------
    Main Program
   *----------------------------------------------------------------------------*/
  int main (void)
  { 	
    // char buffer = 0;
    
    //------- Init ----------
    RCC->AHB1ENR |= 0x02;											// Takt für GPIOB einschalten
    GPIOB->MODER = 0x5555;										// PB0...7 als Ausgang
    GPIOB->ODR = output;											// LED einschalten
  
    SysTick->LOAD = milliseconds*100;					// Startwartezeit Lauflicht 100ms
    SysTick->CTRL = 0x03;											// Systick starten, Interrupt, Systemtakt/8
    
    InitUSART2();
    
    WriteString("\n\r\n\rV2.3 Nucleo\n\r");
    
    while (1) 
    {
  
        // buffer = ReadChar();
        // WriteChar(buffer + 1);
  
    }
  }
  
  
  /*----------------------------------------------------------------------------
    SysTick-Handler
   *----------------------------------------------------------------------------*/
  void SysTick_Handler(void)
  {
    if (dir) {
      output = output >> 1;		// nach rechts schieben
      if (output < 0x0001)
        output = 0x0080;			// Überlauf
    } else {
      output = output <<1;		// nach links schieben
      if (output > 0x0080)
        output = 0x0001;			// Überlauf
    }
    GPIOB->ODR = output;			// LED ausgeben
  }
  
}
