use super::*;
use hex;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

const PATH_VECTORS: &'static str = "test-framework/vectors";

#[derive(PartialEq)]
struct OpAlu0 {
    sreg0: u8,
    a0: u8,
    b: u8,
    a1: u8,
    sreg1: u8,
}

impl fmt::Debug for OpAlu0 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{ sreg0: {:02x}, a0: {:02x}, b: {:02x}, a1: {:02x}, sreg1: {:08b} }}",
            self.sreg0, self.a0, self.b, self.a1, self.sreg1
        )
    }
}

#[derive(PartialEq)]
struct OpAlu1 {
    sreg0: u8,
    a0: u8,
    a1: u8,
    sreg1: u8,
}

impl fmt::Debug for OpAlu1 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{ sreg0: {:02x}, a0: {:02x}, a1: {:02x}, sreg1: {:08b} }}",
            self.sreg0, self.a0, self.a1, self.sreg1
        )
    }
}

#[derive(PartialEq)]
struct OpAlu2 {
    sreg0: u8,
    a0: u8,
    k: u8,
    a1: u8,
    sreg1: u8,
}

impl fmt::Debug for OpAlu2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{ sreg0: {:02x}, a0: {:02x}, k: {:02x}, a1: {:02x}, sreg1: {:08b} }}",
            self.sreg0, self.a0, self.k, self.a1, self.sreg1
        )
    }
}

#[derive(PartialEq)]
struct OpAlu3 {
    a0: u8,
    b0: u8,
    k: u8,
    a1: u8,
    b1: u8,
    sreg1: u8,
}

impl fmt::Debug for OpAlu3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{ a0: {:02x}, b0:  {:02x}, k: {:02x}, a1: {:02x}, b1:  {:02x}, sreg1: {:08b} }}",
            self.a0, self.b0, self.k, self.a1, self.b1, self.sreg1
        )
    }
}

#[derive(PartialEq)]
struct OpAlu4 {
    a0: u8,
    b0: u8,
    a1: u8,
    b1: u8,
    sreg1: u8,
}

impl fmt::Debug for OpAlu4 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{ a0: {:02x}, b0:  {:02x}, a1: {:02x}, b1:  {:02x}, sreg1: {:08b} }}",
            self.a0, self.b0, self.a1, self.b1, self.sreg1
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
            sreg0: line[0][0],
            a0: line[1][0],
            b: line[2][0],
            a1: line[3][0],
            sreg1: line[4][0],
        })
        .collect();
    vectors
}

fn test_op_alu0(mut core: &mut Core, vector: OpAlu0, op: &str, fn_op: &dyn Fn(&mut Core)) {
    core.pc = 0;
    core.status_reg = StatusRegister::from_u8(vector.sreg0);
    core.regs[0] = vector.a0;
    core.regs[1] = vector.b;
    fn_op(&mut core);
    let res = OpAlu0 {
        a1: core.regs[0],
        sreg1: core.status_reg.as_u8(),
        ..vector
    };
    assert_eq!((op, vector), (op, res));
}

fn read_vec_op_alu1(op: &str) -> Vec<OpAlu1> {
    let vectors = read_lines(&op);
    let vectors: Vec<OpAlu1> = vectors
        .iter()
        .map(|line| OpAlu1 {
            sreg0: line[0][0],
            a0: line[1][0],
            a1: line[2][0],
            sreg1: line[3][0],
        })
        .collect();
    vectors
}

fn test_op_alu1(mut core: &mut Core, vector: OpAlu1, op: &str, fn_op: &dyn Fn(&mut Core)) {
    core.pc = 0;
    core.status_reg = StatusRegister::from_u8(vector.sreg0);
    core.regs[0] = vector.a0;
    fn_op(&mut core);
    let res = OpAlu1 {
        a1: core.regs[0],
        sreg1: core.status_reg.as_u8(),
        ..vector
    };
    assert_eq!((op, vector), (op, res));
}

