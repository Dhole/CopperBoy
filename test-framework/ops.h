
void op_add(uint8_t* a, uint8_t b, uint8_t *sreg) {
	SREG = 0;
	asm volatile(
		"add %0, %3 \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a), "r" (b)
		:
	);
}

void op_and(uint8_t* a, uint8_t b, uint8_t *sreg) {
	SREG = 0;
	asm volatile(
		"and %0, %3 \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a), "r" (b)
		:
	);
}

void op_cp(uint8_t* a, uint8_t b, uint8_t *sreg) {
	SREG = 0;
	asm volatile(
		"cp %0, %3 \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a), "r" (b)
		:
	);
}

void op_eor(uint8_t* a, uint8_t b, uint8_t *sreg) {
	SREG = 0;
	asm volatile(
		"eor %0, %3 \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a), "r" (b)
		:
	);
}

void op_mov(uint8_t* a, uint8_t b, uint8_t *sreg) {
	SREG = 0;
	asm volatile(
		"mov %0, %3 \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a), "r" (b)
		:
	);
}

void op_or(uint8_t* a, uint8_t b, uint8_t *sreg) {
	SREG = 0;
	asm volatile(
		"or %0, %3 \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a), "r" (b)
		:
	);
}

void op_sub(uint8_t* a, uint8_t b, uint8_t *sreg) {
	SREG = 0;
	asm volatile(
		"sub %0, %3 \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a), "r" (b)
		:
	);
}

void op_alu0_select(void (**op)(uint8_t* a, uint8_t b, uint8_t *sreg), String cmd) {
	if (cmd.equals("")) {
	} else if (cmd.equals("ADD")) {
		*op = &op_add;
	} else if (cmd.equals("AND")) {
		*op = &op_and;
	} else if (cmd.equals("CP")) {
		*op = &op_cp;
	} else if (cmd.equals("EOR")) {
		*op = &op_eor;
	} else if (cmd.equals("MOV")) {
		*op = &op_mov;
	} else if (cmd.equals("OR")) {
		*op = &op_or;
	} else if (cmd.equals("SUB")) {
		*op = &op_sub;
	} else {
	}
}

void op_adc(uint8_t* a, uint8_t b, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"adc %0, %3 \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a), "r" (b)
		:
	);
}

void op_cpc(uint8_t* a, uint8_t b, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"cpc %0, %3 \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a), "r" (b)
		:
	);
}

void op_sbc(uint8_t* a, uint8_t b, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbc %0, %3 \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a), "r" (b)
		:
	);
}

void op_alu1_select(void (**op)(uint8_t* a, uint8_t b, uint8_t *sreg), String cmd) {
	if (cmd.equals("")) {
	} else if (cmd.equals("ADC")) {
		*op = &op_adc;
	} else if (cmd.equals("CPC")) {
		*op = &op_cpc;
	} else if (cmd.equals("SBC")) {
		*op = &op_sbc;
	} else {
	}
}

void op_com(uint8_t* a, uint8_t *sreg) {
	SREG = 0;
	asm volatile(
		"com %0 \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_neg(uint8_t* a, uint8_t *sreg) {
	SREG = 0;
	asm volatile(
		"neg %0 \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_inc(uint8_t* a, uint8_t *sreg) {
	SREG = 0;
	asm volatile(
		"inc %0 \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_dec(uint8_t* a, uint8_t *sreg) {
	SREG = 0;
	asm volatile(
		"dec %0 \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_ser(uint8_t* a, uint8_t *sreg) {
	SREG = 0;
	asm volatile(
		"ser %0 \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_asr(uint8_t* a, uint8_t *sreg) {
	SREG = 0;
	asm volatile(
		"asr %0 \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_swap(uint8_t* a, uint8_t *sreg) {
	SREG = 0;
	asm volatile(
		"swap %0 \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_alu2_select(void (**op)(uint8_t* a, uint8_t *sreg), String cmd) {
	if (cmd.equals("")) {
	} else if (cmd.equals("COM")) {
		*op = &op_com;
	} else if (cmd.equals("NEG")) {
		*op = &op_neg;
	} else if (cmd.equals("INC")) {
		*op = &op_inc;
	} else if (cmd.equals("DEC")) {
		*op = &op_dec;
	} else if (cmd.equals("SER")) {
		*op = &op_ser;
	} else if (cmd.equals("ASR")) {
		*op = &op_asr;
	} else if (cmd.equals("SWAP")) {
		*op = &op_swap;
	} else {
	}
}

void op_ror(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"ror %0 \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_alu3_select(void (**op)(uint8_t* a, uint8_t *sreg), String cmd) {
	if (cmd.equals("")) {
	} else if (cmd.equals("ROR")) {
		*op = &op_ror;
	} else {
	}
}
