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

// 5
const OPCODE_OP_ADC_BITS: u16 = 0b0001_1100_0000_0000;
const OPCODE_OP_ADC_MASK: u16 = 0b1111_1100_0000_0000;

// 6
const OPCODE_OP_ADD_BITS: u16 = 0b0000_1100_0000_0000;
const OPCODE_OP_ADD_MASK: u16 = 0b1111_1100_0000_0000;

// 7
const OPCODE_OP_ADIW_BITS: u16 = 0b1001_0110_0000_0000;
const OPCODE_OP_ADIW_MASK: u16 = 0b1111_1111_0000_0000;

// 8
const OPCODE_OP_AND_BITS: u16 = 0b0010_0000_0000_0000;
const OPCODE_OP_AND_MASK: u16 = 0b1111_1100_0000_0000;

// 9
const OPCODE_OP_ANDI_BITS: u16 = 0b0111_0000_0000_0000;
const OPCODE_OP_ANDI_MASK: u16 = 0b1111_0000_0000_0000;

// 10
const OPCODE_OP_ASR_BITS: u16 = 0b1001_0100_0000_0101;
const OPCODE_OP_ASR_MASK: u16 = 0b1111_1110_0000_1111;

// 11
const OPCODE_OP_BCLR_BITS: u16 = 0b1001_0100_1000_1000;
const OPCODE_OP_BCLR_MASK: u16 = 0b1111_1111_1000_1111;

// 12
const OPCODE_OP_BLD_BITS: u16 = 0b1111_1000_0000_0000;
const OPCODE_OP_BLD_MASK: u16 = 0b1111_1110_0000_1000;

// 13
const OPCODE_OP_BRBC_BITS: u16 = 0b1111_0100_0000_0000;
const OPCODE_OP_BRBC_MASK: u16 = 0b1111_1100_0000_0000;

// 14
const OPCODE_OP_BRBS_BITS: u16 = 0b1111_0000_0000_0000;
const OPCODE_OP_BRBS_MASK: u16 = 0b1111_1100_0000_0000;

// 17
const OPCODE_OP_BREAK_BITS: u16 = 0b1001_0101_1001_1000;
const OPCODE_OP_BREAK_MASK: u16 = 0b1111_1111_1111_1111;

// 34
const OPCODE_OP_BSET_BITS: u16 = 0b1001_0100_0000_1000;
const OPCODE_OP_BSET_MASK: u16 = 0b1111_1111_1000_1111;

// 35
const OPCODE_OP_BST_BITS: u16 = 0b1111_1010_0000_0000;
const OPCODE_OP_BST_MASK: u16 = 0b1111_1110_0000_1000;

// 36
const OPCODE_OP_CALL_BITS: u16 = 0b1001_0100_0000_1110;
const OPCODE_OP_CALL_MASK: u16 = 0b1111_1110_0000_1110;

// 37
const OPCODE_OP_CBI_BITS: u16 = 0b1001_1000_0000_0000;
const OPCODE_OP_CBI_MASK: u16 = 0b1111_1111_0000_0000;

// 43
const OPCODE_OP_CLR_BITS: u16 = 0b0010_0100_0000_0000;
const OPCODE_OP_CLR_MASK: u16 = 0b1111_1100_0000_0000;

// 48
const OPCODE_OP_COM_BITS: u16 = 0b1001_0100_0000_0000;
const OPCODE_OP_COM_MASK: u16 = 0b1111_1110_0000_1111;

// 49
const OPCODE_OP_CP_BITS: u16 = 0b0001_0100_0000_0000;
const OPCODE_OP_CP_MASK: u16 = 0b1111_1100_0000_0000;

// 50
const OPCODE_OP_CPC_BITS: u16 = 0b0000_0100_0000_0000;
const OPCODE_OP_CPC_MASK: u16 = 0b1111_1100_0000_0000;

// 51
const OPCODE_OP_CPI_BITS: u16 = 0b0011_0000_0000_0000;
const OPCODE_OP_CPI_MASK: u16 = 0b1111_0000_0000_0000;