fn read_vec_op_alu2(op: &str) -> Vec<OpAlu2> {
    let vectors = read_lines(&op);
    let vectors: Vec<OpAlu2> = vectors
        .iter()
        .map(|line| OpAlu2 {
            sreg0: line[0][0],
            a0: line[1][0],
            k: line[2][0],
            a1: line[3][0],
            sreg1: line[4][0],
        })
        .collect();
    vectors
}

fn test_op_alu2(mut core: &mut Core, vector: OpAlu2, op: &str, fn_op: &dyn Fn(&mut Core, u8)) {
    core.pc = 0;
    core.status_reg = StatusRegister::from_u8(vector.sreg0);
    core.regs[0] = vector.a0;
    fn_op(&mut core, vector.k);
    let res = OpAlu2 {
        a1: core.regs[0],
        sreg1: core.status_reg.as_u8(),
        ..vector
    };
    assert_eq!((op, vector), (op, res));
}

fn read_vec_op_alu3(op: &str) -> Vec<OpAlu3> {
    let vectors = read_lines(&op);
    let vectors: Vec<OpAlu3> = vectors
        .iter()
        .map(|line| OpAlu3 {
            a0: line[0][0],
            b0: line[1][0],
            k: line[2][0],
            a1: line[3][0],
            b1: line[4][0],
            sreg1: line[5][0],
        })
        .collect();
    vectors
}

fn test_op_alu3(mut core: &mut Core, vector: OpAlu3, op: &str, fn_op: &dyn Fn(&mut Core, u8)) {
    core.pc = 0;
    core.status_reg = StatusRegister::from_u8(0);
    core.regs[0] = vector.a0;
    core.regs[1] = vector.b0;
    fn_op(&mut core, vector.k);
    let res = OpAlu3 {
        a1: core.regs[0],
        b1: core.regs[1],
        sreg1: core.status_reg.as_u8(),
        ..vector
    };
    assert_eq!((op, vector), (op, res));
}

fn read_vec_op_alu4(op: &str) -> Vec<OpAlu4> {
    let vectors = read_lines(&op);
    let vectors: Vec<OpAlu4> = vectors
        .iter()
        .map(|line| OpAlu4 {
            a0: line[0][0],
            b0: line[1][0],
            a1: line[2][0],
            b1: line[3][0],
            sreg1: line[4][0],
        })
        .collect();
    vectors
}

fn test_op_alu4(mut core: &mut Core, vector: OpAlu4, op: &str, fn_op: &dyn Fn(&mut Core)) {
    core.pc = 0;
    core.status_reg = StatusRegister::from_u8(0);
    core.regs[0] = vector.a0;
    core.regs[1] = vector.b0;
    fn_op(&mut core);
    let res = OpAlu4 {
        a1: core.regs[0],
        b1: core.regs[1],
        sreg1: core.status_reg.as_u8(),
        ..vector
    };
    assert_eq!((op, vector), (op, res));
}

