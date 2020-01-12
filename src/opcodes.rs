use super::core::IOSPACE_ADDR;
use super::io_regs::io_reg_str;
use std::fmt;

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

// 57
const OPCODE_OP_ELPMR0_BITS: u16 = 0b1001_0101_1101_1000;
const OPCODE_OP_ELPMR0_MASK: u16 = 0b1111_1111_1111_1111;

// 57
const OPCODE_OP_ELPM_BITS: u16 = 0b1001_0000_0000_0110;
const OPCODE_OP_ELPM_MASK: u16 = 0b1111_1110_0000_1111;

// 57
const OPCODE_OP_ELPMINC_BITS: u16 = 0b1001_0000_0000_0111;
const OPCODE_OP_ELPMINC_MASK: u16 = 0b1111_1110_0000_1111;

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

// 70
const OPCODE_OP_LDX_BITS: u16 = 0b1001_0000_0000_1100;
const OPCODE_OP_LDX_MASK: u16 = 0b1111_1110_0000_1111;

// 70
const OPCODE_OP_LDXINC_BITS: u16 = 0b1001_0000_0000_1101;
const OPCODE_OP_LDXINC_MASK: u16 = 0b1111_1110_0000_1111;

// 70
const OPCODE_OP_LDXDEC_BITS: u16 = 0b1001_0000_0000_1111;
const OPCODE_OP_LDXDEC_MASK: u16 = 0b1111_1110_0000_1111;

// 71
const OPCODE_OP_LDYINC_BITS: u16 = 0b1001_0000_0000_1001;
const OPCODE_OP_LDYINC_MASK: u16 = 0b1111_1110_0000_1111;

// 71
const OPCODE_OP_LDYDEC_BITS: u16 = 0b1001_0000_0000_1010;
const OPCODE_OP_LDYDEC_MASK: u16 = 0b1111_1110_0000_1111;

// 71
const OPCODE_OP_LDYADQ_BITS: u16 = 0b1000_0000_0000_1000;
const OPCODE_OP_LDYADQ_MASK: u16 = 0b1101_0010_0000_1000;

// 72
const OPCODE_OP_LDZINC_BITS: u16 = 0b1001_0000_0000_0001;
const OPCODE_OP_LDZINC_MASK: u16 = 0b1111_1110_0000_1111;

// 72
const OPCODE_OP_LDZDEC_BITS: u16 = 0b1001_0000_0000_0010;
const OPCODE_OP_LDZDEC_MASK: u16 = 0b1111_1110_0000_1111;

// 72
const OPCODE_OP_LDZADQ_BITS: u16 = 0b1000_0000_0000_0000;
const OPCODE_OP_LDZADQ_MASK: u16 = 0b1101_0010_0000_1000;

// 73
const OPCODE_OP_LDI_BITS: u16 = 0b1110_0000_0000_0000;
const OPCODE_OP_LDI_MASK: u16 = 0b1111_0000_0000_0000;

// 74
const OPCODE_OP_LDS_BITS: u16 = 0b1001_0000_0000_0000;
const OPCODE_OP_LDS_MASK: u16 = 0b1111_1110_0000_1111;

// 76
const OPCODE_OP_LPMR0_BITS: u16 = 0b1001_0101_1100_1000;
const OPCODE_OP_LPMR0_MASK: u16 = 0b1111_1111_1111_1111;

// 76
const OPCODE_OP_LPM_BITS: u16 = 0b1001_0000_0000_0100;
const OPCODE_OP_LPM_MASK: u16 = 0b1111_1110_0000_1111;

// 76
const OPCODE_OP_LPMINC_BITS: u16 = 0b1001_0000_0000_0101;
const OPCODE_OP_LPMINC_MASK: u16 = 0b1111_1110_0000_1111;

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

// 117
const OPCODE_OP_SPM2_BITS: u16 = 0b1001_0101_1111_1000;
const OPCODE_OP_SPM2_MASK: u16 = 0b1111_1111_1111_1111;

// 118
const OPCODE_OP_STX_BITS: u16 = 0b1001_0010_0000_1100;
const OPCODE_OP_STX_MASK: u16 = 0b1111_1110_0000_1111;

// 118
const OPCODE_OP_STXINC_BITS: u16 = 0b1001_0010_0000_1101;
const OPCODE_OP_STXINC_MASK: u16 = 0b1111_1110_0000_1111;

// 118
const OPCODE_OP_STXDEC_BITS: u16 = 0b1001_0010_0000_1111;
const OPCODE_OP_STXDEC_MASK: u16 = 0b1111_1110_0000_1111;

// 119
const OPCODE_OP_STYINC_BITS: u16 = 0b1001_0010_0000_1001;
const OPCODE_OP_STYINC_MASK: u16 = 0b1111_1110_0000_1111;

// 119
const OPCODE_OP_STYDEC_BITS: u16 = 0b1001_0010_0000_1010;
const OPCODE_OP_STYDEC_MASK: u16 = 0b1111_1110_0000_1111;

// 119
const OPCODE_OP_STYADQ_BITS: u16 = 0b1000_0010_0000_1000;
const OPCODE_OP_STYADQ_MASK: u16 = 0b1101_0010_0000_1000;

// 120
const OPCODE_OP_STZINC_BITS: u16 = 0b1001_0010_0000_0001;
const OPCODE_OP_STZINC_MASK: u16 = 0b1111_1110_0000_1111;

// 120
const OPCODE_OP_STZDEC_BITS: u16 = 0b1001_0010_0000_0010;
const OPCODE_OP_STZDEC_MASK: u16 = 0b1111_1110_0000_1111;

