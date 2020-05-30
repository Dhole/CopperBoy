use log::{debug, warn};
use num_traits::ToPrimitive;

use super::clock;
use super::display;
use super::int_vec::*;
use super::io_regs::io_reg_str;
use super::opcodes::*;
use super::*;

#[derive(PartialEq)]
struct StatusRegister {
    /// Global Interrupt Enable
    i: bool,
    /// Bit Copy Storage
    t: bool,
    /// Half Carry Flag
    h: bool,
    /// Sign Bit
    s: bool,
    /// Two's Complement Overflow Flag
    v: bool,
    /// Negative Flag
    n: bool,
    /// Zero Flag
    z: bool,
    /// Carry Flag
    c: bool,
}

impl Index<u8> for StatusRegister {
    type Output = bool;

    fn index(&self, i: u8) -> &bool {
        match i {
            0 => &self.c,
            1 => &self.z,
            2 => &self.n,
            3 => &self.v,
            4 => &self.s,
            5 => &self.h,
            6 => &self.t,
            7 => &self.i,
            _ => unreachable!(),
        }
    }
}

impl IndexMut<u8> for StatusRegister {
    fn index_mut(&mut self, i: u8) -> &mut bool {
        match i {
            0 => &mut self.c,
            1 => &mut self.z,
            2 => &mut self.n,
            3 => &mut self.v,
            4 => &mut self.s,
            5 => &mut self.h,
            6 => &mut self.t,
            7 => &mut self.i,
            _ => unreachable!(),
        }
    }
}

impl fmt::Debug for StatusRegister {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "StatusRegister {{ i: {}, t: {}, h: {}, s: {}, v: {}, n: {}, z: {}, c: {} }}",
            self.i as u8,
            self.t as u8,
            self.h as u8,
            self.s as u8,
            self.v as u8,
            self.n as u8,
            self.z as u8,
            self.c as u8
        )
    }
}

impl StatusRegister {
    fn new() -> Self {
        Self {
            i: false,
            t: false,
            h: false,
            s: false,
            v: false,
            n: false,
            z: false,
            c: false,
        }
    }

    fn from_u8(v: u8) -> Self {
        Self {
            i: v & (1 << 7) != 0,
            t: v & (1 << 6) != 0,
            h: v & (1 << 5) != 0,
            s: v & (1 << 4) != 0,
            v: v & (1 << 3) != 0,
            n: v & (1 << 2) != 0,
            z: v & (1 << 1) != 0,
            c: v & (1 << 0) != 0,
        }
    }

    fn as_u8(&self) -> u8 {
        (self.i as u8) << 7
            | (self.t as u8) << 6
            | (self.h as u8) << 5
            | (self.s as u8) << 4
            | (self.v as u8) << 3
            | (self.n as u8) << 2
            | (self.z as u8) << 1
            | (self.c as u8) << 0
    }
}

#[derive(PartialEq, Debug)]
pub struct GeneralRegisters {
    reg: [u8; 32],
}

impl GeneralRegisters {
    fn new() -> Self {
        Self { reg: [0; 32] }
    }
}

impl Index<u8> for GeneralRegisters {
    type Output = u8;

    fn index(&self, i: u8) -> &u8 {
        &self.reg[i as usize]
    }
}

impl IndexMut<u8> for GeneralRegisters {
    fn index_mut(&mut self, i: u8) -> &mut u8 {
        &mut self.reg[i as usize]
    }
}

impl GeneralRegisters {
    fn w(&self) -> u16 {
        u16::from_le_bytes([self[24], self[25]])
    }
    fn set_w(&mut self, v: u16) {
        let bytes = v.to_le_bytes();
        self[24] = bytes[0];
        self[25] = bytes[1];
    }
    pub fn x(&self) -> u16 {
        u16::from_le_bytes([self[26], self[27]])
    }
    fn set_x(&mut self, v: u16) {
        let bytes = v.to_le_bytes();
        self[26] = bytes[0];
        self[27] = bytes[1];
    }
    fn y(&self) -> u16 {
        u16::from_le_bytes([self[28], self[29]])
    }
    fn set_y(&mut self, v: u16) {
        let bytes = v.to_le_bytes();
        self[28] = bytes[0];
        self[29] = bytes[1];
    }
    pub fn z(&self) -> u16 {
        u16::from_le_bytes([self[30], self[31]])
    }
    fn set_z(&mut self, v: u16) {
        let bytes = v.to_le_bytes();
        self[30] = bytes[0];
        self[31] = bytes[1];
    }

    fn ext(&self, i: u8) -> u16 {
        u16::from_le_bytes([self[i], self[i + 1]])
    }

    fn set_ext(&mut self, i: u8, v: u16) {
        let bytes = v.to_le_bytes();
        self[i] = bytes[0];
        self[i + 1] = bytes[1];
    }
}

fn get_hi(v: u16) -> u8 {
    ((v & 0xff00) >> 8) as u8
}

fn get_lo(v: u16) -> u8 {
    (v & 0x00ff) as u8
}

fn set_hi(v: &mut u16, hi: u8) {
    *v &= 0x00ff;
    *v |= (hi as u16) << 8;
}

fn set_lo(v: &mut u16, lo: u8) {
    *v &= 0xff00;
    *v |= lo as u16;
}

pub const SRAM_SIZE: u16 = 0x0a00;
pub const SRAM_ADDR: u16 = 0x0100;
pub const IOSPACE_SIZE: u16 = 0x0040;
pub const IOSPACE_ADDR: u16 = 0x0020;
pub const DATA_SIZE: u16 = 0x0b00;
pub const PROGRAM_SIZE: u16 = 0x8000;

pub struct Core {
    /// Status Register
    status_reg: StatusRegister,
    /// General Purpose Register File
    pub regs: GeneralRegisters,
    /// IO Registers
    io_space: [u8; IOSPACE_SIZE as usize],
    /// Program Counter
    pub pc: u16,
    /// Stack Pointer
    pub sp: u16,
    /// SRAM
    sram: [u8; SRAM_SIZE as usize],
    /// Program Memory
    pub program: [u8; PROGRAM_SIZE as usize],
    pub program_ops: [Op; PROGRAM_SIZE as usize],
    /// Set if the previos instruction branched.  This flag is used to know if the instruction
    /// ahead must be fetched.
    branch: bool,
    /// Next op
    // op1: Op,
    // Peripherials
    clock: clock::Clock,
    pub display: display::Display,
    /// Sleeping?
    pub sleep: bool,
    pub gpio: GPIO,
}

pub struct GPIO {
    pin_b: u8,
    pin_c: u8,
    pin_d: u8,
    pin_e: u8,
    pin_f: u8,
}

impl GPIO {
    fn new() -> Self {
        GPIO {
            pin_b: 0,
            pin_c: 0,
            pin_d: 0,
            pin_e: 0,
            pin_f: 0,
        }
    }

    pub fn set_port(&mut self, port: GPIOPort, v: u8) {
        match port {
            GPIOPort::B => self.pin_b = v,
            GPIOPort::C => self.pin_c = v,
            GPIOPort::D => self.pin_d = v,
            GPIOPort::E => self.pin_e = v,
            GPIOPort::F => self.pin_f = v,
        }
    }
}

pub enum GPIOPort {
    B,
    C,
    D,
    E,
    F,
}

impl Core {
    pub fn new() -> Self {
        Self {
            regs: GeneralRegisters::new(),
            io_space: [0; IOSPACE_SIZE as usize],
            status_reg: StatusRegister::new(),
            pc: 0,
            sram: [0; SRAM_SIZE as usize],
            program: [0; PROGRAM_SIZE as usize],
            program_ops: [Op::Nop; PROGRAM_SIZE as usize],
            sp: SRAM_ADDR + SRAM_SIZE - 1,
            branch: false,
            // op1: Op::Undefined { w: 0x0000 },
            // Peripherials
            clock: clock::Clock::new(),
            sleep: false,
            display: display::Display::new(),
            gpio: GPIO::new(),
        }
    }

    /// Load a word from Program Memory by PC
    fn program_load_u16(&self, pc: u16) -> u16 {
        u16::from_le_bytes([
            self.program[pc as usize * 2],
            self.program[pc as usize * 2 + 1],
        ])
    }

    /// Reset the core into an initial known state
    pub fn reset(&mut self) {
        self.pc = 0;
        self.sp = SRAM_ADDR + SRAM_SIZE - 1;
        self.branch = false;
        // let w0 = self.program_load_u16(self.pc);
        // let w1 = self.program_load_u16(self.pc + 1);
        // self.op1 = Op::decode(w0, w1);
    }

