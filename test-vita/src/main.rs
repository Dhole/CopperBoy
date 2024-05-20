use std::io::{Read, Write};
use std::net::TcpStream;
use std::sync::Mutex;

use vitasdk_sys::scePowerSetArmClockFrequency;

use arduboy::emulator::Emulator;
use arduboy::mcu::PROGRAM_SIZE;
use arduboy::opcodes::Op;
use arduboy::utils::{replay_keys_state, KeyEvent, KeysState};
use std::time::{Duration, Instant};

static ROM_CASTLEBOY: &'static [u8] = include_bytes!("../../test-roms/CastleBoy/CastleBoy.hex");
static ROM_CASTLEBOY_REPLAY: &'static str = include_str!("../../test-roms/CastleBoy/replay.ron");
static ROM_MB: &'static [u8] = include_bytes!("../../test-roms/Mystic-Balloon/Mystic-Balloon.hex");
static ROM_MB_REPLAY: &'static str = include_str!("../../test-roms/Mystic-Balloon/replay.ron");

static LOG_STREAM: Mutex<Option<TcpStream>> = Mutex::new(None);

macro_rules! log {
    () => {
        write!(LOG_STREAM.lock().unwrap().as_ref().unwrap(), "\n").unwrap();
    };
    ($($arg:tt)*) => {{
        write!(LOG_STREAM.lock().unwrap().as_ref().unwrap(), $($arg)*).unwrap();
        write!(LOG_STREAM.lock().unwrap().as_ref().unwrap(), "\n").unwrap();
    }};
}

fn main() {
    // Connect to log server
    let ip = "192.168.0.130";
    let mut stream = TcpStream::connect((ip, 8888)).unwrap();
    *LOG_STREAM.lock().unwrap() = Some(stream);

    unsafe { scePowerSetArmClockFrequency(444) };
    log!("Let's go!");
    frame_benchmark();
    loop {}
}

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

pub fn frame_benchmark() {
    let emu_cb = setup(ROM_CASTLEBOY, ROM_CASTLEBOY_REPLAY, 780);
    let keys_state = KeysState::default();

    let name = "emulator.run.castle_boy.780";
    {
        let mut emu = emu_cb.clone();
        let start = Instant::now();
        emu.run(&keys_state);
        let duration = start.elapsed();
        log!("{} 1 frame {:?}", name, duration);
    }
    {
        let mut emu = emu_cb.clone();
        let start = Instant::now();
        for _ in 0..60 {
            emu.run(&keys_state)
        }
        let duration = start.elapsed();
        log!("{} 60 frame {:?}", name, duration);
    }

    let emu_mb = setup(ROM_MB, ROM_MB_REPLAY, 1020);
    let keys_state = KeysState::default();
    let name = "emulator.run.mystic_balloon.1020";
    {
        let mut emu = emu_mb.clone();
        let start = Instant::now();
        emu.run(&keys_state);
        let duration = start.elapsed();
        log!("{} 1 frame {:?}", name, duration);
    }
    {
        let mut emu = emu_mb.clone();
        let start = Instant::now();
        for _ in 0..60 {
            emu.run(&keys_state)
        }
        let duration = start.elapsed();
        log!("{} 60 frame {:?}", name, duration);
    }
}
