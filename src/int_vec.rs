use num_traits::ToPrimitive;

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

#[derive(ToPrimitive, Clone, Copy)]
pub enum Interrupt {
    Reset = 1 << 0,
    Int0 = 1 << 1,
    Int1 = 1 << 2,
    Int2 = 1 << 3,
    Int3 = 1 << 4,
    Int6 = 1 << 5,
    Pcint0 = 1 << 6,
    UsbGeneral = 1 << 7,
    UsbEndpoint = 1 << 8,
    Wdt = 1 << 9,
    Timer1Capt = 1 << 10,
    Timer1CompA = 1 << 11,
    Timer1CompB = 1 << 12,
    Timer1CompC = 1 << 13,
    Timer1Ovf = 1 << 14,
    Timer0CompA = 1 << 15,
    Timer0CompB = 1 << 16,
    Timer0Ovf = 1 << 17,
    SpiStc = 1 << 18,
    Usart1Rx = 1 << 19,
    Usart2Udre = 1 << 20,
    Usart1Tx = 1 << 21,
    AnalogComp = 1 << 22,
    Adc = 1 << 23,
    EeReady = 1 << 24,
    Timer3Capt = 1 << 25,
    Timer3CompA = 1 << 26,
    Timer3CompB = 1 << 27,
    Timer3CompC = 1 << 28,
    Timer3Ovf = 1 << 29,
    Twi = 1 << 30,
    StmReady = 1 << 31,
    Timer4CompA = 1 << 32,
    Timer4CompB = 1 << 33,
    Timer4CompD = 1 << 34,
    Timer4Ovf = 1 << 35,
    Timer4Fpf = 1 << 36,
}
