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

    // Com { d: u8 },
    assert_eq!(Op::Com { d: 00 }, Op::decode(0b1001_0100_0000_0000, 0));
    assert_eq!(Op::Com { d: 31 }, Op::decode(0b1001_0101_1111_0000, 0));

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
    assert_eq!(Op::Cpi { d: 16, k: 000 }, Op::decode(0b0011_0000_0000_0000, 0));
    assert_eq!(Op::Cpi { d: 16, k: 255 }, Op::decode(0b0011_1111_0000_1111, 0));
    assert_eq!(Op::Cpi { d: 31, k: 000 }, Op::decode(0b0011_0000_1111_0000, 0));
    assert_eq!(Op::Cpi { d: 31, k: 255 }, Op::decode(0b0011_1111_1111_1111, 0));

    // Cpse { d: u8, r: u8 },
    assert_eq!(Op::Cpse { d: 00, r: 00 }, Op::decode(0b0001_0000_0000_0000, 0));
    assert_eq!(Op::Cpse { d: 00, r: 31 }, Op::decode(0b0001_0010_0000_1111, 0));
    assert_eq!(Op::Cpse { d: 31, r: 00 }, Op::decode(0b0001_0001_1111_0000, 0));
    assert_eq!(Op::Cpse { d: 31, r: 31 }, Op::decode(0b0001_0011_1111_1111, 0));

    // Dec { d: u8 },
    assert_eq!(Op::Dec { d: 00 }, Op::decode(0b1001_0100_0000_1010, 0));
    assert_eq!(Op::Dec { d: 31 }, Op::decode(0b1001_0101_1111_1010, 0));

    // Eicall,
    assert_eq!(Op::Eicall, Op::decode(0b1001_0101_0001_1001, 0));

    // Eijmp,
    assert_eq!(Op::Eijmp, Op::decode(0b1001_0100_0001_1001, 0));

    // Elpmr0,
    assert_eq!(Op::Elpmr0, Op::decode(0b1001_0101_1101_1000, 0));

    // Elpm { d: u8, inc: bool },
    assert_eq!(Op::Elpm { d: 00, inc: false }, Op::decode(0b1001_0000_0000_0110, 0));
    assert_eq!(Op::Elpm { d: 00, inc:  true }, Op::decode(0b1001_0000_0000_0111, 0));
    assert_eq!(Op::Elpm { d: 31, inc: false }, Op::decode(0b1001_0001_1111_0110, 0));
    assert_eq!(Op::Elpm { d: 31, inc:  true }, Op::decode(0b1001_0001_1111_0111, 0));

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
    assert_eq!(Op::Icall, Op::decode(0b1001_0101_0000_1001, 0));

    // Ijmp,
    assert_eq!(Op::Ijmp, Op::decode(0b1001_0100_0000_1001, 0));

    // In { d: u8, a: u8 },
    assert_eq!(Op::In { d: 00, a: 00 }, Op::decode(0b1011_0000_0000_0000, 0));
    assert_eq!(Op::In { d: 00, a: 63 }, Op::decode(0b1011_0110_0000_1111, 0));
    assert_eq!(Op::In { d: 31, a: 00 }, Op::decode(0b1011_0001_1111_0000, 0));
    assert_eq!(Op::In { d: 31, a: 63 }, Op::decode(0b1011_0111_1111_1111, 0));

    // Inc { d: u8 },
    assert_eq!(Op::Inc { d: 00 }, Op::decode(0b1001_0100_0000_0011, 0));
    assert_eq!(Op::Inc { d: 31 }, Op::decode(0b1001_0101_1111_0011, 0));

    // Jmp { k: u32 },
    assert_eq!(Op::Jmp { k: 0x0000 }, Op::decode(0b1001_0100_0000_1100, 0x00_00));
    assert_eq!(Op::Jmp { k: 0xffff }, Op::decode(0b1001_0100_1111_1101, 0xff_ff));

    // Ld { d: u8, idx: LdStIndex, ext: LdStExt },
    assert_eq!(Op::Ld { d: 00, idx: LdStIndex::X, ext: LdStExt::None    }, Op::decode(0b1001_0000_0000_1100, 0));
    assert_eq!(Op::Ld { d: 31, idx: LdStIndex::X, ext: LdStExt::None    }, Op::decode(0b1001_0001_1111_1100, 0));
    assert_eq!(Op::Ld { d: 00, idx: LdStIndex::X, ext: LdStExt::PostInc }, Op::decode(0b1001_0000_0000_1101, 0));
    assert_eq!(Op::Ld { d: 31, idx: LdStIndex::X, ext: LdStExt::PostInc }, Op::decode(0b1001_0001_1111_1101, 0));
    assert_eq!(Op::Ld { d: 00, idx: LdStIndex::X, ext: LdStExt::PreDec  }, Op::decode(0b1001_0000_0000_1110, 0));
    assert_eq!(Op::Ld { d: 31, idx: LdStIndex::X, ext: LdStExt::PreDec  }, Op::decode(0b1001_0001_1111_1110, 0));

    assert_eq!(Op::Ld { d: 00, idx: LdStIndex::Y, ext: LdStExt::PostInc          }, Op::decode(0b1001_0000_0000_1001, 0));
    assert_eq!(Op::Ld { d: 31, idx: LdStIndex::Y, ext: LdStExt::PostInc          }, Op::decode(0b1001_0001_1111_1001, 0));
    assert_eq!(Op::Ld { d: 00, idx: LdStIndex::Y, ext: LdStExt::PreDec           }, Op::decode(0b1001_0000_0000_1010, 0));
    assert_eq!(Op::Ld { d: 31, idx: LdStIndex::Y, ext: LdStExt::PreDec           }, Op::decode(0b1001_0001_1111_1010, 0));
    assert_eq!(Op::Ld { d: 00, idx: LdStIndex::Y, ext: LdStExt::Displacement(00) }, Op::decode(0b1000_0000_0000_1000, 0));
    assert_eq!(Op::Ld { d: 31, idx: LdStIndex::Y, ext: LdStExt::Displacement(00) }, Op::decode(0b1000_0001_1111_1000, 0));
    assert_eq!(Op::Ld { d: 00, idx: LdStIndex::Y, ext: LdStExt::Displacement(63) }, Op::decode(0b1010_1100_0000_1111, 0));
    assert_eq!(Op::Ld { d: 31, idx: LdStIndex::Y, ext: LdStExt::Displacement(63) }, Op::decode(0b1010_1101_1111_1111, 0));

    assert_eq!(Op::Ld { d: 00, idx: LdStIndex::Z, ext: LdStExt::PostInc          }, Op::decode(0b1001_0000_0000_0001, 0));
    assert_eq!(Op::Ld { d: 31, idx: LdStIndex::Z, ext: LdStExt::PostInc          }, Op::decode(0b1001_0001_1111_0001, 0));
    assert_eq!(Op::Ld { d: 00, idx: LdStIndex::Z, ext: LdStExt::PreDec           }, Op::decode(0b1001_0000_0000_0010, 0));
    assert_eq!(Op::Ld { d: 31, idx: LdStIndex::Z, ext: LdStExt::PreDec           }, Op::decode(0b1001_0001_1111_0010, 0));
    assert_eq!(Op::Ld { d: 00, idx: LdStIndex::Z, ext: LdStExt::Displacement(00) }, Op::decode(0b1000_0000_0000_0000, 0));
    assert_eq!(Op::Ld { d: 31, idx: LdStIndex::Z, ext: LdStExt::Displacement(00) }, Op::decode(0b1000_0001_1111_0000, 0));
    assert_eq!(Op::Ld { d: 00, idx: LdStIndex::Z, ext: LdStExt::Displacement(63) }, Op::decode(0b1010_1100_0000_0111, 0));
    assert_eq!(Op::Ld { d: 31, idx: LdStIndex::Z, ext: LdStExt::Displacement(63) }, Op::decode(0b1010_1101_1111_0111, 0));

    // Ldi { d: u8, k: u8 },
    assert_eq!(Op::Ldi { d: 16, k: 000 }, Op::decode(0b1110_0000_0000_0000, 0));
    assert_eq!(Op::Ldi { d: 16, k: 255 }, Op::decode(0b1110_1111_0000_1111, 0));
    assert_eq!(Op::Ldi { d: 31, k: 000 }, Op::decode(0b1110_0000_1111_0000, 0));
    assert_eq!(Op::Ldi { d: 31, k: 255 }, Op::decode(0b1110_1111_1111_1111, 0));

    // Lds { d: u8, k: u16 },
    assert_eq!(Op::Lds { d: 00, k: 0x0000 }, Op::decode(0b1001_0000_0000_0000, 0x00_00));
    assert_eq!(Op::Lds { d: 00, k: 0xffff }, Op::decode(0b1001_0000_0000_0000, 0xff_ff));
    assert_eq!(Op::Lds { d: 31, k: 0x0000 }, Op::decode(0b1001_0001_1111_0000, 0x00_00));
    assert_eq!(Op::Lds { d: 31, k: 0xffff }, Op::decode(0b1001_0001_1111_0000, 0xff_ff));

    // Lpmr0,
    assert_eq!(Op::Lpmr0, Op::decode(0b1001_0101_1100_1000, 0));

    // Lpm { d: u8, inc: bool },
    assert_eq!(Op::Lpm { d: 00, inc: false }, Op::decode(0b1001_0000_0000_0100, 0));
    assert_eq!(Op::Lpm { d: 00, inc:  true }, Op::decode(0b1001_0000_0000_0101, 0));
    assert_eq!(Op::Lpm { d: 31, inc: false }, Op::decode(0b1001_0001_1111_0100, 0));
    assert_eq!(Op::Lpm { d: 31, inc:  true }, Op::decode(0b1001_0001_1111_0101, 0));

    // Lsr { d: u8 },
    assert_eq!(Op::Lsr { d: 00 }, Op::decode(0b1001_0100_0000_0110, 0));
    assert_eq!(Op::Lsr { d: 31 }, Op::decode(0b1001_0101_1111_0110, 0));

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
    assert_eq!(Op::Neg { d: 00 }, Op::decode(0b1001_0100_0000_0001, 0));
    assert_eq!(Op::Neg { d: 31 }, Op::decode(0b1001_0101_1111_0001, 0));

    // Nop,
    assert_eq!(Op::Nop, Op::decode(0b0000_0000_0000_0000, 0));

    // Or { d: u8, r: u8 },
    assert_eq!(Op::Or { d: 00, r: 00 }, Op::decode(0b0010_1000_0000_0000, 0));
    assert_eq!(Op::Or { d: 00, r: 31 }, Op::decode(0b0010_1010_0000_1111, 0));
    assert_eq!(Op::Or { d: 31, r: 00 }, Op::decode(0b0010_1001_1111_0000, 0));
    assert_eq!(Op::Or { d: 31, r: 31 }, Op::decode(0b0010_1011_1111_1111, 0));

    // Ori { d: u8, k: u8 },
    assert_eq!(Op::Ori { d: 16, k: 000 }, Op::decode(0b0110_0000_0000_0000, 0));
    assert_eq!(Op::Ori { d: 16, k: 255 }, Op::decode(0b0110_1111_0000_1111, 0));
    assert_eq!(Op::Ori { d: 31, k: 000 }, Op::decode(0b0110_0000_1111_0000, 0));
    assert_eq!(Op::Ori { d: 31, k: 255 }, Op::decode(0b0110_1111_1111_1111, 0));

    // Out { a: u8, r: u8 },
    assert_eq!(Op::Out { a: 00, r: 00 }, Op::decode(0b1011_1000_0000_0000, 0));
    assert_eq!(Op::Out { a: 00, r: 31 }, Op::decode(0b1011_1001_1111_0000, 0));
    assert_eq!(Op::Out { a: 63, r: 00 }, Op::decode(0b1011_1110_0000_1111, 0));
    assert_eq!(Op::Out { a: 63, r: 31 }, Op::decode(0b1011_1111_1111_1111, 0));

    // Pop { d: u8 },
    assert_eq!(Op::Pop { d: 00 }, Op::decode(0b1001_0000_0000_1111, 0));
    assert_eq!(Op::Pop { d: 31 }, Op::decode(0b1001_0001_1111_1111, 0));

    // Push { r: u8 },
    assert_eq!(Op::Push { r: 00 }, Op::decode(0b1001_0010_0000_1111, 0));
    assert_eq!(Op::Push { r: 31 }, Op::decode(0b1001_0011_1111_1111, 0));

    // Rcall { k: i16 },
    assert_eq!(Op::Rcall { k:     0 }, Op::decode(0b1101_0000_0000_0000, 0));
    assert_eq!(Op::Rcall { k:    -1 }, Op::decode(0b1101_1111_1111_1111, 0));
    assert_eq!(Op::Rcall { k:  2047 }, Op::decode(0b1101_0111_1111_1111, 0));
    assert_eq!(Op::Rcall { k: -2048 }, Op::decode(0b1101_1000_0000_0000, 0));

    // Ret,
    assert_eq!(Op::Ret, Op::decode(0b1001_0101_0000_1000, 0));

    // Reti,
    assert_eq!(Op::Reti, Op::decode(0b1001_0101_0001_1000, 0));

    // Rjmp { k: i16 },
    assert_eq!(Op::Rjmp { k:     0 }, Op::decode(0b1100_0000_0000_0000, 0));
    assert_eq!(Op::Rjmp { k:    -1 }, Op::decode(0b1100_1111_1111_1111, 0));
    assert_eq!(Op::Rjmp { k:  2047 }, Op::decode(0b1100_0111_1111_1111, 0));
    assert_eq!(Op::Rjmp { k: -2048 }, Op::decode(0b1100_1000_0000_0000, 0));

    // Ror { d: u8 },
    assert_eq!(Op::Ror { d: 00 }, Op::decode(0b1001_0100_0000_0111, 0));
    assert_eq!(Op::Ror { d: 31 }, Op::decode(0b1001_0101_1111_0111, 0));

    // Sbc { d: u8, r: u8 },
    assert_eq!(Op::Sbc { d: 00, r: 00 }, Op::decode(0b0000_1000_0000_0000, 0));
    assert_eq!(Op::Sbc { d: 00, r: 31 }, Op::decode(0b0000_1010_0000_1111, 0));
    assert_eq!(Op::Sbc { d: 31, r: 00 }, Op::decode(0b0000_1001_1111_0000, 0));
    assert_eq!(Op::Sbc { d: 31, r: 31 }, Op::decode(0b0000_1011_1111_1111, 0));

    // Sbci { d: u8, k: u8 },
    assert_eq!(Op::Sbci { d: 16, k: 000 }, Op::decode(0b0100_0000_0000_0000, 0));
    assert_eq!(Op::Sbci { d: 16, k: 255 }, Op::decode(0b0100_1111_0000_1111, 0));
    assert_eq!(Op::Sbci { d: 31, k: 000 }, Op::decode(0b0100_0000_1111_0000, 0));
    assert_eq!(Op::Sbci { d: 31, k: 255 }, Op::decode(0b0100_1111_1111_1111, 0));

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
    assert_eq!(Op::Sbiw { d: 24, k: 00 }, Op::decode(0b1001_0111_0000_0000, 0));
    assert_eq!(Op::Sbiw { d: 24, k: 63 }, Op::decode(0b1001_0111_1100_1111, 0));
    assert_eq!(Op::Sbiw { d: 30, k: 00 }, Op::decode(0b1001_0111_0011_0000, 0));
    assert_eq!(Op::Sbiw { d: 30, k: 63 }, Op::decode(0b1001_0111_1111_1111, 0));

    // Sbrc { r: u8, b: u8 },
    assert_eq!(Op::Sbrc { r: 00, b: 0 }, Op::decode(0b1111_1100_0000_0000, 0));
    assert_eq!(Op::Sbrc { r: 00, b: 7 }, Op::decode(0b1111_1100_0000_0111, 0));
    assert_eq!(Op::Sbrc { r: 31, b: 0 }, Op::decode(0b1111_1101_1111_0000, 0));
    assert_eq!(Op::Sbrc { r: 31, b: 7 }, Op::decode(0b1111_1101_1111_0111, 0));

    // Sbrs { r: u8, b: u8 },
    assert_eq!(Op::Sbrs { r: 00, b: 0 }, Op::decode(0b1111_1110_0000_0000, 0));
    assert_eq!(Op::Sbrs { r: 00, b: 7 }, Op::decode(0b1111_1110_0000_0111, 0));
    assert_eq!(Op::Sbrs { r: 31, b: 0 }, Op::decode(0b1111_1111_1111_0000, 0));
    assert_eq!(Op::Sbrs { r: 31, b: 7 }, Op::decode(0b1111_1111_1111_0111, 0));

    // Sleep,
    assert_eq!(Op::Sleep, Op::decode(0b1001_0101_1000_1000, 0));

    // Spm,
    assert_eq!(Op::Spm,  Op::decode(0b1001_0101_1110_1000, 0));

    // Spm2,
    assert_eq!(Op::Spm2, Op::decode(0b1001_0101_1111_1000, 0));

    // St { r: u8, idx: LdStIndex, ext: LdStExt },
    assert_eq!(Op::St { r: 00, idx: LdStIndex::X, ext: LdStExt::None    }, Op::decode(0b1001_0010_0000_1100, 0));
    assert_eq!(Op::St { r: 31, idx: LdStIndex::X, ext: LdStExt::None    }, Op::decode(0b1001_0011_1111_1100, 0));
    assert_eq!(Op::St { r: 00, idx: LdStIndex::X, ext: LdStExt::PostInc }, Op::decode(0b1001_0010_0000_1101, 0));
    assert_eq!(Op::St { r: 31, idx: LdStIndex::X, ext: LdStExt::PostInc }, Op::decode(0b1001_0011_1111_1101, 0));
    assert_eq!(Op::St { r: 00, idx: LdStIndex::X, ext: LdStExt::PreDec  }, Op::decode(0b1001_0010_0000_1110, 0));
    assert_eq!(Op::St { r: 31, idx: LdStIndex::X, ext: LdStExt::PreDec  }, Op::decode(0b1001_0011_1111_1110, 0));

    assert_eq!(Op::St { r: 00, idx: LdStIndex::Y, ext: LdStExt::PostInc          }, Op::decode(0b1001_0010_0000_1001, 0));
    assert_eq!(Op::St { r: 31, idx: LdStIndex::Y, ext: LdStExt::PostInc          }, Op::decode(0b1001_0011_1111_1001, 0));
    assert_eq!(Op::St { r: 00, idx: LdStIndex::Y, ext: LdStExt::PreDec           }, Op::decode(0b1001_0010_0000_1010, 0));
    assert_eq!(Op::St { r: 31, idx: LdStIndex::Y, ext: LdStExt::PreDec           }, Op::decode(0b1001_0011_1111_1010, 0));
    assert_eq!(Op::St { r: 00, idx: LdStIndex::Y, ext: LdStExt::Displacement(00) }, Op::decode(0b1000_0010_0000_1000, 0));
    assert_eq!(Op::St { r: 31, idx: LdStIndex::Y, ext: LdStExt::Displacement(00) }, Op::decode(0b1000_0011_1111_1000, 0));
    assert_eq!(Op::St { r: 00, idx: LdStIndex::Y, ext: LdStExt::Displacement(63) }, Op::decode(0b1010_1110_0000_1111, 0));
    assert_eq!(Op::St { r: 31, idx: LdStIndex::Y, ext: LdStExt::Displacement(63) }, Op::decode(0b1010_1111_1111_1111, 0));

    assert_eq!(Op::St { r: 00, idx: LdStIndex::Z, ext: LdStExt::PostInc          }, Op::decode(0b1001_0010_0000_0001, 0));
    assert_eq!(Op::St { r: 31, idx: LdStIndex::Z, ext: LdStExt::PostInc          }, Op::decode(0b1001_0011_1111_0001, 0));
    assert_eq!(Op::St { r: 00, idx: LdStIndex::Z, ext: LdStExt::PreDec           }, Op::decode(0b1001_0010_0000_0010, 0));
    assert_eq!(Op::St { r: 31, idx: LdStIndex::Z, ext: LdStExt::PreDec           }, Op::decode(0b1001_0011_1111_0010, 0));
    assert_eq!(Op::St { r: 00, idx: LdStIndex::Z, ext: LdStExt::Displacement(00) }, Op::decode(0b1000_0010_0000_0000, 0));
    assert_eq!(Op::St { r: 31, idx: LdStIndex::Z, ext: LdStExt::Displacement(00) }, Op::decode(0b1000_0011_1111_0000, 0));
    assert_eq!(Op::St { r: 00, idx: LdStIndex::Z, ext: LdStExt::Displacement(63) }, Op::decode(0b1010_1110_0000_0111, 0));
    assert_eq!(Op::St { r: 31, idx: LdStIndex::Z, ext: LdStExt::Displacement(63) }, Op::decode(0b1010_1111_1111_0111, 0));

    // Sts { k: u16, r: u8 },
    assert_eq!(Op::Sts { r: 00, k: 0x0000 }, Op::decode(0b1001_0010_0000_0000, 0x00_00));
    assert_eq!(Op::Sts { r: 00, k: 0xffff }, Op::decode(0b1001_0010_0000_0000, 0xff_ff));
    assert_eq!(Op::Sts { r: 31, k: 0x0000 }, Op::decode(0b1001_0011_1111_0000, 0x00_00));
    assert_eq!(Op::Sts { r: 31, k: 0xffff }, Op::decode(0b1001_0011_1111_0000, 0xff_ff));

    // Sub { d: u8, r: u8 },
    assert_eq!(Op::Sub { d: 00, r: 00 }, Op::decode(0b0001_1000_0000_0000, 0));
    assert_eq!(Op::Sub { d: 00, r: 31 }, Op::decode(0b0001_1010_0000_1111, 0));
    assert_eq!(Op::Sub { d: 31, r: 00 }, Op::decode(0b0001_1001_1111_0000, 0));
    assert_eq!(Op::Sub { d: 31, r: 31 }, Op::decode(0b0001_1011_1111_1111, 0));

    // Subi { d: u8, k: u8 },
    assert_eq!(Op::Subi { d: 16, k: 000 }, Op::decode(0b0101_0000_0000_0000, 0));
    assert_eq!(Op::Subi { d: 16, k: 255 }, Op::decode(0b0101_1111_0000_1111, 0));
    assert_eq!(Op::Subi { d: 31, k: 000 }, Op::decode(0b0101_0000_1111_0000, 0));
    assert_eq!(Op::Subi { d: 31, k: 255 }, Op::decode(0b0101_1111_1111_1111, 0));

    // Swap { d: u8 },
    assert_eq!(Op::Swap { d: 00 }, Op::decode(0b1001_0100_0000_0010, 0));
    assert_eq!(Op::Swap { d: 31 }, Op::decode(0b1001_0101_1111_0010, 0));

    // Wdr,
    assert_eq!(Op::Wdr, Op::decode(0b1001_0101_1010_1000, 0));

    // Undefined { w: u16 },
}
