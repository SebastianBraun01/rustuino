#![no_std]

use rustuino::*;

#define schrittmotor_steps 4096

int stepdir = 0;  						// Richtung des Schrittmotors
int akt_pos=0;								// aktuelle Position des Schrittmotors
int ziel_pos=0;								// Zielposition des Schrittmotors
int stepmode=STEPMODE_NONE;		// Modus des Schrittmotors


pub fn InitSysTick(load: u32) {
  // initialisiert SysTick mit load und startet ihn
  (*SYSTICK_PTR).load.write(|w| w.bits(load));
  (*SYSTICK_PTR).ctrl.modify(|_, w| {
    w.interrupt().enabled();
    w.clocksource().enabled();
    w.enable().enabled()
  });
}


pub fn InitStepper()  {
  // initialisiert Schrittmotorausgänge GPIOB0...3 und Referenzeingang GPIOB4
  (*RCC_PTR).ahb1enr.modify(|_, w| w.gpioben().enabled());
  (*GPIOB_PTR).moder.modify(|r, w| w.bits(r.bits() & 0xFFFFF000 | 0x00000055));

	InitSysTick((16000000 / schrittmotor_steps) * 6);						// 6s für eine Umdrehung
}

pub fn Step_Out(step_pattern: u32) {
  // gibt das Schrittmotormuster step_pattern an Motor aus
	let pattern_map: Vec<u8, 8> = Vec::new();
  pattern_map = {0x0001, 0x0003, 0x0002, 0x0006, 0x0004, 0x000C, 0x0008, 0x0009};

  (*GPIOB_PTR).odr.modify(|r, w| w.bits(r.bits() & 0xFFF0 | pattern_map[step_pattern]));	
}


#[exception]
fn SysTick() {
	static pattern_pos: u32 = 0;

	// Aufgabe 3.3 a)
	if stepmode == STEPMODE_NONE {
		// 0 = rechtslauf, 1 = linkslauf
		if stepdir == 1 {
			Step_Out(pattern_pos);
			if pattern_pos < 7 {pattern_pos += 1;}
			else {pattern_pos = 0;}
		}
		else {
			Step_Out(pattern_pos);
			if pattern_pos > 0 {pattern_pos -= 1;}
			else {pattern_pos = 7;}
		}
	}
	// Aufgabe 3.3 b)
	else if stepmode == STEPMODE_POS {
		// ziel > akt = rechtslauf, ziel < akt = linkslauf
		if ziel_pos < akt_pos {
			Step_Out(pattern_pos);
			if pattern_pos < 7 {pattern_pos += 1;}
			else {pattern_pos = 0;}
			akt_pos -= 1;
		}
		else if ziel_pos > akt_pos {
			Step_Out(pattern_pos);
			if pattern_pos > 0 {pattern_pos -= 1;}
			else {pattern_pos = 7;}
			akt_pos += 1;
		}
	}
}