// 52
const OPCODE_OP_CPSE_BITS: u16 = 0b0001_0000_0000_0000;
const OPCODE_OP_CPSE_MASK: u16 = 0b1111_1100_0000_0000;

// 53
const OPCODE_OP_DEC_BITS: u16 = 0b1001_0100_0000_1010;
const OPCODE_OP_DEC_MASK: u16 = 0b1111_1110_0000_1111;

// 55
const OPCODE_OP_EICALL_BITS: u16 = 0b1001_0101_0001_1001;
const OPCODE_OP_EICALL_MASK: u16 = 0b1111_1111_1111_1111;

// 56
const OPCODE_OP_EIJMP_BITS: u16 = 0b1001_0100_0001_1001;
const OPCODE_OP_EIJMP_MASK: u16 = 0b1111_1111_1111_1111;

// TODO
// 57
const OPCODE_OP_ELPM_BITS: u16 = 0b0000_0000_0000_0000;
const OPCODE_OP_ELPM_MASK: u16 = 0b0000_0000_0000_0000;

// 58
const OPCODE_OP_EOR_BITS: u16 = 0b0000_0000_0000_0000;
const OPCODE_OP_EOR_MASK: u16 = 0b0000_0000_0000_0000;

// 59
const OPCODE_OP_FMUL_BITS: u16 = 0b0000_0000_0000_0000;
const OPCODE_OP_FMUL_MASK: u16 = 0b0000_0000_0000_0000;

// 60
const OPCODE_OP_FMULS_BITS: u16 = 0b0000_0000_0000_0000;
const OPCODE_OP_FMULS_MASK: u16 = 0b0000_0000_0000_0000;

// 61
const OPCODE_OP_FMULSU_BITS: u16 = 0b0000_0000_0000_0000;
const OPCODE_OP_FMULSU_MASK: u16 = 0b0000_0000_0000_0000;

// 62
const OPCODE_OP_ICALL_BITS: u16 = 0b0000_0000_0000_0000;
const OPCODE_OP_ICALL_MASK: u16 = 0b0000_0000_0000_0000;

// 63
const OPCODE_OP_IJMP_BITS: u16 = 0b0000_0000_0000_0000;
const OPCODE_OP_IJMP_MASK: u16 = 0b0000_0000_0000_0000;

// 64
const OPCODE_OP_IN_BITS: u16 = 0b0000_0000_0000_0000;
const OPCODE_OP_IN_MASK: u16 = 0b0000_0000_0000_0000;

// 65
const OPCODE_OP_INC_BITS: u16 = 0b0000_0000_0000_0000;
const OPCODE_OP_INC_MASK: u16 = 0b0000_0000_0000_0000;

// 66
const OPCODE_OP_JMP_BITS: u16 = 0b0000_0000_0000_0000;
const OPCODE_OP_JMP_MASK: u16 = 0b0000_0000_0000_0000;

// 70
const OPCODE_OP_LDX_BITS: u16 = 0b0000_0000_0000_0000;
const OPCODE_OP_LDX_MASK: u16 = 0b0000_0000_0000_0000;

// 71
const OPCODE_OP_LDY_BITS: u16 = 0b0000_0000_0000_0000;
const OPCODE_OP_LDY_MASK: u16 = 0b0000_0000_0000_0000;

// 72
const OPCODE_OP_LDZ_BITS: u16 = 0b0000_0000_0000_0000;
const OPCODE_OP_LDZ_MASK: u16 = 0b0000_0000_0000_0000;

// 73
const OPCODE_OP_LDI_BITS: u16 = 0b0000_0000_0000_0000;
const OPCODE_OP_LDI_MASK: u16 = 0b0000_0000_0000_0000;

// 74
const OPCODE_OP_LDS_BITS: u16 = 0b0000_0000_0000_0000;
const OPCODE_OP_LDS_MASK: u16 = 0b0000_0000_0000_0000;

// 75
const OPCODE_OP_LDS16_BITS: u16 = 0b0000_0000_0000_0000;
const OPCODE_OP_LDS16_MASK: u16 = 0b0000_0000_0000_0000;

// 76
const OPCODE_OP_LPM_BITS: u16 = 0b0000_0000_0000_0000;
const OPCODE_OP_LPM_MASK: u16 = 0b0000_0000_0000_0000;

