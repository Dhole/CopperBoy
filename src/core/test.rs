use super::*;

const SRAM_END: u16 = SRAM_ADDR + SRAM_SIZE;

fn new_status_reg_true(list: &[char]) -> StatusRegister {
    let mut status_reg = StatusRegister::new();
    for c in list {
        match c {
            'i' => status_reg.i = true,
            't' => status_reg.t = true,
            'h' => status_reg.h = true,
            's' => status_reg.s = true,
            'v' => status_reg.v = true,
            'n' => status_reg.n = true,
            'z' => status_reg.z = true,
            'c' => status_reg.c = true,
            _ => panic!("Bad status register bit"),
        }
    }
    status_reg
}

macro_rules! assert_status_reg_true {
    ($status_reg:expr, $list:expr) => {
        assert_eq!(*$status_reg, new_status_reg_true($list));
    };
}

#[test]
fn test_op_add() {
    let mut core = Core::new();

    core.regs[0] = 0x01;
    core.regs[1] = 0x02;
    core.op_add(0, 1);
    assert_eq!(core.pc, 0x01);
    assert_eq!(core.regs[0], 0x03);
    assert_status_reg_true!(&core.status_reg, &[]);

    core.regs[0] = 0xff;
    core.regs[1] = 0x01;
    core.op_add(0, 1);
    assert_eq!(core.regs[0], 0x00);
    assert_status_reg_true!(&core.status_reg, &['z', 'c', 'h']);

    core.regs[0] = 0xe0;
    core.regs[1] = 0x40;
    core.op_add(0, 1);
    assert_eq!(core.regs[0], 0x20);
    assert_status_reg_true!(&core.status_reg, &['c']);

    core.regs[0] = 0x0a;
    core.regs[1] = 0x06;
    core.op_add(0, 1);
    assert_eq!(core.regs[0], 0x10);
    assert_status_reg_true!(&core.status_reg, &['h']);

    core.regs[0] = 0xe0;
    core.regs[1] = 0x11;
    core.op_add(0, 1);
    assert_eq!(core.regs[0], 0xf1);
    assert_status_reg_true!(&core.status_reg, &['s', 'n']);

    core.regs[0] = 0x80;
    core.regs[1] = 0x81;
    core.op_add(0, 1);
    assert_eq!(core.regs[0], 0x01);
    assert_status_reg_true!(&core.status_reg, &['c', 'v', 's']);
}

#[test]
fn test_op_adc() {
    let mut core = Core::new();

    core.status_reg.c = true;
    core.regs[0] = 0x02;
    core.regs[1] = 0x04;
    core.op_adc(0, 1);
    assert_eq!(core.pc, 0x01);
    assert_eq!(core.regs[0], 0x07);
    assert_status_reg_true!(&core.status_reg, &[]);

    core.status_reg.c = true;
    core.regs[0] = 0xfd;
    core.regs[1] = 0x02;
    core.op_adc(0, 1);
    assert_eq!(core.regs[0], 0x00);
    assert_status_reg_true!(&core.status_reg, &['z', 'c', 'h']);

    core.status_reg.c = false;
    core.regs[0] = 0xfd;
    core.regs[1] = 0x02;
    core.op_adc(0, 1);
    assert_eq!(core.regs[0], 0xff);
    assert_status_reg_true!(&core.status_reg, &['s', 'n']);
}

#[test]
fn test_op_adiw() {
    let mut core = Core::new();

    core.regs.set_ext(24, 0x23e5);
    core.op_adiw(24, 0x21);
    assert_eq!(core.pc, 0x01);
    assert_eq!(core.regs.ext(24), 0x2406);
    assert_status_reg_true!(&core.status_reg, &[]);

    core.regs.set_ext(24, 0xefff);
    core.op_adiw(24, 0x01);
    assert_eq!(core.regs.ext(24), 0xf000);
    assert_status_reg_true!(&core.status_reg, &['n', 's']);

    core.regs.set_ext(24, 0xffff);
    core.op_adiw(24, 0x01);
    assert_eq!(core.regs.ext(24), 0x0000);
    assert_status_reg_true!(&core.status_reg, &['c', 'z']);

    core.regs.set_ext(24, 0x7fff);
    core.op_adiw(24, 0x01);
    assert_eq!(core.regs.ext(24), 0x8000);
    assert_status_reg_true!(&core.status_reg, &['v', 'n']);
}

#[test]
fn test_op_and() {
    let mut core = Core::new();

    core.regs[0] = 0xa7;
    core.regs[1] = 0x79;
    core.op_and(0, 1);
    assert_eq!(core.pc, 0x01);
    assert_eq!(core.regs[0], 0x21);
    assert_status_reg_true!(&core.status_reg, &[]);

    core.regs[0] = 0xf0;
    core.regs[1] = 0x0f;
    core.op_and(0, 1);
    assert_eq!(core.regs[0], 0x00);
    assert_status_reg_true!(&core.status_reg, &['z']);

    core.regs[0] = 0xa0;
    core.regs[1] = 0x91;
    core.op_and(0, 1);
    assert_eq!(core.regs[0], 0x80);
    assert_status_reg_true!(&core.status_reg, &['n', 's']);
}

