use sdl2::pixels::Color;
use sdl2::rect::Rect;

pub const WIDTH: usize = 128;
pub const HEIGTH: usize = 64;
pub const FB_LEN: usize = WIDTH * HEIGTH / 8;

pub struct Display {
    pub fb: [u8; FB_LEN], // Frame Buffer
    p: usize,             // Cursor Pointer
    dc: bool,             // Data/Command Flag {false -> command, true -> data}
    canvas: sdl2::render::Canvas<sdl2::video::Window>,
    scale: u32,
}

impl Display {
    pub fn new() -> Self {
        let scale = 4;
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem
            .window("arduboy-rs", WIDTH as u32 * scale, HEIGTH as u32 * scale)
            .position_centered()
            .opengl()
            .build()
            .map_err(|e| e.to_string())
            .unwrap();
        let mut canvas = window
            .into_canvas()
            .build()
            .map_err(|e| e.to_string())
            .unwrap();
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.present();
        Self {
            fb: [0; FB_LEN],
            p: 0,
            dc: false,
            canvas: canvas,
            scale: scale,
        }
    }

    pub fn set_dc(&mut self, value: bool) {
        self.dc = value;
    }

    fn paint_8pixels(&mut self, pixels: u8) {
        self.fb[self.p] = pixels;
        self.p = (self.p + 1) % FB_LEN;
        if self.p == FB_LEN - 1 {
            println!("DBG display paint");
            self.canvas.set_draw_color(Color::RGB(0, 0, 0));
            self.canvas.clear();
            self.canvas.set_draw_color(Color::RGB(255, 255, 255));

            for row in 0..HEIGTH / 8 {
                for x in 0..WIDTH {
                    let pixels = self.fb[row * WIDTH + x];
                    for dy in 0..8 {
                        if pixels & (1 << dy) != 0 {
                            self.canvas
                                .fill_rect(Rect::new(
                                    x as i32 * self.scale as i32,
                                    (row * 8 + dy) as i32 * self.scale as i32,
                                    self.scale,
                                    self.scale,
                                ))
                                .unwrap();
                        }
                    }
                }
            }
            self.canvas.present();
        }
    }

    fn command(&mut self, cmd: u8) {
        // TODO
    }

    pub fn spi_write(&mut self, w: u8) {
        if self.dc {
            println!("DBG spi write data {}", w);
            self.paint_8pixels(w);
        } else {
            println!("DBG spi write command {}", w);
            self.command(w);
        }
    }
}
