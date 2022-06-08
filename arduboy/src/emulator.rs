// use crunchy::unroll;

use super::mcu::{Core, GPIOPort};
// use super::opcodes::Op;
use super::utils::{decode_hex_line, HexFileError, KeysState};
use core::mem;
use core::str;

use serde::{self, Deserialize, Serialize};

#[cfg(not(feature = "std"))]
use alloc::vec;
#[cfg(not(feature = "std"))]
use alloc::vec::Vec;

#[cfg_attr(feature = "std", derive(Debug))]
pub enum Error {
    HexFile(HexFileError),
    Utf8(str::Utf8Error),
}

impl From<HexFileError> for Error {
    fn from(err: HexFileError) -> Self {
        Self::HexFile(err)
    }
}

impl From<str::Utf8Error> for Error {
    fn from(err: str::Utf8Error) -> Self {
        Self::Utf8(err)
    }
}

#[cfg_attr(test, derive(core::cmp::PartialEq, core::fmt::Debug))]
#[derive(Serialize, Deserialize, Clone)]
pub struct Emulator {
    pub core: Core,
    cpu_freq: isize,
    cycles: isize,
    pub samples: Vec<(i16, i16)>,
}

pub const AUDIO_SAMPLE_FREQ: isize = 44100;
pub const FPS: isize = 60;
pub const FRAME_SAMPLES: isize = AUDIO_SAMPLE_FREQ / FPS;

impl Emulator {
    pub fn new() -> Self {
        Self {
            core: Core::new(),
            cpu_freq: 16_000_000,
            cycles: 0,
            samples: vec![(0, 0); FRAME_SAMPLES as usize],
        }
    }

    pub fn load_game(&mut self, data: &[u8]) -> Result<(), Error> {
        let data = str::from_utf8(data)?;
        let mut out = [0u8; 32];
        for line in data.lines() {
            if line.is_empty() {
                continue;
            }
            match decode_hex_line(line, &mut out[..])? {
                Some((addr, len)) => {
                    for (i, byte) in out.iter().enumerate().take(len) {
                        self.core.flash_write(addr + i as u16, *byte);
                    }
                }
                None => {}
            }
        }
        self.core.reset();
        Ok(())
    }

    pub fn serialize_len(&self) -> postcard::Result<usize> {
        const BUF_LEN: usize = 0x10000;
        let mut buf = vec![0; BUF_LEN];
        let bin = postcard::to_slice(&self, &mut buf)?;
        Ok(bin.len())
    }

    pub fn serialize(&self, bin: &mut [u8]) -> postcard::Result<()> {
        postcard::to_slice(&self, bin)?;
        Ok(())
    }

    pub fn deserialize(&mut self, bin: &[u8]) -> postcard::Result<()> {
        self.core.deserialize_pre();
        let mut new: Emulator = postcard::from_bytes(bin)?;
        mem::swap(&mut new.core.program, &mut self.core.program);
        mem::swap(&mut new.core.program_ops, &mut self.core.program_ops);
        *self = new;
        self.core.deserialize_post();
        Ok(())
    }

    pub fn run(&mut self, keys_state: &KeysState) {
        let cycles_per_sample = self.cpu_freq / AUDIO_SAMPLE_FREQ;
        for s in self.samples.iter_mut() {
            *s = (0, 0);
        }
        let mut sample_cycles = cycles_per_sample;
        self.cycles += self.cpu_freq / FPS;

        let (port_b, port_e, port_f) = keys_state.to_gpio();
        self.core.gpio.set_port(GPIOPort::B, port_b);
        self.core.gpio.set_port(GPIOPort::E, port_e);
        self.core.gpio.set_port(GPIOPort::F, port_f);

        while self.cycles > 0 {
            // In each iteration, emulate N instructions of the CPU, and the emulate the
            // corresponding cycles in the hardware
            const N_INSTS: usize = 64;
            let mut hw_step_cycles = 0;
            if !self.core.sleeping() {
                hw_step_cycles += self.core.step_n(N_INSTS);
            } else {
                hw_step_cycles += N_INSTS * 2;
            }
            self.core.step_hw(hw_step_cycles);

            sample_cycles -= hw_step_cycles as isize;
            if sample_cycles < 0 {
                let v = if (self.core.gpio.port_c() & (1 << 6)) != 0 {
                    core::i16::MAX
                } else {
                    0
                };
                self.samples[core::cmp::max(0, FRAME_SAMPLES - 1 - self.cycles / cycles_per_sample)
                    as usize] = (v, v);
                sample_cycles += cycles_per_sample;
            }

            self.cycles -= hw_step_cycles as isize;
        }
        self.core.display.render();
    }
}

#[cfg(feature = "std")]
#[cfg(test)]
mod test;
