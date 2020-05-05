#!/usr/bin/env python3

#      Instruction list

# =ALU0= (Rd, Rr) -> (Rd)
#-   6 ADD    0000_11rd_dddd_rrrr
#-   8 AND    0010_00rd_dddd_rrrr
#-  49 CP     0001_01rd_dddd_rrrr
#-  58 EOR    0010_01rd_dddd_rrrr
#-  79 MOV    0010_11rd_dddd_rrrr
#-  86 OR     0010_10rd_dddd_rrrr
#- 123 SUB    0001_10rd_dddd_rrrr

# =ALU1= (SREG, Rd, Rr) -> (Rd)
#-   5 ADC    0001_11rd_dddd_rrrr
#-  97 SBC    0000_10rd_dddd_rrrr
#-  50 CPC    0000_01rd_dddd_rrrr

# =ALU2= (Rd) -> (Rd)
#-  48 COM    1001_010d_dddd_0000
#-  84 NEG    1001_010d_dddd_0001
#-  65 INC    1001_010d_dddd_0011
#-  53 DEC    1001_010d_dddd_1010
#- 110 SER    1110_1111_dddd_1111
#-  10 ASR    1001_010d_dddd_0101
#- 125 SWAP   1001_010d_dddd_0010

# =ALU3= (SREG, Rd) -> (Rd)
#  96 ROR    1001_010d_dddd_0111

#   7 ADIW   1001_0110_KKdd_KKKK
#   9 ANDI   0111_KKKK_dddd_KKKK
#  11 BCLR   1001_0100_1sss_1000
#  12 BLD    1111_100d_dddd_0bbb
#  13 BRBC   1111_01kk_kkkk_ksss
#  14 BRBS   1111_00kk_kkkk_ksss
#  17 BREAK  1001_0101_1001_1000
#  34 BSET   1001_0100_0sss_1000
#  35 BST    1111_101d_dddd_0bbb
#  36 CALL   1001_010k_kkkk_111k kkkk_kkkk_kkkk_kkkk
#  37 CBI    1001_1000_AAAA_Abbb
#  51 CPI    0011_KKKK_dddd_KKKK
#  52 CPSE   0001_00rd_dddd_rrrr
#  55 EICALL 1001_0101_0001_1001
#  56 EIJMP  1001_0100_0001_1001
#  57 ELPMR0 1001_0101_1101_1000
#  57 ELPM   1001_000d_dddd_0110
#  57 ELPMINC 1001_000d_dddd_0111
#  59 FMUL   0000_0011_0ddd_1rrr
#  60 FMULS  0000_0011_1ddd_0rrr
#  61 FMULSU 0000_0011_1ddd_1rrr
#  62 ICALL  1001_0101_0000_1001
#  63 IJMP   1001_0100_0000_1001
#  64 IN     1011_0AAd_dddd_AAAA
#  66 JMP    1001_010k_kkkk_110k kkkk_kkkk_kkkk_kkkk
#  70 LDX    1001_000d_dddd_1100
#  70 LDXINC 1001_000d_dddd_1101
#  70 LDXDEC 1001_000d_dddd_1110
#  71 LDYINC 1001_000d_dddd_1001
#  71 LDYDEC 1001_000d_dddd_1010
#  71 LDYADQ 10q0_qq0d_dddd_1qqq
#  72 LDZINC 1001_000d_dddd_0001
#  72 LDZDEC 1001_000d_dddd_0010
#  72 LDZADQ 10q0_qq0d_dddd_0qqq
#  73 LDI    1110_KKKK_dddd_KKKK
#  74 LDS    1001_000d_dddd_0000 kkkk_kkkk_kkkk_kkkk
#  76 LPMR0  1001_0101_1100_1000
#  76 LPM    1001_000d_dddd_0100
#  76 LPMINC 1001_000d_dddd_0101
#  78 LSR    1001_010d_dddd_0110
#  80 MOVW   0000_0001_dddd_rrrr
#  81 MUL    1001_11rd_dddd_rrrr
#  82 MULS   0000_0010_dddd_rrrr
#  83 MULSU  0000_0011_0ddd_0rrr
#  85 NOP    0000_0000_0000_0000
#  87 ORI    0110_KKKK_dddd_KKKK
#  88 OUT    1011_1AAr_rrrr_AAAA
#  89 POP    1001_000d_dddd_1111
#  90 PUSH   1001_001r_rrrr_1111
#  91 RCALL  1101_kkkk_kkkk_kkkk
#  92 RET    1001_0101_0000_1000
#  93 RETI   1001_0101_0001_1000
#  94 RJMP   1100_kkkk_kkkk_kkkk
#  98 SBCI   0100_KKKK_dddd_KKKK
#  99 SBI    1001_1010_AAAA_Abbb
# 100 SBIC   1001_1001_AAAA_Abbb
# 101 SBIS   1001_1011_AAAA_Abbb
# 102 SBIW   1001_0111_KKdd_KKKK
# 104 SBRC   1111_110r_rrrr_0bbb
# 105 SBRS   1111_111r_rrrr_0bbb
# 115 SLEEP  1001_0101_1000_1000
# 116 SPM    1001_0101_1110_1000
# 117 SPM2   1001_0101_1111_1000
# 118 STX    1001_001r_rrrr_1100
# 118 STXINC 1001_001r_rrrr_1101
# 118 STXDEC 1001_001r_rrrr_1111
# 119 STYINC 1001_001r_rrrr_1001
# 119 STYDEC 1001_001r_rrrr_1010
# 119 STYADQ 10q0_qq1r_rrrr_1qqq
# 120 STZINC 1001_001r_rrrr_0001
# 120 STZDEC 1001_001r_rrrr_0010
# 120 STZADQ 10q0_qq1r_rrrr_0qqq
# 121 STS    1001_001r_rrrr_0000 kkkk_kkkk_kkkk_kkkk
# 124 SUBI   0101_KKKK_dddd_KKKK
# 127 WDR    1001_0101_1010_1000

