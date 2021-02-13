use std::fs::File;
use std::io;
use std::io::{Read, Write};
#[allow(unused_assignments)]
// use std::env;
// use std::fs;
// use std::io::{self, BufRead};
use std::path::Path;
use std::time::Duration;
use std::time::Instant;

use hex;

use arduboy::display::{HEIGTH, WIDTH};
use arduboy::keys::*;
use arduboy::mcu::{Core, GPIOPort};
use arduboy::opcodes::{Op, OpAddr};
use arduboy::utils::{load_hex_file, HexFileError, Key, KeyEvent};

// use sdl2::audio::{AudioCallback, AudioSpecDesired};
use sdl2::audio::{AudioCallback, AudioSpecDesired};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;
use sdl2::render::TextureQuery;

use ron;
// use serde_json;
// use rand::{self, RngCore};

use clap::{App, Arg};

#[derive(Debug)]
pub enum FrontError {
    SDL2(String),
    HexFile(HexFileError),
    Serde(ron::Error),
    Postcard(postcard::Error),
    Io(io::Error),
}

impl From<String> for FrontError {
    fn from(err: String) -> Self {
        Self::SDL2(err)
    }
}

impl From<HexFileError> for FrontError {
    fn from(err: HexFileError) -> Self {
        Self::HexFile(err)
    }
}

impl From<ron::Error> for FrontError {
    fn from(err: ron::Error) -> Self {
        Self::Serde(err)
    }
}

impl From<postcard::Error> for FrontError {
    fn from(err: postcard::Error) -> Self {
        Self::Postcard(err)
    }
}

impl From<io::Error> for FrontError {
    fn from(err: io::Error) -> Self {
        Self::Io(err)
    }
}

const SAMPLE_SIZE: u16 = 735;
struct AudioSample {
    bytes: [u8; SAMPLE_SIZE as usize],
    position: usize,
}

impl AudioCallback for AudioSample {
    type Channel = u8;

    fn callback(&mut self, out: &mut [u8]) {
        let start = self.position;
        let end = std::cmp::min(self.bytes.len(), self.position + out.len());
        self.position = end;

        let audio_data = &self.bytes[start..end];
        for (src, dst) in audio_data.iter().zip(out.iter_mut()) {
            *dst = *src;
        }
    }
}

