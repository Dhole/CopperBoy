
void op_mul(uint8_t* a, uint8_t* b, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
                "push r0 \n"
                "push r1 \n"
		"mul %0, %1 \n"
		"in %2, __SREG__ \n"
                "mov %0, r0 \n"
                "mov %1, r1 \n"
                "pop r1 \n"
                "pop r0 \n"
		: "=a" (*a), "=a" (*b), "=r" (*sreg)
		: "0" (*a), "1" (*b)
		:
	);
}

void op_muls(uint8_t* a, uint8_t* b, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
                "push r0 \n"
                "push r1 \n"
		"muls %0, %1 \n"
		"in %2, __SREG__ \n"
                "mov %0, r0 \n"
                "mov %1, r1 \n"
                "pop r1 \n"
                "pop r0 \n"
		: "=a" (*a), "=a" (*b), "=r" (*sreg)
		: "0" (*a), "1" (*b)
		:
	);
}

void op_mulsu(uint8_t* a, uint8_t* b, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
                "push r0 \n"
                "push r1 \n"
		"mulsu %0, %1 \n"
		"in %2, __SREG__ \n"
                "mov %0, r0 \n"
                "mov %1, r1 \n"
                "pop r1 \n"
                "pop r0 \n"
		: "=a" (*a), "=a" (*b), "=r" (*sreg)
		: "0" (*a), "1" (*b)
		:
	);
}

void op_fmul(uint8_t* a, uint8_t* b, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
                "push r0 \n"
                "push r1 \n"
		"fmul %0, %1 \n"
		"in %2, __SREG__ \n"
                "mov %0, r0 \n"
                "mov %1, r1 \n"
                "pop r1 \n"
                "pop r0 \n"
		: "=a" (*a), "=a" (*b), "=r" (*sreg)
		: "0" (*a), "1" (*b)
		:
	);
}

void op_fmuls(uint8_t* a, uint8_t* b, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
                "push r0 \n"
                "push r1 \n"
		"fmuls %0, %1 \n"
		"in %2, __SREG__ \n"
                "mov %0, r0 \n"
                "mov %1, r1 \n"
                "pop r1 \n"
                "pop r0 \n"
		: "=a" (*a), "=a" (*b), "=r" (*sreg)
		: "0" (*a), "1" (*b)
		:
	);
}

void op_fmulsu(uint8_t* a, uint8_t* b, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
                "push r0 \n"
                "push r1 \n"
		"fmulsu %0, %1 \n"
		"in %2, __SREG__ \n"
                "mov %0, r0 \n"
                "mov %1, r1 \n"
                "pop r1 \n"
                "pop r0 \n"
		: "=a" (*a), "=a" (*b), "=r" (*sreg)
		: "0" (*a), "1" (*b)
		:
	);
}

void op_alu4_select(void (**op)(uint8_t* a, uint8_t* b, uint8_t *sreg), String cmd) {
	if (cmd.equals("")) {
	} else if (cmd.equals("mul")) {
		*op = &op_mul;
	} else if (cmd.equals("muls")) {
		*op = &op_muls;
	} else if (cmd.equals("mulsu")) {
		*op = &op_mulsu;
	} else if (cmd.equals("fmul")) {
		*op = &op_fmul;
	} else if (cmd.equals("fmuls")) {
		*op = &op_fmuls;
	} else if (cmd.equals("fmulsu")) {
		*op = &op_fmulsu;
	} else {
	}
}

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

void test_op(String op_type, String op_name) {
	if (op_type.equals("")) {
        } else if (op_type.equals("alu4")) {
		op_test_alu4(op_name);
	} else {
	}
}
