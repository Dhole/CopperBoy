use super::super::display::{HEIGTH, WIDTH};
use super::super::utils::*;
use super::*;
use core::default::Default;

use pretty_assertions::assert_eq;
use ron;

static ROM_CASTLEBOY: &'static [u8] = include_bytes!("../../../test-roms/CastleBoy/CastleBoy.hex");
static ROM_CASTLEBOY_REPLAY: &'static str = include_str!("../../../test-roms/CastleBoy/replay.ron");

static ROM_MB: &'static [u8] =
    include_bytes!("../../../test-roms/Mystic-Balloon/Mystic-Balloon.hex");
static ROM_MB_REPLAY: &'static str = include_str!("../../../test-roms/Mystic-Balloon/replay.ron");

#[test]
fn test_serialize() {
    subtest_serialize(ROM_CASTLEBOY, ROM_CASTLEBOY_REPLAY);
    subtest_serialize(ROM_MB, ROM_MB_REPLAY);
}

fn subtest_serialize(rom: &[u8], replay_ron: &str) {
    let replay = ron::from_str(replay_ron).unwrap();
    let mut replay_index = 0;
    let mut emu = Emulator::new();
    emu.load_game(rom).unwrap();

    let mut keys_state = KeysState::default();
    let frames = 10 * 60;
    for f in 0..frames {
        replay_index = replay_keys_state(f, replay_index, &replay, &mut keys_state);
        emu.run(&keys_state);

        // Serialize and deserialize at each second and compare
        if f % 60 == 0 {
            let mut emu2 = Emulator::new();
            emu2.load_game(rom).unwrap();

            let mut buf = vec![0; emu.serialize_len().unwrap()];
            emu.serialize(&mut buf).unwrap();
            emu2.deserialize(&buf).unwrap();

            emu_compare(&mut emu, &mut emu2);
        }
    }
}

fn emu_compare(emu1: &mut Emulator, emu2: &mut Emulator) {
    core::assert_eq!(emu1.core.program, emu2.core.program);
    core::assert_eq!(emu1.core.program_ops, emu2.core.program_ops);
    // Remove program and program_ops temporally from core to facilitate comparison
    let mut program = Vec::new();
    let mut program_ops = Vec::new();
    mem::swap(&mut program, &mut emu1.core.program);
    mem::swap(&mut program_ops, &mut emu1.core.program_ops);
    mem::swap(&mut Vec::new(), &mut emu2.core.program);
    mem::swap(&mut Vec::new(), &mut emu2.core.program_ops);
    // Ignore display frame
    emu1.core.display.frame = vec![0; WIDTH * HEIGTH];
    emu2.core.display.frame = vec![0; WIDTH * HEIGTH];
    assert_eq!(emu1, emu2);
    // Set program and program_ops back to emu1 to continue emulation
    emu1.core.program = program;
    emu1.core.program_ops = program_ops;
    emu2.core.program = emu1.core.program.clone();
    emu2.core.program_ops = emu1.core.program_ops.clone();
}
