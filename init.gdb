server define init
set $r0=0x00000000
set $r1=0x00000000
set $r2=0x00000001
set $r7=0x87654312
set $r8=0xabcdef01
set $r10=0x99887766
set $r11=0x55667788
set $r12=0xa1b2c3d4
set $r9=0x08026df8
set $sp=0x08040000
set $lr=0x0801fffd
set $pc=0x08020000
set $cpsr=0x1d7
set *(0x0801fffc as *mut u32)=0xe1200070
set *(0x0801fff8 as *mut u32)=0xeafffffd
set *(0x0801fff4 as *mut u32)=0xe1a00000
set *(0x0801fffc as *mut u32)=0xbe00be00
echo Running RTT...\n
rtt
echo Done\n
end

server define rtt
echo Sourcing rtt.gdb...\n
source rtt.gdb
echo Enabling RTT...\n
mon rtt en
end

server define r
mon j
att 1
rtt
end

set mem inaccessible-by-default off
load
init