pub fn main() -> Result<(), FrontError> {
    env_logger::init();
    let app = App::new("Copperboy")
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
            Arg::with_name("record")
                .short("r")
                .long("record")
                .help("Record all key events"),
        )
        .arg(
            Arg::with_name("input")
                .long("input")
                // .help("Key events to use as input in JSON")
                .help("Key events to use as input in RON")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("path")
                .help("Path to the rom file")
                .index(1)
                .required(true),
        )
        .arg(
            Arg::with_name("sav_path")
                .long("sav_path")
                .help("Save path")
                .takes_value(true),
        )
        .get_matches();

    let scale = app
        .value_of("scale")
        .map(|s| s.parse::<u32>().unwrap())
        .unwrap();
    let path = app.value_of("path").unwrap();
    let mut trace = app.is_present("trace");
    let calltrace = app.is_present("calltrace");
    let record = app.is_present("record");
    let input: Vec<KeyEvent> = match app.value_of("input") {
        // Some(input) => serde_json::from_str(input)?,
        Some(input) => ron::from_str(input)?,
        None => vec![],
    };

    let font_path: &Path = Path::new("./assets/DejaVuSansMono.ttf");

    let mut core = Core::new();
    load_hex_file(&mut core, path)?;

    core.reset();
    let serialize_len = core.serialize_len()?;
    println!("serialize_len = {}", serialize_len);

    let save_path = app
        .value_of("sav_path")
        .map(|p| Path::new(p).to_path_buf())
        .unwrap_or(Path::new(path).with_extension("sav"));

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let audio_subsystem = sdl_context.audio().unwrap();
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;

    // const AUDIO_FREQ: i32 = 44100;
    const AUDIO_FREQ: i32 = 44100;
    let desired_spec = AudioSpecDesired {
        freq: Some(AUDIO_FREQ),
        channels: Some(1),          // mono
        samples: Some(SAMPLE_SIZE), // default sample size
    };

    let mut sample = [127; SAMPLE_SIZE as usize];
    let mut audio = audio_subsystem
        .open_playback(None, &desired_spec, |spec| {
            // initialize the audio callback
            AudioSample {
                bytes: sample.clone(),
                position: 0,
            }
        })
        .unwrap();
    audio.resume();

    let window = video_subsystem
        .window("Copperboy", WIDTH as u32 * scale, HEIGTH as u32 * scale)
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
    let mut tex_display = texture_creator
        .create_texture_streaming(PixelFormatEnum::RGB24, WIDTH as u32, HEIGTH as u32)
        .map_err(|e| e.to_string())?;

    let mut surf_fps = font
        .render(format!("{:02.0}", 0).as_str())
        .blended(Color::RGBA(100, 200, 100, 200))
        .map_err(|e| e.to_string())?;
    let mut tex_fps = texture_creator
        .create_texture_from_surface(&surf_fps)
        .map_err(|e| e.to_string())?;

    let frame_exp_dur = Duration::from_nanos(1_000_000_000u64 / 60);
    let mut now_end_frame = Instant::now();

    const SAMPLE_CYCLES: i32 = 16_000_000 / AUDIO_FREQ;
    const SAMPLES_FRAME: i32 = AUDIO_FREQ / 60;

    let mut save = false;
    let mut load = false;

    let mut port_b = 0xff as u8;
    // let mut pin_c = 0xff as u8;
    // let mut pin_d = 0xff as u8;
    let mut port_e = 0xff as u8;
    let mut port_f = 0xff as u8;
    let mut cycles: i32 = 0;
    let mut frame: u32 = 0;
    let mut d = 0;
    let mut int_ret_addr: Option<u16> = Option::None;
    let mut fps: f32 = 0.0;
    let mut turbo = false;
    let mut paused = false;
    let mut step_cycles_sample: i32 = 0;
    let mut start = Instant::now();
    let mut key_events: Vec<KeyEvent> = Vec::new();
    let mut total_frames = 0;
    let mut input_index = 0;
    'running: loop {
        total_frames += 1;
        let mut key_event = KeyEvent {
            frame: total_frames,
            up: Vec::new(),
            down: Vec::new(),
        };
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
                        Keycode::Left => {
                            port_f &= !PIN_LEFT;
                            if record {
                                key_event.down.push(Key::Left);
                            }
                        }
                        Keycode::Right => {
                            port_f &= !PIN_RIGHT;
                            if record {
                                key_event.down.push(Key::Right);
                            }
                        }
                        Keycode::Up => {
                            port_f &= !PIN_UP;
                            if record {
                                key_event.down.push(Key::Up);
                            }
                        }
                        Keycode::Down => {
                            port_f &= !PIN_DOWN;
                            if record {
                                key_event.down.push(Key::Down);
                            }
                        }
                        Keycode::X => {
                            port_e &= !PIN_A;
                            if record {
                                key_event.down.push(Key::A);
                            }
                        }
                        Keycode::Z => {
                            port_b &= !PIN_B;
                            if record {
                                key_event.down.push(Key::B);
                            }
                        }
                        Keycode::T => trace = !trace,
                        Keycode::Tab => turbo = true,
                        Keycode::Space => paused = !paused,
                        Keycode::F1 => {
                            save = true;
                        }
                        Keycode::F2 => {
                            load = true;
                        }
                        _ => {}
                    };
                }
                Event::KeyUp {
                    keycode: Some(keycode),
                    ..
                } => {
                    match keycode {
                        Keycode::Left => {
                            port_f |= PIN_LEFT;
                            if record {
                                key_event.up.push(Key::Left);
                            }
                        }
                        Keycode::Right => {
                            port_f |= PIN_RIGHT;
                            if record {
                                key_event.up.push(Key::Right);
                            }
                        }
                        Keycode::Up => {
                            port_f |= PIN_UP;
                            if record {
                                key_event.up.push(Key::Up);
                            }
                        }
                        Keycode::Down => {
                            port_f |= PIN_DOWN;
                            if record {
                                key_event.up.push(Key::Down);
                            }
                        }
                        Keycode::X => {
                            port_e |= PIN_A;
                            if record {
                                key_event.up.push(Key::A);
                            }
                        }
                        Keycode::Z => {
                            port_b |= PIN_B;
                            if record {
                                key_event.up.push(Key::B);
                            }
                        }
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
        if record {
            if key_event.up.len() != 0 || key_event.down.len() != 0 {
                key_events.push(key_event);
            }
        }

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

        core.gpio.set_port(GPIOPort::B, port_b);
        // core.gpio.set_port(GPIOPort::C, pin_c);
        // core.gpio.set_port(GPIOPort::D, pin_d);
        core.gpio.set_port(GPIOPort::E, port_e);
        core.gpio.set_port(GPIOPort::F, port_f);
        if !paused {
            cycles += 16_000_000 / 60;
            step_cycles_sample = SAMPLE_CYCLES;
        }
        while cycles > 0 && !paused {
            let addr0 = core.pc << 1;
            // println!("{:04x} !SP: {:04x}", op_addr.addr, 0x0a00 - core.sp);
            // trace = if 0x573a <= op_addr.addr && op_addr.addr <= 0x576a
            //     || 0x4662 <= op_addr.addr && op_addr.addr <= 0x4668
            // {
            //     true
            // } else {
            //     false
            // };
            let mut w0 = 0;
            let mut w1 = 0;
            let mut op_addr = OpAddr {
                op: Op::Nop,
                addr: 0,
            };
            if trace || calltrace {
                let (_w0, _w1, _op_addr) = core.next_op();
                w0 = _w0;
                w1 = _w1;
                op_addr = _op_addr;
            }
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

            // if core.pc == 0x197c / 2 {
            // trace = false;
            // println!("===");
            //  }
            let mut step_cycles = 0;
            const N_STEPS: usize = 1;
            const M_ITERS: usize = 1;
            if !core.sleep {
                for _ in 0..M_ITERS {
                    step_cycles += core.step();
                    // step_cycles += core.step();
                    // step_cycles += core.step();
                    // step_cycles += core.step();
                }
            } else {
                step_cycles += M_ITERS * N_STEPS;
            }
            cycles -= step_cycles as i32;
            let addr1 = core.pc << 1;
            if calltrace {
                if let Option::None = int_ret_addr {
                    if let Op::Rcall { k: 0 } = op_addr.op {
                    } else {
                        match op_addr.op {
                            Op::Call { .. } | Op::Icall { .. } | Op::Rcall { .. } => {
                                println!(
                                    "{pad:<2} {0:<pad$}{1:04x} -> {2:04x} call",
                                    "",
                                    addr0,
                                    core.pc << 1,
                                    pad = d,
                                );
                                d += 1;
                            }
                            Op::Ret { .. } | Op::Reti { .. } => {
                                if d != 0 {
                                    d -= 1;
                                } else {
                                    println!("D UNDERFLOW");
                                }
                                println!(
                                    "{pad:<2} {0:<pad$}{2:04x} <- {1:04x} ret",
                                    "",
                                    addr0,
                                    core.pc << 1,
                                    pad = d,
                                );
                            }
                            _ => {}
                        }
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
            step_cycles_sample -= step_cycles as i32;
            if step_cycles_sample < 0 {
                let v = if (core.gpio.port_c() & (1 << 6)) != 0 {
                    255
                } else {
                    127
                };
                sample[std::cmp::max(0, SAMPLES_FRAME - 1 - cycles / SAMPLE_CYCLES) as usize] = v;
                step_cycles_sample += SAMPLE_CYCLES;
            }
            // if core.pc == arduboy::int_vec::TIMER3_COMPA {
            // trace = true;
            // }
        }
        if d > 100 {
            println!("D OVERFLOW");
            d = 0;
        }

        {
            let mut audio_sample = audio.lock();
            audio_sample.position = 0;
            audio_sample.bytes = sample.clone();
        }

        core.display.render();

        tex_display.with_lock(None, |buffer: &mut [u8], pitch: usize| {
            for y in 0..HEIGTH {
                for x in 0..WIDTH {
                    let offset = y * pitch + x * 3;
                    if core.display.frame[y * WIDTH + x] != 0 {
                        buffer[offset] = 255;
                        buffer[offset + 1] = 255;
                        buffer[offset + 2] = 255;
                    } else {
                        buffer[offset] = 0;
                        buffer[offset + 1] = 0;
                        buffer[offset + 2] = 0;
                    }
                }
            }
        })?;

        canvas.clear();
        canvas.copy(&tex_display, None, None)?;

        // Update frame rate texture every k frames
        if frame % 10 == 0 {
            surf_fps = font
                .render(format!("{:02.0}", fps).as_str())
                .blended(Color::RGBA(100, 200, 100, 200))
                .map_err(|e| e.to_string())?;
            tex_fps = texture_creator
                .create_texture_from_surface(&surf_fps)
                .map_err(|e| e.to_string())?;
        }
        // Render frame rate
        let TextureQuery { width, height, .. } = tex_fps.query();
        const padding: i32 = 1;
        canvas.copy(
            &tex_fps,
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
        if save {
            let mut file = File::create(&save_path).unwrap();
            let mut bin = vec![0; serialize_len + 10];
            core.serialize(&mut bin)?;
            file.write_all(&bin)?;
            save = false;
        }
        if load {
            let mut file = File::open(&save_path).unwrap();
            let mut bin = Vec::new();
            file.read_to_end(&mut bin)?;
            core.deserialize(&bin)?;
            load = false;
        }
    }
    if record {
        // let key_log = serde_json::to_string(&key_events)?;
        let key_log = ron::to_string(&key_events)?;
        println!("{}", key_log);
    }
    // core.stats.print_summary(16, 16, 32);
    // println!("ops: {:?}", core.stats.ops);

    Ok(())
}
