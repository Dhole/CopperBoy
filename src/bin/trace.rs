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

fn bin(w: u16) -> String {
    let s = format!("{:016b}", w);
    format!("{}_{}_{}_{}", &s[0..4], &s[4..8], &s[8..12], &s[12..16])
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
    // println!("addr ( op_hex  ) (                op_bin                 ) [ pc ]: op");
    println!("addr ( op_hex  ) [ pc ]: op");
    loop {
        let (w0, w1, op_addr) = core.next_op();
        print!("{:04x} ", op_addr.addr);
        match op_addr.op.words() {
            1 => print!("({}     ) ", hex::encode(w0.to_le_bytes())),
            2 => print!(
                "({} {}) ",
                hex::encode(w0.to_le_bytes()),
                hex::encode(w1.to_le_bytes())
            ),
            _ => unreachable!(),
        }
        // match op_addr.op.words() {
        //     1 => print!("({}                    ) ", bin(w0)),
        //     2 => print!("({} {}) ", bin(w0), bin(w1)),
        //     _ => unreachable!(),
        // }
        if let Some(op_addr_alt) = op_addr.alt() {
            println!("[{:04x}]: {}; {}", op_addr.addr >> 1, op_addr_alt, op_addr,);
        } else {
            println!("[{:04x}]: {}", op_addr.addr >> 1, op_addr,);
        }
        core.step();
    }
}
