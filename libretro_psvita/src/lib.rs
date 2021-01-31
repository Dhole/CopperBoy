#![no_std]
#![feature(default_alloc_error_handler)]

use core::ffi::c_void;
use libretro::export;
use libretro::libretro_h::*;

export!(fn retro_api_version() -> u32);
export!(fn retro_set_environment(cb: retro_environment_t));
export!(fn retro_set_video_refresh(cb: retro_video_refresh_t));
export!(fn retro_set_audio_sample(cb: retro_audio_sample_t));
export!(fn retro_set_audio_sample_batch(cb: retro_audio_sample_batch_t));
export!(fn retro_set_input_poll(cb: retro_input_poll_t));
export!(fn retro_set_input_state(cb: retro_input_state_t));
export!(fn retro_set_controller_port_device(_port: c_uint, _device: c_uint));
export!(fn retro_serialize_size() -> size_t);
export!(fn retro_serialize(_data: *mut c_void, _size: size_t) -> bool);
export!(fn retro_unserialize(_data: *const c_void, _size: size_t) -> bool);
export!(fn retro_cheat_reset());
export!(fn retro_cheat_set(_index: c_uint, _enabled: bool, _code: *const c_char));
export!(fn retro_get_system_av_info(info: *mut retro_system_av_info));
export!(fn retro_get_system_info(info: *mut retro_system_info));
export!(fn retro_init());
export!(fn retro_deinit());
export!(fn retro_load_game(game: *const retro_game_info) -> bool);
export!(fn retro_load_game_special( _game_type: c_uint, _info: *const retro_game_info, _num_info: size_t) -> bool);
export!(fn retro_unload_game());
export!(fn retro_get_region() -> c_uint);
export!(fn retro_get_memory_data(_id: c_uint) -> *mut c_void);
export!(fn retro_get_memory_size(_id: c_uint) -> size_t);
export!(fn retro_reset());
export!(fn retro_run());

#[panic_handler]
fn panic_impl(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
use core::alloc::{GlobalAlloc, Layout};
use core::ptr::null_mut;

struct MyAllocator;

unsafe impl GlobalAlloc for MyAllocator {
    unsafe fn alloc(&self, _layout: Layout) -> *mut u8 {
        null_mut()
    }
    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {}
}

#[global_allocator]
static A: MyAllocator = MyAllocator;