    /// Instruction used to populate the Program Memory Space.  Simulates flashing the flash
    /// memory.
    pub fn flash(&mut self, addr: u16, v: u8) {
        self.program[addr as usize] = v;
        let addr_align = (addr & 0b1111_1111_1111_1110) as usize;
        self.program_ops[(addr >> 1) as usize] = Op::decode(
            (self.program[addr_align + 0] as u16) | (self.program[addr_align + 1] as u16) << 8,
            (self.program[addr_align + 2] as u16) | (self.program[addr_align + 3] as u16) << 8,
        );
        // If a byte of the second word of a two word op is being flashed, the op needs to be
        // updated.
        if addr_align < 2 {
            return;
        }
        match self.program_ops[(addr >> 1) as usize - 1] {
            Op::Call { .. } | Op::Jmp { .. } | Op::Lds { .. } | Op::Sts { .. } => {}
            _ => {
                return;
            }
        }
        self.program_ops[(addr >> 1) as usize - 1] = Op::decode(
            (self.program[addr_align - 2] as u16) | (self.program[addr_align - 1] as u16) << 8,
            (self.program[addr_align + 0] as u16) | (self.program[addr_align + 1] as u16) << 8,
        );
    }

    pub fn op1(&self) -> Op {
        self.program_ops[self.pc as usize + 1]
    }

    pub fn next_op(&self) -> (u16, u16, OpAddr) {
        (
            self.program_load_u16(self.pc),
            self.program_load_u16(self.pc + 1),
            OpAddr {
                // op: self.op1.clone(),
                op: self.program_ops[self.pc as usize],
                addr: self.pc << 1,
            },
        )
    }

    /// Step one instruction.  Return the number of cycles that have passed.
    pub fn step(&mut self) -> usize {
        if self.sleep {
            return 1;
        }
        // Load current op from previously fetched next op
        //let op0 = self.op1.clone();
        //// Fetch next op
        //let pc1 = self.pc + op0.words() as u16;
        //let w0 = self.program_load_u16(pc1);
        //let w1 = self.program_load_u16(pc1 + 1);
        //// Decode next op
        //self.op1 = Op::decode(w0, w1);
        //let cycles = self.exec_op(op0);
        //if self.branch {
        //    let w0 = self.program_load_u16(self.pc);
        //    let w1 = self.program_load_u16(self.pc + 1);
        //    self.op1 = Op::decode(w0, w1);
        //    self.branch = false;
        //}
        //cycles

        self.exec_op(self.program_ops[self.pc as usize])
    }

    // returns true if an interrupt is fired
    pub fn step_hw(&mut self, cycles: usize) -> bool {
        // The interrupts have priority in accordance with their Interrupt Vector position.  The
        // lower the Interrupt Vector address, the higher the priority.

        // The Global Interrupt Enable Register is cleared by hardware afeter an interrupt has
        // ocurred.

        let mut interrupt_bitmap: u64 = 0;

        // First, step all the enabled peripherials
        self.clock.step(cycles);
        interrupt_bitmap |= self.clock.int();

        // Then, if interrupts are enabled, handle the pending ones

        // Global Interrupt Flag disabled
        if !self.status_reg.i {
            return false;
        }

        // No pending interrupt
        if interrupt_bitmap == 0 {
            return false;
        }

        self.sleep = false;
        // println!("INTERRUPT 1");
        self.status_reg.i = false;

        let pc = if interrupt_bitmap & Interrupt::Reset.to_u64().unwrap() != 0 {
            debug!("Handling interrupt RESET");
            RESET
        } else if interrupt_bitmap & Interrupt::Int0.to_u64().unwrap() != 0 {
            debug!("Handling interrupt INT0");
            INT0
        } else if interrupt_bitmap & Interrupt::Int1.to_u64().unwrap() != 0 {
            debug!("Handling interrupt INT1");
            INT1
        } else if interrupt_bitmap & Interrupt::Int2.to_u64().unwrap() != 0 {
            debug!("Handling interrupt INT2");
            INT2
        } else if interrupt_bitmap & Interrupt::Int3.to_u64().unwrap() != 0 {
            debug!("Handling interrupt INT3");
            INT3
        } else if interrupt_bitmap & Interrupt::Int6.to_u64().unwrap() != 0 {
            debug!("Handling interrupt INT6");
            INT6
        } else if interrupt_bitmap & Interrupt::Pcint0.to_u64().unwrap() != 0 {
            debug!("Handling interrupt PCINT0");
            PCINT0
        } else if interrupt_bitmap & Interrupt::UsbGeneral.to_u64().unwrap() != 0 {
            debug!("Handling interrupt USB_GENERAL");
            USB_GENERAL
        } else if interrupt_bitmap & Interrupt::UsbEndpoint.to_u64().unwrap() != 0 {
            debug!("Handling interrupt USB_ENDPOINT");
            USB_ENDPOINT
        } else if interrupt_bitmap & Interrupt::Wdt.to_u64().unwrap() != 0 {
            debug!("Handling interrupt WDT");
            WDT
        } else if interrupt_bitmap & Interrupt::Timer1Capt.to_u64().unwrap() != 0 {
            debug!("Handling interrupt TIMER1_CAPT");
            TIMER1_CAPT
        } else if interrupt_bitmap & Interrupt::Timer1CompA.to_u64().unwrap() != 0 {
            debug!("Handling interrupt TIMER1_COMPA");
            TIMER1_COMPA
        } else if interrupt_bitmap & Interrupt::Timer1CompB.to_u64().unwrap() != 0 {
            debug!("Handling interrupt TIMER1_COMPB");
            TIMER1_COMPB
        } else if interrupt_bitmap & Interrupt::Timer1CompC.to_u64().unwrap() != 0 {
            debug!("Handling interrupt TIMER1_COMPC");
            TIMER1_COMPC
        } else if interrupt_bitmap & Interrupt::Timer1Ovf.to_u64().unwrap() != 0 {
            debug!("Handling interrupt TIMER1_OVF");
            TIMER1_OVF
        } else if interrupt_bitmap & Interrupt::Timer0CompA.to_u64().unwrap() != 0 {
            debug!("Handling interrupt TIMER0_COMPA");
            TIMER0_COMPA
        } else if interrupt_bitmap & Interrupt::Timer0CompB.to_u64().unwrap() != 0 {
            debug!("Handling interrupt TIMER0_COMPB");
            TIMER0_COMPB
        } else if interrupt_bitmap & Interrupt::Timer0Ovf.to_u64().unwrap() != 0 {
            debug!("Handling interrupt TIMER0_OVF");
            self.clock.clear_int(Interrupt::Timer0Ovf);
            TIMER0_OVF
        } else if interrupt_bitmap & Interrupt::SpiStc.to_u64().unwrap() != 0 {
            debug!("Handling interrupt SPI_STC");
            SPI_STC
        } else if interrupt_bitmap & Interrupt::Usart1Rx.to_u64().unwrap() != 0 {
            debug!("Handling interrupt USART1_RX");
            USART1_RX
        } else if interrupt_bitmap & Interrupt::Usart2Udre.to_u64().unwrap() != 0 {
            debug!("Handling interrupt USART2_UDRE");
            USART2_UDRE
        } else if interrupt_bitmap & Interrupt::Usart1Tx.to_u64().unwrap() != 0 {
            debug!("Handling interrupt USART1_TX");
            USART1_TX
        } else if interrupt_bitmap & Interrupt::AnalogComp.to_u64().unwrap() != 0 {
            debug!("Handling interrupt ANALOG_COMP");
            ANALOG_COMP
        } else if interrupt_bitmap & Interrupt::Adc.to_u64().unwrap() != 0 {
            debug!("Handling interrupt ADC");
            ADC
        } else if interrupt_bitmap & Interrupt::EeReady.to_u64().unwrap() != 0 {
            debug!("Handling interrupt EE_READY");
            EE_READY
        } else if interrupt_bitmap & Interrupt::Timer3Capt.to_u64().unwrap() != 0 {
            debug!("Handling interrupt TIMER3_CAPT");
            TIMER3_CAPT
        } else if interrupt_bitmap & Interrupt::Timer3CompA.to_u64().unwrap() != 0 {
            debug!("Handling interrupt TIMER3_COMPA");
            TIMER3_COMPA
        } else if interrupt_bitmap & Interrupt::Timer3CompB.to_u64().unwrap() != 0 {
            debug!("Handling interrupt TIMER3_COMPB");
            TIMER3_COMPB
        } else if interrupt_bitmap & Interrupt::Timer3CompC.to_u64().unwrap() != 0 {
            debug!("Handling interrupt TIMER3_COMPC");
            TIMER3_COMPC
        } else if interrupt_bitmap & Interrupt::Timer3Ovf.to_u64().unwrap() != 0 {
            debug!("Handling interrupt TIMER3_OVF");
            self.clock.clear_int(Interrupt::Timer3Ovf);
            TIMER3_OVF
        } else if interrupt_bitmap & Interrupt::Twi.to_u64().unwrap() != 0 {
            debug!("Handling interrupt TWI");
            TWI
        } else if interrupt_bitmap & Interrupt::StmReady.to_u64().unwrap() != 0 {
            debug!("Handling interrupt STM_READY");
            STM_READY
        } else if interrupt_bitmap & Interrupt::Timer4CompA.to_u64().unwrap() != 0 {
            debug!("Handling interrupt TIMER4_COMPA");
            TIMER4_COMPA
        } else if interrupt_bitmap & Interrupt::Timer4CompB.to_u64().unwrap() != 0 {
            debug!("Handling interrupt TIMER4_COMPB");
            TIMER4_COMPB
        } else if interrupt_bitmap & Interrupt::Timer4CompD.to_u64().unwrap() != 0 {
            debug!("Handling interrupt TIMER4_COMPD");
            TIMER4_COMPD
        } else if interrupt_bitmap & Interrupt::Timer4Ovf.to_u64().unwrap() != 0 {
            debug!("Handling interrupt TIMER4_OVF");
            TIMER4_OVF
        } else if interrupt_bitmap & Interrupt::Timer4Fpf.to_u64().unwrap() != 0 {
            debug!("Handling interrupt TIMER4_FPF");
            TIMER4_FPF
        } else {
            unreachable!();
        };
        // debug!("INT PUSH PC {:04x}", self.pc << 1);
        // TODO: Before jumping to the interrupt vector, clean the interrupt flag.
        self.push_u16(self.pc);
        self.pc = pc;
        // Fetch op1 because we are branching without runnig step()
        // let w0 = self.program_load_u16(self.pc);
        // let w1 = self.program_load_u16(self.pc + 1);
        // self.op1 = Op::decode(w0, w1);
        true
    }

