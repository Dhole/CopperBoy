pub mod libretro_h;

use std::os::raw::c_void;
// use arduboy::core::{Core, GPIOPort};
use arduboy::display::{HEIGTH, WIDTH};
use arduboy::emulator::{Emulator, AUDIO_SAMPLE_FREQ, FPS};
use libretro_h::*;

macro_rules! c_str {
    ($lit:expr) => {
        concat!($lit, "\0").as_ptr() as *const std::os::raw::c_char
    };
}

macro_rules! as_c_void {
    ($lit:expr) => {
        &mut $lit as *mut _ as *mut c_void
    };
}

static mut emulator: Option<Emulator> = None;

#[allow(non_upper_case_globals)]
static mut cb_video_refresh: retro_video_refresh_t = None;
#[allow(non_upper_case_globals)]
static mut cb_input_poll: retro_input_poll_t = None;
#[allow(non_upper_case_globals)]
static mut cb_input_state: retro_input_state_t = None;
#[allow(non_upper_case_globals)]
static mut cb_audio_sample_batch: retro_audio_sample_batch_t = None;
#[allow(non_upper_case_globals)]
static mut cb_audio_sample: retro_audio_sample_t = None;
#[allow(non_upper_case_globals)]
static mut cb_environment: retro_environment_t = None;
#[allow(non_upper_case_globals)]
static mut cb_log: retro_log_printf_t = None;

#[no_mangle]
pub extern "C" fn retro_api_version() -> u32 {
    RETRO_API_VERSION
}

#[no_mangle]
pub extern "C" fn retro_set_environment(cb: retro_environment_t) {
    unsafe {
        cb_environment = cb;
    }
}
#[no_mangle]
pub extern "C" fn retro_set_video_refresh(cb: retro_video_refresh_t) {
    unsafe {
        cb_video_refresh = cb;
    }
}
#[no_mangle]
pub extern "C" fn retro_set_audio_sample(cb: retro_audio_sample_t) {
    unsafe {
        cb_audio_sample = cb;
    }
}
#[no_mangle]
pub extern "C" fn retro_set_audio_sample_batch(cb: retro_audio_sample_batch_t) {
    unsafe {
        cb_audio_sample_batch = cb;
    }
}
#[no_mangle]
pub extern "C" fn retro_set_input_poll(cb: retro_input_poll_t) {
    unsafe {
        cb_input_poll = cb;
    }
}
#[no_mangle]
pub extern "C" fn retro_set_input_state(cb: retro_input_state_t) {
    unsafe {
        cb_input_state = cb;
    }
}

#[no_mangle]
pub extern "C" fn retro_get_system_av_info(info: *mut retro_system_av_info) {
    let mut info = unsafe { info.as_mut().unwrap() };
    info.geometry = retro_game_geometry {
        base_width: WIDTH as u32,
        base_height: HEIGTH as u32,
        max_width: WIDTH as u32,
        max_height: HEIGTH as u32,
        aspect_ratio: WIDTH as f32 / HEIGTH as f32,
    };
    info.timing = retro_system_timing {
        fps: FPS as f64,
        sample_rate: AUDIO_SAMPLE_FREQ as f64,
    }
}

#[no_mangle]
pub extern "C" fn retro_get_system_info(info: *mut retro_system_info) {
    let info = unsafe { info.as_mut().unwrap() };
    *info = retro_system_info {
        library_name: c_str!("CopperBoy"),
        library_version: c_str!("0.1.0"),
        valid_extensions: c_str!("hex"),
        need_fullpath: false,
        block_extract: false,
    }
}

#[no_mangle]
pub extern "C" fn retro_init() {
    unsafe {
        emulator = Some(Emulator::new());
    }
    let mut log = retro_log_callback { log: None };
    unsafe {
        if cb_environment.unwrap()(RETRO_ENVIRONMENT_GET_LOG_INTERFACE, as_c_void!(log)) {
            cb_log = log.log;
        }
    }
    let fmt = RETRO_PIXEL_FORMAT_RGB565;
    unsafe {
        if !cb_environment.unwrap()(RETRO_ENVIRONMENT_SET_PIXEL_FORMAT, as_c_void!(fmt)) {
            cb_log.unwrap()(RETRO_LOG_ERROR, c_str!("RGB565 is not supported"))
        }
    }
}

#[no_mangle]
pub extern "C" fn retro_deinit() {
    unsafe {
        emulator = None;
    }
}

#[no_mangle]
pub extern "C" fn retro_load_game(game: *const retro_game_info) -> bool {
    let emu = unsafe { emulator.as_mut().unwrap() };
    let game = unsafe { game.as_ref().unwrap() };
    let game_data =
        unsafe { std::slice::from_raw_parts(game.data as *const u8, game.size as usize) };
    match emu.load_game(game_data) {
        Ok(_) => true,
        Err(_e) => {
            unsafe { cb_log.unwrap()(RETRO_LOG_ERROR, c_str!("error loading game")) }
            false
        }
    }
}

#[no_mangle]
pub extern "C" fn retro_reset() {}

#[no_mangle]
pub extern "C" fn retro_run() {
    let emu = unsafe { emulator.as_mut().unwrap() };
    emu.run();
}
