// SSD
use log::{info, warn};

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

const CMD_DEACTIVATE_SCROLL: u8 = 0x2E;
const CMD_ACTIVATE_SCROLL: u8 = 0x2F;

const CMD_ADDR_MODE: u8 = 0x20;
const CMD_COLUMN_ADDR: u8 = 0x21;
const CMD_MULTIPLEX_RATIO: u8 = 0xA8;
const CMD_DISPLAY_OFFSET: u8 = 0xD3;
const CMD_CHARGE_PUMP_SETTING: u8 = 0x8D;
const CMD_COM_PINS_HW: u8 = 0xDA;
const CMD_CONTRAST: u8 = 0x81;
const CMD_PRECHARGE_PERIOD: u8 = 0xD9;
const CMD_VCOMH_DESELECT_LVL: u8 = 0xDB;
const CMD_CLOCK_DIVIDE_RATIO: u8 = 0xD5;

const CMD_ON: u8 = 0xAF;
const CMD_OFF: u8 = 0xAE;

#[derive(Clone, Copy)]
enum AddrMode {
    Vertical,
    Horizontal,
    Page,
}

#[derive(Clone, Copy)]
enum StateColumnAddress {
    Start,
    End,
}

#[derive(Clone, Copy)]
enum CmdState {
    None,
    AddressingMode,
    ColumnAddress(StateColumnAddress),
    MultiplexRatio,
    DisplayOffset,
    ChargePumpSetting,
    ComPinsHardware,
    Contrast,
    PrechargePeriod,
    VcomhDeselectLevel,
    ClockDivideRatio,
}

pub struct Display {
    pub frame: [u8; WIDTH * HEIGTH],
    fb: [u8; FB_LEN], // Frame Buffer
    // p: usize,         // Cursor Pointer
    dc: bool, // Data/Command Flag {false -> command, true -> data}
    // Internal registers
    cmd_state: CmdState, // TODO: Use this
    pixels_inverted: bool,
    vertical_flipped: bool,
    horizontal_flipped: bool,
    all_pixels_on: bool,
    addr_mode: AddrMode,
    on: bool,
    column_start: usize, // Column Start Address // TODO: Use this
    column_end: usize, // Column End Address for Horizontal/Vertical Addressing Mode // TODO: Use this
    column_range: usize,
    multiplex_ratio: u8,
    vertical_shift: u8,
    display_start_line: u8,
    contrast: u8,

    page_addr_column_start: usize, // Column Start Address // TODO: Use this
    page_addr_page_start: usize,

    page: usize,
    col_rel: usize,
}

impl Display {
    pub fn new() -> Self {
        Self {
            frame: [0; WIDTH * HEIGTH],
            fb: [0; FB_LEN],
            // p: 0,
            dc: false,
            //
            cmd_state: CmdState::None,
            pixels_inverted: false,
            vertical_flipped: false,
            horizontal_flipped: false,
            all_pixels_on: false,
            addr_mode: AddrMode::Page,
            on: false,
            column_start: 0,
            column_end: 127,
            column_range: 128,
            multiplex_ratio: 64,
            vertical_shift: 0,
            display_start_line: 0,
            contrast: 0x7f,
            page_addr_column_start: 0,
            page_addr_page_start: 0,
            page: 0,
            col_rel: 0,
        }
    }

    pub fn set_dc(&mut self, value: bool) {
        self.dc = value;
    }

    pub fn dc(&self) -> bool {
        self.dc
    }

    fn inc_col(&mut self) -> bool {
        self.col_rel += 1;
        if self.col_rel == self.column_range {
            self.col_rel = 0;
            true
        } else {
            false
        }
    }

    fn inc_page(&mut self) -> bool {
        self.page += 1;
        if self.page == HEIGTH / 8 {
            self.page = 0;
            true
        } else {
            false
        }
    }

    fn paint_8pixels(&mut self, pixels: u8) {
        self.fb[self.page * WIDTH + self.col_rel + self.column_start] = pixels;
        match self.addr_mode {
            AddrMode::Horizontal => {
                if self.inc_col() {
                    self.inc_page();
                }
            }
            AddrMode::Vertical => {
                if self.inc_page() {
                    self.inc_col();
                }
            }
            AddrMode::Page => {
                unimplemented!();
            }
        }
    }

