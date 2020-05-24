use log::warn;

pub const WIDTH: usize = 128;
pub const HEIGTH: usize = 64;
pub const FB_LEN: usize = WIDTH * HEIGTH / 8;

const CMD_PIXELS_INVERTED: u8 = 0xA7; // All pixels inverted
const CMD_PIXELS_NORMAL: u8 = 0xA6; // All pixels normal

const CMD_ALL_PIXELS_ON: u8 = 0xA5; // all pixels on
const CMD_PIXELS_FROM_RAM: u8 = 0xA4; // pixels mapped to display RAM contents

const CMD_VERTICAL_FLIPPED: u8 = 0xC0; // reversed COM scan direction
const CMD_VERTICAL_NORMAL: u8 = 0xC8; // normal COM scan direction

const CMD_HORIZ_FLIPPED: u8 = 0xA0; // reversed segment re-map
const CMD_HORIZ_NORMAL: u8 = 0xA1; // normal segment re-map

const CMD_ADDR_MODE: u8 = 0x20;

const CMD_ON: u8 = 0xAF;
const CMD_OFF: u8 = 0xAE;

#[derive(Clone, Copy)]
enum StateAddressingMode {
    Mode,
}

#[derive(Clone, Copy)]
enum CmdState {
    None,
    AddressingMode(StateAddressingMode),
}

pub struct Display {
    pub frame: [u8; WIDTH * HEIGTH],
    fb: [u8; FB_LEN], // Frame Buffer
    p: usize,         // Cursor Pointer
    dc: bool,         // Data/Command Flag {false -> command, true -> data}
    // Internal registers
    cmd_state: CmdState,
    pixels_inverted: bool,
    vertical_flipped: bool,
    horizontal_flipped: bool,
    all_pixels_on: bool,
    addr_mode_vertical: bool,
    on: bool,
}

impl Display {
    pub fn new() -> Self {
        Self {
            frame: [0; WIDTH * HEIGTH],
            fb: [0; FB_LEN],
            p: 0,
            dc: false,
            //
            cmd_state: CmdState::None,
            pixels_inverted: false,
            vertical_flipped: false,
            horizontal_flipped: false,
            all_pixels_on: false,
            addr_mode_vertical: false,
            on: false,
        }
    }

    pub fn set_dc(&mut self, value: bool) {
        self.dc = value;
    }

    fn paint_8pixels(&mut self, pixels: u8) {
        self.fb[self.p] = pixels;
        self.p = (self.p + 1) % FB_LEN;
    }

    fn command(&mut self, cmd: u8) {
        match self.cmd_state {
            CmdState::None => match cmd {
                CMD_PIXELS_INVERTED => {
                    self.pixels_inverted = true;
                }
                CMD_PIXELS_NORMAL => {
                    self.pixels_inverted = false;
                }
                CMD_ALL_PIXELS_ON => {
                    self.all_pixels_on = true;
                }
                CMD_PIXELS_FROM_RAM => {
                    self.all_pixels_on = false;
                }
                CMD_VERTICAL_FLIPPED => {
                    self.vertical_flipped = true;
                }
                CMD_VERTICAL_NORMAL => {
                    self.vertical_flipped = false;
                }
                CMD_HORIZ_FLIPPED => {
                    self.horizontal_flipped = true;
                }
                CMD_HORIZ_NORMAL => {
                    self.horizontal_flipped = false;
                }
                CMD_ON => {
                    self.on = true;
                }
                CMD_OFF => {
                    self.on = false;
                }
                CMD_ADDR_MODE => {
                    self.cmd_state = CmdState::AddressingMode(StateAddressingMode::Mode);
                }
                _ => {
                    warn!("Unknown display command {:02x}", cmd);
                }
            },
            CmdState::AddressingMode(s) => match s {
                StateAddressingMode::Mode => {
                    let mode = cmd & 0b11;
                    match mode {
                        0b00 => {
                            self.addr_mode_vertical = false;
                        }
                        0b01 => {
                            self.addr_mode_vertical = true;
                        }
                        _ => {
                            warn!("Unexpected adderssing mode {:02b}", mode);
                        }
                    }
                    self.cmd_state = CmdState::None;
                }
            },
        }
    }

    pub fn spi_write(&mut self, w: u8) {
        if self.dc {
            // println!("DBG spi write data {}", w);
            self.paint_8pixels(w);
        } else {
            // println!("DBG spi write command {}", w);
            self.command(w);
        }
    }

    pub fn render(&mut self) {
        if !self.on {
            for i in 0..WIDTH * HEIGTH {
                self.frame[i] = 0;
            }
            return;
        }
        if !self.addr_mode_vertical {
            for row in 0..HEIGTH / 8 {
                for x in 0..WIDTH {
                    let pixels = self.fb[row * WIDTH + x];
                    draw_8px(&mut self.frame, pixels, x, row, self.horizontal_flipped);
                }
            }
        } else {
            for x in 0..WIDTH {
                for row in 0..HEIGTH / 8 {
                    let pixels = self.fb[x * HEIGTH / 8 + row];
                    draw_8px(&mut self.frame, pixels, x, row, self.horizontal_flipped);
                }
            }
        }
    }
}

fn draw_8px(frame: &mut [u8; WIDTH * HEIGTH], pixels: u8, x: usize, row: usize, flip_v: bool) {
    let x = if flip_v { WIDTH - 1 - x } else { x };
    for dy in 0..8 {
        frame[(row * 8 + dy) * WIDTH + x] = if pixels & (1 << dy) != 0 { 1 } else { 0 };
    }
}
