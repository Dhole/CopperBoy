#[allow(unused_assignments)]
use std::env;
use std::fs;
use std::io::{self, BufRead};
use std::time::Duration;

use hex;

use avremu::core::{Core, GPIOPort};
use avremu::display::{HEIGTH, WIDTH};
use avremu::keys::*;
use avremu::opcodes::{Op, OpAddr};

// use sdl2::audio::{AudioCallback, AudioSpecDesired};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

// use rand::{self, RngCore};

use clap::{App, Arg, SubCommand};

#[derive(Debug)]
pub enum FrontError {
    SDL2(String),
    Io(io::Error),
}

impl From<io::Error> for FrontError {
    fn from(err: io::Error) -> Self {
        Self::Io(err)
    }
}

impl From<String> for FrontError {
    fn from(err: String) -> Self {
        Self::SDL2(err)
    }
}

fn decode_hex_line(line: &str) -> Result<Option<(u16, Vec<u8>)>, hex::FromHexError> {
    let line = line.as_bytes();
    assert_eq!(line[0], b':');
    let line = &line[1..];
    let bytes = hex::decode(&line[0..2])?[0] as usize;
    let addr = hex::decode(&line[2..6])?;
    let addr = u16::from_be_bytes([addr[0], addr[1]]);
    let rtype = hex::decode(&line[6..8])?[0];

    Ok(match rtype {
        0x00 => {
            let data = hex::decode(&line[8..8 + bytes * 2])?;
            let _checksum = hex::decode(&line[8 + bytes * 2..8 + bytes * 2 + 2])?[0];
            Some((addr, data))
        }
        _ => None,
    })
}

pub fn main() -> Result<(), FrontError> {
    env_logger::init();
    let app = App::new("Avremu-rs")
        .version("0.0.1")
        .author("Dhole")
        .arg(
            Arg::with_name("scale")
                .short("s")
                .long("scale")
                .value_name("N")
                .help("Sets the scaling factor")
                .takes_value(true)
                .default_value("8")
                .validator(|scale| match scale.parse::<u32>() {
                    Ok(_) => Ok(()),
                    Err(e) => Err(format!("{}", e)),
                }),
        )
        .arg(
            Arg::with_name("trace")
                .short("t")
                .long("trace")
                .help("Trace all instructions"),
        )
        .arg(
            Arg::with_name("path")
                .help("Path to the rom file")
                .index(1)
                .required(true),
        )
        .get_matches();

    let scale = app
        .value_of("scale")
        .map(|s| s.parse::<u32>().unwrap())
        .unwrap();
    let path = app.value_of("path").unwrap();
    let trace = app.is_present("trace");

    let file = fs::File::open(path)?;

    let mut core = Core::new();
    for line in io::BufReader::new(file).lines() {
        let line = line?;
        if line.len() == 0 {
            continue;
        }
        match decode_hex_line(line.as_str()).map_err(|e| io::Error::new(io::ErrorKind::Other, e))? {
            Some((addr, data)) => {
                for i in 0..data.len() {
                    core.flash(addr + i as u16, data[i]);
                }
            }
            None => {}
        }
    }

    core.reset();
    run(scale, trace, &mut core)
}

fn run(scale: u32, mut trace: bool, core: &mut Core) -> Result<(), FrontError> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("avremu-rs", WIDTH as u32 * scale, HEIGTH as u32 * scale)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump()?;

    let mut pin_b = 0xff as u8;
    let mut pin_c = 0xff as u8;
    let mut pin_d = 0xff as u8;
    let mut pin_e = 0xff as u8;
    let mut pin_f = 0xff as u8;
    let mut cycles: i32 = 0;
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } => {
                    match keycode {
                        Keycode::Left => pin_f &= !PIN_LEFT,
                        Keycode::Right => pin_f &= !PIN_RIGHT,
                        Keycode::Up => pin_f &= !PIN_UP,
                        Keycode::Down => pin_f &= !PIN_DOWN,
                        Keycode::X => pin_e &= !PIN_A,
                        Keycode::Z => pin_b &= !PIN_B,
                        Keycode::T => trace = !trace,
                        _ => {}
                    };
                }
                Event::KeyUp {
                    keycode: Some(keycode),
                    ..
                } => {
                    match keycode {
                        Keycode::Left => pin_f |= PIN_LEFT,
                        Keycode::Right => pin_f |= PIN_RIGHT,
                        Keycode::Up => pin_f |= PIN_UP,
                        Keycode::Down => pin_f |= PIN_DOWN,
                        Keycode::X => pin_e |= PIN_A,
                        Keycode::Z => pin_b |= PIN_B,
                        _ => {}
                    };
                }
                _ => {}
            }
        }

        core.gpio.set_port(GPIOPort::B, pin_b);
        core.gpio.set_port(GPIOPort::C, pin_c);
        core.gpio.set_port(GPIOPort::D, pin_d);
        core.gpio.set_port(GPIOPort::E, pin_e);
        core.gpio.set_port(GPIOPort::F, pin_f);

        cycles += 16_000_000 / 60;
        while cycles > 0 {
            let (w0, w1, op_addr) = core.next_op();
            println!("{:04x} !SP: {:04x}", op_addr.addr, 0x0a00 - core.sp);
            // trace = if 0x4d00 <= op_addr.addr && op_addr.addr <= 0x4e5a {
            //     true
            // } else {
            //     false
            // };
            if trace && !core.sleep {
                print!("{:04x} ", op_addr.addr);
                match op_addr.op.words() {
                    1 => print!("({}     ) ", hex::encode(w0.to_le_bytes())),
                    2 => print!(
                        "({} {}) ",
                        hex::encode(w0.to_le_bytes()),
                        hex::encode(w1.to_le_bytes())
                    ),
                    _ => unreachable!(),
                }
                if let Some(op_addr_alt) = op_addr.alt() {
                    println!("[{:04x}]: {}; {}", op_addr.addr >> 1, op_addr_alt, op_addr,);
                } else {
                    println!("[{:04x}]: {}", op_addr.addr >> 1, op_addr,);
                }
            }
            cycles -= core.step() as i32;
            core.step_hw(cycles as u8);
        }

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.set_draw_color(Color::RGB(255, 255, 255));

        for row in 0..HEIGTH / 8 {
            for x in 0..WIDTH {
                let pixels = core.display.fb[row * WIDTH + x];
                for dy in 0..8 {
                    if pixels & (1 << dy) != 0 {
                        canvas
                            .fill_rect(Rect::new(
                                x as i32 * scale as i32,
                                (row * 8 + dy) as i32 * scale as i32,
                                scale,
                                scale,
                            ))
                            .unwrap();
                    }
                }
            }
        }

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}
