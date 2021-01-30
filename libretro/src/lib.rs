#![cfg_attr(not(feature = "std"), no_std)]

pub mod libretro_h;

use arduboy::display::{HEIGTH, WIDTH};
use arduboy::emulator::{Emulator, AUDIO_SAMPLE_FREQ, FPS, FRAME_SAMPLES};
use arduboy::keys::{PIN_A, PIN_B, PIN_DOWN, PIN_LEFT, PIN_RIGHT, PIN_UP};
use core::ffi::c_void;
use libretro_h::*;

macro_rules! c_str {
    ($lit:expr) => {
        concat!($lit, "\0").as_ptr() as *const c_char
    };
}

macro_rules! as_mut_c_void {
    ($lit:expr) => {
        &mut $lit as *mut _ as *mut c_void
    };
}

const BYTES_PIXEL: usize = 2;

#[allow(non_upper_case_globals)]
static mut emulator: Option<Emulator> = None;
#[allow(non_upper_case_globals)]
static mut framebuffer: [u8; (WIDTH * HEIGTH * BYTES_PIXEL) as usize] =
    [0; (WIDTH * HEIGTH * BYTES_PIXEL) as usize];

#[allow(non_upper_case_globals)]
static mut callback_video_refresh: retro_video_refresh_t = None;
#[allow(non_upper_case_globals)]
static mut callback_input_poll: retro_input_poll_t = None;
#[allow(non_upper_case_globals)]
static mut callback_input_state: retro_input_state_t = None;
#[allow(non_upper_case_globals)]
static mut callback_audio_sample_batch: retro_audio_sample_batch_t = None;
#[allow(non_upper_case_globals)]
static mut callback_audio_sample: retro_audio_sample_t = None;
#[allow(non_upper_case_globals)]
static mut callback_environment: retro_environment_t = None;
#[allow(non_upper_case_globals)]
static mut callback_log_printf: retro_log_printf_t = None;

pub fn retro_api_version() -> u32 {
    RETRO_API_VERSION
}

pub fn retro_set_environment(cb: retro_environment_t) {
    unsafe {
        callback_environment = cb;
    }
}
pub fn retro_set_video_refresh(cb: retro_video_refresh_t) {
    unsafe {
        callback_video_refresh = cb;
    }
}
pub fn retro_set_audio_sample(cb: retro_audio_sample_t) {
    unsafe {
        callback_audio_sample = cb;
    }
}
pub fn retro_set_audio_sample_batch(cb: retro_audio_sample_batch_t) {
    unsafe {
        callback_audio_sample_batch = cb;
    }
}
pub fn retro_set_input_poll(cb: retro_input_poll_t) {
    unsafe {
        callback_input_poll = cb;
    }
}
pub fn retro_set_input_state(cb: retro_input_state_t) {
    unsafe {
        callback_input_state = cb;
    }
}

pub fn retro_set_controller_port_device(_port: c_uint, _device: c_uint) {}

pub fn retro_serialize_size() -> size_t {
    0
}

pub fn retro_serialize(_data: *mut c_void, _size: size_t) -> bool {
    false
}
pub fn retro_unserialize(_data: *const c_void, _size: size_t) -> bool {
    false
}
pub fn retro_cheat_reset() {}
pub fn retro_cheat_set(_index: c_uint, _enabled: bool, _code: *const c_char) {}

pub fn retro_get_system_av_info(info: *mut retro_system_av_info) {
    let mut info = unsafe { info.as_mut().unwrap() };
    info.geometry = retro_game_geometry {
        base_width: WIDTH as u32,
        base_height: HEIGTH as u32,
        max_width: WIDTH as u32,
        max_height: HEIGTH as u32,
        aspect_ratio: 0.,
    };
    info.timing = retro_system_timing {
        fps: FPS as f64,
        sample_rate: AUDIO_SAMPLE_FREQ as f64,
    }
}

pub fn retro_get_system_info(info: *mut retro_system_info) {
    let info = unsafe { info.as_mut().unwrap() };
    *info = retro_system_info {
        library_name: c_str!("CopperBoy"),
        library_version: c_str!("0.1.0"),
        valid_extensions: c_str!("hex"),
        need_fullpath: false,
        block_extract: false,
    }
}

pub fn retro_init() {
    // println!(">>> retro_init");
    let cb_environment = unsafe { callback_environment.unwrap() };
    let mut log = retro_log_callback { log: None };
    unsafe {
        if cb_environment(RETRO_ENVIRONMENT_GET_LOG_INTERFACE, as_mut_c_void!(log)) {
            callback_log_printf = log.log;
        }
    }
    let cb_log_printf = unsafe { callback_log_printf.unwrap() };

    let mut fmt = RETRO_PIXEL_FORMAT_RGB565;
    unsafe {
        if !cb_environment(RETRO_ENVIRONMENT_SET_PIXEL_FORMAT, as_mut_c_void!(fmt)) {
            cb_log_printf(RETRO_LOG_ERROR, c_str!("RGB565 is not supported"))
        }
    }

    let new_desc = |port: c_uint,
                    device: c_uint,
                    index: c_uint,
                    id: c_uint,
                    description: *const c_char|
     -> retro_input_descriptor {
        retro_input_descriptor {
            port,
            device,
            index,
            id,
            description,
        }
    };

    #[rustfmt::skip]
    let mut descs = [
        new_desc(0, RETRO_DEVICE_JOYPAD, 0, RETRO_DEVICE_ID_JOYPAD_UP, c_str!("Up")),
        new_desc(0, RETRO_DEVICE_JOYPAD, 0, RETRO_DEVICE_ID_JOYPAD_DOWN, c_str!("Down")),
        new_desc(0, RETRO_DEVICE_JOYPAD, 0, RETRO_DEVICE_ID_JOYPAD_LEFT, c_str!("Left")),
        new_desc(0, RETRO_DEVICE_JOYPAD, 0, RETRO_DEVICE_ID_JOYPAD_RIGHT, c_str!("Right")),
        new_desc(0, RETRO_DEVICE_JOYPAD, 0, RETRO_DEVICE_ID_JOYPAD_A, c_str!("A")),
        new_desc(0, RETRO_DEVICE_JOYPAD, 0, RETRO_DEVICE_ID_JOYPAD_B, c_str!("B")),
    ];
    unsafe {
        if !cb_environment(
            RETRO_ENVIRONMENT_SET_INPUT_DESCRIPTORS,
            as_mut_c_void!(descs),
        ) {
            cb_log_printf(RETRO_LOG_ERROR, c_str!("unable to set input descriptors"))
        }
    }

    unsafe {
        emulator = Some(Emulator::new());
    }
}

