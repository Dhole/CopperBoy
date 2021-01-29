pub mod libretro_h;

use std::os::raw::c_void;
// use arduboy::core::{Core, GPIOPort};
use arduboy::display::{HEIGTH, WIDTH};
use arduboy::emulator::{Emulator, AUDIO_SAMPLE_FREQ, FPS, FRAME_SAMPLES};
use libretro_h::*;

macro_rules! c_str {
    ($lit:expr) => {
        concat!($lit, "\0").as_ptr() as *const std::os::raw::c_char
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
pub extern "C" fn retro_set_controller_port_device(
    _port: std::os::raw::c_uint,
    _device: std::os::raw::c_uint,
) {
}

#[no_mangle]
pub extern "C" fn retro_serialize_size() -> size_t {
    0
}

#[no_mangle]
pub extern "C" fn retro_serialize(_data: *mut ::std::os::raw::c_void, _size: size_t) -> bool {
    false
}
#[no_mangle]
pub extern "C" fn retro_unserialize(_data: *const ::std::os::raw::c_void, _size: size_t) -> bool {
    false
}
#[no_mangle]
pub extern "C" fn retro_cheat_reset() {}
#[no_mangle]
pub extern "C" fn retro_cheat_set(
    _index: ::std::os::raw::c_uint,
    _enabled: bool,
    _code: *const ::std::os::raw::c_char,
) {
}

#[no_mangle]
pub extern "C" fn retro_get_system_av_info(info: *mut retro_system_av_info) {
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
    println!(">>> retro_init");
    unsafe {
        emulator = Some(Emulator::new());
    }
    let mut log = retro_log_callback { log: None };
    unsafe {
        if cb_environment.unwrap()(RETRO_ENVIRONMENT_GET_LOG_INTERFACE, as_mut_c_void!(log)) {
            cb_log = log.log;
        }
    }
    let mut fmt = RETRO_PIXEL_FORMAT_RGB565;
    // let mut fmt = RETRO_PIXEL_FORMAT_XRGB8888;
    unsafe {
        if !cb_environment.unwrap()(RETRO_ENVIRONMENT_SET_PIXEL_FORMAT, as_mut_c_void!(fmt)) {
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
    println!(">>> load_game");
    let emu = unsafe { emulator.as_mut().unwrap() };
    let game = unsafe { game.as_ref().unwrap() };
    let game_data =
        unsafe { std::slice::from_raw_parts(game.data as *const u8, game.size as usize) };
    println!(">>> load_game size: {}", game_data.len());
    match emu.load_game(game_data) {
        Ok(_) => true,
        Err(_e) => {
            unsafe { cb_log.unwrap()(RETRO_LOG_ERROR, c_str!("error loading game")) }
            false
        }
    }
}

#[no_mangle]
pub extern "C" fn retro_load_game_special(
    _game_type: ::std::os::raw::c_uint,
    _info: *const retro_game_info,
    _num_info: size_t,
) -> bool {
    false
}

#[no_mangle]
pub extern "C" fn retro_unload_game() {}

#[no_mangle]
pub extern "C" fn retro_get_region() -> ::std::os::raw::c_uint {
    0
}
#[no_mangle]
pub extern "C" fn retro_get_memory_data(
    _id: ::std::os::raw::c_uint,
) -> *mut ::std::os::raw::c_void {
    std::ptr::null_mut()
}
#[no_mangle]
pub extern "C" fn retro_get_memory_size(_id: ::std::os::raw::c_uint) -> size_t {
    0
}

#[no_mangle]
pub extern "C" fn retro_reset() {}

#[no_mangle]
pub extern "C" fn retro_run() {
    // println!(">>> retro_run");
    let emu = unsafe { emulator.as_mut().unwrap() };
    // println!(">>> A");
    emu.run();
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
    // let mut _fb: &mut [u8] = fb;
    unsafe {
        // cb_video_refresh.unwrap()(
        //     as_c_void!(fb.as_mut().as_mut_ptr()),
        //     WIDTH as u32,
        //     HEIGTH as u32,
        //     (WIDTH * 3) as u64,
        // );
        cb_video_refresh.unwrap()(
            fb.as_ptr() as *const c_void,
            WIDTH as u32,
            HEIGTH as u32,
            (WIDTH * BYTES_PIXEL) as u64,
        );
    }
    unsafe {
        cb_audio_sample_batch.unwrap()(emu.samples.as_ptr() as *const i16, FRAME_SAMPLES as size_t);
    }
    // println!(">>> D");
}
