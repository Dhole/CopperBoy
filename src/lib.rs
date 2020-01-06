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
const OPCODE_OP_EOR_BITS: u16 = 0b0010_0100_0000_0000;
const OPCODE_OP_EOR_MASK: u16 = 0b1111_1100_0000_0000;

// 59
const OPCODE_OP_FMUL_BITS: u16 = 0b0000_0011_0000_1000;
const OPCODE_OP_FMUL_MASK: u16 = 0b1111_1111_1000_1000;

// 60
const OPCODE_OP_FMULS_BITS: u16 = 0b0000_0011_1000_0000;
const OPCODE_OP_FMULS_MASK: u16 = 0b1111_1111_1000_1000;

// 61
const OPCODE_OP_FMULSU_BITS: u16 = 0b0000_0011_1000_1000;
const OPCODE_OP_FMULSU_MASK: u16 = 0b1111_1111_1000_1000;

// 62
const OPCODE_OP_ICALL_BITS: u16 = 0b1001_0101_0000_1001;
const OPCODE_OP_ICALL_MASK: u16 = 0b1111_1111_1111_1111;

// 63
const OPCODE_OP_IJMP_BITS: u16 = 0b1001_0100_0000_1001;
const OPCODE_OP_IJMP_MASK: u16 = 0b1111_1111_1111_1111;

// 64
const OPCODE_OP_IN_BITS: u16 = 0b1011_0000_0000_0000;
const OPCODE_OP_IN_MASK: u16 = 0b1111_1000_0000_0000;

// 65
const OPCODE_OP_INC_BITS: u16 = 0b1001_0100_0000_0011;
const OPCODE_OP_INC_MASK: u16 = 0b1111_1110_0000_1111;

// 66
const OPCODE_OP_JMP_BITS: u16 = 0b1001_0100_0000_1100;
const OPCODE_OP_JMP_MASK: u16 = 0b1111_1110_0000_1110;

// TODO
// 70
const OPCODE_OP_LDX_BITS: u16 = 0b0000_0000_0000_0000;
const OPCODE_OP_LDX_MASK: u16 = 0b0000_0000_0000_0000;

// TODO
// 71
const OPCODE_OP_LDY_BITS: u16 = 0b0000_0000_0000_0000;
const OPCODE_OP_LDY_MASK: u16 = 0b0000_0000_0000_0000;

// TODO
// 72
const OPCODE_OP_LDZ_BITS: u16 = 0b0000_0000_0000_0000;
const OPCODE_OP_LDZ_MASK: u16 = 0b0000_0000_0000_0000;

// 73
const OPCODE_OP_LDI_BITS: u16 = 0b1110_0000_0000_0000;
const OPCODE_OP_LDI_MASK: u16 = 0b1111_0000_0000_0000;

// 74
const OPCODE_OP_LDS_BITS: u16 = 0b1001_0000_0000_0000;
const OPCODE_OP_LDS_MASK: u16 = 0b1111_1110_0000_1111;

// 75
const OPCODE_OP_LDS16_BITS: u16 = 0b1010_0000_0000_0000;
const OPCODE_OP_LDS16_MASK: u16 = 0b1111_1000_0000_0000;

// TODO
// 76
const OPCODE_OP_LPM_BITS: u16 = 0b0000_0000_0000_0000;
const OPCODE_OP_LPM_MASK: u16 = 0b0000_0000_0000_0000;

// 78
const OPCODE_OP_LSR_BITS: u16 = 0b1001_0100_0000_0110;
const OPCODE_OP_LSR_MASK: u16 = 0b1111_1110_0000_1111;

// 79
const OPCODE_OP_MOV_BITS: u16 = 0b0010_1100_0000_0000;
const OPCODE_OP_MOV_MASK: u16 = 0b1111_1100_0000_0000;

// 80
const OPCODE_OP_MOVW_BITS: u16 = 0b0000_0001_0000_0000;
const OPCODE_OP_MOVW_MASK: u16 = 0b1111_1111_0000_0000;

// 81
const OPCODE_OP_MUL_BITS: u16 = 0b1001_1100_0000_0000;
const OPCODE_OP_MUL_MASK: u16 = 0b1111_1100_0000_0000;

