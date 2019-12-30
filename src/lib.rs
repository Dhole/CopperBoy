use std::ops::{Index, IndexMut};

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

struct GeneralRegisters {
    reg: [u8; 32],
}

impl Index<u8> for GeneralRegisters {
    type Output = u8;

    fn index(&self, d: u8) -> &u8 {
        &self.reg[d as usize]
    }
}

impl IndexMut<u8> for GeneralRegisters {
    fn index_mut(&mut self, d: u8) -> &mut u8 {
        &mut self.reg[d as usize]
    }
}

impl GeneralRegisters {
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
}

struct Core {
    /// General Purpose Register File
    regs: GeneralRegisters,
    /// Status Register
    status_reg: StatusRegister,
    /// Program Counter
    pc: u16,
    /// Stack Pointer
    sp: u16,
}

impl Core {
    fn op_aux_add(&mut self, r: u8, d: u8, c: bool) -> usize {
        let (res, c0) = self.regs[r].overflowing_add(self.regs[d]);
        let (res, c1) = res.overflowing_add(c as u8);

        self.status_reg.c = c0 || c1;
        let r7 = res & 0b10000000;
        self.status_reg.n = r7 != 0;
        let (rr7, rd7) = (self.regs[r] & 0b10000000, self.regs[d] & 0b10000000);
        self.status_reg.v = (rr7 == rd7) && (rr7 != r7);
        self.status_reg.h = self.regs[r] & 0x0f + self.regs[d] & 0x0f + c as u8 > 0xf;
        self.status_reg.z = res == 0;
        self.status_reg.s = self.status_reg.n ^ self.status_reg.v;

        self.regs[r] = res;
        self.pc += 1;
        1
    }
    fn op_adc(&mut self, r: u8, d: u8) -> usize {
        self.op_aux_add(r, d, self.status_reg.c)
    }
    fn op_add(&mut self, r: u8, d: u8) -> usize {
        self.op_aux_add(r, d, false)
    }
}
