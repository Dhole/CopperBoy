#[allow(unused_assignments)]
use std::env;
use std::fs;
use std::io::{self, BufRead};

use hex;

use avremu::core::Core;
use avremu::opcodes::{Op, OpAddr};

fn decode_hex_line(line: &str) -> Result<Option<(u16, Vec<u8>)>, hex::FromHexError> {
    let line = line.as_bytes();
    assert_eq!(line[0], b':');
    let line = &line[1..];
    let bytes = hex::decode(&line[0..2])?[0] as usize;
    let addr = hex::decode(&line[2..6])?;
    let addr = u16::from_be_bytes([addr[0], addr[1]]);
    let rtype = hex::decode(&line[6..8])?[0];

    Ok(match rtype {
        0x00 => {
            let data = hex::decode(&line[8..8 + bytes * 2])?;
            let _checksum = hex::decode(&line[8 + bytes * 2..8 + bytes * 2 + 2])?[0];
            Some((addr, data))
        }
        _ => None,
    })
}

fn main() -> Result<(), io::Error> {
    env_logger::init();
    let filename = env::args().nth(1).unwrap();
    eprintln!("loading file {}", filename);
    let file = fs::File::open(filename)?;

    let mut core = Core::new();
    for line in io::BufReader::new(file).lines() {
        let line = line?;
        if line.len() == 0 {
            continue;
        }
        match decode_hex_line(line.as_str()).map_err(|e| io::Error::new(io::ErrorKind::Other, e))? {
            Some((addr, data)) => {
                for i in 0..data.len() {
                    core.flash(addr + i as u16, data[i]);
                }
            }
            None => {}
        }
    }
    core.reset();
    loop {
        let (w0, _w1, op_addr) = core.next_op();
        println!(
            "{:04x} {:04x} [{:04x}]: {}",
            op_addr.addr,
            w0,
            op_addr.addr >> 1,
            op_addr,
        );
        core.step();
    }
}
