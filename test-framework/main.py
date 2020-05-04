#!/usr/bin/env python3

import sys
import serial
import os
import time

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

ops_alu0 = ['ADD', 'AND', 'CP', 'EOR', 'MOV', 'OR', 'SUB']
ops_alu1 = ['ADC', 'CPC', 'SBC']

ops = [
        ('ALU0', ops_alu0, '# a0 b a1 sreg', 4, [0xff, 0xff]),
        ('ALU1', ops_alu1, '# sreg0 a0 b a1 sreg1', 5, [1 << 0, 0xff, 0xff]),
        ]

if __name__ == "__main__":
    port = sys.argv[1]
    os.makedirs(OUT_PATH, exist_ok=True)
    # reset(port)
    with serial.Serial(port, SERIAL_SPEED) as com:
        com.write(CMD_START + b'\n')
        cmd = com.read(2)
        if cmd != b'OK':
            raise Exception(f'Expecting OK but got {cmd}')
        for op_cfg in ops:
            op_type = op_cfg[0]
            ops = op_cfg[1]
            out_header = op_cfg[2]
            out_len = op_cfg[3]
            out_end = bytearray(op_cfg[4])
            for op in ops:
                print(op)
                com.write(op_type.encode('ascii') + b'\n')
                com.write(op.encode('ascii') + b'\n')
                with open(f'{OUT_PATH}/{op}', 'w') as out:
                    out.write(out_header + '\n')
                    while True:
                        r = com.read(out_len)
                        r_hex = []
                        for b in r:
                            r_hex.append('{:02x}'.format(b))
                        out.write(' '.join(r_hex) + '\n')
                        if r[:len(out_end)] == out_end:
                            print('done')
                            break