pub fn retro_deinit() {
    unsafe {
        emulator = None;
    }
}

pub fn retro_load_game(game: *const retro_game_info) -> bool {
    // println!(">>> load_game");
    let cb_log_printf = unsafe { callback_log_printf.unwrap() };
    let emu = unsafe { emulator.as_mut().unwrap() };
    let game = unsafe { game.as_ref().unwrap() };
    let game_data =
        unsafe { core::slice::from_raw_parts(game.data as *const u8, game.size as usize) };
    // println!(">>> load_game size: {}", game_data.len());
    match emu.load_game(game_data) {
        Ok(_) => true,
        Err(_e) => {
            unsafe { cb_log_printf(RETRO_LOG_ERROR, c_str!("error loading game")) }
            false
        }
    }
}

pub fn retro_load_game_special(
    _game_type: c_uint,
    _info: *const retro_game_info,
    _num_info: size_t,
) -> bool {
    false
}

pub fn retro_unload_game() {}

pub fn retro_get_region() -> c_uint {
    0
}
pub fn retro_get_memory_data(_id: c_uint) -> *mut c_void {
    core::ptr::null_mut()
}
pub fn retro_get_memory_size(_id: c_uint) -> size_t {
    0
}

pub fn retro_reset() {}

pub fn retro_run() {
    // println!(">>> retro_run");
    let cb_input_poll = unsafe { callback_input_poll.unwrap() };
    let cb_input_state = unsafe { callback_input_state.unwrap() };
    let cb_video_refresh = unsafe { callback_video_refresh.unwrap() };
    let cb_audio_sample_batch = unsafe { callback_audio_sample_batch.unwrap() };
    let emu = unsafe { emulator.as_mut().unwrap() };

    unsafe {
        cb_input_poll();
    }
    let mut port_b: u8 = 0xff;
    let mut port_e: u8 = 0xff;
    let mut port_f: u8 = 0xff;
    unsafe {
        if cb_input_state(0, RETRO_DEVICE_JOYPAD, 0, RETRO_DEVICE_ID_JOYPAD_UP) != 0 {
            port_f &= !PIN_UP;
        }
        if cb_input_state(0, RETRO_DEVICE_JOYPAD, 0, RETRO_DEVICE_ID_JOYPAD_DOWN) != 0 {
            port_f &= !PIN_DOWN;
        }
        if cb_input_state(0, RETRO_DEVICE_JOYPAD, 0, RETRO_DEVICE_ID_JOYPAD_LEFT) != 0 {
            port_f &= !PIN_LEFT;
        }
        if cb_input_state(0, RETRO_DEVICE_JOYPAD, 0, RETRO_DEVICE_ID_JOYPAD_RIGHT) != 0 {
            port_f &= !PIN_RIGHT;
        }
        if cb_input_state(0, RETRO_DEVICE_JOYPAD, 0, RETRO_DEVICE_ID_JOYPAD_A) != 0 {
            port_e &= !PIN_A;
        }
        if cb_input_state(0, RETRO_DEVICE_JOYPAD, 0, RETRO_DEVICE_ID_JOYPAD_B) != 0 {
            port_b &= !PIN_B;
        }
    }

    // println!(">>> A");
    emu.run(port_b, port_e, port_f);
    // println!(">>> B");
    let fb = unsafe { framebuffer.as_mut() };
    for y in 0..HEIGTH {
        for x in 0..WIDTH {
            let offset = (y * WIDTH + x) * BYTES_PIXEL;
            if emu.core.display.frame[y * WIDTH + x] == 1 {
                fb[offset] = 255;
                fb[offset + 1] = 255;
            } else {
                fb[offset] = 0;
                fb[offset + 1] = 0;
            }
        }
    }
    // println!(">>> C");
    unsafe {
        cb_video_refresh(
            fb.as_ptr() as *const c_void,
            WIDTH as u32,
            HEIGTH as u32,
            (WIDTH * BYTES_PIXEL) as size_t,
        );
    }
    unsafe {
        cb_audio_sample_batch(emu.samples.as_ptr() as *const i16, FRAME_SAMPLES as size_t);
    }
    // println!(">>> D");
}

#[macro_export]
macro_rules! export {
    (fn $name:ident($( $arg:ident : $type:ty ),*) -> $ret:ty) => {
        #[no_mangle]
        pub unsafe extern "C" fn $name($( $arg : $type),*) -> $ret {
            libretro::$name($( $arg ),*)
        }
    };
    (fn $name:ident($( $arg:ident : $type:ty ),*)) => {
        export!(fn $name($( $arg : $type),*) -> ());
    }
}
