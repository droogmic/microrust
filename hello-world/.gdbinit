# Connects GDB to OpenOCD server port
target remote :3333
# (optional) Unmangle function names when debugging
set print asm-demangle on
# Enable semihosting
monitor arm semihosting enable
# Load your program, breaks at entry
load
# (optional) Add breakpoint at function
break hello::main
# Continue with execution
continue