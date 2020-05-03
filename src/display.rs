use sdl2::video::Window;

pub const WIDTH: u32 = 128;
pub const HEIGHT: u32 = 64;

pub const SCALE: u32 = 4;

struct Display {
    window: Window,
}

impl Display {
    pub fn new() -> Result<Self, String> {
        let sdl_context = sdl2::init()?;
        let video_subsystem = sdl_context.video()?;
        let window = video_subsystem
            .window("Arduboy Emulator", WIDTH * SCALE, HEIGHT * SCALE)
            .position_centered()
            .build()
            .map_err(|e| e.to_string())?;
        Ok(Self { window })
    }

    // pub fn draw_frame(&mut self, )
}
