use super::core::{Core, GPIOPort};
use super::utils::{decode_hex_line, HexFileError};
use std::str;

#[derive(Debug)]
pub enum Error {
    SDL2(String),
    HexFile(HexFileError),
    Utf8(str::Utf8Error),
}

impl From<String> for Error {
    fn from(err: String) -> Self {
        Self::SDL2(err)
    }
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

// type CbAudioSampleBatch = fn(&[(i16, i16)]);
// type CbVideoRefresh = fn(&[u16], u32, u32, usize);

pub struct Emulator {
    pub core: Core,
    cpu_freq: isize,
    cycles: isize,
    pub samples: [(i16, i16); FRAME_SAMPLES as usize],
    // cb_audio_sample_batch: CbAudioSampleBatch,
    // cb_video_refresh: CbVideoRefresh,
}

pub const AUDIO_SAMPLE_FREQ: isize = 44100;
pub const FPS: isize = 60;
pub const FRAME_SAMPLES: isize = AUDIO_SAMPLE_FREQ / FPS;

impl Emulator {
    pub fn new(// cb_audio_sample_batch: CbAudioSampleBatch,
        // cb_video_refresh: CbVideoRefresh,
    ) -> Self {
        Self {
            core: Core::new(),
            cpu_freq: 16_000_000,
            cycles: 0,
            samples: [(0, 0); FRAME_SAMPLES as usize],
            // cb_audio_sample_batch,
            // cb_video_refresh,
        }
    }

    pub fn load_game(&mut self, data: &[u8]) -> Result<(), Error> {
        let data = str::from_utf8(data)?;
        for line in data.lines() {
            if line.len() == 0 {
                continue;
            }
            match decode_hex_line(line)? {
                Some((addr, data)) => {
                    for i in 0..data.len() {
                        self.core.flash_write(addr + i as u16, data[i]);
                    }
                }
                None => {}
            }
        }
        self.core.reset();
        Ok(())
    }

    pub fn run(&mut self, port_b: u8, port_e: u8, port_f: u8) {
        let cycles_per_sample = self.cpu_freq / AUDIO_SAMPLE_FREQ;
        for s in self.samples.iter_mut() {
            *s = (0, 0);
        }
        let mut sample_cycles = cycles_per_sample;
        self.cycles += self.cpu_freq / FPS;

        self.core.gpio.set_port(GPIOPort::B, port_b);
        self.core.gpio.set_port(GPIOPort::E, port_e);
        self.core.gpio.set_port(GPIOPort::F, port_f);

        while self.cycles > 0 {
            // In each iteration, emulate M * N instructions of the CPU, and the emulate the
            // corresponding cycles in the hardware
            const N_INSTS: usize = 4;
            const M_ITERS: usize = 1;
            let mut hw_step_cycles = 0;
            if !self.core.sleep {
                for _ in 0..M_ITERS {
                    hw_step_cycles += self.core.step();
                    hw_step_cycles += self.core.step();
                    hw_step_cycles += self.core.step();
                    hw_step_cycles += self.core.step();
                }
            } else {
                hw_step_cycles += M_ITERS * N_INSTS;
            }
            self.core.step_hw(hw_step_cycles);

            sample_cycles -= hw_step_cycles as isize;
            if sample_cycles < 0 {
                let v = if (self.core.gpio.port_c() & (1 << 6)) != 0 {
                    std::i16::MAX
                } else {
                    0
                };
                self.samples[std::cmp::max(0, FRAME_SAMPLES - 1 - self.cycles / cycles_per_sample)
                    as usize] = (v, v);
                sample_cycles += cycles_per_sample;
            }

            self.cycles -= hw_step_cycles as isize;
        }
        self.core.display.render();
    }
}
