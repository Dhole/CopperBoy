use super::*;

static ROM_CASTLEBOY: &'static [u8] = include_bytes!("../../../test-roms/CastleBoy/CastleBoy.hex");

#[test]
fn test_serialize() {
    let mut emu = Emulator::new();
    emu.load_game(ROM_CASTLEBOY).unwrap();

    for _ in 0..60 {
        emu.run(0, 0, 0);
    }

    let mut emu2 = Emulator::new();
    emu2.load_game(ROM_CASTLEBOY).unwrap();

    let mut buf = vec![0; emu.serialize_len().unwrap()];
    emu.serialize(&mut buf).unwrap();

    emu2.deserialize(&buf).unwrap();

    assert_eq!(emu, emu2);
}
