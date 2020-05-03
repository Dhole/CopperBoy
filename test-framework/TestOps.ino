void setup() {
	// Serial.begin(115200);
	Serial.begin(1000000);
	Serial.print("HELLO\n");
}

const char* CMD_START = "START";

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

void loop_main() {
	String cmd;
	while (true) {
		if (Serial.available()) {
			cmd = Serial.readStringUntil('\n');
			break;
		}
	}
	void (*op)(uint8_t* a, uint8_t b, uint8_t *sreg);
	if (cmd.equals("ADD")) {
		op = &op_add;
	} else if (cmd.equals("AND")) {
		op = &op_and;
	} else {
		op = &op_add;
	}
	uint8_t buf[16];
	uint8_t a, b, sreg;
	int i, j;
	for (i = 0; i < 0x100; i++) {
		for (j = 0; j < 0x100; j++) {
			a = i;
			b = j;
			op(&a, b, &sreg);
			buf[0] = i; buf[1] = j; buf[2] = a; buf[3] = sreg;
			Serial.write(buf, 4);
		}
	}
}

void loop() {
	String cmd;
	while (true) {
		if (Serial.available()) {
			cmd = Serial.readStringUntil('\n');
			if (cmd.equals(CMD_START)) {
				break;
			}
		}
	}
	Serial.print("OK");
	while (true) {
		loop_main();
	}
}
