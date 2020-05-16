pub const WIDTH: usize = 128;
pub const HEIGHT: usize = 64;
pub const FB_LEN: usize = WIDTH * HEIGHT / 8;

pub struct Display {
    pub fb: [u8; FB_LEN], // Frame Buffer
    p: usize,             // Cursor Pointer
    dc: bool,             // Data/Command Flag {false -> command, true -> data}
}

impl Display {
    pub fn new() -> Self {
        Self {
            fb: [0; FB_LEN],
            p: 0,
            dc: false,
        }
    }

    pub fn set_dc(&mut self, value: bool) {
        self.dc = value;
    }

    fn paint_8pixels(&mut self, pixels: u8) {
        self.fb[self.p] = pixels;
        self.p = (self.p + 1) % FB_LEN;
        if self.p == FB_LEN - 1 {}
    }

    fn command(&mut self, cmd: u8) {
        // TODO
    }

    pub fn spi_write(&mut self, w: u8) {
        if self.dc {
            self.paint_8pixels(w);
        } else {
            self.command(w);
        }
    }
}
