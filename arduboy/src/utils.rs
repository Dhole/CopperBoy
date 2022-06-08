use super::keys::*;
#[cfg(feature = "std")]
use super::mcu::Core;
use hex;
#[cfg(feature = "std")]
use std::fs;
#[cfg(feature = "std")]
use std::io::{self, BufRead};

#[cfg_attr(feature = "std", derive(Debug))]
pub enum HexFileError {
    InvalidPrefix,
    InvalidLength,
    OutBufferTooSmall,
    HexDecode(hex::FromHexError),
    #[cfg(feature = "std")]
    Io(io::Error),
}

impl From<hex::FromHexError> for HexFileError {
    fn from(err: hex::FromHexError) -> Self {
        Self::HexDecode(err)
    }
}

#[cfg(feature = "std")]
impl From<io::Error> for HexFileError {
    fn from(err: io::Error) -> Self {
        Self::Io(err)
    }
}

// Returns the address and the number of bytes written into `out`.
pub fn decode_hex_line(line: &str, out: &mut [u8]) -> Result<Option<(u16, usize)>, HexFileError> {
    if !line.starts_with(':') {
        return Err(HexFileError::InvalidPrefix);
    }
    let line = line.as_bytes();
    let line = &line[1..];
    if line.len() < 8 {
        return Err(HexFileError::InvalidLength);
    }
    let mut buf = [0u8; 32];
    let buf = &mut buf[..line.len() / 2];
    hex::decode_to_slice(&line, &mut buf[..line.len() / 2])?;
    let len = buf[0] as usize;
    let addr = u16::from_be_bytes([buf[1], buf[2]]);
    let rtype = buf[3];

    Ok(match rtype {
        0x00 => {
            if buf.len() != 4 + len + 1 {
                return Err(HexFileError::InvalidLength);
            }
            let data = &buf[4..4 + len];
            // TODO: Verify checksum
            let _checksum = buf[4 + len];
            if data.len() > out.len() {
                return Err(HexFileError::OutBufferTooSmall);
            }
            out[..data.len()].copy_from_slice(data);
            Some((addr, data.len()))
        }
        // TODO: Handle other type cases
        _ => None,
    })
}

#[cfg(feature = "std")]
pub fn load_hex_file(core: &mut Core, path: &str) -> Result<(), HexFileError> {
    let file = fs::File::open(path)?;
    let mut out = [0u8; 32];
    for line in io::BufReader::new(file).lines() {
        let line = line?;
        if line.is_empty() {
            continue;
        }
        match decode_hex_line(line.as_str(), &mut out[..])? {
            Some((addr, len)) => {
                for (i, byte) in out.iter().enumerate().take(len) {
                    core.flash_write(addr + i as u16, *byte);
                }
            }
            None => {}
        }
    }
    Ok(())
}

#[cfg(feature = "std")]
use serde::{self, Deserialize, Serialize};

#[cfg(feature = "std")]
#[derive(Serialize, Deserialize)]
pub enum Key {
    #[serde(rename = "l")]
    Left,
    #[serde(rename = "r")]
    Right,
    #[serde(rename = "u")]
    Up,
    #[serde(rename = "d")]
    Down,
    #[serde(rename = "a")]
    A,
    #[serde(rename = "b")]
    B,
}

#[cfg(feature = "std")]
#[derive(Serialize, Deserialize)]
pub struct KeyEvent {
    #[serde(rename = "f")]
    pub frame: usize,
    #[serde(rename = "d")]
    pub down: Vec<Key>,
    #[serde(rename = "u")]
    pub up: Vec<Key>,
}

// true indicates key down, false key up
#[derive(Default)]
pub struct KeysState {
    pub left: bool,
    pub right: bool,
    pub up: bool,
    pub down: bool,
    pub a: bool,
    pub b: bool,
}

impl KeysState {
    // returns (port_b, port_e, port_f)
    pub fn to_gpio(&self) -> (u8, u8, u8) {
        let mut port_b = 0xff_u8;
        let mut port_e = 0xff_u8;
        let mut port_f = 0xff_u8;
        if self.left {
            port_f &= !PIN_LEFT;
        }
        if self.right {
            port_f &= !PIN_RIGHT;
        }
        if self.up {
            port_f &= !PIN_UP;
        }
        if self.down {
            port_f &= !PIN_DOWN;
        }
        if self.a {
            port_e &= !PIN_A;
        }
        if self.b {
            port_b &= !PIN_B;
        }
        (port_b, port_e, port_f)
    }
}

#[cfg(feature = "std")]
pub fn replay_keys_state(
    frame: usize,
    replay_index: usize,
    replay: &Vec<KeyEvent>,
    keys_state: &mut KeysState,
) -> usize {
    let mut replay_index = replay_index;
    if replay_index < replay.len() && replay[replay_index].frame == frame {
        let key_event = &replay[replay_index];
        for down in &key_event.down {
            match down {
                Key::Left => keys_state.left = true,
                Key::Right => keys_state.right = true,
                Key::Up => keys_state.up = true,
                Key::Down => keys_state.down = true,
                Key::A => keys_state.a = true,
                Key::B => keys_state.b = true,
            }
        }
        for up in &key_event.up {
            match up {
                Key::Left => keys_state.left = false,
                Key::Right => keys_state.right = false,
                Key::Up => keys_state.up = false,
                Key::Down => keys_state.down = false,
                Key::A => keys_state.a = false,
                Key::B => keys_state.b = false,
            }
        }
        replay_index += 1;
    }
    replay_index
}
