void setup() {
	Serial.begin(115200);
	Serial.print("HELLO\n");
}

const char* CMD_START = "START";

void op_add(uint8_t* a, uint8_t b, uint8_t *sreg) {
	asm volatile(
		"add %0, %3 \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a), "r" (b)
		:
	);
}

void loop() {
	String cmd;
	while (true) {
		if (Serial.available()) {
			cmd = Serial.readStringUntil('\n');
			if (cmd == CMD_START) {
				break;
			}
		}
	}
	Serial.print("OK");
	while (true) {
		if (Serial.available()) {
			cmd = Serial.readStringUntil('\n');
			break;
		}
	}
	void (*op)(uint8_t* a, uint8_t b, uint8_t *sreg);
	if (cmd.equals("ADD")) {
		op = &op_add;
	} else {
		op = &op_add;
	}
	uint8_t a, b, sreg;
	int i, j;
	for (i = 0; i < 0x100; i++) {
		for (j = 0; j < 0x100; j++) {
			a = i;
			b = j;
			op(&a, b, &sreg);
			Serial.write(i);
			Serial.write(j);
			Serial.write(a);
			Serial.write(sreg);
		}
	}

	// uint8_t a = 0xff;
	// uint8_t b = 0x01;
	// // Serial.write(a);
	// // asm(
	// // 	"add %0, %1 \n"
	// // 	: "=r" (a)
	// // 	: "r" (b)
	// // 	:
	// // );
	// uint8_t sreg = 0xff;
	// // asm volatile(
	// // 	"add %0, %3 \n"
	// // 	"in %1, __SREG__ \n"
	// // 	: "=r" (a), "=r" (sreg)
	// // 	: "0" (a), "r" (b)
	// // 	:
	// // );
	// op_add(&a, b, &sreg);
	// Serial.write(a);
	// Serial.write(sreg);
}
