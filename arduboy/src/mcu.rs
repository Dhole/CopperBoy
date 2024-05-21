// #[cfg(feature = "stats")]
// use core::fmt::Write;
#[cfg(feature = "stats")]
use num_traits::{FromPrimitive, ToPrimitive};
use std::mem::transmute;
use unroll::unroll_for_loops;

// #[cfg(feature = "std")]
// use std::vec::Vec;

#[cfg(not(feature = "std"))]
use alloc::vec;
#[cfg(not(feature = "std"))]
use alloc::vec::Vec;

use core::fmt;
use core::mem;

use serde::{self, Deserialize, Serialize};

use super::clock;
use super::display;
use super::eeprom;
use super::int_vec::*;
use super::opcodes::*;
use super::*;

#[cfg(feature = "stats")]
use super::io_regs::io_reg_str;

const RES_SR_N: u16 = 1 << 09;
const RES_SR_V: u16 = 1 << 10;
const RES_SR_S: u16 = 1 << 11;
const RES_SR_H: u16 = 1 << 12;
const RES_SR_Z: u16 = 1 << 13;
const RES_SR_C: u16 = 1 << 14;

#[derive(PartialEq, Serialize, Deserialize, Clone)]
pub struct StatusRegister {
    /// Carry Flag
    c: bool,
    /// Zero Flag
    z: bool,
    /// Negative Flag
    n: bool,
    /// Two's Complement Overflow Flag
    v: bool,
    /// Sign Bit
    s: bool,
    /// Half Carry Flag
    h: bool,
    /// Bit Copy Storage
    t: bool,
    /// Global Interrupt Enable
    i: bool,
}
// static_assertions::assert_eq_size!(StatusRegister, u8);

impl Index<u8> for StatusRegister {
    type Output = bool;

    #[inline(always)]
    fn index(&self, i: u8) -> &bool {
        unsafe { transmute::<&Self, &[bool; 8]>(self).get_unchecked(i as usize) }
        // match i {
        //     0 => &self.c,
        //     1 => &self.z,
        //     2 => &self.n,
        //     3 => &self.v,
        //     4 => &self.s,
        //     5 => &self.h,
        //     6 => &self.t,
        //     7 => &self.i,
        //     _ => unreachable!(),
        // }
    }
}

impl IndexMut<u8> for StatusRegister {
    fn index_mut(&mut self, i: u8) -> &mut bool {
        unsafe { transmute::<&mut Self, &mut [bool; 8]>(self).get_unchecked_mut(i as usize) }
        // match i {
        //     0 => &mut self.c,
        //     1 => &mut self.z,
        //     2 => &mut self.n,
        //     3 => &mut self.v,
        //     4 => &mut self.s,
        //     5 => &mut self.h,
        //     6 => &mut self.t,
        //     7 => &mut self.i,
        //     _ => unreachable!(),
        // }
    }
}

// #[cfg(feature = "std")]
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
    pub fn new() -> Self {
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

#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub struct GeneralRegisters {
    reg: [u8; 32],
}

impl GeneralRegisters {
    pub fn new() -> Self {
        Self { reg: [0; 32] }
    }
}

impl Index<u8> for GeneralRegisters {
    type Output = u8;

    #[inline(always)]
    #[cfg(not(feature = "vec_unchecked"))]
    fn index(&self, i: u8) -> &u8 {
        &self.reg[i as usize]
    }

    #[inline(always)]
    #[cfg(feature = "vec_unchecked")]
    fn index(&self, i: u8) -> &u8 {
        unsafe { self.reg.get_unchecked(i as usize) }
    }
}

impl IndexMut<u8> for GeneralRegisters {
    #[inline(always)]
    #[cfg(not(feature = "vec_unchecked"))]
    fn index_mut(&mut self, i: u8) -> &mut u8 {
        &mut self.reg[i as usize]
    }

    #[inline(always)]
    #[cfg(feature = "vec_unchecked")]
    fn index_mut(&mut self, i: u8) -> &mut u8 {
        unsafe { self.reg.get_unchecked_mut(i as usize) }
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

#[cfg(feature = "stats")]
#[derive(Clone)]
pub struct Stats {
    pub loads: Vec<usize>,
    pub stores: Vec<usize>,
    pub ops: Vec<usize>,
    pub op_prev: Op,
    pub ops2: Vec<usize>,
}

#[cfg(feature = "stats")]
impl Stats {
    fn new() -> Self {
        let ops_len = OpType::Undefined.to_usize().unwrap();
        Self {
            loads: vec![0; 0xb00],
            stores: vec![0; 0xb00],
            ops: vec![0; ops_len],
            op_prev: Op::Nop,
            ops2: vec![0; ops_len * ops_len],
        }
    }

    pub fn write_summary<W: std::io::Write>(
        &self,
        mut out: W,
        n_loads: usize,
        n_stores: usize,
        n_ops: usize,
    ) -> Result<(), std::io::Error> {
        let mut loads: Vec<(usize, usize)> = self
            .loads
            .iter()
            .enumerate()
            .map(|(addr, count)| (addr, *count))
            .collect();
        loads.sort_by(|(_, a_count), (_, b_count)| b_count.cmp(a_count));
        writeln!(out, "# Loads")?;
        let width = format!("{}", loads[0].1).len();
        for (addr, count) in loads.iter().take(n_loads) {
            writeln!(
                out,
                "0x{:04x}: {:width$} ; {:?}",
                addr,
                count,
                io_reg_str(*addr as u16).unwrap_or(("", "")),
                width = width,
            )?;
        }

        let mut stores: Vec<(usize, usize)> = self
            .stores
            .iter()
            .enumerate()
            .map(|(addr, count)| (addr, *count))
            .collect();
        stores.sort_by(|(_, a_count), (_, b_count)| b_count.cmp(a_count));
        writeln!(out, "\n# Stores")?;
        let width = format!("{}", stores[0].1).len();
        for (addr, count) in stores.iter().take(n_stores) {
            writeln!(
                out,
                "0x{:04x}: {:width$} ; {:?}",
                addr,
                count,
                io_reg_str(*addr as u16).unwrap_or(("", "")),
                width = width,
            )?;
        }

        let mut ops: Vec<(OpType, usize)> = self
            .ops
            .iter()
            .enumerate()
            .map(|(op_num, count)| (OpType::from_usize(op_num).unwrap(), *count))
            .collect();
        ops.sort_by(|(_, a_count), (_, b_count)| b_count.cmp(a_count));
        writeln!(out, "\n# Ops")?;
        let width = format!("{}", ops[0].1).len();
        let ops_len: usize = ops.iter().map(|(_, count)| count).sum();
        for (op, count) in ops.iter().take(n_ops) {
            writeln!(
                out,
                "{:<6}: {:width$} ({perc:.04} %)",
                format!("{:?}", op),
                count,
                width = width,
                perc = (*count as f64) * 100.0 / (ops_len as f64),
            )?;
        }

        let ops_len = OpType::Undefined.to_usize().unwrap();
        let mut ops2: Vec<((OpType, OpType), usize)> = self
            .ops2
            .iter()
            .enumerate()
            .map(|(op_num_2, count)| {
                (
                    (
                        OpType::from_usize(op_num_2 % ops_len).unwrap(),
                        OpType::from_usize(op_num_2 / ops_len).unwrap(),
                    ),
                    *count,
                )
            })
            .collect();
        ops2.sort_by(|(_, a_count), (_, b_count)| b_count.cmp(a_count));
        writeln!(out, "\n# Ops x 2")?;
        let ops2_len: usize = ops2.iter().map(|(_, count)| count).sum();
        let width = format!("{}", ops2[0].1).len();
        for (op, count) in ops2.iter().take(n_ops) {
            writeln!(
                out,
                "{:<16}: {:width$} ({perc:.04} %)",
                format!("{:?}", op),
                count,
                width = width,
                perc = (*count as f64) * 100.0 / (ops2_len as f64),
            )?;
        }
        Ok(())
    }
}

#[cfg(feature = "stats")]
impl Default for Stats {
    fn default() -> Self {
        Self::new()
    }
}

pub const SRAM_SIZE: u16 = 0x0a00;
pub const SRAM_ADDR: u16 = 0x0100;
pub const IOSPACE_SIZE: u16 = 0x0040;
pub const IOSPACE_ADDR: u16 = 0x0020;
pub const DATA_SIZE: u16 = 0x0b00;
pub const PROGRAM_SIZE: u16 = 0x8000;

#[cfg_attr(test, derive(core::cmp::PartialEq, core::fmt::Debug))]
#[derive(Serialize, Deserialize, Clone)]
pub struct Core {
    /// Status Register
    status_reg: StatusRegister,
    /// General Purpose Register File
    pub regs: GeneralRegisters,
    /// IO Registers
    io_space: Vec<u8>,
    /// Program Counter
    pub pc: u16,
    /// Stack Pointer
    pub sp: u16,
    /// SRAM
    sram: Vec<u8>,
    /// Program Memory
    #[serde(skip)]
    pub program: Vec<u8>,
    #[serde(skip)]
    pub program_ops: Vec<Op>,
    // program_fast: Vec<Option<Vec<Box<dyn FnMut(Core) -> usize>>>>,
    /// Set if the previos instruction branched.  This flag is used to know if the instruction
    /// ahead must be fetched.
    branch: bool,
    /// Next op
    // op1: Op,
    // Peripherials
    clock: clock::Clock,
    eeprom: eeprom::Eeprom,
    pub display: display::Display,
    /// Sleeping?
    // pub sleep: bool,
    // Sleep is None when not sleeping, and the Op at pc when sleeping
    sleep: Option<Op>,
    // sleep_op: Op,
    pub gpio: GPIO,
    #[cfg(feature = "stats")]
    #[serde(skip)]
    pub stats: Stats,
    #[serde(skip)]
    lut_adc: Vec<u16>,
}

#[cfg_attr(test, derive(core::cmp::PartialEq, core::fmt::Debug))]
#[derive(Serialize, Deserialize, Clone)]
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
        // if let GPIOPort::C = port {
        //     println!("set port {:?} {:02x}", port, v);
        // }
        match port {
            GPIOPort::B => self.pin_b = v,
            GPIOPort::C => self.pin_c = v,
            GPIOPort::D => self.pin_d = v,
            GPIOPort::E => self.pin_e = v,
            GPIOPort::F => self.pin_f = v,
        }
    }