    /// Load a byte from the User Data Space
    pub fn data_load(&self, addr: u16) -> u8 {
        if addr >= SRAM_ADDR {
            self.sram[(addr - SRAM_ADDR) as usize]
        } else if addr < IOSPACE_ADDR {
            self.regs[addr as u8]
        } else {
            match addr {
                io_regs::SPL
                | io_regs::SPH
                | io_regs::SREG
                | io_regs::SPSR
                | io_regs::PINB
                | io_regs::PINE
                | io_regs::PINF => {}
                _ => debug!(
                    "I/O Registers / Extended I/O Space load at 0x{:04x} {:?}",
                    addr,
                    io_reg_str(addr).unwrap_or(("Unknown", ""))
                ),
            }
            match addr {
                io_regs::SPL => get_lo(self.sp),
                io_regs::SPH => get_hi(self.sp),
                io_regs::SREG => self.status_reg.as_u8(),
                io_regs::TCCR0A => self.clock.reg_tccr0a(), // TODO: Timer/Counter Control Register A
                io_regs::TCCR0B => self.clock.reg_tccr0b(), // TODO: Timer/Counter Control Register B
                io_regs::TCCR1A => 0, // TODO: Timer/Counter Control Register A
                io_regs::TCCR1B => 0, // TODO: Timer/Counter Control Register B
                io_regs::TCCR3A => self.clock.reg_tccr3a(), // TODO: Timer/Counter Control Register A
                io_regs::TCCR3B => self.clock.reg_tccr3b(), // TODO: Timer/Counter Control Register B
                io_regs::TCCR4A => 0, // TODO: Timer/Counter Control Register A
                io_regs::TCCR4B => 0, // TODO: Timer/Counter Control Register B
                io_regs::TCCR4C => 0, // TODO: Timer/Counter Control Register C
                io_regs::TCCR4D => 0, // TODO: Timer/Counter Control Register D
                io_regs::TIMSK0 => self.clock.reg_timsk0(), // TODO: Timer/Counter Interrupt Mask Register
                io_regs::TIMSK4 => 0, // TODO: Timer/Counter Interrupt Mask Register
                io_regs::TIMSK3 => self.clock.reg_timsk3(), // TODO: Timer/Counter Interrupt Mask Register
                io_regs::TCNT0 => self.clock.reg_tcnt0(),   // TODO: Timer/Counter Register
                io_regs::TIFR1 => 0, // TODO: Timer/Counter Interrupt Flag Resiger
                io_regs::ADCSRA => 0, // TODO: ADC Ctrl & Status Register
                io_regs::UHWCON => 0, // TODO
                io_regs::USBCON => 0, // TODO
                io_regs::UDCON => 0, // TODO
                io_regs::UDINT => 0, // TODO
                io_regs::UDIEN => 0, // TODO
                io_regs::DDRD => 0,  // TODO
                io_regs::DDRB => 0,  // TODO
                io_regs::DDRE => 0,  // TODO
                io_regs::DDRF => 0,  // TODO
                io_regs::ADMUX => 0, // TODO
                io_regs::PORTB => 0, // TODO
                io_regs::PORTC => 0, // TODO
                io_regs::PORTD => 0, // TODO
                io_regs::PORTE => 0, // TODO
                io_regs::PORTF => 0, // TODO
                io_regs::DDRC => 0,  // TODO
                io_regs::SPCR => 0,  // TODO
                io_regs::SPSR => 0b10000000, // TODO
                io_regs::SPDR => 0,  // TODO
                io_regs::PRR0 => 0,  // TODO
                io_regs::PRR1 => 0,  // TODO
                io_regs::SMCR => 0,  // TODO
                io_regs::PINB => self.gpio.pin_b, // TODO
                io_regs::PINC => self.gpio.pin_c, // TODO
                io_regs::PIND => self.gpio.pin_d, // TODO
                io_regs::PINE => self.gpio.pin_e, // TODO
                io_regs::PINF => self.gpio.pin_f, // TODO
                io_regs::PLLCSR => self.clock.reg_pllcsr(), // TODO: PLL Control and Status Register
                io_regs::EEDR => 0,  // TODO
                io_regs::EECR => 0,  // TODO
                io_regs::ADCL => 0,  // TODO
                io_regs::ADCH => 0,  // TODO
                io_regs::ADCSRB => 0, // TODO
                // 0x20 => 0,            // TODO
                // 0x21 => 0,            // TODO
                // 0x22 => 0,            // TODO
                // 0x32 => 0,            // TODO
                // 0x33 => 0,            // TODO
                // 0x34 => 0,            // TODO
                _ => {
                    warn!(
                        "[{:04x}] I/O Registers / Extended I/O Space unimplemented load at 0x{:04x} {:?}",
                        self.pc,
                        addr,
                        io_reg_str(addr).unwrap_or(("Unknown", ""))
                    );
                    unimplemented!()
                }
            }
        }
    }

    /// Load a word from the User Data Space
    fn data_load_u16(&self, addr: u16) -> u16 {
        u16::from_le_bytes([self.data_load(addr), self.data_load(addr + 1)])
    }