#[test]
fn test_op_andi() {
    let mut core = Core::new();

    core.regs[16] = 0xa7;
    core.op_andi(16, 0x79);
    assert_eq!(core.pc, 0x01);
    assert_eq!(core.regs[16], 0x21);
    assert_status_reg_true!(&core.status_reg, &[]);

    core.regs[16] = 0xf0;
    core.op_andi(16, 0x0f);
    assert_eq!(core.regs[16], 0x00);
    assert_status_reg_true!(&core.status_reg, &['z']);

    core.regs[16] = 0xa0;
    core.op_andi(16, 0x91);
    assert_eq!(core.regs[16], 0x80);
    assert_status_reg_true!(&core.status_reg, &['n', 's']);
}

#[test]
fn test_op_asr() {
    let mut core = Core::new();

    core.regs[0] = 0x0e;
    core.op_asr(0);
    assert_eq!(core.pc, 0x01);
    assert_eq!(core.regs[0], 0x07);
    assert_status_reg_true!(&core.status_reg, &[]);

    core.regs[0] = 0x88;
    core.op_asr(0);
    assert_eq!(core.regs[0], 0xc4);
    assert_status_reg_true!(&core.status_reg, &['n', 'v']);

    core.regs[0] = 0x01;
    core.op_asr(0);
    assert_eq!(core.regs[0], 0x00);
    assert_status_reg_true!(&core.status_reg, &['z', 'c', 'v', 's']);
}

#[test]
fn test_op_bclr() {
    let mut core = Core::new();

    core.status_reg.h = true;
    core.op_bclr(5);
    assert_eq!(core.pc, 0x01);
    assert_status_reg_true!(&core.status_reg, &[]);
}

#[test]
fn test_op_bld() {
    let mut core = Core::new();

    core.status_reg.t = true;
    core.regs[0] = 0x00;
    core.op_bld(0, 5);
    assert_eq!(core.pc, 0x01);
    assert_eq!(core.regs[0], 0x20);

    core.status_reg.t = false;
    core.regs[0] = 0xff;
    core.op_bld(0, 5);
    assert_eq!(core.regs[0], 0xdf);
}

#[test]
fn test_op_brbc() {
    let mut core = Core::new();

    core.status_reg.z = true;
    assert_eq!(core.op_brbc(1, 0x10), 1);
    assert_eq!(core.pc, 0x01);

    core.status_reg.z = false;
    assert_eq!(core.op_brbc(1, 0x10), 2);
    assert_eq!(core.pc, 0x12);

    core.status_reg.h = false;
    assert_eq!(core.op_brbc(5, -0x05), 2);
    assert_eq!(core.pc, 0x0e);
}

#[test]
fn test_op_brbs() {
    let mut core = Core::new();

    core.status_reg.z = false;
    assert_eq!(core.op_brbs(1, 0x10), 1);
    assert_eq!(core.pc, 0x01);

    core.status_reg.z = true;
    assert_eq!(core.op_brbs(1, 0x10), 2);
    assert_eq!(core.pc, 0x12);

    core.status_reg.h = true;
    assert_eq!(core.op_brbs(5, -0x05), 2);
    assert_eq!(core.pc, 0x0e);
}

#[test]
fn test_op_bset() {
    let mut core = Core::new();

    core.op_bset(5);
    assert_eq!(core.pc, 0x01);
    assert_status_reg_true!(&core.status_reg, &['h']);
}

#[test]
fn test_op_bst() {
    let mut core = Core::new();

    core.regs[0] = 0x20;
    core.op_bst(0, 5);
    assert_eq!(core.pc, 0x01);
    assert_status_reg_true!(&core.status_reg, &['t']);

    core.regs[0] = 0xdf;
    core.op_bst(0, 5);
    assert_status_reg_true!(&core.status_reg, &[]);
}

#[test]
fn test_op_call() {
    let mut core = Core::new();

    core.pc = 0x01;
    core.op_call(0x0123);
    assert_eq!(core.pc, 0x0123);
    assert_eq!(core.sp, (SRAM_END - 1) - 2);
    assert_eq!(core.data_load_u16((SRAM_END - 1) - 1), 0x03);
}

#[test]
fn test_op_cbi() {
    let mut core = Core::new();

    core.data_store(io_regs::SPL, 0xff);
    core.op_cbi((io_regs::SPL - IOSPACE_ADDR) as u8, 5);
    assert_eq!(core.pc, 0x01);
    assert_eq!(core.data_load(io_regs::SPL), 0b11011111);
}

#[test]
fn test_op_clr() {
    let mut core = Core::new();

    core.regs[0] = 0xab;
    core.op_clr(0);
    assert_eq!(core.pc, 0x01);
    assert_eq!(core.regs[0], 0x00);
    assert_status_reg_true!(&core.status_reg, &['z']);
}

