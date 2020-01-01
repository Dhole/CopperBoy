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

    // Auxiliary function to update some flags in adc and add
    fn aux_op_add_flags(&mut self, r: u8, d: u8, c: bool, res: u8) {
        let (r7, rr7, rd7) = (res & 1 << 7, self.regs[r] & 1 << 7, self.regs[d] & 1 << 7);
        self.status_reg.n = r7 != 0;
        self.status_reg.v = (rr7 == rd7) && (rr7 != r7);
        self.status_reg.s = self.status_reg.n ^ self.status_reg.v;
        self.status_reg.h = ((self.regs[r] & 0x0f) + (self.regs[d] & 0x0f) + c as u8) > 0x0f;
        self.status_reg.z = res == 0;
    }
    /// 5. Add with Carry (ADC)
    fn op_adc(&mut self, d: u8, r: u8) -> usize {
        let (res, c0) = self.regs[d].overflowing_add(self.regs[r]);
        let (res, c1) = res.overflowing_add(self.status_reg.c as u8);
        self.status_reg.c = c0 || c1;
        self.aux_op_add_flags(r, d, self.status_reg.c, res);
        self.regs[d] = res;

        self.pc += 1;
        1
    }
    /// 6. Add without Carry (ADD)
    fn op_add(&mut self, d: u8, r: u8) -> usize {
        let (res, c0) = self.regs[d].overflowing_add(self.regs[r]);
        self.status_reg.c = c0;
        self.aux_op_add_flags(r, d, false, res);
        self.regs[d] = res;

        self.pc += 1;
        1
    }

    /// 7. Add Immediate Word (ADIW)
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

    /// 8. Logical AND (AND)
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

    /// 9. Logical AND with Immediate (ANDI)
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

    /// 10. Arithmetic Shift Right (ASR)
    fn op_asr(&mut self, d: u8) -> usize {
        let res = self.regs[d] >> 1 | self.regs[d] & 1 << 7;
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

    /// 11. Bit Clear in SREG (BCLR)
    fn op_bclr(&mut self, s: u8) -> usize {
        self.status_reg[s] = false;

        self.pc += 1;
        1
    }

    /// 12. Bit Load from the T Flag in SREG to a Bit in Register (BLD)
    fn op_bld(&mut self, d: u8, b: u8) -> usize {
        self.regs[d] &= !(1 << b);
        self.regs[d] |= (self.status_reg.t as u8) << b;

        self.pc += 1;
        1
    }

    // Auxiliary function to branch if a boolean is true
    fn aux_op_branch_if(&mut self, k: i8, test: bool) -> usize {
        if test {
            let pc = if k >= 0 {
                let (pc, _) = self.pc.overflowing_add(k as u16 + 1);
                pc
            } else {
                let (pc, _) = self.pc.overflowing_sub(-k as u16 - 1);
                pc
            };
            self.pc = pc;
            2
        } else {
            self.pc += 1;
            1
        }
    }
    /// 13. Branch if Bit in SREG is Cleared (BRBC)
    fn op_brbc(&mut self, s: u8, k: i8) -> usize {
        self.aux_op_branch_if(k, !self.status_reg[s])
    }
    /// 14. Branch if Bit in SREG is Set (BRBS)
    fn op_brbs(&mut self, s: u8, k: i8) -> usize {
        self.aux_op_branch_if(k, self.status_reg[s])
    }
    // 15. Branch if Carry Cleared (BRCC) -> BRBC C
    // 16. Branch if Carry Set (BRCS) -> BRBS C

    /// 17. Break (BREAK)
    fn op_break(&mut self) -> usize {
        unimplemented!();
        1
    }

    // 18. Branch if Equal (BREQ) -> BRBS Z
    // 19. Branch if Greater or Equal (Signed) (BRGE) -> BRBC S
    // 20. Branch if Half Carry Flag is Cleared (BRHC) -> BRBC H
    // 21. Branch if Half Carry Flag is Set (BRHS) -> BRBS H
    // 22. Branch if Global Interrupt is Disabled (BRID) -> BRBC I
    // 23. Branch if Global Interrupt is Enabled (BRIE) -> BRBS I
    // 24. Branch if Lower (Unsigned) (BRLO) -> BRBS C
    // 25. Branch if Less Than (Signed) (BRLT) -> BRBS S
    // 26. Branch if Minus (BRMI) -> BRBS N
    // 27. Branch if Not Equal (BRNE) -> BRBC Z
    // 28. Branch if Plus (BRPL) -> BRBC N
    // 29. Branch if Same or Higher (Unsigned) (BRSH) -> BRBC C
    // 30. Branch if the T Flag is Cleared (BRTC) -> BRBC T
    // 31. Branch if the T Flag is Set (BRTS) -> BRBS T
    // 32. Branch if Overflow Cleared (BRVS) -> BRBC V
    // 33. Branch if Overflow Set (BRVS) -> BRBS V

    /// 34. Bit Set in SREG (BSET)
    fn op_bset(&mut self, s: u8) -> usize {
        self.status_reg[s] = true;

        self.pc += 1;
        1
    }

    /// 35. Bit Store from Bit in Register to T Flag in SREG (BST)
    fn op_bst(&mut self, d: u8, b: u8) -> usize {
        self.status_reg.t = (self.regs[d] & (1 << b)) != 0;

        self.pc += 1;
        1
    }

    /// 36. Long Call to a Subroutine (CALL)
    fn op_call(&mut self, k: u16) -> usize {
        self.push_u16(self.pc + 2);
        self.pc = k;
        4
    }

    /// 37. Clear Bit in I/O Register (CBI)
    fn op_cbi(&mut self, a: u8, d: u8) -> usize {
        self.io_regs[a as usize] &= !(1 << d);
        self.pc += 1;
        2
    }

    // 38. Clear Bits in Register (CBR) -> ANDI with K complemented
    // 39. Clear Carry Flag (CLC) -> BCLR C
    // 40. Clear Half Carry Flag (CLH) -> BCLR H
    // 41. Clear Global Interrupt Flag (CLI) -> BCLR I
    // 42. Clear Negative Flag (CLN) -> BCLR N

    /// 43. Clear Register
    fn op_clr(&mut self, d: u8) -> usize {
        self.regs[d] = 0;
        self.status_reg.s = false;
        self.status_reg.v = false;
        self.status_reg.n = false;
        self.status_reg.z = true;

        self.pc += 1;
        1
    }

    // 44. Clear Signed Flag (CLS) -> BCLR S
    // 45. Clear T Flag (CLT) -> BCLR T
    // 46. Clear Overflow Flag (CLV) -> BCLR V
    // 47. Clear Zero Flag (CLZ) -> BCLR Z

    /// 48. One's Complement (COM)
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

    /// 49. Compare (CP)
    fn op_cp(&mut self, d: u8, r: u8) -> usize {
        let (res, c0) = self.regs[d].overflowing_sub(self.regs[r]);

        let (r7, rr7, rd7) = (res & 1 << 7, self.regs[r] & 1 << 7, self.regs[d] & 1 << 7);
        self.status_reg.n = r7 != 0;
        self.status_reg.v = (rr7 != rd7) && (rr7 == r7);
        self.status_reg.s = self.status_reg.n ^ self.status_reg.v;
        self.status_reg.h = ((self.regs[r] & 0x0f) > (self.regs[d] & 0x0f));
        self.status_reg.z = res == 0;
        self.status_reg.c = c0;

        self.pc += 1;
        1
    }

    /// 50. Compare with Carry (CPC)
    fn op_cpc(&mut self, d: u8, r: u8) -> usize {
        let (res, c0) = self.regs[d].overflowing_sub(self.regs[r]);
        let (res, c1) = res.overflowing_sub(self.status_reg.c as u8);

        let (r7, rr7, rd7) = (res & 1 << 7, self.regs[r] & 1 << 7, self.regs[d] & 1 << 7);
        self.status_reg.n = r7 != 0;
        self.status_reg.v = (rr7 != rd7) && (rr7 == r7);
        self.status_reg.s = self.status_reg.n ^ self.status_reg.v;
        self.status_reg.h =
            (((self.regs[r] & 0x0f) + self.status_reg.c as u8) > (self.regs[d] & 0x0f));
        if res != 0 {
            self.status_reg.z = false;
        }
        self.status_reg.c = c0 || c1;

        self.pc += 1;
        1
    }
    // TODO: 51. Compare with Immediate (CPI)
    // TODO: 52. Compare Skip if Equal (CPSE)
    // TODO: 53. Decrement (DEC)
    // TODO: 54. Data Encryption Standard (DES)
    // TODO: 55. Extended Indirect Call to Subroutine (EICALL)
    // TODO: 56. Extended Indirect Jump (EIJMP)
    // TODO: 57. Extended Load Program Memory (ELPM)
    // TODO: 58. Exclusive OR (EOR)
    // TODO: 59. Fractional Multiply Unsiged (FMUL)
    // TODO: 60. Fractional Multiply Signed (FMULS)
    // TODO: 61. Fractional Multiply Signed with Unsigned (FMULSU)
    // TODO: 62. Indirect Call to Subroutine (ICALL)
    // TODO: 63. Indirect Jump (IJMP)
    // TODO: 64. Load an I/O Location to Register (IN)
    // TODO: 65. Increment (INC)
    // TODO: 66. Jump (JMP)
    // TODO: 67. Load and Clear (LAC)
    // TODO: 68. Load and Set (LAS)
    // TODO: 69. Load and Toggle (LAT)
    // TODO: 70. Load Indirect from Data Space to Register using Index X (LD)
    // TODO: 71. Load Indirect from Data Space to Register using Index Y (LD)
    // TODO: 72. Load Indirect from Data Space to Register using Index Z (LD)
    // TODO: 73. Load Immediate (LDI)
    // TODO: 74. Load Direct from Data Space (LDS)
    // TODO: 75. Load Direct from Data Space (LDS 16bit)
    // TODO: 76. Load Program Memory (LPM)
    // TODO: 77. Logical Shift Left (LSL)
    // TODO: 78. Logical Shift Right (LSR)
    // TODO: 79. Copy Register (MOV)
    // TODO: 80. Copy Register Word (MOVW)
    // TODO: 81. Multiply Unsiged (MUL)
    // TODO: 82. Multiply Signed (MULS)
    // TODO: 83. Multiply Signed with Unsigned (MULSU)
    // TODO: 84. Two's Complement (NEG)
    // TODO: 85. No Operation (NOP)
    // TODO: 86. Logical OR (OR)
    // TODO: 87. Logical OR with Immediate (ORI)
    // TODO: 88. Store Register to I/O Location (OUT)
    // TODO: 89. Pop Register from Stack (POP)
    // TODO: 90. Push Register on Stack (PUSH)
    // TODO: 91. Relative Call to Subroutione (RCALL)
    // TODO: 92. Return from Subroutine (RET)
    // TODO: 93. Return from Interrupt (RETI)
    // TODO: 94. Relative Jump (RJMP)
    // TODO: 95. Rotate Left through Carry (ROL)
    // TODO: 96. Rotrate Right through Carry (ROR)
    // TODO: 97. Subtract with Carry (SBC)
    // TODO: 98. Subtract Immediate with Carry (SBCI)
    // TODO: 99. Set Bit in I/O Register (SBI)
    // TODO: 100. Skip if Bit in I/O Register is Cleared (SBIC)
    // TODO: 101. Skip if Bit in I/O Register is Set (SBIS)
    // TODO: 102. Subtract Immedaite from Word (SBIW)
    // TODO: 103. Set Bits in Register (SBR)
    // TODO: 104. Skip if Bit in Register is Cleared (SBRC)
    // TODO: 105. Skip if Bit in Register is Set (SBRS)
    // 106. Set Carry Flag (SEC) -> BSET C
    // 107. Set Half Carry Flag (SEH) -> BSET H
    // 108. Set Global Interrupt Flag (SEI) -> BSET I
    // 109. Set Negative Flag (SEN) -> BSET N
    // TODO: 110. Set all Bits in Register (SER)
    // 111. Set Signed Flag (SES) -> BSET S
    // 112. Set T Flag (SET) -> BSET T
    // 113. Set Overflow Flag (SEV) -> BSET V
    // 114. Set Zero Flag (SEZ) -> BSET Z
    // TODO: 115. SLEEP
    // TODO: 116. Store Program Memory (SPM)
    // TODO: 117. Store Program Memory (SPM #2)
    // TODO: 118. Store Indirect from Register to Data Space using Index X (ST)
    // TODO: 119. Store Indirect from Register to Data Space using Index Y (ST)
    // TODO: 120. Store Indirect from Register to Data Space using Index Z (ST)
    // TODO: 121. Store Direct to Data Space
    // TODO: 122. Store Direct to Data Space (STS 16bit)
    // TODO: 123. Subtract without Carry (SUB)
    // TODO: 124. Subtract Immediate (SUBI)
    // TODO: 125. Swap Nibbles (SWAP)
    // TODO: 126. Test for Zero or Minus (TST)
    // TODO: 127. Watchdog Reset (WDR)
    // TODO: 128. Exchange (XCH)
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

    // #[test]
    // fn test_op_brcc() {
    //     let mut core = Core::new();

    //     core.status_reg.c = true;
    //     core.op_brcc(1, 0x10);
    //     assert_eq!(core.pc, 0x01);

    //     core.status_reg.c = false;
    //     core.op_brcc(1, 0x10);
    //     assert_eq!(core.pc, 0x12);
    // }

    // #[test]
    // fn test_op_brcs() {
    //     let mut core = Core::new();

    //     core.status_reg.c = false;
    //     core.op_brcs(1, 0x10);
    //     assert_eq!(core.pc, 0x01);

    //     core.status_reg.c = true;
    //     core.op_brcs(1, 0x10);
    //     assert_eq!(core.pc, 0x12);
    // }

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
}
