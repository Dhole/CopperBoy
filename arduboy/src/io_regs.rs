pub const PINB: u16 = 0x23;
pub const DDRB: u16 = 0x24;
pub const PORTB: u16 = 0x25;
pub const PINC: u16 = 0x26;
pub const DDRC: u16 = 0x27;
pub const PORTC: u16 = 0x28;
pub const PIND: u16 = 0x29;
pub const DDRD: u16 = 0x2a;
pub const PORTD: u16 = 0x2b;
pub const PINE: u16 = 0x2c;
pub const DDRE: u16 = 0x2d;
pub const PORTE: u16 = 0x2e;
pub const PINF: u16 = 0x2f;
pub const DDRF: u16 = 0x30;
pub const PORTF: u16 = 0x31;

pub const TIFR1: u16 = 0x35;
pub const TIFR0: u16 = 0x36;

pub const TIFR3: u16 = 0x38;
pub const TIFR4: u16 = 0x39;

pub const PCIFR: u16 = 0x3b;
pub const EIFR: u16 = 0x3c;
pub const EIMSK: u16 = 0x3d;
pub const GPIOR0: u16 = 0x3e;
pub const EECR: u16 = 0x3f;
pub const EEDR: u16 = 0x40;
pub const EEARL: u16 = 0x41;
pub const EEARH: u16 = 0x42;
pub const GTCCR: u16 = 0x43;
pub const TCCR0A: u16 = 0x44;
pub const TCCR0B: u16 = 0x45;
pub const TCNT0: u16 = 0x46;
pub const OCR0A: u16 = 0x47;
pub const OCR0B: u16 = 0x48;
pub const PLLCSR: u16 = 0x49;
pub const GPIOR1: u16 = 0x4a;
pub const GPIOR2: u16 = 0x4b;
pub const SPCR: u16 = 0x4c;
pub const SPSR: u16 = 0x4d;
pub const SPDR: u16 = 0x4e;

pub const ACSR: u16 = 0x50;
pub const OCDR_MONDR: u16 = 0x51;
pub const PLLFRQ: u16 = 0x52;
pub const SMCR: u16 = 0x53;
pub const MCUSR: u16 = 0x54;
pub const MCUCR: u16 = 0x55;

pub const SPMCSR: u16 = 0x57;

pub const RAMPZ: u16 = 0x5b;

pub const SPL: u16 = 0x5d;
pub const SPH: u16 = 0x5e;
pub const SREG: u16 = 0x5f;
pub const WDTCSR: u16 = 0x60;
pub const CLKPR: u16 = 0x61;

pub const PRR0: u16 = 0x64;
pub const PRR1: u16 = 0x65;
pub const OSCCAL: u16 = 0x66;
pub const RCCTRL: u16 = 0x67;
pub const PCICR: u16 = 0x68;
pub const EICRA: u16 = 0x69;
pub const EICRB: u16 = 0x6a;

pub const TIMSK0: u16 = 0x6e;
pub const TIMSK1: u16 = 0x6f;

pub const TIMSK3: u16 = 0x71;
pub const TIMSK4: u16 = 0x72;

pub const ADCL: u16 = 0x78;
pub const ADCH: u16 = 0x79;
pub const ADCSRA: u16 = 0x7a;
pub const ADCSRB: u16 = 0x7b;
pub const ADMUX: u16 = 0x7c;
pub const DIDR2: u16 = 0x7d;
pub const DIDR0: u16 = 0x7e;
pub const DIDR1: u16 = 0x7f;
pub const TCCR1A: u16 = 0x80;
pub const TCCR1B: u16 = 0x81;
pub const TCCR1C: u16 = 0x82;

pub const TCNT1L: u16 = 0x84;
pub const TCNT1H: u16 = 0x85;
pub const ICR1L: u16 = 0x86;
pub const ICR1H: u16 = 0x87;
pub const OCR1AL: u16 = 0x88;
pub const OCR1AH: u16 = 0x89;
pub const OCR1BL: u16 = 0x8a;
pub const OCR1BH: u16 = 0x8b;
pub const OCR1CL: u16 = 0x8c;
pub const OCR1CH: u16 = 0x8d;

