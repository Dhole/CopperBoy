#include "ops.h"

void setup() {
	// Serial.begin(57600);
	// Serial.begin(115200);
	Serial.begin(1000000);
	Serial.print("HELLO\n");
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
	test_op(op_type, op_name);
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
		interrupts();
		loop_main();
	}
}
