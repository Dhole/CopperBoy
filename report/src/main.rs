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
use arduboy::utils::{load_hex_file, HexFileError, Key, KeyEvent};

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
            Arg::with_name("input")
                .long("input")
                .help("Key events to use as input in JSON")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("out_path")
                .help("Output path to store results")
                .index(2),
        )
        .get_matches();

    let scale = app
        .value_of("scale")
        .map(|s| s.parse::<u32>().unwrap())
        .unwrap();
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
    let input: Vec<KeyEvent> = match app.value_of("input") {
        Some(input) => ron::from_str(input)?,
        None => vec![],
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
        fs::remove_dir_all(&out_path)?;
        fs::create_dir_all(&out_path)?;
    }

    // const AUDIO_FREQ: i32 = 44100;

    // let mut sample = [127; SAMPLE_SIZE as usize];
    // let frame_exp_dur = Duration::from_nanos(1_000_000_000u64 / 60);
    // let mut now_end_frame = Instant::now();

    // const SAMPLE_CYCLES: i32 = 16_000_000 / AUDIO_FREQ;
    // const SAMPLES_FRAME: i32 = AUDIO_FREQ / 60;

    let mut port_b = 0xff as u8;
    // let mut pin_c = 0xff as u8;
    // let mut pin_d = 0xff as u8;
    let mut port_e = 0xff as u8;
    let mut port_f = 0xff as u8;
    // let mut cycles: i32 = 0;
    // let mut frame: u32 = 0;
    // let mut d = 0;
    // let mut int_ret_addr: Option<u16> = Option::None;
    // let mut fps: f32 = 0.0;
    // let mut step_cycles_sample: i32 = 0;
    // let mut start = Instant::now();

    let mut total_frames = 0;
    let mut input_index = 0;
    for s in 0..seconds {
        for frame in 0..60 {
            total_frames += 1;
            if input_index < input.len() && input[input_index].frame == total_frames {
                let key_event = &input[input_index];
                for down in &key_event.down {
                    match down {
                        Key::Left => port_f &= !PIN_LEFT,
                        Key::Right => port_f &= !PIN_RIGHT,
                        Key::Up => port_f &= !PIN_UP,
                        Key::Down => port_f &= !PIN_DOWN,
                        Key::A => port_e &= !PIN_A,
                        Key::B => port_b &= !PIN_B,
                    }
                }
                for up in &key_event.up {
                    match up {
                        Key::Left => port_f |= PIN_LEFT,
                        Key::Right => port_f |= PIN_RIGHT,
                        Key::Up => port_f |= PIN_UP,
                        Key::Down => port_f |= PIN_DOWN,
                        Key::A => port_e |= PIN_A,
                        Key::B => port_b |= PIN_B,
                    }
                }
                input_index += 1;
            }

            emu.run(port_b, port_e, port_f);

            if let Some(ref out_path) = out_path {
                if total_frames % screenshot_interval == 0 {
                    let path = out_path.join(format!("frame_{:08}.png", total_frames));
                    let file = File::create(path).unwrap();
                    let ref mut w = BufWriter::new(file);

                    let mut encoder = png::Encoder::new(w, WIDTH as u32, HEIGTH as u32);
                    encoder.set_color(png::ColorType::Grayscale);
                    encoder.set_depth(png::BitDepth::Eight);
                    let mut writer = encoder.write_header().unwrap();

                    writer.write_image_data(&emu.core.display.frame).unwrap(); // Save
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