#[test]
#[rustfmt::skip]
fn test_vec_ops_alu0() {
    let mut ops: HashMap<&str, Box<dyn Fn(&mut Core)>> = HashMap::new();
    ops.insert("add", Box::new(|c: &mut Core| { c.op_add(0, 1); }));
    ops.insert("and", Box::new(|c: &mut Core| { c.op_and(0, 1); }));
    ops.insert("cp" , Box::new(|c: &mut Core| { c.op_cp( 0, 1); }));
    ops.insert("eor", Box::new(|c: &mut Core| { c.op_eor(0, 1); }));
    ops.insert("mov", Box::new(|c: &mut Core| { c.op_mov(0, 1); }));
    ops.insert("or" , Box::new(|c: &mut Core| { c.op_or( 0, 1); }));
    ops.insert("sub", Box::new(|c: &mut Core| { c.op_sub(0, 1); }));
    ops.insert("adc", Box::new(|c: &mut Core| { c.op_adc(0, 1); }));
    ops.insert("cpc", Box::new(|c: &mut Core| { c.op_cpc(0, 1); }));
    ops.insert("sbc", Box::new(|c: &mut Core| { c.op_sbc(0, 1); }));

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
    ops.insert("com" , Box::new(|c: &mut Core| { c.op_com( 0); }));
    ops.insert("neg" , Box::new(|c: &mut Core| { c.op_neg( 0); }));
    ops.insert("inc" , Box::new(|c: &mut Core| { c.op_inc( 0); }));
    ops.insert("dec" , Box::new(|c: &mut Core| { c.op_dec( 0); }));
    // ops.insert("ser" , Box::new(|c: &mut Core| { c.op_ser( 0); }));
    ops.insert("asr" , Box::new(|c: &mut Core| { c.op_asr( 0); }));
    ops.insert("swap", Box::new(|c: &mut Core| { c.op_swap(0); }));
    ops.insert("ror" , Box::new(|c: &mut Core| { c.op_ror( 0); }));
    ops.insert("lsr" , Box::new(|c: &mut Core| { c.op_lsr( 0); }));

    let mut core = Core::new();
    for (op, f) in &ops {
        for vector in read_vec_op_alu1(op) {
            test_op_alu1(&mut core, vector, op, f.as_ref());
        }
    }
}

#[test]
#[rustfmt::skip]
fn test_vec_ops_alu2() {
    let mut ops: HashMap<&str, Box<dyn Fn(&mut Core, u8)>> = HashMap::new();
    ops.insert("subi", Box::new(|c: &mut Core, k: u8| { c.op_subi(0, k); }));
    ops.insert("andi", Box::new(|c: &mut Core, k: u8| { c.op_andi(0, k); }));
    ops.insert("ori" , Box::new(|c: &mut Core, k: u8| { c.op_ori( 0, k); }));
    ops.insert("cpi" , Box::new(|c: &mut Core, k: u8| { c.op_cpi( 0, k); }));
    ops.insert("ldi" , Box::new(|c: &mut Core, k: u8| { c.op_ldi( 0, k); }));
    ops.insert("sbci", Box::new(|c: &mut Core, k: u8| { c.op_sbci(0, k); }));

    let mut core = Core::new();
    for (op, f) in &ops {
        for vector in read_vec_op_alu2(op) {
            test_op_alu2(&mut core, vector, op, f.as_ref());
        }
    }
}

#[test]
#[rustfmt::skip]
fn test_vec_ops_alu3() {
    let mut ops: HashMap<&str, Box<dyn Fn(&mut Core, u8)>> = HashMap::new();
    ops.insert("adiw", Box::new(|c: &mut Core, k: u8| { c.op_adiw(0, k); }));
    ops.insert("sbiw", Box::new(|c: &mut Core, k: u8| { c.op_sbiw(0, k); }));

    let mut core = Core::new();
    for (op, f) in &ops {
        for vector in read_vec_op_alu3(op) {
            test_op_alu3(&mut core, vector, op, f.as_ref());
        }
    }
}

#[test]
#[rustfmt::skip]
fn test_vec_ops_alu4() {
    let mut ops: HashMap<&str, Box<dyn Fn(&mut Core)>> = HashMap::new();
    ops.insert("mul"   , Box::new(|c: &mut Core| { c.op_mul(   0, 1); }));
    ops.insert("muls"  , Box::new(|c: &mut Core| { c.op_muls(  0, 1); }));
    ops.insert("mulsu" , Box::new(|c: &mut Core| { c.op_mulsu( 0, 1); }));
    ops.insert("fmul"  , Box::new(|c: &mut Core| { c.op_fmul(  0, 1); }));
    ops.insert("fmuls" , Box::new(|c: &mut Core| { c.op_fmuls( 0, 1); }));
    ops.insert("fmulsu", Box::new(|c: &mut Core| { c.op_fmulsu(0, 1); }));

    let mut core = Core::new();
    for (op, f) in &ops {
        for vector in read_vec_op_alu4(op) {
            test_op_alu4(&mut core, vector, op, f.as_ref());
        }
    }
}
