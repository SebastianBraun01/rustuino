# Rustuino
## The Problem
Rust is a languange that has souriously taken off the last couple of years. As a systems development language, embedded devices profit from the better memory safety that rust offers. The Problem is that there are not many good crates that provide a got Hardware Abstraction Layer for many devices. Many crates are spacific for one devices or device family.

## My Vision
### Software
My vision for this project is to provide a Arduino-like Framework for Rust Embedded Development. My crate will provide high level functions to controll gpio-pins or communicate over UART while still offering precise register level control for those wo want to use more advanced functionalities (there will be also functions for advances stuff, don´t worry ;) ).

### Hardware
Because there are many types of embedded devices out there, I want to make this project as portable as possible. I will start with supporting the lineup of the stm32f chips because they are the newest and most popular and the Cortex-M0 chips are the goto for the new arduino boards.

## Roadmap
### Software
- [ ] Basic functions for using the GPIO-pins
- [ ] Delay and time related functions
- [ ] Functions for simple communication over the serial busses
- [ ] Framework for Hardware Interrupts
- [ ] More to come!

### Hardware
- [ ] stm32f0
- [ ] stm32f1
- [ ] stm32f2
- [ ] stm32f3
- [ ] stm32f4
- [ ] stm32f7
- [ ] There will be more!

### Other
- [ ] Begin to write a documentation/reference
- [ ] Clean up Project and make it easier for beginners

## Contributing
I always appreciate bugreports, suggestions, etc., this project will take a lot of time and effort and if someone wants to help I would welcome it.
This is my first open source project and I am still learning Rust, so if you have ideas for better code, make an issue.
Every good embedded framework needs many different libraries for peripheral devices (Neopixel, Temperature Sensors, etc.). This is the biggest way you can contribute.
