use num_traits::FromPrimitive;

#[derive(FromPrimitive, Clone, Copy)]
enum PllInputPrescaler {
    Mhz8 = 0b0000_0000,
    Mhz16 = 0b0001_0000,
}

pub struct Clock {
    pll_input_prescaler: PllInputPrescaler,
    pll_enable: bool,
    pll_lock_detector: bool,
}

impl Clock {
    pub fn new() -> Self {
        Self {
            pll_input_prescaler: PllInputPrescaler::Mhz8,
            pll_enable: false,
            pll_lock_detector: false,
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
}
