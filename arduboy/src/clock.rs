use num_traits::FromPrimitive;
// use num_traits::ToPrimitive;
use serde::{self, Deserialize, Serialize};

use super::int_vec::*;

#[cfg_attr(test, derive(core::cmp::PartialEq, core::fmt::Debug))]
#[derive(FromPrimitive, Clone, Copy, Serialize, Deserialize)]
enum PllInputPrescaler {
    Mhz8 = 0b0000_0000,
    Mhz16 = 0b0001_0000,
}

#[cfg_attr(test, derive(core::cmp::PartialEq, core::fmt::Debug))]
#[derive(FromPrimitive, Clone, Copy, Serialize, Deserialize)]
enum Mode {
    Normal = 0b00,
    PWMPhaseCorrect = 0b01,
    CTC = 0b10, // Clear Timer on Compare Match
    PWMFast = 0b11,
}

#[cfg_attr(test, derive(core::cmp::PartialEq, core::fmt::Debug))]
#[derive(FromPrimitive, ToPrimitive, Clone, Copy, Serialize, Deserialize)]
enum WaveGenMode {
    Normal = 0b0000,
    PWMPhaseCorrect8b = 0b0001,
    PWMPhaseCorrect9b = 0b0010,
    PWMPhaseCorrect10b = 0b0011,
    CTCOCR = 0b0100, // Clear Timer on Compare Match
    PWMFast8b = 0b0101,
    PWMFast9b = 0b0110,
    PWMFast10b = 0b0111,
    PWMPhaseFreqCorrectICR = 0b1000,
    PWMPhaseFreqCorrectOCR = 0b1001,
    PWMPhaseCorrectICR = 0b1010,
    PWMPhaseCorrectOCR = 0b1011,
    CTCICR = 0b1100, // Clear Timer on Compare Match
    Reserved = 0b1101,
    FastPWMICR = 0b1110,
    FASTPWMOCR = 0b1111,
}

#[cfg_attr(test, derive(core::cmp::PartialEq, core::fmt::Debug))]
#[derive(FromPrimitive, Clone, Copy, Serialize, Deserialize)]
enum ClockSelect {
    No = 0b000,
    Clk = 0b001,
    Clk8 = 0b010,
    Clk64 = 0b011,
    Clk256 = 0b100,
    Clk1024 = 0b101,
    ExtFalling = 0b110,
    ExtRising = 0b111,
}

#[cfg_attr(test, derive(core::cmp::PartialEq, core::fmt::Debug))]
#[derive(Serialize, Deserialize, Clone)]
pub struct Clock {
    pll_input_prescaler: PllInputPrescaler,
    pll_enable: bool,
    pll_lock_detector: bool,
    timer_0_mode: Mode,                // TCCR0A{WGM01, WGM00}
    timer_0_clock_select: ClockSelect, // TCCR0B{CS02, CS01, CS00}
    timer_0_cmp_b_int_enable: bool,
    timer_0_cmp_b_int: bool,
    timer_0_cmp_a_int_enable: bool,
    timer_0_cmp_a_int: bool,
    timer_0_ovf_int_enable: bool, // TIMSK0{TOIE0}
    timer_0_ovf_int: bool,        // TIFR0{TOV0}
    timer_0: u8,
    timer_0_cycles: u32,

    timer_3_wgm: WaveGenMode, // TCCR3B{WGM33, WGM32}, TCCR3A{WGM31, WGM30}
    timer_3_clock_select: ClockSelect, // TCCR3B{CS32, CS31, CS30}
    timer_3_cmp_c_int_enable: bool,
    timer_3_cmp_b_int_enable: bool,
    timer_3_cmp_a_int_enable: bool,
    timer_3_ovf_int_enable: bool, // TIMSK3{TOIE3}
    timer_3_ovf_int: bool,        // TIFR3{TOV3}
    timer_3_cmp_a_int: bool,      // TIFR3{OFC1A}
    timer_3: u16,
    timer_3_icr: u16,
    timer_3_ocra: u16,
    timer_3_cycles: u32,
}

