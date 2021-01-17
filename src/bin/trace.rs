#[allow(unused_assignments)]
use std::env;
use std::fs;
use std::io::{self, BufRead};

use hex;

use avremu::core::Core;
use avremu::opcodes::{Op, OpAddr};
use avremu::utils::decode_hex_line;

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
        match decode_hex_line(line.as_str())
            .map_err(|e| io::Error::new(io::ErrorKind::Other, std::format!("{:?} ", e)))?
        {
            Some((addr, data)) => {
                for i in 0..data.len() {
                    core.flash_write(addr + i as u16, data[i]);
                }
            }
            None => {}
        }
    }
    core.reset();
    // println!("addr ( op_hex  ) (                op_bin                 ) [ pc ]: op");
    println!("addr ( op_hex  ) [ pc ]: op");
    loop {
        if !core.sleep {
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

            // if op_addr.addr == 0x0c90 {
            //     println!(
            //         ">>> X: 0x{:04x}, [X]: 0x{:02x}, R24: 0x{:02x}",
            //         core.regs.x(),
            //         core.program[core.regs.x() as usize],
            //         core.regs[24]
            //     );
            // }
            // if op_addr.addr == 0x0c92 {
            //     println!(
            //         ">>> X: 0x{:04x}, [X]: 0x{:02x}, R25: 0x{:02x}",
            //         core.regs.x(),
            //         core.program[core.regs.x() as usize],
            //         core.regs[25]
            //     );
            // }
        }
        let cycles = core.step();
        core.step_hw(cycles);
    }
}
