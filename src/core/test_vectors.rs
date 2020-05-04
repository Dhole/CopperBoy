use super::*;
use hex;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

const PATH_VECTORS: &'static str = "test-framework/vectors";

#[derive(PartialEq)]
struct OpAlu0 {
    a0: u8,
    b: u8,
    a1: u8,
    sreg: u8,
}

impl fmt::Debug for OpAlu0 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{ a0: {:02x}, b: {:02x}, a1: {:02x}, sreg: {:08b} }}",
            self.a0, self.b, self.a1, self.sreg
        )
    }
}

#[derive(PartialEq)]
struct OpAlu1 {
    sreg0: u8,
    a0: u8,
    b: u8,
    a1: u8,
    sreg1: u8,
}

impl fmt::Debug for OpAlu1 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{ sreg0: {:02x}, a0: {:02x}, b: {:02x}, a1: {:02x}, sreg1: {:08b} }}",
            self.sreg0, self.a0, self.b, self.a1, self.sreg1
        )
    }
}

fn read_lines(op: &str) -> Vec<Vec<Vec<u8>>> {
    let mut file = File::open(format!("{}/{}", PATH_VECTORS, op)).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let lines: Vec<String> = contents
        .split("\n")
        .filter(|line| line.len() > 0 && !line.starts_with("#"))
        .map(|line| line.to_string())
        .collect();
    let vectors: Vec<Vec<Vec<u8>>> = lines
        .iter()
        .map(|line| line.split(" ").map(|v| hex::decode(v).unwrap()).collect())
        .collect();
    vectors
}

fn read_vec_op_alu0(op: &str) -> Vec<OpAlu0> {
    let vectors = read_lines(&op);
    let vectors: Vec<OpAlu0> = vectors
        .iter()
        .map(|line| OpAlu0 {
            a0: line[0][0],
            b: line[1][0],
            a1: line[2][0],
            sreg: line[3][0],
        })
        .collect();
    vectors
}

fn test_op_alu0(mut core: &mut Core, vector: OpAlu0, op: &str, fn_op: &dyn Fn(&mut Core)) {
    core.pc = 0;
    core.status_reg = StatusRegister::from_u8(0);
    core.regs[0] = vector.a0;
    core.regs[1] = vector.b;
    fn_op(&mut core);
    let res = OpAlu0 {
        a1: core.regs[0],
        sreg: core.status_reg.as_u8(),
        ..vector
    };
    let calc = OpAlu0 {
        sreg: res.sreg & 0b0111_1111,
        ..res
    };
    let expe = OpAlu0 {
        sreg: vector.sreg & 0b0111_1111,
        ..vector
    };
    assert_eq!((op, expe), (op, calc));
}

fn read_vec_op_alu1(op: &str) -> Vec<OpAlu1> {
    let vectors = read_lines(&op);
    let vectors: Vec<OpAlu1> = vectors
        .iter()
        .map(|line| OpAlu1 {
            sreg0: line[0][0],
            a0: line[1][0],
            b: line[2][0],
            a1: line[3][0],
            sreg1: line[4][0],
        })
        .collect();
    vectors
}

fn test_op_alu1(mut core: &mut Core, vector: OpAlu1, op: &str, fn_op: &dyn Fn(&mut Core)) {
    core.pc = 0;
    core.status_reg = StatusRegister::from_u8(vector.sreg0);
    core.regs[0] = vector.a0;
    core.regs[1] = vector.b;
    fn_op(&mut core);
    let res = OpAlu1 {
        a1: core.regs[0],
        sreg1: core.status_reg.as_u8(),
        ..vector
    };
    let calc = OpAlu1 {
        sreg1: res.sreg1 & 0b0111_1111,
        ..res
    };
    let expe = OpAlu1 {
        sreg1: vector.sreg1 & 0b0111_1111,
        ..vector
    };
    assert_eq!((op, expe), (op, calc));
}

// #[test]
// fn test_vec_op_add() {
//     for vector in read_vec_op_alu0("ADD") {
//         test_op_alu0(vector, "ADD", |core| {
//             core.op_add(0, 1);
//         });
//     }
// }

// #[test]
// fn test_vec_op_and() {
//     for vector in read_vec_op_alu0("AND") {
//         test_op_alu0(vector, "AND", |core| {
//             core.op_and(0, 1);
//         });
//     }
// }

#[test]
#[rustfmt::skip]
fn test_vec_ops_alu0() {
    let mut ops: HashMap<&str, Box<dyn Fn(&mut Core)>> = HashMap::new();
    ops.insert("ADD", Box::new(|c: &mut Core| { c.op_add(0, 1); }));
    ops.insert("AND", Box::new(|c: &mut Core| { c.op_and(0, 1); }));
    ops.insert("CP",  Box::new(|c: &mut Core| {  c.op_cp(0, 1); }));
    ops.insert("EOR", Box::new(|c: &mut Core| { c.op_eor(0, 1); }));
    ops.insert("MOV", Box::new(|c: &mut Core| { c.op_mov(0, 1); }));
    ops.insert("OR",  Box::new(|c: &mut Core| {  c.op_or(0, 1); }));
    ops.insert("SUB", Box::new(|c: &mut Core| { c.op_sub(0, 1); }));

    let mut core = Core::new();
    for (op, f) in &ops {
        for vector in read_vec_op_alu0(op) {
            test_op_alu0(&mut core, vector, op, f.as_ref());
        }
    }
}

#[test]
#[rustfmt::skip]
fn test_vec_ops_alu1() {
    let mut ops: HashMap<&str, Box<dyn Fn(&mut Core)>> = HashMap::new();
    ops.insert("ADC", Box::new(|c: &mut Core| { c.op_adc(0, 1); }));
    ops.insert("CPC", Box::new(|c: &mut Core| { c.op_cpc(0, 1); }));
    ops.insert("SBC", Box::new(|c: &mut Core| { c.op_sbc(0, 1); }));

    let mut core = Core::new();
    for (op, f) in &ops {
        for vector in read_vec_op_alu1(op) {
            test_op_alu1(&mut core, vector, op, f.as_ref());
        }
    }
}