#[test]
fn test_op_com() {
    let mut core = Core::new();

    core.regs[0] = 0x12;
    core.op_com(0);
    assert_eq!(core.pc, 0x01);
    assert_eq!(core.regs[0], 0xed);
    assert_status_reg_true!(&core.status_reg, &['s', 'n', 'c']);

    core.regs[0] = 0xff;
    core.op_com(0);
    assert_eq!(core.regs[0], 0x00);
    assert_status_reg_true!(&core.status_reg, &['z', 'c']);
}

#[test]
fn test_op_cp() {
    let mut core = Core::new();

    core.regs[0] = 0xff;
    core.regs[1] = 0x01;
    core.op_cp(0, 1);
    assert_eq!(core.pc, 0x01);
    assert_status_reg_true!(&core.status_reg, &['n', 's']);

    core.regs[0] = 0x54;
    core.regs[1] = 0x54;
    core.op_cp(0, 1);
    assert_status_reg_true!(&core.status_reg, &['z']);

    core.regs[0] = 0x11;
    core.regs[1] = 0x20;
    core.op_cp(0, 1);
    assert_status_reg_true!(&core.status_reg, &['c', 's', 'n']);

    core.regs[0] = 0x11;
    core.regs[1] = 0x22;
    core.op_cp(0, 1);
    assert_status_reg_true!(&core.status_reg, &['c', 'h', 's', 'n']);

    core.regs[0] = 0x80;
    core.regs[1] = 0x70;
    core.op_cp(0, 1);
    assert_status_reg_true!(&core.status_reg, &['v', 's']);
}

#[test]
fn test_op_cpc() {
    let mut core = Core::new();

    core.regs[0] = 0x54;
    core.regs[1] = 0x53;
    core.status_reg.c = true;
    core.status_reg.z = true;
    core.op_cpc(0, 1);
    assert_status_reg_true!(&core.status_reg, &['z']);

    core.regs[0] = 0x11;
    core.regs[1] = 0x20;
    core.status_reg.c = true;
    core.op_cpc(0, 1);
    assert_status_reg_true!(&core.status_reg, &['c', 's', 'n']);

    core.regs[0] = 0x11;
    core.regs[1] = 0x21;
    core.status_reg.c = true;
    core.op_cpc(0, 1);
    assert_status_reg_true!(&core.status_reg, &['c', 'h', 's', 'n']);

    core.regs[0] = 0x81;
    core.regs[1] = 0x70;
    core.status_reg.c = true;
    core.op_cpc(0, 1);
    assert_status_reg_true!(&core.status_reg, &['v', 's']);
}

#[test]
fn test_op_cpi() {
    let mut core = Core::new();

    core.regs[0] = 0xff;
    core.op_cpi(0, 0x01);
    assert_eq!(core.pc, 0x01);
    assert_status_reg_true!(&core.status_reg, &['n', 's']);

    core.regs[0] = 0x54;
    core.op_cpi(0, 0x54);
    assert_status_reg_true!(&core.status_reg, &['z']);

    core.regs[0] = 0x11;
    core.op_cpi(0, 0x20);
    assert_status_reg_true!(&core.status_reg, &['c', 's', 'n']);

    core.regs[0] = 0x11;
    core.op_cpi(0, 0x22);
    assert_status_reg_true!(&core.status_reg, &['c', 'h', 's', 'n']);

    core.regs[0] = 0x80;
    core.op_cpi(0, 0x70);
    assert_status_reg_true!(&core.status_reg, &['v', 's']);
}

#[test]
fn test_op_cpse() {
    let mut core = Core::new();

    core.regs[0] = 0xaa;
    core.regs[1] = 0xbb;
    core.op_cpse(0, 1);
    assert_eq!(core.pc, 0x01);

    core.regs[0] = 0xaa;
    core.regs[1] = 0xaa;
    core.op_cpse(0, 1);
    assert_eq!(core.pc, 0x03);
}

#[test]
fn test_op_dec() {
    let mut core = Core::new();

    core.regs[0] = 0x21;
    core.op_dec(0);
    assert_eq!(core.pc, 0x01);
    assert_eq!(core.regs[0], 0x20);
    assert_status_reg_true!(&core.status_reg, &[]);

    core.regs[0] = 0x00;
    core.op_dec(0);
    assert_eq!(core.regs[0], 0xff);
    assert_status_reg_true!(&core.status_reg, &['s', 'n']);

    core.regs[0] = 0x80;
    core.op_dec(0);
    assert_eq!(core.regs[0], 0x7f);
    assert_status_reg_true!(&core.status_reg, &['s', 'v']);

    core.regs[0] = 0x01;
    core.op_dec(0);
    assert_eq!(core.regs[0], 0x00);
    assert_status_reg_true!(&core.status_reg, &['z']);
}

