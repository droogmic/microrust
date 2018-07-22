# GDB cheatsheet

| Short | Command             | Action                                         |
|:----- |:------------------- |:---------------------------------------------- |
| b     | break [location]    | Set breakpoint at specified location.          |
| c     | continue            | Continue program being debugged.               |
| s     | step                | Step program until it reaches a different source line.       |
| si    | stepi               | Step one instruction exactly.                  |
| p     | print               | Print value of expression EXP.                 |
| i lo  | info locals         | Local variables of current stack frame.        |
| la s  | layout src          | Displays source and command windows.           |
| la a  | layout asm          | Displays disassembly and command windows.      |
| tu d  | tui disable         | Disable TUI display mode.                      |
|       | dissasmble /m       | Disassemble a specified section of memory.     |
|       | monitor reset halt  | Send a command to the remote monitor. |
