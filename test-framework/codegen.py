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

# =ALU0S= (SREG, Rd, Rr) -> (Rd)
#-   5 ADC    0001_11rd_dddd_rrrr
#-  97 SBC    0000_10rd_dddd_rrrr
#-  50 CPC    0000_01rd_dddd_rrrr

# =ALU1= (Rd) -> (Rd)
#-  48 COM    1001_010d_dddd_0000
#-  84 NEG    1001_010d_dddd_0001
#-  65 INC    1001_010d_dddd_0011
#-  53 DEC    1001_010d_dddd_1010
#- 110 SER    1110_1111_dddd_1111
#-  10 ASR    1001_010d_dddd_0101
#- 125 SWAP   1001_010d_dddd_0010

# =ALU1S= (SREG, Rd) -> (Rd)
#-  96 ROR    1001_010d_dddd_0111

# =ALU2= (Rd, k) -> (Rd)
#- 124 SUBI   0101_KKKK_dddd_KKKK
#-   9 ANDI   0111_KKKK_dddd_KKKK
#-  87 ORI    0110_KKKK_dddd_KKKK
#-  51 CPI    0011_KKKK_dddd_KKKK
#-  73 LDI    1110_KKKK_dddd_KKKK

# =ALU2S= (SREG, Rd, k) -> (Rd)
#-  98 SBCI   0100_KKKK_dddd_KKKK

# =ALU3= (Rd0, Rd1, k) -> (Rd0, Rd1)
#-   7 ADIW   1001_0110_KKdd_KKKK
#- 102 SBIW   1001_0111_KKdd_KKKK

# =ALU4= (Rr0, Rr1) -> (Rd0, Rd1)
#  80 MOVW   0000_0001_dddd_rrrr

# =ALU5= (Rd, Rr) -> (R0, R1)
#  81 MUL    1001_11rd_dddd_rrrr
#  82 MULS   0000_0010_dddd_rrrr
#  83 MULSU  0000_0011_0ddd_0rrr
#  59 FMUL   0000_0011_0ddd_1rrr
#  60 FMULS  0000_0011_1ddd_0rrr
#  61 FMULSU 0000_0011_1ddd_1rrr

#  11 BCLR   1001_0100_1sss_1000
#  12 BLD    1111_100d_dddd_0bbb
#  13 BRBC   1111_01kk_kkkk_ksss
#  14 BRBS   1111_00kk_kkkk_ksss
#  17 BREAK  1001_0101_1001_1000
#  34 BSET   1001_0100_0sss_1000
#  35 BST    1111_101d_dddd_0bbb
#  36 CALL   1001_010k_kkkk_111k kkkk_kkkk_kkkk_kkkk
#  37 CBI    1001_1000_AAAA_Abbb
#  52 CPSE   0001_00rd_dddd_rrrr
#  55 EICALL 1001_0101_0001_1001
#  56 EIJMP  1001_0100_0001_1001
#  57 ELPMR0 1001_0101_1101_1000
#  57 ELPM   1001_000d_dddd_0110
#  57 ELPMINC 1001_000d_dddd_0111
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
#  74 LDS    1001_000d_dddd_0000 kkkk_kkkk_kkkk_kkkk
#  76 LPMR0  1001_0101_1100_1000
#  76 LPM    1001_000d_dddd_0100
#  76 LPMINC 1001_000d_dddd_0101
#  78 LSR    1001_010d_dddd_0110
#  85 NOP    0000_0000_0000_0000
#  88 OUT    1011_1AAr_rrrr_AAAA
#  89 POP    1001_000d_dddd_1111
#  90 PUSH   1001_001r_rrrr_1111
#  91 RCALL  1101_kkkk_kkkk_kkkk
#  92 RET    1001_0101_0000_1000
#  93 RETI   1001_0101_0001_1000
#  94 RJMP   1100_kkkk_kkkk_kkkk
#  99 SBI    1001_1010_AAAA_Abbb
# 100 SBIC   1001_1001_AAAA_Abbb
# 101 SBIS   1001_1011_AAAA_Abbb
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
# 127 WDR    1001_0101_1010_1000