// 78
const OPCODE_OP_LSR_BITS: u16 = 0b0000_0000_0000_0000;
const OPCODE_OP_LSR_MASK: u16 = 0b0000_0000_0000_0000;

// 79
const OPCODE_OP_MOV_BITS: u16 = 0b0000_0000_0000_0000;
const OPCODE_OP_MOV_MASK: u16 = 0b0000_0000_0000_0000;

// 80
const OPCODE_OP_MOVW_BITS: u16 = 0b0000_0000_0000_0000;
const OPCODE_OP_MOVW_MASK: u16 = 0b0000_0000_0000_0000;

// 81
const OPCODE_OP_MUL_BITS: u16 = 0b0000_0000_0000_0000;
const OPCODE_OP_MUL_MASK: u16 = 0b0000_0000_0000_0000;

// 82
const OPCODE_OP_MULS_BITS: u16 = 0b0000_0000_0000_0000;
const OPCODE_OP_MULS_MASK: u16 = 0b0000_0000_0000_0000;

// 83
const OPCODE_OP_MULSU_BITS: u16 = 0b0000_0000_0000_0000;
const OPCODE_OP_MULSU_MASK: u16 = 0b0000_0000_0000_0000;

// 84
const OPCODE_OP_NEG_BITS: u16 = 0b0000_0000_0000_0000;
const OPCODE_OP_NEG_MASK: u16 = 0b0000_0000_0000_0000;

// 85
const OPCODE_OP_NOP_BITS: u16 = 0b0000_0000_0000_0000;
const OPCODE_OP_NOP_MASK: u16 = 0b0000_0000_0000_0000;

// 86
const OPCODE_OP_OR_BITS: u16 = 0b0000_0000_0000_0000;
const OPCODE_OP_OR_MASK: u16 = 0b0000_0000_0000_0000;

// 87
const OPCODE_OP_ORI_BITS: u16 = 0b0000_0000_0000_0000;
const OPCODE_OP_ORI_MASK: u16 = 0b0000_0000_0000_0000;

// 88
const OPCODE_OP_OUT_BITS: u16 = 0b0000_0000_0000_0000;
const OPCODE_OP_OUT_MASK: u16 = 0b0000_0000_0000_0000;

// 89
const OPCODE_OP_POP_BITS: u16 = 0b0000_0000_0000_0000;
const OPCODE_OP_POP_MASK: u16 = 0b0000_0000_0000_0000;

// 90
const OPCODE_OP_PUSH_BITS: u16 = 0b0000_0000_0000_0000;
const OPCODE_OP_PUSH_MASK: u16 = 0b0000_0000_0000_0000;

// 91
const OPCODE_OP_RCALL_BITS: u16 = 0b0000_0000_0000_0000;
const OPCODE_OP_RCALL_MASK: u16 = 0b0000_0000_0000_0000;

// 92
const OPCODE_OP_RET_BITS: u16 = 0b0000_0000_0000_0000;
const OPCODE_OP_RET_MASK: u16 = 0b0000_0000_0000_0000;

// 93
const OPCODE_OP_RETI_BITS: u16 = 0b0000_0000_0000_0000;
const OPCODE_OP_RETI_MASK: u16 = 0b0000_0000_0000_0000;

// 94
const OPCODE_OP_RJMP_BITS: u16 = 0b0000_0000_0000_0000;
const OPCODE_OP_RJMP_MASK: u16 = 0b0000_0000_0000_0000;

// 95
const OPCODE_OP_ROL_BITS: u16 = 0b0000_0000_0000_0000;
const OPCODE_OP_ROL_MASK: u16 = 0b0000_0000_0000_0000;

// 96
const OPCODE_OP_ROR_BITS: u16 = 0b0000_0000_0000_0000;
const OPCODE_OP_ROR_MASK: u16 = 0b0000_0000_0000_0000;

// 97
const OPCODE_OP_SBC_BITS: u16 = 0b0000_0000_0000_0000;
const OPCODE_OP_SBC_MASK: u16 = 0b0000_0000_0000_0000;

