#!/bin/env fish
set start (nm -a target/armebv7r-none-eabi/debug/tms570 | grep _SEGGER_RTT | awk '{print $1}' | sed 's/^/0x/')
set end (math -b 16 $start + 0x40)
echo "mon rtt ram $start $end" > rtt.gdb
echo "set \$segger=$start" >> rtt.gdb
