#![no_std]

use rustuino::*;

pub fn InitTIM3_PWM() {
  // Initialisierung des Timer 3

	// GPIO Init
	// PA6 Servo, PA7 R, PC8 G, PC9 B
	RCC->AHB1ENR |= RCC_AHB1ENR_GPIOAEN;		// GPIOA Takt einschalten
	GPIOA->MODER &= 0xFFFF0FFF;							// GPIOA 6/7 auf Alternate Function schalten
	GPIOA->MODER |= 0x0000A000;
	GPIOA->AFR[0] &= 0x00FFFFFF;						// GPIOA 6/7 AF auf Timer 3 schalten (s. Tab 11 Datenblatt)
	GPIOA->AFR[0] |= 0x22000000;

	RCC->AHB1ENR |= RCC_AHB1ENR_GPIOCEN;		// GPIOC Takt einschalten
	GPIOC->MODER &= 0xFFF0FFFF;							// GPIOC 8/9 auf Alternate Function schalten
	GPIOC->MODER |= 0x000A0000;
	GPIOC->AFR[1] &= 0xFFFFFF00;						// GPIOC AF auf Timer 3 schalten (s. Tab 11 Datenblatt)
	GPIOC->AFR[1] |= 0x00000022;

	// Timer init
	RCC->APB1ENR |= RCC_APB1ENR_TIM3EN;   	// Timer 3 Takt einschalten

	// Aufgabe 1:
	// ARR: 100 Schritte zwischen 1ms und 2ms für 20ms Periodendauer = 2000
	// PRE: 16MHz * (1ms/100) = 160
	// CCR: 100 - 200

	TIM3->ARR = 2000;												// Auto-reload-Register laden
	TIM3->PSC = 160;												// Prescaler laden
	TIM3->CCMR1 |= 0x6868;									// Compare 1 & 2 preload enable + PWM Mode 1
	TIM3->CCMR2 |= 0x6868;									// Compare 3 & 4 preload enable + PWM Mode 1
	TIM3->CCER |= 0x1111;										// Enable alle PWM Kanäle
	// TIM3->CCER |= 0x3331;										// Enable alle PWM Kanäle und 2, 3, und 4 als low-aktiv
	TIM3->EGR = 0x1;												// Update events aktivieren
	TIM3->CR1 |= 0x1;												// Zähler starten

}

pub fn TIM3_servo(pos: u32) {
  // Servo auf Position fahren (0...100)
  if pos < 0 || pos > 100 {return;}				// Abbruch, weil ausserhalb des Wertebereichs
	
  (*TIM3_PTR).ccr1.write(|w| w.bits(pos + 100));
}


pub fn TIM3_RGB(red: u32, green: u32, blue: u32) {
  // RGB-LED-Helligkeit setzen (0...255)

	// 2000 / 255 ~= 7 (7 * 255 = 1785 )	gut genug, man kann den unterschied eh nicht sehen
  (*TIM3_PTR).ccr2.write(|w| w.bits(7 * red));      // CCR 2 Register laden
  (*TIM3_PTR).ccr3.write(|w| w.bits(7 * green));    // CCR 3 Register laden
  (*TIM3_PTR).ccr4.write(|w| w.bits(7 * blue));     // CCR 4 Register laden
}