impl Clock {
    pub fn new() -> Self {
        Self {
            pll_input_prescaler: PllInputPrescaler::Mhz8,
            pll_enable: false,
            pll_lock_detector: false,
            timer_0_mode: Mode::Normal,
            timer_0_clock_select: ClockSelect::No,
            timer_0_cmp_b_int_enable: false,
            timer_0_cmp_b_int: false,
            timer_0_cmp_a_int_enable: false,
            timer_0_cmp_a_int: false,
            timer_0_ovf_int_enable: false,
            timer_0_ovf_int: false,
            timer_0: 0,
            timer_0_cycles: 0,

            timer_3_wgm: WaveGenMode::Normal,
            timer_3_clock_select: ClockSelect::No,
            timer_3_cmp_c_int_enable: false,
            timer_3_cmp_b_int_enable: false,
            timer_3_cmp_a_int_enable: false,
            timer_3_ovf_int: false,
            timer_3_cmp_a_int: false,
            timer_3_ovf_int_enable: false,
            timer_3: 0,
            timer_3_icr: 0,
            timer_3_ocra: 0,
            timer_3_cycles: 0,
        }
    }

    pub fn step_timer_0(&mut self, cycles: usize) {
        match self.timer_0_clock_select {
            ClockSelect::No => return,
            ClockSelect::ExtFalling => return,
            ClockSelect::ExtRising => return,
            _ => {}
        }
        self.timer_0_cycles += cycles as u32;
        let (t, overflow) = match self.timer_0_clock_select {
            ClockSelect::Clk => {
                let delta = self.timer_0_cycles;
                self.timer_0_cycles = 0;
                self.timer_0.overflowing_add(delta as u8)
            }
            ClockSelect::Clk8 => {
                let delta = self.timer_0_cycles / 8;
                self.timer_0_cycles = self.timer_0_cycles % 8;
                self.timer_0.overflowing_add(delta as u8)
            }
            ClockSelect::Clk64 => {
                let delta = self.timer_0_cycles / 64;
                self.timer_0_cycles = self.timer_0_cycles % 64;
                self.timer_0.overflowing_add(delta as u8)
            }
            ClockSelect::Clk256 => {
                let delta = self.timer_0_cycles / 256;
                self.timer_0_cycles = self.timer_0_cycles % 256;
                self.timer_0.overflowing_add(delta as u8)
            }
            ClockSelect::Clk1024 => {
                let delta = self.timer_0_cycles / 1024;
                self.timer_0_cycles = self.timer_0_cycles % 1024;
                self.timer_0.overflowing_add(delta as u8)
            }
            _ => unreachable!(),
        };
        self.timer_0 = t;
        if overflow {
            self.timer_0_ovf_int = true;
        }
        match self.timer_0_mode {
            Mode::Normal => {
                if overflow {
                    self.timer_3_ovf_int = true;
                }
            }
            Mode::PWMPhaseCorrect => {
                if overflow {
                    self.timer_3_ovf_int = true;
                }
            }
            Mode::PWMFast => {
                if overflow {
                    self.timer_3_ovf_int = true;
                }
            }
            _ => {
                warn!("timer_0_mode: {:02x}", self.timer_0_mode as u8);
                unimplemented!()
            }
        }
    }