#[test]
fn test_op_eor() {
    let mut core = Core::new();

    core.regs[0] = 0x7a;
    core.regs[1] = 0xb6;
    core.op_eor(0, 1);
    assert_eq!(core.pc, 0x01);
    assert_eq!(core.regs[0], 0xcc);
    assert_status_reg_true!(&core.status_reg, &['n', 's']);

    core.regs[0] = 0x42;
    core.regs[1] = 0x42;
    core.op_eor(0, 1);
    assert_eq!(core.regs[0], 0x00);
    assert_status_reg_true!(&core.status_reg, &['z']);
}

#[test]
fn test_op_fmul() {
    let mut core = Core::new();

    core.regs[16] = 0x62;
    core.regs[17] = 0x53;
    core.op_fmul(16, 17);
    assert_eq!(core.pc, 0x01);
    assert_eq!(u16::from_le_bytes([core.regs[0], core.regs[1]]), 0x3f8c);
    assert_status_reg_true!(&core.status_reg, &[]);

    core.regs[16] = 0xa3;
    core.regs[17] = 0xf5;
    core.op_fmul(16, 17);
    assert_eq!(u16::from_le_bytes([core.regs[0], core.regs[1]]), 0x37fe);
    assert_status_reg_true!(&core.status_reg, &['c']);

    core.regs[16] = 0;
    core.regs[17] = 0;
    core.op_fmul(16, 17);
    assert_eq!(u16::from_le_bytes([core.regs[0], core.regs[1]]), 0x0000);
    assert_status_reg_true!(&core.status_reg, &['z']);
}

#[test]
fn test_op_fmuls() {
    let mut core = Core::new();

    core.regs[16] = 0x62;
    core.regs[17] = 0x53;
    core.op_fmuls(16, 17);
    assert_eq!(core.pc, 0x01);
    assert_eq!(u16::from_le_bytes([core.regs[0], core.regs[1]]), 0x3f8c);
    assert_status_reg_true!(&core.status_reg, &[]);

    core.regs[16] = 0xa3;
    core.regs[17] = 0xf5;
    core.op_fmuls(16, 17);
    assert_eq!(u16::from_le_bytes([core.regs[0], core.regs[1]]), 0x07fe);
    assert_status_reg_true!(&core.status_reg, &[]);

    core.regs[16] = 0;
    core.regs[17] = 0;
    core.op_fmuls(16, 17);
    assert_eq!(u16::from_le_bytes([core.regs[0], core.regs[1]]), 0x0000);
    assert_status_reg_true!(&core.status_reg, &['z']);

    core.regs[16] = 0x80;
    core.regs[17] = 0x80;
    core.op_fmuls(16, 17);
    assert_eq!(u16::from_le_bytes([core.regs[0], core.regs[1]]), 0x8000);
    assert_status_reg_true!(&core.status_reg, &[]);

    core.regs[16] = 0xff;
    core.regs[17] = 0x01;
    core.op_fmuls(16, 17);
    assert_eq!(u16::from_le_bytes([core.regs[0], core.regs[1]]), 0xfffe);
    assert_status_reg_true!(&core.status_reg, &['c']);
}

#[test]
fn test_op_fmulsu() {
    let mut core = Core::new();

    core.regs[16] = 0x62;
    core.regs[17] = 0x53;
    core.op_fmulsu(16, 17);
    assert_eq!(core.pc, 0x01);
    assert_eq!(u16::from_le_bytes([core.regs[0], core.regs[1]]), 0x3f8c);
    assert_status_reg_true!(&core.status_reg, &[]);

    core.regs[16] = 0xa3;
    core.regs[17] = 0xf5;
    core.op_fmulsu(16, 17);
    assert_eq!(u16::from_le_bytes([core.regs[0], core.regs[1]]), 0x4dfe);
    assert_status_reg_true!(&core.status_reg, &['c']);

    core.regs[16] = 0;
    core.regs[17] = 0;
    core.op_fmulsu(16, 17);
    assert_eq!(u16::from_le_bytes([core.regs[0], core.regs[1]]), 0x0000);
    assert_status_reg_true!(&core.status_reg, &['z']);

    core.regs[16] = 0x80;
    core.regs[17] = 0x80;
    core.op_fmulsu(16, 17);
    assert_eq!(u16::from_le_bytes([core.regs[0], core.regs[1]]), 0x8000);
    assert_status_reg_true!(&core.status_reg, &['c']);

    core.regs[16] = 0xff;
    core.regs[17] = 0x01;
    core.op_fmulsu(16, 17);
    assert_eq!(u16::from_le_bytes([core.regs[0], core.regs[1]]), 0xfffe);
    assert_status_reg_true!(&core.status_reg, &['c']);
}

#[test]
fn test_op_icall() {
    let mut core = Core::new();

    core.pc = 0x01;
    let bytes = 0x0123u16.to_le_bytes();
    core.regs[30] = bytes[0];
    core.regs[31] = bytes[1];
    core.op_icall();
    assert_eq!(core.pc, 0x0123);
    assert_eq!(core.sp, (SRAM_END - 1) - 2);
    assert_eq!(core.data_load_u16((SRAM_END - 1) - 1), 0x02);
}