    /// Store a byte into the User Data Space
    fn data_store(&mut self, addr: u16, v: u8) {
        if addr >= SRAM_ADDR {
            self.sram[(addr - SRAM_ADDR) as usize] = v;
        } else if addr < IOSPACE_ADDR {
            self.regs[addr as u8] = v;
        } else {
            match addr {
                io_regs::SPL | io_regs::SPH | io_regs::SREG | io_regs::SPDR | io_regs::PORTC => {}
                _ => debug!(
                    "I/O Registers / Extended I/O Space store (0x{:02x}) at 0x{:04x} {:?}",
                    v,
                    addr,
                    io_reg_str(addr).unwrap_or(("Unknown", ""))
                ),
            }
            match addr {
                io_regs::SPL => set_lo(&mut self.sp, v),
                io_regs::SPH => set_hi(&mut self.sp, v),
                io_regs::SREG => self.status_reg = StatusRegister::from_u8(v),
                io_regs::TCCR0A => self.clock.set_reg_tccr0a(v),
                io_regs::TCCR0B => self.clock.set_reg_tccr0b(v),
                io_regs::TCCR1A => {} // TODO: Timer/Counter Control Register A
                io_regs::TCCR1B => {} // TODO: Timer/Counter Control Register B
                io_regs::TCCR3A => self.clock.set_reg_tccr3a(v),
                io_regs::TCCR3B => self.clock.set_reg_tccr3b(v),
                io_regs::TCCR4A => {} // TODO: Timer/Counter Control Register A
                io_regs::TCCR4B => {} // TODO: Timer/Counter Control Register B
                io_regs::TCCR4C => {} // TODO: Timer/Counter Control Register B
                io_regs::TCCR4D => {} // TODO: Timer/Counter Control Register B
                io_regs::TIMSK0 => self.clock.set_reg_timsk0(v),
                io_regs::TIMSK1 => {} // TODO: Timer/Counter Interrupt Mask Register
                io_regs::TIMSK3 => self.clock.set_reg_timsk3(v),
                io_regs::TIMSK4 => {} // TODO: Timer/Counter Interrupt Mask Register
                io_regs::TCNT0 => self.clock.set_reg_tcnt0(v),
                io_regs::TIFR1 => {} // TODO: Timer/Counter Interrupt Flag Resiger
                io_regs::ADCSRA => {} // TODO: ADC Ctrl & Status Register
                io_regs::UHWCON => {} // TODO
                io_regs::USBCON => {} // TODO
                io_regs::UDCON => {} // TODO
                io_regs::UDINT => {} // TODO
                io_regs::UDIEN => {} // TODO
                io_regs::DDRD => {}  // TODO
                io_regs::DDRB => {}  // TODO
                io_regs::DDRE => {}  // TODO
                io_regs::DDRF => {}  // TODO
                io_regs::ADMUX => {} // TODO
                io_regs::PORTB => {} // TODO
                io_regs::PORTC => {} // TODO
                io_regs::PORTD => self.display.set_dc((v & (1 << 4)) != 0),
                io_regs::PORTE => {} // TODO
                io_regs::PORTF => {} // TODO
                io_regs::DDRC => {}  // TODO
                io_regs::SPCR => {}  // TODO: SPI Control Register
                io_regs::SPSR => {}  // TODO: SPI Status Register
                io_regs::SPDR => self.display.spi_write(v),
                io_regs::PRR0 => {} // TODO: Power Reduction Register 0
                io_regs::PRR1 => {} // TODO: Power Reduction Register 1
                io_regs::SMCR => {} // TODO: Sleep Mode Control Register
                io_regs::PINB => {} // TODO
                io_regs::PINC => {} // TODO
                io_regs::PIND => {} // TODO
                io_regs::PINE => {} // TODO
                io_regs::PINF => {} // TODO
                io_regs::PLLCSR => self.clock.set_reg_pllcsr(v), // TODO: PLL Control and Status Register
                io_regs::EEARL => {}                             // TODO
                io_regs::EEARH => {}                             // TODO
                io_regs::EECR => {}                              // TODO
                io_regs::TCCR3C => {}                            // TODO
                io_regs::ICR3H => self.clock.set_reg_icr3h(v),   // TODO
                io_regs::ICR3L => self.clock.set_reg_icr3l(v),   // TODO
                io_regs::EEDR => {}                              // TODO
                io_regs::OCR3AH => self.clock.set_reg_ocr3ah(v),
                io_regs::OCR3AL => self.clock.set_reg_ocr3al(v),
                io_regs::TCNT3H => self.clock.set_reg_tcnt3h(v),
                io_regs::TCNT3L => self.clock.set_reg_tcnt3l(v),
                io_regs::OCR1AH => {} // TODO
                io_regs::OCR1AL => {} // TODO
                io_regs::OCR0A => {}  // TODO
                io_regs::OCR1BL => {} // TODO
                io_regs::OCR1BH => {} // TODO
                io_regs::ADCSRB => {} // TODO
                io_regs::GPIOR0 => {
                    warn!("DBG: {:02x}", v);
                }
                _ => {
                    warn!(
                        "I/O Registers / Extended I/O Space unimplemented store (0x{:02x}) at 0x{:04x} {:?}",
                        v,
                        addr,
                        io_reg_str(addr).unwrap_or(("Unknown", ""))
                    );
                    unimplemented!()
                }
            }
        }
    }

    /// Store a word into the User Data Space
    fn data_store_u16(&mut self, addr: u16, v: u16) {
        let bytes = v.to_le_bytes();
        self.data_store(addr, bytes[0]);
        self.data_store(addr + 1, bytes[1]);
    }

    /// Push a word into the stack
    fn push_u16(&mut self, v: u16) {
        let bytes = v.to_le_bytes();
        self.sram[(self.sp - SRAM_ADDR - 1) as usize] = bytes[0];
        self.sram[(self.sp - SRAM_ADDR) as usize] = bytes[1];
        self.sp -= 2;
    }

    /// Pop a word from the stack
    fn pop_u16(&mut self) -> u16 {
        self.sp += 2;
        u16::from_le_bytes([
            self.sram[(self.sp - SRAM_ADDR - 1) as usize],
            self.sram[(self.sp - SRAM_ADDR) as usize],
        ])
    }

    // Auxiliary function to update some flags in adc and add
    fn aux_op_add_flags(&mut self, a: u8, b: u8, c: bool, res: u8) {
        let (r7, rr7, rd7) = (res & 1 << 7, b & 1 << 7, a & 1 << 7);
        self.status_reg.n = r7 != 0;
        self.status_reg.v = (rr7 == rd7) && (rr7 != r7);
        self.status_reg.s = self.status_reg.n ^ self.status_reg.v;
        self.status_reg.h = ((b & 0x0f) + (a & 0x0f) + c as u8) > 0x0f;
        self.status_reg.z = res == 0;
    }
    /// 5. Add with Carry (ADC Rd, Rr) OK
    fn op_adc(&mut self, d: u8, r: u8) -> usize {
        let (res, c0) = self.regs[d].overflowing_add(self.regs[r]);
        let (res, c1) = res.overflowing_add(self.status_reg.c as u8);
        self.aux_op_add_flags(self.regs[d], self.regs[r], self.status_reg.c, res);
        self.status_reg.c = c0 || c1;
        self.regs[d] = res;

        self.pc += 1;
        1
    }
    /// 6. Add without Carry (ADD Rd, Rr) OK
    fn op_add(&mut self, d: u8, r: u8) -> usize {
        let (res, c0) = self.regs[d].overflowing_add(self.regs[r]);
        self.status_reg.c = c0;
        self.aux_op_add_flags(self.regs[d], self.regs[r], false, res);
        self.regs[d] = res;

        self.pc += 1;
        1
    }

    /// 7. Add Immediate Word (ADIW Rdl, K) OK
    fn op_adiw(&mut self, d: u8, k: u8) -> usize {
        let ext = self.regs.ext(d);
        let (res, c) = ext.overflowing_add(k as u16);
        let (r15, rdh7) = (res & 1 << 15, ext & 1 << 15);
        self.status_reg.n = r15 != 0;
        self.status_reg.v = !(rdh7 != 0) & (r15 != 0);
        self.status_reg.s = self.status_reg.n ^ self.status_reg.v;
        self.status_reg.c = c;
        self.status_reg.z = res == 0;
        self.regs.set_ext(d, res);

        self.pc += 1;
        2
    }

    /// 8. Logical AND (AND Rd, Rr) OK
    fn op_and(&mut self, d: u8, r: u8) -> usize {
        let res = self.regs[d] & self.regs[r];
        let r7 = res & 1 << 7;
        self.status_reg.n = r7 != 0;
        self.status_reg.v = false;
        self.status_reg.s = self.status_reg.n ^ self.status_reg.v;
        self.status_reg.z = res == 0;
        self.regs[d] = res;

        self.pc += 1;
        1
    }

    /// 9. Logical AND with Immediate (ANDI Rd, K) OK
    fn op_andi(&mut self, d: u8, k: u8) -> usize {
        let res = self.regs[d] & k;
        let r7 = res & 1 << 7;
        self.status_reg.n = r7 != 0;
        self.status_reg.v = false;
        self.status_reg.s = self.status_reg.n ^ self.status_reg.v;
        self.status_reg.z = res == 0;
        self.regs[d] = res;

        self.pc += 1;
        1
    }

    /// 10. Arithmetic Shift Right (ASR Rd) OK
    fn op_asr(&mut self, d: u8) -> usize {
        let res = self.regs[d] >> 1 | self.regs[d] & (1 << 7);
        let r7 = res & 1 << 7;
        self.status_reg.n = r7 != 0;
        self.status_reg.c = self.regs[d] & 1 != 0;
        self.status_reg.v = self.status_reg.n ^ self.status_reg.c;
        self.status_reg.s = self.status_reg.n ^ self.status_reg.v;
        self.status_reg.z = res == 0;
        self.regs[d] = res;

        self.pc += 1;
        1
    }

    /// 11. Bit Clear in SREG (BCLR s) OK
    fn op_bclr(&mut self, s: u8) -> usize {
        self.status_reg[s] = false;

        self.pc += 1;
        1
    }

    /// 12. Bit Load from the T Flag in SREG to a Bit in Register (BLD Rd, b) OK
    fn op_bld(&mut self, d: u8, b: u8) -> usize {
        self.regs[d] &= !(1 << b);
        self.regs[d] |= (self.status_reg.t as u8) << b;

        self.pc += 1;
        1
    }

    // Auxiliary function to branch if a boolean is true
    fn aux_op_branch_if(&mut self, k: i8, test: bool) -> usize {
        if test {
            let (pc, _) = (self.pc as i16).overflowing_add(k as i16 + 1);
            self.pc = pc as u16;
            self.branch = true;
            2
        } else {
            self.pc += 1;
            1
        }
    }
    /// 13. Branch if Bit in SREG is Cleared (BRBC s, k) OK
    fn op_brbc(&mut self, s: u8, k: i8) -> usize {
        self.aux_op_branch_if(k, !self.status_reg[s])
    }
    /// 14. Branch if Bit in SREG is Set (BRBS s, k) OK
    fn op_brbs(&mut self, s: u8, k: i8) -> usize {
        self.aux_op_branch_if(k, self.status_reg[s])
    }
    // 15. Branch if Carry Cleared (BRCC k) OK -> BRBC C
    // 16. Branch if Carry Set (BRCS k) OK -> BRBS C