    pub fn step_timer_3(&mut self, cycles: usize) {
        match self.timer_3_clock_select {
            ClockSelect::No => return,
            ClockSelect::ExtFalling => return,
            ClockSelect::ExtRising => return,
            _ => {}
        }
        self.timer_3_cycles += cycles as u32;
        let (t, overflow) = match self.timer_3_clock_select {
            ClockSelect::Clk => {
                let delta = self.timer_3_cycles;
                self.timer_3_cycles = 0;
                self.timer_3.overflowing_add(delta as u16)
            }
            ClockSelect::Clk8 => {
                let delta = self.timer_3_cycles / 8;
                self.timer_3_cycles = self.timer_3_cycles % 8;
                self.timer_3.overflowing_add(delta as u16)
            }
            ClockSelect::Clk64 => {
                let delta = self.timer_3_cycles / 64;
                self.timer_3_cycles = self.timer_3_cycles % 64;
                self.timer_3.overflowing_add(delta as u16)
            }
            ClockSelect::Clk256 => {
                let delta = self.timer_3_cycles / 256;
                self.timer_3_cycles = self.timer_3_cycles % 256;
                self.timer_3.overflowing_add(delta as u16)
            }
            ClockSelect::Clk1024 => {
                let delta = self.timer_3_cycles / 1024;
                self.timer_3_cycles = self.timer_3_cycles % 1024;
                self.timer_3.overflowing_add(delta as u16)
                // let delta = self.timer_3_cycles / 100_000;
                // self.timer_3_cycles = self.timer_3_cycles % 10_000;
                // self.timer_3_cycles -= delta * 10_000;
                // self.timer_3_cycles = 0;
                // self.timer_3.overflowing_add(delta as u16)
            }
            _ => unreachable!(),
        };
        self.timer_3 = t;
        // debug!(">>> cycles: {}, timer3: {}", cycles, self.timer_3);
        match self.timer_3_wgm {
            WaveGenMode::Normal => {
                if overflow {
                    self.timer_3_ovf_int = true;
                }
            }
            WaveGenMode::PWMPhaseCorrect8b => {
                if self.timer_3 >= 0xff {
                    self.timer_3 = self.timer_3 % 0xff;
                    self.timer_3_ovf_int = true;
                }
            }
            WaveGenMode::PWMPhaseCorrect9b => {
                if self.timer_3 >= 0x1ff {
                    self.timer_3 = self.timer_3 % 0x1ff;
                    self.timer_3_ovf_int = true;
                }
            }
            WaveGenMode::PWMPhaseCorrect10b => {
                if self.timer_3 >= 0x3ff {
                    self.timer_3 = self.timer_3 % 0x3ff;
                    self.timer_3_ovf_int = true;
                }
            }
            WaveGenMode::CTCOCR => {
                if self.timer_3 >= self.timer_3_ocra {
                    if self.timer_3_ocra == 0 {
                        self.timer_3 = 0;
                    } else {
                        self.timer_3 = self.timer_3 % self.timer_3_ocra;
                    }
                    self.timer_3_ovf_int = true;
                    self.timer_3_cmp_a_int = true;
                }
            }
            WaveGenMode::PWMPhaseFreqCorrectICR => {
                if self.timer_3 >= self.timer_3_icr {
                    if self.timer_3_icr == 0 {
                        self.timer_3 = 0;
                    } else {
                        self.timer_3 = self.timer_3 % self.timer_3_icr;
                    }
                    self.timer_3_ovf_int = true;
                }
            }
            _ => {
                warn!("timer_3_wgm: {:02x}", self.timer_3_wgm as u8);
                unimplemented!()
            }
        }
    }

    pub fn step(&mut self, cycles: usize) {
        self.step_timer_0(cycles);
        self.step_timer_3(cycles);
    }

    pub fn int(&self) -> u64 {
        let mut bitmap = 0;
        if self.timer_0_ovf_int_enable && self.timer_0_ovf_int {
            bitmap |= TIMER0_OVF_BIT;
        }
        if self.timer_3_ovf_int_enable && self.timer_3_ovf_int {
            bitmap |= TIMER3_OVF_BIT;
        }
        if self.timer_3_cmp_a_int_enable && self.timer_3_cmp_a_int {
            bitmap |= TIMER3_COMPA_BIT;
        }
        bitmap
    }

    pub fn clear_int(&mut self, int: Interrupt) {
        match int {
            Interrupt::Timer0CompA => self.timer_0_cmp_a_int = false,
            Interrupt::Timer0CompB => self.timer_0_cmp_b_int = false,
            Interrupt::Timer0Ovf => self.timer_0_ovf_int = false,
            Interrupt::Timer3Ovf => self.timer_3_ovf_int = false,
            Interrupt::Timer3CompA => self.timer_3_cmp_a_int = false,
            _ => unreachable!(),
        }
    }

    pub fn reg_pllcsr(&self) -> u8 {
        self.pll_input_prescaler as u8 | (self.pll_enable as u8) << 1 | self.pll_lock_detector as u8
    }

