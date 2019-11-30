target remote :3333
monitor arm semihosting enable
set print asm-demangle on
load
break main
continue
