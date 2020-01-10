#!/usr/bin/env python3

instructions = """
   5 ADC    0001_11rd_dddd_rrrr
   6 ADD    0000_11rd_dddd_rrrr
   7 ADIW   1001_0110_KKdd_KKKK
   8 AND    0010_00rd_dddd_rrrr
   9 ANDI   0111_KKKK_dddd_KKKK
  10 ASR    1001_010d_dddd_0101
  11 BCLR   1001_0100_1sss_1000
  12 BLD    1111_100d_dddd_0bbb
  13 BRBC   1111_01kk_kkkk_ksss
  14 BRBS   1111_00kk_kkkk_ksss
  17 BREAK  1001_0101_1001_1000
  34 BSET   1001_0100_0sss_1000
  35 BST    1111_101d_dddd_0bbb
  36 CALL   1001_010k_kkkk_111k kkkk_kkkk_kkkk_kkkk
  37 CBI    1001_1000_AAAA_Abbb
  43 CLR    0010_01dd_dddd_dddd
  48 COM    1001_010d_dddd_0000
  49 CP     0001_01rd_dddd_rrrr
  50 CPC    0000_01rd_dddd_rrrr
  51 CPI    0011_KKKK_dddd_KKKK
  52 CPSE   0001_00rd_dddd_rrrr
  53 DEC    1001_010d_dddd_1010
  55 EICALL 1001_0101_0001_1001
  56 EIJMP  1001_0100_0001_1001
  57 ELPMR0 1001_0101_1101_1000
  57 ELPM   1001_000d_dddd_0110
  57 ELPMINC 1001_000d_dddd_0111
  58 EOR    0010_01rd_dddd_rrrr
  59 FMUL   0000_0011_0ddd_1rrr
  60 FMULS  0000_0011_1ddd_0rrr
  61 FMULSU 0000_0011_1ddd_1rrr
  62 ICALL  1001_0101_0000_1001
  63 IJMP   1001_0100_0000_1001
  64 IN     1011_0AAd_dddd_AAAA
  65 INC    1001_010d_dddd_0011
  66 JMP    1001_010k_kkkk_110k kkkk_kkkk_kkkk_kkkk
  70 LDX    1001_000d_dddd_1100
  70 LDXINC 1001_000d_dddd_1101
  70 LDXDEC 1001_000d_dddd_1111
  71 LDYINC 1001_000d_dddd_1001
  71 LDYDEC 1001_000d_dddd_1010
  71 LDYADQ 10q0_qq0d_dddd_1qqq
  72 LDZINC 1001_000d_dddd_0001
  72 LDZDEC 1001_000d_dddd_0010
  72 LDZADQ 10q0_qq0d_dddd_0qqq
  73 LDI    1110_KKKK_dddd_KKKK
  74 LDS    1001_000d_dddd_0000 kkkk_kkkk_kkkk_kkkk
  75 LDS16  1010_0kkk_dddd_kkkk
  76 LPMR0  1001_0101_1100_1000
  76 LPM    1001_000d_dddd_0100
  76 LPMINC 1001_000d_dddd_0101
  78 LSR    1001_010d_dddd_0110
  79 MOV    0010_11rd_dddd_rrrr
  80 MOVW   0000_0001_dddd_rrrr
  81 MUL    1001_11rd_dddd_rrrr
  82 MULS   0000_0010_dddd_rrrr
  83 MULSU  0000_0011_0ddd_0rrr
  84 NEG    1001_010d_dddd_0001
  85 NOP    0000_0000_0000_0000
  86 OR     0010_10rd_dddd_rrrr
  87 ORI    0110_KKKK_dddd_KKKK
  88 OUT    1011_1AAr_rrrr_AAAA
  89 POP    1001_000d_dddd_1111
  90 PUSH   1001_001r_rrrr_1111
  91 RCALL  1101_kkkk_kkkk_kkkk
  92 RET    1001_0101_0000_1000
  93 RETI   1001_0101_0001_1000
  94 RJMP   1100_kkkk_kkkk_kkkk
  96 ROR    1001_010d_dddd_0111
  97 SBC    0000_10rd_dddd_rrrr
  98 SBCI   0100_KKKK_dddd_KKKK
  99 SBI    1001_1010_AAAA_Abbb
 100 SBIC   1001_1001_AAAA_Abbb
 101 SBIS   1001_1011_AAAA_Abbb
 102 SBIW   1001_0111_KKdd_KKKK
 104 SBRC   1111_110r_rrrr_0bbb
 105 SBRS   1111_110r_rrrr_0bbb
 110 SER    1110_1111_dddd_1111
 115 SLEEP  1001_0101_1000_1000
 116 SPM    1001_0101_1110_1000
 117 SPM2   1001_0101_1111_1000
 118 STX    1001_001r_rrrr_1100
 118 STXINC 1001_001r_rrrr_1101
 118 STXDEC 1001_001r_rrrr_1111
 119 STYINC 1001_001r_rrrr_1001
 119 STYDEC 1001_001r_rrrr_1010
 119 STYADQ 10q0_qq1r_rrrr_1qqq
 120 STZINC 1001_001r_rrrr_0001
 120 STZDEC 1001_001r_rrrr_0010
 120 STZADQ 10q0_qq1r_rrrr_0qqq
 121 STS    1001_001d_dddd_0000 kkkk_kkkk_kkkk_kkkk
 122 STS16  1010_1kkk_dddd_kkkk
 123 SUB    0001_10rd_dddd_rrrr
 124 SUBI   0101_KKKK_dddd_KKKK
 125 SWAP   1001_010d_dddd_0010
 127 WDR    1001_0101_1010_1000
"""