// 82
const OPCODE_OP_MULS_BITS: u16 = 0b0000_0010_0000_0000;
const OPCODE_OP_MULS_MASK: u16 = 0b1111_1111_0000_0000;

// 83
const OPCODE_OP_MULSU_BITS: u16 = 0b0000_0011_0000_0000;
const OPCODE_OP_MULSU_MASK: u16 = 0b1111_1111_1000_1000;

// 84
const OPCODE_OP_NEG_BITS: u16 = 0b1001_0100_0000_0001;
const OPCODE_OP_NEG_MASK: u16 = 0b1111_1110_0000_1111;

// 85
const OPCODE_OP_NOP_BITS: u16 = 0b0000_0000_0000_0000;
const OPCODE_OP_NOP_MASK: u16 = 0b1111_1111_1111_1111;

// 86
const OPCODE_OP_OR_BITS: u16 = 0b0010_1000_0000_0000;
const OPCODE_OP_OR_MASK: u16 = 0b1111_1100_0000_0000;

// 87
const OPCODE_OP_ORI_BITS: u16 = 0b0110_0000_0000_0000;
const OPCODE_OP_ORI_MASK: u16 = 0b1111_0000_0000_0000;

// 88
const OPCODE_OP_OUT_BITS: u16 = 0b1011_1000_0000_0000;
const OPCODE_OP_OUT_MASK: u16 = 0b1111_1000_0000_0000;

// 89
const OPCODE_OP_POP_BITS: u16 = 0b1001_0000_0000_1111;
const OPCODE_OP_POP_MASK: u16 = 0b1111_1110_0000_1111;

// 90
const OPCODE_OP_PUSH_BITS: u16 = 0b1001_0010_0000_1111;
const OPCODE_OP_PUSH_MASK: u16 = 0b1111_1110_0000_1111;

// 91
const OPCODE_OP_RCALL_BITS: u16 = 0b1101_0000_0000_0000;
const OPCODE_OP_RCALL_MASK: u16 = 0b1111_0000_0000_0000;

// 92
const OPCODE_OP_RET_BITS: u16 = 0b1001_0101_0000_1000;
const OPCODE_OP_RET_MASK: u16 = 0b1111_1111_1111_1111;

// 93
const OPCODE_OP_RETI_BITS: u16 = 0b1001_0101_0001_1000;
const OPCODE_OP_RETI_MASK: u16 = 0b1111_1111_1111_1111;

// 94
const OPCODE_OP_RJMP_BITS: u16 = 0b1100_0000_0000_0000;
const OPCODE_OP_RJMP_MASK: u16 = 0b1111_0000_0000_0000;

// 96
const OPCODE_OP_ROR_BITS: u16 = 0b1001_0100_0000_0111;
const OPCODE_OP_ROR_MASK: u16 = 0b1111_1110_0000_1111;

// 97
const OPCODE_OP_SBC_BITS: u16 = 0b0000_1000_0000_0000;
const OPCODE_OP_SBC_MASK: u16 = 0b1111_1100_0000_0000;

// 98
const OPCODE_OP_SBCI_BITS: u16 = 0b0100_0000_0000_0000;
const OPCODE_OP_SBCI_MASK: u16 = 0b1111_0000_0000_0000;

// 99
const OPCODE_OP_SBI_BITS: u16 = 0b1001_1010_0000_0000;
const OPCODE_OP_SBI_MASK: u16 = 0b1111_1111_0000_0000;

// 100
const OPCODE_OP_SBIC_BITS: u16 = 0b1001_1001_0000_0000;
const OPCODE_OP_SBIC_MASK: u16 = 0b1111_1111_0000_0000;

// 101
const OPCODE_OP_SBIS_BITS: u16 = 0b1001_1011_0000_0000;
const OPCODE_OP_SBIS_MASK: u16 = 0b1111_1111_0000_0000;

// 102
const OPCODE_OP_SBIW_BITS: u16 = 0b1001_0111_0000_0000;
const OPCODE_OP_SBIW_MASK: u16 = 0b1111_1111_0000_0000;

// 104
const OPCODE_OP_SBRC_BITS: u16 = 0b0110_0000_0000_0000;
const OPCODE_OP_SBRC_MASK: u16 = 0b1111_0000_0000_0000;

