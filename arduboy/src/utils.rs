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
    if !line.starts_with(":") {
        return Err(HexFileError::InvalidPrefix);
    }
    let line = line.as_bytes();
    let line = &line[1..];
    if line.len() < 8 {
        return Err(HexFileError::InvalidLength);
    }
    let mut buf = [0u8; 32];
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
        if line.len() == 0 {
            continue;
        }
        match decode_hex_line(line.as_str(), &mut out[..])? {
            Some((addr, len)) => {
                for i in 0..len {
                    core.flash_write(addr + i as u16, out[i]);
                }
            }
            None => {}
        }
    }
    Ok(())
}
