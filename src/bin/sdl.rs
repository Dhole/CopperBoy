#[allow(unused_assignments)]
use std::env;
use std::fs;
use std::io::{self, BufRead};
use std::path::Path;
use std::time::Duration;
use std::time::Instant;

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
use sdl2::render::TextureQuery;

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
            Arg::with_name("calltrace")
                .short("c")
                .long("calltrace")
                .help("Trace all calls"),
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
    let calltrace = app.is_present("calltrace");

    let file = fs::File::open(path)?;

    let font_path: &Path = Path::new("./assets/DejaVuSansMono.ttf");

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
    run(scale, trace, calltrace, font_path, &mut core)
}

fn run(
    scale: u32,
    mut trace: bool,
    mut calltrace: bool,
    font_path: &Path,
    core: &mut Core,
) -> Result<(), FrontError> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;

    let window = video_subsystem
        .window("avremu-rs", WIDTH as u32 * scale, HEIGTH as u32 * scale)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    let mut font = ttf_context.load_font(font_path, 128)?;
    font.set_style(sdl2::ttf::FontStyle::NORMAL);

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump()?;

    let texture_creator = canvas.texture_creator();

    let frame_exp_dur = Duration::from_nanos(1_000_000_000u64 / 60);
    let mut now_end_frame = Instant::now();

    let mut pin_b = 0xff as u8;
    let mut pin_c = 0xff as u8;
    let mut pin_d = 0xff as u8;
    let mut pin_e = 0xff as u8;
    let mut pin_f = 0xff as u8;
    let mut cycles: i32 = 0;
    let mut frame: u32 = 0;
    let mut d = 0;
    let mut int_ret_addr: Option<(u16)> = Option::None;
    let mut fps: f32 = 0.0;
    let mut turbo = false;
    let mut paused = false;
    let mut start = Instant::now();
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
                        Keycode::Tab => turbo = true,
                        Keycode::Space => paused = !paused,
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
                        Keycode::Tab => {
                            start = Instant::now();
                            frame = 0;
                            turbo = false
                        }
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
        if !paused {
            cycles += 16_000_000 / 60;
        }
        while cycles > 0 && !paused {
            let addr0 = core.pc << 1;
            let (w0, w1, op_addr) = core.next_op();
            // println!("{:04x} !SP: {:04x}", op_addr.addr, 0x0a00 - core.sp);
            // trace = if 0x573a <= op_addr.addr && op_addr.addr <= 0x576a
            //     || 0x4662 <= op_addr.addr && op_addr.addr <= 0x4668
            // {
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
                    print!("[{:04x}]: {}; {}", op_addr.addr >> 1, op_addr_alt, op_addr,);
                } else {
                    print!("[{:04x}]: {}", op_addr.addr >> 1, op_addr,);
                }
                println!("; SP = {:04x}", core.sp);
            }
            let step_cycles = core.step();
            cycles -= step_cycles as i32;
            let addr1 = core.pc << 1;
            if let Option::None = int_ret_addr {
                if let Op::Rcall { k: 0 } = op_addr.op {
                } else {
                    match op_addr.op {
                        Op::Call { .. } | Op::Icall { .. } | Op::Rcall { .. } => {
                            if calltrace {
                                println!(
                                    "{pad:<2} {0:<pad$}{1:04x} -> {2:04x} call",
                                    "",
                                    addr0,
                                    core.pc << 1,
                                    pad = d,
                                );
                            }
                            d += 1;
                        }
                        Op::Ret { .. } | Op::Reti { .. } => {
                            if d != 0 {
                                d -= 1;
                            } else {
                                println!("D UNDERFLOW");
                            }
                            if calltrace {
                                println!(
                                    "{pad:<2} {0:<pad$}{2:04x} <- {1:04x} ret",
                                    "",
                                    addr0,
                                    core.pc << 1,
                                    pad = d,
                                );
                            }
                        }
                        _ => {}
                    }
                }
            }
            if let Some(addr) = int_ret_addr {
                if core.pc << 1 == addr {
                    // println!("EXIT INT");
                    int_ret_addr = Option::None;
                }
            }
            let int = core.step_hw(step_cycles);
            if int {
                // println!("ENTER INT");
                int_ret_addr = Some(addr1);
                // println!(
                //     "{0:<pad$}{1:04x} -> {2:04x} int {3:04x}",
                //     "",
                //     addr1,
                //     core.pc << 1,
                //     core.sp,
                //     pad = d,
                // );
                // d += 1;
            }
        }
        if d > 100 {
            println!("D OVERFLOW");
            d = 0;
        }

        core.display.render();

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.set_draw_color(Color::RGB(255, 255, 255));

        for y in 0..HEIGTH {
            for x in 0..WIDTH {
                if core.display.frame[y * WIDTH + x] == 1 {
                    canvas
                        .fill_rect(Rect::new(
                            x as i32 * scale as i32,
                            y as i32 * scale as i32,
                            scale,
                            scale,
                        ))
                        .unwrap();
                }
            }
        }

        // Render frame rate
        let surface = font
            .render(format!("{:02.0}", fps).as_str())
            .blended(Color::RGBA(100, 200, 100, 200))
            .map_err(|e| e.to_string())?;
        let texture = texture_creator
            .create_texture_from_surface(&surface)
            .map_err(|e| e.to_string())?;
        let TextureQuery { width, height, .. } = texture.query();
        const padding: i32 = 1;
        canvas.copy(
            &texture,
            None,
            Some(Rect::new(
                padding * scale as i32,
                padding * scale as i32,
                width / 16 * scale,
                height / 16 * scale,
            )),
        )?;

        canvas.present();
        frame += 1;

        let now = Instant::now();
        let elapsed = now - start;
        let expected = frame_exp_dur * frame;
        if elapsed < expected && !turbo {
            ::std::thread::sleep(expected - elapsed);
        }
        let now = Instant::now();
        let frame_dur = now - now_end_frame;
        // println!(
        //     "elapsed: {:?}, expected: {:?}. frame_dur: {:?}",
        //     Instant::now() - start,
        //     expected,
        //     frame_dur,
        // );
        const update: f32 = 0.2;
        fps = (1.0 - update) * fps + update * (1_000_000_000.0 / frame_dur.subsec_nanos() as f32);
        now_end_frame = now;
    }

    Ok(())
}