// 105
const OPCODE_OP_SBRS_BITS: u16 = 0b1111_1100_0000_0000;
const OPCODE_OP_SBRS_MASK: u16 = 0b1111_1110_0000_1000;

// 110
const OPCODE_OP_SER_BITS: u16 = 0b1110_1111_0000_1111;
const OPCODE_OP_SER_MASK: u16 = 0b1111_1111_0000_1111;

// 115
const OPCODE_OP_SLEEP_BITS: u16 = 0b1001_0101_1000_1000;
const OPCODE_OP_SLEEP_MASK: u16 = 0b1111_1111_1111_1111;

// 116
const OPCODE_OP_SPM_BITS: u16 = 0b1001_0101_1110_1000;
const OPCODE_OP_SPM_MASK: u16 = 0b1111_1111_1111_1111;

// TODO
// 117
const OPCODE_OP_SPM2_BITS: u16 = 0b0000_0000_0000_0000;
const OPCODE_OP_SPM2_MASK: u16 = 0b0000_0000_0000_0000;

// TODO
// 118
const OPCODE_OP_STX_BITS: u16 = 0b0000_0000_0000_0000;
const OPCODE_OP_STX_MASK: u16 = 0b0000_0000_0000_0000;

// TODO
// 119
const OPCODE_OP_STY_BITS: u16 = 0b0000_0000_0000_0000;
const OPCODE_OP_STY_MASK: u16 = 0b0000_0000_0000_0000;

// TODO
// 120
const OPCODE_OP_STZ_BITS: u16 = 0b0000_0000_0000_0000;
const OPCODE_OP_STZ_MASK: u16 = 0b0000_0000_0000_0000;

// 121
const OPCODE_OP_STS_BITS: u16 = 0b1001_0010_0000_0000;
const OPCODE_OP_STS_MASK: u16 = 0b1111_1110_0000_1111;

// 122
const OPCODE_OP_STS16_BITS: u16 = 0b1010_1000_0000_0000;
const OPCODE_OP_STS16_MASK: u16 = 0b1111_1000_0000_0000;

// 123
const OPCODE_OP_SUB_BITS: u16 = 0b0001_1000_0000_0000;
const OPCODE_OP_SUB_MASK: u16 = 0b1111_1100_0000_0000;

// 124
const OPCODE_OP_SUBI_BITS: u16 = 0b0101_0000_0000_0000;
const OPCODE_OP_SUBI_MASK: u16 = 0b1111_0000_0000_0000;

// 125
const OPCODE_OP_SWAP_BITS: u16 = 0b1001_0100_0000_0010;
const OPCODE_OP_SWAP_MASK: u16 = 0b1111_1110_0000_1111;

// 127
const OPCODE_OP_WDR_BITS: u16 = 0b1001_0101_1010_1000;
const OPCODE_OP_WDR_MASK: u16 = 0b1111_1111_1111_1111;

enum Op {
    Adc { d: u8, r: u8 },
    Add { d: u8, r: u8 },
    Adiw { d: u8, k: u8 },
    And { d: u8, r: u8 },
    Andi { d: u8, k: u8 },
    Asr { d: u8 },
    Bclr { s: u8 },
    Bld { d: u8, b: u8 },
    Brbc { s: u8, k: i8 },
    Brbs { s: u8, k: i8 },
    Break,
    Bset { s: u8 },
    Bst { d: u8, b: u8 },
    Call { k: u32 },
    Cbi { a: u8, b: u8 },
    Clr { d: u8 },
    Com { d: u8 },
    Cp { d: u8, r: u8 },
    Cpc { d: u8, r: u8 },
    Cpi { d: u8, k: u8 },
    Cpse { d: u8, r: u8 },
    Dec { d: u8 },
    Eicall,
    Eijmp,
    Elpm, // TODO
    Eor { d: u8, r: u8 },
    Fmul { d: u8, r: u8 },
    Fmuls { d: u8, r: u8 },
    Fmulsu { d: u8, r: u8 },
    Icall,
    Ijmp,
    In { d: u8, a: u8 },
    Inc { d: u8 },
    Jmp { k: u32 },
    Ld, // TODO
    Ldi { d: u8, k: u8 },
    Lds,   // TODO
    Lds16, // TODO
    Lpm,   // TODO
    Lsr { d: u8 },
    Mov { d: u8, r: u8 },
    Movw { d: u8, r: u8 },
    Mul { d: u8, r: u8 },
    Muls { d: u8, r: u8 },
    Mulsu { d: u8, r: u8 },
    Neg { d: u8 },
    Nop,
    Or { d: u8, r: u8 },
    Ori { d: u8, k: u8 },
    Out, // TODO
    Pop { d: u8 },
    Push { r: u8 },
    Rcall { k: i16 },
    Ret,
    Reti,
    Rjmp { k: i16 },
    Ror { d: u8 },
    Sbc { d: u8, r: u8 },
    Sbci { d: u8, k: u8 },
    Sbi,  // TODO
    Sbic, // TODO
    Sbis, // TODO
    Sbiw, // TODO
    Sbrc { r: u8, b: u8 },
    Sbrs { r: u8, b: u8 },
    Ser { d: u8 },
    Sleep,
    Spm,   // TODO
    Spm2,  // TODO
    St,    // TODO
    Sts,   // TODO
    Sts16, // TODO
    Sub { d: u8, r: u8 },
    Subi { d: u8, k: u8 },
    Swap { d: u8 },
    Wdr,
}