    pub fn set_reg_pllcsr(&mut self, v: u8) {
        self.pll_input_prescaler =
            PllInputPrescaler::from_u8(v & PllInputPrescaler::Mhz16 as u8).unwrap();
        self.pll_enable = v & 0b0000_00010 != 0;
        if self.pll_enable {
            self.pll_lock_detector = true;
        }
    }

    pub fn reg_tccr0a(&self) -> u8 {
        self.timer_0_mode as u8
    }

    pub fn set_reg_tccr0a(&mut self, v: u8) {
        self.timer_0_mode = Mode::from_u8(v & 0b11).unwrap();
    }

    pub fn reg_tccr0b(&self) -> u8 {
        self.timer_0_clock_select as u8
    }

    pub fn set_reg_tccr0b(&mut self, v: u8) {
        self.timer_0_clock_select = ClockSelect::from_u8(v & 0b111).unwrap();
    }

    pub fn reg_tccr3a(&self) -> u8 {
        self.timer_3_wgm as u8
    }

    pub fn set_reg_tccr3a(&mut self, v: u8) {
        self.timer_3_wgm =
            WaveGenMode::from_u8((self.timer_3_wgm as u8 & 0b0000_1100) | v & 0b11).unwrap()
    }

    pub fn reg_tccr3b(&self) -> u8 {
        self.timer_3_clock_select as u8
    }

    pub fn set_reg_tccr3b(&mut self, v: u8) {
        self.timer_3_clock_select = ClockSelect::from_u8(v & 0b111).unwrap();
        self.timer_3_wgm =
            WaveGenMode::from_u8((self.timer_3_wgm as u8 & 0b0000_0011) | (v & 0b0001_1000) >> 1)
                .unwrap();
    }

    pub fn reg_timsk0(&self) -> u8 {
        (self.timer_0_cmp_b_int_enable as u8) << 2
            | (self.timer_0_cmp_a_int_enable as u8) << 1
            | (self.timer_0_ovf_int_enable as u8) << 0
    }

    pub fn set_reg_timsk0(&mut self, v: u8) {
        self.timer_0_cmp_b_int_enable = (v & 1 << 2) != 0;
        self.timer_0_cmp_a_int_enable = (v & 1 << 1) != 0;
        self.timer_0_ovf_int_enable = (v & 1 << 0) != 0;
    }

    pub fn reg_timsk3(&self) -> u8 {
        (self.timer_3_cmp_c_int_enable as u8) << 3
            | (self.timer_3_cmp_b_int_enable as u8) << 2
            | (self.timer_3_cmp_a_int_enable as u8) << 1
            | (self.timer_3_ovf_int_enable as u8) << 0
    }

    pub fn set_reg_timsk3(&mut self, v: u8) {
        self.timer_3_cmp_c_int_enable = (v & 1 << 3) != 0;
        self.timer_3_cmp_b_int_enable = (v & 1 << 2) != 0;
        self.timer_3_cmp_a_int_enable = (v & 1 << 1) != 0;
        self.timer_3_ovf_int_enable = (v & 1 << 0) != 0;
    }

    pub fn reg_tcnt0(&self) -> u8 {
        self.timer_0
    }

    pub fn set_reg_tcnt0(&mut self, v: u8) {
        self.timer_0 = v;
    }

    pub fn set_reg_tcnt3h(&mut self, v: u8) {
        self.timer_3 &= 0x00ff;
        self.timer_3 |= (v as u16) << 8;
    }

    pub fn set_reg_tcnt3l(&mut self, v: u8) {
        self.timer_3 &= 0xff00;
        self.timer_3 |= v as u16;
    }

    pub fn set_reg_icr3h(&mut self, v: u8) {
        self.timer_3_icr &= 0x00ff;
        self.timer_3_icr |= (v as u16) << 8;
    }

    pub fn set_reg_icr3l(&mut self, v: u8) {
        self.timer_3_icr &= 0xff00;
        self.timer_3_icr |= v as u16;
    }

    pub fn set_reg_ocr3ah(&mut self, v: u8) {
        self.timer_3_ocra &= 0x00ff;
        self.timer_3_ocra |= (v as u16) << 8;
    }

    pub fn set_reg_ocr3al(&mut self, v: u8) {
        self.timer_3_ocra &= 0xff00;
        self.timer_3_ocra |= v as u16;
    }
}