    pub fn port_c(&self) -> u8 {
        self.pin_c
    }
}

#[derive(Debug)]
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
            io_space: vec![0; IOSPACE_SIZE as usize],
            status_reg: StatusRegister::new(),
            pc: 0,
            sram: vec![0; SRAM_SIZE as usize],
            program: vec![0; PROGRAM_SIZE as usize],
            // program_fast: vec![None; PROGRAM_SIZE as usize],
            program_ops: vec![Op::Nop; PROGRAM_SIZE as usize],
            sp: SRAM_ADDR + SRAM_SIZE - 1,
            branch: false,
            // op1: Op::Undefined { w: 0x0000 },
            // Peripherials
            clock: clock::Clock::new(),
            eeprom: eeprom::Eeprom::new(),
            sleep: None,
            // sleep_op: Op::Nop,
            display: display::Display::new(),
            gpio: GPIO::new(),
            #[cfg(feature = "stats")]
            stats: Stats::new(),
            lut_adc: Vec::new(),
        }
    }

    pub fn serialize_len(&self) -> postcard::Result<usize> {
        const BUF_LEN: usize = 0x10000;
        let mut buf = vec![0; BUF_LEN];
        let bin = postcard::to_slice(&self, &mut buf)?;
        Ok(bin.len())
    }

    pub fn serialize(&self, bin: &mut [u8]) -> postcard::Result<()> {
        postcard::to_slice(&self, bin)?;
        Ok(())
    }

    pub fn deserialize_pre(&mut self) {
        self.sleep_unset();
    }

    pub fn deserialize_post(&mut self) {
        if self.sleep.is_some() {
            self.program_ops[self.pc as usize] = Op::Zzz;
        }
        self.display.deserialize_post();
    }

    pub fn deserialize(&mut self, bin: &[u8]) -> postcard::Result<()> {
        let mut new: Core = postcard::from_bytes(bin)?;
        mem::swap(&mut new.program, &mut self.program);
        mem::swap(&mut new.program_ops, &mut self.program_ops);
        *self = new;
        Ok(())
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
    pub fn flash_write(&mut self, addr: u16, v: u8) {
        self.program[addr as usize] = v;
        // align address to 16 bit words and store the decoded instruction.
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
        // Filter two word ops, and return otherwise
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

    pub fn optimize_op_pairs(&mut self) {
        let mut i = 0;
        loop {
            if i >= self.program_ops.len() - 1 {
                break;
            }
            let op0 = self.program_ops[i];
            let op1 = self.program_ops[i + 1];
            match op0 {
                Op::Lds { d, k } => {
                    if k >= SRAM_ADDR {
                        self.program_ops[i] = Op::LdsSRAM { d, k };
                    }
                }
                _ => {}
            }
            match (op0, op1) {
                (Op::Add { r, d }, Op::Adc { .. }) => {
                    self.program_ops[i] = Op::AddAdc { r, d };
                    i += 2;
                }
                (Op::Adc { r, d }, Op::Adc { .. }) => {
                    self.program_ops[i] = Op::AdcAdc { r, d };
                    i += 2;
                }
                (Op::Subi { d, k }, Op::Sbci { .. }) => {
                    self.program_ops[i] = Op::SubiSbci { d, k };
                    i += 2;
                }
                (Op::Dec { d }, Op::Brbc { s, .. }) => {
                    // 1 is flag Z
                    if s == 1 {
                        self.program_ops[i] = Op::DecBrbcZ { d };
                        i += 2;
                    } else {
                        i += 1;
                    }
                }
                // (Op::Lds { d, k }, Op::Lds { d: _, k: k1 }) => {
                //     println!("Lds Lds {:04x} {:04x}", k, k1);
                //     self.program_ops[i] = Op::LdsLds { d, k };
                //     i += 2;
                // }
                (Op::Cp { d, r }, Op::Cpc { .. }) => {
                    self.program_ops[i] = Op::CpCpc { d, r };
                    i += 2;
                }
                (Op::Cpc { d, r }, Op::Brbs { s, .. }) => {
                    self.program_ops[i] = Op::CpcBrbs { d, r };
                    i += 2;
                }
                (_, _) => i += 1,
            }
        }
    }

    pub fn op1(&self) -> Op {
        vec_get!(self.program_ops, self.pc as usize + 1)
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

    pub fn sleeping(&self) -> bool {
        self.sleep.is_some()
    }

    fn sleep_set(&mut self) {
        self.sleep = Some(self.program_ops[self.pc as usize]);
        // self.sleep = true;
        // self.sleep_op = self.program_ops[self.pc as usize];
        self.program_ops[self.pc as usize] = Op::Zzz;
    }

    pub fn sleep_unset(&mut self) {
        match self.sleep.take() {
            None => {}
            Some(op) => self.program_ops[self.pc as usize] = op,
        }
    }

    /// Step one instruction.  Return the number of cycles that have passed.
    pub fn step(&mut self) -> usize {
        // if self.sleep {
        //     return 1;
        // }
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
        // let op = self.get_next_op();
        let op = vec_get!(self.program_ops, self.pc as usize);
        // self.exec_op(self.program_ops[self.pc as usize])
        self.exec_op(op)
    }

    #[unroll_for_loops]
    pub fn step_n<const N: usize>(&mut self) -> usize {
        let mut cycles = 0;
        for _i in 0..N {
            let op = vec_get!(self.program_ops, self.pc as usize);
            cycles += self.exec_op(op);
        }
        cycles
    }

    // returns true if an interrupt is fired
    pub fn step_hw(&mut self, cycles: usize) -> bool {
        // The interrupts have priority in accordance with their Interrupt Vector position.  The
        // lower the Interrupt Vector address, the higher the priority.

        // The Global Interrupt Enable Register is cleared by hardware after an interrupt has
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

        self.sleep_unset();
        // println!("INTERRUPT 1");
        self.status_reg.i = false;

        let pc = if interrupt_bitmap & RESET_BIT != 0 {
            debug!("Handling interrupt RESET");
            RESET
        } else if interrupt_bitmap & INT0_BIT != 0 {
            debug!("Handling interrupt INT0");
            INT0
        } else if interrupt_bitmap & INT1_BIT != 0 {
            debug!("Handling interrupt INT1");
            INT1
        } else if interrupt_bitmap & INT2_BIT != 0 {
            debug!("Handling interrupt INT2");
            INT2
        } else if interrupt_bitmap & INT3_BIT != 0 {
            debug!("Handling interrupt INT3");
            INT3
        } else if interrupt_bitmap & INT6_BIT != 0 {
            debug!("Handling interrupt INT6");
            INT6
        } else if interrupt_bitmap & PCINT0_BIT != 0 {
            debug!("Handling interrupt PCINT0");
            PCINT0
        } else if interrupt_bitmap & USB_GENERAL_BIT != 0 {
            debug!("Handling interrupt USB_GENERAL");
            USB_GENERAL
        } else if interrupt_bitmap & USB_ENDPOINT_BIT != 0 {
            debug!("Handling interrupt USB_ENDPOINT");
            USB_ENDPOINT
        } else if interrupt_bitmap & WDT_BIT != 0 {
            debug!("Handling interrupt WDT");
            WDT
        } else if interrupt_bitmap & TIMER1_CAPT_BIT != 0 {
            debug!("Handling interrupt TIMER1_CAPT");
            TIMER1_CAPT
        } else if interrupt_bitmap & TIMER1_COMPA_BIT != 0 {
            debug!("Handling interrupt TIMER1_COMPA");
            TIMER1_COMPA
        } else if interrupt_bitmap & TIMER1_COMPB_BIT != 0 {
            debug!("Handling interrupt TIMER1_COMPB");
            TIMER1_COMPB
        } else if interrupt_bitmap & TIMER1_COMPC_BIT != 0 {
            debug!("Handling interrupt TIMER1_COMPC");
            TIMER1_COMPC
        } else if interrupt_bitmap & TIMER1_OVF_BIT != 0 {
            debug!("Handling interrupt TIMER1_OVF");
            TIMER1_OVF
        } else if interrupt_bitmap & TIMER0_COMPA_BIT != 0 {
            debug!("Handling interrupt TIMER0_COMPA");
            TIMER0_COMPA
        } else if interrupt_bitmap & TIMER0_COMPB_BIT != 0 {
            debug!("Handling interrupt TIMER0_COMPB");
            TIMER0_COMPB
        } else if interrupt_bitmap & TIMER0_OVF_BIT != 0 {
            debug!("Handling interrupt TIMER0_OVF");
            self.clock.clear_int(Interrupt::Timer0Ovf);
            TIMER0_OVF
        } else if interrupt_bitmap & SPI_STC_BIT != 0 {
            debug!("Handling interrupt SPI_STC");
            SPI_STC
        } else if interrupt_bitmap & USART1_RX_BIT != 0 {
            debug!("Handling interrupt USART1_RX");
            USART1_RX
        } else if interrupt_bitmap & USART2_UDRE_BIT != 0 {
            debug!("Handling interrupt USART2_UDRE");
            USART2_UDRE
        } else if interrupt_bitmap & USART1_TX_BIT != 0 {
            debug!("Handling interrupt USART1_TX");
            USART1_TX
        } else if interrupt_bitmap & ANALOG_COMP_BIT != 0 {
            debug!("Handling interrupt ANALOG_COMP");
            ANALOG_COMP
        } else if interrupt_bitmap & ADC_BIT != 0 {
            debug!("Handling interrupt ADC");
            ADC
        } else if interrupt_bitmap & EE_READY_BIT != 0 {
            debug!("Handling interrupt EE_READY");
            EE_READY
        } else if interrupt_bitmap & TIMER3_CAPT_BIT != 0 {
            debug!("Handling interrupt TIMER3_CAPT");
            TIMER3_CAPT
        } else if interrupt_bitmap & TIMER3_COMPA_BIT != 0 {
            debug!("Handling interrupt TIMER3_COMPA");
            // println!("timer3_comp_a int");
            self.clock.clear_int(Interrupt::Timer3CompA);
            TIMER3_COMPA
        } else if interrupt_bitmap & TIMER3_COMPB_BIT != 0 {
            debug!("Handling interrupt TIMER3_COMPB");
            self.clock.clear_int(Interrupt::Timer3CompB);
            TIMER3_COMPB
        } else if interrupt_bitmap & TIMER3_COMPC_BIT != 0 {
            debug!("Handling interrupt TIMER3_COMPC");
            self.clock.clear_int(Interrupt::Timer3CompC);
            TIMER3_COMPC
        } else if interrupt_bitmap & TIMER3_OVF_BIT != 0 {
            debug!("Handling interrupt TIMER3_OVF");
            self.clock.clear_int(Interrupt::Timer3Ovf);
            TIMER3_OVF
        } else if interrupt_bitmap & TWI_BIT != 0 {
            debug!("Handling interrupt TWI");
            TWI
        } else if interrupt_bitmap & STM_READY_BIT != 0 {
            debug!("Handling interrupt STM_READY");
            STM_READY
        } else if interrupt_bitmap & TIMER4_COMPA_BIT != 0 {
            debug!("Handling interrupt TIMER4_COMPA");
            TIMER4_COMPA
        } else if interrupt_bitmap & TIMER4_COMPB_BIT != 0 {
            debug!("Handling interrupt TIMER4_COMPB");
            TIMER4_COMPB
        } else if interrupt_bitmap & TIMER4_COMPD_BIT != 0 {
            debug!("Handling interrupt TIMER4_COMPD");
            TIMER4_COMPD
        } else if interrupt_bitmap & TIMER4_OVF_BIT != 0 {
            debug!("Handling interrupt TIMER4_OVF");
            TIMER4_OVF
        } else if interrupt_bitmap & TIMER4_FPF_BIT != 0 {
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
    pub fn data_load(&mut self, addr: u16) -> u8 {
        #[cfg(feature = "stats")]
        {
            if addr < SRAM_ADDR {
                self.stats.loads[addr as usize] += 1;
            }
        }

        if addr >= SRAM_ADDR {
            // self.sram[(addr - SRAM_ADDR) as usize]
            vec_get!(self.sram, (addr - SRAM_ADDR) as usize)
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
                io_regs::PORTC => {
                    // println!("read  port C: {:08b}", self.gpio.pin_c);
                    self.gpio.pin_c
                }
                io_regs::PORTD => {
                    if self.display.dc() {
                        1 << 4
                    } else {
                        0
                    }
                } // TODO
                io_regs::PORTE => 0,              // TODO
                io_regs::PORTF => 0,              // TODO
                io_regs::DDRC => 0,               // TODO
                io_regs::SPCR => 0,               // TODO
                io_regs::SPSR => 0b10000000,      // TODO
                io_regs::SPDR => 0,               // TODO
                io_regs::PRR0 => 0,               // TODO
                io_regs::PRR1 => 0,               // TODO
                io_regs::SMCR => 0,               // TODO
                io_regs::PINB => self.gpio.pin_b, // TODO
                io_regs::PINC => {
                    // println!("read pin c {:02x}", self.gpio.pin_c);
                    self.gpio.pin_c
                } // TODO
                io_regs::PIND => self.gpio.pin_d, // TODO
                io_regs::PINE => self.gpio.pin_e, // TODO
                io_regs::PINF => self.gpio.pin_f, // TODO
                io_regs::PLLCSR => self.clock.reg_pllcsr(), // TODO: PLL Control and Status Register
                io_regs::EEDR => self.eeprom.reg_eedr(), // TODO
                io_regs::EECR => self.eeprom.reg_eecr(), // TODO
                io_regs::ADCL => 0,               // TODO
                io_regs::ADCH => 0,               // TODO
                io_regs::ADCSRB => 0,             // TODO
                // 0x20 => 0,            // TODO
                // 0x21 => 0,            // TODO
                // 0x22 => 0,            // TODO
                // 0x32 => 0,            // TODO
                // 0x33 => 0,            // TODO
                // 0x34 => 0,            // TODO
                io_regs::WDTCSR => 0, // TODO
                io_regs::CLKPR => 0,  // TODO
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
    fn data_load_u16(&mut self, addr: u16) -> u16 {
        u16::from_le_bytes([self.data_load(addr), self.data_load(addr + 1)])
    }

    /// Store a byte into the User Data Space
    fn data_store(&mut self, addr: u16, v: u8) {
        #[cfg(feature = "stats")]
        {
            if addr < SRAM_ADDR {
                self.stats.stores[addr as usize] += 1;
            }
        }

        if addr >= SRAM_ADDR {
            // if 0x0155 <= addr && addr <= 0x0158 {
            //     println!("write {:04x} <- {:02x}", addr, v);
            // }
            // self.sram[(addr - SRAM_ADDR) as usize] = v;
            vec_set!(self.sram, (addr - SRAM_ADDR) as usize, v);
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
                io_regs::PORTC => {
                    // println!("write port C: {:08b}", v);
                    self.gpio.set_port(GPIOPort::C, v);
                }
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
                io_regs::PINC => {
                    // println!("write pin c {:02x}", v);
                } // TODO
                io_regs::PIND => {} // TODO
                io_regs::PINE => {} // TODO
                io_regs::PINF => {} // TODO
                io_regs::PLLCSR => self.clock.set_reg_pllcsr(v), // TODO: PLL Control and Status Register
                io_regs::EEARL => self.eeprom.set_reg_eearl(v),  // TODO
                io_regs::EEARH => self.eeprom.set_reg_eearh(v),  // TODO
                io_regs::EECR => self.eeprom.set_reg_eecr(v),    // TODO
                io_regs::TCCR3C => {}                            // TODO
                io_regs::ICR3H => self.clock.set_reg_icr3h(v),   // TODO
                io_regs::ICR3L => self.clock.set_reg_icr3l(v),   // TODO
                io_regs::EEDR => self.eeprom.set_reg_eedr(v),    // TODO
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
                    // TODO
                    warn!("DBG: {:02x}", v);
                }
                io_regs::GPIOR2 => {
                    // TODO
                    // warn!("DBG: {:02x}", v);
                }
                io_regs::TC4H => {}  // TODO
                io_regs::OCR4C => {} // TODO
                io_regs::OCR4B => {} // TODO
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
    #[inline(always)]
    fn data_store_u16(&mut self, addr: u16, v: u16) {
        let bytes = v.to_le_bytes();
        self.data_store(addr, bytes[0]);
        self.data_store(addr + 1, bytes[1]);
    }

    /// Push a word into the stack
    #[inline(always)]
    fn push_u16(&mut self, v: u16) {
        let bytes = v.to_le_bytes();
        // self.sram[(self.sp - SRAM_ADDR - 1) as usize] = bytes[0];
        vec_set!(self.sram, (self.sp - SRAM_ADDR - 1) as usize, bytes[0]);
        // self.sram[(self.sp - SRAM_ADDR) as usize] = bytes[1];
        vec_set!(self.sram, (self.sp - SRAM_ADDR) as usize, bytes[1]);
        self.sp -= 2;
    }

    /// Pop a word from the stack
    #[inline(always)]
    fn pop_u16(&mut self) -> u16 {
        self.sp += 2;
        u16::from_le_bytes([
            // self.sram[(self.sp - SRAM_ADDR - 1) as usize],
            vec_get!(self.sram, (self.sp - SRAM_ADDR - 1) as usize),
            // self.sram[(self.sp - SRAM_ADDR) as usize],
            vec_get!(self.sram, (self.sp - SRAM_ADDR) as usize),
        ])
    }

    // Auxiliary function to update some flags in adc and add
    #[inline(always)]
    fn aux_op_add_flags(&mut self, a: u8, b: u8, c: bool, res: u8) {
        let (r7, rr7, rd7) = (res & 1 << 7, b & 1 << 7, a & 1 << 7);
        self.status_reg.n = r7 != 0;
        self.status_reg.v = (rr7 == rd7) && (rr7 != r7);
        self.status_reg.s = self.status_reg.n ^ self.status_reg.v;
        self.status_reg.h = ((b & 0x0f) + (a & 0x0f) + c as u8) > 0x0f;
        self.status_reg.z = res == 0;
    }
    /// 5. Add with Carry (ADC Rd, Rr) OK
    #[inline(always)]
    fn op_adc(&mut self, d: u8, r: u8) -> usize {
        let (res, c0) = self.regs[d].overflowing_add(self.regs[r]);
        let (res, c1) = res.overflowing_add(self.status_reg.c as u8);
        self.aux_op_add_flags(self.regs[d], self.regs[r], self.status_reg.c, res);
        self.status_reg.c = c0 || c1;
        self.regs[d] = res;

        self.pc += 1;
        1
    }
    #[inline(always)]
    fn op_lut_adc(&mut self, d: u8, r: u8) -> usize {
        let index = (self.regs[d] as usize)
            | ((self.regs[r] as usize) << 8)
            | ((self.status_reg.c as usize) << 16);
        let result = vec_get!(self.lut_adc, index);
        self.regs[d] = (result & 0xff) as u8;
        self.status_reg.n = (result & RES_SR_N) != 0;
        self.status_reg.v = (result & RES_SR_V) != 0;
        self.status_reg.s = (result & RES_SR_S) != 0;
        self.status_reg.h = (result & RES_SR_H) != 0;
        self.status_reg.z = (result & RES_SR_Z) != 0;
        self.status_reg.c = (result & RES_SR_C) != 0;

        self.pc += 1;
        1
    }

    pub fn enable_lut_adc(&mut self) {
        self.lut_adc = vec![0; 256 * 256 * 2];
        for a in 0u8..=255 {
            for b in 0u8..=255 {
                for c in [false, true] {
                    self.pc = 0;
                    self.regs[0] = a;
                    self.regs[1] = b;
                    self.status_reg.c = c;
                    self.op_adc(0, 1);
                    let mut result: u16 = self.regs[0] as u16;
                    result |= if self.status_reg.n { RES_SR_N } else { 0 };
                    result |= if self.status_reg.v { RES_SR_V } else { 0 };
                    result |= if self.status_reg.s { RES_SR_S } else { 0 };
                    result |= if self.status_reg.h { RES_SR_H } else { 0 };
                    result |= if self.status_reg.z { RES_SR_Z } else { 0 };
                    result |= if self.status_reg.c { RES_SR_C } else { 0 };
                    let index = (a as usize) | ((b as usize) << 8) | ((c as usize) << 16);
                    self.lut_adc[index] = result;
                }
            }
        }
        self.reset();
    }

    /// 6. Add without Carry (ADD Rd, Rr) OK
    #[inline(always)]
    fn op_add(&mut self, d: u8, r: u8) -> usize {
        let (res, c0) = self.regs[d].overflowing_add(self.regs[r]);
        self.status_reg.c = c0;
        self.aux_op_add_flags(self.regs[d], self.regs[r], false, res);
        self.regs[d] = res;

        self.pc += 1;
        1
    }

    #[inline(always)]
    fn op2_adc_adc(&mut self, d: u8, r: u8, d1: u8, r1: u8) -> usize {
        let (res, c0) = self.regs[d].overflowing_add(self.regs[r]);
        let (res, c1) = res.overflowing_add(self.status_reg.c as u8);
        let status_reg_c = c0 || c1;
        self.regs[d] = res;

        let (d, r) = (d1, r1);
        let (res, c0) = self.regs[d].overflowing_add(self.regs[r]);
        let (res, c1) = res.overflowing_add(status_reg_c as u8);
        self.aux_op_add_flags(self.regs[d], self.regs[r], status_reg_c, res);
        self.status_reg.c = c0 || c1;
        self.regs[d] = res;

        self.pc += 2;
        2
    }
    #[inline(always)]
    fn op2_add_adc(&mut self, d: u8, r: u8, d1: u8, r1: u8) -> usize {
        let (res, c0) = self.regs[d].overflowing_add(self.regs[r]);
        let status_reg_c = c0;
        self.regs[d] = res;

        let (d, r) = (d1, r1);
        let (res, c0) = self.regs[d].overflowing_add(self.regs[r]);
        let (res, c1) = res.overflowing_add(status_reg_c as u8);
        self.aux_op_add_flags(self.regs[d], self.regs[r], status_reg_c, res);
        self.status_reg.c = c0 || c1;
        self.regs[d] = res;

        self.pc += 2;
        2
    }
    #[inline(always)]
    fn op2_subi_sbci(&mut self, d: u8, k: u8, d1: u8, k1: u8) -> usize {
        let (res, c0) = self.regs[d].overflowing_sub(k);
        let status_reg_z = res == 0;
        let status_reg_c = c0;
        self.regs[d] = res;

        let (d, k) = (d1, k1);
        let (res, c0) = self.regs[d].overflowing_sub(k);
        let (res, c1) = res.overflowing_sub(status_reg_c as u8);
        self.aux_op_sub_flags(self.regs[d], k, status_reg_c, res);
        self.status_reg.c = c0 || c1;
        if res != 0 {
            self.status_reg.z = false;
        } else {
            self.status_reg.z = status_reg_z;
        }
        self.regs[d] = res;

        self.pc += 2;
        2
    }
    #[inline(always)]
    fn op2_dec_brbc_z(&mut self, d: u8, k: i8) -> usize {
        let (res, _) = self.regs[d].overflowing_sub(1);
        let z = res == 0;
        self.status_reg.z = z;
        self.regs[d] = res;
        self.pc += 1;

        self.aux_op_branch_if(k, !z)
    }
    #[inline(always)]
    fn op2_lds_lds(&mut self, d: u8, k: u16, d1: u8, k1: u16) -> usize {
        self.regs[d] = self.data_load(k);
        self.regs[d1] = self.data_load(k1);
        self.pc += 4;
        4
    }
    #[inline(always)]
    fn op2_cp_cpc(&mut self, d: u8, r: u8, d1: u8, r1: u8) -> usize {
        let (res, c0) = self.regs[d].overflowing_sub(self.regs[r]);
        let status_reg_z = res == 0;
        let status_reg_c = c0;

        let (d, r) = (d1, r1);
        let (res, c0) = self.regs[d].overflowing_sub(self.regs[r]);
        let (res, c1) = res.overflowing_sub(status_reg_c as u8);
        self.aux_op_cp_flags(self.regs[d], self.regs[r], status_reg_c, res);
        if res != 0 {
            self.status_reg.z = false;
        } else {
            self.status_reg.z = status_reg_z;
        }
        self.status_reg.c = c0 || c1;

        self.pc += 2;
        2
    }
    #[inline(always)]
    fn op2_cpi_brbc(&mut self, d: u8, k: u8, s: u8, k1: i8) -> usize {
        let (res, c0) = self.regs[d].overflowing_sub(k);

        self.aux_op_cp_flags(self.regs[d], k, false, res);
        self.status_reg.z = res == 0;
        self.status_reg.c = c0;

        self.pc += 1;

        self.aux_op_branch_if(k1, !self.status_reg[s])
    }
    #[inline(always)]
    fn op2_cpc_brbs(&mut self, d: u8, r: u8, s: u8, k: i8) -> usize {
        let (res, c0) = self.regs[d].overflowing_sub(self.regs[r]);
        let (res, c1) = res.overflowing_sub(self.status_reg.c as u8);
        self.aux_op_cp_flags(self.regs[d], self.regs[r], self.status_reg.c, res);
        if res != 0 {
            self.status_reg.z = false;
        }
        self.status_reg.c = c0 || c1;

        self.pc += 1;

        self.aux_op_branch_if(k, self.status_reg[s])
    }
    #[inline(always)]
    fn op2_cpc_brbc(&mut self, d: u8, r: u8, s: u8, k: i8) -> usize {
        let (res, c0) = self.regs[d].overflowing_sub(self.regs[r]);
        let (res, c1) = res.overflowing_sub(self.status_reg.c as u8);
        self.aux_op_cp_flags(self.regs[d], self.regs[r], self.status_reg.c, res);
        if res != 0 {
            self.status_reg.z = false;
        }
        self.status_reg.c = c0 || c1;

        self.pc += 1;

        self.aux_op_branch_if(k, !self.status_reg[s])
    }

    /// 7. Add Immediate Word (ADIW Rdl, K) OK
    #[inline(always)]
    fn op_adiw(&mut self, d: u8, k: u8) -> usize {
        let ext = self.regs.ext(d);
        let (res, c) = ext.overflowing_add(k as u16);
        let (r15, rdh7) = (res & 1 << 15, ext & 1 << 15);
        self.status_reg.n = r15 != 0;
        self.status_reg.v = (rdh7 == 0) & (r15 != 0);
        self.status_reg.s = self.status_reg.n ^ self.status_reg.v;
        self.status_reg.c = c;
        self.status_reg.z = res == 0;
        self.regs.set_ext(d, res);

        self.pc += 1;
        2
    }

    /// 8. Logical AND (AND Rd, Rr) OK
    #[inline(always)]
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
    #[inline(always)]
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
    #[inline(always)]
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
    #[inline(always)]
    fn op_bclr(&mut self, s: u8) -> usize {
        self.status_reg[s] = false;

        self.pc += 1;
        1
    }

    /// 12. Bit Load from the T Flag in SREG to a Bit in Register (BLD Rd, b) OK
    #[inline(always)]
    fn op_bld(&mut self, d: u8, b: u8) -> usize {
        self.regs[d] &= !(1 << b);
        self.regs[d] |= (self.status_reg.t as u8) << b;

        self.pc += 1;
        1
    }

    // Auxiliary function to branch if a boolean is true
    #[inline(always)]
    fn aux_op_branch_if(&mut self, k: i8, test: bool) -> usize {
        if test {
            // let (pc, _) = (self.pc as i16).overflowing_add(k as i16 + 1);
            self.pc = ((self.pc + 1) as i16 + k as i16) as u16;
            self.branch = true;
            2
        } else {
            self.pc += 1;
            1
        }
    }
    /// 13. Branch if Bit in SREG is Cleared (BRBC s, k) OK
    #[inline(always)]
    fn op_brbc(&mut self, s: u8, k: i8) -> usize {
        self.aux_op_branch_if(k, !self.status_reg[s])
    }
    /// 14. Branch if Bit in SREG is Set (BRBS s, k) OK
    #[inline(always)]
    fn op_brbs(&mut self, s: u8, k: i8) -> usize {
        self.aux_op_branch_if(k, self.status_reg[s])
    }
    // 15. Branch if Carry Cleared (BRCC k) OK -> BRBC C
    // 16. Branch if Carry Set (BRCS k) OK -> BRBS C

    /// 17. Break (BREAK) OK
    #[inline(always)]
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
    #[inline(always)]
    fn op_bset(&mut self, s: u8) -> usize {
        self.status_reg[s] = true;

        self.pc += 1;
        1
    }

    /// 35. Bit Store from Bit in Register to T Flag in SREG (BST Rr, b) OK
    #[inline(always)]
    fn op_bst(&mut self, d: u8, b: u8) -> usize {
        self.status_reg.t = (self.regs[d] & (1 << b)) != 0;

        self.pc += 1;
        1
    }

    /// 36. Long Call to a Subroutine (CALL k) OK
    #[inline(always)]
    fn op_call(&mut self, k: u32) -> usize {
        self.push_u16(self.pc + 2);
        self.pc = k as u16;
        self.branch = true;
        4
    }

    /// 37. Clear Bit in I/O Register (CBI A, b) OK
    #[inline(always)]
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
    #[inline(always)]
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
    #[inline(always)]
    fn aux_op_cp_flags(&mut self, a: u8, b: u8, c: bool, res: u8) {
        let (r7, rr7, rd7) = (res & 1 << 7, b & 1 << 7, a & 1 << 7);
        self.status_reg.n = r7 != 0;
        self.status_reg.v = (rr7 != rd7) && (rr7 == r7);
        self.status_reg.s = self.status_reg.n ^ self.status_reg.v;
        self.status_reg.h = ((b & 0x0f) + c as u8) > (a & 0x0f);
    }

    /// 49. Compare (CP Rd, Rr) OK
    #[inline(always)]
    fn op_cp(&mut self, d: u8, r: u8) -> usize {
        let (res, c0) = self.regs[d].overflowing_sub(self.regs[r]);

        self.aux_op_cp_flags(self.regs[d], self.regs[r], false, res);
        self.status_reg.z = res == 0;
        self.status_reg.c = c0;

        self.pc += 1;
        1
    }

    /// 50. Compare with Carry (CPC Rd, Rr) OK
    #[inline(always)]
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
    #[inline(always)]
    fn op_cpi(&mut self, d: u8, k: u8) -> usize {
        let (res, c0) = self.regs[d].overflowing_sub(k);

        self.aux_op_cp_flags(self.regs[d], k, false, res);
        self.status_reg.z = res == 0;
        self.status_reg.c = c0;

        self.pc += 1;
        1
    }

    /// 52. Compare Skip if Equal (CPSE Rd, Rr) OK
    #[inline(always)]
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
    #[inline(always)]
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
    #[inline(always)]
    fn op_eicall(&mut self) -> usize {
        warn!("EICALL unimplemented.  Maybe not supported by ATmega32u4.");
        unimplemented!();
    }

    /// 56. Extended Indirect Jump (EIJMP) OK
    #[inline(always)]
    fn op_eijmp(&mut self) -> usize {
        warn!("EIJMP unimplemented.  Maybe not supported by ATmega32u4.");
        unimplemented!();
    }

    /// 57. Extended Load Program Memory (ELPM Z{+}) OK
    #[inline(always)]
    fn op_elpm(&mut self) -> usize {
        warn!("ELPM unimplemented.  Maybe not supported by ATmega32u4.");
        unimplemented!();
    }

    /// 58. Exclusive OR (EOR, Rd, Rr) OK
    #[inline(always)]
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
    #[inline(always)]
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
    #[inline(always)]
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
    #[inline(always)]
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
    #[inline(always)]
    fn op_icall(&mut self) -> usize {
        self.push_u16(self.pc + 1);
        self.pc = self.regs.z();
        self.branch = true;
        3
    }

    /// 63. Indirect Jump (IJMP) OK
    #[inline(always)]
    fn op_ijmp(&mut self) -> usize {
        self.pc = self.regs.z();
        self.branch = true;
        2
    }

    /// 64. Load an I/O Location to Register (IN Rd, a) OK
    #[inline(always)]
    fn op_in(&mut self, d: u8, a: u8) -> usize {
        self.regs[d] = self.data_load(IOSPACE_ADDR + a as u16);

        self.pc += 1;
        1
    }

    /// 65. Increment (INC Rd) OK
    #[inline(always)]
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
    #[inline(always)]
    fn op_jmp(&mut self, k: u32) -> usize {
        self.pc = k as u16;
        self.branch = true;
        3
    }

    // 67. Load and Clear (LAC) (NOT APPLICABLE)
    // 68. Load and Set (LAS) (NOT APPLICABLE)
    // 69. Load and Toggle (LAT) (NOT APPLICABLE)

    ///  70, 71, 72. Load Indirect from Data Space to Register using Index {X, Y, Z} (LD, {-}{X,Y,Z}{+}{q}) OK
    #[inline(always)]
    fn op_ld<const LD_ST_INDEX: u8>(&mut self, d: u8, ext: LdStExt) -> usize {
        self.pc += 1;
        let mut addr = self.regs.ext(LD_ST_INDEX);

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
        self.regs.set_ext(LD_ST_INDEX, addr);

        cycles
    }

    /// 73. Load Immediate (LDI Rd, K) OK
    #[inline(always)]
    fn op_ldi(&mut self, d: u8, k: u8) -> usize {
        self.regs[d] = k;

        self.pc += 1;
        1
    }

    /// 74. Load Direct from Data Space (LDS Rd, k) OK
    #[inline(always)]
    fn op_lds(&mut self, d: u8, k: u16) -> usize {
        self.regs[d] = self.data_load(k);
        self.pc += 2;
        2
    }
    #[inline(always)]
    fn op_lds_sram(&mut self, d: u8, k: u16) -> usize {
        self.regs[d] = vec_get!(self.sram, (k - SRAM_ADDR) as usize);
        self.pc += 2;
        2
    }

    // 75. Load Direct from Data Space (LDS Rd, k ; 16bit) (NOT APPLICABLE)
    /// 76. Load Program Memory (LPM Rd, Z) OK
    #[inline(always)]
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
    #[inline(always)]
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
    #[inline(always)]
    fn op_mov(&mut self, d: u8, r: u8) -> usize {
        self.regs[d] = self.regs[r];

        self.pc += 1;
        1
    }

    /// 80. Copy Register Word (MOVW Rd, Rr) OK
    #[inline(always)]
    fn op_movw(&mut self, d: u8, r: u8) -> usize {
        self.regs[d] = self.regs[r];
        self.regs[d + 1] = self.regs[r + 1];

        self.pc += 1;
        1
    }
    /// 81. Multiply Unsiged (MUL Rd, Rr) OK
    #[inline(always)]
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
    #[inline(always)]
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
    #[inline(always)]
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
    #[inline(always)]
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
    #[inline(always)]
    fn op_nop(&mut self) -> usize {
        self.pc += 1;
        1
    }

    /// 86. Logical OR (OR Rd, Rr) OK
    #[inline(always)]
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
    #[inline(always)]
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
    #[inline(always)]
    fn op_out(&mut self, a: u8, r: u8) -> usize {
        self.data_store(IOSPACE_ADDR + a as u16, self.regs[r]);

        self.pc += 1;
        1
    }

    /// 89. Pop Register from Stack (POP Rd) OK
    #[inline(always)]
    fn op_pop(&mut self, d: u8) -> usize {
        self.sp += 1;
        self.regs[d] = self.sram[(self.sp - SRAM_ADDR) as usize];

        self.pc += 1;
        2
    }

    /// 90. Push Register on Stack (PUSH Rr) OK
    #[inline(always)]
    fn op_push(&mut self, r: u8) -> usize {
        self.sram[(self.sp - SRAM_ADDR) as usize] = self.regs[r];
        self.sp -= 1;

        self.pc += 1;
        2
    }

    /// 91. Relative Call to Subroutione (RCALL k) OK
    #[inline(always)]
    fn op_rcall(&mut self, k: i16) -> usize {
        let pc = self.pc + 1;
        self.push_u16(pc);
        // let (pc, _) = (self.pc as i16).overflowing_add(k);
        // let (pc, _) = pc.overflowing_add(1);
        self.pc = (pc as i16 + k) as u16;
        self.branch = true;
        3
    }

    /// 92. Return from Subroutine (RET) OK
    #[inline(always)]
    fn op_ret(&mut self) -> usize {
        let pc = self.pop_u16();
        self.pc = pc;
        self.branch = true;
        4
    }

    /// 93. Return from Interrupt (RETI) OK
    #[inline(always)]
    fn op_reti(&mut self) -> usize {
        let pc = self.pop_u16();
        self.status_reg.i = true;
        self.pc = pc;
        self.branch = true;
        4
    }

    /// 94. Relative Jump (RJMP k) OK
    #[inline(always)]
    fn op_rjmp(&mut self, k: i16) -> usize {
        // let (pc, _) = (self.pc as i16).overflowing_add(k);
        // let (pc, _) = pc.overflowing_add(1);
        let pc = (self.pc + 1) as i16 + k;
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
    #[inline(always)]
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
    #[inline(always)]
    fn aux_op_sub_flags(&mut self, a: u8, b: u8, c: bool, res: u8) {
        let (r7, rr7, rd7) = (res & 1 << 7, b & 1 << 7, a & 1 << 7);
        self.status_reg.n = r7 != 0;
        self.status_reg.v = (rr7 != rd7) && (rr7 == r7);
        self.status_reg.s = self.status_reg.n ^ self.status_reg.v;
        self.status_reg.h = ((a & 0x0f) as i8) < ((b & 0x0f) as i8) + c as i8;
    }

    /// 97. Subtract with Carry (SBC Rd, Rr) OK
    #[inline(always)]
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
    #[inline(always)]
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
    #[inline(always)]
    fn op_sbi(&mut self, a: u8, b: u8) -> usize {
        let v = self.data_load(IOSPACE_ADDR + a as u16);
        self.data_store(IOSPACE_ADDR + a as u16, v | (1 << b));
        self.pc += 1;
        2
    }

    /// 100. Skip if Bit in I/O Register is Cleared (SBIC P, b) OK
    #[inline(always)]
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
    #[inline(always)]
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
    #[inline(always)]
    fn op_sbiw(&mut self, d: u8, k: u8) -> usize {
        let ext = self.regs.ext(d);
        let (res, c) = ext.overflowing_sub(k as u16);
        let (r15, rdh7) = (res & 1 << 15, ext & 1 << 15);
        self.status_reg.n = r15 != 0;
        self.status_reg.v = (rdh7 != 0) & (r15 == 0);
        self.status_reg.s = self.status_reg.n ^ self.status_reg.v;
        self.status_reg.c = c;
        self.status_reg.z = res == 0;
        self.regs.set_ext(d, res);

        self.pc += 1;
        2
    }

    // 103. Set Bits in Register (SBR Rd, K) OK -> ORI Rd, K

    /// 104. Skip if Bit in Register is Cleared (SBRC Rr, b) OK
    #[inline(always)]
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
    #[inline(always)]
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
    #[inline(always)]
    fn op_sleep(&mut self) -> usize {
        debug!("Entering sleep");
        self.pc += 1;
        self.sleep_set();
        1
    }
    /// 116. Store Program Memory (SPM) OK
    #[inline(always)]
    fn op_spm(&mut self) -> usize {
        warn!("SPM unimplemented.");
        unimplemented!();
    }
    /// 117. Store Program Memory (SPM #2)
    #[inline(always)]
    fn op_spm2(&mut self) -> usize {
        warn!("SPM #2 unimplemented.");
        unimplemented!();
    }
    /// 118, 119, 120. Store Indirect from Register to Data Space using Index {X, Y, Z} (ST {-}{X,Y,Z}{+}{q}, Rr) OK
    #[inline(always)]
    fn op_st<const LD_ST_INDEX: u8>(&mut self, r: u8, ext: LdStExt) -> usize {
        self.pc += 1;
        let mut addr = self.regs.ext(LD_ST_INDEX);

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
        self.regs.set_ext(LD_ST_INDEX, addr);

        cycles
    }

    /// 121. Store Direct to Data Space (STS k, Rr) OK
    #[inline(always)]
    fn op_sts(&mut self, k: u16, r: u8) -> usize {
        self.data_store(k % (SRAM_ADDR + SRAM_SIZE), self.regs[r]);
        self.pc += 2;
        2
    }

    // 122. Store Direct to Data Space (STS k, Rr ; 16bit) (NOT APPLICABLE)

    /// 123. Subtract without Carry (SUB Rd, Rr) OK
    #[inline(always)]
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
    #[inline(always)]
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
    #[inline(always)]
    fn op_swap(&mut self, d: u8) -> usize {
        let (hi, lo) = ((self.regs[d] & 0xf0) >> 4, self.regs[d] & 0x0f);
        self.regs[d] = (lo << 4) | hi;

        self.pc += 1;
        1
    }

    // 126. Test for Zero or Minus (TST Rd) OK -> AND Rd, Rd
    /// 127. Watchdog Reset (WDR) OK
    #[inline(always)]
    fn op_wdr(&mut self) -> usize {
        warn!("WDR unimplemented.");
        unimplemented!();
        // self.pc += 1;
        // 1
    }
    // 128. Exchange (XCH) (NOT APPLICABLE)

    fn closure_op<'a>(&self, op: Op) -> Box<dyn FnMut(&mut Self) -> usize> {
        match op {
            Op::Adc { d, r } => Box::new(move |s: &mut Self| s.op_adc(d, r)),
            _ => unimplemented!(),
        }
    }

    #[allow(unused_variables)]
    fn exec_op(&mut self, op: Op) -> usize {
        #[cfg(feature = "stats")]
        {
            let op_type = OpType::new_from_op(op).to_usize().unwrap();
            let op_type_prev = OpType::new_from_op(self.stats.op_prev).to_usize().unwrap();
            let ops_len = OpType::Undefined.to_usize().unwrap();
            self.stats.ops[op_type] += 1;
            self.stats.ops2[op_type_prev + op_type * ops_len] += 1;
            self.stats.op_prev = op;
        }
        match op {
            Op::Adc { d, r } => self.op_adc(d, r),
            // Op::Adc { d, r } => self.op_lut_adc(d, r),
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
            Op::Call { k } => self.op_call(k as u32),
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
            Op::Jmp { k } => self.op_jmp(k as u32),
            // Op::Ld { d, idx, ext } => self.op_ld(d, idx, ext),
            Op::LdX { d, ext } => self.op_ld::<{ LD_ST_INDEX_X }>(d, ext),
            Op::LdY { d, ext } => self.op_ld::<{ LD_ST_INDEX_Y }>(d, ext),
            Op::LdZ { d, ext } => self.op_ld::<{ LD_ST_INDEX_Z }>(d, ext),
            Op::Ldi { d, k } => self.op_ldi(d, k),
            Op::Lds { d, k } => self.op_lds(d, k),
            Op::LdsSRAM { d, k } => self.op_lds_sram(d, k),
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
            // Op::St { r, idx, ext } => self.op_st(r, idx, ext),
            Op::StX { r, ext } => self.op_st::<{ LD_ST_INDEX_X }>(r, ext),
            Op::StY { r, ext } => self.op_st::<{ LD_ST_INDEX_Y }>(r, ext),
            Op::StZ { r, ext } => self.op_st::<{ LD_ST_INDEX_Z }>(r, ext),
            Op::Sts { k, r } => self.op_sts(k, r),
            Op::Sub { d, r } => self.op_sub(d, r),
            Op::Subi { d, k } => self.op_subi(d, k),
            Op::Swap { d } => self.op_swap(d),
            Op::Wdr => self.op_wdr(),
            Op::Zzz => 4,
            Op::AddAdc { d, r } => {
                let op1 = self.op1();
                let [_, d1, r1, _] = unsafe { transmute::<_, [u8; 4]>(op1) };
                self.op2_add_adc(d, r, d1, r1)
            }
            Op::AdcAdc { d, r } => {
                let op1 = self.op1();
                let [_, d1, r1, _] = unsafe { transmute::<_, [u8; 4]>(op1) };
                self.op2_adc_adc(d, r, d1, r1)
            }
            Op::SubiSbci { d, k } => {
                let op1 = self.op1();
                let [_, d1, k1, _] = unsafe { transmute::<_, [u8; 4]>(op1) };
                self.op2_subi_sbci(d, k, d1, k1)
            }
            Op::DecBrbcZ { d } => {
                let op1 = self.op1();
                let [_, _, k, _] = unsafe { transmute::<_, [u8; 4]>(op1) };
                self.op2_dec_brbc_z(d, k as i8)
            }
            Op::LdsLds { d, k } => {
                let op1 = self.op1();
                let (_, d1, k1) = unsafe { transmute::<_, (u8, u8, u16)>(op1) };
                self.op2_lds_lds(d, k, d1, k1)
            }
            Op::CpCpc { d, r } => {
                let op1 = self.op1();
                let [_, d1, r1, _] = unsafe { transmute::<_, [u8; 4]>(op1) };
                self.op2_cp_cpc(d, r, d1, r1)
            }
            Op::CpiBrbc { d, k } => {
                let op1 = self.op1();
                let [_, s, k1, _] = unsafe { transmute::<_, [u8; 4]>(op1) };
                self.op2_cpi_brbc(d, k, s, k1 as i8)
            }
            Op::CpcBrbs { d, r } => {
                let op1 = self.op1();
                let [_, s, k, _] = unsafe { transmute::<_, [u8; 4]>(op1) };
                self.op2_cpc_brbs(d, r, s, k as i8)
            }
            Op::CpcBrbc { d, r } => {
                let op1 = self.op1();
                let [_, s, k, _] = unsafe { transmute::<_, [u8; 4]>(op1) };
                self.op2_cpc_brbc(d, r, s, k as i8)
            }
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
mod test_ops;

#[cfg(feature = "test_vectors")]
#[cfg(test)]
mod test_vectors;