#[test]
fn test_op_ijmp() {
    let mut core = Core::new();

    let bytes = 0x0123u16.to_le_bytes();
    core.regs[30] = bytes[0];
    core.regs[31] = bytes[1];
    core.op_ijmp();
    assert_eq!(core.pc, 0x0123);
}

#[test]
fn test_op_in() {
    let mut core = Core::new();

    core.op_in(0, (io_regs::SPL - IOSPACE_ADDR) as u8);
    assert_eq!(core.pc, 0x01);
    assert_eq!(core.regs[0], (core.sp & 0x00ff) as u8);
}

#[test]
fn test_op_inc() {
    let mut core = Core::new();

    core.regs[0] = 0x1f;
    core.op_inc(0);
    assert_eq!(core.pc, 0x01);
    assert_eq!(core.regs[0], 0x20);
    assert_status_reg_true!(&core.status_reg, &[]);

    core.regs[0] = 0x80;
    core.op_inc(0);
    assert_eq!(core.regs[0], 0x81);
    assert_status_reg_true!(&core.status_reg, &['s', 'n']);

    core.regs[0] = 0x7f;
    core.op_inc(0);
    assert_eq!(core.regs[0], 0x80);
    assert_status_reg_true!(&core.status_reg, &['v', 'n']);

    core.regs[0] = 0xff;
    core.op_inc(0);
    assert_eq!(core.regs[0], 0x00);
    assert_status_reg_true!(&core.status_reg, &['z']);
}

#[test]
fn test_op_jmp() {
    let mut core = Core::new();

    core.op_jmp(0x0123);
    assert_eq!(core.pc, 0x0123);
}

#[test]
fn test_op_ldi() {
    let mut core = Core::new();

    core.op_ldi(16, 0xab);
    assert_eq!(core.pc, 0x01);
    assert_eq!(core.regs[16], 0xab);
}

#[test]
fn test_op_lsr() {
    let mut core = Core::new();

    core.regs[0] = 0x0e;
    core.op_lsr(0);
    assert_eq!(core.pc, 0x01);
    assert_eq!(core.regs[0], 0x07);
    assert_status_reg_true!(&core.status_reg, &[]);

    core.regs[0] = 0x88;
    core.op_lsr(0);
    assert_eq!(core.regs[0], 0x44);
    assert_status_reg_true!(&core.status_reg, &[]);

    core.regs[0] = 0x01;
    core.op_lsr(0);
    assert_eq!(core.regs[0], 0x00);
    assert_status_reg_true!(&core.status_reg, &['z', 'c', 'v', 's']);
}

#[test]
fn test_op_mov() {
    let mut core = Core::new();

    core.regs[1] = 0xab;
    core.op_mov(0, 1);
    assert_eq!(core.pc, 0x01);
    assert_eq!(core.regs[0], 0xab);
}

#[test]
fn test_op_movw() {
    let mut core = Core::new();

    core.regs[2] = 0x01;
    core.regs[3] = 0x23;
    core.op_movw(0, 2);
    assert_eq!(core.pc, 0x01);
    assert_eq!(core.regs[0], 0x01);
    assert_eq!(core.regs[1], 0x23);
}

#[test]
fn test_op_mul() {
    let mut core = Core::new();

    core.regs[2] = 4;
    core.regs[3] = 3;
    core.op_mul(2, 3);
    assert_eq!(core.pc, 0x01);
    assert_eq!(u16::from_le_bytes([core.regs[0], core.regs[1]]), 12);
    assert_status_reg_true!(&core.status_reg, &[]);

    core.regs[2] = 241;
    core.regs[3] = 242;
    core.op_mul(2, 3);
    assert_eq!(u16::from_le_bytes([core.regs[0], core.regs[1]]), 58322);
    assert_status_reg_true!(&core.status_reg, &['c']);

    core.regs[2] = 171;
    core.regs[3] = 0;
    core.op_mul(2, 3);
    assert_eq!(u16::from_le_bytes([core.regs[0], core.regs[1]]), 0);
    assert_status_reg_true!(&core.status_reg, &['z']);
}

#[test]
fn test_op_muls() {
    let mut core = Core::new();

    core.regs[2] = -3i8 as u8;
    core.regs[3] = 4i8 as u8;
    core.op_muls(2, 3);
    assert_eq!(core.pc, 0x01);
    assert_eq!(i16::from_le_bytes([core.regs[0], core.regs[1]]), -12);
    assert_status_reg_true!(&core.status_reg, &['c']);

    core.regs[2] = -81i8 as u8;
    core.regs[3] = -79i8 as u8;
    core.op_muls(2, 3);
    assert_eq!(i16::from_le_bytes([core.regs[0], core.regs[1]]), 6399);
    assert_status_reg_true!(&core.status_reg, &[]);

    core.regs[2] = -23i8 as u8;
    core.regs[3] = 0i8 as u8;
    core.op_muls(2, 3);
    assert_eq!(i16::from_le_bytes([core.regs[0], core.regs[1]]), 0);
    assert_status_reg_true!(&core.status_reg, &['z']);
}

