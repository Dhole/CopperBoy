use num_traits::FromPrimitive;
use num_traits::ToPrimitive;

use super::int_vec::*;

#[derive(FromPrimitive, Clone, Copy)]
enum PllInputPrescaler {
    Mhz8 = 0b0000_0000,
    Mhz16 = 0b0001_0000,
}

#[derive(FromPrimitive, Clone, Copy)]
enum Mode {
    Normal = 0b00,
    PWMPhaseCorrect = 0b01,
    CTC = 0b10, // Clear Timer on COmpare Match
    PWMFast = 0b11,
}

#[derive(FromPrimitive, Clone, Copy)]
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

pub struct Clock {
    pll_input_prescaler: PllInputPrescaler,
    pll_enable: bool,
    pll_lock_detector: bool,
    timer_0_mode: Mode,                // TCCR0A{WGM01, WGM00}
    timer_0_clock_select: ClockSelect, // TCCR0B{CS01, CS00}
    timer_0_cmp_b_int_enable: bool,
    timer_0_cmp_b_int: bool,
    timer_0_cmp_a_int_enable: bool,
    timer_0_cmp_a_int: bool,
    timer_0_ovf_int_enable: bool, // TIMSK0{TOIE0}
    timer_0_ovf_int: bool,        // TIFR0{TOV0}
    timer_0: u8,
    timer_0_cycles: u32,
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
        }
    }

    pub fn step(&mut self, cycles: u8) {
        match self.timer_0_clock_select {
            ClockSelect::No => return,
            ClockSelect::ExtFalling => return,
            ClockSelect::ExtRising => return,
            _ => {}
        }
        self.timer_0_cycles += cycles as u32;
        let (t, overflow) = match self.timer_0_clock_select {
            ClockSelect::Clk => self.timer_0.overflowing_add(self.timer_0_cycles as u8),
            ClockSelect::Clk8 => {
                let delta = self.timer_0_cycles / 8;
                self.timer_0_cycles = self.timer_0_cycles % 8;
                self.timer_0.overflowing_add(delta as u8)
            }
            ClockSelect::Clk64 => {
                println!("CLOCK 64: {}", self.timer_0);
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
            println!("INTERRUPT 1");
            self.timer_0_ovf_int = true;
        }
    }

    pub fn int(&self) -> u64 {
        let mut bitmap = 0;
        if self.timer_0_ovf_int_enable && self.timer_0_ovf_int {
            bitmap |= Interrupt::Timer0Ovf.to_u64().unwrap();
        }
        bitmap
    }

    pub fn clear_int(&mut self, int: Interrupt) {
        match int {
            Interrupt::Timer0CompA => self.timer_0_cmp_a_int = false,
            Interrupt::Timer0CompB => self.timer_0_cmp_b_int = false,
            Interrupt::Timer0Ovf => self.timer_0_ovf_int = false,
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

    pub fn reg_tcnt0(&self) -> u8 {
        self.timer_0
    }

    pub fn set_reg_tcnt0(&mut self, v: u8) {
        self.timer_0 = v;
    }
}
