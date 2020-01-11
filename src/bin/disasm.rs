#[allow(unused_assignments)]
use std::env;
use std::fs;
use std::io::{self, BufRead};

use hex;

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
    let filename = env::args().nth(1).unwrap();
    eprintln!("loading file {}", filename);
    let file = fs::File::open(filename)?;
    let mut program = vec![0; 0x8000];
    for line in io::BufReader::new(file).lines() {
        let line = line?;
        if line.len() == 0 {
            continue;
        }
        match decode_hex_line(line.as_str()).map_err(|e| io::Error::new(io::ErrorKind::Other, e))? {
            Some((addr, data)) => {
                program[addr as usize..addr as usize + data.len()].copy_from_slice(&data);
            }
            None => {}
        }
    }
    // for b in &program {
    //     println!("{:02x}", b);
    // }
    let mut w1 = u16::from_le_bytes([program[0], program[1]]);
    let mut i = 2;
    loop {
        if i + 2 > program.len() {
            break;
        }
        let w0 = w1;
        w1 = u16::from_le_bytes([program[i], program[i + 1]]);
        let op = Op::decode(w0, w1);
        let addr = i as u16 - 2;
        let mut opcode = String::new();
        match op.words() {
            1 => {
                opcode = hex::encode([program[i - 2], program[i - 1]]);
                i += 2;
            }
            2 => {
                w1 = u16::from_le_bytes([program[i + 2], program[i + 3]]);
                opcode = format!(
                    "{} {}",
                    hex::encode([program[i - 2], program[i - 1]]),
                    hex::encode([program[i], program[i + 1]])
                );
                i += 4;
            }
            _ => unreachable!(),
        }
        println!(
            "{:04x} [{:04x}]: {:09}  {}",
            addr,
            addr >> 1,
            opcode,
            OpAddr { op: &op, addr }
        );
    }
    Ok(())
}