#[test]
fn test_op_mulsu() {
    let mut core = Core::new();

    core.regs[2] = -3i8 as u8;
    core.regs[3] = 200;
    core.op_mulsu(2, 3);
    assert_eq!(core.pc, 0x01);
    assert_eq!(i16::from_le_bytes([core.regs[0], core.regs[1]]), -600);
    assert_status_reg_true!(&core.status_reg, &['c']);

    core.regs[2] = 120i8 as u8;
    core.regs[3] = 234;
    core.op_mulsu(2, 3);
    assert_eq!(i16::from_le_bytes([core.regs[0], core.regs[1]]), 28080);
    assert_status_reg_true!(&core.status_reg, &[]);

    core.regs[2] = -23i8 as u8;
    core.regs[3] = 0 as u8;
    core.op_mulsu(2, 3);
    assert_eq!(i16::from_le_bytes([core.regs[0], core.regs[1]]), 0);
    assert_status_reg_true!(&core.status_reg, &['z']);
}

#[test]
fn test_op_neg() {
    let mut core = Core::new();

    core.regs[0] = 0x00;
    core.op_neg(0);
    assert_eq!(core.pc, 0x01);
    assert_eq!(core.regs[0], 0x00);
    assert_status_reg_true!(&core.status_reg, &['z']);

    core.regs[0] = 0x80;
    core.op_neg(0);
    assert_eq!(core.regs[0], 0x80);
    assert_status_reg_true!(&core.status_reg, &['v', 'n', 'c']);

    core.regs[0] = 0x01;
    core.op_neg(0);
    assert_eq!(core.regs[0], 0xff);
    assert_status_reg_true!(&core.status_reg, &['h', 's', 'n', 'c']);
}

#[test]
fn test_op_or() {
    let mut core = Core::new();

    core.regs[0] = 0x12;
    core.regs[1] = 0x34;
    core.op_or(0, 1);
    assert_eq!(core.pc, 0x01);
    assert_eq!(core.regs[0], 0x36);
    assert_status_reg_true!(&core.status_reg, &[]);

    core.regs[0] = 0x81;
    core.regs[1] = 0x65;
    core.op_or(0, 1);
    assert_eq!(core.regs[0], 0xe5);
    assert_status_reg_true!(&core.status_reg, &['n']);

    core.regs[0] = 0x00;
    core.regs[1] = 0x00;
    core.op_or(0, 1);
    assert_eq!(core.regs[0], 0x00);
    assert_status_reg_true!(&core.status_reg, &['z']);
}

#[test]
fn test_op_ori() {
    let mut core = Core::new();

    core.regs[0] = 0x12;
    core.op_ori(0, 0x34);
    assert_eq!(core.pc, 0x01);
    assert_eq!(core.regs[0], 0x36);
    assert_status_reg_true!(&core.status_reg, &[]);

    core.regs[0] = 0x81;
    core.op_ori(0, 0x65);
    assert_eq!(core.regs[0], 0xe5);
    assert_status_reg_true!(&core.status_reg, &['n']);

    core.regs[0] = 0x00;
    core.op_ori(0, 0x00);
    assert_eq!(core.regs[0], 0x00);
    assert_status_reg_true!(&core.status_reg, &['z']);
}

#[test]
fn test_op_out() {
    let mut core = Core::new();

    core.regs[0] = 0x12;
    core.op_out((io_regs::SPL - IOSPACE_ADDR) as u8, 0);
    assert_eq!(core.pc, 0x01);
    assert_eq!(core.data_load(io_regs::SPL), 0x12);
}

#[test]
fn test_op_push_pop() {
    let mut core = Core::new();

    core.regs[0] = 0x42;
    core.op_push(0);
    assert_eq!(core.pc, 0x01);
    assert_eq!(core.sp, SRAM_END - 1 - 1);
    assert_eq!(core.data_load(SRAM_END - 1), 0x42);

    core.op_pop(1);
    assert_eq!(core.pc, 0x02);
    assert_eq!(core.sp, SRAM_END - 1);
    assert_eq!(core.regs[1], 0x42);
}

#[test]
fn test_op_rcall() {
    let mut core = Core::new();

    core.pc = 0x01;
    core.op_rcall(0x0123);
    assert_eq!(core.pc, 0x0124);
    assert_eq!(core.sp, (SRAM_END - 1) - 2);
    assert_eq!(core.data_load_u16((SRAM_END - 1) - 1), 0x02);

    core.op_rcall(-0x0025);
    assert_eq!(core.pc, 0x00ff);
    assert_eq!(core.sp, (SRAM_END - 1) - (2 + 2));
    assert_eq!(core.data_load_u16((SRAM_END - 1) - (1 + 2)), 0x0125);
}