pub const TCCR3A: u16 = 0x90;
pub const TCCR3B: u16 = 0x91;
pub const TCCR3C: u16 = 0x92;

pub const TCNT3L: u16 = 0x94;
pub const TCNT3H: u16 = 0x95;
pub const ICR3L: u16 = 0x96;
pub const ICR3H: u16 = 0x97;
pub const OCR3AL: u16 = 0x98;
pub const OCR3AH: u16 = 0x99;
pub const OCR3BL: u16 = 0x9a;
pub const OCR3BH: u16 = 0x9b;
pub const OCR3CL: u16 = 0x9c;
pub const OCR3CH: u16 = 0x9d;

pub const TWBR: u16 = 0xb8;
pub const TWSR: u16 = 0xb9;
pub const TWAR: u16 = 0xba;
pub const TWDR: u16 = 0xbb;
pub const TWCR: u16 = 0xbc;
pub const TWAMR: u16 = 0xbd;
pub const TCNT4: u16 = 0xbe;
pub const TC4H: u16 = 0xbf;
pub const TCCR4A: u16 = 0xc0;
pub const TCCR4B: u16 = 0xc1;
pub const TCCR4C: u16 = 0xc2;
pub const TCCR4D: u16 = 0xc3;
pub const TCCR4E: u16 = 0xc4;
pub const CLKSEL0: u16 = 0xc5;
pub const CLKSEL1: u16 = 0xc6;
pub const CLKSTA: u16 = 0xc7;
pub const UCSR1A: u16 = 0xc8;
pub const UCSR1B: u16 = 0xc9;
pub const UCSR1C: u16 = 0xca;

pub const UBRR1L: u16 = 0xcc;
pub const UBRR1H: u16 = 0xcd;
pub const UDR1: u16 = 0xce;
pub const OCR4A: u16 = 0xcf;
pub const OCR4B: u16 = 0xd0;
pub const OCR4C: u16 = 0xd1;
pub const OCR4D: u16 = 0xd2;

pub const DT4: u16 = 0xd4;

pub const UHWCON: u16 = 0xd7;
pub const USBCON: u16 = 0xd8;
pub const USBSTA: u16 = 0xd9;
pub const USBINT: u16 = 0xda;

pub const UDCON: u16 = 0xe0;
pub const UDINT: u16 = 0xe1;
pub const UDIEN: u16 = 0xe2;
pub const UDADDR: u16 = 0xe3;
pub const UDFNUML: u16 = 0xe4;
pub const UDFNUMH: u16 = 0xe5;
pub const UDMFN: u16 = 0xe6;

pub const UEINTX: u16 = 0xe8;
pub const UENUM: u16 = 0xe9;
pub const UERST: u16 = 0xea;
pub const UECONX: u16 = 0xeb;
pub const UECFG0X: u16 = 0xec;
pub const UECFG1X: u16 = 0xed;
pub const UESTA0X: u16 = 0xee;
pub const UESTA1X: u16 = 0xef;
pub const UEIENX: u16 = 0xf0;
pub const UEDATX: u16 = 0xf1;
pub const UEBCLX: u16 = 0xf2;
pub const UEBCHX: u16 = 0xf3;
pub const UEINT: u16 = 0xf4;

