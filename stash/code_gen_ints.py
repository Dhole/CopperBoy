#!/usr/bin/env python3

int_vecs = """
RESET 0x0000
INT0 0x0002
INT1 0x0004
INT2 0x0006
INT3 0x0008
INT6 0x000e
PCINT0 0x0012
USB_GENERAL 0x0014
USB_ENDPOINT 0x0016
WDT 0x0018
TIMER1_CAPT 0x0020
TIMER1_COMPA 0x0022
TIMER1_COMPB 0x0024
TIMER1_COMPC 0x0026
TIMER1_OVF 0x0028
TIMER0_COMPA 0x002a
TIMER0_COMPB 0x002c
TIMER0_OVF 0x002e
SPI_STC 0x0030
USART1_RX 0x0032
USART2_UDRE 0x0034
USART1_TX 0x0036
ANALOG_COMP 0x0038
ADC 0x003a
EE_READY 0x003c
TIMER3_CAPT 0x003e
TIMER3_COMPA 0x0040
TIMER3_COMPB 0x0042
TIMER3_COMPC 0x0044
TIMER3_OVF 0x0046
TWI 0x0048
STM_READY 0x004a
TIMER4_COMPA 0x004c
TIMER4_COMPB 0x004e
TIMER4_COMPD 0x0050
TIMER4_OVF 0x0052
TIMER4_FPF 0x0054
"""

def rename_opt(opt):
    opt = opt.replace("Compa", "CompA")
    opt = opt.replace("Compb", "CompB")
    opt = opt.replace("Compc", "CompC")
    opt = opt.replace("Compd", "CompD")
    return opt

for int_vec in int_vecs.split('\n'):
    if len(int_vec) == 0 or int_vec[0] == '#':
        continue
    [name, pc] = int_vec.split()
    print(f'pub const {name}: u16 = {pc};')

print()

print('#[derive(ToPrimitive, Clone, Copy)]')
print('pub enum Interrupt {')
i = 0
for int_vec in int_vecs.split('\n'):
    if len(int_vec) == 0 or int_vec[0] == '#':
        continue
    [name, pc] = int_vec.split()
    opt = ''.join([n.title() for n in name.split('_')])
    opt = rename_opt(opt)
    print(f'  {opt} = 1 << {i},')
    i += 1
print('}')

print()

first = True
for int_vec in int_vecs.split('\n'):
    if len(int_vec) == 0 or int_vec[0] == '#':
        continue
    [name, pc] = int_vec.split()
    opt = ''.join([n.title() for n in name.split('_')])
    opt = rename_opt(opt)
    if first:
        first = False
        print('let pc = if', end='')
    else:
        print('} else if', end='')
    print(f' interrupt_bitmap & Interrupt::{opt}.to_u64().unwrap() != 0 {{')
    print(f'  debug!("Handling interrupt {name}");')
    print(f'  {name}')
print('} else {')
print('  unreachable!();')
print('};')