    /// 17. Break (BREAK) OK
    fn op_break(&mut self) -> usize {
        warn!("BREAK unimplemented.");
        unimplemented!();
        // 1
    }

    // 18. Branch if Equal (BREQ k) OK -> BRBS Z
    // 19. Branch if Greater or Equal (Signed) (BRGE k) OK -> BRBC S
    // 20. Branch if Half Carry Flag is Cleared (BRHC k) OK -> BRBC H
    // 21. Branch if Half Carry Flag is Set (BRHS k) OK -> BRBS H
    // 22. Branch if Global Interrupt is Disabled (BRID k) OK -> BRBC I
    // 23. Branch if Global Interrupt is Enabled (BRIE k) OK -> BRBS I
    // 24. Branch if Lower (Unsigned) (BRLO k) OK -> BRBS C
    // 25. Branch if Less Than (Signed) (BRLT k) OK -> BRBS S
    // 26. Branch if Minus (BRMI k) OK -> BRBS N
    // 27. Branch if Not Equal (BRNE k) OK -> BRBC Z
    // 28. Branch if Plus (BRPL k) OK -> BRBC N
    // 29. Branch if Same or Higher (Unsigned) (BRSH k) OK -> BRBC C
    // 30. Branch if the T Flag is Cleared (BRTC k) OK -> BRBC T
    // 31. Branch if the T Flag is Set (BRTS k) OK -> BRBS T
    // 32. Branch if Overflow Cleared (BRVC k) OK -> BRBC V
    // 33. Branch if Overflow Set (BRVS k) OK -> BRBS V

    /// 34. Bit Set in SREG (BSET s) OK
    fn op_bset(&mut self, s: u8) -> usize {
        self.status_reg[s] = true;

        self.pc += 1;
        1
    }

    /// 35. Bit Store from Bit in Register to T Flag in SREG (BST Rr, b) OK
    fn op_bst(&mut self, d: u8, b: u8) -> usize {
        self.status_reg.t = (self.regs[d] & (1 << b)) != 0;

        self.pc += 1;
        1
    }

    /// 36. Long Call to a Subroutine (CALL k) OK
    fn op_call(&mut self, k: u32) -> usize {
        self.push_u16(self.pc + 2);
        self.pc = k as u16;
        self.branch = true;
        4
    }

    /// 37. Clear Bit in I/O Register (CBI A, b) OK
    fn op_cbi(&mut self, a: u8, b: u8) -> usize {
        let v = self.data_load(IOSPACE_ADDR + a as u16);
        self.data_store(IOSPACE_ADDR + a as u16, v & !(1 << b));
        self.pc += 1;
        2
    }

    // 38. Clear Bits in Register (CBR Rd, K) OK -> ANDI with K complemented
    // 39. Clear Carry Flag (CLC) OK -> BCLR C
    // 40. Clear Half Carry Flag (CLH) OK -> BCLR H
    // 41. Clear Global Interrupt Flag (CLI) OK -> BCLR I
    // 42. Clear Negative Flag (CLN) OK -> BCLR N

    // 43. Clear Register (CLR Rd) OK -> EOR Rd, Rd
    // fn op_clr(&mut self, d: u8) -> usize {
    //     self.regs[d] = 0;
    //     self.status_reg.s = false;
    //     self.status_reg.v = false;
    //     self.status_reg.n = false;
    //     self.status_reg.z = true;

    //     self.pc += 1;
    //     1
    // }

    // 44. Clear Signed Flag (CLS) OK -> BCLR S
    // 45. Clear T Flag (CLT) OK -> BCLR T
    // 46. Clear Overflow Flag (CLV) OK -> BCLR V
    // 47. Clear Zero Flag (CLZ) OK -> BCLR Z

    /// 48. One's Complement (COM Rd) OK
    fn op_com(&mut self, d: u8) -> usize {
        let (res, _) = 0xffu8.overflowing_sub(self.regs[d]);
        let r7 = res & 1 << 7;
        self.status_reg.n = r7 != 0;
        self.status_reg.v = false;
        self.status_reg.s = self.status_reg.n ^ self.status_reg.v;
        self.status_reg.z = res == 0;
        self.status_reg.c = true;
        self.regs[d] = res;

        self.pc += 1;
        1
    }

    // Auxiliary function to update some flags in cp and cpc
    fn aux_op_cp_flags(&mut self, a: u8, b: u8, c: bool, res: u8) {
        let (r7, rr7, rd7) = (res & 1 << 7, b & 1 << 7, a & 1 << 7);
        self.status_reg.n = r7 != 0;
        self.status_reg.v = (rr7 != rd7) && (rr7 == r7);
        self.status_reg.s = self.status_reg.n ^ self.status_reg.v;
        self.status_reg.h = ((b & 0x0f) + c as u8) > (a & 0x0f);
    }

    /// 49. Compare (CP Rd, Rr) OK
    fn op_cp(&mut self, d: u8, r: u8) -> usize {
        let (res, c0) = self.regs[d].overflowing_sub(self.regs[r]);

        self.aux_op_cp_flags(self.regs[d], self.regs[r], false, res);
        self.status_reg.z = res == 0;
        self.status_reg.c = c0;

        self.pc += 1;
        1
    }

    /// 50. Compare with Carry (CPC Rd, Rr) OK
    fn op_cpc(&mut self, d: u8, r: u8) -> usize {
        let (res, c0) = self.regs[d].overflowing_sub(self.regs[r]);
        let (res, c1) = res.overflowing_sub(self.status_reg.c as u8);
        self.aux_op_cp_flags(self.regs[d], self.regs[r], self.status_reg.c, res);
        if res != 0 {
            self.status_reg.z = false;
        }
        self.status_reg.c = c0 || c1;

        self.pc += 1;
        1
    }

    /// 51. Compare with Immediate (CPIRd, K) OK
    fn op_cpi(&mut self, d: u8, k: u8) -> usize {
        let (res, c0) = self.regs[d].overflowing_sub(k);

        self.aux_op_cp_flags(self.regs[d], k, false, res);
        self.status_reg.z = res == 0;
        self.status_reg.c = c0;

        self.pc += 1;
        1
    }

    /// 52. Compare Skip if Equal (CPSE Rd, Rr) OK
    fn op_cpse(&mut self, d: u8, r: u8, next_op_len: u8) -> usize {
        if self.regs[d] == self.regs[r] {
            self.pc += 1 + next_op_len as u16;
            self.branch = true;
            2
        } else {
            self.pc += 1;
            1
        }
    }

    /// 53. Decrement (DEC Rd) OK
    fn op_dec(&mut self, d: u8) -> usize {
        let (res, _) = self.regs[d].overflowing_sub(1);
        let r7 = res & 1 << 7;
        self.status_reg.n = r7 != 0;
        self.status_reg.v = res == 0b01111111;
        self.status_reg.s = self.status_reg.n ^ self.status_reg.v;
        self.status_reg.z = res == 0;
        self.regs[d] = res;

        self.pc += 1;
        1
    }

    // 54. Data Encryption Standard (DES) (NOT APPLICABLE)

    /// 55. Extended Indirect Call to Subroutine (EICALL) OK
    fn op_eicall(&mut self) -> usize {
        warn!("EICALL unimplemented.  Maybe not supported by ATmega32u4.");
        unimplemented!();
    }

    /// 56. Extended Indirect Jump (EIJMP) OK
    fn op_eijmp(&mut self) -> usize {
        warn!("EIJMP unimplemented.  Maybe not supported by ATmega32u4.");
        unimplemented!();
    }

    /// 57. Extended Load Program Memory (ELPM Z{+}) OK
    fn op_elpm(&mut self) -> usize {
        warn!("ELPM unimplemented.  Maybe not supported by ATmega32u4.");
        unimplemented!();
    }

    /// 58. Exclusive OR (EOR, Rd, Rr) OK
    fn op_eor(&mut self, d: u8, r: u8) -> usize {
        let res = self.regs[d] ^ self.regs[r];
        let r7 = res & 1 << 7;
        self.status_reg.n = r7 != 0;
        self.status_reg.v = false;
        self.status_reg.s = self.status_reg.n ^ self.status_reg.v;
        self.status_reg.z = res == 0;
        self.regs[d] = res;

        self.pc += 1;
        1
    }

    /// 59. Fractional Multiply Unsiged (FMUL Rd, Rr) OK
    fn op_fmul(&mut self, d: u8, r: u8) -> usize {
        let res = self.regs[d] as u16 * self.regs[r] as u16;
        let r16 = res & 1 << 15;
        let res = res << 1;
        self.status_reg.c = r16 != 0;
        self.status_reg.z = res == 0;
        let bytes = res.to_le_bytes();
        self.regs[0] = bytes[0];
        self.regs[1] = bytes[1];

        self.pc += 1;
        2
    }

