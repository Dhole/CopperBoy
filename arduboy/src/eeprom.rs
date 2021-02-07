use serde::{self, Deserialize, Serialize};

pub const EERE: u8 = 1 << 0;
pub const EEPE: u8 = 1 << 1;
pub const EEMPE: u8 = 1 << 2;
pub const EERIE: u8 = 1 << 3;
pub const EEPM0: u8 = 1 << 4;
pub const EEPM1: u8 = 1 << 5;

pub const EEPROM_SIZE: u16 = 0x400;

#[derive(Serialize, Deserialize)]
pub struct Eeprom {
    rom: Vec<u8>,
    addr: u16,
    data: u8,
}

impl Eeprom {
    pub fn new() -> Self {
        Self {
            rom: vec![0xff; EEPROM_SIZE as usize],
            addr: 0,
            data: 0,
        }
    }

    pub fn reg_eearh(&self) -> u8 {
        ((self.addr & 0xff00) >> 8) as u8
    }

    pub fn reg_eearl(&self) -> u8 {
        (self.addr & 0x00ff) as u8
    }

    pub fn set_reg_eearh(&mut self, v: u8) {
        self.addr &= 0x00ff;
        self.addr |= (v as u16) << 8;
    }

    pub fn set_reg_eearl(&mut self, v: u8) {
        self.addr &= 0x0f00;
        self.addr |= (v & 0x0f) as u16;
    }

    pub fn reg_eedr(&self) -> u8 {
        self.data
    }

    pub fn set_reg_eedr(&mut self, v: u8) {
        self.data = v;
    }

    pub fn reg_eecr(&self) -> u8 {
        0
    }

    pub fn set_reg_eecr(&mut self, v: u8) {
        if (v & EERE) != 0 {
            self.data = self.rom[(self.addr % EEPROM_SIZE) as usize];
        }
    }
}