// 98
const OPCODE_OP_SBCI_BITS: u16 = 0b0000_0000_0000_0000;
const OPCODE_OP_SBCI_MASK: u16 = 0b0000_0000_0000_0000;

// 99
const OPCODE_OP_SBI_BITS: u16 = 0b0000_0000_0000_0000;
const OPCODE_OP_SBI_MASK: u16 = 0b0000_0000_0000_0000;

// 100
const OPCODE_OP_SBIC_BITS: u16 = 0b0000_0000_0000_0000;
const OPCODE_OP_SBIC_MASK: u16 = 0b0000_0000_0000_0000;

// 101
const OPCODE_OP_SBIS_BITS: u16 = 0b0000_0000_0000_0000;
const OPCODE_OP_SBIS_MASK: u16 = 0b0000_0000_0000_0000;

// 102
const OPCODE_OP_SBIW_BITS: u16 = 0b0000_0000_0000_0000;
const OPCODE_OP_SBIW_MASK: u16 = 0b0000_0000_0000_0000;

// 104
const OPCODE_OP_SBRC_BITS: u16 = 0b0000_0000_0000_0000;
const OPCODE_OP_SBRC_MASK: u16 = 0b0000_0000_0000_0000;

// 105
const OPCODE_OP_SBRS_BITS: u16 = 0b0000_0000_0000_0000;
const OPCODE_OP_SBRS_MASK: u16 = 0b0000_0000_0000_0000;

// 110
const OPCODE_OP_SER_BITS: u16 = 0b0000_0000_0000_0000;
const OPCODE_OP_SER_MASK: u16 = 0b0000_0000_0000_0000;

// 115
const OPCODE_OP_SLEEP_BITS: u16 = 0b0000_0000_0000_0000;
const OPCODE_OP_SLEEP_MASK: u16 = 0b0000_0000_0000_0000;

// 116
const OPCODE_OP_SPM_BITS: u16 = 0b0000_0000_0000_0000;
const OPCODE_OP_SPM_MASK: u16 = 0b0000_0000_0000_0000;

// 117
const OPCODE_OP_SPM2_BITS: u16 = 0b0000_0000_0000_0000;
const OPCODE_OP_SPM2_MASK: u16 = 0b0000_0000_0000_0000;

// 118
const OPCODE_OP_STX_BITS: u16 = 0b0000_0000_0000_0000;
const OPCODE_OP_STX_MASK: u16 = 0b0000_0000_0000_0000;

// 119
const OPCODE_OP_STY_BITS: u16 = 0b0000_0000_0000_0000;
const OPCODE_OP_STY_MASK: u16 = 0b0000_0000_0000_0000;

// 120
const OPCODE_OP_STZ_BITS: u16 = 0b0000_0000_0000_0000;
const OPCODE_OP_STZ_MASK: u16 = 0b0000_0000_0000_0000;

// 121
const OPCODE_OP_STS_BITS: u16 = 0b0000_0000_0000_0000;
const OPCODE_OP_STS_MASK: u16 = 0b0000_0000_0000_0000;

// 122
const OPCODE_OP_STS16_BITS: u16 = 0b0000_0000_0000_0000;
const OPCODE_OP_STS16_MASK: u16 = 0b0000_0000_0000_0000;

// 123
const OPCODE_OP_SUB_BITS: u16 = 0b0000_0000_0000_0000;
const OPCODE_OP_SUB_MASK: u16 = 0b0000_0000_0000_0000;

// 124
const OPCODE_OP_SUBI_BITS: u16 = 0b0000_0000_0000_0000;
const OPCODE_OP_SUBI_MASK: u16 = 0b0000_0000_0000_0000;

// 125
const OPCODE_OP_SWAP_BITS: u16 = 0b0000_0000_0000_0000;
const OPCODE_OP_SWAP_MASK: u16 = 0b0000_0000_0000_0000;

// 127
const OPCODE_OP_WDR_BITS: u16 = 0b0000_0000_0000_0000;
const OPCODE_OP_WDR_MASK: u16 = 0b0000_0000_0000_0000;

fn decode(w0: u16, w1: u16) {}

pub mod core;
