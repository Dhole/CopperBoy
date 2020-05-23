use super::*;

#[test]
#[rustfmt::skip]
fn test_decode() {
    // Adc { d: u8, r: u8 },
    assert_eq!(Op::Adc { d: 00, r: 00 }, Op::decode(0b0001_1100_0000_0000, 0));
    assert_eq!(Op::Adc { d: 00, r: 31 }, Op::decode(0b0001_1110_0000_1111, 0));
    assert_eq!(Op::Adc { d: 31, r: 00 }, Op::decode(0b0001_1101_1111_0000, 0));
    assert_eq!(Op::Adc { d: 31, r: 31 }, Op::decode(0b0001_1111_1111_1111, 0));

    // Add { d: u8, r: u8 },
    assert_eq!(Op::Add { d: 00, r: 00 }, Op::decode(0b0000_1100_0000_0000, 0));
    assert_eq!(Op::Add { d: 00, r: 31 }, Op::decode(0b0000_1110_0000_1111, 0));
    assert_eq!(Op::Add { d: 31, r: 00 }, Op::decode(0b0000_1101_1111_0000, 0));
    assert_eq!(Op::Add { d: 31, r: 31 }, Op::decode(0b0000_1111_1111_1111, 0));

    // Adiw { d: u8, k: u8 },
    assert_eq!(Op::Adiw { d: 24, k: 00 }, Op::decode(0b1001_0110_0000_0000, 0));
    assert_eq!(Op::Adiw { d: 24, k: 63 }, Op::decode(0b1001_0110_1100_1111, 0));
    assert_eq!(Op::Adiw { d: 30, k: 00 }, Op::decode(0b1001_0110_0011_0000, 0));
    assert_eq!(Op::Adiw { d: 30, k: 63 }, Op::decode(0b1001_0110_1111_1111, 0));

    // And { d: u8, r: u8 },
    assert_eq!(Op::And { d: 00, r: 00 }, Op::decode(0b0010_0000_0000_0000, 0));
    assert_eq!(Op::And { d: 00, r: 31 }, Op::decode(0b0010_0010_0000_1111, 0));
    assert_eq!(Op::And { d: 31, r: 00 }, Op::decode(0b0010_0001_1111_0000, 0));
    assert_eq!(Op::And { d: 31, r: 31 }, Op::decode(0b0010_0011_1111_1111, 0));

    // Andi { d: u8, k: u8 },
    assert_eq!(Op::Andi { d: 16, k: 000 }, Op::decode(0b0111_0000_0000_0000, 0));
    assert_eq!(Op::Andi { d: 16, k: 255 }, Op::decode(0b0111_1111_0000_1111, 0));
    assert_eq!(Op::Andi { d: 31, k: 000 }, Op::decode(0b0111_0000_1111_0000, 0));
    assert_eq!(Op::Andi { d: 31, k: 255 }, Op::decode(0b0111_1111_1111_1111, 0));

    // Asr { d: u8 },
    assert_eq!(Op::Asr { d: 00 }, Op::decode(0b1001_0100_0000_0101, 0));
    assert_eq!(Op::Asr { d: 31 }, Op::decode(0b1001_0101_1111_0101, 0));

    // Bclr { s: u8 },
    assert_eq!(Op::Bclr { s: 0 }, Op::decode(0b1001_0100_1000_1000, 0));
    assert_eq!(Op::Bclr { s: 7 }, Op::decode(0b1001_0100_1111_1000, 0));

    // Bld { d: u8, b: u8 },
    assert_eq!(Op::Bld { d: 00, b: 0 }, Op::decode(0b1111_1000_0000_0000, 0));
    assert_eq!(Op::Bld { d: 00, b: 7 }, Op::decode(0b1111_1000_0000_0111, 0));
    assert_eq!(Op::Bld { d: 31, b: 0 }, Op::decode(0b1111_1001_1111_0000, 0));
    assert_eq!(Op::Bld { d: 31, b: 7 }, Op::decode(0b1111_1001_1111_0111, 0));

    // Brbc { s: u8, k: i8 },
    assert_eq!(Op::Brbc { s: 0, k:  00 }, Op::decode(0b1111_0100_0000_0000, 0));
    assert_eq!(Op::Brbc { s: 0, k: -01 }, Op::decode(0b1111_0111_1111_1000, 0));
    assert_eq!(Op::Brbc { s: 0, k: -64 }, Op::decode(0b1111_0110_0000_0000, 0));
    assert_eq!(Op::Brbc { s: 0, k:  63 }, Op::decode(0b1111_0101_1111_1000, 0));
    assert_eq!(Op::Brbc { s: 7, k:  00 }, Op::decode(0b1111_0100_0000_0111, 0));
    assert_eq!(Op::Brbc { s: 7, k:  63 }, Op::decode(0b1111_0101_1111_1111, 0));

    // Brbs { s: u8, k: i8 },
    assert_eq!(Op::Brbs { s: 0, k:  00 }, Op::decode(0b1111_0000_0000_0000, 0));
    assert_eq!(Op::Brbs { s: 0, k: -01 }, Op::decode(0b1111_0011_1111_1000, 0));
    assert_eq!(Op::Brbs { s: 0, k: -64 }, Op::decode(0b1111_0010_0000_0000, 0));
    assert_eq!(Op::Brbs { s: 0, k:  63 }, Op::decode(0b1111_0001_1111_1000, 0));
    assert_eq!(Op::Brbs { s: 7, k:  00 }, Op::decode(0b1111_0000_0000_0111, 0));
    assert_eq!(Op::Brbs { s: 7, k:  63 }, Op::decode(0b1111_0001_1111_1111, 0));

    // Break,
    assert_eq!(Op::Break, Op::decode(0b1001_0101_1001_1000, 0));

    // Bset { s: u8 },
    assert_eq!(Op::Bset { s: 0 }, Op::decode(0b1001_0100_0000_1000, 0));
    assert_eq!(Op::Bset { s: 7 }, Op::decode(0b1001_0100_0111_1000, 0));

    // Bst { d: u8, b: u8 },
    assert_eq!(Op::Bst { d: 00, b: 0 }, Op::decode(0b1111_1010_0000_0000, 0));
    assert_eq!(Op::Bst { d: 00, b: 7 }, Op::decode(0b1111_1010_0000_0111, 0));
    assert_eq!(Op::Bst { d: 31, b: 0 }, Op::decode(0b1111_1011_1111_0000, 0));
    assert_eq!(Op::Bst { d: 31, b: 7 }, Op::decode(0b1111_1011_1111_0111, 0));

    // Call { k: u32 },
    assert_eq!(Op::Call { k: 0x0000 }, Op::decode(0b1001_0100_0000_1110, 0x00_00));
    assert_eq!(Op::Call { k: 0xffff }, Op::decode(0b1001_0100_1111_1111, 0xff_ff));

    // Cbi { a: u8, b: u8 },
    assert_eq!(Op::Cbi { a: 00, b: 0 }, Op::decode(0b1001_1000_0000_0000, 0));
    assert_eq!(Op::Cbi { a: 00, b: 7 }, Op::decode(0b1001_1000_0000_0111, 0));
    assert_eq!(Op::Cbi { a: 31, b: 0 }, Op::decode(0b1001_1000_1111_1000, 0));
    assert_eq!(Op::Cbi { a: 31, b: 7 }, Op::decode(0b1001_1000_1111_1111, 0));

    // TODO

    // Com { d: u8 },
    // Cp { d: u8, r: u8 },
    assert_eq!(Op::Cp { d: 00, r: 00 }, Op::decode(0b0001_0100_0000_0000, 0));
    assert_eq!(Op::Cp { d: 00, r: 31 }, Op::decode(0b0001_0110_0000_1111, 0));
    assert_eq!(Op::Cp { d: 31, r: 00 }, Op::decode(0b0001_0101_1111_0000, 0));
    assert_eq!(Op::Cp { d: 31, r: 31 }, Op::decode(0b0001_0111_1111_1111, 0));
    // Cpc { d: u8, r: u8 },
    assert_eq!(Op::Cpc { d: 00, r: 00 }, Op::decode(0b0000_0100_0000_0000, 0));
    assert_eq!(Op::Cpc { d: 00, r: 31 }, Op::decode(0b0000_0110_0000_1111, 0));
    assert_eq!(Op::Cpc { d: 31, r: 00 }, Op::decode(0b0000_0101_1111_0000, 0));
    assert_eq!(Op::Cpc { d: 31, r: 31 }, Op::decode(0b0000_0111_1111_1111, 0));
    // Cpi { d: u8, k: u8 },
    // Cpse { d: u8, r: u8 },
    assert_eq!(Op::Cpse { d: 00, r: 00 }, Op::decode(0b0001_0000_0000_0000, 0));
    assert_eq!(Op::Cpse { d: 00, r: 31 }, Op::decode(0b0001_0010_0000_1111, 0));
    assert_eq!(Op::Cpse { d: 31, r: 00 }, Op::decode(0b0001_0001_1111_0000, 0));
    assert_eq!(Op::Cpse { d: 31, r: 31 }, Op::decode(0b0001_0011_1111_1111, 0));
    // Dec { d: u8 },
    // Eicall,
    // Eijmp,
    // Elpmr0,
    // Elpm { d: u8, inc: bool },
    // Eor { d: u8, r: u8 },
    assert_eq!(Op::Eor { d: 00, r: 00 }, Op::decode(0b0010_0100_0000_0000, 0));
    assert_eq!(Op::Eor { d: 00, r: 31 }, Op::decode(0b0010_0110_0000_1111, 0));
    assert_eq!(Op::Eor { d: 31, r: 00 }, Op::decode(0b0010_0101_1111_0000, 0));
    assert_eq!(Op::Eor { d: 31, r: 31 }, Op::decode(0b0010_0111_1111_1111, 0));
    // Fmul { d: u8, r: u8 },
    assert_eq!(Op::Fmul { d: 16, r: 16 }, Op::decode(0b0000_0011_0000_1000, 0));
    assert_eq!(Op::Fmul { d: 16, r: 23 }, Op::decode(0b0000_0011_0000_1111, 0));
    assert_eq!(Op::Fmul { d: 23, r: 16 }, Op::decode(0b0000_0011_0111_1000, 0));
    assert_eq!(Op::Fmul { d: 23, r: 23 }, Op::decode(0b0000_0011_0111_1111, 0));
    // Fmuls { d: u8, r: u8 },
    assert_eq!(Op::Fmuls { d: 16, r: 16 }, Op::decode(0b0000_0011_1000_0000, 0));
    assert_eq!(Op::Fmuls { d: 16, r: 23 }, Op::decode(0b0000_0011_1000_0111, 0));
    assert_eq!(Op::Fmuls { d: 23, r: 16 }, Op::decode(0b0000_0011_1111_0000, 0));
    assert_eq!(Op::Fmuls { d: 23, r: 23 }, Op::decode(0b0000_0011_1111_0111, 0));
    // Fmulsu { d: u8, r: u8 },
    assert_eq!(Op::Fmulsu { d: 16, r: 16 }, Op::decode(0b0000_0011_1000_1000, 0));
    assert_eq!(Op::Fmulsu { d: 16, r: 23 }, Op::decode(0b0000_0011_1000_1111, 0));
    assert_eq!(Op::Fmulsu { d: 23, r: 16 }, Op::decode(0b0000_0011_1111_1000, 0));
    assert_eq!(Op::Fmulsu { d: 23, r: 23 }, Op::decode(0b0000_0011_1111_1111, 0));
    // Icall,
    // Ijmp,
    // In { d: u8, a: u8 },
    // Inc { d: u8 },
    // Jmp { k: u32 },
    // Ld { d: u8, idx: LdStIndex, ext: LdStExt }, // NOTE: Review undefined Rd combinations
    // Ldi { d: u8, k: u8 },
    // Lds { d: u8, k: u16 },
    // Lpmr0,
    // Lpm { d: u8, inc: bool },
    // Lsr { d: u8 },
    // Mov { d: u8, r: u8 },
    assert_eq!(Op::Mov { d: 00, r: 00 }, Op::decode(0b0010_1100_0000_0000, 0));
    assert_eq!(Op::Mov { d: 00, r: 31 }, Op::decode(0b0010_1110_0000_1111, 0));
    assert_eq!(Op::Mov { d: 31, r: 00 }, Op::decode(0b0010_1101_1111_0000, 0));
    assert_eq!(Op::Mov { d: 31, r: 31 }, Op::decode(0b0010_1111_1111_1111, 0));
    // Movw { d: u8, r: u8 },
    assert_eq!(Op::Movw { d: 00, r: 00 }, Op::decode(0b0000_0001_0000_0000, 0));
    assert_eq!(Op::Movw { d: 00, r: 30 }, Op::decode(0b0000_0001_0000_1111, 0));
    assert_eq!(Op::Movw { d: 30, r: 00 }, Op::decode(0b0000_0001_1111_0000, 0));
    assert_eq!(Op::Movw { d: 30, r: 30 }, Op::decode(0b0000_0001_1111_1111, 0));
    // Mul { d: u8, r: u8 },
    assert_eq!(Op::Mul { d: 00, r: 00 }, Op::decode(0b1001_1100_0000_0000, 0));
    assert_eq!(Op::Mul { d: 00, r: 31 }, Op::decode(0b1001_1110_0000_1111, 0));
    assert_eq!(Op::Mul { d: 31, r: 00 }, Op::decode(0b1001_1101_1111_0000, 0));
    assert_eq!(Op::Mul { d: 31, r: 31 }, Op::decode(0b1001_1111_1111_1111, 0));
    // Muls { d: u8, r: u8 },
    assert_eq!(Op::Muls { d: 16, r: 16 }, Op::decode(0b0000_0010_0000_0000, 0));
    assert_eq!(Op::Muls { d: 16, r: 31 }, Op::decode(0b0000_0010_0000_1111, 0));
    assert_eq!(Op::Muls { d: 31, r: 16 }, Op::decode(0b0000_0010_1111_0000, 0));
    assert_eq!(Op::Muls { d: 31, r: 31 }, Op::decode(0b0000_0010_1111_1111, 0));
    // Mulsu { d: u8, r: u8 },
    assert_eq!(Op::Mulsu { d: 16, r: 16 }, Op::decode(0b0000_0011_0000_0000, 0));
    assert_eq!(Op::Mulsu { d: 16, r: 23 }, Op::decode(0b0000_0011_0000_0111, 0));
    assert_eq!(Op::Mulsu { d: 23, r: 16 }, Op::decode(0b0000_0011_0111_0000, 0));
    assert_eq!(Op::Mulsu { d: 23, r: 23 }, Op::decode(0b0000_0011_0111_0111, 0));
    // Neg { d: u8 },
    // Nop,
    // Or { d: u8, r: u8 },
    assert_eq!(Op::Or { d: 00, r: 00 }, Op::decode(0b0010_1000_0000_0000, 0));
    assert_eq!(Op::Or { d: 00, r: 31 }, Op::decode(0b0010_1010_0000_1111, 0));
    assert_eq!(Op::Or { d: 31, r: 00 }, Op::decode(0b0010_1001_1111_0000, 0));
    assert_eq!(Op::Or { d: 31, r: 31 }, Op::decode(0b0010_1011_1111_1111, 0));
    // Ori { d: u8, k: u8 },
    // Out { a: u8, r: u8 },
    // Pop { d: u8 },
    // Push { r: u8 },
    // Rcall { k: i16 },
    // Ret,
    // Reti,
    // Rjmp { k: i16 },
    // Ror { d: u8 },
    // Sbc { d: u8, r: u8 },
    assert_eq!(Op::Sbc { d: 00, r: 00 }, Op::decode(0b0000_1000_0000_0000, 0));
    assert_eq!(Op::Sbc { d: 00, r: 31 }, Op::decode(0b0000_1010_0000_1111, 0));
    assert_eq!(Op::Sbc { d: 31, r: 00 }, Op::decode(0b0000_1001_1111_0000, 0));
    assert_eq!(Op::Sbc { d: 31, r: 31 }, Op::decode(0b0000_1011_1111_1111, 0));
    // Sbci { d: u8, k: u8 },
    // Sbi { a: u8, b: u8 },
    assert_eq!(Op::Sbi { a: 00, b: 0 }, Op::decode(0b1001_1010_0000_0000, 0));
    assert_eq!(Op::Sbi { a: 00, b: 7 }, Op::decode(0b1001_1010_0000_0111, 0));
    assert_eq!(Op::Sbi { a: 31, b: 0 }, Op::decode(0b1001_1010_1111_1000, 0));
    assert_eq!(Op::Sbi { a: 31, b: 7 }, Op::decode(0b1001_1010_1111_1111, 0));

    // Sbic { a: u8, b: u8 },
    assert_eq!(Op::Sbic { a: 00, b: 0 }, Op::decode(0b1001_1001_0000_0000, 0));
    assert_eq!(Op::Sbic { a: 00, b: 7 }, Op::decode(0b1001_1001_0000_0111, 0));
    assert_eq!(Op::Sbic { a: 31, b: 0 }, Op::decode(0b1001_1001_1111_1000, 0));
    assert_eq!(Op::Sbic { a: 31, b: 7 }, Op::decode(0b1001_1001_1111_1111, 0));

    // Sbis { a: u8, b: u8 },
    assert_eq!(Op::Sbis { a: 00, b: 0 }, Op::decode(0b1001_1011_0000_0000, 0));
    assert_eq!(Op::Sbis { a: 00, b: 7 }, Op::decode(0b1001_1011_0000_0111, 0));
    assert_eq!(Op::Sbis { a: 31, b: 0 }, Op::decode(0b1001_1011_1111_1000, 0));
    assert_eq!(Op::Sbis { a: 31, b: 7 }, Op::decode(0b1001_1011_1111_1111, 0));

    // Sbiw { d: u8, k: u8 },
    // Sbrc { r: u8, b: u8 },
    // Sbrs { r: u8, b: u8 },
    // Ser { d: u8 },
    // Sleep,
    // Spm,
    // Spm2,
    // St { r: u8, idx: LdStIndex, ext: LdStExt },
    // Sts { k: u16, r: u8 },
    // Sub { d: u8, r: u8 },
    assert_eq!(Op::Sub { d: 00, r: 00 }, Op::decode(0b0001_1000_0000_0000, 0));
    assert_eq!(Op::Sub { d: 00, r: 31 }, Op::decode(0b0001_1010_0000_1111, 0));
    assert_eq!(Op::Sub { d: 31, r: 00 }, Op::decode(0b0001_1001_1111_0000, 0));
    assert_eq!(Op::Sub { d: 31, r: 31 }, Op::decode(0b0001_1011_1111_1111, 0));
    // Subi { d: u8, k: u8 },
    // Swap { d: u8 },
    // Wdr,
    // Undefined { w: u16 },
}
