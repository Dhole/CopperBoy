use log::warn;

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
            i: ((v & 1) << 7) != 0,
            t: ((v & 1) << 6) != 0,
            h: ((v & 1) << 5) != 0,
            s: ((v & 1) << 4) != 0,
            v: ((v & 1) << 3) != 0,
            n: ((v & 1) << 2) != 0,
            z: ((v & 1) << 1) != 0,
            c: ((v & 1) << 0) != 0,
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
struct GeneralRegisters {
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
    fn x(&self) -> u16 {
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
    fn z(&self) -> u16 {
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

pub const SRAM_SIZE: u16 = 0x0a00;
pub const SRAM_ADDR: u16 = 0x0100;
pub const IOSPACE_SIZE: u16 = 0x0040;
pub const IOSPACE_ADDR: u16 = 0x0020;
pub const DATA_SIZE: u16 = 0x0b00;
pub const PROGRAM_SIZE: u16 = 0x8000;

struct Core {
    /// Status Register
    status_reg: StatusRegister,
    /// General Purpose Register File
    regs: GeneralRegisters,
    /// IO Registers
    io_space: [u8; IOSPACE_SIZE as usize],
    /// Program Counter
    pc: u16,
    /// Stack Pointer
    sp: u16,
    /// SRAM
    sram: [u8; SRAM_SIZE as usize],
    /// Program Memory
    program: [u8; PROGRAM_SIZE as usize],
    /// Set if the previos instruction branched.  This flag is used to know if the instruction
    /// ahead must be fetched.
    branch: bool,
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

impl Core {
    fn new() -> Self {
        Self {
            regs: GeneralRegisters::new(),
            io_space: [0; IOSPACE_SIZE as usize],
            status_reg: StatusRegister::new(),
            pc: 0,
            sram: [0; SRAM_SIZE as usize],
            program: [0; PROGRAM_SIZE as usize],
            sp: SRAM_ADDR + SRAM_SIZE - 1,
            branch: false,
        }
    }

    /// Instruction used to populate the Program Memory Space.  Simulates flashing the flash
    /// memory.
    fn flash(&mut self, addr: u16, v: u8) {
        self.program[addr as usize] = v;
    }

    fn data_load(&self, addr: u16) -> u8 {
        if addr >= SRAM_ADDR {
            self.sram[(addr - SRAM_ADDR) as usize]
        } else {
            match addr {
                io_regs::SPL => get_lo(self.sp),
                io_regs::SPH => get_hi(self.sp),
                io_regs::SREG => self.status_reg.as_u8(),
                _ => {
                    warn!(
                        "I/O Registers / Extended I/O Space unimplemented load at 0x{:04x}",
                        addr
                    );
                    unimplemented!()
                }
            }
        }
    }

    fn data_load_u16(&self, addr: u16) -> u16 {
        u16::from_le_bytes([self.data_load(addr), self.data_load(addr + 1)])
    }

    fn data_store(&mut self, addr: u16, v: u8) {
        if addr >= SRAM_ADDR {
            self.sram[(addr - SRAM_ADDR) as usize] = v;
        } else {
            match addr {
                io_regs::SPL => set_lo(&mut self.sp, v),
                io_regs::SPH => set_hi(&mut self.sp, v),
                io_regs::SREG => self.status_reg = StatusRegister::from_u8(v),
                _ => {
                    warn!(
                        "I/O Registers / Extended I/O Space unimplemented store at 0x{:04x}",
                        addr
                    );
                    unimplemented!()
                }
            }
        }
    }

    fn data_store_u16(&mut self, addr: u16, v: u16) {
        let bytes = v.to_le_bytes();
        self.data_store(addr, bytes[0]);
        self.data_store(addr + 1, bytes[1]);
    }

    fn push_u16(&mut self, v: u16) {
        let bytes = v.to_le_bytes();
        self.sram[(self.sp - SRAM_ADDR - 1) as usize] = bytes[0];
        self.sram[(self.sp - SRAM_ADDR) as usize] = bytes[1];
        self.sp -= 2;
    }

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
        self.status_reg.c = c0 || c1;
        self.aux_op_add_flags(self.regs[d], self.regs[r], self.status_reg.c, res);
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
    // 32. Branch if Overflow Cleared (BRVC k) OK-> BRBC V
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
    fn op_call(&mut self, k: u16) -> usize {
        self.push_u16(self.pc + 2);
        self.pc = k;
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

    /// 43. Clear Register (CLR Rd) OK
    fn op_clr(&mut self, d: u8) -> usize {
        self.regs[d] = 0;
        self.status_reg.s = false;
        self.status_reg.v = false;
        self.status_reg.n = false;
        self.status_reg.z = true;

        self.pc += 1;
        1
    }

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
    fn op_cpse(&mut self, d: u8, r: u8) -> usize {
        if self.regs[d] == self.regs[r] {
            // FIXME: If next instruction is two words, 3 instead of 2.
            self.pc += 2;
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
        3
    }

    /// 63. Indirect Jump (IJMP) OK
    fn op_ijmp(&mut self) -> usize {
        self.pc = self.regs.z();
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
        3
    }

    // 67. Load and Clear (LAC) (NOT APPLICABLE)
    // 68. Load and Set (LAS) (NOT APPLICABLE)
    // 69. Load and Toggle (LAT) (NOT APPLICABLE)
    // TODO: 70. Load Indirect from Data Space to Register using Index X (LD, {-}X{+}) OK
    // TODO: 71. Load Indirect from Data Space to Register using Index Y (LD, {-}Y{+}) OK
    // TODO: 72. Load Indirect from Data Space to Register using Index Z (LD, {-}Z{+}) OK
    // TODO: ??. Load Indirect with Displacement (LDD {Y,Z}+q) OK

    /// 73. Load Immediate (LDI Rd, K) OK
    fn op_ldi(&mut self, d: u8, k: u8) -> usize {
        self.regs[d] = k;

        self.pc += 1;
        1
    }

    // TODO: 74. Load Direct from Data Space (LDS Rd, k) OK
    // 75. Load Direct from Data Space (LDS Rd, k ; 16bit) (NOT APPLICABLE)
    // TODO: 76. Load Program Memory (LPM Rd, Z) OK

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
        let (r7, r3, rd3) = (res & 1 << 7, res & 1 << 3, self.regs[d] & 1 << 3);
        self.status_reg.n = r7 != 0;
        self.status_reg.v = res == 0x80;
        self.status_reg.s = self.status_reg.n ^ self.status_reg.v;
        self.status_reg.h = r3 != 0 && rd3 == 0;
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
        self.pc = pc as u16;
        3
    }

    /// 92. Return from Subroutine (RET) OK
    fn op_ret(&mut self) -> usize {
        let pc = self.pop_u16();
        self.pc = pc;
        4
    }

    /// 93. Return from Interrupt (RETI) OK
    fn op_reti(&mut self) -> usize {
        let pc = self.pop_u16();
        self.status_reg.i = true;
        self.pc = pc;
        4
    }

    /// 94. Relative Jump (RJMP k) OK
    fn op_rjmp(&mut self, k: i16) -> usize {
        let (pc, _) = (self.pc as i16).overflowing_add(k);
        self.pc = pc as u16;
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
        self.status_reg.h = (a & 0x0f) < (b & 0x0f) + c as u8;
    }

    /// 97. Subtract with Carry (SBC Rd, Rr) OK
    fn op_sbc(&mut self, d: u8, r: u8) -> usize {
        let (res, c0) = self.regs[d].overflowing_sub(self.regs[r]);
        let (res, c1) = res.overflowing_sub(self.status_reg.c as u8);
        self.status_reg.c = c0 || c1;
        self.aux_op_sub_flags(self.regs[d], self.regs[r], self.status_reg.c, res);
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
        self.status_reg.c = c0 || c1;
        self.aux_op_sub_flags(self.regs[d], k, self.status_reg.c, res);
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
    fn op_sbic(&mut self, a: u8, b: u8) -> usize {
        let v = self.data_load(IOSPACE_ADDR + a as u16);
        if (v & (1 << b)) == 0 {
            // FIXME: If next instruction is two words, 3 instead of 2.
            self.pc += 2;
            2
        } else {
            self.pc += 1;
            1
        }
    }

    /// 101. Skip if Bit in I/O Register is Set (SBIS P, b) OK
    fn op_sbis(&mut self, a: u8, b: u8) -> usize {
        let v = self.data_load(IOSPACE_ADDR + a as u16);
        if (v & (1 << b)) != 0 {
            // FIXME: If next instruction is two words, 3 instead of 2.
            self.pc += 2;
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
    fn op_sbrc(&mut self, r: u8, b: u8) -> usize {
        if self.regs[r] & (1 << b) == 0 {
            // FIXME: If next instruction is two words, 3 instead of 2.
            self.pc += 2;
            2
        } else {
            self.pc += 1;
            1
        }
    }

    /// 105. Skip if Bit in Register is Set (SBRS Rr, b) OK
    fn op_sbrs(&mut self, r: u8, b: u8) -> usize {
        if self.regs[r] & (1 << b) != 0 {
            // FIXME: If next instruction is two words, 3 instead of 2.
            self.pc += 2;
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

    /// 110. Set all Bits in Register (SER Rd) OK
    fn op_ser(&mut self, d: u8) -> usize {
        self.regs[d] = 0xff;
        self.pc += 1;
        1
    }
    // 111. Set Signed Flag (SES) OK -> BSET S
    // 112. Set T Flag (SET) OK -> BSET T
    // 113. Set Overflow Flag (SEV) OK -> BSET V
    // 114. Set Zero Flag (SEZ) OK -> BSET Z

    /// 115. SLEEP (SLEEP) OK
    fn op_sleep(&mut self) -> usize {
        unimplemented!();
        // self.pc += 1;
        // 1
    }
    // TODO: 116. Store Program Memory (SPM) OK
    // TODO: 117. Store Program Memory (SPM #2)
    // TODO: 118. Store Indirect from Register to Data Space using Index X (ST {-}X{+}, Rr) OK
    // TODO: 119. Store Indirect from Register to Data Space using Index Y (ST {-}Y{+}, Rr) OK
    // TODO: 120. Store Indirect from Register to Data Space using Index Z (ST {-}Z{+}, Rr) OK
    // TODO: ???. Store Indirect with Displacement (STD {Y,Z}+q, Rr) OK
    // TODO: 121. Store Direct to Data Space (STS k, Rr) OK
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
    fn op_wdr(&mut self) {
        unimplemented!();
        // self.pc += 1;
        // 1
    }
    // 128. Exchange (XCH) (NOT APPLICABLE)
}

#[cfg(test)]
mod test;
