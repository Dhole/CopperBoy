use super::core::Core;
use hex;
use std::fs;
use std::io::{self, BufRead};

#[derive(Debug)]
pub enum HexFileError {
    InvalidPrefix,
    InvalidLength,
    HexDecode(hex::FromHexError),
    Io(io::Error),
}

impl From<hex::FromHexError> for HexFileError {
    fn from(err: hex::FromHexError) -> Self {
        Self::HexDecode(err)
    }
}

impl From<io::Error> for HexFileError {
    fn from(err: io::Error) -> Self {
        Self::Io(err)
    }
}

pub fn decode_hex_line(line: &str) -> Result<Option<(u16, Vec<u8>)>, HexFileError> {
    if !line.starts_with(":") {
        return Err(HexFileError::InvalidPrefix);
    }
    let line = line.as_bytes();
    let line = &line[1..];
    if line.len() < 8 {
        return Err(HexFileError::InvalidLength);
    }
    let buf = hex::decode(&line)?;
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
            Some((addr, data.to_vec()))
        }
        // TODO: Handle other type cases
        _ => None,
    })
}

pub fn load_hex_file(core: &mut Core, path: &str) -> Result<(), HexFileError> {
    let file = fs::File::open(path)?;
    for line in io::BufReader::new(file).lines() {
        let line = line?;
        if line.len() == 0 {
            continue;
        }
        match decode_hex_line(line.as_str())? {
            Some((addr, data)) => {
                for i in 0..data.len() {
                    core.flash_write(addr + i as u16, data[i]);
                }
            }
            None => {}
        }
    }
    Ok(())
}
