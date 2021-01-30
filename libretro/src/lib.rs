pub mod libretro_h;

use std::os::raw::c_void;
// use arduboy::core::{Core, GPIOPort};
use arduboy::display::{HEIGTH, WIDTH};
use arduboy::emulator::{Emulator, AUDIO_SAMPLE_FREQ, FPS, FRAME_SAMPLES};
use arduboy::keys::{PIN_A, PIN_B, PIN_DOWN, PIN_LEFT, PIN_RIGHT, PIN_UP};
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

// #[allow(dead_code)]
// struct Callbacks {
//     video_refresh: unsafe extern "C" fn(
//         data: *const ::std::os::raw::c_void,
//         width: ::std::os::raw::c_uint,
//         height: ::std::os::raw::c_uint,
//         pitch: size_t,
//     ),
//     input_poll: unsafe extern "C" fn(),
//     input_state: unsafe extern "C" fn(
//         port: ::std::os::raw::c_uint,
//         device: ::std::os::raw::c_uint,
//         index: ::std::os::raw::c_uint,
//         id: ::std::os::raw::c_uint,
//     ) -> i16,
//     audio_sample_batch: unsafe extern "C" fn(data: *const i16, frames: size_t) -> size_t,
//     audio_sample: unsafe extern "C" fn(left: i16, right: i16),
//     environment: unsafe extern "C" fn(
//         cmd: ::std::os::raw::c_uint,
//         data: *mut ::std::os::raw::c_void,
//     ) -> bool,
//     log_printf:
//         unsafe extern "C" fn(level: retro_log_level, fmt: *const ::std::os::raw::c_char, ...),
// }

const BYTES_PIXEL: usize = 2;

#[allow(non_upper_case_globals)]
static mut emulator: Option<Emulator> = None;
#[allow(non_upper_case_globals)]
static mut framebuffer: [u8; (WIDTH * HEIGTH * BYTES_PIXEL) as usize] =
    [0; (WIDTH * HEIGTH * BYTES_PIXEL) as usize];

// #[allow(non_upper_case_globals)]
// static mut callbacks: Option<Callbacks> = None;
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

#[no_mangle]
pub extern "C" fn retro_api_version() -> u32 {
    RETRO_API_VERSION
}

#[no_mangle]
pub extern "C" fn retro_set_environment(cb: retro_environment_t) {
    unsafe {
        callback_environment = cb;
    }
}
#[no_mangle]
pub extern "C" fn retro_set_video_refresh(cb: retro_video_refresh_t) {
    unsafe {
        callback_video_refresh = cb;
    }
}
#[no_mangle]
pub extern "C" fn retro_set_audio_sample(cb: retro_audio_sample_t) {
    unsafe {
        callback_audio_sample = cb;
    }
}
#[no_mangle]
pub extern "C" fn retro_set_audio_sample_batch(cb: retro_audio_sample_batch_t) {
    unsafe {
        callback_audio_sample_batch = cb;
    }
}
#[no_mangle]
pub extern "C" fn retro_set_input_poll(cb: retro_input_poll_t) {
    unsafe {
        callback_input_poll = cb;
    }
}
#[no_mangle]
pub extern "C" fn retro_set_input_state(cb: retro_input_state_t) {
    unsafe {
        callback_input_state = cb;
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
    // println!(">>> retro_init");
    let cb_environment = unsafe { callback_environment.unwrap() };
    let mut log = retro_log_callback { log: None };
    unsafe {
        if cb_environment(RETRO_ENVIRONMENT_GET_LOG_INTERFACE, as_mut_c_void!(log)) {
            callback_log_printf = log.log;
        }
    }
    let cb_log_printf = unsafe { callback_log_printf.unwrap() };
    // unsafe {
    //     callbacks = Some(Callbacks {
    //         video_refresh: cb_video_refresh.unwrap(),
    //         input_poll: cb_input_poll.unwrap(),
    //         input_state: cb_input_state.unwrap(),
    //         audio_sample_batch: cb_audio_sample_batch.unwrap(),
    //         audio_sample: cb_audio_sample.unwrap(),
    //         environment: cb_environment.unwrap(),
    //         log_printf: cb_log_printf.unwrap(),
    //     });
    // }
    // let cbs = unsafe { callbacks.as_ref().unwrap() };

    let mut fmt = RETRO_PIXEL_FORMAT_RGB565;
    unsafe {
        if !cb_environment(RETRO_ENVIRONMENT_SET_PIXEL_FORMAT, as_mut_c_void!(fmt)) {
            cb_log_printf(RETRO_LOG_ERROR, c_str!("RGB565 is not supported"))
        }
    }

    let new_desc = |port: ::std::os::raw::c_uint,
                    device: ::std::os::raw::c_uint,
                    index: ::std::os::raw::c_uint,
                    id: ::std::os::raw::c_uint,
                    description: *const ::std::os::raw::c_char|
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

#[no_mangle]
pub extern "C" fn retro_deinit() {
    unsafe {
        emulator = None;
    }
}

#[no_mangle]
pub extern "C" fn retro_load_game(game: *const retro_game_info) -> bool {
    // println!(">>> load_game");
    let cb_log_printf = unsafe { callback_log_printf.unwrap() };
    let emu = unsafe { emulator.as_mut().unwrap() };
    let game = unsafe { game.as_ref().unwrap() };
    let game_data =
        unsafe { std::slice::from_raw_parts(game.data as *const u8, game.size as usize) };
    // println!(">>> load_game size: {}", game_data.len());
    match emu.load_game(game_data) {
        Ok(_) => true,
        Err(_e) => {
            unsafe { cb_log_printf(RETRO_LOG_ERROR, c_str!("error loading game")) }
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
    // let mut _fb: &mut [u8] = fb;
    unsafe {
        cb_video_refresh(
            fb.as_ptr() as *const c_void,
            WIDTH as u32,
            HEIGTH as u32,
            (WIDTH * BYTES_PIXEL) as u64,
        );
    }
    unsafe {
        cb_audio_sample_batch(emu.samples.as_ptr() as *const i16, FRAME_SAMPLES as size_t);
    }
    // println!(">>> D");
}
