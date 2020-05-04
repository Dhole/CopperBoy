#include "ops.h"

void setup() {
	// Serial.begin(115200);
	Serial.begin(1000000);
	Serial.print("HELLO\n");
}

void op_test_alu0(String op_name) {
	void (*op)(uint8_t* a, uint8_t b, uint8_t *sreg);
	op_alu0_select(&op, op_name);
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

void op_test_alu1(String op_name) {
	void (*op)(uint8_t* a, uint8_t b, uint8_t *sreg1);
	op_alu1_select(&op, op_name);
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

void loop_main() {
	String op_type;
	while (true) {
		if (Serial.available()) {
			op_type = Serial.readStringUntil('\n');
			break;
		}
	}
	String op_name;
	while (true) {
		if (Serial.available()) {
			op_name = Serial.readStringUntil('\n');
			break;
		}
	}
	if (op_type.equals("")) {
	} else if (op_type.equals("ALU0")) {
		op_test_alu0(op_name);
	} else if (op_type.equals("ALU1")) {
		op_test_alu1(op_name);
	} else {
	}
}

void loop() {
	String cmd;
	while (true) {
		if (Serial.available()) {
			cmd = Serial.readStringUntil('\n');
			if (cmd.equals("START")) {
				break;
			}
		}
	}
	Serial.print("OK");
	while (true) {
		loop_main();
	}
}
