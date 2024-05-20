use criterion::{criterion_group, criterion_main, BatchSize, Criterion, SamplingMode};
use std::time::Duration;

use arduboy::emulator::Emulator;
use arduboy::mcu::PROGRAM_SIZE;
use arduboy::opcodes::Op;
use arduboy::utils::{replay_keys_state, KeyEvent, KeysState};

static ROM_CASTLEBOY: &'static [u8] = include_bytes!("../../test-roms/CastleBoy/CastleBoy.hex");
static ROM_CASTLEBOY_REPLAY: &'static str = include_str!("../../test-roms/CastleBoy/replay.ron");
static ROM_MB: &'static [u8] = include_bytes!("../../test-roms/Mystic-Balloon/Mystic-Balloon.hex");
static ROM_MB_REPLAY: &'static str = include_str!("../../test-roms/Mystic-Balloon/replay.ron");

pub fn setup(rom: &[u8], replay_ron: &str, frames: usize) -> Emulator {
    let replay: Vec<KeyEvent> = ron::from_str(replay_ron).unwrap();
    let mut emu = Emulator::new();
    emu.load_game(rom).unwrap();

    let mut keys_state = KeysState::default();
    let mut replay_index = 0;
    for f in 0..frames {
        replay_index = replay_keys_state(f, replay_index, &replay, &mut keys_state);
        emu.run(&keys_state);
    }
    emu
}

// pub fn run_frames(emu: &mut Emulator, replay: &Vec<KeyEvent>, frames: usize) {
//     let mut keys_state = KeysState::default();
//     let mut replay_index = 0;
//     for f in 0..frames {
//         replay_index = replay_keys_state(f, replay_index, &replay, &mut keys_state);
//         emu.run(&keys_state);
//     }
// }

pub fn frame_benchmark(c: &mut Criterion) {
    let emu_cb = setup(ROM_CASTLEBOY, ROM_CASTLEBOY_REPLAY, 780);
    let keys_state = KeysState::default();

    let mut group = c.benchmark_group("emulator.run.castle_boy.780");
    group.sampling_mode(SamplingMode::Flat);
    group.measurement_time(Duration::from_secs(10));
    group.bench_function("1", |b| {
        b.iter_batched(
            || emu_cb.clone(),
            |mut emu| emu.run(&keys_state),
            BatchSize::LargeInput,
        )
    });
    group.bench_function("60", |b| {
        b.iter_batched(
            || emu_cb.clone(),
            |mut emu| {
                for _ in 0..60 {
                    emu.run(&keys_state)
                }
            },
            BatchSize::LargeInput,
        )
    });
    group.finish();

    let emu_mb = setup(ROM_MB, ROM_MB_REPLAY, 1020);
    let mut group = c.benchmark_group("emulator.run.mystic_balloon.1020");
    group.sampling_mode(SamplingMode::Flat);
    group.measurement_time(Duration::from_secs(10));
    group.bench_function("1", |b| {
        b.iter_batched(
            || emu_mb.clone(),
            |mut emu| emu.run(&keys_state),
            BatchSize::LargeInput,
        )
    });
    group.bench_function("60", |b| {
        b.iter_batched(
            || emu_mb.clone(),
            |mut emu| {
                for _ in 0..60 {
                    emu.run(&keys_state)
                }
            },
            BatchSize::LargeInput,
        )
    });
    group.finish();
}

pub fn op_benchmark(c: &mut Criterion) {
    let keys_state = KeysState::default();

    let mut emu = Emulator::new();

    let mut group = c.benchmark_group("emulator.run.op");
    group.sampling_mode(SamplingMode::Flat);
    group.measurement_time(Duration::from_secs(5));
    for (op_name, op_value) in &[
        ("add", Op::Add { d: 0, r: 1 }),
        ("adc", Op::Adc { d: 0, r: 1 }),
        ("cp", Op::Cp { d: 0, r: 1 }),
        ("cpc", Op::Cpc { d: 0, r: 1 }),
        ("movw", Op::Movw { d: 0, r: 1 }),
    ] {
        group.bench_function(*op_name, |b| {
            b.iter_batched(
                || {
                    let mut emu = emu.clone();
                    for op in emu.core.program_ops.iter_mut() {
                        *op = *op_value;
                    }
                    emu.core.program_ops[PROGRAM_SIZE as usize - 1] = Op::Jmp { k: 0x0 };
                    emu.core.regs[1] = 0x01;
                    emu
                },
                |mut emu| emu.run(&keys_state),
                BatchSize::LargeInput,
            )
        });
    }
    group.finish();
}

criterion_group!(benches, frame_benchmark, op_benchmark);
criterion_main!(benches);