OP_SELECT_BEGIN = """
void op_{op_type}_select(void (**op)({op_args}), String cmd) {{
	if (cmd.equals("")) {{"""
OP_SELECT_IF = """
	}} else if (cmd.equals("{op}")) {{
		*op = &op_{op_low};"""
OP_SELECT_END = """
	} else {
	}
}
"""

ops_alu0 = ['ADD', 'AND', 'CP', 'EOR', 'MOV', 'OR', 'SUB']
OP_ALU0_ARGS = "uint8_t* a, uint8_t b, uint8_t *sreg"
OP_ALU0_TEST = """
void op_{op_low}(uint8_t* a, uint8_t b, uint8_t *sreg) {{
	SREG = 0;
	asm volatile(
		"{op_low} %0, %3 \\n"
		"in %1, __SREG__ \\n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a), "r" (b)
		:
	);
}}
"""

ops_alu1 = ['ADC', 'CPC', 'SBC']
OP_ALU1_ARGS = "uint8_t* a, uint8_t b, uint8_t *sreg"
OP_ALU1_TEST = """
void op_{op_low}(uint8_t* a, uint8_t b, uint8_t *sreg) {{
	SREG = *sreg;
	asm volatile(
		"{op_low} %0, %3 \\n"
		"in %1, __SREG__ \\n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a), "r" (b)
		:
	);
}}
"""

ops_alu2 = ['COM', 'NEG', 'INC', 'DEC', 'SER', 'ASR', 'SWAP']
OP_ALU2_ARGS = "uint8_t* a, uint8_t *sreg"
OP_ALU2_TEST = """
void op_{op_low}(uint8_t* a, uint8_t *sreg) {{
	SREG = 0;
	asm volatile(
		"{op_low} %0 \\n"
		"in %1, __SREG__ \\n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}}
"""

ops_alu3 = ['ROR']
OP_ALU3_ARGS = "uint8_t* a, uint8_t *sreg"
OP_ALU3_TEST = """
void op_{op_low}(uint8_t* a, uint8_t *sreg) {{
	SREG = *sreg;
	asm volatile(
		"{op_low} %0 \\n"
		"in %1, __SREG__ \\n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}}
"""

ops = {
        'alu0': (ops_alu0, OP_ALU0_TEST, OP_ALU0_ARGS),
        'alu1': (ops_alu1, OP_ALU1_TEST, OP_ALU1_ARGS),
        'alu2': (ops_alu2, OP_ALU2_TEST, OP_ALU2_ARGS),
        'alu3': (ops_alu3, OP_ALU3_TEST, OP_ALU3_ARGS),
        }



if __name__ == "__main__":
    with open('ops.h', 'w') as f:
        for op_type, ops_cfg in ops.items():
            ops_list = ops_cfg[0]
            op_test = ops_cfg[1]
            op_args = ops_cfg[2]
            for op in ops_list:
                op_low = op.lower()
                f.write(op_test.format(op_low=op_low))
            f.write(OP_SELECT_BEGIN.format(op_type=op_type, op_args=op_args))
            for op in ops_list:
                op_low = op.lower()
                f.write(OP_SELECT_IF.format(op_low=op_low, op=op))
            f.write(OP_SELECT_END)
