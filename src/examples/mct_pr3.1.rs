#![no_std]
#![no_main]

use rustuino::*;

#[entry]
fn main() -> ! {
  void Help_Display(void);

  //Sinustabelle
  int sinus[32] = { 2048, 2419, 2775, 3104, 3392, 3628, 3803, 3911, 3948,
                    3911, 3803, 3628, 3392, 3104, 2775, 2419, 2048, 1677,
                    1321, 992, 704, 468, 293, 185, 148, 185, 293, 468,
                    704, 992, 1321, 1677};
  
  int amp =9;						// Amplitude Sinus
  int freq =300;				// Frequenz Sinus 300 = Ton a
  
  void ExecuteCmd (void)
  // Ausführung des eingegebenen Befehls
  {
    int pos, wert;			// Position und Wert in der Sinustabelle
    int puls;						// Position Servo
    int r,g,b;					// Werte für RGB-LED
    char led;						// Wert für einzelne RGB-LED
  
    switch (inputBuffer[0])		// erster Buchstabe des Befehls
    {
      // Sinuswert ändern
      case 'p':
        sscanf(inputBuffer,"p%dw%d",&pos,&wert);
        sinus[pos] = wert;
        break;
      // Frequenz/Amplitude ändern
      case 'f':
        sscanf(inputBuffer,"f%da%d",&freq,&amp);
        break;	
  
      // Servo auf Position fahren
      case 's':
        sscanf(inputBuffer,"s%d", &puls);
        TIM3_servo(puls);
        WriteString("Neue Servo Position...\r\n");
        break;
      // RGB LED farbe komplett einstellen
      case 'r':
        sscanf(inputBuffer,"r%dg%db%d", &r, &g, &b);
        TIM3_RGB(r, g, b);
        WriteString("Neuer RGB Wert...\r\n");
        break;
      // RGB LED farben individuel einstellen
      case 'l':
        sscanf(inputBuffer,"l%c%d", &led, &wert);
        if(led == 'r') TIM3_RGB(wert, g, b);
        else if(led == 'g') TIM3_RGB(r, wert, b);
        else if(led == 'b') TIM3_RGB(r, g, wert);
        WriteString("Neuer RGB Wert...\r\n");
        break;
      // Schrittmotor rechtslauf
      case '+':
        stepdir = 0;
        stepmode = STEPMODE_NONE;
        WriteString("Schrittmotor Rechtslauf...\r\n");
        break;
      // Schrittmotor linkslauf
      case '-':
        stepdir = 1;
        stepmode = STEPMODE_NONE;
        WriteString("Schrittmotor Linkslauf...\r\n");
        break;
      // Schrittmotor auf Position fahren bzw Referenz finden
      case 'm':
        stepmode = STEPMODE_POS;
        if(inputBuffer[1] == 'r') ziel_pos = 0;
        else sscanf(inputBuffer,"m%d", &ziel_pos);
        WriteString("Schrittmotor auf Referenz...\r\n");
        break;
      
      // Anzeige Kommandos
      case 'h':
        Help_Display();
        break;	
    }
  }
  
  void Help_Display(void)
  // Anzeige des Hilfetextes
  {
    WriteString("Kommandos:\n\r");
    WriteString("\tp[pos]w[wert]:\t\tSinustabelle: an Stelle pos den Wert wert setzen\n\r");
    WriteString("\tf[freq]a[amp]:\t\tSinustabelle: Frequenz und Amplitude [1-9]  setzen\n\r");
    WriteString("\ts[pos]:\t\t\tServo auf Position pos fahren\n\r");
    WriteString("\tr[r]g[g]b[b]\t\tRGB-LED auf Farbe r/g/b setzen [0...255]\n\r");
    WriteString("\tl[r/g/b][wert]:\t\tBei RGB-LED Farbe r/g/b auf Wert wert setzen\n\r");
    WriteString("\t+:\t\t\tSchrittmotor dreht vorwaerts\n\r");
    WriteString("\t-:\t\t\tSchrittmotor dreht rueckwaerts\n\r");
    WriteString("\tm[pos]\t\t\tSchrittmotor auf Position pos fahren\n\r");
    WriteString("\tmr\t\t\tReferenzfahrt Schrittmotor\n\r");
  }
  
  void delay (int del) 
  // einige µs warten	
  {
    int  d1;
     for (d1 = 0; d1< del; d1++);
  }
  
  /*----------------------------------------------------------------------------
    Main Program
   *----------------------------------------------------------------------------*/
  int main (void)
  { 	
    int i;											// Zählervariable
    int sin_a;
    
    //------- Init ----------
    InitDAC();
    InitUSART2();
    InitTIM3_PWM();
    InitStepper();
    
    WriteString("\n\r\n\rV3.1 Nucleo\n\r");	
    
    while (1) 
    {
      // Sinus ausgeben
      for (i=0;i<32;i++)
      {
        delay(freq);
        sin_a = sinus[i]/(10-amp%10); // sin wird durch (1 bis 9) geteilt
        WriteDAC(sin_a);
        if (cmdflag == 1)			// anliegendes Eingabekommando
        {
          ExecuteCmd();
          cmdflag = 0;
        }
      }
    }
  }  
}
