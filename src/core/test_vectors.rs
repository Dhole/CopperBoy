use super::*;
use hex;
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
        write!(f, "{{ a0: {:02x}, b: {:02x}, a1: {:02x}, sreg: {:08b} }}",
            self.a0, self.b, self.a1, self.sreg)
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
    let vectors: Vec<Vec<Vec<u8>>> = lines.iter()
         .map(|line| line.split(" ").map(|v| hex::decode(v).unwrap()).collect())
         .collect();
    vectors
}

fn read_vec_op_alu0(op: &str) -> Vec<OpAlu0> {
    let vectors = read_lines(&op);
    let vectors: Vec<OpAlu0> = vectors.iter()
        .map(|line| OpAlu0 {
            a0: line[0][0],
            b: line[1][0],
            a1: line[2][0],
            sreg: line[3][0],
        })
        .collect();
    vectors
}

fn test_op_alu0<F>(vector: OpAlu0, op: &str, mut fn_op: F)
where
F: FnMut(&mut Core) {
    let mut core = Core::new();

    core.regs[0] = vector.a0;
    core.regs[1] = vector.b;
    fn_op(&mut core);
    let res = OpAlu0 {a1: core.regs[0], sreg: core.status_reg.as_u8() , ..vector};
    let calc = OpAlu0 {sreg: res.sreg & 0b0111_1111, ..res};
    let expe = OpAlu0 {sreg: vector.sreg & 0b0111_1111, ..vector};
    assert_eq!((op, calc), (op, expe));
}

#[test]
fn test_vec_op_add() {
    for vector in read_vec_op_alu0("ADD") {
        test_op_alu0(vector, "ADD", |core| {core.op_add(0, 1);});
    }
}

#[test]
fn test_vec_op_and() {
    for vector in read_vec_op_alu0("AND") {
        test_op_alu0(vector, "AND", |core| {core.op_and(0, 1);});
    }
}
