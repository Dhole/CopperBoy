/*
#[allow(unused_assignments)]
use std::env;
use std::fs;
use std::io::{self, BufRead};

// use hex;

use arduboy::opcodes::{Op, OpAddr};
use arduboy::utils::decode_hex_line;

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
        match decode_hex_line(line.as_str())
            .map_err(|e| io::Error::new(io::ErrorKind::Other, std::format!("{:?} ", e)))?
        {
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
            OpAddr { op: op, addr }
        );
    }
    Ok(())
}
*/

fn main() {}
