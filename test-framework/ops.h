
void op_com(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"com %0 \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_neg(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"neg %0 \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_inc(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"inc %0 \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_dec(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"dec %0 \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_ser(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"ser %0 \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_asr(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"asr %0 \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_swap(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"swap %0 \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_lsr(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"lsr %0 \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_alu1_select(void (**op)(uint8_t* a, uint8_t *sreg), String cmd) {
	if (cmd.equals("")) {
	} else if (cmd.equals("com")) {
		*op = &op_com;
	} else if (cmd.equals("neg")) {
		*op = &op_neg;
	} else if (cmd.equals("inc")) {
		*op = &op_inc;
	} else if (cmd.equals("dec")) {
		*op = &op_dec;
	} else if (cmd.equals("ser")) {
		*op = &op_ser;
	} else if (cmd.equals("asr")) {
		*op = &op_asr;
	} else if (cmd.equals("swap")) {
		*op = &op_swap;
	} else if (cmd.equals("lsr")) {
		*op = &op_lsr;
	} else {
	}
}

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

void test_op(String op_type, String op_name) {
	if (op_type.equals("")) {
        } else if (op_type.equals("alu1")) {
		op_test_alu1(op_name);
	} else {
	}
}
