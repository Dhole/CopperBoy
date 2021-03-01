use std::fs;
use std::fs::File;
use std::io::{self, BufWriter, Read};
#[allow(unused_assignments)]
// use std::env;
// use std::fs;
// use std::io::{self, BufRead};
use std::path::{Path, PathBuf};
use std::time::Duration;
use std::time::Instant;

use hex;

use arduboy::display::{HEIGTH, WIDTH};
use arduboy::emulator::{self, Emulator};
use arduboy::keys::*;
use arduboy::mcu::GPIOPort;
use arduboy::opcodes::{Op, OpAddr};
use arduboy::utils::{load_hex_file, replay_keys_state, HexFileError, Key, KeyEvent, KeysState};

use clap::{App, Arg};

#[derive(Debug)]
pub enum FrontError {
    Emulator(emulator::Error),
    Io(io::Error),
    Serde(ron::Error),
}

impl From<emulator::Error> for FrontError {
    fn from(err: emulator::Error) -> Self {
        Self::Emulator(err)
    }
}

impl From<io::Error> for FrontError {
    fn from(err: io::Error) -> Self {
        Self::Io(err)
    }
}

impl From<ron::Error> for FrontError {
    fn from(err: ron::Error) -> Self {
        Self::Serde(err)
    }
}

const SAMPLE_SIZE: u16 = 735;

pub fn main() -> Result<(), FrontError> {
    env_logger::init();
    let app = App::new("Copperboy")
        .version("0.0.1")
        .author("Dhole")
        .arg(
            Arg::with_name("path")
                .help("Path to the rom file")
                .index(1)
                .required(true),
        )
        .arg(
            Arg::with_name("seconds")
                .long("seconds")
                .help("Seconds to emulate")
                .takes_value(true)
                .required(true)
                .validator(|v| match v.parse::<usize>() {
                    Ok(_) => Ok(()),
                    Err(e) => Err(format!("{}", e)),
                }),
        )
        .arg(
            Arg::with_name("frames_screenshot")
                .long("frames_screenshot")
                .help("Frames interval after every screenshot")
                .takes_value(true)
                .required(true)
                .validator(|v| match v.parse::<usize>() {
                    Ok(_) => Ok(()),
                    Err(e) => Err(format!("{}", e)),
                }),
        )
        .arg(
            Arg::with_name("name")
                .long("name")
                .help("Name of the folder to store the report")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("replay")
                .long("replay")
                .help("Key events to use as replay in RON")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("replay_file")
                .long("replay_file")
                .help("File with key events to use as replay in RON")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("out_path")
                .help("Output path to store results")
                .index(2),
        )
        .get_matches();

    let seconds = app
        .value_of("seconds")
        .map(|s| s.parse::<usize>().unwrap())
        .unwrap();
    let screenshot_interval = app
        .value_of("frames_screenshot")
        .map(|s| s.parse::<usize>().unwrap())
        .unwrap();
    let path = app.value_of("path").unwrap();
    let out_path = app.value_of("out_path").map(|s| Path::new(s));
    let replay: Vec<KeyEvent> = match app.value_of("replay") {
        Some(replay_ron) => ron::from_str(replay_ron)?,
        None => vec![],
    };
    let replay: Vec<KeyEvent> = match app.value_of("replay_file") {
        Some(replay_file) => {
            let replay_ron = fs::read_to_string(replay_file)?;
            ron::from_str(&replay_ron)?
        }
        None => replay,
    };

    let mut emu = Emulator::new();

    let mut f = File::open(path)?;
    let mut data = Vec::new();
    f.read_to_end(&mut data)?;
    emu.load_game(&data[..])?;

    let name = app.value_of("name").unwrap_or(
        Path::new(path)
            .iter()
            .rev()
            .nth(1)
            .unwrap()
            .to_str()
            .unwrap(),
    );

    let out_path = out_path.map(|p| p.join(name));
    if let Some(ref out_path) = out_path {
        fs::remove_dir_all(&out_path).unwrap_or(());
        fs::create_dir_all(&out_path)?;
    }

    let mut total_frames = 0;
    let mut replay_index = 0;
    let mut keys_state = KeysState::default();
    let mut framebuffer = vec![0u8; WIDTH * HEIGTH];
    for s in 0..seconds {
        for frame in 0..60 {
            total_frames += 1;

            replay_index = replay_keys_state(total_frames, replay_index, &replay, &mut keys_state);

            emu.run(&keys_state);

            if let Some(ref out_path) = out_path {
                if total_frames % screenshot_interval == 0 {
                    let path = out_path.join(format!("frame_{:08}.png", total_frames));
                    let file = File::create(path).unwrap();
                    let ref mut w = BufWriter::new(file);

                    let mut encoder = png::Encoder::new(w, WIDTH as u32, HEIGTH as u32);
                    encoder.set_color(png::ColorType::Grayscale);
                    encoder.set_depth(png::BitDepth::Eight);
                    let mut writer = encoder.write_header().unwrap();

                    for i in 0..framebuffer.len() {
                        framebuffer[i] = emu.core.display.frame[i] as u8;
                    }
                    writer.write_image_data(&framebuffer[..]).unwrap(); // Save
                }
            }

            // let now = Instant::now();
            // let elapsed = now - start;
            // let now = Instant::now();
            // let frame_dur = now - now_end_frame;
            // const update: f32 = 0.2;
            // fps =
            //     (1.0 - update) * fps + update * (1_000_000_000.0 / frame_dur.subsec_nanos() as f32);
            // now_end_frame = now;
        }
    }
    if let Some(out_path) = out_path {
        let path = out_path.join("stats.txt");
        let file = File::create(path).unwrap();
        let ref mut w = BufWriter::new(file);
        emu.core.stats.write_summary(w, 32, 32, 64);
    }
    // println!("ops: {:?}", core.stats.ops);

    Ok(())
}
