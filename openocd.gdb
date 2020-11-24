# connect to the microcontroller
target remote :3333

# halt the microcontroller to programm it
monitor reset halt

# flash the programm onto the microcontroller
load

# set a break point on the main function to skip the initialization
break main

# start the execution; the programm will halt at the breakpoint at main
continue