#[test]
fn test_op_ret() {
    let mut core = Core::new();

    core.pc = 0x51;
    core.data_store_u16(SRAM_END - 1 - 1, 0x0123);
    core.sp = SRAM_END - 1 - 2;
    core.op_ret();
    assert_eq!(core.pc, 0x0123);
    assert_eq!(core.sp, SRAM_END - 1);
}

#[test]
fn test_op_reti() {
    let mut core = Core::new();

    core.pc = 0x51;
    core.data_store_u16(SRAM_END - 1 - 1, 0x0123);
    core.sp = SRAM_END - 1 - 2;
    core.op_reti();
    assert_eq!(core.pc, 0x0123);
    assert_eq!(core.sp, SRAM_END - 1);
    assert_status_reg_true!(&core.status_reg, &['i']);
}

#[test]
fn test_op_rjmp() {
    let mut core = Core::new();

    core.pc = 0x01;
    core.op_rjmp(0x0123);
    assert_eq!(core.pc, 0x0124);

    core.op_rjmp(-0x0025);
    assert_eq!(core.pc, 0x00ff);
}

// #[test]
// fn test_op_rol() {
//     let mut core = Core::new();
//
//     core.regs[0] = 0x00;
//     core.status_reg.c = false;
//     core.op_rol(0);
//     assert_eq!(core.pc, 0x01);
//     assert_eq!(core.regs[0], 0x00);
//     assert_status_reg_true!(&core.status_reg, &['z']);
//
//     core.regs[0] = 0x00;
//     core.status_reg.c = true;
//     core.op_rol(0);
//     assert_eq!(core.regs[0], 0x01);
//     assert_status_reg_true!(&core.status_reg, &[]);
//
//     core.regs[0] = 0x90;
//     core.status_reg.c = false;
//     core.op_rol(0);
//     assert_eq!(core.regs[0], 0x20);
//     assert_status_reg_true!(&core.status_reg, &['c', 's', 'v']);
//
//     core.regs[0] = 0x48;
//     core.status_reg.c = false;
//     core.op_rol(0);
//     assert_eq!(core.regs[0], 0x90);
//     assert_status_reg_true!(&core.status_reg, &['n', 'v', 'h']);
// }

#[test]
fn test_op_ror() {
    let mut core = Core::new();

    core.regs[0] = 0x00;
    core.status_reg.c = false;
    core.op_ror(0);
    assert_eq!(core.pc, 0x01);
    assert_eq!(core.regs[0], 0x00);
    assert_status_reg_true!(&core.status_reg, &['z']);

    core.regs[0] = 0x00;
    core.status_reg.c = true;
    core.op_ror(0);
    assert_eq!(core.regs[0], 0x80);
    assert_status_reg_true!(&core.status_reg, &['n', 'v']);

    core.regs[0] = 0x81;
    core.status_reg.c = false;
    core.op_ror(0);
    assert_eq!(core.regs[0], 0x40);
    assert_status_reg_true!(&core.status_reg, &['c', 's', 'v']);
}

#[test]
fn test_op_sbc() {
    let mut core = Core::new();

    core.status_reg.c = true;
    core.regs[0] = 0x08;
    core.regs[1] = 0x04;
    core.op_sbc(0, 1);
    assert_eq!(core.pc, 0x01);
    assert_eq!(core.regs[0], 0x03);
    assert_status_reg_true!(&core.status_reg, &[]);

    core.status_reg.c = true;
    core.status_reg.z = true;
    core.regs[0] = 0x80;
    core.regs[1] = 0x7f;
    core.op_sbc(0, 1);
    assert_eq!(core.regs[0], 0x00);
    assert_status_reg_true!(&core.status_reg, &['z', 's', 'h', 'v']);

    core.status_reg.c = false;
    core.regs[0] = 0x00;
    core.regs[1] = 0x01;
    core.op_sbc(0, 1);
    assert_eq!(core.regs[0], 0xff);
    assert_status_reg_true!(&core.status_reg, &['s', 'n', 'c', 'h']);
}

#[test]
fn test_op_sbci() {
    let mut core = Core::new();

    core.status_reg.c = true;
    core.regs[0] = 0x08;
    core.op_sbci(0, 0x04);
    assert_eq!(core.pc, 0x01);
    assert_eq!(core.regs[0], 0x03);
    assert_status_reg_true!(&core.status_reg, &[]);

    core.status_reg.c = true;
    core.status_reg.z = true;
    core.regs[0] = 0x80;
    core.op_sbci(0, 0x7f);
    assert_eq!(core.regs[0], 0x00);
    assert_status_reg_true!(&core.status_reg, &['z', 's', 'h', 'v']);

    core.status_reg.c = false;
    core.regs[0] = 0x00;
    core.op_sbci(0, 0x01);
    assert_eq!(core.regs[0], 0xff);
    assert_status_reg_true!(&core.status_reg, &['s', 'n', 'c', 'h']);
}