def parse(line):
    elems = inst.split()
    num = elems[0]
    mnemonic = elems[1]
    w0 = elems[2]
    w1 = None
    if len(elems) > 3:
        w1 = elems[3]
    return (num, mnemonic, w0, 1)

# Bit Masks

for inst in instructions.split('\n'):
    if len(inst) == 0 or inst[0] == '#':
        continue
    (num, mnemonic, w0, w1) = parse(inst)
    vars_masked = ''.join([b if b in ['0', '1', '_'] else 'x' for b in w0])
    bits = vars_masked.replace('x', '0')
    mask = vars_masked.replace('0', '1').replace('x', '0')
    print(f'// {num}')
    print(f'const OPCODE_OP_{mnemonic}_BITS: u16 = 0b{bits};')
    print(f'const OPCODE_OP_{mnemonic}_MASK: u16 = 0b{mask};')
    print()

# Opcode decoding

def build_mask(mask):
    binmask = ''.join('1' if i in mask else '0' for i in range(16))
    binmask = '_'.join(binmask[i*4:i*4+4] for i in range(int(16/4)))
    return f'0b{binmask}'

def build_arg_decoder(arg, w0):
    bits = w0.replace('_', '')
    masks = []
    mask = []
    for i in range(16):
        if arg == bits[i]:
            mask.append(i)
        elif len(mask) != 0:
            masks.append(mask)
            mask = []
    if len(mask) != 0:
        masks.append(mask)

    parts = []
    acc = 0
    for mask in masks[::-1]:
        if mask[-1] == 15:
            parts.append(f'w0 & {build_mask(mask)}')
        else:
            shift = 15 - mask[-1] - acc
            parts.append(f'(w0 & {build_mask(mask)}) >> {shift}')
        acc = acc + (mask[-1] + 1 - mask[0])
    return ' | '.join(parts[::-1])

print('match w0 {')
for inst in instructions.split('\n'):
    if len(inst) == 0 or inst[0] == '#':
        continue
    (num, mnemonic, w0, w1) = parse(inst)
    w0 = w0.lower()
    args = set([b for b in w0])
    args = args.difference(set(['0', '1', '_']))

    args_decoder = {}
    for arg in args:
        args_decoder[arg] = build_arg_decoder(arg, w0)

    print(f'  _ if (w0 & OPCODE_OP_{mnemonic}_MASK) == OPCODE_OP_{mnemonic}_BITS => {{')
    if len(args) == 0:
        print(f'    Op::{mnemonic.title()}')
    else:
        print(f'    Op::{mnemonic.title()} {{')
        for arg in args:
            print(f'      {arg}: ({args_decoder[arg]}) as u8,')
        print('    }')
    print('  },')

print('  _ => unreachable!(),')
print('}')