fn decode(w0: u16, w1: u16) -> Op {
    match w0 {
        _ if (w0 & OPCODE_OP_ADC_MASK) == OPCODE_OP_ADC_BITS => Op::Adc {
            r: ((w0 & 0b0000_0010_0000_0000) >> 5 | w0 & 0b0000_0000_0000_1111) as u8,
            d: ((w0 & 0b0000_0001_1111_0000) >> 3) as u8,
        },
        _ if (w0 & OPCODE_OP_ADD_MASK) == OPCODE_OP_ADD_BITS => Op::Add {
            r: ((w0 & 0b0000_0010_0000_0000) >> 5 | w0 & 0b0000_0000_0000_1111) as u8,
            d: ((w0 & 0b0000_0001_1111_0000) >> 3) as u8,
        },
        _ if (w0 & OPCODE_OP_ADIW_MASK) == OPCODE_OP_ADIW_BITS => Op::Adiw {
            k: ((w0 & 0b0000_0000_1100_0000) >> 2 | w0 & 0b0000_0000_0000_1111) as u8,
            d: ((w0 & 0b0000_0000_0011_0000) >> 3) as u8,
        },
        _ if (w0 & OPCODE_OP_AND_MASK) == OPCODE_OP_AND_BITS => Op::And {
            r: ((w0 & 0b0000_0010_0000_0000) >> 5 | w0 & 0b0000_0000_0000_1111) as u8,
            d: ((w0 & 0b0000_0001_1111_0000) >> 3) as u8,
        },
        _ if (w0 & OPCODE_OP_ANDI_MASK) == OPCODE_OP_ANDI_BITS => Op::Andi {
            k: ((w0 & 0b0000_1111_0000_0000) >> 4 | w0 & 0b0000_0000_0000_1111) as u8,
            d: ((w0 & 0b0000_0000_1111_0000) >> 3) as u8,
        },
        _ if (w0 & OPCODE_OP_ASR_MASK) == OPCODE_OP_ASR_BITS => Op::Asr {
            d: ((w0 & 0b0000_0001_1111_0000) >> 3) as u8,
        },
        _ if (w0 & OPCODE_OP_BCLR_MASK) == OPCODE_OP_BCLR_BITS => Op::Bclr {
            s: ((w0 & 0b0000_0000_0111_0000) >> 3) as u8,
        },
        _ if (w0 & OPCODE_OP_BLD_MASK) == OPCODE_OP_BLD_BITS => Op::Bld {
            b: (w0 & 0b0000_0000_0000_0111) as u8,
            d: ((w0 & 0b0000_0001_1111_0000) >> 3) as u8,
        },
        _ if (w0 & OPCODE_OP_BRBC_MASK) == OPCODE_OP_BRBC_BITS => Op::Brbc {
            k: ((w0 & 0b0000_0011_1111_1000) >> 2) as i8,
            s: (w0 & 0b0000_0000_0000_0111) as u8,
        },
        _ if (w0 & OPCODE_OP_BRBS_MASK) == OPCODE_OP_BRBS_BITS => Op::Brbs {
            k: ((w0 & 0b0000_0011_1111_1000) >> 2) as i8,
            s: (w0 & 0b0000_0000_0000_0111) as u8,
        },
        _ if (w0 & OPCODE_OP_BREAK_MASK) == OPCODE_OP_BREAK_BITS => Op::Break,
        _ if (w0 & OPCODE_OP_BSET_MASK) == OPCODE_OP_BSET_BITS => Op::Bset {
            s: ((w0 & 0b0000_0000_0111_0000) >> 3) as u8,
        },
        _ if (w0 & OPCODE_OP_BST_MASK) == OPCODE_OP_BST_BITS => Op::Bst {
            b: (w0 & 0b0000_0000_0000_0111) as u8,
            d: ((w0 & 0b0000_0001_1111_0000) >> 3) as u8,
        },
        _ if (w0 & OPCODE_OP_CALL_MASK) == OPCODE_OP_CALL_BITS => Op::Call {
            k: ((w0 & 0b0000_0001_1111_0000) as u32 >> 3 | (w0 & 0b0000_0000_0000_0001) as u32)
                >> 16
                | (w1 as u32),
        },
        _ if (w0 & OPCODE_OP_CBI_MASK) == OPCODE_OP_CBI_BITS => Op::Cbi {
            b: (w0 & 0b0000_0000_0000_0111) as u8,
            a: ((w0 & 0b0000_0000_1111_1000) >> 2) as u8,
        },
        _ if (w0 & OPCODE_OP_CLR_MASK) == OPCODE_OP_CLR_BITS => Op::Clr {
            d: (w0 & 0b0000_0011_1111_1111) as u8,
        },
        _ if (w0 & OPCODE_OP_COM_MASK) == OPCODE_OP_COM_BITS => Op::Com {
            d: ((w0 & 0b0000_0001_1111_0000) >> 3) as u8,
        },
        _ if (w0 & OPCODE_OP_CP_MASK) == OPCODE_OP_CP_BITS => Op::Cp {
            r: ((w0 & 0b0000_0010_0000_0000) >> 5 | w0 & 0b0000_0000_0000_1111) as u8,
            d: ((w0 & 0b0000_0001_1111_0000) >> 3) as u8,
        },
        _ if (w0 & OPCODE_OP_CPC_MASK) == OPCODE_OP_CPC_BITS => Op::Cpc {
            r: ((w0 & 0b0000_0010_0000_0000) >> 5 | w0 & 0b0000_0000_0000_1111) as u8,
            d: ((w0 & 0b0000_0001_1111_0000) >> 3) as u8,
        },
        _ if (w0 & OPCODE_OP_CPI_MASK) == OPCODE_OP_CPI_BITS => Op::Cpi {
            k: ((w0 & 0b0000_1111_0000_0000) >> 4 | w0 & 0b0000_0000_0000_1111) as u8,
            d: ((w0 & 0b0000_0000_1111_0000) >> 3) as u8,
        },
        _ if (w0 & OPCODE_OP_CPSE_MASK) == OPCODE_OP_CPSE_BITS => Op::Cpse {
            r: ((w0 & 0b0000_0010_0000_0000) >> 5 | w0 & 0b0000_0000_0000_1111) as u8,
            d: ((w0 & 0b0000_0001_1111_0000) >> 3) as u8,
        },
        _ if (w0 & OPCODE_OP_DEC_MASK) == OPCODE_OP_DEC_BITS => Op::Dec {
            d: ((w0 & 0b0000_0001_1111_0000) >> 3) as u8,
        },
        _ if (w0 & OPCODE_OP_EICALL_MASK) == OPCODE_OP_EICALL_BITS => Op::Eicall,
        _ if (w0 & OPCODE_OP_EIJMP_MASK) == OPCODE_OP_EIJMP_BITS => Op::Eijmp,
        _ if (w0 & OPCODE_OP_EOR_MASK) == OPCODE_OP_EOR_BITS => Op::Eor {
            r: ((w0 & 0b0000_0010_0000_0000) >> 5 | w0 & 0b0000_0000_0000_1111) as u8,
            d: ((w0 & 0b0000_0001_1111_0000) >> 3) as u8,
        },
        _ if (w0 & OPCODE_OP_FMUL_MASK) == OPCODE_OP_FMUL_BITS => Op::Fmul {
            r: (w0 & 0b0000_0000_0000_0111) as u8,
            d: ((w0 & 0b0000_0000_0111_0000) >> 3) as u8,
        },
        _ if (w0 & OPCODE_OP_FMULS_MASK) == OPCODE_OP_FMULS_BITS => Op::Fmuls {
            r: (w0 & 0b0000_0000_0000_0111) as u8,
            d: ((w0 & 0b0000_0000_0111_0000) >> 3) as u8,
        },
        _ if (w0 & OPCODE_OP_FMULSU_MASK) == OPCODE_OP_FMULSU_BITS => Op::Fmulsu {
            r: (w0 & 0b0000_0000_0000_0111) as u8,
            d: ((w0 & 0b0000_0000_0111_0000) >> 3) as u8,
        },
        _ if (w0 & OPCODE_OP_ICALL_MASK) == OPCODE_OP_ICALL_BITS => Op::Icall,
        _ if (w0 & OPCODE_OP_IJMP_MASK) == OPCODE_OP_IJMP_BITS => Op::Ijmp,
        _ if (w0 & OPCODE_OP_IN_MASK) == OPCODE_OP_IN_BITS => Op::In {
            d: ((w0 & 0b0000_0001_1111_0000) >> 3) as u8,
            a: ((w0 & 0b0000_0110_0000_0000) >> 5 | w0 & 0b0000_0000_0000_1111) as u8,
        },
        _ if (w0 & OPCODE_OP_INC_MASK) == OPCODE_OP_INC_BITS => Op::Inc {
            d: ((w0 & 0b0000_0001_1111_0000) >> 3) as u8,
        },
        _ if (w0 & OPCODE_OP_JMP_MASK) == OPCODE_OP_JMP_BITS => Op::Jmp {
            k: ((w0 & 0b0000_0001_1111_0000) as u32 >> 3 | (w0 & 0b0000_0000_0000_0001) as u32)
                >> 16
                | (w1 as u32),
        },
        _ if (w0 & OPCODE_OP_LDI_MASK) == OPCODE_OP_LDI_BITS => Op::Ldi {
            k: ((w0 & 0b0000_1111_0000_0000) >> 4 | w0 & 0b0000_0000_0000_1111) as u8,
            d: ((w0 & 0b0000_0000_1111_0000) >> 3) as u8,
        },
        // TODO
        // _ if (w0 & OPCODE_OP_LDS_MASK) == OPCODE_OP_LDS_BITS => Op::Lds {
        //     d: ((w0 & 0b0000_0001_1111_0000) >> 3) as u8,
        // },
        // TODO
        // _ if (w0 & OPCODE_OP_LDS16_MASK) == OPCODE_OP_LDS16_BITS => Op::Lds16 {
        //     k: ((w0 & 0b0000_0111_0000_0000) >> 4 | w0 & 0b0000_0000_0000_1111) as u8,
        //     d: ((w0 & 0b0000_0000_1111_0000) >> 3) as u8,
        // },
        _ if (w0 & OPCODE_OP_LSR_MASK) == OPCODE_OP_LSR_BITS => Op::Lsr {
            d: ((w0 & 0b0000_0001_1111_0000) >> 3) as u8,
        },
        _ if (w0 & OPCODE_OP_MOV_MASK) == OPCODE_OP_MOV_BITS => Op::Mov {
            r: ((w0 & 0b0000_0010_0000_0000) >> 5 | w0 & 0b0000_0000_0000_1111) as u8,
            d: ((w0 & 0b0000_0001_1111_0000) >> 3) as u8,
        },
        _ if (w0 & OPCODE_OP_MOVW_MASK) == OPCODE_OP_MOVW_BITS => Op::Movw {
            r: (w0 & 0b0000_0000_0000_1111) as u8,
            d: ((w0 & 0b0000_0000_1111_0000) >> 3) as u8,
        },
        _ if (w0 & OPCODE_OP_MUL_MASK) == OPCODE_OP_MUL_BITS => Op::Mul {
            r: ((w0 & 0b0000_0010_0000_0000) >> 5 | w0 & 0b0000_0000_0000_1111) as u8,
            d: ((w0 & 0b0000_0001_1111_0000) >> 3) as u8,
        },
        _ if (w0 & OPCODE_OP_MULS_MASK) == OPCODE_OP_MULS_BITS => Op::Muls {
            r: (w0 & 0b0000_0000_0000_1111) as u8,
            d: ((w0 & 0b0000_0000_1111_0000) >> 3) as u8,
        },
        _ if (w0 & OPCODE_OP_MULSU_MASK) == OPCODE_OP_MULSU_BITS => Op::Mulsu {
            r: (w0 & 0b0000_0000_0000_0111) as u8,
            d: ((w0 & 0b0000_0000_0111_0000) >> 3) as u8,
        },
        _ if (w0 & OPCODE_OP_NEG_MASK) == OPCODE_OP_NEG_BITS => Op::Neg {
            d: ((w0 & 0b0000_0001_1111_0000) >> 3) as u8,
        },
        _ if (w0 & OPCODE_OP_NOP_MASK) == OPCODE_OP_NOP_BITS => Op::Nop,
        _ if (w0 & OPCODE_OP_OR_MASK) == OPCODE_OP_OR_BITS => Op::Or {
            r: ((w0 & 0b0000_0010_0000_0000) >> 5 | w0 & 0b0000_0000_0000_1111) as u8,
            d: ((w0 & 0b0000_0001_1111_0000) >> 3) as u8,
        },
        _ if (w0 & OPCODE_OP_ORI_MASK) == OPCODE_OP_ORI_BITS => Op::Ori {
            k: ((w0 & 0b0000_1111_0000_0000) >> 4 | w0 & 0b0000_0000_0000_1111) as u8,
            d: ((w0 & 0b0000_0000_1111_0000) >> 3) as u8,
        },
        // TODO
        // _ if (w0 & OPCODE_OP_OUT_MASK) == OPCODE_OP_OUT_BITS => Op::Out {
        //     r: ((w0 & 0b0000_0001_1111_0000) >> 3) as u8,
        //     a: ((w0 & 0b0000_0110_0000_0000) >> 5 | w0 & 0b0000_0000_0000_1111) as u8,
        // },
        _ if (w0 & OPCODE_OP_POP_MASK) == OPCODE_OP_POP_BITS => Op::Pop {
            d: ((w0 & 0b0000_0001_1111_0000) >> 3) as u8,
        },
        _ if (w0 & OPCODE_OP_PUSH_MASK) == OPCODE_OP_PUSH_BITS => Op::Push {
            r: ((w0 & 0b0000_0001_1111_0000) >> 3) as u8,
        },
        _ if (w0 & OPCODE_OP_RCALL_MASK) == OPCODE_OP_RCALL_BITS => Op::Rcall {
            k: (w0 & 0b0000_1111_1111_1111) as i16,
        },
        _ if (w0 & OPCODE_OP_RET_MASK) == OPCODE_OP_RET_BITS => Op::Ret,
        _ if (w0 & OPCODE_OP_RETI_MASK) == OPCODE_OP_RETI_BITS => Op::Reti,
        _ if (w0 & OPCODE_OP_RJMP_MASK) == OPCODE_OP_RJMP_BITS => Op::Rjmp {
            k: (w0 & 0b0000_1111_1111_1111) as i16,
        },
        _ if (w0 & OPCODE_OP_ROR_MASK) == OPCODE_OP_ROR_BITS => Op::Ror {
            d: ((w0 & 0b0000_0001_1111_0000) >> 3) as u8,
        },
        _ if (w0 & OPCODE_OP_SBC_MASK) == OPCODE_OP_SBC_BITS => Op::Sbc {
            r: ((w0 & 0b0000_0010_0000_0000) >> 5 | w0 & 0b0000_0000_0000_1111) as u8,
            d: ((w0 & 0b0000_0001_1111_0000) >> 3) as u8,
        },
        _ if (w0 & OPCODE_OP_SBCI_MASK) == OPCODE_OP_SBCI_BITS => Op::Sbci {
            k: ((w0 & 0b0000_1111_0000_0000) >> 4 | w0 & 0b0000_0000_0000_1111) as u8,
            d: ((w0 & 0b0000_0000_1111_0000) >> 3) as u8,
        },
        // TODO
        // _ if (w0 & OPCODE_OP_SBI_MASK) == OPCODE_OP_SBI_BITS => Op::Sbi {
        //     b: (w0 & 0b0000_0000_0000_0111) as u8,
        //     a: ((w0 & 0b0000_0000_1111_1000) >> 2) as u8,
        // },
        // TODO
        // _ if (w0 & OPCODE_OP_SBIC_MASK) == OPCODE_OP_SBIC_BITS => Op::Sbic {
        //     b: (w0 & 0b0000_0000_0000_0111) as u8,
        //     a: ((w0 & 0b0000_0000_1111_1000) >> 2) as u8,
        // },
        // TODO
        // _ if (w0 & OPCODE_OP_SBIS_MASK) == OPCODE_OP_SBIS_BITS => Op::Sbis {
        //     b: (w0 & 0b0000_0000_0000_0111) as u8,
        //     a: ((w0 & 0b0000_0000_1111_1000) >> 2) as u8,
        // },
        // TODO
        // _ if (w0 & OPCODE_OP_SBIW_MASK) == OPCODE_OP_SBIW_BITS => Op::Sbiw {
        //     k: ((w0 & 0b0000_0000_1100_0000) >> 2 | w0 & 0b0000_0000_0000_1111) as u8,
        //     d: ((w0 & 0b0000_0000_0011_0000) >> 3) as u8,
        // },
        _ if (w0 & OPCODE_OP_SBRC_MASK) == OPCODE_OP_SBRC_BITS => Op::Sbrc {
            b: (w0 & 0b0000_0000_0000_0111) as u8,
            r: ((w0 & 0b0000_0001_1111_0000) >> 3) as u8,
        },
        _ if (w0 & OPCODE_OP_SBRS_MASK) == OPCODE_OP_SBRS_BITS => Op::Sbrs {
            b: (w0 & 0b0000_0000_0000_0111) as u8,
            r: ((w0 & 0b0000_0001_1111_0000) >> 3) as u8,
        },
        _ if (w0 & OPCODE_OP_SER_MASK) == OPCODE_OP_SER_BITS => Op::Ser {
            d: ((w0 & 0b0000_0000_1111_0000) >> 3) as u8,
        },
        _ if (w0 & OPCODE_OP_SLEEP_MASK) == OPCODE_OP_SLEEP_BITS => Op::Sleep,
        _ if (w0 & OPCODE_OP_SPM_MASK) == OPCODE_OP_SPM_BITS => Op::Spm,
        // TODO
        //_ if (w0 & OPCODE_OP_STS_MASK) == OPCODE_OP_STS_BITS => Op::Sts {
        //    d: ((w0 & 0b0000_0001_1111_0000) >> 3) as u8,
        //},
        // TODO
        //_ if (w0 & OPCODE_OP_STS16_MASK) == OPCODE_OP_STS16_BITS => Op::Sts16 {
        //    k: ((w0 & 0b0000_0111_0000_0000) >> 4 | w0 & 0b0000_0000_0000_1111) as u8,
        //    d: ((w0 & 0b0000_0000_1111_0000) >> 3) as u8,
        //},
        _ if (w0 & OPCODE_OP_SUB_MASK) == OPCODE_OP_SUB_BITS => Op::Sub {
            r: ((w0 & 0b0000_0010_0000_0000) >> 5 | w0 & 0b0000_0000_0000_1111) as u8,
            d: ((w0 & 0b0000_0001_1111_0000) >> 3) as u8,
        },
        _ if (w0 & OPCODE_OP_SUBI_MASK) == OPCODE_OP_SUBI_BITS => Op::Subi {
            k: ((w0 & 0b0000_1111_0000_0000) >> 4 | w0 & 0b0000_0000_0000_1111) as u8,
            d: ((w0 & 0b0000_0000_1111_0000) >> 3) as u8,
        },
        _ if (w0 & OPCODE_OP_SWAP_MASK) == OPCODE_OP_SWAP_BITS => Op::Swap {
            d: ((w0 & 0b0000_0001_1111_0000) >> 3) as u8,
        },
        _ if (w0 & OPCODE_OP_WDR_MASK) == OPCODE_OP_WDR_BITS => Op::Wdr,
        _ => unreachable!(),
    }
}

pub mod core;