    fn command(&mut self, cmd: u8) {
        match self.cmd_state {
            CmdState::None => match cmd {
                0x26 | 0x27 => {
                    warn!("TODO: continuous horizontal scroll setup");
                }
                0x29 | 0x2A => {
                    warn!("TODO: continuous vertical and horizontal scroll setup");
                }
                0xA3 => {
                    warn!("TODO: set vertical scroll area");
                }
                0x22 => {
                    warn!("TODO: set page address");
                }
                0xE3 => {
                    info!("NOP");
                }
                CMD_PIXELS_INVERTED => {
                    info!("pixels inverted");
                    self.pixels_inverted = true;
                }
                CMD_PIXELS_NORMAL => {
                    info!("pixels normal");
                    self.pixels_inverted = false;
                }
                CMD_ALL_PIXELS_ON => {
                    info!("all pixels on");
                    self.all_pixels_on = true;
                }
                CMD_PIXELS_FROM_RAM => {
                    info!("pixels from ram");
                    self.all_pixels_on = false;
                }
                CMD_VERTICAL_FLIPPED => {
                    info!("vertical: flipped");
                    self.vertical_flipped = true;
                }
                CMD_VERTICAL_NORMAL => {
                    info!("vertical: normal");
                    self.vertical_flipped = false;
                }
                CMD_HORIZ_FLIPPED => {
                    info!("horizontal: flipped");
                    self.horizontal_flipped = true;
                }
                CMD_HORIZ_NORMAL => {
                    info!("horizontal: normal");
                    self.horizontal_flipped = false;
                }
                CMD_ON => {
                    info!("on");
                    self.on = true;
                }
                CMD_OFF => {
                    info!("off");
                    self.on = false;
                }
                CMD_DEACTIVATE_SCROLL => {
                    info!("deactivate scroll");
                }
                CMD_ACTIVATE_SCROLL => {
                    info!("activate scroll");
                }
                CMD_ADDR_MODE => {
                    self.cmd_state = CmdState::AddressingMode;
                }
                CMD_COLUMN_ADDR => {
                    self.cmd_state = CmdState::ColumnAddress(StateColumnAddress::Start);
                }
                CMD_MULTIPLEX_RATIO => {
                    self.cmd_state = CmdState::MultiplexRatio;
                }
                CMD_DISPLAY_OFFSET => {
                    self.cmd_state = CmdState::DisplayOffset;
                }
                CMD_CHARGE_PUMP_SETTING => {
                    self.cmd_state = CmdState::ChargePumpSetting;
                }
                CMD_COM_PINS_HW => {
                    self.cmd_state = CmdState::ComPinsHardware;
                }
                CMD_CONTRAST => {
                    self.cmd_state = CmdState::Contrast;
                }
                CMD_PRECHARGE_PERIOD => {
                    self.cmd_state = CmdState::PrechargePeriod;
                }
                CMD_VCOMH_DESELECT_LVL => {
                    self.cmd_state = CmdState::VcomhDeselectLevel;
                }
                CMD_CLOCK_DIVIDE_RATIO => {
                    self.cmd_state = CmdState::ClockDivideRatio;
                }
                0x00..=0x0F => {
                    self.page_addr_column_start =
                        (self.page_addr_column_start & 0xf0) | ((cmd as usize & 0x0f) << 0);
                    info!(
                        "page_addr_column_start (L): {}",
                        self.page_addr_column_start
                    );
                }
                0x10..=0x1F => {
                    self.page_addr_column_start =
                        (self.page_addr_column_start & 0x0f) | ((cmd as usize & 0x0f) << 4);
                    info!(
                        "page_addr_column_start (H): {}",
                        self.page_addr_column_start
                    );
                }
                0x40..=0x7F => {
                    self.display_start_line = cmd & 0b0011_1111;
                    info!("display_start_line: {}", self.display_start_line);
                }
                0xB0..=0xB7 => {
                    self.page_addr_page_start = (cmd & 0b0000_0111) as usize;
                    info!("page_addr_page_start: {}", self.page_addr_page_start);
                }
                _ => {
                    warn!("Unknown display command {:02x}", cmd);
                }
            },
            CmdState::AddressingMode => {
                let mode = cmd & 0b11;
                match mode {
                    0b00 => {
                        info!("addr_mode: horizontal");
                        self.addr_mode = AddrMode::Horizontal;
                    }
                    0b01 => {
                        info!("addr_mode: vertical");
                        self.addr_mode = AddrMode::Vertical;
                    }
                    _ => {
                        warn!("Unexpected adderssing mode {:02b}", mode);
                    }
                }
                self.cmd_state = CmdState::None;
            }
            CmdState::Contrast => {
                self.contrast = cmd;
                info!("contrast: 0x{:02x}", self.contrast);
                self.cmd_state = CmdState::None;
            }
            CmdState::MultiplexRatio => {
                self.multiplex_ratio = std::cmp::max(15, cmd & 0b0011_1111) + 1;
                info!("multiplex_ratio: {}", self.multiplex_ratio);
                self.cmd_state = CmdState::None;
            }
            CmdState::PrechargePeriod => {
                info!(
                    "precharge_period: {}, {}",
                    (cmd & 0x0f) >> 0,
                    (cmd & 0xf0) >> 4
                );
                self.cmd_state = CmdState::None;
            }
            CmdState::VcomhDeselectLevel => {
                info!("vcomh_deselect_level: {}", (cmd & 0b0111_0000) >> 4);
                self.cmd_state = CmdState::None;
            }
            CmdState::ChargePumpSetting => {
                info!("charge_pump_setting: {}", (cmd & 0b0000_0100) >> 2);
                self.cmd_state = CmdState::None;
            }
            CmdState::ComPinsHardware => {
                info!("com_pins_hw: {}", (cmd & 0b0011_0000) >> 4);
                self.cmd_state = CmdState::None;
            }
            CmdState::ClockDivideRatio => {
                info!(
                    "divide_ratio: {}, freq_osc: {}",
                    ((cmd & 0x0f) >> 0) + 1,
                    (cmd & 0xf0) >> 4
                );
                self.cmd_state = CmdState::None;
            }
            CmdState::DisplayOffset => {
                self.vertical_shift = cmd & 0b0011_1111;
                info!("vertical_shift: {}", self.vertical_shift);
                self.cmd_state = CmdState::None;
            }
            CmdState::ColumnAddress(s) => match s {
                StateColumnAddress::Start => {
                    self.column_start = cmd as usize & 0b0111_1111;
                    self.cmd_state = CmdState::ColumnAddress(StateColumnAddress::End);
                }
                StateColumnAddress::End => {
                    self.column_end = cmd as usize & 0b0111_1111;
                    info!(
                        "colum_addr start-end: {}-{}",
                        self.column_start, self.column_end
                    );
                    self.column_range = if self.column_end >= self.column_start {
                        self.column_end + 1 - self.column_start
                    } else {
                        self.column_end + 1 + (WIDTH - self.column_start)
                    };
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
        // if !self.addr_mode_vertical {
        for row in 0..HEIGTH / 8 {
            for x in 0..WIDTH {
                let pixels = self.fb[row * WIDTH + x];
                draw_8px(&mut self.frame, pixels, x, row, self.horizontal_flipped);
            }
        }
        // } else {
        //     for x in 0..WIDTH {
        //         for row in 0..HEIGTH / 8 {
        //             let pixels = self.fb[x * HEIGTH / 8 + row];
        //             draw_8px(&mut self.frame, pixels, x, row, self.horizontal_flipped);
        //         }
        //     }
        // }
    }
}

fn draw_8px(frame: &mut [u8; WIDTH * HEIGTH], pixels: u8, x: usize, row: usize, flip_v: bool) {
    let x = if flip_v { WIDTH - 1 - x } else { x };
    for dy in 0..8 {
        frame[(row * 8 + dy) * WIDTH + x] = if pixels & (1 << dy) != 0 { 1 } else { 0 };
    }
}
