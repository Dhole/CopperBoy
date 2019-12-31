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
    fn new() -> Self {
        Self {
            regs: GeneralRegisters::new(),
            status_reg: StatusRegister::new(),
            pc: 0,
            sp: 0,
        }
    }

    // Aux function to update some flags in adc and add
    fn aux_op_add_flags(&mut self, r: u8, d: u8, c: bool, res: u8) {
        let (r7, rr7, rd7) = (res & 1 << 7, self.regs[r] & 1 << 7, self.regs[d] & 1 << 7);
        self.status_reg.n = r7 != 0;
        self.status_reg.v = (rr7 == rd7) && (rr7 != r7);
        self.status_reg.s = self.status_reg.n ^ self.status_reg.v;
        self.status_reg.h = ((self.regs[r] & 0x0f) + (self.regs[d] & 0x0f) + c as u8) > 0x0f;
        self.status_reg.z = res == 0;
    }
    /// Add with Carry
    fn op_adc(&mut self, d: u8, r: u8) -> usize {
        let (res, c0) = self.regs[d].overflowing_add(self.regs[r]);
        let (res, c1) = res.overflowing_add(self.status_reg.c as u8);
        self.status_reg.c = c0 || c1;
        self.aux_op_add_flags(r, d, self.status_reg.c, res);
        self.regs[d] = res;

        self.pc += 1;
        1
    }
    /// Add without Carry
    fn op_add(&mut self, d: u8, r: u8) -> usize {
        let (res, c0) = self.regs[d].overflowing_add(self.regs[r]);
        self.status_reg.c = c0;
        self.aux_op_add_flags(r, d, false, res);
        self.regs[d] = res;

        self.pc += 1;
        1
    }

    /// Add Immediate Word
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

    /// Logical AND
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

    /// Logical AND with Immediate
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

    /// Arithmetic Shift Right
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

    /// Bit Clear in SREG
    fn op_bclr(&mut self, s: u8) -> usize {
        self.status_reg[s] = false;

        self.pc += 1;
        1
    }

    /// Bit Load from the T Flag in SREG to a Bit in Register
    fn op_bld(&mut self, d: u8, b: u8) -> usize {
        self.regs[d] &= !(1 << b);
        self.regs[d] |= (self.status_reg.t as u8) << b;

        self.pc += 1;
        1
    }

    /// Branch if Bit in SREG is Cleared
    fn op_brbc(&mut self, s: u8, k: i8) -> usize {
        if self.status_reg[s] {
            let pc = if k >= 0 {
                println!("positive {}", k);
                let (pc, _) = self.pc.overflowing_add(k as u16 + 1);
                pc
            } else {
                let (pc, _) = self.pc.overflowing_sub(-k as u16 - 1);
                println!("negative {}", k);
                pc
            };
            self.pc = pc;
            2
        } else {
            self.pc += 1;
            1
        }
    }
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

        core.status_reg.z = false;
        core.op_brbc(1, 0x10);
        assert_eq!(core.pc, 0x01);

        core.status_reg.z = true;
        core.op_brbc(1, 0x10);
        assert_eq!(core.pc, 0x12);

        core.status_reg.h = true;
        core.op_brbc(5, -0x05);
        assert_eq!(core.pc, 0x0e);
    }
}
