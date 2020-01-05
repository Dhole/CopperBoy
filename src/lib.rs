#![allow(dead_code)]

use std::fmt;
use std::ops::{Index, IndexMut};

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

#[derive(PartialEq, Debug)]
struct Memory {
    buf: Vec<u8>,
}

impl Memory {
    fn new(size: u16) -> Self {
        Self {
            buf: vec![0; size as usize],
        }
    }

    fn get_u16(&mut self, a: u16) -> u16 {
        u16::from_le_bytes([self.buf[a as usize], self.buf[(a + 1) as usize]])
    }

    fn set_u16(&mut self, a: u16, v: u16) {
        let bytes = v.to_le_bytes();
        self.buf[a as usize] = bytes[0];
        self.buf[(a + 1) as usize] = bytes[1];
    }
}

impl Index<u16> for Memory {
    type Output = u8;

    fn index(&self, i: u16) -> &u8 {
        &self.buf[i as usize]
    }
}

impl IndexMut<u16> for Memory {
    fn index_mut(&mut self, i: u16) -> &mut u8 {
        &mut self.buf[i as usize]
    }
}

#[derive(Debug)]
struct Core {
    /// Status Register
    status_reg: StatusRegister,
    /// General Purpose Register File
    regs: GeneralRegisters,
    /// IO Registers
    io_regs: Vec<u8>,
    /// Program Counter
    pc: u16,
    /// Stack Pointer
    sp: u16,
    /// Memory
    ram: Memory,
}

const SRAM_SIZE: u16 = 0x0b00;

impl Core {
    fn new() -> Self {
        Self {
            regs: GeneralRegisters::new(),
            io_regs: vec![0; 64],
            status_reg: StatusRegister::new(),
            pc: 0,
            ram: Memory::new(SRAM_SIZE),
            sp: SRAM_SIZE - 1,
        }
    }

    fn push_u16(&mut self, v: u16) {
        self.ram.set_u16(self.sp - 1, v);
        self.sp -= 2;
    }

