
void op_sbci_00(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0x00 \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_01(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0x01 \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_02(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0x02 \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_03(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0x03 \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_04(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0x04 \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_05(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0x05 \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_06(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0x06 \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_07(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0x07 \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_08(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0x08 \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_09(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0x09 \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_0a(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0x0a \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_0b(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0x0b \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_0c(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0x0c \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_0d(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0x0d \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_0e(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0x0e \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_0f(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0x0f \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_10(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0x10 \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_11(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0x11 \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_12(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0x12 \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_13(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0x13 \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_14(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0x14 \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_15(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0x15 \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_16(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0x16 \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_17(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0x17 \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_18(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0x18 \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_19(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0x19 \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_1a(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0x1a \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_1b(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0x1b \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_1c(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0x1c \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_1d(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0x1d \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_1e(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0x1e \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_1f(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0x1f \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_20(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0x20 \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_21(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0x21 \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_22(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0x22 \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_23(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0x23 \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_24(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0x24 \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_25(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0x25 \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_26(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0x26 \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_27(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0x27 \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_28(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0x28 \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_29(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0x29 \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_2a(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0x2a \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_2b(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0x2b \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_2c(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0x2c \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_2d(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0x2d \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_2e(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0x2e \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_2f(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0x2f \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_30(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0x30 \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_31(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0x31 \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_32(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0x32 \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_33(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0x33 \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_34(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0x34 \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_35(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0x35 \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_36(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0x36 \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_37(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0x37 \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_38(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0x38 \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_39(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0x39 \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_3a(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0x3a \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_3b(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0x3b \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_3c(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0x3c \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_3d(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0x3d \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_3e(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0x3e \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_3f(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0x3f \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_40(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0x40 \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_41(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0x41 \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_42(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0x42 \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_43(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0x43 \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_44(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0x44 \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_45(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0x45 \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_46(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0x46 \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_47(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0x47 \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_48(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0x48 \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_49(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0x49 \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_4a(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0x4a \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_4b(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0x4b \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_4c(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0x4c \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_4d(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0x4d \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_4e(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0x4e \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_4f(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0x4f \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_50(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0x50 \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_51(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0x51 \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_52(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0x52 \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_53(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0x53 \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_54(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0x54 \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_55(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0x55 \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_56(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0x56 \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_57(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0x57 \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_58(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0x58 \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_59(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0x59 \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_5a(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0x5a \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_5b(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0x5b \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_5c(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0x5c \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_5d(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0x5d \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_5e(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0x5e \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_5f(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0x5f \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_60(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0x60 \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_61(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0x61 \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_62(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0x62 \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_63(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0x63 \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_64(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0x64 \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_65(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0x65 \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_66(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0x66 \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_67(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0x67 \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_68(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0x68 \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_69(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0x69 \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_6a(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0x6a \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_6b(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0x6b \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_6c(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0x6c \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_6d(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0x6d \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_6e(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0x6e \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_6f(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0x6f \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_70(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0x70 \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_71(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0x71 \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_72(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0x72 \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_73(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0x73 \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_74(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0x74 \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_75(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0x75 \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_76(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0x76 \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_77(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0x77 \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_78(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0x78 \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_79(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0x79 \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_7a(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0x7a \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_7b(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0x7b \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_7c(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0x7c \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_7d(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0x7d \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_7e(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0x7e \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_7f(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0x7f \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_80(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0x80 \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_81(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0x81 \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_82(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0x82 \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_83(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0x83 \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_84(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0x84 \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_85(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0x85 \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_86(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0x86 \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_87(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0x87 \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_88(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0x88 \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_89(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0x89 \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_8a(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0x8a \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_8b(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0x8b \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_8c(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0x8c \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_8d(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0x8d \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_8e(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0x8e \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_8f(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0x8f \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_90(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0x90 \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_91(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0x91 \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_92(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0x92 \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_93(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0x93 \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_94(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0x94 \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_95(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0x95 \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_96(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0x96 \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_97(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0x97 \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_98(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0x98 \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_99(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0x99 \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_9a(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0x9a \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_9b(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0x9b \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_9c(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0x9c \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_9d(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0x9d \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_9e(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0x9e \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_9f(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0x9f \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_a0(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0xa0 \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_a1(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0xa1 \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_a2(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0xa2 \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_a3(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0xa3 \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_a4(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0xa4 \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_a5(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0xa5 \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_a6(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0xa6 \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_a7(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0xa7 \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_a8(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0xa8 \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_a9(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0xa9 \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_aa(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0xaa \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_ab(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0xab \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_ac(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0xac \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_ad(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0xad \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_ae(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0xae \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_af(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0xaf \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_b0(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0xb0 \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_b1(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0xb1 \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_b2(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0xb2 \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_b3(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0xb3 \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_b4(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0xb4 \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_b5(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0xb5 \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_b6(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0xb6 \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_b7(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0xb7 \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_b8(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0xb8 \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_b9(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0xb9 \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_ba(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0xba \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_bb(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0xbb \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_bc(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0xbc \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_bd(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0xbd \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_be(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0xbe \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_bf(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0xbf \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_c0(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0xc0 \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_c1(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0xc1 \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_c2(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0xc2 \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_c3(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0xc3 \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_c4(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0xc4 \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_c5(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0xc5 \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_c6(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0xc6 \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_c7(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0xc7 \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_c8(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0xc8 \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_c9(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0xc9 \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_ca(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0xca \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_cb(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0xcb \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_cc(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0xcc \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_cd(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0xcd \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_ce(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0xce \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_cf(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0xcf \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_d0(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0xd0 \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_d1(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0xd1 \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_d2(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0xd2 \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_d3(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0xd3 \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_d4(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0xd4 \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_d5(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0xd5 \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_d6(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0xd6 \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_d7(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0xd7 \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_d8(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0xd8 \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_d9(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0xd9 \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_da(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0xda \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_db(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0xdb \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_dc(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0xdc \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_dd(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0xdd \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_de(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0xde \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_df(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0xdf \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_e0(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0xe0 \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_e1(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0xe1 \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_e2(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0xe2 \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_e3(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0xe3 \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_e4(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0xe4 \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_e5(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0xe5 \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_e6(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0xe6 \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_e7(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0xe7 \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_e8(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0xe8 \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_e9(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0xe9 \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_ea(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0xea \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_eb(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0xeb \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_ec(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0xec \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_ed(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0xed \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_ee(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0xee \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_ef(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0xef \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_f0(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0xf0 \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_f1(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0xf1 \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_f2(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0xf2 \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_f3(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0xf3 \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_f4(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0xf4 \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_f5(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0xf5 \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_f6(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0xf6 \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_f7(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0xf7 \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_f8(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0xf8 \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_f9(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0xf9 \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_fa(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0xfa \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_fb(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0xfb \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_fc(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0xfc \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_fd(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0xfd \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_fe(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0xfe \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_sbci_ff(uint8_t* a, uint8_t *sreg) {
	SREG = *sreg;
	asm volatile(
		"sbci %0, 0xff \n"
		"in %1, __SREG__ \n"
		: "=r" (*a), "=r" (*sreg)
		: "0" (*a)
		:
	);
}

void op_alu2s_select(void (**op)(uint8_t* a, uint8_t *sreg), String cmd) {
	if (cmd.equals("")) {
	} else if (cmd.equals("sbci_00")) {
		*op = &op_sbci_00;
	} else if (cmd.equals("sbci_01")) {
		*op = &op_sbci_01;
	} else if (cmd.equals("sbci_02")) {
		*op = &op_sbci_02;
	} else if (cmd.equals("sbci_03")) {
		*op = &op_sbci_03;
	} else if (cmd.equals("sbci_04")) {
		*op = &op_sbci_04;
	} else if (cmd.equals("sbci_05")) {
		*op = &op_sbci_05;
	} else if (cmd.equals("sbci_06")) {
		*op = &op_sbci_06;
	} else if (cmd.equals("sbci_07")) {
		*op = &op_sbci_07;
	} else if (cmd.equals("sbci_08")) {
		*op = &op_sbci_08;
	} else if (cmd.equals("sbci_09")) {
		*op = &op_sbci_09;
	} else if (cmd.equals("sbci_0a")) {
		*op = &op_sbci_0a;
	} else if (cmd.equals("sbci_0b")) {
		*op = &op_sbci_0b;
	} else if (cmd.equals("sbci_0c")) {
		*op = &op_sbci_0c;
	} else if (cmd.equals("sbci_0d")) {
		*op = &op_sbci_0d;
	} else if (cmd.equals("sbci_0e")) {
		*op = &op_sbci_0e;
	} else if (cmd.equals("sbci_0f")) {
		*op = &op_sbci_0f;
	} else if (cmd.equals("sbci_10")) {
		*op = &op_sbci_10;
	} else if (cmd.equals("sbci_11")) {
		*op = &op_sbci_11;
	} else if (cmd.equals("sbci_12")) {
		*op = &op_sbci_12;
	} else if (cmd.equals("sbci_13")) {
		*op = &op_sbci_13;
	} else if (cmd.equals("sbci_14")) {
		*op = &op_sbci_14;
	} else if (cmd.equals("sbci_15")) {
		*op = &op_sbci_15;
	} else if (cmd.equals("sbci_16")) {
		*op = &op_sbci_16;
	} else if (cmd.equals("sbci_17")) {
		*op = &op_sbci_17;
	} else if (cmd.equals("sbci_18")) {
		*op = &op_sbci_18;
	} else if (cmd.equals("sbci_19")) {
		*op = &op_sbci_19;
	} else if (cmd.equals("sbci_1a")) {
		*op = &op_sbci_1a;
	} else if (cmd.equals("sbci_1b")) {
		*op = &op_sbci_1b;
	} else if (cmd.equals("sbci_1c")) {
		*op = &op_sbci_1c;
	} else if (cmd.equals("sbci_1d")) {
		*op = &op_sbci_1d;
	} else if (cmd.equals("sbci_1e")) {
		*op = &op_sbci_1e;
	} else if (cmd.equals("sbci_1f")) {
		*op = &op_sbci_1f;
	} else if (cmd.equals("sbci_20")) {
		*op = &op_sbci_20;
	} else if (cmd.equals("sbci_21")) {
		*op = &op_sbci_21;
	} else if (cmd.equals("sbci_22")) {
		*op = &op_sbci_22;
	} else if (cmd.equals("sbci_23")) {
		*op = &op_sbci_23;
	} else if (cmd.equals("sbci_24")) {
		*op = &op_sbci_24;
	} else if (cmd.equals("sbci_25")) {
		*op = &op_sbci_25;
	} else if (cmd.equals("sbci_26")) {
		*op = &op_sbci_26;
	} else if (cmd.equals("sbci_27")) {
		*op = &op_sbci_27;
	} else if (cmd.equals("sbci_28")) {
		*op = &op_sbci_28;
	} else if (cmd.equals("sbci_29")) {
		*op = &op_sbci_29;
	} else if (cmd.equals("sbci_2a")) {
		*op = &op_sbci_2a;
	} else if (cmd.equals("sbci_2b")) {
		*op = &op_sbci_2b;
	} else if (cmd.equals("sbci_2c")) {
		*op = &op_sbci_2c;
	} else if (cmd.equals("sbci_2d")) {
		*op = &op_sbci_2d;
	} else if (cmd.equals("sbci_2e")) {
		*op = &op_sbci_2e;
	} else if (cmd.equals("sbci_2f")) {
		*op = &op_sbci_2f;
	} else if (cmd.equals("sbci_30")) {
		*op = &op_sbci_30;
	} else if (cmd.equals("sbci_31")) {
		*op = &op_sbci_31;
	} else if (cmd.equals("sbci_32")) {
		*op = &op_sbci_32;
	} else if (cmd.equals("sbci_33")) {
		*op = &op_sbci_33;
	} else if (cmd.equals("sbci_34")) {
		*op = &op_sbci_34;
	} else if (cmd.equals("sbci_35")) {
		*op = &op_sbci_35;
	} else if (cmd.equals("sbci_36")) {
		*op = &op_sbci_36;
	} else if (cmd.equals("sbci_37")) {
		*op = &op_sbci_37;
	} else if (cmd.equals("sbci_38")) {
		*op = &op_sbci_38;
	} else if (cmd.equals("sbci_39")) {
		*op = &op_sbci_39;
	} else if (cmd.equals("sbci_3a")) {
		*op = &op_sbci_3a;
	} else if (cmd.equals("sbci_3b")) {
		*op = &op_sbci_3b;
	} else if (cmd.equals("sbci_3c")) {
		*op = &op_sbci_3c;
	} else if (cmd.equals("sbci_3d")) {
		*op = &op_sbci_3d;
	} else if (cmd.equals("sbci_3e")) {
		*op = &op_sbci_3e;
	} else if (cmd.equals("sbci_3f")) {
		*op = &op_sbci_3f;
	} else if (cmd.equals("sbci_40")) {
		*op = &op_sbci_40;
	} else if (cmd.equals("sbci_41")) {
		*op = &op_sbci_41;
	} else if (cmd.equals("sbci_42")) {
		*op = &op_sbci_42;
	} else if (cmd.equals("sbci_43")) {
		*op = &op_sbci_43;
	} else if (cmd.equals("sbci_44")) {
		*op = &op_sbci_44;
	} else if (cmd.equals("sbci_45")) {
		*op = &op_sbci_45;
	} else if (cmd.equals("sbci_46")) {
		*op = &op_sbci_46;
	} else if (cmd.equals("sbci_47")) {
		*op = &op_sbci_47;
	} else if (cmd.equals("sbci_48")) {
		*op = &op_sbci_48;
	} else if (cmd.equals("sbci_49")) {
		*op = &op_sbci_49;
	} else if (cmd.equals("sbci_4a")) {
		*op = &op_sbci_4a;
	} else if (cmd.equals("sbci_4b")) {
		*op = &op_sbci_4b;
	} else if (cmd.equals("sbci_4c")) {
		*op = &op_sbci_4c;
	} else if (cmd.equals("sbci_4d")) {
		*op = &op_sbci_4d;
	} else if (cmd.equals("sbci_4e")) {
		*op = &op_sbci_4e;
	} else if (cmd.equals("sbci_4f")) {
		*op = &op_sbci_4f;
	} else if (cmd.equals("sbci_50")) {
		*op = &op_sbci_50;
	} else if (cmd.equals("sbci_51")) {
		*op = &op_sbci_51;
	} else if (cmd.equals("sbci_52")) {
		*op = &op_sbci_52;
	} else if (cmd.equals("sbci_53")) {
		*op = &op_sbci_53;
	} else if (cmd.equals("sbci_54")) {
		*op = &op_sbci_54;
	} else if (cmd.equals("sbci_55")) {
		*op = &op_sbci_55;
	} else if (cmd.equals("sbci_56")) {
		*op = &op_sbci_56;
	} else if (cmd.equals("sbci_57")) {
		*op = &op_sbci_57;
	} else if (cmd.equals("sbci_58")) {
		*op = &op_sbci_58;
	} else if (cmd.equals("sbci_59")) {
		*op = &op_sbci_59;
	} else if (cmd.equals("sbci_5a")) {
		*op = &op_sbci_5a;
	} else if (cmd.equals("sbci_5b")) {
		*op = &op_sbci_5b;
	} else if (cmd.equals("sbci_5c")) {
		*op = &op_sbci_5c;
	} else if (cmd.equals("sbci_5d")) {
		*op = &op_sbci_5d;
	} else if (cmd.equals("sbci_5e")) {
		*op = &op_sbci_5e;
	} else if (cmd.equals("sbci_5f")) {
		*op = &op_sbci_5f;
	} else if (cmd.equals("sbci_60")) {
		*op = &op_sbci_60;
	} else if (cmd.equals("sbci_61")) {
		*op = &op_sbci_61;
	} else if (cmd.equals("sbci_62")) {
		*op = &op_sbci_62;
	} else if (cmd.equals("sbci_63")) {
		*op = &op_sbci_63;
	} else if (cmd.equals("sbci_64")) {
		*op = &op_sbci_64;
	} else if (cmd.equals("sbci_65")) {
		*op = &op_sbci_65;
	} else if (cmd.equals("sbci_66")) {
		*op = &op_sbci_66;
	} else if (cmd.equals("sbci_67")) {
		*op = &op_sbci_67;
	} else if (cmd.equals("sbci_68")) {
		*op = &op_sbci_68;
	} else if (cmd.equals("sbci_69")) {
		*op = &op_sbci_69;
	} else if (cmd.equals("sbci_6a")) {
		*op = &op_sbci_6a;
	} else if (cmd.equals("sbci_6b")) {
		*op = &op_sbci_6b;
	} else if (cmd.equals("sbci_6c")) {
		*op = &op_sbci_6c;
	} else if (cmd.equals("sbci_6d")) {
		*op = &op_sbci_6d;
	} else if (cmd.equals("sbci_6e")) {
		*op = &op_sbci_6e;
	} else if (cmd.equals("sbci_6f")) {
		*op = &op_sbci_6f;
	} else if (cmd.equals("sbci_70")) {
		*op = &op_sbci_70;
	} else if (cmd.equals("sbci_71")) {
		*op = &op_sbci_71;
	} else if (cmd.equals("sbci_72")) {
		*op = &op_sbci_72;
	} else if (cmd.equals("sbci_73")) {
		*op = &op_sbci_73;
	} else if (cmd.equals("sbci_74")) {
		*op = &op_sbci_74;
	} else if (cmd.equals("sbci_75")) {
		*op = &op_sbci_75;
	} else if (cmd.equals("sbci_76")) {
		*op = &op_sbci_76;
	} else if (cmd.equals("sbci_77")) {
		*op = &op_sbci_77;
	} else if (cmd.equals("sbci_78")) {
		*op = &op_sbci_78;
	} else if (cmd.equals("sbci_79")) {
		*op = &op_sbci_79;
	} else if (cmd.equals("sbci_7a")) {
		*op = &op_sbci_7a;
	} else if (cmd.equals("sbci_7b")) {
		*op = &op_sbci_7b;
	} else if (cmd.equals("sbci_7c")) {
		*op = &op_sbci_7c;
	} else if (cmd.equals("sbci_7d")) {
		*op = &op_sbci_7d;
	} else if (cmd.equals("sbci_7e")) {
		*op = &op_sbci_7e;
	} else if (cmd.equals("sbci_7f")) {
		*op = &op_sbci_7f;
	} else if (cmd.equals("sbci_80")) {
		*op = &op_sbci_80;
	} else if (cmd.equals("sbci_81")) {
		*op = &op_sbci_81;
	} else if (cmd.equals("sbci_82")) {
		*op = &op_sbci_82;
	} else if (cmd.equals("sbci_83")) {
		*op = &op_sbci_83;
	} else if (cmd.equals("sbci_84")) {
		*op = &op_sbci_84;
	} else if (cmd.equals("sbci_85")) {
		*op = &op_sbci_85;
	} else if (cmd.equals("sbci_86")) {
		*op = &op_sbci_86;
	} else if (cmd.equals("sbci_87")) {
		*op = &op_sbci_87;
	} else if (cmd.equals("sbci_88")) {
		*op = &op_sbci_88;
	} else if (cmd.equals("sbci_89")) {
		*op = &op_sbci_89;
	} else if (cmd.equals("sbci_8a")) {
		*op = &op_sbci_8a;
	} else if (cmd.equals("sbci_8b")) {
		*op = &op_sbci_8b;
	} else if (cmd.equals("sbci_8c")) {
		*op = &op_sbci_8c;
	} else if (cmd.equals("sbci_8d")) {
		*op = &op_sbci_8d;
	} else if (cmd.equals("sbci_8e")) {
		*op = &op_sbci_8e;
	} else if (cmd.equals("sbci_8f")) {
		*op = &op_sbci_8f;
	} else if (cmd.equals("sbci_90")) {
		*op = &op_sbci_90;
	} else if (cmd.equals("sbci_91")) {
		*op = &op_sbci_91;
	} else if (cmd.equals("sbci_92")) {
		*op = &op_sbci_92;
	} else if (cmd.equals("sbci_93")) {
		*op = &op_sbci_93;
	} else if (cmd.equals("sbci_94")) {
		*op = &op_sbci_94;
	} else if (cmd.equals("sbci_95")) {
		*op = &op_sbci_95;
	} else if (cmd.equals("sbci_96")) {
		*op = &op_sbci_96;
	} else if (cmd.equals("sbci_97")) {
		*op = &op_sbci_97;
	} else if (cmd.equals("sbci_98")) {
		*op = &op_sbci_98;
	} else if (cmd.equals("sbci_99")) {
		*op = &op_sbci_99;
	} else if (cmd.equals("sbci_9a")) {
		*op = &op_sbci_9a;
	} else if (cmd.equals("sbci_9b")) {
		*op = &op_sbci_9b;
	} else if (cmd.equals("sbci_9c")) {
		*op = &op_sbci_9c;
	} else if (cmd.equals("sbci_9d")) {
		*op = &op_sbci_9d;
	} else if (cmd.equals("sbci_9e")) {
		*op = &op_sbci_9e;
	} else if (cmd.equals("sbci_9f")) {
		*op = &op_sbci_9f;
	} else if (cmd.equals("sbci_a0")) {
		*op = &op_sbci_a0;
	} else if (cmd.equals("sbci_a1")) {
		*op = &op_sbci_a1;
	} else if (cmd.equals("sbci_a2")) {
		*op = &op_sbci_a2;
	} else if (cmd.equals("sbci_a3")) {
		*op = &op_sbci_a3;
	} else if (cmd.equals("sbci_a4")) {
		*op = &op_sbci_a4;
	} else if (cmd.equals("sbci_a5")) {
		*op = &op_sbci_a5;
	} else if (cmd.equals("sbci_a6")) {
		*op = &op_sbci_a6;
	} else if (cmd.equals("sbci_a7")) {
		*op = &op_sbci_a7;
	} else if (cmd.equals("sbci_a8")) {
		*op = &op_sbci_a8;
	} else if (cmd.equals("sbci_a9")) {
		*op = &op_sbci_a9;
	} else if (cmd.equals("sbci_aa")) {
		*op = &op_sbci_aa;
	} else if (cmd.equals("sbci_ab")) {
		*op = &op_sbci_ab;
	} else if (cmd.equals("sbci_ac")) {
		*op = &op_sbci_ac;
	} else if (cmd.equals("sbci_ad")) {
		*op = &op_sbci_ad;
	} else if (cmd.equals("sbci_ae")) {
		*op = &op_sbci_ae;
	} else if (cmd.equals("sbci_af")) {
		*op = &op_sbci_af;
	} else if (cmd.equals("sbci_b0")) {
		*op = &op_sbci_b0;
	} else if (cmd.equals("sbci_b1")) {
		*op = &op_sbci_b1;
	} else if (cmd.equals("sbci_b2")) {
		*op = &op_sbci_b2;
	} else if (cmd.equals("sbci_b3")) {
		*op = &op_sbci_b3;
	} else if (cmd.equals("sbci_b4")) {
		*op = &op_sbci_b4;
	} else if (cmd.equals("sbci_b5")) {
		*op = &op_sbci_b5;
	} else if (cmd.equals("sbci_b6")) {
		*op = &op_sbci_b6;
	} else if (cmd.equals("sbci_b7")) {
		*op = &op_sbci_b7;
	} else if (cmd.equals("sbci_b8")) {
		*op = &op_sbci_b8;
	} else if (cmd.equals("sbci_b9")) {
		*op = &op_sbci_b9;
	} else if (cmd.equals("sbci_ba")) {
		*op = &op_sbci_ba;
	} else if (cmd.equals("sbci_bb")) {
		*op = &op_sbci_bb;
	} else if (cmd.equals("sbci_bc")) {
		*op = &op_sbci_bc;
	} else if (cmd.equals("sbci_bd")) {
		*op = &op_sbci_bd;
	} else if (cmd.equals("sbci_be")) {
		*op = &op_sbci_be;
	} else if (cmd.equals("sbci_bf")) {
		*op = &op_sbci_bf;
	} else if (cmd.equals("sbci_c0")) {
		*op = &op_sbci_c0;
	} else if (cmd.equals("sbci_c1")) {
		*op = &op_sbci_c1;
	} else if (cmd.equals("sbci_c2")) {
		*op = &op_sbci_c2;
	} else if (cmd.equals("sbci_c3")) {
		*op = &op_sbci_c3;
	} else if (cmd.equals("sbci_c4")) {
		*op = &op_sbci_c4;
	} else if (cmd.equals("sbci_c5")) {
		*op = &op_sbci_c5;
	} else if (cmd.equals("sbci_c6")) {
		*op = &op_sbci_c6;
	} else if (cmd.equals("sbci_c7")) {
		*op = &op_sbci_c7;
	} else if (cmd.equals("sbci_c8")) {
		*op = &op_sbci_c8;
	} else if (cmd.equals("sbci_c9")) {
		*op = &op_sbci_c9;
	} else if (cmd.equals("sbci_ca")) {
		*op = &op_sbci_ca;
	} else if (cmd.equals("sbci_cb")) {
		*op = &op_sbci_cb;
	} else if (cmd.equals("sbci_cc")) {
		*op = &op_sbci_cc;
	} else if (cmd.equals("sbci_cd")) {
		*op = &op_sbci_cd;
	} else if (cmd.equals("sbci_ce")) {
		*op = &op_sbci_ce;
	} else if (cmd.equals("sbci_cf")) {
		*op = &op_sbci_cf;
	} else if (cmd.equals("sbci_d0")) {
		*op = &op_sbci_d0;
	} else if (cmd.equals("sbci_d1")) {
		*op = &op_sbci_d1;
	} else if (cmd.equals("sbci_d2")) {
		*op = &op_sbci_d2;
	} else if (cmd.equals("sbci_d3")) {
		*op = &op_sbci_d3;
	} else if (cmd.equals("sbci_d4")) {
		*op = &op_sbci_d4;
	} else if (cmd.equals("sbci_d5")) {
		*op = &op_sbci_d5;
	} else if (cmd.equals("sbci_d6")) {
		*op = &op_sbci_d6;
	} else if (cmd.equals("sbci_d7")) {
		*op = &op_sbci_d7;
	} else if (cmd.equals("sbci_d8")) {
		*op = &op_sbci_d8;
	} else if (cmd.equals("sbci_d9")) {
		*op = &op_sbci_d9;
	} else if (cmd.equals("sbci_da")) {
		*op = &op_sbci_da;
	} else if (cmd.equals("sbci_db")) {
		*op = &op_sbci_db;
	} else if (cmd.equals("sbci_dc")) {
		*op = &op_sbci_dc;
	} else if (cmd.equals("sbci_dd")) {
		*op = &op_sbci_dd;
	} else if (cmd.equals("sbci_de")) {
		*op = &op_sbci_de;
	} else if (cmd.equals("sbci_df")) {
		*op = &op_sbci_df;
	} else if (cmd.equals("sbci_e0")) {
		*op = &op_sbci_e0;
	} else if (cmd.equals("sbci_e1")) {
		*op = &op_sbci_e1;
	} else if (cmd.equals("sbci_e2")) {
		*op = &op_sbci_e2;
	} else if (cmd.equals("sbci_e3")) {
		*op = &op_sbci_e3;
	} else if (cmd.equals("sbci_e4")) {
		*op = &op_sbci_e4;
	} else if (cmd.equals("sbci_e5")) {
		*op = &op_sbci_e5;
	} else if (cmd.equals("sbci_e6")) {
		*op = &op_sbci_e6;
	} else if (cmd.equals("sbci_e7")) {
		*op = &op_sbci_e7;
	} else if (cmd.equals("sbci_e8")) {
		*op = &op_sbci_e8;
	} else if (cmd.equals("sbci_e9")) {
		*op = &op_sbci_e9;
	} else if (cmd.equals("sbci_ea")) {
		*op = &op_sbci_ea;
	} else if (cmd.equals("sbci_eb")) {
		*op = &op_sbci_eb;
	} else if (cmd.equals("sbci_ec")) {
		*op = &op_sbci_ec;
	} else if (cmd.equals("sbci_ed")) {
		*op = &op_sbci_ed;
	} else if (cmd.equals("sbci_ee")) {
		*op = &op_sbci_ee;
	} else if (cmd.equals("sbci_ef")) {
		*op = &op_sbci_ef;
	} else if (cmd.equals("sbci_f0")) {
		*op = &op_sbci_f0;
	} else if (cmd.equals("sbci_f1")) {
		*op = &op_sbci_f1;
	} else if (cmd.equals("sbci_f2")) {
		*op = &op_sbci_f2;
	} else if (cmd.equals("sbci_f3")) {
		*op = &op_sbci_f3;
	} else if (cmd.equals("sbci_f4")) {
		*op = &op_sbci_f4;
	} else if (cmd.equals("sbci_f5")) {
		*op = &op_sbci_f5;
	} else if (cmd.equals("sbci_f6")) {
		*op = &op_sbci_f6;
	} else if (cmd.equals("sbci_f7")) {
		*op = &op_sbci_f7;
	} else if (cmd.equals("sbci_f8")) {
		*op = &op_sbci_f8;
	} else if (cmd.equals("sbci_f9")) {
		*op = &op_sbci_f9;
	} else if (cmd.equals("sbci_fa")) {
		*op = &op_sbci_fa;
	} else if (cmd.equals("sbci_fb")) {
		*op = &op_sbci_fb;
	} else if (cmd.equals("sbci_fc")) {
		*op = &op_sbci_fc;
	} else if (cmd.equals("sbci_fd")) {
		*op = &op_sbci_fd;
	} else if (cmd.equals("sbci_fe")) {
		*op = &op_sbci_fe;
	} else if (cmd.equals("sbci_ff")) {
		*op = &op_sbci_ff;
	} else {
	}
}

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

void test_op(String op_type, String op_name) {
	if (op_type.equals("")) {
        } else if (op_type.equals("alu2s")) {
		op_test_alu2s(op_name);
	} else {
	}
}