#[test]
fn test_op_sbi() {
    let mut core = Core::new();

    core.data_store(io_regs::SPL, 0b10001000);
    core.op_sbi((io_regs::SPL - IOSPACE_ADDR) as u8, 1);
    assert_eq!(core.pc, 0x01);
    assert_eq!(core.data_load(io_regs::SPL), 0b10001010);
}

#[test]
fn test_op_sbic() {
    let mut core = Core::new();

    core.data_store(io_regs::SPL, 0b00000010);
    core.op_sbic((io_regs::SPL - IOSPACE_ADDR) as u8, 1);
    assert_eq!(core.pc, 0x01);

    core.data_store(io_regs::SPL, 0b00000100);
    core.op_sbic((io_regs::SPL - IOSPACE_ADDR) as u8, 1);
    assert_eq!(core.pc, 0x03);
}

#[test]
fn test_op_sbis() {
    let mut core = Core::new();

    core.data_store(io_regs::SPL, 0b000001000);
    core.op_sbis((io_regs::SPL - IOSPACE_ADDR) as u8, 1);
    assert_eq!(core.pc, 0x01);

    core.data_store(io_regs::SPL, 0b00000010);
    core.op_sbis((io_regs::SPL - IOSPACE_ADDR) as u8, 1);
    assert_eq!(core.pc, 0x03);
}

#[test]
fn test_op_sbiw() {
    let mut core = Core::new();

    core.regs.set_ext(24, 0x2315);
    core.op_sbiw(24, 0x21);
    assert_eq!(core.pc, 0x01);
    assert_eq!(core.regs.ext(24), 0x22f4);
    assert_status_reg_true!(&core.status_reg, &[]);

    core.regs.set_ext(24, 0x0000);
    core.op_sbiw(24, 0x01);
    assert_eq!(core.regs.ext(24), 0xffff);
    assert_status_reg_true!(&core.status_reg, &['n', 's', 'c']);

    core.regs.set_ext(24, 0x0001);
    core.op_sbiw(24, 0x01);
    assert_eq!(core.regs.ext(24), 0x0000);
    assert_status_reg_true!(&core.status_reg, &['z']);
}

#[test]
fn test_op_sbrc() {
    let mut core = Core::new();

    core.regs[0] = 0b00000010;
    core.op_sbrc(0, 1);
    assert_eq!(core.pc, 0x01);

    core.regs[0] = 0b00000100;
    core.op_sbrc(0, 1);
    assert_eq!(core.pc, 0x03);
}

#[test]
fn test_op_sbrs() {
    let mut core = Core::new();

    core.regs[0] = 0b00000100;
    core.op_sbrs(0, 1);
    assert_eq!(core.pc, 0x01);

    core.regs[0] = 0b00000010;
    core.op_sbrs(0, 1);
    assert_eq!(core.pc, 0x03);
}

#[test]
fn test_op_ser() {
    let mut core = Core::new();

    core.op_ser(0);
    assert_eq!(core.pc, 0x01);
    assert_eq!(core.regs[0], 0xff);
}

#[test]
fn test_op_sub() {
    let mut core = Core::new();

    core.regs[0] = 0x08;
    core.regs[1] = 0x05;
    core.op_sub(0, 1);
    assert_eq!(core.pc, 0x01);
    assert_eq!(core.regs[0], 0x03);
    assert_status_reg_true!(&core.status_reg, &[]);

    core.regs[0] = 0x80;
    core.regs[1] = 0x80;
    core.op_sub(0, 1);
    assert_eq!(core.regs[0], 0x00);
    assert_status_reg_true!(&core.status_reg, &['z']);

    core.regs[0] = 0x00;
    core.regs[1] = 0x01;
    core.op_sub(0, 1);
    assert_eq!(core.regs[0], 0xff);
    assert_status_reg_true!(&core.status_reg, &['s', 'n', 'c', 'h']);
}

#[test]
fn test_op_subi() {
    let mut core = Core::new();

    core.regs[0] = 0x08;
    core.op_subi(0, 0x05);
    assert_eq!(core.pc, 0x01);
    assert_eq!(core.regs[0], 0x03);
    assert_status_reg_true!(&core.status_reg, &[]);

    core.regs[0] = 0x80;
    core.op_subi(0, 0x80);
    assert_eq!(core.regs[0], 0x00);
    assert_status_reg_true!(&core.status_reg, &['z']);

    core.regs[0] = 0x00;
    core.op_subi(0, 0x01);
    assert_eq!(core.regs[0], 0xff);
    assert_status_reg_true!(&core.status_reg, &['s', 'n', 'c', 'h']);
}

#[test]
fn test_op_swap() {
    let mut core = Core::new();

    core.regs[0] = 0xab;
    core.op_swap(0);
    assert_eq!(core.pc, 0x01);
    assert_eq!(core.regs[0], 0xba);
}