    /// 60. Fractional Multiply Signed (FMULS Rd, Rr) OK
    fn op_fmuls(&mut self, d: u8, r: u8) -> usize {
        let res = self.regs[d] as i8 as i16 * self.regs[r] as i8 as i16;
        let r16 = res & 1 << 15;
        let res = res << 1;
        self.status_reg.c = r16 != 0;
        self.status_reg.z = res == 0;
        let bytes = res.to_le_bytes();
        self.regs[0] = bytes[0];
        self.regs[1] = bytes[1];

        self.pc += 1;
        2
    }

    /// 61. Fractional Multiply Signed with Unsigned (FMULSU Rd, Rr) OK
    fn op_fmulsu(&mut self, d: u8, r: u8) -> usize {
        let res = self.regs[d] as i8 as i16 * self.regs[r] as i16;
        let r16 = res & 1 << 15;
        let res = res << 1;
        self.status_reg.c = r16 != 0;
        self.status_reg.z = res == 0;
        let bytes = res.to_le_bytes();
        self.regs[0] = bytes[0];
        self.regs[1] = bytes[1];

        self.pc += 1;
        2
    }

    /// 62. Indirect Call to Subroutine (ICALL) OK
    fn op_icall(&mut self) -> usize {
        self.push_u16(self.pc + 1);
        self.pc = self.regs.z();
        self.branch = true;
        3
    }

    /// 63. Indirect Jump (IJMP) OK
    fn op_ijmp(&mut self) -> usize {
        self.pc = self.regs.z();
        self.branch = true;
        2
    }

    /// 64. Load an I/O Location to Register (IN Rd, a) OK
    fn op_in(&mut self, d: u8, a: u8) -> usize {
        self.regs[d] = self.data_load(IOSPACE_ADDR + a as u16);

        self.pc += 1;
        1
    }

    /// 65. Increment (INC Rd) OK
    fn op_inc(&mut self, d: u8) -> usize {
        let (res, _) = self.regs[d].overflowing_add(1);
        let r7 = res & 1 << 7;
        self.status_reg.n = r7 != 0;
        self.status_reg.v = res == 0b10000000;
        self.status_reg.s = self.status_reg.n ^ self.status_reg.v;
        self.status_reg.z = res == 0;
        self.regs[d] = res;

        self.pc += 1;
        1
    }

    /// 66. Jump (JMP k) OK
    fn op_jmp(&mut self, k: u32) -> usize {
        self.pc = k as u16;
        self.branch = true;
        3
    }

    // 67. Load and Clear (LAC) (NOT APPLICABLE)
    // 68. Load and Set (LAS) (NOT APPLICABLE)
    // 69. Load and Toggle (LAT) (NOT APPLICABLE)

    ///  70, 71, 72. Load Indirect from Data Space to Register using Index {X, Y, Z} (LD, {-}{X,Y,Z}{+}{q}) OK
    fn op_ld(&mut self, d: u8, idx: LdStIndex, ext: LdStExt) -> usize {
        self.pc += 1;
        let mut addr = self.regs.ext(idx.into());

        let cycles = match ext {
            LdStExt::None => {
                self.regs[d] = self.data_load(addr);
                return 2;
            }
            LdStExt::PostInc => {
                self.regs[d] = self.data_load(addr);
                addr += 1;
                2
            }
            LdStExt::PreDec => {
                addr -= 1;
                self.regs[d] = self.data_load(addr);
                2
            }
            LdStExt::Displacement(q) => {
                self.regs[d] = self.data_load(addr + q as u16);
                return 2;
            }
        };
        self.regs.set_ext(idx.into(), addr);

        cycles
    }

    /// 73. Load Immediate (LDI Rd, K) OK
    fn op_ldi(&mut self, d: u8, k: u8) -> usize {
        self.regs[d] = k;

        self.pc += 1;
        1
    }

    /// 74. Load Direct from Data Space (LDS Rd, k) OK
    fn op_lds(&mut self, d: u8, k: u16) -> usize {
        self.regs[d] = self.data_load(k);
        self.pc += 2;
        2
    }

    // 75. Load Direct from Data Space (LDS Rd, k ; 16bit) (NOT APPLICABLE)
    /// 76. Load Program Memory (LPM Rd, Z) OK
    fn op_lpm(&mut self, d: u8, inc: bool) -> usize {
        let z = self.regs.z();
        self.regs[d] = self.program[z as usize];
        if inc {
            self.regs.set_z(z + 1);
        }

        self.pc += 1;
        1
    }

    // 77. Logical Shift Left (LSL Rd) OK -> ADD Rd, Rd

    /// 78. Logical Shift Right (LSR Rd) OK
    fn op_lsr(&mut self, d: u8) -> usize {
        let res = self.regs[d] >> 1;
        self.status_reg.n = false;
        self.status_reg.c = self.regs[d] & 1 != 0;
        self.status_reg.v = self.status_reg.n ^ self.status_reg.c;
        self.status_reg.s = self.status_reg.n ^ self.status_reg.v;
        self.status_reg.z = res == 0;
        self.regs[d] = res;

        self.pc += 1;
        1
    }

    /// 79. Copy Register (MOV Rd, Rr) OK
    fn op_mov(&mut self, d: u8, r: u8) -> usize {
        self.regs[d] = self.regs[r];

        self.pc += 1;
        1
    }

    /// 80. Copy Register Word (MOVW Rd, Rr) OK
    fn op_movw(&mut self, d: u8, r: u8) -> usize {
        self.regs[d] = self.regs[r];
        self.regs[d + 1] = self.regs[r + 1];

        self.pc += 1;
        1
    }
    /// 81. Multiply Unsiged (MUL Rd, Rr) OK
    fn op_mul(&mut self, d: u8, r: u8) -> usize {
        let res = self.regs[d] as u16 * self.regs[r] as u16;
        let bytes = res.to_le_bytes();
        self.regs[0] = bytes[0];
        self.regs[1] = bytes[1];
        let r15 = res & 1 << 15;
        self.status_reg.c = r15 != 0;
        self.status_reg.z = res == 0;

        self.pc += 1;
        2
    }

    /// 82. Multiply Signed (MULS Rd, Rr) OK
    fn op_muls(&mut self, d: u8, r: u8) -> usize {
        let res = self.regs[d] as i8 as i16 * self.regs[r] as i8 as i16;
        let bytes = res.to_le_bytes();
        self.regs[0] = bytes[0];
        self.regs[1] = bytes[1];
        let r15 = res & 1 << 15;
        self.status_reg.c = r15 != 0;
        self.status_reg.z = res == 0;

        self.pc += 1;
        2
    }

    /// 83. Multiply Signed with Unsigned (MULSU Rd, Rr) OK
    fn op_mulsu(&mut self, d: u8, r: u8) -> usize {
        let res = self.regs[d] as i8 as i16 * self.regs[r] as i16;
        let bytes = res.to_le_bytes();
        self.regs[0] = bytes[0];
        self.regs[1] = bytes[1];
        let r15 = res & 1 << 15;
        self.status_reg.c = r15 != 0;
        self.status_reg.z = res == 0;

        self.pc += 1;
        2
    }

    // Auxiliary function to update some flags in subtraction
    // fn aux_op_sub_flags(&mut self, a: u8, b: u8, c: bool, res: u8) {
    //     let (r7, rr7, rd7) = (res & 1 << 7, b & 1 << 7, a & 1 << 7);
    //     self.status_reg.n = r7 != 0;
    //     self.status_reg.v = res == 0x80;
    //     // self.status_reg.v = (rr7 == rd7) && (rr7 != r7);
    //     self.status_reg.s = self.status_reg.n ^ self.status_reg.v;
    //     self.status_reg.h = (b & 0x0f) + c as u8 > (a & 0x0f);
    //     self.status_reg.z = res == 0;
    // }

    /// 84. Two's Complement (NEG Rd) OK
    fn op_neg(&mut self, d: u8) -> usize {
        let (res, _) = 0x00u8.overflowing_sub(self.regs[d]);
        self.status_reg.c = res != 0;
        let r7 = res & 1 << 7;
        self.status_reg.n = r7 != 0;
        self.status_reg.v = res == 0x80;
        self.status_reg.s = self.status_reg.n ^ self.status_reg.v;
        self.status_reg.h = (self.regs[d] & 0x0f) != 0;
        self.status_reg.z = res == 0;
        self.regs[d] = res;

        self.pc += 1;
        1
    }

    /// 85. No Operation (NOP) OK
    fn op_nop(&mut self) -> usize {
        self.pc += 1;
        1
    }

