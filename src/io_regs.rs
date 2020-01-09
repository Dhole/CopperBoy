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