OP_TEST_LOOP_ALU0 = """
void op_test_alu0(String op_name) {
	void (*op)(uint8_t* a, uint8_t b, uint8_t *sreg);
	op_alu0_select(&op, op_name);
	uint8_t buf[16];
	uint8_t a, b, sreg, sreg0;
	int i, j;
	for (i = 0; i < 0x100; i++) {
		for (j = 0; j < 0x100; j++) {
                        sreg0 = 0;
			a = i;
			b = j;
			sreg = sreg0;
			op(&a, b, &sreg);
			buf[0] = sreg0; buf[1] = i; buf[2] = j; buf[3] = a; buf[4] = sreg;
			Serial.write(buf, 5);
		}
	}
}
"""

OP_TEST_LOOP_ALU0S = """
void op_test_alu0s(String op_name) {
	void (*op)(uint8_t* a, uint8_t b, uint8_t *sreg);
	op_alu0s_select(&op, op_name);
	uint8_t buf[16];
	uint8_t a, b, sreg, sreg0;
	int i, j, s;
	for (s = 0; s < 2; s++) {
		if (s == 0) {
			sreg0 = 0;
		} else {
			sreg0 = 1 << 0;
		}
		for (i = 0; i < 0x100; i++) {
			for (j = 0; j < 0x100; j++) {
				a = i;
				b = j;
				sreg = sreg0;
				op(&a, b, &sreg);
				buf[0] = sreg0; buf[1] = i; buf[2] = j; buf[3] = a; buf[4] = sreg;
				Serial.write(buf, 5);
			}
		}
	}
}
"""

OP_TEST_LOOP_ALU1 = """
void op_test_alu1(String op_name) {
	void (*op)(uint8_t* a, uint8_t *sreg);
	op_alu1_select(&op, op_name);
	uint8_t buf[16];
	uint8_t a, sreg, sreg0;
	int i;
	for (i = 0; i < 0x100; i++) {
                sreg0 = 0;
		a = i;
		sreg = sreg0;
		op(&a, &sreg);
		buf[0] = sreg0; buf[1] = i; buf[2] = a; buf[3] = sreg;
		Serial.write(buf, 4);
	}
}
"""

OP_TEST_LOOP_ALU1S = """
void op_test_alu1s(String op_name) {
	void (*op)(uint8_t* a, uint8_t *sreg);
	op_alu1s_select(&op, op_name);
	uint8_t buf[16];
	uint8_t a, sreg, sreg0;
	int i, s;
	for (s = 0; s < 2; s++) {
		if (s == 0) {
			sreg0 = 0;
		} else {
			sreg0 = 1 << 0;
		}
		for (i = 0; i < 0x100; i++) {
			a = i;
			sreg = sreg0;
			op(&a, &sreg);
			buf[0] = sreg0; buf[1] = i; buf[2] = a; buf[3] = sreg;
			Serial.write(buf, 4);
		}
	}
}
"""

OP_TEST_LOOP_ALU2 = """
void op_test_alu2(String op_name) {
	void (*op)(uint8_t* a, uint8_t *sreg);
	op_alu2_select(&op, op_name);
	uint8_t buf[16];
	uint8_t a, sreg, sreg0;
	int i;
	for (i = 0; i < 0x100; i++) {
                sreg0 = 0;
		a = i;
		sreg = sreg0;
		op(&a, &sreg);
		buf[0] = sreg0; buf[1] = i; buf[2] = a; buf[3] = sreg;
		Serial.write(buf, 4);
	}
}
"""

OP_TEST_LOOP_ALU2S = """
void op_test_alu2s(String op_name) {
	void (*op)(uint8_t* a, uint8_t *sreg);
	op_alu2s_select(&op, op_name);
	uint8_t buf[16];
	uint8_t a, sreg, sreg0;
	int i, s;
	for (s = 0; s < 2; s++) {
		if (s == 0) {
			sreg0 = 0;
		} else {
			sreg0 = 1 << 0;
		}
		for (i = 0; i < 0x100; i++) {
			a = i;
			sreg = sreg0;
			op(&a, &sreg);
			buf[0] = sreg0; buf[1] = i; buf[2] = a; buf[3] = sreg;
			Serial.write(buf, 4);
		}
	}
}
"""