// 120
const OPCODE_OP_STZADQ_BITS: u16 = 0b1000_0010_0000_0000;
const OPCODE_OP_STZADQ_MASK: u16 = 0b1101_0010_0000_1000;

// 121
const OPCODE_OP_STS_BITS: u16 = 0b1001_0010_0000_0000;
const OPCODE_OP_STS_MASK: u16 = 0b1111_1110_0000_1111;

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

#[derive(Debug, Clone, Copy)]
pub enum LdStIndex {
    X,
    Y,
    Z,
}

impl Into<u8> for LdStIndex {
    fn into(self) -> u8 {
        match self {
            Self::X => 26,
            Self::Y => 28,
            Self::Z => 30,
        }
    }
}

impl fmt::Display for LdStIndex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::X => write!(f, "X"),
            Self::Y => write!(f, "Y"),
            Self::Z => write!(f, "Z"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum LdStExt {
    None,
    PostInc,
    PreDec,
    Displacement(u8),
}

// NOTE: Review undefined combinations
#[derive(Debug, Clone)]
pub enum Op {
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
    Com { d: u8 },
    Cp { d: u8, r: u8 },
    Cpc { d: u8, r: u8 },
    Cpi { d: u8, k: u8 },
    Cpse { d: u8, r: u8 },
    Dec { d: u8 },
    Eicall,
    Eijmp,
    Elpmr0,
    Elpm { d: u8, inc: bool },
    Eor { d: u8, r: u8 },
    Fmul { d: u8, r: u8 },
    Fmuls { d: u8, r: u8 },
    Fmulsu { d: u8, r: u8 },
    Icall,
    Ijmp,
    In { d: u8, a: u8 },
    Inc { d: u8 },
    Jmp { k: u32 },
    Ld { d: u8, idx: LdStIndex, ext: LdStExt }, // NOTE: Review undefined Rd combinations
    Ldi { d: u8, k: u8 },
    Lds { d: u8, k: u16 },
    Lpmr0,
    Lpm { d: u8, inc: bool },
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
    Out { a: u8, r: u8 },
    Pop { d: u8 },
    Push { r: u8 },
    Rcall { k: i16 },
    Ret,
    Reti,
    Rjmp { k: i16 },
    Ror { d: u8 },
    Sbc { d: u8, r: u8 },
    Sbci { d: u8, k: u8 },
    Sbi { a: u8, b: u8 },
    Sbic { a: u8, b: u8 },
    Sbis { a: u8, b: u8 },
    Sbiw { d: u8, k: u8 },
    Sbrc { r: u8, b: u8 },
    Sbrs { r: u8, b: u8 },
    Ser { d: u8 },
    Sleep,
    Spm,
    Spm2,
    St { r: u8, idx: LdStIndex, ext: LdStExt },
    Sts { k: u16, r: u8 },
    Sub { d: u8, r: u8 },
    Subi { d: u8, k: u8 },
    Swap { d: u8 },
    Wdr,
    Undefined { w: u16 },
}

impl Op {
    pub fn words(&self) -> u8 {
        match self {
            Self::Call { .. } => 2,
            Self::Jmp { .. } => 2,
            Self::Lds { .. } => 2,
            Self::Sts { .. } => 2,
            _ => 1,
        }
    }

    pub fn decode(w0: u16, w1: u16) -> Self {
        match w0 {
            _ if (w0 & OPCODE_OP_ADC_MASK) == OPCODE_OP_ADC_BITS => Self::Adc {
                r: ((w0 & 0b0000_0010_0000_0000) >> 5 | w0 & 0b0000_0000_0000_1111) as u8,
                d: ((w0 & 0b0000_0001_1111_0000) >> 4) as u8,
            },
            _ if (w0 & OPCODE_OP_ADD_MASK) == OPCODE_OP_ADD_BITS => Self::Add {
                r: ((w0 & 0b0000_0010_0000_0000) >> 5 | w0 & 0b0000_0000_0000_1111) as u8,
                d: ((w0 & 0b0000_0001_1111_0000) >> 4) as u8,
            },
            _ if (w0 & OPCODE_OP_ADIW_MASK) == OPCODE_OP_ADIW_BITS => Self::Adiw {
                k: ((w0 & 0b0000_0000_1100_0000) >> 2 | w0 & 0b0000_0000_0000_1111) as u8,
                d: (((w0 & 0b0000_0000_0011_0000) >> 4) as u8) * 2 + 24,
            },
            _ if (w0 & OPCODE_OP_AND_MASK) == OPCODE_OP_AND_BITS => Self::And {
                r: ((w0 & 0b0000_0010_0000_0000) >> 5 | w0 & 0b0000_0000_0000_1111) as u8,
                d: ((w0 & 0b0000_0001_1111_0000) >> 4) as u8,
            },
            _ if (w0 & OPCODE_OP_ANDI_MASK) == OPCODE_OP_ANDI_BITS => Self::Andi {
                k: ((w0 & 0b0000_1111_0000_0000) >> 4 | w0 & 0b0000_0000_0000_1111) as u8,
                d: ((w0 & 0b0000_0000_1111_0000) >> 4) as u8 + 16,
            },
            _ if (w0 & OPCODE_OP_ASR_MASK) == OPCODE_OP_ASR_BITS => Self::Asr {
                d: ((w0 & 0b0000_0001_1111_0000) >> 4) as u8,
            },
            _ if (w0 & OPCODE_OP_BCLR_MASK) == OPCODE_OP_BCLR_BITS => Self::Bclr {
                s: ((w0 & 0b0000_0000_0111_0000) >> 4) as u8,
            },
            _ if (w0 & OPCODE_OP_BLD_MASK) == OPCODE_OP_BLD_BITS => Self::Bld {
                b: (w0 & 0b0000_0000_0000_0111) as u8,
                d: ((w0 & 0b0000_0001_1111_0000) >> 4) as u8,
            },
            _ if (w0 & OPCODE_OP_BRBC_MASK) == OPCODE_OP_BRBC_BITS => Self::Brbc {
                k: (((w0 & 0b0000_0011_1111_1000) >> 2) as i8) >> 1,
                s: (w0 & 0b0000_0000_0000_0111) as u8,
            },
            _ if (w0 & OPCODE_OP_BRBS_MASK) == OPCODE_OP_BRBS_BITS => Self::Brbs {
                k: (((w0 & 0b0000_0011_1111_1000) >> 2) as i8) >> 1,
                s: (w0 & 0b0000_0000_0000_0111) as u8,
            },
            _ if (w0 & OPCODE_OP_BREAK_MASK) == OPCODE_OP_BREAK_BITS => Self::Break,
            _ if (w0 & OPCODE_OP_BSET_MASK) == OPCODE_OP_BSET_BITS => Self::Bset {
                s: ((w0 & 0b0000_0000_0111_0000) >> 4) as u8,
            },
            _ if (w0 & OPCODE_OP_BST_MASK) == OPCODE_OP_BST_BITS => Self::Bst {
                b: (w0 & 0b0000_0000_0000_0111) as u8,
                d: ((w0 & 0b0000_0001_1111_0000) >> 4) as u8,
            },
            _ if (w0 & OPCODE_OP_CALL_MASK) == OPCODE_OP_CALL_BITS => Self::Call {
                k: ((w0 & 0b0000_0001_1111_0000) as u32 >> 4 | (w0 & 0b0000_0000_0000_0001) as u32)
                    >> 16
                    | (w1 as u32),
            },
            _ if (w0 & OPCODE_OP_CBI_MASK) == OPCODE_OP_CBI_BITS => Self::Cbi {
                b: (w0 & 0b0000_0000_0000_0111) as u8,
                a: ((w0 & 0b0000_0000_1111_1000) >> 3) as u8,
            },
            _ if (w0 & OPCODE_OP_COM_MASK) == OPCODE_OP_COM_BITS => Self::Com {
                d: ((w0 & 0b0000_0001_1111_0000) >> 4) as u8,
            },
            _ if (w0 & OPCODE_OP_CP_MASK) == OPCODE_OP_CP_BITS => Self::Cp {
                r: ((w0 & 0b0000_0010_0000_0000) >> 5 | w0 & 0b0000_0000_0000_1111) as u8,
                d: ((w0 & 0b0000_0001_1111_0000) >> 4) as u8,
            },
            _ if (w0 & OPCODE_OP_CPC_MASK) == OPCODE_OP_CPC_BITS => Self::Cpc {
                r: ((w0 & 0b0000_0010_0000_0000) >> 5 | w0 & 0b0000_0000_0000_1111) as u8,
                d: ((w0 & 0b0000_0001_1111_0000) >> 4) as u8,
            },
            _ if (w0 & OPCODE_OP_CPI_MASK) == OPCODE_OP_CPI_BITS => Self::Cpi {
                k: ((w0 & 0b0000_1111_0000_0000) >> 4 | w0 & 0b0000_0000_0000_1111) as u8,
                d: ((w0 & 0b0000_0000_1111_0000) >> 4) as u8 + 16,
            },
            _ if (w0 & OPCODE_OP_CPSE_MASK) == OPCODE_OP_CPSE_BITS => Self::Cpse {
                r: ((w0 & 0b0000_0010_0000_0000) >> 5 | w0 & 0b0000_0000_0000_1111) as u8,
                d: ((w0 & 0b0000_0001_1111_0000) >> 4) as u8,
            },
            _ if (w0 & OPCODE_OP_DEC_MASK) == OPCODE_OP_DEC_BITS => Self::Dec {
                d: ((w0 & 0b0000_0001_1111_0000) >> 4) as u8,
            },
            _ if (w0 & OPCODE_OP_EICALL_MASK) == OPCODE_OP_EICALL_BITS => Self::Eicall,
            _ if (w0 & OPCODE_OP_EIJMP_MASK) == OPCODE_OP_EIJMP_BITS => Self::Eijmp,
            _ if (w0 & OPCODE_OP_ELPMR0_MASK) == OPCODE_OP_ELPMR0_BITS => Op::Elpmr0,
            _ if (w0 & OPCODE_OP_ELPM_MASK) == OPCODE_OP_ELPM_BITS => Op::Elpm {
                d: ((w0 & 0b0000_0001_1111_0000) >> 4) as u8,
                inc: false,
            },
            _ if (w0 & OPCODE_OP_ELPMINC_MASK) == OPCODE_OP_ELPMINC_BITS => Op::Elpm {
                d: ((w0 & 0b0000_0001_1111_0000) >> 4) as u8,
                inc: true,
            },
            _ if (w0 & OPCODE_OP_EOR_MASK) == OPCODE_OP_EOR_BITS => Self::Eor {
                r: ((w0 & 0b0000_0010_0000_0000) >> 5 | w0 & 0b0000_0000_0000_1111) as u8,
                d: ((w0 & 0b0000_0001_1111_0000) >> 4) as u8,
            },
            _ if (w0 & OPCODE_OP_FMUL_MASK) == OPCODE_OP_FMUL_BITS => Self::Fmul {
                r: (w0 & 0b0000_0000_0000_0111) as u8 + 16,
                d: ((w0 & 0b0000_0000_0111_0000) >> 4) as u8 + 16,
            },
            _ if (w0 & OPCODE_OP_FMULS_MASK) == OPCODE_OP_FMULS_BITS => Self::Fmuls {
                r: (w0 & 0b0000_0000_0000_0111) as u8 + 16,
                d: ((w0 & 0b0000_0000_0111_0000) >> 4) as u8 + 16,
            },
            _ if (w0 & OPCODE_OP_FMULSU_MASK) == OPCODE_OP_FMULSU_BITS => Self::Fmulsu {
                r: (w0 & 0b0000_0000_0000_0111) as u8 + 16,
                d: ((w0 & 0b0000_0000_0111_0000) >> 4) as u8 + 16,
            },
            _ if (w0 & OPCODE_OP_ICALL_MASK) == OPCODE_OP_ICALL_BITS => Self::Icall,
            _ if (w0 & OPCODE_OP_IJMP_MASK) == OPCODE_OP_IJMP_BITS => Self::Ijmp,
            _ if (w0 & OPCODE_OP_IN_MASK) == OPCODE_OP_IN_BITS => Self::In {
                d: ((w0 & 0b0000_0001_1111_0000) >> 4) as u8,
                a: ((w0 & 0b0000_0110_0000_0000) >> 5 | w0 & 0b0000_0000_0000_1111) as u8,
            },
            _ if (w0 & OPCODE_OP_INC_MASK) == OPCODE_OP_INC_BITS => Self::Inc {
                d: ((w0 & 0b0000_0001_1111_0000) >> 4) as u8,
            },
            _ if (w0 & OPCODE_OP_JMP_MASK) == OPCODE_OP_JMP_BITS => Self::Jmp {
                k: ((w0 & 0b0000_0001_1111_0000) as u32 >> 4 | (w0 & 0b0000_0000_0000_0001) as u32)
                    >> 16
                    | (w1 as u32),
            },
            _ if (w0 & OPCODE_OP_LDX_MASK) == OPCODE_OP_LDX_BITS => Op::Ld {
                d: ((w0 & 0b0000_0001_1111_0000) >> 4) as u8,
                idx: LdStIndex::X,
                ext: LdStExt::None,
            },
            _ if (w0 & OPCODE_OP_LDXINC_MASK) == OPCODE_OP_LDXINC_BITS => Op::Ld {
                d: ((w0 & 0b0000_0001_1111_0000) >> 4) as u8,
                idx: LdStIndex::X,
                ext: LdStExt::PostInc,
            },
            _ if (w0 & OPCODE_OP_LDXDEC_MASK) == OPCODE_OP_LDXDEC_BITS => Op::Ld {
                d: ((w0 & 0b0000_0001_1111_0000) >> 4) as u8,
                idx: LdStIndex::X,
                ext: LdStExt::PreDec,
            },
            _ if (w0 & OPCODE_OP_LDYINC_MASK) == OPCODE_OP_LDYINC_BITS => Op::Ld {
                d: ((w0 & 0b0000_0001_1111_0000) >> 4) as u8,
                idx: LdStIndex::Y,
                ext: LdStExt::PostInc,
            },
            _ if (w0 & OPCODE_OP_LDYDEC_MASK) == OPCODE_OP_LDYDEC_BITS => Op::Ld {
                d: ((w0 & 0b0000_0001_1111_0000) >> 4) as u8,
                idx: LdStIndex::Y,
                ext: LdStExt::PreDec,
            },
            _ if (w0 & OPCODE_OP_LDYADQ_MASK) == OPCODE_OP_LDYADQ_BITS => Op::Ld {
                d: ((w0 & 0b0000_0001_1111_0000) >> 4) as u8,
                idx: LdStIndex::Y,
                ext: LdStExt::Displacement(
                    ((w0 & 0b0010_0000_0000_0000) >> 4
                        | (w0 & 0b0000_1100_0000_0000) >> 7
                        | w0 & 0b0000_0000_0000_0111) as u8,
                ),
            },
            _ if (w0 & OPCODE_OP_LDZINC_MASK) == OPCODE_OP_LDZINC_BITS => Op::Ld {
                d: ((w0 & 0b0000_0001_1111_0000) >> 4) as u8,
                idx: LdStIndex::Z,
                ext: LdStExt::PostInc,
            },
            _ if (w0 & OPCODE_OP_LDZDEC_MASK) == OPCODE_OP_LDZDEC_BITS => Op::Ld {
                d: ((w0 & 0b0000_0001_1111_0000) >> 4) as u8,
                idx: LdStIndex::Z,
                ext: LdStExt::PreDec,
            },
            _ if (w0 & OPCODE_OP_LDZADQ_MASK) == OPCODE_OP_LDZADQ_BITS => Op::Ld {
                d: ((w0 & 0b0000_0001_1111_0000) >> 4) as u8,
                idx: LdStIndex::Z,
                ext: LdStExt::Displacement(
                    ((w0 & 0b0010_0000_0000_0000) >> 4
                        | (w0 & 0b0000_1100_0000_0000) >> 7
                        | w0 & 0b0000_0000_0000_0111) as u8,
                ),
            },
            _ if (w0 & OPCODE_OP_LDI_MASK) == OPCODE_OP_LDI_BITS => Self::Ldi {
                k: ((w0 & 0b0000_1111_0000_0000) >> 4 | w0 & 0b0000_0000_0000_1111) as u8,
                d: ((w0 & 0b0000_0000_1111_0000) >> 4) as u8 + 16,
            },
            _ if (w0 & OPCODE_OP_LDS_MASK) == OPCODE_OP_LDS_BITS => Self::Lds {
                d: ((w0 & 0b0000_0001_1111_0000) >> 4) as u8,
                k: w1,
            },
            _ if (w0 & OPCODE_OP_LPMR0_MASK) == OPCODE_OP_LPMR0_BITS => Op::Lpmr0,
            _ if (w0 & OPCODE_OP_LPM_MASK) == OPCODE_OP_LPM_BITS => Op::Lpm {
                d: ((w0 & 0b0000_0001_1111_0000) >> 4) as u8,
                inc: false,
            },
            _ if (w0 & OPCODE_OP_LPMINC_MASK) == OPCODE_OP_LPMINC_BITS => Op::Lpm {
                d: ((w0 & 0b0000_0001_1111_0000) >> 4) as u8,
                inc: true,
            },
            _ if (w0 & OPCODE_OP_LSR_MASK) == OPCODE_OP_LSR_BITS => Self::Lsr {
                d: ((w0 & 0b0000_0001_1111_0000) >> 4) as u8,
            },
            _ if (w0 & OPCODE_OP_MOV_MASK) == OPCODE_OP_MOV_BITS => Self::Mov {
                r: ((w0 & 0b0000_0010_0000_0000) >> 5 | w0 & 0b0000_0000_0000_1111) as u8,
                d: ((w0 & 0b0000_0001_1111_0000) >> 4) as u8,
            },
            _ if (w0 & OPCODE_OP_MOVW_MASK) == OPCODE_OP_MOVW_BITS => Self::Movw {
                r: ((w0 & 0b0000_0000_0000_1111) as u8) * 2,
                d: (((w0 & 0b0000_0000_1111_0000) >> 4) as u8) * 2,
            },
            _ if (w0 & OPCODE_OP_MUL_MASK) == OPCODE_OP_MUL_BITS => Self::Mul {
                r: ((w0 & 0b0000_0010_0000_0000) >> 5 | w0 & 0b0000_0000_0000_1111) as u8,
                d: ((w0 & 0b0000_0001_1111_0000) >> 4) as u8,
            },
            _ if (w0 & OPCODE_OP_MULS_MASK) == OPCODE_OP_MULS_BITS => Self::Muls {
                r: (w0 & 0b0000_0000_0000_1111) as u8 + 16,
                d: ((w0 & 0b0000_0000_1111_0000) >> 4) as u8 + 16,
            },
            _ if (w0 & OPCODE_OP_MULSU_MASK) == OPCODE_OP_MULSU_BITS => Self::Mulsu {
                r: (w0 & 0b0000_0000_0000_0111) as u8 + 16,
                d: ((w0 & 0b0000_0000_0111_0000) >> 4) as u8 + 16,
            },
            _ if (w0 & OPCODE_OP_NEG_MASK) == OPCODE_OP_NEG_BITS => Self::Neg {
                d: ((w0 & 0b0000_0001_1111_0000) >> 4) as u8,
            },
            _ if (w0 & OPCODE_OP_NOP_MASK) == OPCODE_OP_NOP_BITS => Self::Nop,
            _ if (w0 & OPCODE_OP_OR_MASK) == OPCODE_OP_OR_BITS => Self::Or {
                r: ((w0 & 0b0000_0010_0000_0000) >> 5 | w0 & 0b0000_0000_0000_1111) as u8,
                d: ((w0 & 0b0000_0001_1111_0000) >> 4) as u8,
            },
            _ if (w0 & OPCODE_OP_ORI_MASK) == OPCODE_OP_ORI_BITS => Self::Ori {
                k: ((w0 & 0b0000_1111_0000_0000) >> 4 | w0 & 0b0000_0000_0000_1111) as u8,
                d: ((w0 & 0b0000_0000_1111_0000) >> 4) as u8 + 16,
            },
            _ if (w0 & OPCODE_OP_OUT_MASK) == OPCODE_OP_OUT_BITS => Self::Out {
                r: ((w0 & 0b0000_0001_1111_0000) >> 4) as u8,
                a: ((w0 & 0b0000_0110_0000_0000) >> 5 | w0 & 0b0000_0000_0000_1111) as u8,
            },
            _ if (w0 & OPCODE_OP_POP_MASK) == OPCODE_OP_POP_BITS => Self::Pop {
                d: ((w0 & 0b0000_0001_1111_0000) >> 4) as u8,
            },
            _ if (w0 & OPCODE_OP_PUSH_MASK) == OPCODE_OP_PUSH_BITS => Self::Push {
                r: ((w0 & 0b0000_0001_1111_0000) >> 4) as u8,
            },
            _ if (w0 & OPCODE_OP_RCALL_MASK) == OPCODE_OP_RCALL_BITS => Self::Rcall {
                k: ((w0 & 0b0000_1111_1111_1111) << 4) as i16 >> 4,
            },
            _ if (w0 & OPCODE_OP_RET_MASK) == OPCODE_OP_RET_BITS => Self::Ret,
            _ if (w0 & OPCODE_OP_RETI_MASK) == OPCODE_OP_RETI_BITS => Self::Reti,
            _ if (w0 & OPCODE_OP_RJMP_MASK) == OPCODE_OP_RJMP_BITS => Self::Rjmp {
                k: ((w0 & 0b0000_1111_1111_1111) << 4) as i16 >> 4,
            },
            _ if (w0 & OPCODE_OP_ROR_MASK) == OPCODE_OP_ROR_BITS => Self::Ror {
                d: ((w0 & 0b0000_0001_1111_0000) >> 4) as u8,
            },
            _ if (w0 & OPCODE_OP_SBC_MASK) == OPCODE_OP_SBC_BITS => Self::Sbc {
                r: ((w0 & 0b0000_0010_0000_0000) >> 5 | w0 & 0b0000_0000_0000_1111) as u8,
                d: ((w0 & 0b0000_0001_1111_0000) >> 4) as u8,
            },
            _ if (w0 & OPCODE_OP_SBCI_MASK) == OPCODE_OP_SBCI_BITS => Self::Sbci {
                k: ((w0 & 0b0000_1111_0000_0000) >> 4 | w0 & 0b0000_0000_0000_1111) as u8,
                d: ((w0 & 0b0000_0000_1111_0000) >> 4) as u8 + 16,
            },
            _ if (w0 & OPCODE_OP_SBI_MASK) == OPCODE_OP_SBI_BITS => Self::Sbi {
                a: ((w0 & 0b0000_0000_1111_1000) >> 3) as u8,
                b: (w0 & 0b0000_0000_0000_0111) as u8,
            },
            _ if (w0 & OPCODE_OP_SBIC_MASK) == OPCODE_OP_SBIC_BITS => Self::Sbic {
                a: ((w0 & 0b0000_0000_1111_1000) >> 3) as u8,
                b: (w0 & 0b0000_0000_0000_0111) as u8,
            },
            _ if (w0 & OPCODE_OP_SBIS_MASK) == OPCODE_OP_SBIS_BITS => Self::Sbis {
                a: ((w0 & 0b0000_0000_1111_1000) >> 3) as u8,
                b: (w0 & 0b0000_0000_0000_0111) as u8,
            },
            _ if (w0 & OPCODE_OP_SBIW_MASK) == OPCODE_OP_SBIW_BITS => Self::Sbiw {
                k: ((w0 & 0b0000_0000_1100_0000) >> 3 | w0 & 0b0000_0000_0000_1111) as u8,
                d: (((w0 & 0b0000_0000_0011_0000) >> 4) as u8) * 2 + 24,
            },
            _ if (w0 & OPCODE_OP_SBRC_MASK) == OPCODE_OP_SBRC_BITS => Self::Sbrc {
                b: (w0 & 0b0000_0000_0000_0111) as u8,
                r: ((w0 & 0b0000_0001_1111_0000) >> 4) as u8,
            },
            _ if (w0 & OPCODE_OP_SBRS_MASK) == OPCODE_OP_SBRS_BITS => Self::Sbrs {
                b: (w0 & 0b0000_0000_0000_0111) as u8,
                r: ((w0 & 0b0000_0001_1111_0000) >> 4) as u8,
            },
            _ if (w0 & OPCODE_OP_SER_MASK) == OPCODE_OP_SER_BITS => Self::Ser {
                d: ((w0 & 0b0000_0000_1111_0000) >> 4) as u8 + 16,
            },
            _ if (w0 & OPCODE_OP_SLEEP_MASK) == OPCODE_OP_SLEEP_BITS => Self::Sleep,
            _ if (w0 & OPCODE_OP_SPM_MASK) == OPCODE_OP_SPM_BITS => Op::Spm,
            _ if (w0 & OPCODE_OP_SPM2_MASK) == OPCODE_OP_SPM2_BITS => Op::Spm2,
            _ if (w0 & OPCODE_OP_STX_MASK) == OPCODE_OP_STX_BITS => Op::St {
                r: ((w0 & 0b0000_0001_1111_0000) >> 4) as u8,
                idx: LdStIndex::X,
                ext: LdStExt::None,
            },
            _ if (w0 & OPCODE_OP_STXINC_MASK) == OPCODE_OP_STXINC_BITS => Op::St {
                r: ((w0 & 0b0000_0001_1111_0000) >> 4) as u8,
                idx: LdStIndex::X,
                ext: LdStExt::PostInc,
            },
            _ if (w0 & OPCODE_OP_STXDEC_MASK) == OPCODE_OP_STXDEC_BITS => Op::St {
                r: ((w0 & 0b0000_0001_1111_0000) >> 4) as u8,
                idx: LdStIndex::X,
                ext: LdStExt::PreDec,
            },
            _ if (w0 & OPCODE_OP_STYINC_MASK) == OPCODE_OP_STYINC_BITS => Op::St {
                r: ((w0 & 0b0000_0001_1111_0000) >> 4) as u8,
                idx: LdStIndex::Y,
                ext: LdStExt::PostInc,
            },
            _ if (w0 & OPCODE_OP_STYDEC_MASK) == OPCODE_OP_STYDEC_BITS => Op::St {
                r: ((w0 & 0b0000_0001_1111_0000) >> 4) as u8,
                idx: LdStIndex::Y,
                ext: LdStExt::PreDec,
            },
            _ if (w0 & OPCODE_OP_STYADQ_MASK) == OPCODE_OP_STYADQ_BITS => Op::St {
                r: ((w0 & 0b0000_0001_1111_0000) >> 4) as u8,
                idx: LdStIndex::Y,
                ext: LdStExt::Displacement(
                    ((w0 & 0b0010_0000_0000_0000) >> 4
                        | (w0 & 0b0000_1100_0000_0000) >> 7
                        | w0 & 0b0000_0000_0000_0111) as u8,
                ),
            },
            _ if (w0 & OPCODE_OP_STZINC_MASK) == OPCODE_OP_STZINC_BITS => Op::St {
                r: ((w0 & 0b0000_0001_1111_0000) >> 4) as u8,
                idx: LdStIndex::Z,
                ext: LdStExt::PostInc,
            },
            _ if (w0 & OPCODE_OP_STZDEC_MASK) == OPCODE_OP_STZDEC_BITS => Op::St {
                r: ((w0 & 0b0000_0001_1111_0000) >> 4) as u8,
                idx: LdStIndex::Z,
                ext: LdStExt::PreDec,
            },
            _ if (w0 & OPCODE_OP_STZADQ_MASK) == OPCODE_OP_STZADQ_BITS => Op::St {
                r: ((w0 & 0b0000_0001_1111_0000) >> 4) as u8,
                idx: LdStIndex::Z,
                ext: LdStExt::Displacement(
                    ((w0 & 0b0010_0000_0000_0000) >> 4
                        | (w0 & 0b0000_1100_0000_0000) >> 7
                        | w0 & 0b0000_0000_0000_0111) as u8,
                ),
            },
            _ if (w0 & OPCODE_OP_STS_MASK) == OPCODE_OP_STS_BITS => Self::Sts {
                k: w1,
                r: ((w0 & 0b0000_0001_1111_0000) >> 4) as u8,
            },
            _ if (w0 & OPCODE_OP_SUB_MASK) == OPCODE_OP_SUB_BITS => Self::Sub {
                r: ((w0 & 0b0000_0010_0000_0000) >> 5 | w0 & 0b0000_0000_0000_1111) as u8,
                d: ((w0 & 0b0000_0001_1111_0000) >> 4) as u8,
            },
            _ if (w0 & OPCODE_OP_SUBI_MASK) == OPCODE_OP_SUBI_BITS => Self::Subi {
                k: ((w0 & 0b0000_1111_0000_0000) >> 4 | w0 & 0b0000_0000_0000_1111) as u8,
                d: ((w0 & 0b0000_0000_1111_0000) >> 4) as u8 + 16,
            },
            _ if (w0 & OPCODE_OP_SWAP_MASK) == OPCODE_OP_SWAP_BITS => Self::Swap {
                d: ((w0 & 0b0000_0001_1111_0000) >> 4) as u8,
            },
            _ if (w0 & OPCODE_OP_WDR_MASK) == OPCODE_OP_WDR_BITS => Self::Wdr,
            _ => Self::Undefined { w: w0 },
        }
    }
}

pub struct OpAddr {
    pub op: Op,
    pub addr: u16,
}

impl<'a> fmt::Display for OpAddr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let pc = self.addr >> 1;
        match self.op {
            Op::Adc { d, r } => write!(f, "ADC R{}, R{}", d, r),
            Op::Add { d, r } => write!(f, "ADD R{}, R{}", d, r),
            Op::Adiw { d, k } => write!(f, "ADIW R{}, {}", d, k),
            Op::And { d, r } => write!(f, "AND R{}, R{}", d, r),
            Op::Andi { d, k } => write!(f, "ANDI R{}, {}", d, k),
            Op::Asr { d } => write!(f, "ASR R{}", d),
            Op::Bclr { s } => write!(f, "BCLR {}", s),
            Op::Bld { d, b } => write!(f, "BLD R{}, {}", d, b),
            Op::Brbc { s, k } => {
                let (pc1, _) = (pc as i16).overflowing_add(1);
                let (pc1, _) = (pc1 as i16).overflowing_add(k as i16);
                write!(f, "BRBC {}, 0x{:04x}; k={}", s, (pc1 as u16) << 1, k)
            }
            Op::Brbs { s, k } => {
                let (pc1, _) = (pc as i16).overflowing_add(1);
                let (pc1, _) = (pc1 as i16).overflowing_add(k as i16);
                write!(f, "BRBS {}, 0x{:04x}; k={}", s, (pc1 as u16) << 1, k)
            }
            Op::Break => write!(f, "BREAK"),
            Op::Bset { s } => write!(f, "BSET {}", s),
            Op::Bst { d, b } => write!(f, "BST R{}, {}", d, b),
            Op::Call { k } => write!(f, "CALL 0x{:04x}", k << 1),
            Op::Cbi { a, b } => {
                write!(f, "CBI {}, {}", a, b)?;
                if let Some(io_reg) = io_reg_str(IOSPACE_ADDR + a as u16) {
                    write!(f, "; {} = 0x{:02x}", io_reg, a)?;
                }
                Ok(())
            }
            Op::Com { d } => write!(f, "COM R{}", d),
            Op::Cp { d, r } => write!(f, "CP R{}, R{}", d, r),
            Op::Cpc { d, r } => write!(f, "CPC R{}, R{}", d, r),
            Op::Cpi { d, k } => write!(f, "CPI R{}, {}", d, k),
            Op::Cpse { d, r } => write!(f, "CPSE R{}, R{}", d, r),
            Op::Dec { d } => write!(f, "DEC R{}", d),
            Op::Eicall => write!(f, "EICALL"),
            Op::Eijmp => write!(f, "EIJMP"),
            Op::Elpmr0 => write!(f, "ELPM"),
            Op::Elpm { d, inc } => {
                write!(f, "ELPM R{}, Z", d)?;
                if inc {
                    write!(f, "+")?;
                }
                Ok(())
            }
            Op::Eor { d, r } => write!(f, "EOR R{}, R{}", d, r),
            Op::Fmul { d, r } => write!(f, "FMUL R{}, R{}", d, r),
            Op::Fmuls { d, r } => write!(f, "FMULS R{}, R{}", d, r),
            Op::Fmulsu { d, r } => write!(f, "FMULSU R{}, R{}", d, r),
            Op::Icall => write!(f, "ICALL"),
            Op::Ijmp => write!(f, "IJMP"),
            Op::In { d, a } => {
                write!(f, "IN R{}, {}", d, a)?;
                if let Some(io_reg) = io_reg_str(IOSPACE_ADDR + a as u16) {
                    write!(f, "; {} = 0x{:02x}", io_reg, a)?;
                }
                Ok(())
            }
            Op::Inc { d } => write!(f, "INC R{}", d),
            Op::Jmp { k } => write!(f, "JMP 0x{:04x}", k << 1),
            Op::Ld {
                d,
                ref idx,
                ref ext,
            } => {
                write!(f, "LD R{}, ", d)?;
                match ext {
                    LdStExt::None => write!(f, "{}", idx),
                    LdStExt::PostInc => write!(f, "{}+", idx),
                    LdStExt::PreDec => write!(f, "-{}", idx),
                    LdStExt::Displacement(q) => write!(f, "{}+{}", idx, q),
                }
            }
            Op::Ldi { d, k } => write!(f, "LDI R{}, {}", d, k),
            Op::Lds { d, k } => write!(f, "LDS R{}, 0x{:04x}", d, k),
            Op::Lpmr0 => write!(f, "LPM"),
            Op::Lpm { d, inc } => {
                write!(f, "LPM R{}, Z", d)?;
                if inc {
                    write!(f, "+")?;
                }
                Ok(())
            }
            Op::Lsr { d } => write!(f, "LSR R{}", d),
            Op::Mov { d, r } => write!(f, "MOV R{}, R{}", d, r),
            Op::Movw { d, r } => write!(f, "MOVW R{}, R{}", d, r),
            Op::Mul { d, r } => write!(f, "MUL R{}, R{}", d, r),
            Op::Muls { d, r } => write!(f, "MULS R{}, R{}", d, r),
            Op::Mulsu { d, r } => write!(f, "MULSU R{}, R{}", d, r),
            Op::Neg { d } => write!(f, "NEG R{}", d),
            Op::Nop => write!(f, "NOP"),
            Op::Or { d, r } => write!(f, "OR R{}, R{}", d, r),
            Op::Ori { d, k } => write!(f, "ORI R{}, {}", d, k),
            Op::Out { a, r } => {
                write!(f, "OUT 0x{:02x}, R{}", a, r)?;
                if let Some(io_reg) = io_reg_str(IOSPACE_ADDR + a as u16) {
                    write!(f, "; {} = 0x{:02x}", io_reg, a)?;
                }
                Ok(())
            }
            Op::Pop { d } => write!(f, "POP R{}", d),
            Op::Push { r } => write!(f, "PUSH R{}", r),
            Op::Rcall { k } => {
                let (pc1, _) = (pc as i16).overflowing_add(1);
                let (pc1, _) = (pc1 as i16).overflowing_add(k);
                write!(f, "RCALL 0x{:04x}; k={}", (pc1 as u16) << 1, k)
            }
            Op::Ret => write!(f, "RET"),
            Op::Reti => write!(f, "RETI"),
            Op::Rjmp { k } => {
                let (pc1, _) = (pc as i16).overflowing_add(1);
                let (pc1, _) = (pc1 as i16).overflowing_add(k);
                write!(f, "RJMP 0x{:04x}; k={}", (pc1 as u16) << 1, k)
            }
            Op::Ror { d } => write!(f, "ROR R{}", d),
            Op::Sbc { d, r } => write!(f, "SCB R{}, R{}", d, r),
            Op::Sbci { d, k } => write!(f, "SBCI R{}, {}", d, k),
            Op::Sbi { a, b } => {
                write!(f, "SBI 0x{:02x}, {}", a, b)?;
                if let Some(io_reg) = io_reg_str(IOSPACE_ADDR + a as u16) {
                    write!(f, "; {} = 0x{:02x}", io_reg, a)?;
                }
                Ok(())
            }
            Op::Sbic { a, b } => {
                write!(f, "SBIC 0x{:02x}, {}", a, b)?;
                if let Some(io_reg) = io_reg_str(IOSPACE_ADDR + a as u16) {
                    write!(f, "; {} = 0x{:02x}", io_reg, a)?;
                }
                Ok(())
            }
            Op::Sbis { a, b } => {
                write!(f, "SBIS 0x{:02x}, {}", a, b)?;
                if let Some(io_reg) = io_reg_str(IOSPACE_ADDR + a as u16) {
                    write!(f, "; {} = 0x{:02x}", io_reg, a)?;
                }
                Ok(())
            }
            Op::Sbiw { d, k } => write!(f, "SBIW R{}, {}", d, k),
            Op::Sbrc { r, b } => write!(f, "SBRC R{}, {}", r, b),
            Op::Sbrs { r, b } => write!(f, "SBRS R{}, {}", r, b),
            Op::Ser { d } => write!(f, "SER R{}", d),
            Op::Sleep => write!(f, "SLEEP"),
            Op::Spm => write!(f, "SPM"),
            Op::Spm2 => write!(f, "SPM Z+"),
            Op::St {
                r,
                ref idx,
                ref ext,
            } => {
                write!(f, "ST R{}, ", r)?;
                match ext {
                    LdStExt::None => write!(f, "{}", idx),
                    LdStExt::PostInc => write!(f, "{}+", idx),
                    LdStExt::PreDec => write!(f, "-{}", idx),
                    LdStExt::Displacement(q) => write!(f, "{}+{}", idx, q),
                }
            }
            Op::Sts { k, r } => write!(f, "STS 0x{:04x}, R{}", k, r),
            Op::Sub { d, r } => write!(f, "SUB R{}, R{}", d, r),
            Op::Subi { d, k } => write!(f, "SUBI R{}, {}", d, k),
            Op::Swap { d } => write!(f, "SWAP R{}", d),
            Op::Wdr => write!(f, "WDR"),
            Op::Undefined { w } => write!(f, "UNDEF ; 0x{:04}", w),
        }
    }
}