    /// 86. Logical OR (OR Rd, Rr) OK
    fn op_or(&mut self, d: u8, r: u8) -> usize {
        let res = self.regs[d] | self.regs[r];
        let r7 = res & 1 << 7;
        self.status_reg.n = r7 != 0;
        self.status_reg.v = false;
        self.status_reg.s = self.status_reg.n ^ self.status_reg.v;
        self.status_reg.z = res == 0;
        self.regs[d] = res;

        self.pc += 1;
        1
    }

    /// 87. Logical OR with Immediate (ORI Rd, K) OK
    fn op_ori(&mut self, d: u8, k: u8) -> usize {
        let res = self.regs[d] | k;
        let r7 = res & 1 << 7;
        self.status_reg.n = r7 != 0;
        self.status_reg.v = false;
        self.status_reg.s = self.status_reg.n ^ self.status_reg.v;
        self.status_reg.z = res == 0;
        self.regs[d] = res;

        self.pc += 1;
        1
    }

    /// 88. Store Register to I/O Location (OUT A, Rr) OK
    fn op_out(&mut self, a: u8, r: u8) -> usize {
        self.data_store(IOSPACE_ADDR + a as u16, self.regs[r]);

        self.pc += 1;
        1
    }

    /// 89. Pop Register from Stack (POP Rd) OK
    fn op_pop(&mut self, d: u8) -> usize {
        self.sp += 1;
        self.regs[d] = self.sram[(self.sp - SRAM_ADDR) as usize];

        self.pc += 1;
        2
    }

    /// 90. Push Register on Stack (PUSH Rr) OK
    fn op_push(&mut self, r: u8) -> usize {
        self.sram[(self.sp - SRAM_ADDR) as usize] = self.regs[r];
        self.sp -= 1;

        self.pc += 1;
        2
    }

    /// 91. Relative Call to Subroutione (RCALL k) OK
    fn op_rcall(&mut self, k: i16) -> usize {
        self.push_u16(self.pc + 1);
        let (pc, _) = (self.pc as i16).overflowing_add(k);
        let (pc, _) = (pc as i16).overflowing_add(1);
        self.pc = pc as u16;
        self.branch = true;
        3
    }

    /// 92. Return from Subroutine (RET) OK
    fn op_ret(&mut self) -> usize {
        let pc = self.pop_u16();
        self.pc = pc;
        self.branch = true;
        4
    }

    /// 93. Return from Interrupt (RETI) OK
    fn op_reti(&mut self) -> usize {
        let pc = self.pop_u16();
        self.status_reg.i = true;
        self.pc = pc;
        self.branch = true;
        4
    }

    /// 94. Relative Jump (RJMP k) OK
    fn op_rjmp(&mut self, k: i16) -> usize {
        let (pc, _) = (self.pc as i16).overflowing_add(k);
        let (pc, _) = (pc as i16).overflowing_add(1);
        self.pc = pc as u16;
        self.branch = true;
        2
    }

    // 95. Rotate Left through Carry (ROL Rd) OK -> ADC Rd, Rd
    // fn op_rol(&mut self, d: u8) -> usize {
    //     let res = self.regs[d] << 1;
    //     let res = res | self.status_reg.c as u8;
    //     let (r7, rd7, rd3) = (res & 1 << 7, self.regs[d] & 1 << 7, self.regs[d] & 1 << 3);
    //     self.status_reg.h = rd3 != 0;
    //     self.status_reg.n = r7 != 0;
    //     self.status_reg.c = rd7 != 0;
    //     self.status_reg.v = self.status_reg.n ^ self.status_reg.c;
    //     self.status_reg.s = self.status_reg.n ^ self.status_reg.v;
    //     self.status_reg.z = res == 0;
    //     self.regs[d] = res;

    //     self.pc += 1;
    //     1
    // }

    /// 96. Rotrate Right through Carry (ROR Rd) OK
    fn op_ror(&mut self, d: u8) -> usize {
        let res = self.regs[d] >> 1;
        let res = res | ((self.status_reg.c as u8) << 7);
        let (r7, rd0) = (res & 1 << 7, self.regs[d] & 1);
        self.status_reg.n = r7 != 0;
        self.status_reg.c = rd0 != 0;
        self.status_reg.v = self.status_reg.n ^ self.status_reg.c;
        self.status_reg.s = self.status_reg.n ^ self.status_reg.v;
        self.status_reg.z = res == 0;
        self.regs[d] = res;

        self.pc += 1;
        1
    }

    // Auxiliary function to update some flags in subtraction
    fn aux_op_sub_flags(&mut self, a: u8, b: u8, c: bool, res: u8) {
        let (r7, rr7, rd7) = (res & 1 << 7, b & 1 << 7, a & 1 << 7);
        self.status_reg.n = r7 != 0;
        self.status_reg.v = (rr7 != rd7) && (rr7 == r7);
        self.status_reg.s = self.status_reg.n ^ self.status_reg.v;
        self.status_reg.h = ((a & 0x0f) as i8) < ((b & 0x0f) as i8) + c as i8;
    }

    /// 97. Subtract with Carry (SBC Rd, Rr) OK
    fn op_sbc(&mut self, d: u8, r: u8) -> usize {
        let (res, c0) = self.regs[d].overflowing_sub(self.regs[r]);
        let (res, c1) = res.overflowing_sub(self.status_reg.c as u8);
        self.aux_op_sub_flags(self.regs[d], self.regs[r], self.status_reg.c, res);
        self.status_reg.c = c0 || c1;
        if res != 0 {
            self.status_reg.z = false;
        }
        self.regs[d] = res;

        self.pc += 1;
        1
    }

    /// 98. Subtract Immediate with Carry (SBCI Rd, K) OK
    fn op_sbci(&mut self, d: u8, k: u8) -> usize {
        let (res, c0) = self.regs[d].overflowing_sub(k);
        let (res, c1) = res.overflowing_sub(self.status_reg.c as u8);
        self.aux_op_sub_flags(self.regs[d], k, self.status_reg.c, res);
        self.status_reg.c = c0 || c1;
        if res != 0 {
            self.status_reg.z = false;
        }
        self.regs[d] = res;

        self.pc += 1;
        1
    }

    /// 99. Set Bit in I/O Register (SBI P, b) OK
    fn op_sbi(&mut self, a: u8, b: u8) -> usize {
        let v = self.data_load(IOSPACE_ADDR + a as u16);
        self.data_store(IOSPACE_ADDR + a as u16, v | (1 << b));
        self.pc += 1;
        2
    }

    /// 100. Skip if Bit in I/O Register is Cleared (SBIC P, b) OK
    fn op_sbic(&mut self, a: u8, b: u8, next_op_len: u8) -> usize {
        let v = self.data_load(IOSPACE_ADDR + a as u16);
        if (v & (1 << b)) == 0 {
            self.pc += 1 + next_op_len as u16;
            self.branch = true;
            2
        } else {
            self.pc += 1;
            1
        }
    }

    /// 101. Skip if Bit in I/O Register is Set (SBIS P, b) OK
    fn op_sbis(&mut self, a: u8, b: u8, next_op_len: u8) -> usize {
        let v = self.data_load(IOSPACE_ADDR + a as u16);
        if (v & (1 << b)) != 0 {
            self.pc += 1 + next_op_len as u16;
            self.branch = true;
            2
        } else {
            self.pc += 1;
            1
        }
    }

    /// 102. Subtract Immedaite from Word (SBIW Rdl, K) OK
    fn op_sbiw(&mut self, d: u8, k: u8) -> usize {
        let ext = self.regs.ext(d);
        let (res, c) = ext.overflowing_sub(k as u16);
        let (r15, rdh7) = (res & 1 << 15, ext & 1 << 15);
        self.status_reg.n = r15 != 0;
        self.status_reg.v = (rdh7 != 0) & !(r15 != 0);
        self.status_reg.s = self.status_reg.n ^ self.status_reg.v;
        self.status_reg.c = c;
        self.status_reg.z = res == 0;
        self.regs.set_ext(d, res);

        self.pc += 1;
        2
    }

    // 103. Set Bits in Register (SBR Rd, K) OK -> ORI Rd, K

    /// 104. Skip if Bit in Register is Cleared (SBRC Rr, b) OK
    fn op_sbrc(&mut self, r: u8, b: u8, next_op_len: u8) -> usize {
        if self.regs[r] & (1 << b) == 0 {
            self.pc += 1 + next_op_len as u16;
            self.branch = true;
            2
        } else {
            self.pc += 1;
            1
        }
    }

    /// 105. Skip if Bit in Register is Set (SBRS Rr, b) OK
    fn op_sbrs(&mut self, r: u8, b: u8, next_op_len: u8) -> usize {
        if self.regs[r] & (1 << b) != 0 {
            self.pc += 1 + next_op_len as u16;
            self.branch = true;
            2
        } else {
            self.pc += 1;
            1
        }
    }

    // 106. Set Carry Flag (SEC) OK -> BSET C
    // 107. Set Half Carry Flag (SEH) OK -> BSET H
    // 108. Set Global Interrupt Flag (SEI) OK -> BSET I
    // 109. Set Negative Flag (SEN) OK -> BSET N