OP_TEST_LOOP_ALU3 = """
void op_test_alu3(String op_name) {
	void (*op)(uint16_t* a, uint8_t *sreg);
	op_alu3_select(&op, op_name);
	uint8_t buf[16];
        uint16_t a;
	uint8_t sreg, sreg0;
	uint32_t i;
        for (i = 0; i < 0x10000; i++) {
                sreg = 0;
                a = i;
                sreg = sreg0;
                op(&a, &sreg);
                buf[0] = (uint8_t) (i & 0x00ff);
                buf[1] = (uint8_t) ((i >> 8) & 0x00ff);
                buf[2] = (uint8_t) (a & 0x00ff);
                buf[3] = (uint8_t) ((a >> 8) & 0x00ff);
                buf[4] = sreg;
                Serial.write(buf, 5);
        }
}
"""

OP_TEST_LOOP_ALU4 = """
void op_test_alu4(String op_name) {
	void (*op)(uint8_t* a, uint8_t* b, uint8_t *sreg);
	op_alu4_select(&op, op_name);
	uint8_t buf[16];
	uint8_t a, b, sreg;
	uint32_t i, j;
        for (i = 0; i < 0x100; i++) {
                for (j = 0; j < 0x100; j++) {
                        sreg = 0;
                        a = i;
                        b = j;
                        op(&a, &b, &sreg);
                        buf[0] = i;
                        buf[1] = j;
                        buf[2] = a;
                        buf[3] = b;
                        buf[4] = sreg;
                        Serial.write(buf, 5);
                }
        }
}
"""


OP_TEST_MAIN_BEGIN = """
void test_op(String op_type, String op_name) {
	if (op_type.equals("")) {"""
OP_TEST_MAIN_IF = """
        }} else if (op_type.equals("{op_type}")) {{
		op_test_{op_type}(op_name);"""
OP_TEST_MAIN_END = """
	} else {
	}
}
"""

OP_SELECT_BEGIN = """
void op_{op_type}_select(void (**op)({op_args}), String cmd) {{
	if (cmd.equals("")) {{"""
OP_SELECT_IF = """
	}} else if (cmd.equals("{op}")) {{
		*op = &op_{op};"""
OP_SELECT_END = """
	} else {
	}
}
"""

OP_K_SELECT_BEGIN = """
void op_{op_type}_select(void (**op)({op_args}), String cmd) {{
	if (cmd.equals("")) {{"""
OP_K_SELECT_IF = """
	}} else if (cmd.equals("{op}_{k}")) {{
		*op = &op_{op}_{k};"""
OP_K_SELECT_END = """
	} else {
	}
}
"""

ops_alu0 = ['ADD', 'AND', 'CP', 'EOR', 'MOV', 'OR', 'SUB']
ops_alu0s = ['ADC', 'CPC', 'SBC']
OP_ALU0_ARGS = "uint8_t* a, uint8_t b, uint8_t *sreg"
OP_ALU0_TEST = """
void op_{op}(uint8_t* a, uint8_t b, uint8_t *sreg) {{
	SREG = *sreg;
	asm volatile(
		"{op} %0, %3 \\n"
		"in %1, __SREG__ \\n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a), "r" (b)
		:
	);
}}
"""

ops_alu1 = ['COM', 'NEG', 'INC', 'DEC', 'SER', 'ASR', 'SWAP']
ops_alu1s = ['ROR']
OP_ALU1_ARGS = "uint8_t* a, uint8_t *sreg"
OP_ALU1_TEST = """
void op_{op}(uint8_t* a, uint8_t *sreg) {{
	SREG = *sreg;
	asm volatile(
		"{op} %0 \\n"
		"in %1, __SREG__ \\n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}}
"""

ops_alu2 = ['SUBI', 'ANDI', 'ORI', 'CPI', 'LDI']
ops_alu2s = ['SBCI']
ops_alu2s = []
OP_ALU2_ARGS = "uint8_t* a, uint8_t *sreg"
OP_ALU2_TEST = """
void op_{op}_{k}(uint8_t* a, uint8_t *sreg) {{
	SREG = *sreg;
	asm volatile(
		"{op} %0, 0x{k} \\n"
		"in %1, __SREG__ \\n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}}
"""

ops_alu3 = ['ADIW', 'SBIW']
OP_ALU3_ARGS = "uint16_t* a, uint8_t *sreg"
OP_ALU3_TEST = """
void op_{op}_{k}(uint16_t* a, uint8_t *sreg) {{
	SREG = *sreg;
	asm volatile(
		"{op} %0, 0x{k} \\n"
		"in %1, __SREG__ \\n"
		: "=w" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}}
"""



