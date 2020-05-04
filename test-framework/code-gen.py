#!/usr/bin/env python3

ops_alu0 = ['ADD', 'AND', 'CP', 'EOR', 'MOV', 'OR', 'SUB']
ops_alu1 = ['ADC', 'CPC', 'SBC']

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

ops = {
        'alu0': (ops_alu0, OP_ALU0_TEST, OP_ALU0_ARGS),
        'alu1': (ops_alu1, OP_ALU1_TEST, OP_ALU1_ARGS),
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