    // 110. Set all Bits in Register (SER Rd) OK -> LDI Rd, 0xff
    // fn op_ser(&mut self, d: u8) -> usize {
    //     self.regs[d] = 0xff;
    //     self.pc += 1;
    //     1
    // }
    // 111. Set Signed Flag (SES) OK -> BSET S
    // 112. Set T Flag (SET) OK -> BSET T
    // 113. Set Overflow Flag (SEV) OK -> BSET V
    // 114. Set Zero Flag (SEZ) OK -> BSET Z

    /// 115. SLEEP (SLEEP) OK
    fn op_sleep(&mut self) -> usize {
        debug!("Entering sleep");
        self.sleep = true;
        self.pc += 1;
        1
    }
    /// 116. Store Program Memory (SPM) OK
    fn op_spm(&mut self) -> usize {
        warn!("SPM unimplemented.");
        unimplemented!();
    }
    /// 117. Store Program Memory (SPM #2)
    fn op_spm2(&mut self) -> usize {
        warn!("SPM #2 unimplemented.");
        unimplemented!();
    }
    /// 118, 119, 120. Store Indirect from Register to Data Space using Index {X, Y, Z} (ST {-}{X,Y,Z}{+}{q}, Rr) OK
    fn op_st(&mut self, r: u8, idx: LdStIndex, ext: LdStExt) -> usize {
        self.pc += 1;
        let mut addr = self.regs.ext(idx.into());

        let cycles = match ext {
            LdStExt::None => {
                self.data_store(addr, self.regs[r]);
                return 2;
            }
            LdStExt::PostInc => {
                self.data_store(addr, self.regs[r]);
                addr += 1;
                2
            }
            LdStExt::PreDec => {
                addr -= 1;
                self.data_store(addr, self.regs[r]);
                2
            }
            LdStExt::Displacement(q) => {
                self.data_store(addr + q as u16, self.regs[r]);
                return 2;
            }
        };
        self.regs.set_ext(idx.into(), addr);

        cycles
    }

    /// 121. Store Direct to Data Space (STS k, Rr) OK
    fn op_sts(&mut self, k: u16, r: u8) -> usize {
        self.data_store(k, self.regs[r]);
        self.pc += 2;
        2
    }

    // 122. Store Direct to Data Space (STS k, Rr ; 16bit) (NOT APPLICABLE)

    /// 123. Subtract without Carry (SUB Rd, Rr) OK
    fn op_sub(&mut self, d: u8, r: u8) -> usize {
        let (res, c0) = self.regs[d].overflowing_sub(self.regs[r]);
        self.status_reg.c = c0;
        self.aux_op_sub_flags(self.regs[d], self.regs[r], false, res);
        self.status_reg.z = res == 0;
        self.regs[d] = res;

        self.pc += 1;
        1
    }

    /// 124. Subtract Immediate (SUBI Rd, K) OK
    fn op_subi(&mut self, d: u8, k: u8) -> usize {
        let (res, c0) = self.regs[d].overflowing_sub(k);
        self.status_reg.c = c0;
        self.aux_op_sub_flags(self.regs[d], k, false, res);
        self.status_reg.z = res == 0;
        self.regs[d] = res;

        self.pc += 1;
        1
    }

    /// 125. Swap Nibbles (SWAP Rd) OK
    fn op_swap(&mut self, d: u8) -> usize {
        let (hi, lo) = ((self.regs[d] & 0xf0) >> 4, self.regs[d] & 0x0f);
        self.regs[d] = (lo << 4) | hi;

        self.pc += 1;
        1
    }

    // 126. Test for Zero or Minus (TST Rd) OK -> AND Rd, Rd
    /// 127. Watchdog Reset (WDR) OK
    fn op_wdr(&mut self) -> usize {
        warn!("WDR unimplemented.");
        unimplemented!();
        // self.pc += 1;
        // 1
    }
    // 128. Exchange (XCH) (NOT APPLICABLE)

    fn exec_op(&mut self, op: Op) -> usize {
        match op {
            Op::Adc { d, r } => self.op_adc(d, r),
            Op::Add { d, r } => self.op_add(d, r),
            Op::Adiw { d, k } => self.op_adiw(d, k),
            Op::And { d, r } => self.op_and(d, r),
            Op::Andi { d, k } => self.op_andi(d, k),
            Op::Asr { d } => self.op_asr(d),
            Op::Bclr { s } => self.op_bclr(s),
            Op::Bld { d, b } => self.op_bld(d, b),
            Op::Brbc { s, k } => self.op_brbc(s, k),
            Op::Brbs { s, k } => self.op_brbs(s, k),
            Op::Break => self.op_break(),
            Op::Bset { s } => self.op_bset(s),
            Op::Bst { d, b } => self.op_bst(d, b),
            Op::Call { k } => self.op_call(k),
            Op::Cbi { a, b } => self.op_cbi(a, b),
            Op::Com { d } => self.op_com(d),
            Op::Cp { d, r } => self.op_cp(d, r),
            Op::Cpc { d, r } => self.op_cpc(d, r),
            Op::Cpi { d, k } => self.op_cpi(d, k),
            Op::Cpse { d, r } => self.op_cpse(d, r, self.op1().words()),
            Op::Dec { d } => self.op_dec(d),
            Op::Eicall => self.op_eicall(),
            Op::Eijmp => self.op_eijmp(),
            Op::Elpmr0 => self.op_elpm(),
            Op::Elpm { .. } => self.op_elpm(),
            Op::Eor { d, r } => self.op_eor(d, r),
            Op::Fmul { d, r } => self.op_fmul(d, r),
            Op::Fmuls { d, r } => self.op_fmuls(d, r),
            Op::Fmulsu { d, r } => self.op_fmulsu(d, r),
            Op::Icall => self.op_icall(),
            Op::Ijmp => self.op_ijmp(),
            Op::In { d, a } => self.op_in(d, a),
            Op::Inc { d } => self.op_inc(d),
            Op::Jmp { k } => self.op_jmp(k),
            Op::Ld { d, idx, ext } => self.op_ld(d, idx, ext),
            Op::Ldi { d, k } => self.op_ldi(d, k),
            Op::Lds { d, k } => self.op_lds(d, k),
            Op::Lpmr0 => self.op_lpm(0, false),
            Op::Lpm { d, inc } => self.op_lpm(d, inc),
            Op::Lsr { d } => self.op_lsr(d),
            Op::Mov { d, r } => self.op_mov(d, r),
            Op::Movw { d, r } => self.op_movw(d, r),
            Op::Mul { d, r } => self.op_mul(d, r),
            Op::Muls { d, r } => self.op_muls(d, r),
            Op::Mulsu { d, r } => self.op_mulsu(d, r),
            Op::Neg { d } => self.op_neg(d),
            Op::Nop => self.op_nop(),
            Op::Or { d, r } => self.op_or(d, r),
            Op::Ori { d, k } => self.op_ori(d, k),
            Op::Out { a, r } => self.op_out(a, r),
            Op::Pop { d } => self.op_pop(d),
            Op::Push { r } => self.op_push(r),
            Op::Rcall { k } => self.op_rcall(k),
            Op::Ret => self.op_ret(),
            Op::Reti => self.op_reti(),
            Op::Rjmp { k } => self.op_rjmp(k),
            Op::Ror { d } => self.op_ror(d),
            Op::Sbc { d, r } => self.op_sbc(d, r),
            Op::Sbci { d, k } => self.op_sbci(d, k),
            Op::Sbi { a, b } => self.op_sbi(a, b),
            Op::Sbic { a, b } => self.op_sbic(a, b, self.op1().words()),
            Op::Sbis { a, b } => self.op_sbis(a, b, self.op1().words()),
            Op::Sbiw { d, k } => self.op_sbiw(d, k),
            Op::Sbrc { r, b } => self.op_sbrc(r, b, self.op1().words()),
            Op::Sbrs { r, b } => self.op_sbrs(r, b, self.op1().words()),
            // Op::Ser { d } => self.op_ser(d),
            Op::Sleep => self.op_sleep(),
            Op::Spm => self.op_spm(),
            Op::Spm2 => self.op_spm2(),
            Op::St { r, idx, ext } => self.op_st(r, idx, ext),
            Op::Sts { k, r } => self.op_sts(k, r),
            Op::Sub { d, r } => self.op_sub(d, r),
            Op::Subi { d, k } => self.op_subi(d, k),
            Op::Swap { d } => self.op_swap(d),
            Op::Wdr => self.op_wdr(),
            Op::Undefined { w } => {
                warn!(
                    "Tried to execute undefined op 0x{:04x} at address 0x{:04x}",
                    w, self.pc
                );
                unreachable!();
            }
        }
    }
}

#[cfg(test)]
mod test;

#[cfg(test)]
mod test_vectors;
