#!/usr/bin/env python3

import termios
import sys
import serial
import os
import time
import codegen
import subprocess

# SERIAL_SPEED = 57600
# SERIAL_SPEED = 115200
SERIAL_SPEED = 1000000

def reset(port):
    com = serial.Serial(port, 1200)
    com.dtr=False
    com.close()

    ok = False
    for i in range(32):
        try:
            statinfo = os.stat(port)
            if statinfo.st_gid != 0:
                ok = True
                break
        except FileNotFoundError:
            pass
        time.sleep(0.2)
    if not ok:
        raise Exception('Serial port not comming up as expected after reset')

CMD_START = b'START'
OUT_PATH = 'vectors'

OPS_ALU0  = ['add', 'and', 'cp', 'eor', 'mov', 'or', 'sub']
OPS_ALU0S = ['adc', 'cpc', 'sbc']
OPS_ALU1  = ['com', 'neg', 'inc', 'dec', 'ser', 'asr', 'swap']
OPS_ALU1S = ['ror']
OPS_ALU2 = ['subi', 'andi', 'ori', 'cpi', 'ldi']
OPS_ALU2S = ['sbci']
OPS_ALU3 = ['adiw', 'sbiw']
OPS_ALU4 = ['mul', 'muls', 'mulsu', 'fmul', 'fmuls', 'fmulsu']

# OPS_ALU0  = []
# OPS_ALU0S = []
# OPS_ALU1  = []
# OPS_ALU1S = []
# OPS_ALU2 = ['andi']
# OPS_ALU2S = []

OPS = [
        # (OP_TYPE, OP_LIST, OUT_HEADER, OUT_LEN, OUT_END)
        # ('alu0' , OPS_ALU0 , '# sreg0 a0 b a1 sreg1', 5, [0 << 0, 0xff, 0xff]),
        # ('alu0s', OPS_ALU0S, '# sreg0 a0 b a1 sreg1', 5, [1 << 0, 0xff, 0xff]),
        # ('alu1' , OPS_ALU1 , '# sreg0 a0 a1 sreg1'  , 4, [0 << 0, 0xff]      ),
        # ('alu1s', OPS_ALU1S, '# sreg0 a0 a1 sreg1'  , 4, [1 << 0, 0xff]      ),
        ('alu4' , OPS_ALU4 , '# a0 b0 a1 b1 sreg1'  , 5, [0xff, 0xff]        ),
        ]

OPS_K = [
        # ('alu2' , OPS_ALU2 , '# sreg0 a0 k a1 sreg1'  , 4, [0 << 0, 0xff], codegen.OPS_K['alu2'][3]),
        # ('alu2s', OPS_ALU2S, '# sreg0 a0 k a1 sreg1'  , 4, [1 << 0, 0xff], codegen.OPS_K['alu2s'][3]),
        ('alu3' , OPS_ALU3 , '# a0 b0 k a1 b1 sreg1'  , 5, [0xff, 0xff]  , codegen.OPS_K['alu3'][3]),
        ]

def test_op(com, op_type, op, out_header, out_len, out_end):
    print(f'{op} .. ', flush=True, end='')
    com.write(op_type.encode('ascii') + b'\n')
    com.write(op.encode('ascii') + b'\n')
    with open(f'{OUT_PATH}/{op}', 'w') as out:
        out.write(out_header + '\n')
        while True:
            r = com.read(out_len)
            r_hex = []
            for b in r:
                r_hex.append('{:02x}'.format(b))
            # print(' '.join(r_hex))
            out.write(' '.join(r_hex) + '\n')
            if r[:len(out_end)] == out_end:
                print('done')
                # print('Early exit')
                # com.close()
                # sys.exit(1)
                break

def test_op_k(com, op_type, op, out_header, out_len, out_end, k_range):
    print(f'{op}_k')
    with open(f'{OUT_PATH}/{op}', 'w') as out:
        out.write(out_header + '\n')
        for k in range(k_range):
            k = '{:02x}'.format(k)
            print(f'{op}_{k} .. ', flush=True, end='')
            com.write(op_type.encode('ascii') + b'\n')
            com.write(f'{op}_{k}'.encode('ascii') + b'\n')
            while True:
                r = com.read(out_len)
                r_hex = []
                for b in r:
                    r_hex.append('{:02x}'.format(b))
                r_hex = r_hex[:2] + [k] + r_hex[2:]
                # print(r_hex)
                out.write(' '.join(r_hex) + '\n')
                if r[:len(out_end)] == out_end:
                    print('done')
                    break

def connect(port):
    # f = open(port)
    # attrs = termios.tcgetattr(f)
    # attrs[2] = attrs[2] & ~termios.HUPCL
    # termios.tcsetattr(f, termios.TCSAFLUSH, attrs)
    # f.close()

    com = serial.Serial(None, SERIAL_SPEED)
    com.port = port
    # com.close()
    # com.dtr=False
    # com._dtr_state=False
    # com._rts_state=False
    # com.rts=False
    com.open()
    # com.setRTS(True)
    # com.setDTR(False)
    # com.rts=False
    com.write(CMD_START + b'\n')
    print('wait OK...')
    cmd = com.read(2)
    if cmd != b'OK':
        raise Exception(f'Expecting OK but got {cmd}')
    print('OK')
    return com

def make_upload():
    subprocess.run(['make', 'upload'], stderr=subprocess.STDOUT)
    time.sleep(2)
    # subprocess.run(['make', 'upload'], stderr=subprocess.STDOUT, check=True)

def run_test_ops(port):
    print('Writing ops.h for ops')
    with open('ops.h', 'w') as f:
        op_types = []
        for op_cfg in OPS:
            op_type = op_cfg[0]
            op_types.append(op_type)
            op_list = op_cfg[1]
            codegen.gen_ops(f, op_type, op_list)
            # codegen.gen_test_op_select(f, op_type, op_list)
        codegen.gen_test_main(f, op_types)
    # reset(port)
    make_upload()

    com = connect(port)
    for op_cfg in OPS:
        op_type = op_cfg[0]
        op_list = op_cfg[1]
        out_header = op_cfg[2]
        out_len = op_cfg[3]
        out_end = bytearray(op_cfg[4])
        for op in op_list:
            test_op(com, op_type, op, out_header, out_len, out_end)
    com.close()

def run_test_ops_k(port):
    for op_cfg in OPS_K:
        op_type = op_cfg[0]
        op_list = op_cfg[1]
        out_header = op_cfg[2]
        out_len = op_cfg[3]
        out_end = bytearray(op_cfg[4])
        k_range = op_cfg[5]
        for op in op_list:
            print(f'Writing ops.h for op_k: {op_type}:{op}')
            with open('ops.h', 'w') as f:
                codegen.gen_ops_k(f, op_type, op)
                # codegen.gen_test_op_k_select(f, op_type, op, k_range)
                codegen.gen_test_main(f, [op_type])
            # reset(port)
            make_upload()

            com = connect(port)
            test_op_k(com, op_type, op, out_header, out_len, out_end, k_range)
            com.close()

if __name__ == "__main__":
    port = sys.argv[1]
    os.makedirs(OUT_PATH, exist_ok=True)

    run_test_ops(port)
    # run_test_ops_k(port)

