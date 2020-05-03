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

if __name__ == "__main__":
    port = sys.argv[1]
    os.makedirs(OUT_PATH, exist_ok=True)
    # reset(port)
    with serial.Serial(port, SERIAL_SPEED) as com:
        com.write(CMD_START + b'\n')
        cmd = com.read(2)
        if cmd != b'OK':
            raise Exception(f'Expecting OK but got {cmd}')
        ops = ['ADD', 'AND']
        for op in ops:
            print(op)
            com.write(op.encode('ascii') + b'\n')
            with open(f'{OUT_PATH}/{op}', 'w') as out:
                out.write('# a0 b a1 sreg\n')
                while True:
                    r = com.read(4)
                    a, b, r, sreg = r[0], r[1], r[2], r[3]
                    out.write('{:02x} {:02x} {:02x} {:02x}\n'.format(a, b, r, sreg))
                    if a == 0xff and b == 0xff:
                        print('done')
                        break