ops_alu4 = ['MUL', 'MULS', 'MULSU', 'FMUL', 'FMULS', 'FMULSU']
OP_ALU4_ARGS = "uint8_t* a, uint8_t* b, uint8_t *sreg"
OP_ALU4_TEST = """
void op_{op}(uint8_t* a, uint8_t* b, uint8_t *sreg) {{
	SREG = *sreg;
	asm volatile(
                "push r0 \\n"
                "push r1 \\n"
		"{op} %0, %1 \\n"
		"in %2, __SREG__ \\n"
                "mov %0, r0 \\n"
                "mov %1, r1 \\n"
                "pop r1 \\n"
                "pop r0 \\n"
		: "=a" (*a), "=a" (*b), "=r" (*sreg)
		: "0" (*a), "1" (*b)
		:
	);
}}
"""


OPS = {
        'alu0' : (OP_TEST_LOOP_ALU0 , OP_ALU0_TEST, OP_ALU0_ARGS),
        'alu0s': (OP_TEST_LOOP_ALU0S, OP_ALU0_TEST, OP_ALU0_ARGS),
        'alu1' : (OP_TEST_LOOP_ALU1 , OP_ALU1_TEST, OP_ALU1_ARGS),
        'alu1s': (OP_TEST_LOOP_ALU1S, OP_ALU1_TEST, OP_ALU1_ARGS),
        'alu4' : (OP_TEST_LOOP_ALU4 , OP_ALU4_TEST, OP_ALU4_ARGS),
        }

OPS_K = {
        'alu2' : (OP_TEST_LOOP_ALU2 , OP_ALU2_TEST, OP_ALU2_ARGS, 0x100),
        'alu2s': (OP_TEST_LOOP_ALU2S, OP_ALU2_TEST, OP_ALU2_ARGS, 0x100),
        'alu3' : (OP_TEST_LOOP_ALU3 , OP_ALU3_TEST, OP_ALU3_ARGS, 0x40),
        }

def gen_ops(f, op_type, ops_list):
    ops_cfg = OPS[op_type]
    op_test_loop = ops_cfg[0]
    op_test = ops_cfg[1]
    op_args = ops_cfg[2]
    for op in ops_list:
        f.write(op_test.format(op=op))
    f.write(OP_SELECT_BEGIN.format(op_type=op_type, op_args=op_args))
    for op in ops_list:
        f.write(OP_SELECT_IF.format(op=op))
    f.write(OP_SELECT_END)
    f.write(op_test_loop)

def gen_ops_k(f, op_type, op):
    ops_cfg = OPS_K[op_type]
    op_test_loop = ops_cfg[0]
    op_test = ops_cfg[1]
    op_args = ops_cfg[2]
    k_range = ops_cfg[3]
    for k in range(k_range):
        k = '{:02x}'.format(k)
        f.write(op_test.format(op=op, k=k))
    f.write(OP_K_SELECT_BEGIN.format(op_type=op_type, op_args=op_args))
    for k in range(k_range):
        k = '{:02x}'.format(k)
        f.write(OP_K_SELECT_IF.format(op=op, k=k))
    f.write(OP_K_SELECT_END)
    f.write(op_test_loop)

def gen_test_main(f, op_types):
    f.write(OP_TEST_MAIN_BEGIN)
    for op_type in op_types:
        f.write(OP_TEST_MAIN_IF.format(op_type=op_type))
    f.write(OP_TEST_MAIN_END)


# def gen_test_op_select(f, op_type, op_list):
#     ops_cfg = ops[op_type]
#     op_args = ops_cfg[2]
#     f.write(OP_SELECT_BEGIN.format(op_type=op_type, op_args=op_args))
#     for op in op_list:
#         f.write(OP_SELECT_IF.format(op=op))
#     f.write(OP_SELECT_END)
# 
# def gen_test_op_k_select(f, op_type, op, k_range):
#     ops_cfg = ops[op_type]
#     op_args = ops_cfg[2]
#     f.write(OP_K_SELECT_BEGIN.format(op_type=op_type, op_args=op_args))
#     for k in range(k_range):
#         f.write(OP_K_SELECT_IF.format(op=op, k=k))
#     f.write(OP_K_SELECT_END)
