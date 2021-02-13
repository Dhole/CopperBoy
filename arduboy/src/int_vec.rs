// use num_traits::ToPrimitive;

// Values are pc

pub const RESET: u16 = 0x0000;
pub const INT0: u16 = 0x0002;
pub const INT1: u16 = 0x0004;
pub const INT2: u16 = 0x0006;
pub const INT3: u16 = 0x0008;
pub const INT6: u16 = 0x000e;
pub const PCINT0: u16 = 0x0012;
pub const USB_GENERAL: u16 = 0x0014;
pub const USB_ENDPOINT: u16 = 0x0016;
pub const WDT: u16 = 0x0018;
pub const TIMER1_CAPT: u16 = 0x0020;
pub const TIMER1_COMPA: u16 = 0x0022;
pub const TIMER1_COMPB: u16 = 0x0024;
pub const TIMER1_COMPC: u16 = 0x0026;
pub const TIMER1_OVF: u16 = 0x0028;
pub const TIMER0_COMPA: u16 = 0x002a;
pub const TIMER0_COMPB: u16 = 0x002c;
pub const TIMER0_OVF: u16 = 0x002e;
pub const SPI_STC: u16 = 0x0030;
pub const USART1_RX: u16 = 0x0032;
pub const USART2_UDRE: u16 = 0x0034;
pub const USART1_TX: u16 = 0x0036;
pub const ANALOG_COMP: u16 = 0x0038;
pub const ADC: u16 = 0x003a;
pub const EE_READY: u16 = 0x003c;
pub const TIMER3_CAPT: u16 = 0x003e;
pub const TIMER3_COMPA: u16 = 0x0040;
pub const TIMER3_COMPB: u16 = 0x0042;
pub const TIMER3_COMPC: u16 = 0x0044;
pub const TIMER3_OVF: u16 = 0x0046;
pub const TWI: u16 = 0x0048;
pub const STM_READY: u16 = 0x004a;
pub const TIMER4_COMPA: u16 = 0x004c;
pub const TIMER4_COMPB: u16 = 0x004e;
pub const TIMER4_COMPD: u16 = 0x0050;
pub const TIMER4_OVF: u16 = 0x0052;
pub const TIMER4_FPF: u16 = 0x0054;

#[derive(Clone, Copy)]
pub enum Interrupt {
    Reset,
    Int0,
    Int1,
    Int2,
    Int3,
    Int6,
    Pcint0,
    UsbGeneral,
    UsbEndpoint,
    Wdt,
    Timer1Capt,
    Timer1CompA,
    Timer1CompB,
    Timer1CompC,
    Timer1Ovf,
    Timer0CompA,
    Timer0CompB,
    Timer0Ovf,
    SpiStc,
    Usart1Rx,
    Usart2Udre,
    Usart1Tx,
    AnalogComp,
    Adc,
    EeReady,
    Timer3Capt,
    Timer3CompA,
    Timer3CompB,
    Timer3CompC,
    Timer3Ovf,
    Twi,
    StmReady,
    Timer4CompA,
    Timer4CompB,
    Timer4CompD,
    Timer4Ovf,
    Timer4Fpf,
}

pub const RESET_BIT: u64 = 1 << 0;
pub const INT0_BIT: u64 = 1 << 1;
pub const INT1_BIT: u64 = 1 << 2;
pub const INT2_BIT: u64 = 1 << 3;
pub const INT3_BIT: u64 = 1 << 4;
pub const INT6_BIT: u64 = 1 << 5;
pub const PCINT0_BIT: u64 = 1 << 6;
pub const USB_GENERAL_BIT: u64 = 1 << 7;
pub const USB_ENDPOINT_BIT: u64 = 1 << 8;
pub const WDT_BIT: u64 = 1 << 9;
pub const TIMER1_CAPT_BIT: u64 = 1 << 10;
pub const TIMER1_COMPA_BIT: u64 = 1 << 11;
pub const TIMER1_COMPB_BIT: u64 = 1 << 12;
pub const TIMER1_COMPC_BIT: u64 = 1 << 13;
pub const TIMER1_OVF_BIT: u64 = 1 << 14;
pub const TIMER0_COMPA_BIT: u64 = 1 << 15;
pub const TIMER0_COMPB_BIT: u64 = 1 << 16;
pub const TIMER0_OVF_BIT: u64 = 1 << 17;
pub const SPI_STC_BIT: u64 = 1 << 18;
pub const USART1_RX_BIT: u64 = 1 << 19;
pub const USART2_UDRE_BIT: u64 = 1 << 20;
pub const USART1_TX_BIT: u64 = 1 << 21;
pub const ANALOG_COMP_BIT: u64 = 1 << 22;
pub const ADC_BIT: u64 = 1 << 23;
pub const EE_READY_BIT: u64 = 1 << 24;
pub const TIMER3_CAPT_BIT: u64 = 1 << 25;
pub const TIMER3_COMPA_BIT: u64 = 1 << 26;
pub const TIMER3_COMPB_BIT: u64 = 1 << 27;
pub const TIMER3_COMPC_BIT: u64 = 1 << 28;
pub const TIMER3_OVF_BIT: u64 = 1 << 29;
pub const TWI_BIT: u64 = 1 << 30;
pub const STM_READY_BIT: u64 = 1 << 31;
pub const TIMER4_COMPA_BIT: u64 = 1 << 32;
pub const TIMER4_COMPB_BIT: u64 = 1 << 33;
pub const TIMER4_COMPD_BIT: u64 = 1 << 34;
pub const TIMER4_OVF_BIT: u64 = 1 << 35;
pub const TIMER4_FPF_BIT: u64 = 1 << 36;