pub fn io_reg_str(addr: u16) -> Option<(&'static str, &'static str)> {
    match addr {
        0x23 => Some(("PINB", "Port B Input Pins Address")),
        0x24 => Some(("DDRB", "Port B Data Direction Register")),
        0x25 => Some(("PORTB", "Port B Data Register")),
        0x26 => Some(("PINC", "Port C Input Pins Address")),
        0x27 => Some(("DDRC", "Port C Data Direction Register")),
        0x28 => Some(("PORTC", "Port C Data Register")),
        0x29 => Some(("PIND", "Port D Input Pins Address")),
        0x2a => Some(("DDRD", "Port D Data Direction Register")),
        0x2b => Some(("PORTD", "Port D Data Register")),
        0x2c => Some(("PINE", "Port E Input Pins Address")),
        0x2d => Some(("DDRE", "Port E Data Direction Register")),
        0x2e => Some(("PORTE", "Port E Data Register")),
        0x2f => Some(("PINF", "Port F Input Pins Address")),
        0x30 => Some(("DDRF", "Port F Data Direction Register")),
        0x31 => Some(("PORTF", "Port F Data Register")),
        0x35 => Some(("TIFR1", "Timer/Counter 1 Interrupt Flag Register")),
        0x36 => Some(("TIFR0", "Timer/Counter 0 Interrupt Flag Register")),
        0x38 => Some(("TIFR3", "Timer/Counter 3 Interrupt Flag Register")),
        0x39 => Some(("TIFR4", "Timer/Counter 4 Interrupt Flag Register")),
        0x3b => Some(("PCIFR", "Pin Change Interrupt Flag Register")),
        0x3c => Some(("EIFR", "External Interrupt Flag Register")),
        0x3d => Some(("EIMSK", "External Interrupt Mask Register")),
        0x3e => Some(("GPIOR0", "General Purpose I/O Register 0")),
        0x3f => Some(("EECR", "EEPROM Control Register")),
        0x40 => Some(("EEDR", "EEPROM Data Register")),
        0x41 => Some(("EEARL", "EEPROM Address Register Low")),
        0x42 => Some(("EEARH", "EEPROM Address Register High")),
        0x43 => Some(("GTCCR", "General Timer/Counter Control Register")),
        0x44 => Some(("TCCR0A", "Timer/Counter Control Register A")),
        0x45 => Some(("TCCR0B", "Timer/Counter Control Register B")),
        0x46 => Some(("TCNT0", "Timer/Counter Regsiter")),
        0x47 => Some(("OCR0A", "Output Compare Register A")),
        0x48 => Some(("OCR0B", "Output Compare Register B")),
        0x49 => Some(("PLLCSR", "PLL Control and Status Register")),
        0x4a => Some(("GPIOR1", "General Purpose I/O Register 1")),
        0x4b => Some(("GPIOR2", "General Purpose I/O Register 2")),
        0x4c => Some(("SPCR", "SPI Control Register")),
        0x4d => Some(("SPSR", "SPI Status Register")),
        0x4e => Some(("SPDR", "SPI Data Register")),
        0x50 => Some(("ACSR", "Analog Comparator Control and Status Register")),
        0x51 => Some((
            "OCDR_MONDR",
            "On-chip Debug Register / Monitor Data Register",
        )),
        0x52 => Some(("PLLFRQ", "PLL Frequency Control Register")),
        0x53 => Some(("SMCR", "Sleep Mode Control Register")),
        0x54 => Some(("MCUSR", "MCU Status Register")),
        0x55 => Some(("MCUCR", "MCU Control Register")),
        0x57 => Some(("SPMCSR", "Store Program Memory Control and Status Register")),
        0x5b => Some(("RAMPZ", "Extended Z-pointer Register for ELPM/SPM")),
        0x5d => Some(("SPL", "Stack Pointer Low")),
        0x5e => Some(("SPH", "Stack Pointer High")),
        0x5f => Some(("SREG", "Status Regsiter")),
        0x60 => Some(("WDTCSR", "Watchdog Timer Control Register")),
        0x61 => Some(("CLKPR", "Clock Prescaler Register")),
        0x64 => Some(("PRR0", "Power Reduction Register 0")),
        0x65 => Some(("PRR1", "Power Reduction Register 1")),
        0x66 => Some(("OSCCAL", "Oscillator Calibration Register")),
        0x67 => Some(("RCCTRL", "Oscillator Control Register")),
        0x68 => Some(("PCICR", "Pin Change Interrupt Control Register")),
        0x69 => Some(("EICRA", "External Interrupt Control Register A")),
        0x6a => Some(("EICRB", "External Interrupt Control Register B")),
        0x6e => Some(("TIMSK0", "Timer/Counter Interrupt Mask Register")),
        0x6f => Some(("TIMSK1", "Timer/Counter1 Interrupt Mask Register")),
        0x71 => Some(("TIMSK3", "Timer/Counter3 Interrupt Mask Register")),
        0x72 => Some(("TIMSK4", "Timer/Counter4 Interrupt Mask Register")),
        0x78 => Some(("ADCL", "ADC Data Register Low")),
        0x79 => Some(("ADCH", "ADC Data Register High")),
        0x7a => Some(("ADCSRA", "ADC Control and Status Regsiter A")),
        0x7b => Some(("ADCSRB", "ADC Control and Status Regsiter B")),
        0x7c => Some(("ADMUX", "ADC Multiplexer Selection Register")),
        0x7d => Some(("DIDR2", "Digital Input Disable Register 2")),
        0x7e => Some(("DIDR0", "Digital Input Disable Register 0")),
        0x7f => Some(("DIDR1", "Digital Input Disable Register 1")),
        0x80 => Some(("TCCR1A", "Timer/Counter1 Control Register A")),
        0x81 => Some(("TCCR1B", "Timer/Counter1 Control Register B")),
        0x82 => Some(("TCCR1C", "Timer/Counter1 Control Register C")),
        0x84 => Some(("TCNT1L", "Timer/Counter1 Low")),
        0x85 => Some(("TCNT1H", "Timer/Counter1 High")),
        0x86 => Some(("ICR1L", "Timer/Counter1 Input Capture Register Low")),
        0x87 => Some(("ICR1H", "Timer/Counter1 Input Capture Register High")),
        0x88 => Some(("OCR1AL", "Timer/Counter1 Output Compare Register A Low")),
        0x89 => Some(("OCR1AH", "Timer/Counter1 Output Compare Register A High")),
        0x8a => Some(("OCR1BL", "Timer/Counter1 Output Compare Register B Low")),
        0x8b => Some(("OCR1BH", "Timer/Counter1 Output Compare Register B High")),
        0x8c => Some(("OCR1CL", "Timer/Counter1 Output Compare Register C Low")),
        0x8d => Some(("OCR1CH", "Timer/Counter1 Output Compare Register C High")),
        0x90 => Some(("TCCR3A", "Timer/Counter3 Control Register A")),
        0x91 => Some(("TCCR3B", "Timer/Counter3 Control Register B")),
        0x92 => Some(("TCCR3C", "Timer/Counter3 Control Register C")),
        0x94 => Some(("TCNT3L", "Timer/Counter3 Low")),
        0x95 => Some(("TCNT3H", "Timer/Counter3 High")),
        0x96 => Some(("ICR3L", "Timer/Counter3 Input Capture Register High")),
        0x97 => Some(("ICR3H", "Timer/Counter3 Input Capture Register High")),
        0x98 => Some(("OCR3AL", "Timer/Counter3 Output Compare Register A Low")),
        0x99 => Some(("OCR3AH", "Timer/Counter3 Output Compare Register A High")),
        0x9a => Some(("OCR3BL", "Timer/Counter3 Output Compare Register B Low")),
        0x9b => Some(("OCR3BH", "Timer/Counter3 Output Compare Register B High")),
        0x9c => Some(("OCR3CL", "Timer/Counter3 Output Compare Register C Low")),
        0x9d => Some(("OCR3CH", "Timer/Counter3 Output Compare Register C High")),
        0xb8 => Some(("TWBR", "TWI Bit Rate Register")),
        0xb9 => Some(("TWSR", "TWI Status Register")),
        0xba => Some(("TWAR", "TWI (Slave) Address Register")),
        0xbb => Some(("TWDR", "TWI Data Register")),
        0xbc => Some(("TWCR", "TWI Control Register")),
        0xbd => Some(("TWAMR", "TWI (Slave) Address Mask Register")),
        0xbe => Some(("TCNT4", "Timer/Counter4")),
        0xbf => Some(("TC4H", "Timer/Counter4 High Byte")),
        0xc0 => Some(("TCCR4A", "Timer/Counter4 Control Register A")),
        0xc1 => Some(("TCCR4B", "Timer/Counter4 Control Register B")),
        0xc2 => Some(("TCCR4C", "Timer/Counter4 Control Register C")),
        0xc3 => Some(("TCCR4D", "Timer/Counter4 Control Register D")),
        0xc4 => Some(("TCCR4E", "Timer/Counter4 Control Register E")),
        0xc5 => Some(("CLKSEL0", "Clock Selection Register 0")),
        0xc6 => Some(("CLKSEL1", "Clock Selection Register 1")),
        0xc7 => Some(("CLKSTA", "Clock Status Register")),
        0xc8 => Some(("UCSR1A", "USART1 Control and Status Register A")),
        0xc9 => Some(("UCSR1B", "USART1 Control and Status Register B")),
        0xca => Some(("UCSR1C", "USART1 Control and Status Register C")),
        0xcc => Some(("UBRR1L", "USART1 Baud Rate Regsiters Low")),
        0xcd => Some(("UBRR1H", "USART1 Baud Rate Regsiters High")),
        0xce => Some(("UDR1", "USART1 I/O Data Register")),
        0xcf => Some(("OCR4A", "Timer/Counter4 Output Compare Register A")),
        0xd0 => Some(("OCR4B", "Timer/Counter4 Output Compare Register B")),
        0xd1 => Some(("OCR4C", "Timer/Counter4 Output Compare Register C")),
        0xd2 => Some(("OCR4D", "Timer/Counter4 Output Compare Register D")),
        0xd4 => Some(("DT4", "Timer/Counter4 Dead Time Value")),
        0xd7 => Some(("UHWCON", "USB Hardware Control Regsiter")),
        0xd8 => Some(("USBCON", "USB Control Regsiter")),
        0xd9 => Some(("USBSTA", "USB Status Regsiter")),
        0xda => Some(("USBINT", "USB Interrupt Regsiter")),
        0xe0 => Some(("UDCON", "USB Device Control Regsiter")),
        0xe1 => Some(("UDINT", "USB Device Interrupt Regsiter")),
        0xe2 => Some(("UDIEN", "USB Device Interrupt Enable Regsiter")),
        0xe3 => Some(("UDADDR", "USB Device Address Regsiter")),
        0xe4 => Some(("UDFNUML", "USB Frame Number Register Low")),
        0xe5 => Some(("UDFNUMH", "USB Frame Number Register High")),
        0xe6 => Some(("UDMFN", "USB Frame Number CRC Error Flag")),
        0xe8 => Some(("UEINTX", "USB Endpoint Interrupt Register")),
        0xe9 => Some(("UENUM", "USB Endpoint Number Register")),
        0xea => Some(("UERST", "USB Endpoint Reset Register")),
        0xeb => Some(("UECONX", "USB Endpoint Control Register")),
        0xec => Some(("UECFG0X", "USB Endpoint Configuration 0 Register")),
        0xed => Some(("UECFG1X", "USB Endpoint Configuration 1 Register")),
        0xee => Some(("UESTA0X", "USB Endpoint Status 0 Register")),
        0xef => Some(("UESTA1X", "USB Endpoint Status 1 Register")),
        0xf0 => Some(("UEIENX", "USB Endpoint Interrupt Enable Register")),
        0xf1 => Some(("UEDATX", "USB Endpoint Data Register")),
        0xf2 => Some(("UEBCLX", "USB Endpoint Byte Count Register Low")),
        0xf3 => Some(("UEBCHX", "USB Endpoint Byte Count Register High")),
        0xf4 => Some(("UEINT", "USB Endpoint Interrupts Register")),
        _ => None,
    }
}