    fn pop_u16(&mut self) -> u16 {
        self.sp += 2;
        let v = self.ram.get_u16(self.sp - 1);
        v
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
        1
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

    /// 37. Clear Bit in I/O Register (CBI P, b) OK
    fn op_cbi(&mut self, a: u8, d: u8) -> usize {
        self.io_regs[a as usize] &= !(1 << d);
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
        self.status_reg.h = (((b & 0x0f) + c as u8) > (a & 0x0f));
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
        if self.regs[d] != self.regs[r] {
            self.pc += 1;
            1
        } else {
            // FIXME: If next instruction is two words, 3 instead of 2.
            self.pc += 2;
            2
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

    // TODO: 55. Extended Indirect Call to Subroutine (EICALL) OK
    // TODO: 56. Extended Indirect Jump (EIJMP) OK

    // TODO: 57. Extended Load Program Memory (ELPM Z{+}) OK

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

    // TODO: 59. Fractional Multiply Unsiged (FMUL Rd, Rr) OK
    // TODO: 60. Fractional Multiply Signed (FMULS Rd, Rr) OK
    // TODO: 61. Fractional Multiply Signed with Unsigned (FMULSU Rd, Rr) OK
    // TODO: 62. Indirect Call to Subroutine (ICALL) OK
    // TODO: 63. Indirect Jump (IJMP) OK
    // TODO: 64. Load an I/O Location to Register (IN Rd, P) OK

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
    // TODO: 75. Load Direct from Data Space (LDS Rd, k # 16bit) OK
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

    // TODO: 88. Store Register to I/O Location (OUT P, Rr) OK

    /// 89. Pop Register from Stack (POP Rd) OK
    fn op_pop(&mut self, d: u8) -> usize {
        self.sp += 1;
        self.regs[d] = self.ram[self.sp];

        self.pc += 1;
        2
    }

    /// 90. Push Register on Stack (PUSH Rr) OK
    fn op_push(&mut self, r: u8) -> usize {
        self.ram[self.sp] = self.regs[r];
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

    /// 95. Rotate Left through Carry (ROL Rd) OK
    fn op_rol(&mut self, d: u8) -> usize {
        let res = self.regs[d] << 1;
        let res = res | self.status_reg.c as u8;
        let (r7, rd7, rd3) = (res & 1 << 7, self.regs[d] & 1 << 7, self.regs[d] & 1 << 3);
        self.status_reg.h = rd3 != 0;
        self.status_reg.n = r7 != 0;
        self.status_reg.c = rd7 != 0;
        self.status_reg.v = self.status_reg.n ^ self.status_reg.c;
        self.status_reg.s = self.status_reg.n ^ self.status_reg.v;
        self.status_reg.z = res == 0;
        self.regs[d] = res;

        self.pc += 1;
        1
    }

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
        self.status_reg.h = ((a & 0x0f) < (b & 0x0f) + c as u8);
        if res != 0 {
            self.status_reg.z = false;
        }
    }

    /// 97. Subtract with Carry (SBC Rd, Rr) OK
    fn op_sbc(&mut self, d: u8, r: u8) -> usize {
        let (res, c0) = self.regs[d].overflowing_sub(self.regs[r]);
        let (res, c1) = res.overflowing_sub(self.status_reg.c as u8);
        self.status_reg.c = c0 || c1;
        self.aux_op_sub_flags(self.regs[d], self.regs[r], self.status_reg.c, res);
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
        self.regs[d] = res;

        self.pc += 1;
        1
    }

    // TODO: 99. Set Bit in I/O Register (SBI P, b) OK
    // TODO: 100. Skip if Bit in I/O Register is Cleared (SBIC P, b) OK
    // TODO: 101. Skip if Bit in I/O Register is Set (SBIS P, b) OK
    // TODO: 102. Subtract Immedaite from Word (SBIW Rdl, K) OK

    // 103. Set Bits in Register (SBR Rd, K) OK -> ORI Rd, K
    // TODO: 104. Skip if Bit in Register is Cleared (SBRC Rr, b) OK
    // TODO: 105. Skip if Bit in Register is Set (SBRS Rr, b) OK
    // 106. Set Carry Flag (SEC) OK -> BSET C
    // 107. Set Half Carry Flag (SEH) OK -> BSET H
    // 108. Set Global Interrupt Flag (SEI) OK -> BSET I
    // 109. Set Negative Flag (SEN) OK -> BSET N
    // TODO: 110. Set all Bits in Register (SER Rd) OK
    // 111. Set Signed Flag (SES) OK -> BSET S
    // 112. Set T Flag (SET) OK -> BSET T
    // 113. Set Overflow Flag (SEV) OK -> BSET V
    // 114. Set Zero Flag (SEZ) OK -> BSET Z
    // TODO: 115. SLEEP (SLEEP) OK
    // TODO: 116. Store Program Memory (SPM) OK
    // TODO: 117. Store Program Memory (SPM #2)
    // TODO: 118. Store Indirect from Register to Data Space using Index X (ST {-}X{+}, Rr) OK
    // TODO: 119. Store Indirect from Register to Data Space using Index Y (ST {-}Y{+}, Rr) OK
    // TODO: 120. Store Indirect from Register to Data Space using Index Z (ST {-}Z{+}, Rr) OK
    // TODO: ???. Store Indirect with Displacement (STD {Y,Z}+q, Rr) OK
    // TODO: 121. Store Direct to Data Space (STS k, Rr) OK
    // TODO: 122. Store Direct to Data Space (STS k, Rr # 16bit) OK
    // TODO: 123. Subtract without Carry (SUB Rd, Rr) OK
    // TODO: 124. Subtract Immediate (SUBI Rd, K) OK
    // TODO: 125. Swap Nibbles (SWAP Rd) OK
    // TODO: 126. Test for Zero or Minus (TST Rd) OK
    // TODO: 127. Watchdog Reset (WDR) OK
    // 128. Exchange (XCH) (NOT APPLICABLE)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn new_status_reg_true(list: &[char]) -> StatusRegister {
        let mut status_reg = StatusRegister::new();
        for c in list {
            match c {
                'i' => status_reg.i = true,
                't' => status_reg.t = true,
                'h' => status_reg.h = true,
                's' => status_reg.s = true,
                'v' => status_reg.v = true,
                'n' => status_reg.n = true,
                'z' => status_reg.z = true,
                'c' => status_reg.c = true,
                _ => panic!("Bad status register bit"),
            }
        }
        status_reg
    }

    macro_rules! assert_status_reg_true {
        ($status_reg:expr, $list:expr) => {
            assert_eq!(*$status_reg, new_status_reg_true($list));
        };
    }

    #[test]
    fn test_op_add() {
        let mut core = Core::new();

        core.regs[0] = 0x01;
        core.regs[1] = 0x02;
        core.op_add(0, 1);
        assert_eq!(core.pc, 0x01);
        assert_eq!(core.regs[0], 0x03);
        assert_status_reg_true!(&core.status_reg, &[]);

        core.regs[0] = 0xff;
        core.regs[1] = 0x01;
        core.op_add(0, 1);
        assert_eq!(core.regs[0], 0x00);
        assert_status_reg_true!(&core.status_reg, &['z', 'c', 'h']);

        core.regs[0] = 0xe0;
        core.regs[1] = 0x40;
        core.op_add(0, 1);
        assert_eq!(core.regs[0], 0x20);
        assert_status_reg_true!(&core.status_reg, &['c']);

        core.regs[0] = 0x0a;
        core.regs[1] = 0x06;
        core.op_add(0, 1);
        assert_eq!(core.regs[0], 0x10);
        assert_status_reg_true!(&core.status_reg, &['h']);

        core.regs[0] = 0xe0;
        core.regs[1] = 0x11;
        core.op_add(0, 1);
        assert_eq!(core.regs[0], 0xf1);
        assert_status_reg_true!(&core.status_reg, &['s', 'n']);

        core.regs[0] = 0x80;
        core.regs[1] = 0x81;
        core.op_add(0, 1);
        assert_eq!(core.regs[0], 0x01);
        assert_status_reg_true!(&core.status_reg, &['c', 'v', 's']);
    }

    #[test]
    fn test_op_adc() {
        let mut core = Core::new();

        core.status_reg.c = true;
        core.regs[0] = 0x02;
        core.regs[1] = 0x04;
        core.op_adc(0, 1);
        assert_eq!(core.pc, 0x01);
        assert_eq!(core.regs[0], 0x07);
        assert_status_reg_true!(&core.status_reg, &[]);

        core.status_reg.c = true;
        core.regs[0] = 0xfd;
        core.regs[1] = 0x02;
        core.op_adc(0, 1);
        assert_eq!(core.regs[0], 0x00);
        assert_status_reg_true!(&core.status_reg, &['z', 'c', 'h']);

        core.status_reg.c = false;
        core.regs[0] = 0xfd;
        core.regs[1] = 0x02;
        core.op_adc(0, 1);
        assert_eq!(core.regs[0], 0xff);
        assert_status_reg_true!(&core.status_reg, &['s', 'n']);
    }

    #[test]
    fn test_op_adiw() {
        let mut core = Core::new();

        core.regs.set_ext(24, 0x23e5);
        core.op_adiw(24, 0x21);
        assert_eq!(core.pc, 0x01);
        assert_eq!(core.regs.ext(24), 0x2406);
        assert_status_reg_true!(&core.status_reg, &[]);

        core.regs.set_ext(24, 0xefff);
        core.op_adiw(24, 0x01);
        assert_eq!(core.regs.ext(24), 0xf000);
        assert_status_reg_true!(&core.status_reg, &['n', 's']);

        core.regs.set_ext(24, 0xffff);
        core.op_adiw(24, 0x01);
        assert_eq!(core.regs.ext(24), 0x0000);
        assert_status_reg_true!(&core.status_reg, &['c', 'z']);

        core.regs.set_ext(24, 0x7fff);
        core.op_adiw(24, 0x01);
        assert_eq!(core.regs.ext(24), 0x8000);
        assert_status_reg_true!(&core.status_reg, &['v', 'n']);
    }

    #[test]
    fn test_op_and() {
        let mut core = Core::new();

        core.regs[0] = 0xa7;
        core.regs[1] = 0x79;
        core.op_and(0, 1);
        assert_eq!(core.pc, 0x01);
        assert_eq!(core.regs[0], 0x21);
        assert_status_reg_true!(&core.status_reg, &[]);

        core.regs[0] = 0xf0;
        core.regs[1] = 0x0f;
        core.op_and(0, 1);
        assert_eq!(core.regs[0], 0x00);
        assert_status_reg_true!(&core.status_reg, &['z']);

        core.regs[0] = 0xa0;
        core.regs[1] = 0x91;
        core.op_and(0, 1);
        assert_eq!(core.regs[0], 0x80);
        assert_status_reg_true!(&core.status_reg, &['n', 's']);
    }

    #[test]
    fn test_op_andi() {
        let mut core = Core::new();

        core.regs[16] = 0xa7;
        core.op_andi(16, 0x79);
        assert_eq!(core.pc, 0x01);
        assert_eq!(core.regs[16], 0x21);
        assert_status_reg_true!(&core.status_reg, &[]);

        core.regs[16] = 0xf0;
        core.op_andi(16, 0x0f);
        assert_eq!(core.regs[16], 0x00);
        assert_status_reg_true!(&core.status_reg, &['z']);

        core.regs[16] = 0xa0;
        core.op_andi(16, 0x91);
        assert_eq!(core.regs[16], 0x80);
        assert_status_reg_true!(&core.status_reg, &['n', 's']);
    }

    #[test]
    fn test_op_asr() {
        let mut core = Core::new();

        core.regs[0] = 0x0e;
        core.op_asr(0);
        assert_eq!(core.pc, 0x01);
        assert_eq!(core.regs[0], 0x07);
        assert_status_reg_true!(&core.status_reg, &[]);

        core.regs[0] = 0x88;
        core.op_asr(0);
        assert_eq!(core.regs[0], 0xc4);
        assert_status_reg_true!(&core.status_reg, &['n', 'v']);

        core.regs[0] = 0x01;
        core.op_asr(0);
        assert_eq!(core.regs[0], 0x00);
        assert_status_reg_true!(&core.status_reg, &['z', 'c', 'v', 's']);
    }

    #[test]
    fn test_op_bclr() {
        let mut core = Core::new();

        core.status_reg.h = true;
        core.op_bclr(5);
        assert_eq!(core.pc, 0x01);
        assert_status_reg_true!(&core.status_reg, &[]);
    }

    #[test]
    fn test_op_bld() {
        let mut core = Core::new();

        core.status_reg.t = true;
        core.regs[0] = 0x00;
        core.op_bld(0, 5);
        assert_eq!(core.pc, 0x01);
        assert_eq!(core.regs[0], 0x20);

        core.status_reg.t = false;
        core.regs[0] = 0xff;
        core.op_bld(0, 5);
        assert_eq!(core.regs[0], 0xdf);
    }

    #[test]
    fn test_op_brbc() {
        let mut core = Core::new();

        core.status_reg.z = true;
        assert_eq!(core.op_brbc(1, 0x10), 1);
        assert_eq!(core.pc, 0x01);

        core.status_reg.z = false;
        assert_eq!(core.op_brbc(1, 0x10), 2);
        assert_eq!(core.pc, 0x12);

        core.status_reg.h = false;
        assert_eq!(core.op_brbc(5, -0x05), 2);
        assert_eq!(core.pc, 0x0e);
    }

    #[test]
    fn test_op_brbs() {
        let mut core = Core::new();

        core.status_reg.z = false;
        assert_eq!(core.op_brbs(1, 0x10), 1);
        assert_eq!(core.pc, 0x01);

        core.status_reg.z = true;
        assert_eq!(core.op_brbs(1, 0x10), 2);
        assert_eq!(core.pc, 0x12);

        core.status_reg.h = true;
        assert_eq!(core.op_brbs(5, -0x05), 2);
        assert_eq!(core.pc, 0x0e);
    }

    #[test]
    fn test_op_bset() {
        let mut core = Core::new();

        core.op_bset(5);
        assert_eq!(core.pc, 0x01);
        assert_status_reg_true!(&core.status_reg, &['h']);
    }

    #[test]
    fn test_op_bst() {
        let mut core = Core::new();

        core.regs[0] = 0x20;
        core.op_bst(0, 5);
        assert_eq!(core.pc, 0x01);
        assert_status_reg_true!(&core.status_reg, &['t']);

        core.regs[0] = 0xdf;
        core.op_bst(0, 5);
        assert_status_reg_true!(&core.status_reg, &[]);
    }

    #[test]
    fn test_op_call() {
        let mut core = Core::new();

        core.pc = 0x01;
        core.op_call(0x0123);
        assert_eq!(core.pc, 0x0123);
        assert_eq!(core.sp, (SRAM_SIZE - 1) - 2);
        assert_eq!(core.ram.get_u16((SRAM_SIZE - 1) - 1), 0x03);
    }

    #[test]
    fn test_op_cbi() {
        let mut core = Core::new();

        core.io_regs[3] = 0xff;
        core.op_cbi(3, 5);
        assert_eq!(core.pc, 0x01);
        assert_eq!(core.io_regs[3], 0b11011111);
    }

    #[test]
    fn test_op_clr() {
        let mut core = Core::new();

        core.regs[0] = 0xab;
        core.op_clr(0);
        assert_eq!(core.pc, 0x01);
        assert_eq!(core.regs[0], 0x00);
        assert_status_reg_true!(&core.status_reg, &['z']);
    }

    #[test]
    fn test_op_com() {
        let mut core = Core::new();

        core.regs[0] = 0x12;
        core.op_com(0);
        assert_eq!(core.pc, 0x01);
        assert_eq!(core.regs[0], 0xed);
        assert_status_reg_true!(&core.status_reg, &['s', 'n', 'c']);

        core.regs[0] = 0xff;
        core.op_com(0);
        assert_eq!(core.regs[0], 0x00);
        assert_status_reg_true!(&core.status_reg, &['z', 'c']);
    }

    #[test]
    fn test_op_cp() {
        let mut core = Core::new();

        core.regs[0] = 0xff;
        core.regs[1] = 0x01;
        core.op_cp(0, 1);
        assert_eq!(core.pc, 0x01);
        assert_status_reg_true!(&core.status_reg, &['n', 's']);

        core.regs[0] = 0x54;
        core.regs[1] = 0x54;
        core.op_cp(0, 1);
        assert_status_reg_true!(&core.status_reg, &['z']);

        core.regs[0] = 0x11;
        core.regs[1] = 0x20;
        core.op_cp(0, 1);
        assert_status_reg_true!(&core.status_reg, &['c', 's', 'n']);

        core.regs[0] = 0x11;
        core.regs[1] = 0x22;
        core.op_cp(0, 1);
        assert_status_reg_true!(&core.status_reg, &['c', 'h', 's', 'n']);

        core.regs[0] = 0x80;
        core.regs[1] = 0x70;
        core.op_cp(0, 1);
        assert_status_reg_true!(&core.status_reg, &['v', 's']);
    }

    #[test]
    fn test_op_cpc() {
        let mut core = Core::new();

        core.regs[0] = 0x54;
        core.regs[1] = 0x53;
        core.status_reg.c = true;
        core.status_reg.z = true;
        core.op_cpc(0, 1);
        assert_status_reg_true!(&core.status_reg, &['z']);

        core.regs[0] = 0x11;
        core.regs[1] = 0x20;
        core.status_reg.c = true;
        core.op_cpc(0, 1);
        assert_status_reg_true!(&core.status_reg, &['c', 's', 'n']);

        core.regs[0] = 0x11;
        core.regs[1] = 0x21;
        core.status_reg.c = true;
        core.op_cpc(0, 1);
        assert_status_reg_true!(&core.status_reg, &['c', 'h', 's', 'n']);

        core.regs[0] = 0x81;
        core.regs[1] = 0x70;
        core.status_reg.c = true;
        core.op_cpc(0, 1);
        assert_status_reg_true!(&core.status_reg, &['v', 's']);
    }

    #[test]
    fn test_op_cpi() {
        let mut core = Core::new();

        core.regs[0] = 0xff;
        core.op_cpi(0, 0x01);
        assert_eq!(core.pc, 0x01);
        assert_status_reg_true!(&core.status_reg, &['n', 's']);

        core.regs[0] = 0x54;
        core.op_cpi(0, 0x54);
        assert_status_reg_true!(&core.status_reg, &['z']);

        core.regs[0] = 0x11;
        core.op_cpi(0, 0x20);
        assert_status_reg_true!(&core.status_reg, &['c', 's', 'n']);

        core.regs[0] = 0x11;
        core.op_cpi(0, 0x22);
        assert_status_reg_true!(&core.status_reg, &['c', 'h', 's', 'n']);

        core.regs[0] = 0x80;
        core.op_cpi(0, 0x70);
        assert_status_reg_true!(&core.status_reg, &['v', 's']);
    }

    #[test]
    fn test_op_cpse() {
        let mut core = Core::new();

        core.regs[0] = 0xaa;
        core.regs[1] = 0xbb;
        core.op_cpse(0, 1);
        assert_eq!(core.pc, 0x01);

        core.regs[0] = 0xaa;
        core.regs[1] = 0xaa;
        core.op_cpse(0, 1);
        assert_eq!(core.pc, 0x03);
    }

    #[test]
    fn test_op_dec() {
        let mut core = Core::new();

        core.regs[0] = 0x21;
        core.op_dec(0);
        assert_eq!(core.pc, 0x01);
        assert_eq!(core.regs[0], 0x20);
        assert_status_reg_true!(&core.status_reg, &[]);

        core.regs[0] = 0x00;
        core.op_dec(0);
        assert_eq!(core.regs[0], 0xff);
        assert_status_reg_true!(&core.status_reg, &['s', 'n']);

        core.regs[0] = 0x80;
        core.op_dec(0);
        assert_eq!(core.regs[0], 0x7f);
        assert_status_reg_true!(&core.status_reg, &['s', 'v']);

        core.regs[0] = 0x01;
        core.op_dec(0);
        assert_eq!(core.regs[0], 0x00);
        assert_status_reg_true!(&core.status_reg, &['z']);
    }

    #[test]
    fn test_op_eor() {
        let mut core = Core::new();

        core.regs[0] = 0x7a;
        core.regs[1] = 0xb6;
        core.op_eor(0, 1);
        assert_eq!(core.pc, 0x01);
        assert_eq!(core.regs[0], 0xcc);
        assert_status_reg_true!(&core.status_reg, &['n', 's']);

        core.regs[0] = 0x42;
        core.regs[1] = 0x42;
        core.op_eor(0, 1);
        assert_eq!(core.regs[0], 0x00);
        assert_status_reg_true!(&core.status_reg, &['z']);
    }

    #[test]
    fn test_op_inc() {
        let mut core = Core::new();

        core.regs[0] = 0x1f;
        core.op_inc(0);
        assert_eq!(core.pc, 0x01);
        assert_eq!(core.regs[0], 0x20);
        assert_status_reg_true!(&core.status_reg, &[]);

        core.regs[0] = 0x80;
        core.op_inc(0);
        assert_eq!(core.regs[0], 0x81);
        assert_status_reg_true!(&core.status_reg, &['s', 'n']);

        core.regs[0] = 0x7f;
        core.op_inc(0);
        assert_eq!(core.regs[0], 0x80);
        assert_status_reg_true!(&core.status_reg, &['v', 'n']);

        core.regs[0] = 0xff;
        core.op_inc(0);
        assert_eq!(core.regs[0], 0x00);
        assert_status_reg_true!(&core.status_reg, &['z']);
    }

    #[test]
    fn test_op_jmp() {
        let mut core = Core::new();

        core.op_jmp(0x0123);
        assert_eq!(core.pc, 0x0123);
    }

    #[test]
    fn test_op_ldi() {
        let mut core = Core::new();

        core.op_ldi(16, 0xab);
        assert_eq!(core.pc, 0x01);
        assert_eq!(core.regs[16], 0xab);
    }

    #[test]
    fn test_op_lsr() {
        let mut core = Core::new();

        core.regs[0] = 0x0e;
        core.op_lsr(0);
        assert_eq!(core.pc, 0x01);
        assert_eq!(core.regs[0], 0x07);
        assert_status_reg_true!(&core.status_reg, &[]);

        core.regs[0] = 0x88;
        core.op_lsr(0);
        assert_eq!(core.regs[0], 0x44);
        assert_status_reg_true!(&core.status_reg, &[]);

        core.regs[0] = 0x01;
        core.op_lsr(0);
        assert_eq!(core.regs[0], 0x00);
        assert_status_reg_true!(&core.status_reg, &['z', 'c', 'v', 's']);
    }

    #[test]
    fn test_op_mov() {
        let mut core = Core::new();

        core.regs[1] = 0xab;
        core.op_mov(0, 1);
        assert_eq!(core.pc, 0x01);
        assert_eq!(core.regs[0], 0xab);
    }

    #[test]
    fn test_op_movw() {
        let mut core = Core::new();

        core.regs[2] = 0x01;
        core.regs[3] = 0x23;
        core.op_movw(0, 2);
        assert_eq!(core.pc, 0x01);
        assert_eq!(core.regs[0], 0x01);
        assert_eq!(core.regs[1], 0x23);
    }

    #[test]
    fn test_op_mul() {
        let mut core = Core::new();

        core.regs[2] = 4;
        core.regs[3] = 3;
        core.op_mul(2, 3);
        assert_eq!(core.pc, 0x01);
        assert_eq!(u16::from_le_bytes([core.regs[0], core.regs[1]]), 12);
        assert_status_reg_true!(&core.status_reg, &[]);

        core.regs[2] = 241;
        core.regs[3] = 242;
        core.op_mul(2, 3);
        assert_eq!(u16::from_le_bytes([core.regs[0], core.regs[1]]), 58322);
        assert_status_reg_true!(&core.status_reg, &['c']);

        core.regs[2] = 171;
        core.regs[3] = 0;
        core.op_mul(2, 3);
        assert_eq!(u16::from_le_bytes([core.regs[0], core.regs[1]]), 0);
        assert_status_reg_true!(&core.status_reg, &['z']);
    }

    #[test]
    fn test_op_muls() {
        let mut core = Core::new();

        core.regs[2] = -3i8 as u8;
        core.regs[3] = 4i8 as u8;
        core.op_muls(2, 3);
        assert_eq!(core.pc, 0x01);
        assert_eq!(i16::from_le_bytes([core.regs[0], core.regs[1]]), -12);
        assert_status_reg_true!(&core.status_reg, &['c']);

        core.regs[2] = -81i8 as u8;
        core.regs[3] = -79i8 as u8;
        core.op_muls(2, 3);
        assert_eq!(i16::from_le_bytes([core.regs[0], core.regs[1]]), 6399);
        assert_status_reg_true!(&core.status_reg, &[]);

        core.regs[2] = -23i8 as u8;
        core.regs[3] = 0i8 as u8;
        core.op_muls(2, 3);
        assert_eq!(i16::from_le_bytes([core.regs[0], core.regs[1]]), 0);
        assert_status_reg_true!(&core.status_reg, &['z']);
    }

    #[test]
    fn test_op_mulsu() {
        let mut core = Core::new();

        core.regs[2] = -3i8 as u8;
        core.regs[3] = 200;
        core.op_mulsu(2, 3);
        assert_eq!(core.pc, 0x01);
        assert_eq!(i16::from_le_bytes([core.regs[0], core.regs[1]]), -600);
        assert_status_reg_true!(&core.status_reg, &['c']);

        core.regs[2] = 120i8 as u8;
        core.regs[3] = 234;
        core.op_mulsu(2, 3);
        assert_eq!(i16::from_le_bytes([core.regs[0], core.regs[1]]), 28080);
        assert_status_reg_true!(&core.status_reg, &[]);

        core.regs[2] = -23i8 as u8;
        core.regs[3] = 0 as u8;
        core.op_mulsu(2, 3);
        assert_eq!(i16::from_le_bytes([core.regs[0], core.regs[1]]), 0);
        assert_status_reg_true!(&core.status_reg, &['z']);
    }

    #[test]
    fn test_op_neg() {
        let mut core = Core::new();

        core.regs[0] = 0x00;
        core.op_neg(0);
        assert_eq!(core.pc, 0x01);
        assert_eq!(core.regs[0], 0x00);
        assert_status_reg_true!(&core.status_reg, &['z']);

        core.regs[0] = 0x80;
        core.op_neg(0);
        assert_eq!(core.regs[0], 0x80);
        assert_status_reg_true!(&core.status_reg, &['v', 'n', 'c']);

        core.regs[0] = 0x01;
        core.op_neg(0);
        assert_eq!(core.regs[0], 0xff);
        assert_status_reg_true!(&core.status_reg, &['h', 's', 'n', 'c']);
    }

    #[test]
    fn test_op_or() {
        let mut core = Core::new();

        core.regs[0] = 0x12;
        core.regs[1] = 0x34;
        core.op_or(0, 1);
        assert_eq!(core.pc, 0x01);
        assert_eq!(core.regs[0], 0x36);
        assert_status_reg_true!(&core.status_reg, &[]);

        core.regs[0] = 0x81;
        core.regs[1] = 0x65;
        core.op_or(0, 1);
        assert_eq!(core.regs[0], 0xe5);
        assert_status_reg_true!(&core.status_reg, &['n']);

        core.regs[0] = 0x00;
        core.regs[1] = 0x00;
        core.op_or(0, 1);
        assert_eq!(core.regs[0], 0x00);
        assert_status_reg_true!(&core.status_reg, &['z']);
    }

    #[test]
    fn test_op_ori() {
        let mut core = Core::new();

        core.regs[0] = 0x12;
        core.op_ori(0, 0x34);
        assert_eq!(core.pc, 0x01);
        assert_eq!(core.regs[0], 0x36);
        assert_status_reg_true!(&core.status_reg, &[]);

        core.regs[0] = 0x81;
        core.op_ori(0, 0x65);
        assert_eq!(core.regs[0], 0xe5);
        assert_status_reg_true!(&core.status_reg, &['n']);

        core.regs[0] = 0x00;
        core.op_ori(0, 0x00);
        assert_eq!(core.regs[0], 0x00);
        assert_status_reg_true!(&core.status_reg, &['z']);
    }

    #[test]
    fn test_op_push_pop() {
        let mut core = Core::new();

        core.regs[0] = 0x42;
        core.op_push(0);
        assert_eq!(core.pc, 0x01);
        assert_eq!(core.sp, SRAM_SIZE - 1 - 1);
        assert_eq!(core.ram[SRAM_SIZE - 1], 0x42);

        core.op_pop(1);
        assert_eq!(core.pc, 0x02);
        assert_eq!(core.sp, SRAM_SIZE - 1);
        assert_eq!(core.regs[1], 0x42);
    }

    #[test]
    fn test_op_rcall() {
        let mut core = Core::new();

        core.pc = 0x01;
        core.op_rcall(0x0123);
        assert_eq!(core.pc, 0x0124);
        assert_eq!(core.sp, (SRAM_SIZE - 1) - 2);
        assert_eq!(core.ram.get_u16((SRAM_SIZE - 1) - 1), 0x02);

        core.op_rcall(-0x0025);
        assert_eq!(core.pc, 0x00ff);
        assert_eq!(core.sp, (SRAM_SIZE - 1) - (2 + 2));
        assert_eq!(core.ram.get_u16((SRAM_SIZE - 1) - (1 + 2)), 0x0125);
    }

    #[test]
    fn test_op_ret() {
        let mut core = Core::new();

        core.pc = 0x51;
        core.ram[SRAM_SIZE - 1] = 0x01;
        core.ram[SRAM_SIZE - 1 - 1] = 0x23;
        core.sp = SRAM_SIZE - 1 - 2;
        core.op_ret();
        assert_eq!(core.pc, 0x0123);
        assert_eq!(core.sp, SRAM_SIZE - 1);
    }

    #[test]
    fn test_op_reti() {
        let mut core = Core::new();

        core.pc = 0x51;
        core.ram[SRAM_SIZE - 1] = 0x01;
        core.ram[SRAM_SIZE - 1 - 1] = 0x23;
        core.sp = SRAM_SIZE - 1 - 2;
        core.op_reti();
        assert_eq!(core.pc, 0x0123);
        assert_eq!(core.sp, SRAM_SIZE - 1);
        assert_status_reg_true!(&core.status_reg, &['i']);
    }

    #[test]
    fn test_op_rjmp() {
        let mut core = Core::new();

        core.pc = 0x01;
        core.op_rjmp(0x0123);
        assert_eq!(core.pc, 0x0124);

        core.op_rjmp(-0x0025);
        assert_eq!(core.pc, 0x00ff);
    }

    #[test]
    fn test_op_rol() {
        let mut core = Core::new();

        core.regs[0] = 0x00;
        core.status_reg.c = false;
        core.op_rol(0);
        assert_eq!(core.pc, 0x01);
        assert_eq!(core.regs[0], 0x00);
        assert_status_reg_true!(&core.status_reg, &['z']);

        core.regs[0] = 0x00;
        core.status_reg.c = true;
        core.op_rol(0);
        assert_eq!(core.regs[0], 0x01);
        assert_status_reg_true!(&core.status_reg, &[]);

        core.regs[0] = 0x90;
        core.status_reg.c = false;
        core.op_rol(0);
        assert_eq!(core.regs[0], 0x20);
        assert_status_reg_true!(&core.status_reg, &['c', 's', 'v']);

        core.regs[0] = 0x48;
        core.status_reg.c = false;
        core.op_rol(0);
        assert_eq!(core.regs[0], 0x90);
        assert_status_reg_true!(&core.status_reg, &['n', 'v', 'h']);
    }

    #[test]
    fn test_op_ror() {
        let mut core = Core::new();

        core.regs[0] = 0x00;
        core.status_reg.c = false;
        core.op_ror(0);
        assert_eq!(core.pc, 0x01);
        assert_eq!(core.regs[0], 0x00);
        assert_status_reg_true!(&core.status_reg, &['z']);

        core.regs[0] = 0x00;
        core.status_reg.c = true;
        core.op_ror(0);
        assert_eq!(core.regs[0], 0x80);
        assert_status_reg_true!(&core.status_reg, &['n', 'v']);

        core.regs[0] = 0x81;
        core.status_reg.c = false;
        core.op_ror(0);
        assert_eq!(core.regs[0], 0x40);
        assert_status_reg_true!(&core.status_reg, &['c', 's', 'v']);
    }

    #[test]
    fn test_op_sbc() {
        let mut core = Core::new();

        core.status_reg.c = true;
        core.regs[0] = 0x08;
        core.regs[1] = 0x04;
        core.op_sbc(0, 1);
        assert_eq!(core.pc, 0x01);
        assert_eq!(core.regs[0], 0x03);
        assert_status_reg_true!(&core.status_reg, &[]);

        core.status_reg.c = true;
        core.status_reg.z = true;
        core.regs[0] = 0x80;
        core.regs[1] = 0x7f;
        core.op_sbc(0, 1);
        assert_eq!(core.regs[0], 0x00);
        assert_status_reg_true!(&core.status_reg, &['z', 's', 'h', 'v']);

        core.status_reg.c = false;
        core.regs[0] = 0x00;
        core.regs[1] = 0x01;
        core.op_sbc(0, 1);
        assert_eq!(core.regs[0], 0xff);
        assert_status_reg_true!(&core.status_reg, &['s', 'n', 'c', 'h']);
    }

    #[test]
    fn test_op_sbci() {
        let mut core = Core::new();

        core.status_reg.c = true;
        core.regs[0] = 0x08;
        core.op_sbci(0, 0x04);
        assert_eq!(core.pc, 0x01);
        assert_eq!(core.regs[0], 0x03);
        assert_status_reg_true!(&core.status_reg, &[]);

        core.status_reg.c = true;
        core.status_reg.z = true;
        core.regs[0] = 0x80;
        core.op_sbci(0, 0x7f);
        assert_eq!(core.regs[0], 0x00);
        assert_status_reg_true!(&core.status_reg, &['z', 's', 'h', 'v']);

        core.status_reg.c = false;
        core.regs[0] = 0x00;
        core.op_sbci(0, 0x01);
        assert_eq!(core.regs[0], 0xff);
        assert_status_reg_true!(&core.status_reg, &['s', 'n', 'c', 'h']);
    }
}
